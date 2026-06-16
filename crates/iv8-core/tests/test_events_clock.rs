#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]

//! Integration tests for DateInterceptor (Task 32).
//! Verifies Date.now() and performance.now() return logical time.

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};

fn make_kernel() -> EmbeddedV8Kernel {
    EmbeddedV8Kernel::new(KernelConfig::default()).unwrap()
}

fn as_f64(v: &RustValue) -> f64 {
    match v {
        RustValue::Int(i) => *i as f64,
        RustValue::Float(f) => *f,
        other => panic!("expected numeric, got: {:?}", other),
    }
}

#[test]
fn date_now_returns_epoch_at_start() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("Date.now()");
    let now = as_f64(&result);
    // Should be epoch (2024-01-01) since eventLoop time is 0
    // Epoch = 1704067200000
    assert!(
        (now - 1704067200000.0).abs() < 1.0,
        "Date.now() should be epoch at start, got: {}",
        now
    );
}

#[test]
fn date_now_advances_with_event_loop() {
    let mut kernel = make_kernel();
    let before = as_f64(&kernel.eval_to_rust_value("Date.now()"));
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(1000)"); // advance 1 second
    let after = as_f64(&kernel.eval_to_rust_value("Date.now()"));
    let diff = after - before;
    assert!(
        (diff - 1000.0).abs() < 1.0,
        "Date.now() should advance by 1000ms, diff was: {}",
        diff
    );
}

#[test]
fn performance_now_starts_at_zero() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("performance.now()");
    let now = as_f64(&result);
    assert!(
        now.abs() < 1.0,
        "performance.now() should start near 0, got: {}",
        now
    );
}

#[test]
fn performance_now_advances_with_event_loop() {
    let mut kernel = make_kernel();
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(500)");
    let result = kernel.eval_to_rust_value("performance.now()");
    let now = as_f64(&result);
    assert!(
        (now - 500.0).abs() < 1.0,
        "performance.now() should be ~500 after advance(500), got: {}",
        now
    );
}

#[test]
fn new_date_uses_logical_time() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("new Date().getTime()");
    let time = as_f64(&result);
    // Should be epoch (2024-01-01) since eventLoop time is 0
    assert!(
        (time - 1704067200000.0).abs() < 1.0,
        "new Date().getTime() should be epoch, got: {}",
        time
    );
}

#[test]
fn new_date_with_arg_uses_arg() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("new Date(0).getTime()");
    let time = as_f64(&result);
    assert_eq!(time, 0.0, "new Date(0) should be Unix epoch");
}

#[test]
fn date_now_deterministic() {
    // Two calls without advancing should return the same value
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("Date.now() === Date.now()");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn performance_now_deterministic() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("performance.now() === performance.now()");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn date_now_type_is_number() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("typeof Date.now()");
    assert_eq!(result, RustValue::String("number".into()));
}

#[test]
fn performance_now_type_is_number() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("typeof performance.now()");
    assert_eq!(result, RustValue::String("number".into()));
}
