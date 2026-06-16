#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]

//! Integration tests for WebGL parameter stubs (Task 45).

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};

fn make_kernel() -> EmbeddedV8Kernel {
    EmbeddedV8Kernel::new(KernelConfig::default()).unwrap()
}

#[test]
fn webgl_context_exists() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("typeof __webglContext__");
    assert_eq!(result, RustValue::String("object".into()));
}

#[test]
fn webgl_get_parameter_vendor() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("__webglContext__.getParameter(0x9245)"); // UNMASKED_VENDOR
    match result {
        RustValue::String(s) => assert!(
            s.contains("NVIDIA") || s.contains("Google"),
            "vendor: {}",
            s
        ),
        other => panic!("expected String, got: {:?}", other),
    }
}

#[test]
fn webgl_get_parameter_renderer() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("__webglContext__.getParameter(0x9246)"); // UNMASKED_RENDERER
    match result {
        RustValue::String(s) => assert!(
            s.contains("ANGLE") || s.contains("GeForce"),
            "renderer: {}",
            s
        ),
        other => panic!("expected String, got: {:?}", other),
    }
}

#[test]
fn webgl_get_parameter_version() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("__webglContext__.getParameter(0x1F02)"); // GL_VERSION
    match result {
        RustValue::String(s) => assert!(s.contains("WebGL"), "version: {}", s),
        other => panic!("expected String, got: {:?}", other),
    }
}

#[test]
fn webgl_get_parameter_max_texture_size() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("__webglContext__.getParameter(0x0D33)"); // MAX_TEXTURE_SIZE
    assert_eq!(result, RustValue::Int(16384));
}

#[test]
fn webgl_get_extension_debug_renderer_info() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var ext = __webglContext__.getExtension('WEBGL_debug_renderer_info');
        ext !== null && typeof ext.UNMASKED_VENDOR_WEBGL === 'number'
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn webgl_get_supported_extensions() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var exts = __webglContext__.getSupportedExtensions();
        Array.isArray(exts) && exts.length > 10 && exts.includes('WEBGL_debug_renderer_info')
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn webgl_get_parameter_unknown_returns_null() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("__webglContext__.getParameter(99999)");
    assert_eq!(result, RustValue::Null);
}

#[test]
fn webgl_environment_override_renderer() {
    let mut overrides = std::collections::HashMap::new();
    overrides.insert(
        "webgl.UNMASKED_RENDERER_WEBGL".to_string(),
        serde_json::Value::String("Custom GPU".to_string()),
    );
    let config = iv8_core::KernelConfig {
        environment_overrides: Some(overrides),
        ..Default::default()
    };
    let mut kernel = EmbeddedV8Kernel::new(config).unwrap();
    let result = kernel.eval_to_rust_value("__webglContext__.getParameter(0x9246)");
    assert_eq!(result, RustValue::String("Custom GPU".into()));
}
