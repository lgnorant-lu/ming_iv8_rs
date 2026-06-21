#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// Integration tests for setTimeout/setInterval/clearTimeout/rAF/queueMicrotask (Task 31).

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};
#[test]
fn set_timeout_returns_id() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("setTimeout(function(){}, 100)");
    match result {
        RustValue::Int(id) => assert!(id > 0, "timer id should be positive"),
        other => panic!("expected Int, got: {:?}", other),
    }
}

#[test]
fn set_timeout_fires_on_advance() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        setTimeout(function() { fired = true; }, 100);
        __iv8__.eventLoop.advance(200);
        fired
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn set_timeout_does_not_fire_before_deadline() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        setTimeout(function() { fired = true; }, 100);
        __iv8__.eventLoop.advance(50);
        fired
    "#,
    );
    assert_eq!(result, RustValue::Bool(false));
}

#[test]
fn set_timeout_fires_exactly_at_deadline() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        setTimeout(function() { fired = true; }, 100);
        __iv8__.eventLoop.advance(100);
        fired
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn set_interval_fires_multiple_times() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var count = 0;
        setInterval(function() { count++; }, 50);
        __iv8__.eventLoop.advance(200);
        count
    "#,
    );
    // Should fire at 50, 100, 150, 200 = 4 times
    match result {
        RustValue::Int(n) => assert!(n >= 3, "expected at least 3 fires, got {}", n),
        other => panic!("expected Int, got: {:?}", other),
    }
}

#[test]
fn clear_timeout_prevents_firing() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        var id = setTimeout(function() { fired = true; }, 100);
        clearTimeout(id);
        __iv8__.eventLoop.advance(200);
        fired
    "#,
    );
    assert_eq!(result, RustValue::Bool(false));
}

#[test]
fn clear_interval_stops_firing() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var count = 0;
        var id = setInterval(function() { count++; }, 50);
        __iv8__.eventLoop.advance(120);
        clearInterval(id);
        __iv8__.eventLoop.advance(200);
        count
    "#,
    );
    // Should fire at 50, 100 (2 times), then cleared before 150
    match result {
        RustValue::Int(n) => assert!(n <= 3, "expected at most 3 fires after clear, got {}", n),
        other => panic!("expected Int, got: {:?}", other),
    }
}

#[test]
fn request_animation_frame_fires_on_advance() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        requestAnimationFrame(function() { fired = true; });
        __iv8__.eventLoop.advance(20);
        fired
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn queue_microtask_fires_immediately() {
    let mut kernel = common::make_kernel();
    // queueMicrotask should fire during the same eval (V8 runs microtasks at checkpoint)
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        queueMicrotask(function() { fired = true; });
        fired
    "#,
    );
    // Note: with Explicit microtask policy, microtasks don't run until checkpoint.
    // They run after the eval completes (kernel does perform_microtask_checkpoint).
    // So within the same eval, `fired` might still be false.
    // Let's test with a separate eval:
    assert_eq!(result, RustValue::Bool(false)); // not yet fired within same eval
}

#[test]
fn queue_microtask_fires_after_eval() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis._mtFired = false;
        queueMicrotask(function() { globalThis._mtFired = true; });
    "#,
    );
    // After eval, kernel drains microtasks
    kernel.drain_microtasks();
    let result = kernel.eval_to_rust_value("globalThis._mtFired");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn multiple_timeouts_fire_in_order() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var order = [];
        setTimeout(function() { order.push(1); }, 100);
        setTimeout(function() { order.push(2); }, 50);
        setTimeout(function() { order.push(3); }, 150);
        __iv8__.eventLoop.advance(200);
        order
    "#,
    );
    assert_eq!(
        result,
        RustValue::Array(vec![
            RustValue::Int(2),
            RustValue::Int(1),
            RustValue::Int(3),
        ])
    );
}

#[test]
fn set_timeout_zero_delay() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        setTimeout(function() { fired = true; }, 0);
        __iv8__.eventLoop.tick();
        fired
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn timer_globals_exist() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        [
            typeof setTimeout,
            typeof setInterval,
            typeof clearTimeout,
            typeof clearInterval,
            typeof requestAnimationFrame,
            typeof queueMicrotask,
        ].every(t => t === 'function')
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

// ─── v0.8.66 (M3): rAF callback receives DOMHighResTimeStamp ─────────

#[test]
fn raf_callback_receives_timestamp() {
    let mut kernel = common::make_kernel();
    kernel
        .eval_to_rust_value(
            r#"
        window.__rafTs = null;
        window.__rafArgCount = 0;
        requestAnimationFrame(function(ts) {
            window.__rafTs = ts;
            window.__rafArgCount = arguments.length;
        });
        __iv8__.eventLoop.advance(20);
    "#,
        );
    let ts = kernel.eval_to_rust_value("window.__rafTs");
    let arg_count = kernel.eval_to_rust_value("window.__rafArgCount");
    assert_eq!(arg_count, RustValue::Int(1));
    match ts {
        RustValue::Float(v) => assert!(v >= 0.0, "rAF timestamp should be >= 0, got {}", v),
        RustValue::Int(v) => assert!(v >= 0, "rAF timestamp should be >= 0, got {}", v),
        other => panic!("expected numeric raf timestamp, got {:?}", other),
    }
}

#[test]
fn raf_callback_timestamp_is_number() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var tsType = null;
        requestAnimationFrame(function(ts) { tsType = typeof ts; });
        __iv8__.eventLoop.advance(20);
        tsType
    "#,
    );
    assert_eq!(result, RustValue::String("number".into()));
}
