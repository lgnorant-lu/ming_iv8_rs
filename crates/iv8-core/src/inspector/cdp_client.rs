//! CDP Client — programmatic V8 Inspector control from Rust.
//!
//! Sends CDP protocol messages directly to the V8 InspectorSession
//! (bypassing WebSocket) and collects responses/events synchronously.
//!
//! This enables Python-side APIs like:
//! - ctx.cdp_set_breakpoint(url, line, col, condition)
//! - ctx.cdp_evaluate_on_frame(frame_idx, expression)
//! - ctx.cdp_step_over() / step_into() / step_out() / resume()
//! - ctx.cdp_get_call_frames()

use crate::inspector::channel::{InspectorMessage, SharedChannelState, lock_channel_state};
use std::sync::atomic::{AtomicU32, Ordering};

/// A programmatic CDP client that talks directly to V8 Inspector.
pub struct CdpClient {
    channel_state: SharedChannelState,
    next_id: AtomicU32,
    /// Whether Debugger.enable has been sent.
    debugger_enabled: bool,
    /// Stored call frames from the last Debugger.paused event.
    pub last_paused_frames: Option<serde_json::Value>,
    /// Whether execution is currently paused.
    pub is_paused: bool,
}

impl CdpClient {
    pub fn new(channel_state: SharedChannelState) -> Self {
        Self {
            channel_state,
            next_id: AtomicU32::new(1000), // Start at 1000 to avoid collision with DevTools
            debugger_enabled: false,
            last_paused_frames: None,
            is_paused: false,
        }
    }

    /// Ensure Debugger domain is enabled. Idempotent.
    pub fn ensure_debugger_enabled(
        &mut self,
        session: &v8::inspector::V8InspectorSession,
    ) {
        if self.debugger_enabled {
            return;
        }
        let _ = self.send_and_wait(session, "Debugger.enable", serde_json::json!({}));
        // Also enable Runtime for evaluateOnCallFrame
        let _ = self.send_and_wait(session, "Runtime.enable", serde_json::json!({}));
        self.debugger_enabled = true;
    }

    /// Send a CDP method call and wait for the response.
    ///
    /// This is synchronous: it dispatches the message, then drains outgoing
    /// messages from the channel until we find the matching response (by id).
    pub fn send_and_wait(
        &self,
        session: &v8::inspector::V8InspectorSession,
        method: &str,
        params: serde_json::Value,
    ) -> Result<serde_json::Value, String> {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);

        let msg = serde_json::json!({
            "id": id,
            "method": method,
            "params": params,
        });
        let msg_str = msg.to_string();

        // Dispatch to V8 Inspector
        let view = v8::inspector::StringView::from(msg_str.as_bytes() as &[u8]);
        session.dispatch_protocol_message(view);

