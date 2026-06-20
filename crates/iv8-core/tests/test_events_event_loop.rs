#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// Integration tests for EventLoop V8 bindings (Task 30).

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};
/// Helper: extract numeric value from RustValue (Int or Float).
fn as_f64(v: &RustValue) -> f64 {
    match v {
        RustValue::Int(i) => *i as f64,
        RustValue::Float(f) => *f,
        other => panic!("expected numeric, got: {:?}", other),
    }
}

#[test]
fn event_loop_exists() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("typeof __iv8__.eventLoop");
    assert_eq!(result, RustValue::String("object".into()));
}

#[test]
fn event_loop_get_time_initial() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    assert_eq!(as_f64(&result), 0.0);
}

#[test]
fn event_loop_advance_updates_time() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(100)");
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    assert_eq!(as_f64(&result), 100.0);
}

#[test]
fn event_loop_sleep_updates_time() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value("__iv8__.eventLoop.sleep(50)");
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    assert_eq!(as_f64(&result), 50.0);
}

#[test]
fn event_loop_tick_advances_by_step() {
    let mut kernel = common::make_kernel();
    // Default step is 4ms (4000μs)
    kernel.eval_to_rust_value("__iv8__.eventLoop.tick()");
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    assert_eq!(as_f64(&result), 4.0);
}

#[test]
fn event_loop_tick_with_explicit_ms() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value("__iv8__.eventLoop.tick(10)");
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    assert_eq!(as_f64(&result), 10.0);
}

#[test]
fn event_loop_reset() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(500)");
    kernel.eval_to_rust_value("__iv8__.eventLoop.reset()");
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    assert_eq!(as_f64(&result), 0.0);
}

#[test]
fn event_loop_set_auto_advance_step() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value("__iv8__.eventLoop.setAutoAdvanceStep(1000)"); // 1ms = 1000μs
    kernel.eval_to_rust_value("__iv8__.eventLoop.tick()");
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    assert_eq!(as_f64(&result), 1.0);
}

#[test]
fn event_loop_advance_cumulative() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(100)");
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(200)");
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    assert_eq!(as_f64(&result), 300.0);
}

#[test]
fn event_loop_methods_are_functions() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        [
            typeof __iv8__.eventLoop.advance,
            typeof __iv8__.eventLoop.sleep,
            typeof __iv8__.eventLoop.tick,
            typeof __iv8__.eventLoop.drain,
            typeof __iv8__.eventLoop.drainMicrotasks,
            typeof __iv8__.eventLoop.drainTimers,
            typeof __iv8__.eventLoop.getTime,
            typeof __iv8__.eventLoop.reset,
            typeof __iv8__.eventLoop.setAutoAdvanceStep,
        ].every(t => t === 'function')
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}
