//! v0.8.51: Integration tests for console surface.
mod common;

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
