//! V8 Inspector Channel — bridges V8 Inspector messages to WebSocket.
//!
//! The Channel receives CDP messages from V8 and forwards them to
//! connected DevTools clients via WebSocket.

use std::sync::{Arc, Mutex};

/// Messages sent from V8 Inspector to the channel.
#[derive(Debug, Clone)]
pub enum InspectorMessage {
    /// A response to a CDP request.
    Response { call_id: i32, message: String },
    /// A notification (event) from V8.
    Notification { message: String },
}

/// Shared state between the Channel and the WebSocket server.
pub struct ChannelState {
    /// Outgoing messages from V8 → DevTools client.
    pub outgoing: Vec<InspectorMessage>,
    /// Incoming messages from DevTools client → V8.
    pub incoming: Vec<String>,
    /// Whether a DevTools client is connected.
    pub connected: bool,
    /// Whether execution is paused at a breakpoint.
    pub paused: bool,
}

impl Default for ChannelState {
    fn default() -> Self {
        Self::new()
    }
}

impl ChannelState {
    pub fn new() -> Self {
        Self {
            outgoing: Vec::new(),
            incoming: Vec::new(),
            connected: false,
            paused: false,
        }
    }
}

pub type SharedChannelState = Arc<Mutex<ChannelState>>;

pub fn lock_channel_state(state: &SharedChannelState) -> std::sync::MutexGuard<'_, ChannelState> {
    match state.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_state_new_defaults() {
        let state = ChannelState::new();
        assert!(state.outgoing.is_empty());
        assert!(state.incoming.is_empty());
        assert!(!state.connected);
        assert!(!state.paused);
    }

    #[test]
    fn test_channel_state_default_trait() {
        let state = ChannelState::default();
        assert!(!state.connected);
        assert!(!state.paused);
    }

    #[test]
    fn test_inspector_message_response_variant() {
        let msg = InspectorMessage::Response {
            call_id: 42,
            message: "{\"id\":42}".to_string(),
        };
        match msg {
            InspectorMessage::Response { call_id, message } => {
                assert_eq!(call_id, 42);
                assert!(message.contains("42"));
            }
            _ => panic!("expected Response variant"),
        }
    }

    #[test]
    fn test_inspector_message_notification_variant() {
        let msg = InspectorMessage::Notification {
            message: "Debugger.paused".to_string(),
        };
        match msg {
            InspectorMessage::Notification { message } => {
                assert_eq!(message, "Debugger.paused");
            }
            _ => panic!("expected Notification variant"),
        }
    }

    #[test]
    fn test_lock_channel_state_acquires_lock() {
        let state: SharedChannelState = std::sync::Arc::new(std::sync::Mutex::new(ChannelState::new()));
        {
            let mut guard = lock_channel_state(&state);
            guard.connected = true;
        }
        let guard = lock_channel_state(&state);
        assert!(guard.connected);
    }

    #[test]
    fn test_lock_channel_state_recovers_from_poison() {
        let state: SharedChannelState = std::sync::Arc::new(std::sync::Mutex::new(ChannelState::new()));
        // Poison the mutex by panicking while holding the lock
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = state.lock().unwrap();
            panic!("intentional poison");
        }));
        // lock_channel_state should recover from poison
        let guard = lock_channel_state(&state);
        assert!(!guard.connected);
    }
}
