//! v0.8.51: Integration tests for Canvas 2D surface.
mod common;

#[test]
fn test_canvas_create_element() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("typeof document.createElement('canvas')"));
    assert_eq!(val, "object");
}

#[test]
fn test_canvas_get_context_2d() {
    let mut k = common::make_kernel();
    let val = common::to_str(
        &k.eval_to_rust_value("typeof document.createElement('canvas').getContext('2d')"),
    );
    assert!(val == "object" || val.contains("Canvas"), "got {}", val);
}

#[test]
fn test_canvas_2d_fill_rect() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof document.createElement('canvas').getContext('2d').fillRect",
        "function",
    );
}

#[test]
fn test_canvas_2d_fill_style() {
    let mut k = common::make_kernel();
    let ctx = k.eval_to_rust_value("document.createElement('canvas').getContext('2d').fillStyle");
    assert!(!common::to_str(&ctx).is_empty(), "fillStyle empty");
}

#[test]
fn test_canvas_2d_stroke_rect() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof document.createElement('canvas').getContext('2d').strokeRect",
        "function",
    );
}

#[test]
fn test_canvas_2d_clear_rect() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof document.createElement('canvas').getContext('2d').clearRect",
        "function",
    );
}

#[test]
fn test_canvas_2d_begin_path() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof document.createElement('canvas').getContext('2d').beginPath",
        "function",
    );
}

#[test]
fn test_canvas_2d_save_restore() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof document.createElement('canvas').getContext('2d').save",
        "function",
    );
    common::assert_js_str(
        &mut k,
        "typeof document.createElement('canvas').getContext('2d').restore",
        "function",
    );
}

#[test]
fn test_canvas_2d_measure_text() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value(
        "typeof document.createElement('canvas').getContext('2d').measureText('x')",
    ));
    assert_eq!(val, "object");
}
