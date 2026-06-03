//! V8 Inspector Session — integrates V8 Inspector with the kernel.

use std::time::Duration;
use std::thread;

use crate::inspector::channel::{InspectorMessage, SharedChannelState, lock_channel_state};

/// Configuration for the inspector session.
pub struct InspectorConfig {
    pub port: u16,
    pub watch_apis: Vec<String>,
    pub enable_console: bool,
}

impl Default for InspectorConfig {
    fn default() -> Self {
        Self {
            port: 9229,
            watch_apis: Vec::new(),
            enable_console: true,
        }
    }
}

/// An active inspector session.
pub struct InspectorSession {
    pub channel_state: SharedChannelState,
    pub devtools_url: String,
    pub config: InspectorConfig,
    inspector: Option<v8::inspector::V8Inspector>,
    session: Option<v8::inspector::V8InspectorSession>,
}

impl InspectorSession {
    pub fn new(config: InspectorConfig) -> Self {
        let (channel_state, devtools_url) = crate::inspector::server::start_server(config.port);
        Self {
            channel_state,
            devtools_url,
            config,
            inspector: None,
            session: None,
        }
    }

    /// Set the inspector and session after external creation.
    pub fn set_inspector(
        &mut self,
        inspector: v8::inspector::V8Inspector,
        session: v8::inspector::V8InspectorSession,
    ) {
        self.inspector = Some(inspector);
        self.session = Some(session);
        tracing::info!("V8 Inspector initialized on port {}", self.config.port);
    }

    /// Get a reference to the underlying V8InspectorSession (for CDP client).
    pub fn session_ref(&self) -> Option<&v8::inspector::V8InspectorSession> {
        self.session.as_ref()
    }

    /// Process pending CDP messages from DevTools client.
    pub fn process_messages(&mut self) {
        let incoming: Vec<String> = {
            let mut state = lock_channel_state(&self.channel_state);
            state.incoming.drain(..).collect()
        };

        if let Some(ref session) = self.session {
            for msg in incoming {
                let view = v8::inspector::StringView::from(msg.as_bytes() as &[u8]);
                session.dispatch_protocol_message(view);
            }
        }
    }

    /// Wait for DevTools to connect (blocks until connected or timeout).
    pub fn wait_for_connection(&self, timeout_ms: u64) {
        let deadline = std::time::Instant::now() + Duration::from_millis(timeout_ms);
        loop {
            if lock_channel_state(&self.channel_state).connected {
                tracing::info!("DevTools client connected");
                break;
            }
            if std::time::Instant::now() >= deadline {
                tracing::warn!("DevTools connection timeout after {}ms", timeout_ms);
                break;
            }
            thread::sleep(Duration::from_millis(50));
        }
    }

    /// Schedule a pause on the next JS statement (breakpoint).
    pub fn schedule_pause(&self, reason: &str) {
        if let Some(ref session) = self.session {
            let reason_bytes = reason.as_bytes();
            let reason_view = v8::inspector::StringView::from(reason_bytes);
            let detail_view = v8::inspector::StringView::from(b"{}" as &[u8]);
            session.schedule_pause_on_next_statement(reason_view, detail_view);
        }
    }

    /// Install vdebugger global variable (triggers breakpoint when accessed).
    /// Returns the JS source to eval.
    pub fn vdebugger_js() -> &'static str {
        r#"
(function() {
    Object.defineProperty(globalThis, 'vdebugger', {
        get: function() {
            debugger;
            return undefined;
        },
        configurable: true,
        enumerable: false,
    });
})();
"#
    }

    /// Returns the JS source for watch_apis breakpoints.
    pub fn watch_apis_js(&self) -> Option<String> {
        if self.config.watch_apis.is_empty() {
            return None;
        }

        let apis_json = serde_json::to_string(&self.config.watch_apis)
            .unwrap_or_else(|_| "[]".to_string());

        Some(format!(r#"
(function() {{
    var watchApis = {apis_json};
    watchApis.forEach(function(path) {{
        var parts = path.split('.');
        if (parts.length < 2) return;
        var obj = globalThis;
        for (var i = 0; i < parts.length - 1; i++) {{
            if (!obj || typeof obj[parts[i]] === 'undefined') return;
            obj = obj[parts[i]];
        }}
        var prop = parts[parts.length - 1];
        var origDesc = Object.getOwnPropertyDescriptor(obj, prop);
        if (!origDesc) return;
        var origGet = origDesc.get;
        var origValue = origDesc.value;
        Object.defineProperty(obj, prop, {{
            get: function() {{
                vdebugger;
                return origGet ? origGet.call(this) : origValue;
            }},
            configurable: true,
            enumerable: origDesc.enumerable !== false,
        }});
    }});
}})();
"#))
    }
}

// ─── V8InspectorClientImpl ────────────────────────────────────────────────────

pub struct InspectorClientImpl {
    channel_state: SharedChannelState,
}

impl InspectorClientImpl {
    pub fn new(channel_state: SharedChannelState) -> Self {
        Self { channel_state }
    }
}

impl v8::inspector::V8InspectorClientImpl for InspectorClientImpl {
    fn run_message_loop_on_pause(&self, _context_group_id: i32) {
        let mut state = lock_channel_state(&self.channel_state);
        state.paused = true;
    }

    fn quit_message_loop_on_pause(&self) {
        let mut state = lock_channel_state(&self.channel_state);
        state.paused = false;
    }
}

// ─── ChannelImpl ──────────────────────────────────────────────────────────────

pub struct InspectorChannelImpl {
    channel_state: SharedChannelState,
}

impl InspectorChannelImpl {
    pub fn new(channel_state: SharedChannelState) -> Self {
        Self { channel_state }
    }
}

impl v8::inspector::ChannelImpl for InspectorChannelImpl {
    fn send_response(
        &self,
        call_id: i32,
        message: v8::UniquePtr<v8::inspector::StringBuffer>,
    ) {
        if let Some(msg) = message.as_ref() {
            let text = msg.string().to_string();
            let mut state = lock_channel_state(&self.channel_state);
            state.outgoing.push(InspectorMessage::Response { call_id, message: text });
        }
    }

    fn send_notification(&self, message: v8::UniquePtr<v8::inspector::StringBuffer>) {
        if let Some(msg) = message.as_ref() {
            let text = msg.string().to_string();
            let mut state = lock_channel_state(&self.channel_state);
            state.outgoing.push(InspectorMessage::Notification { message: text });
        }
    }

    fn flush_protocol_notifications(&self) {}
}
