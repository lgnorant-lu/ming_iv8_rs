//! iv8-core: V8 embedded browser host engine
//!
//! This crate provides the core Rust API for creating and managing
//! JavaScript contexts with full browser surface emulation.

// V8 callback code extensively uses `expect("key")` for string creation.
// These are safe because v8::String::new only fails on OOM, which is
// handled by V8's OOM handler. We allow these patterns in this crate.
#![allow(clippy::expect_used)]
// unwrap_used: V8 API returns Option in many places where None is unreachable
// in practice (e.g., after is_object() check). Allow for now.
#![allow(clippy::unwrap_used)]

pub mod config;
pub mod convert;
pub mod canvas;
pub mod crypto;
pub mod dom;
pub mod env_inject;
pub mod error;
pub mod events;
pub mod expose;
pub mod inspector;
pub mod kernel;
pub mod network;
pub mod shims;
pub mod state;
pub mod v8_init;
#[macro_use]
pub mod safe_callback;

pub use config::EnvironmentMap;
pub use convert::{v8_to_rust_impl, RustValue};
pub use error::IV8Error;
pub use kernel::embedded_v8::EmbeddedV8Kernel;
pub use kernel::{EvalOpts, KernelConfig};
