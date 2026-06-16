#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;


// Integration tests for Event/CustomEvent/MouseEvent constructors (Task 68).
// Acceptance criteria:
// - new Event('click') creates event object
// - new Event('x', {bubbles: true}) sets bubbles
// - event.type / bubbles / cancelable correct
// - event.stopPropagation() works
// - event.preventDefault() sets defaultPrevented

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};
// ─── Event ──────────────────────────────────────────────────────────────────

#[test]
fn event_constructor_exists() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof Event"),
        RustValue::String("function".into())
    );
}

#[test]
fn event_basic_creation() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("new Event('click').type");
    assert_eq!(result, RustValue::String("click".into()));
}

#[test]
fn event_bubbles_default_false() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new Event('x').bubbles"),
        RustValue::Bool(false)
    );
}

#[test]
fn event_bubbles_option() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new Event('x', {bubbles: true}).bubbles"),
        RustValue::Bool(true)
    );
}

#[test]
fn event_cancelable_option() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new Event('x', {cancelable: true}).cancelable"),
        RustValue::Bool(true)
    );
}

#[test]
fn event_prevent_default() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var e = new Event('x', {cancelable: true});
        e.preventDefault();
        e.defaultPrevented
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn event_prevent_default_non_cancelable() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var e = new Event('x', {cancelable: false});
        e.preventDefault();
        e.defaultPrevented
    "#,
    );
    assert_eq!(result, RustValue::Bool(false));
}

#[test]
fn event_stop_propagation() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var e = new Event('x');
        e.stopPropagation();
        e._stopPropagation
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn event_timestamp() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("typeof new Event('x').timeStamp === 'number'");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn event_instanceof() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new Event('x') instanceof Event"),
        RustValue::Bool(true)
    );
}

// ─── CustomEvent ────────────────────────────────────────────────────────────

#[test]
fn custom_event_exists() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof CustomEvent"),
        RustValue::String("function".into())
    );
}

#[test]
fn custom_event_detail() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("new CustomEvent('x', {detail: {foo: 42}}).detail.foo");
    assert_eq!(result, RustValue::Int(42));
}

#[test]
fn custom_event_instanceof_event() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new CustomEvent('x') instanceof Event"),
        RustValue::Bool(true)
    );
}

// ─── MouseEvent ─────────────────────────────────────────────────────────────

#[test]
fn mouse_event_exists() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof MouseEvent"),
        RustValue::String("function".into())
    );
}

#[test]
fn mouse_event_coordinates() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var e = new MouseEvent('click', {clientX: 100, clientY: 200});
        e.clientX + ',' + e.clientY
    "#,
    );
    assert_eq!(result, RustValue::String("100,200".into()));
}

#[test]
fn mouse_event_button() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new MouseEvent('click', {button: 2}).button"),
        RustValue::Int(2)
    );
}

#[test]
fn mouse_event_instanceof_event() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new MouseEvent('click') instanceof Event"),
        RustValue::Bool(true)
    );
}

// ─── KeyboardEvent ──────────────────────────────────────────────────────────

#[test]
fn keyboard_event_exists() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof KeyboardEvent"),
        RustValue::String("function".into())
    );
}

#[test]
fn keyboard_event_key() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new KeyboardEvent('keydown', {key: 'Enter'}).key"),
        RustValue::String("Enter".into())
    );
}

// ─── PointerEvent ───────────────────────────────────────────────────────────

#[test]
fn pointer_event_exists() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof PointerEvent"),
        RustValue::String("function".into())
    );
}

#[test]
fn pointer_event_pointer_type() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value(
            "new PointerEvent('pointerdown', {pointerType: 'touch'}).pointerType"
        ),
        RustValue::String("touch".into())
    );
}

#[test]
fn pointer_event_instanceof_mouse_event() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new PointerEvent('x') instanceof MouseEvent"),
        RustValue::Bool(true)
    );
}
