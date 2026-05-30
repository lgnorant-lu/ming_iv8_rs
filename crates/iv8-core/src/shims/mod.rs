//! Browser API shims: atob/btoa, URL, location, TextEncoder, etc.
//!
//! These are standard Web APIs that V8 doesn't provide natively.

pub mod atob_btoa;
pub mod location;
pub mod event_constructors;
pub mod geometry;
pub mod url;
pub mod message_channel;
pub mod document_props;
pub mod storage;
pub mod navigator_extras;
pub mod dom_prototypes;
pub mod element_prototypes;
pub mod tier1_stubs;
pub mod console;
pub mod native_env;
