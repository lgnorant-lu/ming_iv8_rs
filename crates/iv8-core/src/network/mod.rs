//! Network subsystem: ResourceBundle for offline resource serving.
//!
//! Current runtime: ResourceBundle -> optional Python callback -> NetworkError.
//! Real HTTP backends such as reqwest remain deferred design items.

pub mod bundle;
pub mod fetch;
pub mod xhr;

pub use bundle::{Resource, ResourceBundle};
