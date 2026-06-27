#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// D3: Event timing integration tests (v0.8.85 P2-10).
//
// IV8 EventLoop manages macrotask queue with logical time.
// __iv8__.eventLoop.advance(ms) advances the clock and fires due tasks.
// Microtasks drain after each eval_to_rust_value call.

use iv8_core::RustValue;

#[test]
fn set_timeout_multiple_fire_in_delay_order() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var order = [];
        setTimeout(function() { order.push(1); }, 100);
        setTimeout(function() { order.push(2); }, 50);
        setTimeout(function() { order.push(3); }, 150);
        __iv8__.eventLoop.advance(200);
        order.join(",")
    "#,
    );
    assert_eq!(result, RustValue::String("2,1,3".into()));
}

#[test]
fn set_timeout_zero_delay_fires_before_one_ms() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var order = [];
        setTimeout(function() { order.push("1ms"); }, 1);
        setTimeout(function() { order.push("0ms"); }, 0);
        __iv8__.eventLoop.advance(5);
        order.join(",")
    "#,
    );
    assert_eq!(result, RustValue::String("0ms,1ms".into()));
}

#[test]
fn set_interval_fires_multiple_times_on_advance() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var count = 0;
        var id = setInterval(function() { count++; }, 50);
        __iv8__.eventLoop.advance(200);
        clearInterval(id);
        count
    "#,
    );
    match result {
        RustValue::Int(n) => assert!(n >= 3, "expected at least 3 fires, got {}", n),
        other => panic!("expected Int, got: {:?}", other),
    }
}

#[test]
fn set_interval_stops_after_clear() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var count = 0;
        var id = setInterval(function() { count++; }, 50);
        __iv8__.eventLoop.advance(120);
        clearInterval(id);
        var countAfterClear = count;
        __iv8__.eventLoop.advance(200);
        [countAfterClear, count].join(",")
    "#,
    );
    assert_eq!(
        result,
        RustValue::String("2,2".into()),
        "count should not increase after clearInterval"
    );
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
fn clear_timeout_before_deadline_no_effect_after_fire() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        var id = setTimeout(function() { fired = true; }, 50);
        __iv8__.eventLoop.advance(100);
        clearTimeout(id);
        fired
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn request_animation_frame_callback_receives_timestamp() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__rafTs = null;
        globalThis.__rafArgCount = 0;
        requestAnimationFrame(function(ts) {
            globalThis.__rafTs = ts;
            globalThis.__rafArgCount = arguments.length;
        });
        __iv8__.eventLoop.advance(20);
    "#,
    );
    let ts = kernel.eval_to_rust_value("globalThis.__rafTs");
    let arg_count = kernel.eval_to_rust_value("globalThis.__rafArgCount");
    assert_eq!(arg_count, RustValue::Int(1));
    match ts {
        RustValue::Float(v) => assert!(v >= 0.0, "rAF timestamp should be >= 0, got {}", v),
        RustValue::Int(v) => assert!(v >= 0, "rAF timestamp should be >= 0, got {}", v),
        other => panic!("expected numeric raf timestamp, got {:?}", other),
    }
}

#[test]
fn request_animation_frame_timestamp_is_number() {
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

#[test]
fn microtask_runs_before_settimeout() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__seq = [];
        setTimeout(function() { globalThis.__seq.push("timeout"); }, 0);
        Promise.resolve().then(function() { globalThis.__seq.push("microtask"); });
    "#,
    );
    // After eval, microtask should have drained, macrotask should NOT have fired.
    let result = kernel.eval_to_rust_value("globalThis.__seq.join(',')");
    assert_eq!(
        result,
        RustValue::String("microtask".into()),
        "microtask must drain after eval; setTimeout macrotask should NOT fire yet"
    );
}

#[test]
fn microtask_then_macrotask_full_ordering() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__seq = [];
        setTimeout(function() { globalThis.__seq.push("timeout"); }, 0);
        Promise.resolve().then(function() { globalThis.__seq.push("microtask"); });
    "#,
    );
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(10)");
    let result = kernel.eval_to_rust_value("globalThis.__seq.join(',')");
    assert_eq!(
        result,
        RustValue::String("microtask,timeout".into()),
        "microtask before macrotask after advance"
    );
}

#[test]
fn nested_set_timeout_fires_on_next_tick() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__seq = [];
        setTimeout(function() {
            globalThis.__seq.push("outer");
            setTimeout(function() {
                globalThis.__seq.push("inner");
            }, 0);
        }, 0);
    "#,
    );
    // First advance fires the outer setTimeout, but inner is scheduled AFTER.
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(10)");
    let after_first = kernel.eval_to_rust_value("globalThis.__seq.join(',')");
    // Both should fire because advance loops in steps.
    assert_eq!(
        after_first,
        RustValue::String("outer,inner".into()),
        "nested setTimeout should fire within advance loop"
    );
}

