//! iv8-undetect: Anti-detection utilities for iv8-rs.
#![cfg_attr(
    test,
    allow(
        clippy::expect_used,
        clippy::panic,
        clippy::unwrap_used,
        unused_imports,
        unused_variables
    )
)]
//!
//! Provides MarkAsUndetectable, wrapNative, hookNative,
//! window.chrome installation, and navigator/screen/window field injection.

pub mod hook_native;
pub mod undetectable;
pub mod window_chrome;
pub mod wrap_native;