        // Collect response from outgoing queue
        self.drain_response(id)
    }

    /// Drain the outgoing queue looking for a response with the given id.
    /// Also processes notifications (events) along the way.
    fn drain_response(&self, target_id: u32) -> Result<serde_json::Value, String> {
        let mut state = lock_channel_state(&self.channel_state);
        // V8 Inspector processes messages synchronously in dispatch_protocol_message,
        // so the response should already be in the outgoing queue.
        let mut found_response: Option<String> = None;

        let mut remaining = Vec::new();
        for msg in state.outgoing.drain(..) {
            match &msg {
                InspectorMessage::Response { call_id, message } => {
                    if *call_id == target_id as i32 {
                        found_response = Some(message.clone());
                    } else {
                        remaining.push(msg);
                    }
                }
                InspectorMessage::Notification { message: _ } => {
                    // Check for Debugger.paused event
                    // We'll handle it outside the lock
                    remaining.push(msg);
                }
            }
        }
        state.outgoing = remaining;
        drop(state);

        match found_response {
            Some(json_str) => {
                serde_json::from_str(&json_str)
                    .map_err(|e| format!("Failed to parse CDP response: {e}"))
            }
            None => Err(format!("No response received for CDP message id={target_id}")),
        }
    }

    /// Process any pending notifications (events) from V8 Inspector.
    /// Call this after operations that may trigger events (like resume/step).
    /// Returns true if a Debugger.paused event was found.
    pub fn process_events(&mut self) -> bool {
        let mut state = lock_channel_state(&self.channel_state);
        let mut paused = false;
        let mut remaining = Vec::new();

        for msg in state.outgoing.drain(..) {
            match &msg {
                InspectorMessage::Notification { message } => {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(message) {
                        if parsed.get("method").and_then(|m| m.as_str()) == Some("Debugger.paused") {
                            self.last_paused_frames = parsed.get("params")
                                .and_then(|p| p.get("callFrames"))
                                .cloned();
                            self.is_paused = true;
                            paused = true;
                        } else if parsed.get("method").and_then(|m| m.as_str()) == Some("Debugger.resumed") {
                            self.is_paused = false;
                            self.last_paused_frames = None;
                        }
                    }
                    // Don't keep notifications (they're consumed)
                }
                _ => remaining.push(msg),
            }
        }
        state.outgoing = remaining;
        paused
    }

    // ─── High-level CDP operations ───────────────────────────────────────────

    /// Set a breakpoint by URL pattern.
    /// Returns breakpointId on success.
    pub fn set_breakpoint_by_url(
        &self,
        session: &v8::inspector::V8InspectorSession,
        url: &str,
        line: u32,
        column: Option<u32>,
        condition: Option<&str>,
    ) -> Result<String, String> {
        let mut params = serde_json::json!({
            "url": url,
            "lineNumber": line,
        });
        if let Some(col) = column {
            params["columnNumber"] = serde_json::json!(col);
        }
        if let Some(cond) = condition {
            params["condition"] = serde_json::json!(cond);
        }

        let response = self.send_and_wait(session, "Debugger.setBreakpointByUrl", params)?;
        response
            .get("result")
            .and_then(|r| r.get("breakpointId"))
            .and_then(|id| id.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| format!("No breakpointId in response: {response}"))
    }

    /// Remove a breakpoint by id.
    pub fn remove_breakpoint(
        &self,
        session: &v8::inspector::V8InspectorSession,
        breakpoint_id: &str,
    ) -> Result<(), String> {
        let params = serde_json::json!({ "breakpointId": breakpoint_id });
        self.send_and_wait(session, "Debugger.removeBreakpoint", params)?;
        Ok(())
    }

    /// Evaluate an expression on a specific call frame (while paused).
    pub fn evaluate_on_call_frame(
        &self,
        session: &v8::inspector::V8InspectorSession,
        call_frame_id: &str,
        expression: &str,
    ) -> Result<serde_json::Value, String> {
        let params = serde_json::json!({
            "callFrameId": call_frame_id,
            "expression": expression,
            "returnByValue": true,
        });
        let response = self.send_and_wait(session, "Debugger.evaluateOnCallFrame", params)?;
        Ok(response
            .get("result")
            .and_then(|r| r.get("result"))
            .cloned()
            .unwrap_or(serde_json::Value::Null))
    }

    /// Resume execution.
    pub fn resume(
        &self,
        session: &v8::inspector::V8InspectorSession,
    ) -> Result<(), String> {
        self.send_and_wait(session, "Debugger.resume", serde_json::json!({}))?;
        Ok(())
    }

    /// Step over.
    pub fn step_over(
        &self,
        session: &v8::inspector::V8InspectorSession,
    ) -> Result<(), String> {
        self.send_and_wait(session, "Debugger.stepOver", serde_json::json!({}))?;
        Ok(())
    }

    /// Step into.
    pub fn step_into(
        &self,
        session: &v8::inspector::V8InspectorSession,
    ) -> Result<(), String> {
        self.send_and_wait(session, "Debugger.stepInto", serde_json::json!({}))?;
        Ok(())
    }

    /// Step out.
    pub fn step_out(
        &self,
        session: &v8::inspector::V8InspectorSession,
    ) -> Result<(), String> {
        self.send_and_wait(session, "Debugger.stepOut", serde_json::json!({}))?;
        Ok(())
    }

    /// Get the call frames from the last Debugger.paused event.
    pub fn get_call_frames(&self) -> Option<&serde_json::Value> {
        self.last_paused_frames.as_ref()
    }

    /// Get properties of a remote object (e.g. scope object).
    /// Uses Runtime.getProperties CDP method.
    pub fn get_properties(
        &mut self,
        session: &v8::inspector::V8InspectorSession,
        object_id: &str,
        own_properties: bool,
    ) -> Result<serde_json::Value, String> {
        let params = serde_json::json!({
            "objectId": object_id,
            "ownProperties": own_properties,
            "generatePreview": false,
        });
        let response = self.send_and_wait(session, "Runtime.getProperties", params)?;
        Ok(response
            .get("result")
            .cloned()
            .unwrap_or(serde_json::Value::Null))
    }
}