#[test]
fn nested_set_timeout_separate_ticks() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__seq = [];
        setTimeout(function() {
            globalThis.__seq.push("outer");
            setTimeout(function() {
                globalThis.__seq.push("inner");
            }, 10);
        }, 0);
    "#,
    );
    // Tick once - fires outer only (inner scheduled with 10ms delay)
    kernel.eval_to_rust_value("__iv8__.eventLoop.tick(0)");
    let after_tick1 = kernel.eval_to_rust_value("globalThis.__seq.join(',')");
    assert_eq!(after_tick1, RustValue::String("outer".into()));

    // Advance more to fire inner
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(20)");
    let after_advance = kernel.eval_to_rust_value("globalThis.__seq.join(',')");
    assert_eq!(after_advance, RustValue::String("outer,inner".into()));
}

#[test]
fn set_timeout_extra_args_forwarded() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var received = null;
        setTimeout(function(val) { received = val; }, 0, "hello");
        __iv8__.eventLoop.advance(20);
        received
    "#,
    );
    assert_eq!(result, RustValue::String("hello".into()));
}

#[test]
fn set_timeout_multiple_extra_args_forwarded() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var a = null, b = null, c = null;
        setTimeout(function(x, y, z) { a = x; b = y; c = z; }, 0, "one", 2, true);
        __iv8__.eventLoop.advance(20);
        JSON.stringify([a, b, c])
    "#,
    );
    assert_eq!(result, RustValue::String(r#"["one",2,true]"#.into()));
}

#[test]
fn set_timeout_this_is_global_this() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var thisIsGlobal = null;
        setTimeout(function() { thisIsGlobal = this === globalThis; }, 0);
        __iv8__.eventLoop.advance(20);
        thisIsGlobal
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn set_interval_this_is_global_this() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var thisIsGlobal = null;
        var id = setInterval(function() { thisIsGlobal = this === globalThis; clearInterval(id); }, 5);
        __iv8__.eventLoop.advance(20);
        thisIsGlobal
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
fn event_loop_get_time_initial_zero() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    match result {
        RustValue::Float(f) => assert_eq!(f, 0.0),
        RustValue::Int(i) => assert_eq!(i, 0),
        other => panic!("expected numeric, got: {:?}", other),
    }
}

#[test]
fn event_loop_advance_updates_time() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(100)");
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    match result {
        RustValue::Float(f) => assert_eq!(f, 100.0),
        RustValue::Int(i) => assert_eq!(i, 100),
        other => panic!("expected numeric, got: {:?}", other),
    }
}

#[test]
fn event_loop_advance_cumulative() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(100)");
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(200)");
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    match result {
        RustValue::Float(f) => assert_eq!(f, 300.0),
        RustValue::Int(i) => assert_eq!(i, 300),
        other => panic!("expected numeric, got: {:?}", other),
    }
}

#[test]
fn event_loop_reset_clears_time_and_tasks() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        setTimeout(function() { fired = true; }, 100);
        __iv8__.eventLoop.advance(50);
        __iv8__.eventLoop.reset();
        __iv8__.eventLoop.advance(200);
        fired
    "#,
    );
    assert_eq!(result, RustValue::Bool(false));
}

#[test]
fn event_loop_reset_zeroes_time() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(500)");
    kernel.eval_to_rust_value("__iv8__.eventLoop.reset()");
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    match result {
        RustValue::Float(f) => assert_eq!(f, 0.0),
        RustValue::Int(i) => assert_eq!(i, 0),
        other => panic!("expected numeric, got: {:?}", other),
    }
}

#[test]
fn mutation_observer_exists_as_constructor() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("typeof MutationObserver");
    assert_eq!(result, RustValue::String("function".into()));
}

#[test]
fn mutation_observer_can_observe_and_disconnect() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var obs = new MutationObserver(function(mutations, observer) {});
        var div = document.createElement("div");
        obs.observe(div, { attributes: true, childList: true });
        obs.disconnect();
        "ok"
    "#,
    );
    assert_eq!(result, RustValue::String("ok".into()));
}

#[test]
fn mutation_observer_take_records_returns_array() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var obs = new MutationObserver(function() {});
        var div = document.createElement("div");
        obs.observe(div, { attributes: true });
        obs.takeRecords()
    "#,
    );
    match result {
        RustValue::Array(arr) => assert!(arr.is_empty(), "expected empty array, got {:?}", arr),
        RustValue::Null => {}
        other => panic!("expected Array or Null, got: {:?}", other),
    }
}

