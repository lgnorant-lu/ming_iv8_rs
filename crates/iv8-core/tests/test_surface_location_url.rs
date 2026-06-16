#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]

//! Integration tests for location object (Task 67).
//! Acceptance criteria:
//! - location.href returns full URL
//! - location.origin/protocol/host/hostname/pathname/search/hash correct
//! - Initialized from environment
//! - toString() returns href

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};
use std::collections::HashMap;

fn make_kernel_with_url() -> EmbeddedV8Kernel {
    let mut overrides = HashMap::new();
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

#[test]
fn location_exists() {
    let mut kernel = make_kernel_with_url();
    assert_eq!(
        kernel.eval_to_rust_value("typeof location"),
        RustValue::String("object".into())
    );
}

#[test]
fn location_href() {
    let mut kernel = make_kernel_with_url();
    assert_eq!(
        kernel.eval_to_rust_value("location.href"),
        RustValue::String("https://www.example.com:8080/path/page?q=1&r=2#section".into())
    );
}

#[test]
fn location_origin() {
    let mut kernel = make_kernel_with_url();
    assert_eq!(
        kernel.eval_to_rust_value("location.origin"),
        RustValue::String("https://www.example.com:8080".into())
    );
}

#[test]
fn location_protocol() {
    let mut kernel = make_kernel_with_url();
    assert_eq!(
        kernel.eval_to_rust_value("location.protocol"),
        RustValue::String("https:".into())
    );
}

#[test]
fn location_host() {
    let mut kernel = make_kernel_with_url();
    assert_eq!(
        kernel.eval_to_rust_value("location.host"),
        RustValue::String("www.example.com:8080".into())
    );
}

#[test]
fn location_hostname() {
    let mut kernel = make_kernel_with_url();
    assert_eq!(
        kernel.eval_to_rust_value("location.hostname"),
        RustValue::String("www.example.com".into())
    );
}

#[test]
fn location_port() {
    let mut kernel = make_kernel_with_url();
    assert_eq!(
        kernel.eval_to_rust_value("location.port"),
        RustValue::String("8080".into())
    );
}

#[test]
fn location_pathname() {
    let mut kernel = make_kernel_with_url();
    assert_eq!(
        kernel.eval_to_rust_value("location.pathname"),
        RustValue::String("/path/page".into())
    );
}

#[test]
fn location_search() {
    let mut kernel = make_kernel_with_url();
    assert_eq!(
        kernel.eval_to_rust_value("location.search"),
        RustValue::String("?q=1&r=2".into())
    );
}

#[test]
fn location_hash() {
    let mut kernel = make_kernel_with_url();
    assert_eq!(
        kernel.eval_to_rust_value("location.hash"),
        RustValue::String("#section".into())
    );
}

#[test]
fn location_to_string() {
    let mut kernel = make_kernel_with_url();
    assert_eq!(
        kernel.eval_to_rust_value("location.toString()"),
        RustValue::String("https://www.example.com:8080/path/page?q=1&r=2#section".into())
    );
}

#[test]
fn location_assign_no_crash() {
    let mut kernel = make_kernel_with_url();
    // assign is a no-op in offline mode, should not crash
    kernel.eval_to_rust_value("location.assign('https://other.com')");
}

#[test]
fn location_default_about_blank() {
    // Without environment override, location.href defaults to about:blank
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    assert_eq!(
        kernel.eval_to_rust_value("location.href"),
        RustValue::String("about:blank".into())
    );
}

#[test]
fn window_location_same_as_location() {
    let mut kernel = make_kernel_with_url();
    assert_eq!(
        kernel.eval_to_rust_value("window.location === location"),
        RustValue::Bool(true)
    );
}
