#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// D2: Promise semantics integration tests (v0.8.85 P2-9).
//
// IV8 uses V8's native Promise implementation. Microtasks drain after
// each eval_to_rust_value call (kernel.eval calls perform_microtask_checkpoint).
// Macrotasks (setTimeout) require __iv8__.eventLoop.advance to fire.

use iv8_core::RustValue;

#[test]
fn promise_resolve_then_sets_result_after_microtask_drain() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = 0;
        Promise.resolve(42).then(function(v) { globalThis.__result = v; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::Int(42));
}

#[test]
fn promise_chain_passes_values_through_then() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = 0;
        Promise.resolve(1)
            .then(function(v) { return v + 1; })
            .then(function(v) { return v + 1; })
            .then(function(v) { globalThis.__result = v; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::Int(3));
}

#[test]
fn promise_reject_propagates_to_catch() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__caught = null;
        Promise.reject(new Error("boom"))
            .catch(function(e) { globalThis.__caught = e.message; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__caught");
    assert_eq!(result, RustValue::String("boom".into()));
}

#[test]
fn promise_reject_unhandled_does_not_throw() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        globalThis.__ok = true;
        Promise.reject(new Error("unhandled"));
        "survived"
    "#,
    );
    assert_eq!(result, RustValue::String("survived".into()));
}

#[test]
fn async_function_return_value_resolves_to_promise() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = 0;
        async function f() { return 42; }
        f().then(function(v) { globalThis.__result = v; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::Int(42));
}

#[test]
fn async_await_chained_values() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = 0;
        async function step1() { return 10; }
        async function step2() { return await step1() + 5; }
        step2().then(function(v) { globalThis.__result = v; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::Int(15));
}

#[test]
fn promise_all_collects_results_in_order() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = "";
        Promise.all([Promise.resolve(1), Promise.resolve(2)])
            .then(function(arr) { globalThis.__result = arr.join(","); });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::String("1,2".into()));
}

#[test]
fn promise_all_three_values() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = "";
        Promise.all([Promise.resolve("a"), Promise.resolve("b"), Promise.resolve("c")])
            .then(function(arr) { globalThis.__result = arr.join(""); });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::String("abc".into()));
}

#[test]
fn promise_race_first_settled_wins() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = null;
        Promise.race([Promise.resolve("fast"), Promise.resolve("slow")])
            .then(function(v) { globalThis.__result = v; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::String("fast".into()));
}

#[test]
fn promise_race_reject_propagates() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__caught = null;
        Promise.race([Promise.reject(new Error("err")), Promise.resolve("ok")])
            .catch(function(e) { globalThis.__caught = e.message; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__caught");
    assert_eq!(result, RustValue::String("err".into()));
}

#[test]
fn microtask_runs_before_settimeout_macrotask() {
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
fn microtask_then_macrotask_after_advance() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__seq = [];
        setTimeout(function() { globalThis.__seq.push("timeout"); }, 0);
        Promise.resolve().then(function() { globalThis.__seq.push("microtask"); });
    "#,
    );
    // Microtask drained after eval. Now advance to fire macrotask.
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(10)");
    let result = kernel.eval_to_rust_value("globalThis.__seq.join(',')");
    assert_eq!(
        result,
        RustValue::String("microtask,timeout".into()),
        "microtask before macrotask after advance"
    );
}

#[test]
fn promise_constructor_resolve_immediately() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = 0;
        new Promise(function(resolve) { resolve(42); })
            .then(function(v) { globalThis.__result = v; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::Int(42));
}

#[test]
fn promise_constructor_reject_immediately() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__caught = null;
        new Promise(function(resolve, reject) { reject(new Error("ctor-reject")); })
            .catch(function(e) { globalThis.__caught = e.message; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__caught");
    assert_eq!(result, RustValue::String("ctor-reject".into()));
}

#[test]
fn promise_constructor_async_resolve() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = 0;
        var _resolve;
        var p = new Promise(function(resolve) { _resolve = resolve; });
        p.then(function(v) { globalThis.__result = v; });
        _resolve(99);
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::Int(99));
}

