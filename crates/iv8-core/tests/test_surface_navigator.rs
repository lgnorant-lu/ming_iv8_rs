//! v0.8.51 S3: Integration tests for Navigator surface.
mod common;

#[test]
fn test_navigator_user_agent() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("navigator.userAgent"));
    assert!(val.contains("Chrome"), "UA missing Chrome: {}", val);
    assert!(val.contains("Windows"), "UA missing Windows: {}", val);
}

#[test]
fn test_navigator_platform() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.platform", "string");
}

#[test]
fn test_navigator_webdriver_false() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("navigator.webdriver"));
    assert_eq!(val, "false");
}

#[test]
fn test_navigator_java_enabled_method() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.javaEnabled", "function");
    let val = common::to_str(&k.eval_to_rust_value("navigator.javaEnabled()"));
    assert_eq!(val, "false");
}

#[test]
fn test_navigator_plugins_empty() {
    let mut k = common::make_kernel();
    let len = common::to_str(&k.eval_to_rust_value("navigator.plugins.length"));
    assert_eq!(len, "0");
    common::assert_js_str(&mut k, "typeof navigator.plugins.item", "function");
}

#[test]
fn test_navigator_mime_types_empty() {
    let mut k = common::make_kernel();
    let len = common::to_str(&k.eval_to_rust_value("navigator.mimeTypes.length"));
    assert_eq!(len, "0");
}

#[test]
fn test_navigator_hardware_concurrency() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.hardwareConcurrency", "number");
}

#[test]
fn test_navigator_language() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.language", "string");
}
