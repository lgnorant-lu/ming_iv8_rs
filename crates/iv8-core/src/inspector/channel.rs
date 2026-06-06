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
