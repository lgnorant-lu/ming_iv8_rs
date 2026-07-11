//! Codegen [Global] interface attribute/operation split tests.
//!
//! Validates that for [Global] interfaces (Window, WorkerGlobalScope):
//! - Attributes are on instance_template (appear on globalThis as own props)
//! - Operations are on prototype_template (appear on Window.prototype)
//! - self/window/top/parent/frames return globalThis

mod common;

use common::*;

#[test]
fn test_window_prototype_has_operations() {
    let mut k = make_kernel();
    assert_js_str(&mut k, "typeof Window.prototype.postMessage", "function");
    assert_js_str(&mut k, "typeof Window.prototype.setTimeout", "function");
    assert_js_str(&mut k, "typeof Window.prototype.clearTimeout", "function");
    assert_js_str(&mut k, "typeof Window.prototype.btoa", "function");
    assert_js_str(&mut k, "typeof Window.prototype.atob", "function");
}

#[test]
fn test_window_prototype_has_scroll_operations() {
    let mut k = make_kernel();
    assert_js_str(&mut k, "typeof Window.prototype.scroll", "function");
    assert_js_str(&mut k, "typeof Window.prototype.scrollTo", "function");
    assert_js_str(&mut k, "typeof Window.prototype.scrollBy", "function");
}

#[test]
fn test_globalthis_has_window_attributes() {
    let mut k = make_kernel();
    assert_js_str(&mut k, "typeof self", "object");
    assert_js_str(&mut k, "typeof window", "object");
    assert_js_str(&mut k, "typeof document", "object");
    assert_js_str(&mut k, "typeof navigator", "object");
    assert_js_str(&mut k, "typeof location", "object");
}

#[test]
fn test_self_returns_globalthis() {
    let mut k = make_kernel();
    assert_js_str(&mut k, "self === globalThis", "true");
    assert_js_str(&mut k, "window === globalThis", "true");
}

#[test]
fn test_window_prototype_not_missing_attributes() {
    let mut k = make_kernel();
    let val = to_str(&k.eval_to_rust_value(
        "Object.getOwnPropertyDescriptor(Window.prototype, 'devicePixelRatio') === undefined"
    ));
    assert_eq!(val, "true", "devicePixelRatio should NOT be on Window.prototype (it's on instance)");
}

#[test]
fn test_window_attribute_getter_throws_on_wrong_receiver() {
    let mut k = make_kernel();
    assert_js_error(&mut k, "(function(){ var g = Object.getOwnPropertyDescriptor(globalThis, 'self').get; g.call({}); })()");
}

#[test]
fn test_screen_width_positive() {
    let mut k = make_kernel();
    let val = to_str(&k.eval_to_rust_value("screen.width > 0"));
    assert_eq!(val, "true", "screen.width should be positive");
}
