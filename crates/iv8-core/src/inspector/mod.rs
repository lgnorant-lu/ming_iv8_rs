//! V8 Inspector integration — CDP (Chrome DevTools Protocol) over WebSocket.
//!
//! Provides:
//! - `InspectorSession`: manages a V8 Inspector + WebSocket server
//! - `watch_apis`: auto-breakpoint when specified APIs are accessed
//! - `vdebugger` statement support (replaces native `debugger`)
//!
//! Architecture:
//!   V8 Isolate ← V8Inspector ← V8InspectorSession ← Channel (WebSocket)
//!
//! The WebSocket server runs in a background thread. CDP messages are
//! exchanged between the DevTools frontend and V8 via the Channel.

pub mod cdp_client;
pub mod channel;
pub mod server;
pub mod session;

pub use cdp_client::CdpClient;
pub use session::InspectorSession;
