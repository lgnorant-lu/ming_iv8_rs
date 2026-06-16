//! v0.8.51: Integration tests for Navigator extras (plugins, mimeTypes, connection).
mod common;

#[test]
fn test_navigator_plugins_empty_array() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.plugins", "object");
    let len = common::to_str(&k.eval_to_rust_value("navigator.plugins.length"));
    assert_eq!(len, "0");
}

#[test]
fn test_navigator_plugins_refresh() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.plugins.refresh", "function");
}

#[test]
fn test_navigator_mime_types_empty() {
    let mut k = common::make_kernel();
    let len = common::to_str(&k.eval_to_rust_value("navigator.mimeTypes.length"));
    assert_eq!(len, "0");
}

#[test]
fn test_navigator_mime_types_named_item() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.mimeTypes.namedItem", "function");
}

#[test]
fn test_navigator_connection_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.connection", "object");
}

#[test]
fn test_navigator_connection_effective_type() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.connection.effectiveType", "string");
}

#[test]
fn test_navigator_send_beacon_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.sendBeacon", "function");
}
