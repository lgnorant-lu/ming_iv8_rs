//! iv8-core: V8 embedded browser host engine
//!
//! This crate provides the core Rust API for creating and managing
//! JavaScript contexts with full browser surface emulation.

pub mod config;
pub mod convert;
pub mod canvas;
pub mod crypto;
pub mod dom;
pub mod entry;
pub mod env_inject;
pub mod error;
pub mod events;
pub mod expose;
pub mod inspector;
pub mod kernel;
pub mod network;
pub mod shims;
pub mod state;
pub mod v8_extra;
pub mod v8_init;
pub mod v8_utils;
#[macro_use]
pub mod safe_callback;

pub use config::EnvironmentMap;
pub use convert::{v8_to_rust_impl, RustValue};
pub use error::IV8Error;
pub use kernel::embedded_v8::EmbeddedV8Kernel;
pub use kernel::{EvalOpts, KernelConfig};
