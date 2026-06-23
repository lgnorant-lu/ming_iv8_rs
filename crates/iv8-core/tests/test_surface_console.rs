//! v0.8.51: Integration tests for console surface.
//! v0.8.77: Expanded with actual call tests for all 16 methods.
mod common;

use iv8_core::RustValue;

// === T1: Existence tests (original v0.8.51) ===

#[test]
fn test_console_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof console", "object");
}

#[test]
fn test_console_log_is_function() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof console.log", "function");
}

#[test]
fn test_console_warn_is_function() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof console.warn", "function");
}

#[test]
fn test_console_error_is_function() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof console.error", "function");
}

#[test]
fn test_console_info_is_function() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof console.info", "function");
}

#[test]
fn test_console_debug_is_function() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof console.debug", "function");
}

// === T2: All 16 methods exist (v0.8.77 expansion) ===

#[test]
fn test_console_all_methods_exist() {
    let mut k = common::make_kernel();
    let methods = [
        "log", "info", "warn", "error", "debug", "trace",
        "dir", "table", "group", "groupCollapsed", "groupEnd",
        "time", "timeEnd", "count", "countReset", "assert", "clear",
    ];
    for m in &methods {
        let js = format!("typeof console.{}", m);
        let result = common::to_str(&k.eval_to_rust_value(&js));
        assert_eq!(
            result, "function",
            "console.{} should be function, got {}", m, result
        );
    }
}

// === T3: Actual call tests — verify no crash + message storage ===

#[test]
fn test_console_log_call_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.log('hello world')");
}

#[test]
fn test_console_warn_call_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.warn('warning text')");
}

#[test]
fn test_console_error_call_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.error('error text')");
}

#[test]
fn test_console_info_call_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.info('info text')");
}

#[test]
fn test_console_debug_call_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.debug('debug text')");
}

#[test]
fn test_console_trace_call_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.trace('trace text')");
}

#[test]
fn test_console_dir_call_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.dir({a: 1})");
}

#[test]
fn test_console_table_call_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.table([[1,2],[3,4]])");
}

#[test]
fn test_console_group_call_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.group('group label'); console.groupEnd();");
}

#[test]
fn test_console_time_timeend_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.time('timer'); console.timeEnd('timer');");
}

#[test]
fn test_console_count_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.count('counter'); console.count('counter');");
}

#[test]
fn test_console_assert_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.assert(false, 'assertion failed')");
}

#[test]
fn test_console_clear_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.clear()");
}

#[test]
fn test_console_multiple_args_no_crash() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("console.log('a', 1, true, null, {x: 1}, [1,2,3])");
}

// === T4: Message storage retrieval ===
// TODO: __iv8__.console_messages is documented in console.rs but not
// implemented. Messages are routed to tracing only, not stored in a
// JS-accessible array. This is a feature gap — see TODO-infrastructure.

#[test]
fn test_console_log_does_not_throw() {
    let mut k = common::make_kernel();
    // Verify console.log returns undefined (no throw)
    let result = k.eval_to_rust_value("console.log('test'); 'ok'");
    assert_eq!(common::to_str(&result), "ok");
}