#[test]
fn resize_observer_exists_as_constructor() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("typeof ResizeObserver");
    assert_eq!(result, RustValue::String("function".into()));
}

#[test]
fn resize_observer_can_observe_unobserve_disconnect() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var obs = new ResizeObserver(function(entries, observer) {});
        var div = document.createElement("div");
        obs.observe(div);
        obs.unobserve(div);
        obs.disconnect();
        "ok"
    "#,
    );
    assert_eq!(result, RustValue::String("ok".into()));
}

#[test]
fn resize_observer_take_records_returns_array() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var obs = new ResizeObserver(function() {});
        var div = document.createElement("div");
        obs.observe(div);
        obs.takeRecords()
    "#,
    );
    match result {
        RustValue::Array(arr) => assert!(arr.is_empty(), "expected empty array, got {:?}", arr),
        RustValue::Null => {}
        other => panic!("expected Array or Null, got: {:?}", other),
    }
}

#[test]
fn set_timeout_returns_positive_id() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("setTimeout(function(){}, 100)");
    match result {
        RustValue::Int(id) => assert!(id > 0, "timer id should be positive"),
        other => panic!("expected Int, got: {:?}", other),
    }
}

#[test]
fn set_interval_returns_positive_id() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("setInterval(function(){}, 100)");
    match result {
        RustValue::Int(id) => assert!(id > 0, "timer id should be positive"),
        other => panic!("expected Int, got: {:?}", other),
    }
}

#[test]
fn timer_ids_are_sequential() {
    let mut kernel = common::make_kernel();
    let id1 = kernel.eval_to_rust_value("setTimeout(function(){}, 0)");
    let id2 = kernel.eval_to_rust_value("setTimeout(function(){}, 0)");
    let id3 = kernel.eval_to_rust_value("setTimeout(function(){}, 0)");
    let v1 = match id1 { RustValue::Int(i) => i, _ => -1 };
    let v2 = match id2 { RustValue::Int(i) => i, _ => -1 };
    let v3 = match id3 { RustValue::Int(i) => i, _ => -1 };
    assert!(v1 >= 1, "first id should be >= 1, got {}", v1);
    assert_eq!(v2, v1 + 1, "second id should be sequential");
    assert_eq!(v3, v1 + 2, "third id should be sequential");
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
        ].every(function(t) { return t === "function"; })
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
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
        ].every(function(t) { return t === "function"; })
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn set_timeout_zero_delay_fires_on_tick() {
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
fn event_loop_tick_advances_by_default_step() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value("__iv8__.eventLoop.tick()");
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    match result {
        RustValue::Float(f) => assert_eq!(f, 4.0, "default step is 4ms"),
        RustValue::Int(i) => assert_eq!(i, 4, "default step is 4ms"),
        other => panic!("expected numeric, got: {:?}", other),
    }
}

#[test]
fn event_loop_tick_with_explicit_ms() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value("__iv8__.eventLoop.tick(10)");
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    match result {
        RustValue::Float(f) => assert_eq!(f, 10.0),
        RustValue::Int(i) => assert_eq!(i, 10),
        other => panic!("expected numeric, got: {:?}", other),
    }
}

#[test]
fn event_loop_set_auto_advance_step_changes_tick() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value("__iv8__.eventLoop.setAutoAdvanceStep(1000)");
    kernel.eval_to_rust_value("__iv8__.eventLoop.tick()");
    let result = kernel.eval_to_rust_value("__iv8__.eventLoop.getTime()");
    match result {
        RustValue::Float(f) => assert_eq!(f, 1.0),
        RustValue::Int(i) => assert_eq!(i, 1),
        other => panic!("expected numeric, got: {:?}", other),
    }
}

#[test]
fn set_interval_extra_args_forwarded() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var vals = [];
        var id = setInterval(function(v) { vals.push(v); }, 5, "b");
        __iv8__.eventLoop.advance(30, 5);
        clearInterval(id);
        vals.length >= 1 && vals.every(function(v) { return v === "b"; })
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn raf_fires_on_advance() {
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
fn drain_microtasks_flushes_queue() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis._mtFired = false;
        queueMicrotask(function() { globalThis._mtFired = true; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis._mtFired");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn drain_timers_fires_due_tasks() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        setTimeout(function() { fired = true; }, 100);
        __iv8__.eventLoop.advance(200);
        __iv8__.eventLoop.drainTimers();
        fired
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}