#[test]
fn promise_then_returns_new_promise() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__isPromise = false;
        var p1 = Promise.resolve(1);
        var p2 = p1.then(function(v) { return v + 1; });
        globalThis.__isPromise = (p2 instanceof Promise);
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__isPromise");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn promise_finally_runs_on_resolve() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__ran = false;
        Promise.resolve(1).finally(function() { globalThis.__ran = true; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__ran");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn promise_finally_runs_on_reject() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__ran = false;
        Promise.reject(new Error("x")).finally(function() { globalThis.__ran = true; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__ran");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn promise_resolve_nested_promise_unwraps() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = 0;
        var inner = Promise.resolve(7);
        Promise.resolve(inner).then(function(v) { globalThis.__result = v; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::Int(7));
}

#[test]
fn promise_all_settled_collects_all() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = "";
        Promise.allSettled([Promise.resolve(1), Promise.reject(new Error("e"))])
            .then(function(arr) {
                globalThis.__result = arr.map(function(r) { return r.status; }).join(",");
            });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::String("fulfilled,rejected".into()));
}

#[test]
fn promise_any_returns_first_fulfilled() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = null;
        Promise.any([Promise.reject(new Error("a")), Promise.resolve("ok"), Promise.resolve("late")])
            .then(function(v) { globalThis.__result = v; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::String("ok".into()));
}

#[test]
fn promise_microtask_ordering_fifo() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__seq = [];
        Promise.resolve().then(function() { globalThis.__seq.push(1); });
        Promise.resolve().then(function() { globalThis.__seq.push(2); });
        Promise.resolve().then(function() { globalThis.__seq.push(3); });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__seq.join(',')");
    assert_eq!(result, RustValue::String("1,2,3".into()));
}

#[test]
fn promise_chained_microtask_ordering() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__seq = [];
        var p = Promise.resolve();
        p.then(function() { globalThis.__seq.push("a"); })
         .then(function() { globalThis.__seq.push("b"); });
        p.then(function() { globalThis.__seq.push("c"); });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__seq.join(',')");
    // a and c are at the same chain level, b is one level deeper.
    // FIFO: a, c, then b (b is scheduled after a resolves).
    assert_eq!(result, RustValue::String("a,c,b".into()));
}

#[test]
fn queue_microtask_runs_after_eval() {
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
fn promise_then_callback_receives_undefined_for_void() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = "unchanged";
        Promise.resolve().then(function(v) { globalThis.__result = String(v); });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::String("undefined".into()));
}

#[test]
fn promise_throw_in_then_becomes_rejection() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__caught = null;
        Promise.resolve()
            .then(function() { throw new Error("thrown-in-then"); })
            .catch(function(e) { globalThis.__caught = e.message; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__caught");
    assert_eq!(result, RustValue::String("thrown-in-then".into()));
}

#[test]
fn promise_resolve_then_on_rejected_chain() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__seq = [];
        Promise.reject(new Error("fail"))
            .then(function(v) { globalThis.__seq.push("then-resolve:" + v); },
                  function(e) { globalThis.__seq.push("then-reject:" + e.message); })
            .then(function(v) { globalThis.__seq.push("after:" + v); });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__seq.join(',')");
    // reject -> second callback of then (recovery) -> next then receives undefined
    assert_eq!(
        result,
        RustValue::String("then-reject:fail,after:undefined".into())
    );
}

#[test]
fn async_function_rejection_propagates() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__caught = null;
        async function f() { throw new Error("async-throw"); }
        f().catch(function(e) { globalThis.__caught = e.message; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__caught");
    assert_eq!(result, RustValue::String("async-throw".into()));
}

#[test]
fn async_await_reject_propagates() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__caught = null;
        async function f() {
            try {
                await Promise.reject(new Error("await-reject"));
            } catch(e) {
                globalThis.__caught = e.message;
            }
        }
        f();
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__caught");
    assert_eq!(result, RustValue::String("await-reject".into()));
}

#[test]
fn promise_all_reject_on_first_rejection() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__caught = null;
        Promise.all([Promise.resolve(1), Promise.reject(new Error("mid-reject")), Promise.resolve(3)])
            .catch(function(e) { globalThis.__caught = e.message; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__caught");
    assert_eq!(result, RustValue::String("mid-reject".into()));
}

#[test]
fn promise_constructor_no_resolve_stays_pending() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = "pending";
        var p = new Promise(function() {});
        Promise.race([Promise.resolve("resolved"), p]).then(function(v) { globalThis.__result = v; });
    "#,
    );
    // Since p never resolves, race resolves with "resolved".
    // Microtask drains after eval.
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::String("resolved".into()));
}

#[test]
fn promise_is_constructor_callable() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("typeof Promise");
    assert_eq!(result, RustValue::String("function".into()));
}

#[test]
fn promise_all_with_empty_array_resolves_empty() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = "pending";
        Promise.all([]).then(function(arr) { globalThis.__result = arr.length; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::Int(0));
}

#[test]
fn promise_chained_with_catch_recovery() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        globalThis.__result = 0;
        Promise.resolve(1)
            .then(function(v) { throw new Error("chain-error"); })
            .catch(function(e) { return 99; })
            .then(function(v) { globalThis.__result = v; });
    "#,
    );
    let result = kernel.eval_to_rust_value("globalThis.__result");
    assert_eq!(result, RustValue::Int(99));
}
