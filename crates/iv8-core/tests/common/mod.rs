//! Shared test harness helpers for iv8-core integration tests.
//!
//! This module is NOT a test crate. It provides utility functions used by
//! all integration test files under `tests/`.
//!
//! Usage from any test file in `crates/iv8-core/tests/`:
//! ```ignore
//! mod common;
//! use common::*;
//! ```

#![allow(dead_code)]

use iv8_core::convert::RustValue;
use iv8_core::kernel::embedded_v8::EmbeddedV8Kernel;
use iv8_core::kernel::KernelConfig;

/// Create a kernel with default configuration.
pub fn make_kernel() -> EmbeddedV8Kernel {
    EmbeddedV8Kernel::new(KernelConfig::default()).unwrap()
}

/// Create a kernel with a fixed random seed for deterministic tests.
pub fn make_kernel_seeded(seed: u64) -> EmbeddedV8Kernel {
    let mut cfg = KernelConfig::default();
    cfg.random_seed = Some(seed);
    EmbeddedV8Kernel::new(cfg).unwrap()
}

/// Extract a Rust string from a RustValue for assertion comparison.
pub fn to_str(v: &RustValue) -> String {
    match v {
        RustValue::String(s) => s.clone(),
        RustValue::Null => "null".to_string(),
        RustValue::Bool(b) => b.to_string(),
        RustValue::Int(n) => n.to_string(),
        RustValue::Float(f) => f.to_string(),
        RustValue::JsObject(s) => s.clone(),
        other => format!("{:?}", other),
    }
}

/// Assert that a JS expression evaluates to the expected Rust string.
pub fn assert_js_str(kernel: &mut EmbeddedV8Kernel, js: &str, expected: &str) {
    let val = to_str(&kernel.eval_to_rust_value(js));
    assert_eq!(val, expected, "for expr: {}", js);
}

/// Assert that a JS expression evaluates to the expected RustValue.
pub fn assert_js_val(kernel: &mut EmbeddedV8Kernel, js: &str, expected: RustValue) {
    let val = kernel.eval_to_rust_value(js);
    assert_eq!(val, expected, "for expr: {}", js);
}

/// Assert that a JS expression throws (returns Null on evaluation failure).
pub fn assert_js_error(kernel: &mut EmbeddedV8Kernel, js: &str) {
    let result = kernel.eval_to_rust_value(js);
    assert_eq!(
        result,
        RustValue::Null,
        "expected error for expr: {}, got: {:?}",
        js,
        result
    );
}
