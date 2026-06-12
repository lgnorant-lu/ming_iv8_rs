//! Browser API stubs for fingerprint bitmask detection.
//!
//! Adds navigator/window properties that anti-fingerprint systems check via
//! `'X' in navigator` or `typeof window.X`. Each stub is minimal but
//! sufficient to pass existence checks and basic method calls.
//!
//! Also fixes:
//! - performance.now() returning 0 in time_freeze mode
//! - navigator.webdriver property descriptor (data vs accessor)
//! - window.matchMedia (returns MediaQueryList stub)
//! - window.customElements (define/get/whenDefined)

pub const BROWSER_APIS_JS: &str = include_str!("browser_apis.js");
