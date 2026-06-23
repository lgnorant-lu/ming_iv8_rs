#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};

// ─── Canvas element existence ────────────────────────────────────────────

#[test]
fn html_canvas_element_exists() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    assert_eq!(
        kernel.eval_to_rust_value("typeof HTMLCanvasElement"),
        RustValue::String("function".into())
    );
}

#[test]
fn canvas_get_context_exists() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    assert_eq!(
        kernel.eval_to_rust_value("typeof document.getElementById('c').getContext"),
        RustValue::String("function".into())
    );
}

#[test]
fn canvas_get_context_2d_returns_object() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    assert_eq!(
        kernel.eval_to_rust_value(
            "typeof document.getElementById('c').getContext('2d')"
        ),
        RustValue::String("object".into())
    );
}

#[test]
fn canvas_to_data_url_exists() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    assert_eq!(
        kernel.eval_to_rust_value("typeof document.getElementById('c').toDataURL"),
        RustValue::String("function".into())
    );
}

#[test]
fn canvas_to_data_url_returns_string() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    let result = kernel.eval_to_rust_value(
        "typeof document.getElementById('c').toDataURL() === 'string'"
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn canvas_width_height_are_numbers() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c' width='300' height='150'></canvas>");
    let result = kernel.eval_to_rust_value(
        r#"
        var c = document.getElementById('c');
        typeof c.width === 'number' && c.width === 300 &&
        typeof c.height === 'number' && c.height === 150
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn canvas_2d_context_fill_rect_shape() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    let result = kernel.eval_to_rust_value(
        r#"
        var ctx = document.getElementById('c').getContext('2d');
        typeof ctx.fillRect === 'function' &&
        typeof ctx.fillStyle !== 'undefined' &&
        typeof ctx.strokeRect === 'function'
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

// ─── WebGL surface ───────────────────────────────────────────────────────

#[test]
fn canvas_get_context_webgl_returns_object() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    let result = kernel.eval_to_rust_value(
        "typeof document.getElementById('c').getContext('webgl')"
    );
    assert_eq!(result, RustValue::String("object".into()));
}

#[test]
fn webgl_get_parameter_exists() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    let result = kernel.eval_to_rust_value(
        r#"
        var gl = document.getElementById('c').getContext('webgl');
        typeof gl.getParameter === 'function'
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn webgl_unmasked_renderer_is_string() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    let result = kernel.eval_to_rust_value(
        r#"
        var gl = document.getElementById('c').getContext('webgl');
        var ext = gl.getExtension('WEBGL_debug_renderer_info');
        typeof ext.UNMASKED_RENDERER_WEBGL === 'number'
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn webgl_default_parameters_exist() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    let result = kernel.eval_to_rust_value(
        r#"
        var gl = document.getElementById('c').getContext('webgl');
        typeof gl.RENDERER === 'number' &&
        typeof gl.VENDOR === 'number' &&
        typeof gl.SHADING_LANGUAGE_VERSION === 'number'
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

// ─── Audio surface ───────────────────────────────────────────────────────

#[test]
fn audio_context_exists() {
    let mut kernel = common::make_kernel_with_doc("");
    assert_eq!(
        kernel.eval_to_rust_value("typeof AudioContext"),
        RustValue::String("function".into())
    );
}

#[test]
fn audio_context_create_oscillator_shape() {
    let mut kernel = common::make_kernel_with_doc("");
    let result = kernel.eval_to_rust_value(
        r#"
        var ctx = new AudioContext();
        var osc = ctx.createOscillator();
        typeof osc.type === 'string' &&
        typeof osc.frequency !== 'undefined' &&
        typeof osc.connect === 'function'
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn audio_context_create_gain_shape() {
    let mut kernel = common::make_kernel_with_doc("");
    let result = kernel.eval_to_rust_value(
        r#"
        var ctx = new AudioContext();
        var gain = ctx.createGain();
        typeof gain.gain !== 'undefined' &&
        typeof gain.connect === 'function'
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn audio_context_sample_rate_is_number() {
    let mut kernel = common::make_kernel_with_doc("");
    assert_eq!(
        kernel.eval_to_rust_value("typeof new AudioContext().sampleRate === 'number'"),
        RustValue::Bool(true)
    );
}

// ─── T2: 值/类型级 canvas 测试 ─────────────────────────────────────────

#[test]
fn canvas_data_url_has_png_prefix() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    let result = kernel.eval_to_rust_value(
        r#"
        var url = document.getElementById('c').toDataURL();
        url.indexOf('data:image/png;base64,') === 0
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn canvas_2d_fillstyle_is_string() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    let result = kernel.eval_to_rust_value(
        r#"
        var ctx = document.getElementById('c').getContext('2d');
        typeof ctx.fillStyle === 'string'
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn canvas_2d_strokestyle_is_string() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    assert_eq!(
        kernel.eval_to_rust_value(
            "typeof document.getElementById('c').getContext('2d').strokeStyle === 'string'"
        ),
        RustValue::Bool(true)
    );
}

#[test]
fn canvas_2d_measure_text_returns_object() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    let result = kernel.eval_to_rust_value(
        r#"
        var ctx = document.getElementById('c').getContext('2d');
        var m = ctx.measureText('Hello');
        typeof m === 'object' && typeof m.width === 'number'
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn canvas_2d_save_restore_exist() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    let result = kernel.eval_to_rust_value(
        r#"
        var ctx = document.getElementById('c').getContext('2d');
        typeof ctx.save === 'function' &&
        typeof ctx.restore === 'function' &&
        typeof ctx.scale === 'function' &&
        typeof ctx.rotate === 'function' &&
        typeof ctx.translate === 'function'
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

// ─── T2: WebGL 常量值级测试 ────────────────────────────────────────────

#[test]
fn webgl_get_parameter_renderer_returns_string() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    let result = kernel.eval_to_rust_value(
        r#"
        var gl = document.getElementById('c').getContext('webgl');
        typeof gl.getParameter(gl.RENDERER) === 'string'
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn webgl_get_parameter_vendor_returns_string() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    assert_eq!(
        kernel.eval_to_rust_value(
            "typeof document.getElementById('c').getContext('webgl').getParameter(document.getElementById('c').getContext('webgl').VENDOR) === 'string'"
        ),
        RustValue::Bool(true)
    );
}

#[test]
fn webgl_context_attributes_exist() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    let result = kernel.eval_to_rust_value(
        r#"
        var gl = document.getElementById('c').getContext('webgl');
        typeof gl.canvas === 'object' &&
        typeof gl.drawingBufferWidth === 'number' &&
        typeof gl.drawingBufferHeight === 'number'
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn webgl_extensions_list_has_debug_renderer() {
    let mut kernel = common::make_kernel_with_doc("<canvas id='c'></canvas>");
    let result = kernel.eval_to_rust_value(
        r#"
        var gl = document.getElementById('c').getContext('webgl');
        var ext = gl.getExtension('WEBGL_debug_renderer_info');
        ext !== null &&
        typeof ext.UNMASKED_VENDOR_WEBGL === 'number' &&
        typeof ext.UNMASKED_RENDERER_WEBGL === 'number'
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

// ─── T2: Audio 更深值级测试 ─────────────────────────────────────────────

#[test]
fn audio_context_destination_exists() {
    let mut kernel = common::make_kernel_with_doc("");
    assert_eq!(
        kernel.eval_to_rust_value("typeof new AudioContext().destination === 'object'"),
        RustValue::Bool(true)
    );
}

#[test]
fn oscillator_frequency_value_is_number() {
    let mut kernel = common::make_kernel_with_doc("");
    let result = kernel.eval_to_rust_value(
        r#"
        var ctx = new AudioContext();
        var osc = ctx.createOscillator();
        typeof osc.frequency.value === 'number'
    "#
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn audio_context_current_time_is_number() {
    let mut kernel = common::make_kernel_with_doc("");
    assert_eq!(
        kernel.eval_to_rust_value("typeof new AudioContext().currentTime === 'number'"),
        RustValue::Bool(true)
    );
}

