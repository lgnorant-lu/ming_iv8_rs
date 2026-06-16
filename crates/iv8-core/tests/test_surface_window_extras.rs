//! v0.8.51: Integration tests for window extras (performance, history).
mod common;

// ── performance ──

#[test]
fn test_performance_now_returns_number() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof performance.now", "function");
    let val = common::to_str(&k.eval_to_rust_value("typeof performance.now()"));
    assert_eq!(val, "number", "performance.now() must return number");
}

#[test]
fn test_performance_time_origin_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof performance.timeOrigin", "number");
}

#[test]
fn test_performance_now_monotonic() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("__iv8__.eventLoop.advance(500)");
    let t0 = common::to_str(&k.eval_to_rust_value("performance.now()"));
    k.eval_to_rust_value("__iv8__.eventLoop.advance(500)");
    let t1 = common::to_str(&k.eval_to_rust_value("performance.now()"));
    let a: f64 = t0.parse().unwrap();
    let b: f64 = t1.parse().unwrap();
    assert!(b >= a, "performance.now() not monotonic: {} then {}", a, b);
}

#[test]
fn test_performance_timing_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof performance.timing", "object");
}

// ── history ──

#[test]
fn test_history_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof history", "object");
}

#[test]
fn test_history_length() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof history.length", "number");
}

#[test]
fn test_history_push_state_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof history.pushState", "function");
}

#[test]
fn test_history_replace_state_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof history.replaceState", "function");
}
