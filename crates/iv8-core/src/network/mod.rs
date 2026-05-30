//! Network subsystem: ResourceBundle for offline resource serving.
//!
//! v0.1: Only ResourceBundle (pre-registered responses).
//! v0.2+: Python callback handler + reqwest real HTTP.

pub mod bundle;
pub mod fetch;
pub mod xhr;

pub use bundle::{Resource, ResourceBundle};
