//! v0.8.51 S3: Integration tests for Navigator extras (plugins, mimeTypes, connection, sendBeacon, getBattery).
//! v0.8.55: Updated to match native getter behavior (plugins/mimeTypes now native, no JS shim methods).
mod common;

#[test]
fn test_navigator_plugins_array() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.plugins", "object");
    let len = common::to_str(&k.eval_to_rust_value("navigator.plugins.length"));
    assert_eq!(len, "5");
    common::assert_js_str(&mut k, "typeof navigator.plugins[0]", "object");
}

#[test]
fn test_navigator_mime_types_array() {
    let mut k = common::make_kernel();
    let len = common::to_str(&k.eval_to_rust_value("navigator.mimeTypes.length"));
    assert_eq!(len, "2");
    common::assert_js_str(&mut k, "typeof navigator.mimeTypes[0]", "object");
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

#[test]
fn test_navigator_connection_no_own_data_descriptor() {
    let mut k = common::make_kernel();
    let desc = k.eval_to_rust_value(
        "Object.getOwnPropertyDescriptor(navigator, 'connection')"
    );
    let desc_str = common::to_str(&desc);
    assert!(
        desc_str == "undefined" || desc_str == "null",
        "connection should not be own data descriptor: {}",
        desc_str
    );
}

#[test]
fn test_navigator_send_beacon_no_own_data_descriptor() {
    let mut k = common::make_kernel();
    let desc = k.eval_to_rust_value(
        "Object.getOwnPropertyDescriptor(navigator, 'sendBeacon')"
    );
    let desc_str = common::to_str(&desc);
    assert!(
        desc_str == "undefined" || desc_str == "null",
        "sendBeacon should not be own data descriptor: {}",
        desc_str
    );
}

#[test]
fn test_navigator_get_battery_no_own_data_descriptor() {
    let mut k = common::make_kernel();
    let desc = k.eval_to_rust_value(
        "Object.getOwnPropertyDescriptor(navigator, 'getBattery')"
    );
    let desc_str = common::to_str(&desc);
    assert!(
        desc_str == "undefined" || desc_str == "null",
        "getBattery should not be own data descriptor: {}",
        desc_str
    );
}
