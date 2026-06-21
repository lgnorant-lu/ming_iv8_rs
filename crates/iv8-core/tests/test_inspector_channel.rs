use iv8_core::inspector::channel::{lock_channel_state, ChannelState, InspectorMessage};
use iv8_core::inspector::session::{InspectorConfig, InspectorSession};
use std::sync::{Arc, Mutex};

#[test]
fn test_channel_state_default_empty() {
    let state = ChannelState::new();
    assert!(state.outgoing.is_empty());
    assert!(state.incoming.is_empty());
    assert!(!state.connected);
    assert!(!state.paused);
}

#[test]
fn test_channel_state_connected_flag() {
    let state = Arc::new(Mutex::new(ChannelState::new()));
    {
        let mut s = lock_channel_state(&state);
        s.connected = true;
    }
    assert!(lock_channel_state(&state).connected);
}

#[test]
fn test_channel_state_incoming_queue() {
    let state = Arc::new(Mutex::new(ChannelState::new()));
    {
        let mut s = lock_channel_state(&state);
        s.incoming
            .push(r#"{"id":1,"method":"Debugger.enable"}"#.into());
        s.incoming
            .push(r#"{"id":2,"method":"Runtime.enable"}"#.into());
    }
    let s = lock_channel_state(&state);
    assert_eq!(s.incoming.len(), 2);
}

#[test]
fn test_inspector_session_creates_with_default_config() {
    let config = InspectorConfig::default();
    let session = InspectorSession::new(config);
    assert!(
        session.is_ok(),
        "InspectorSession::new with default config should succeed"
    );
    let session = session.unwrap();
    assert_eq!(session.config.port, 9229);
    assert!(
        session.devtools_url.contains("9229"),
        "devtools_url should reference the port"
    );
}

#[test]
fn test_inspector_config_default_values() {
    let config = InspectorConfig::default();
    assert_eq!(config.port, 9229);
    assert!(config.watch_apis.is_empty());
    assert!(config.enable_console);
}

// ─── CDP client tests (no V8 required for construction and state ops) ───

use iv8_core::inspector::CdpClient;

#[test]
fn test_cdp_client_new_creates_valid_client() {
    let state = Arc::new(Mutex::new(ChannelState::new()));
    let client = CdpClient::new(state);
    assert!(!client.is_paused);
    assert!(client.last_paused_frames.is_none());
}

#[test]
fn test_cdp_client_channel_state_roundtrip() {
    let state = Arc::new(Mutex::new(ChannelState::new()));
    let client = CdpClient::new(state.clone());
    assert!(!client.is_paused);

    let mut s = lock_channel_state(&state);
    s.incoming
        .push(r#"{"id":1,"method":"Debugger.enable","params":{}}"#.into());
    drop(s);

    let s = lock_channel_state(&state);
    assert_eq!(s.incoming.len(), 1);
}

#[test]
fn test_inspector_message_response_variant() {
    let msg = InspectorMessage::Response {
        call_id: 42,
        message: r#"{"id":42,"result":{}}"#.into(),
    };
    match msg {
        InspectorMessage::Response { call_id, message } => {
            assert_eq!(call_id, 42);
            assert!(message.contains("result"));
        }
        _ => panic!("expected Response variant"),
    }
}

#[test]
fn test_inspector_message_notification_variant() {
    let msg = InspectorMessage::Notification {
        message: r#"{"method":"Debugger.paused","params":{}}"#.into(),
    };
    match msg {
        InspectorMessage::Notification { message } => {
            assert!(message.contains("Debugger.paused"));
        }
        _ => panic!("expected Notification variant"),
    }
}

#[test]
fn test_channel_state_paused_flag() {
    let state = Arc::new(Mutex::new(ChannelState::new()));
    {
        let mut s = lock_channel_state(&state);
        s.paused = true;
    }
    assert!(lock_channel_state(&state).paused);
}
