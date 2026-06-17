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

/// Create a kernel with a pre-loaded HTML document.
pub fn make_kernel_with_doc(html: &str) -> EmbeddedV8Kernel {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    kernel.set_document(html, None);
    kernel
}

/// Create a kernel with location URL overrides for URL parsing tests.
pub fn make_kernel_with_url() -> EmbeddedV8Kernel {
    let mut overrides = std::collections::HashMap::new();
    overrides.insert(
        "location.href".to_string(),
        serde_json::json!("https://www.example.com:8080/path/page?q=1&r=2#section"),
    );
    overrides.insert(
        "location.origin".to_string(),
        serde_json::json!("https://www.example.com:8080"),
    );
    overrides.insert("location.protocol".to_string(), serde_json::json!("https:"));
    overrides.insert(
        "location.host".to_string(),
        serde_json::json!("www.example.com:8080"),
    );
    overrides.insert(
        "location.hostname".to_string(),
        serde_json::json!("www.example.com"),
    );
    overrides.insert("location.port".to_string(), serde_json::json!("8080"));
    overrides.insert(
        "location.pathname".to_string(),
        serde_json::json!("/path/page"),
    );
    overrides.insert("location.search".to_string(), serde_json::json!("?q=1&r=2"));
    overrides.insert("location.hash".to_string(), serde_json::json!("#section"));

    let config = KernelConfig {
        environment_overrides: Some(overrides),
        ..Default::default()
    };
    EmbeddedV8Kernel::new(config).unwrap()
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
