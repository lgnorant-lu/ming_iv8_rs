//! Browser API shims: atob/btoa, URL, location, TextEncoder, etc.
//!
//! These are standard Web APIs that V8 doesn't provide natively.

pub mod atob_btoa;
pub mod audio_context;
pub mod browser_profile;
pub mod console;
pub mod cssom;
pub mod document_props;
pub mod event_constructors;
pub mod geometry;
pub mod location;
pub mod message_channel;
pub mod native_env;
pub mod navigator_extras;
pub mod probes;
pub mod storage;
pub mod structured_clone;
pub mod url;
pub mod user_agent_data;
pub mod window_extras;
pub mod window_properties;
pub mod worker;
