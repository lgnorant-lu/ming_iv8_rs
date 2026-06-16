//! v0.8.51: Integration tests for __iv8__ tool event bindings.
mod common;

#[test]
fn test_iv8_event_loop_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof __iv8__", "undefined");
    common::assert_js_str(&mut k, "typeof __iv8__.eventLoop", "object");
}

#[test]
fn test_iv8_event_loop_advance() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof __iv8__.eventLoop.advance", "function");
}

#[test]
fn test_iv8_event_loop_get_time() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof __iv8__.eventLoop.getTime", "function");
    let t0 = common::to_str(&k.eval_to_rust_value("__iv8__.eventLoop.getTime()"));
    assert_eq!(t0, "0");
}

#[test]
fn test_iv8_netlog_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof __iv8__.netLog", "object");
    common::assert_js_str(&mut k, "Array.isArray(__iv8__.netLog.entries)", "true");
}

#[test]
fn test_iv8_page_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof __iv8__.page", "object");
    common::assert_js_str(&mut k, "typeof __iv8__.page.load", "function");
}

#[test]
fn test_iv8_input_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof __iv8__.input", "object");
}

#[test]
fn test_iv8_undetectable_typeof() {
    let mut k = common::make_kernel();
    // __iv8__ is MarkAsUndetectable (HTMLDDA) — typeof returns "undefined"
    common::assert_js_str(&mut k, "typeof __iv8__", "undefined");
    // but it IS accessible as a property
    let exists = common::to_str(&k.eval_to_rust_value("'__iv8__' in window"));
    assert_eq!(exists, "true");
}
