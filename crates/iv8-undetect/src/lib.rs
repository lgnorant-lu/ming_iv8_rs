//! iv8-undetect: Anti-detection utilities for iv8-rs.
//!
//! Provides MarkAsUndetectable, wrapNative, hookNative,
//! window.chrome installation, and navigator/screen/window field injection.

pub mod hook_native;
pub mod undetectable;
pub mod window_chrome;
pub mod wrap_native;
