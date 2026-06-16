//! v0.8.51: Integration tests for input simulation API.
mod common;

#[test]
fn test_input_api_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof __iv8__.input", "object");
}

#[test]
fn test_input_dispatch_mouse_event_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof __iv8__.input.dispatchMouseEvent",
        "function",
    );
}

#[test]
fn test_input_dispatch_keyboard_event_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof __iv8__.input.dispatchKeyboardEvent",
        "function",
    );
}

#[test]
fn test_input_dispatch_pointer_event_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof __iv8__.input.dispatchPointerEvent",
        "function",
    );
}
