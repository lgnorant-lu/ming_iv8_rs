#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_mut,
    unused_variables
)]
mod common;


// Edge case tests for iv8-core (补丁：覆盖审计中发现的缺口)

use iv8_core::{EmbeddedV8Kernel, EvalOpts, KernelConfig, RustValue};

#[test]
fn eval_empty_string_returns_null() {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    let result = kernel.eval_to_rust_value("");
    assert_eq!(result, RustValue::Null);
}

#[test]
fn eval_multiline_source() {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    let source = "var a = 1;\nvar b = 2;\na + b";
    let result = kernel.eval_to_rust_value(source);
    assert_eq!(result, RustValue::Int(3));
}

#[test]
fn eval_unicode_string() {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    let result = kernel.eval_to_rust_value("'你好世界'");
    assert_eq!(result, RustValue::String("你好世界".to_string()));
}

#[test]
fn eval_large_source_does_not_crash() {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    // 10KB source
    let source = format!("var s = '{}'; s.length", "x".repeat(10000));
    let result = kernel.eval_to_rust_value(&source);
    assert_eq!(result, RustValue::Int(10000));
}

#[test]
fn multiple_contexts_independent() {
    // V8 isolates must be used one at a time on the same thread (LIFO enter/exit).
    // Create, use, and drop sequentially.
    let result1 = {
        let mut k1 = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        k1.eval("var x = 'from_k1'", EvalOpts::default()).unwrap();
        k1.eval_to_rust_value("x")
    };

    let result2 = {
        let mut k2 = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        k2.eval("var x = 'from_k2'", EvalOpts::default()).unwrap();
        k2.eval_to_rust_value("x")
    };

    assert_eq!(result1, RustValue::String("from_k1".into()));
    assert_eq!(result2, RustValue::String("from_k2".into()));
}

#[test]
fn eval_after_dispose_still_works() {
    // dispose marks state but doesn't destroy isolate (that happens on drop)
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    kernel.dispose();
    // eval should still technically work (isolate is alive until drop)
    let result = kernel.eval_to_rust_value("1 + 1");
    assert_eq!(result, RustValue::Int(2));
}

#[test]
fn eval_reference_error() {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    let err = kernel
        .eval("nonexistent_var", EvalOpts::default())
        .unwrap_err();
    match err {
        iv8_core::IV8Error::Js { name, message, .. } => {
            assert_eq!(name, "ReferenceError");
            assert!(
                message.contains("nonexistent_var") || message.contains("not defined"),
                "message: {}",
                message
            );
        }
        other => panic!("expected Js error, got: {:?}", other),
    }
}

#[test]
fn eval_type_error() {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    let err = kernel.eval("null.foo", EvalOpts::default()).unwrap_err();
    match err {
        iv8_core::IV8Error::Js { name, .. } => {
            assert_eq!(name, "TypeError");
        }
        other => panic!("expected TypeError, got: {:?}", other),
    }
}

#[test]
fn runtime_state_environment_accessible_from_isolate() {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    // Access RuntimeState from isolate and verify environment is there
    let state = iv8_core::state::RuntimeState::get(kernel.isolate_ref());
    let ua = state.environment.get_str("navigator.userAgent");
    assert!(
        ua.is_some(),
        "environment should be accessible from RuntimeState"
    );
    assert!(ua.unwrap().contains("Chrome"));
}
