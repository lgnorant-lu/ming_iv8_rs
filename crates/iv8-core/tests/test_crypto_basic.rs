#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// Integration tests for crypto.getRandomValues + crypto.randomUUID (Task 41).

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};
#[test]
fn crypto_get_random_values_exists() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof crypto.getRandomValues"),
        RustValue::String("function".into())
    );
}

#[test]
fn crypto_random_uuid_exists() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof crypto.randomUUID"),
        RustValue::String("function".into())
    );
}

#[test]
fn crypto_random_uuid_format() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("crypto.randomUUID()");
    match result {
        RustValue::String(s) => {
            // UUID v4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
            assert_eq!(s.len(), 36, "UUID should be 36 chars: {}", s);
            assert_eq!(&s[8..9], "-");
            assert_eq!(&s[13..14], "-");
            assert_eq!(&s[14..15], "4"); // version 4
            assert_eq!(&s[18..19], "-");
            assert_eq!(&s[23..24], "-");
            // variant: char at position 19 should be 8, 9, a, or b
            let variant_char = s.chars().nth(19).unwrap();
            assert!(
                "89ab".contains(variant_char),
                "variant char should be 8/9/a/b, got: {}",
                variant_char
            );
        }
        other => panic!("expected String, got: {:?}", other),
    }
}

#[test]
fn crypto_random_uuid_unique() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("crypto.randomUUID() !== crypto.randomUUID()");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn crypto_get_random_values_fills_uint8array() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var arr = new Uint8Array(16);
        crypto.getRandomValues(arr);
        arr.some(x => x !== 0)  // at least one non-zero byte
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn crypto_get_random_values_returns_same_array() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var arr = new Uint8Array(4);
        var returned = crypto.getRandomValues(arr);
        arr === returned
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn crypto_get_random_values_uint32array() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var arr = new Uint32Array(4);
        crypto.getRandomValues(arr);
        arr.some(x => x !== 0)
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn crypto_get_random_values_quota_exceeded() {
    let mut kernel = common::make_kernel();
    let err = kernel
        .eval(
            "crypto.getRandomValues(new Uint8Array(65537))",
            iv8_core::EvalOpts::default(),
        )
        .unwrap_err();
    match err {
        iv8_core::IV8Error::Js { message, .. } => {
            assert!(message.contains("quota"), "msg: {}", message);
        }
        other => panic!("expected Js error, got: {:?}", other),
    }
}

#[test]
fn crypto_get_random_values_non_typed_array_throws() {
    let mut kernel = common::make_kernel();
    let err = kernel
        .eval(
            "crypto.getRandomValues([1,2,3])",
            iv8_core::EvalOpts::default(),
        )
        .unwrap_err();
    match err {
        iv8_core::IV8Error::Js { message, .. } => {
            assert!(message.contains("TypedArray"), "msg: {}", message);
        }
        other => panic!("expected Js error, got: {:?}", other),
    }
}
