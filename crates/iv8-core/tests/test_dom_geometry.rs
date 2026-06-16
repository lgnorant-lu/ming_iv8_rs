#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]

//! Integration tests for getBoundingClientRect + getComputedStyle (Task 70+71).
//! Acceptance criteria:
//! - element.getBoundingClientRect() returns {x, y, width, height, top, right, bottom, left}
//! - getComputedStyle(element).display returns string
//! - getComputedStyle(element).getPropertyValue('font-size') works

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};

fn make_kernel_with_doc(html: &str) -> EmbeddedV8Kernel {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    kernel.set_document(html, None);
    kernel
}

#[test]
fn get_bounding_client_rect_exists() {
    let mut kernel = make_kernel_with_doc("<div id='x'></div>");
    assert_eq!(
        kernel.eval_to_rust_value("typeof document.getElementById('x').getBoundingClientRect"),
        RustValue::String("function".into())
    );
}

#[test]
fn get_bounding_client_rect_returns_object() {
    let mut kernel = make_kernel_with_doc("<div id='x'></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var rect = document.getElementById('x').getBoundingClientRect();
        typeof rect.x === 'number' && typeof rect.width === 'number' &&
        typeof rect.top === 'number' && typeof rect.right === 'number'
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn get_bounding_client_rect_has_all_props() {
    let mut kernel = make_kernel_with_doc("<div id='x'></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var rect = document.getElementById('x').getBoundingClientRect();
        ['x','y','width','height','top','right','bottom','left'].every(function(k) {
            return k in rect;
        })
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn dom_rect_constructor() {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    assert_eq!(
        kernel.eval_to_rust_value("typeof DOMRect"),
        RustValue::String("function".into())
    );
    let result = kernel.eval_to_rust_value("new DOMRect(10, 20, 100, 50).right");
    assert_eq!(result, RustValue::Int(110));
}

#[test]
fn offset_width_height() {
    let mut kernel = make_kernel_with_doc("<div id='x'></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var el = document.getElementById('x');
        typeof el.offsetWidth === 'number' && typeof el.offsetHeight === 'number'
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn get_computed_style_exists() {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    assert_eq!(
        kernel.eval_to_rust_value("typeof getComputedStyle"),
        RustValue::String("function".into())
    );
}

#[test]
fn get_computed_style_display() {
    let mut kernel = make_kernel_with_doc("<div id='x'></div>");
    let result =
        kernel.eval_to_rust_value("getComputedStyle(document.getElementById('x')).display");
    assert_eq!(result, RustValue::String("block".into()));
}

#[test]
fn get_computed_style_font_size() {
    let mut kernel = make_kernel_with_doc("<div id='x'></div>");
    let result =
        kernel.eval_to_rust_value("getComputedStyle(document.getElementById('x')).fontSize");
    assert_eq!(result, RustValue::String("16px".into()));
}

#[test]
fn get_computed_style_get_property_value() {
    let mut kernel = make_kernel_with_doc("<div id='x'></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        getComputedStyle(document.getElementById('x')).getPropertyValue('fontSize')
    "#,
    );
    assert_eq!(result, RustValue::String("16px".into()));
}

#[test]
fn client_width_height() {
    let mut kernel = make_kernel_with_doc("<div id='x'></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var el = document.getElementById('x');
        typeof el.clientWidth === 'number' && typeof el.clientHeight === 'number'
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}
