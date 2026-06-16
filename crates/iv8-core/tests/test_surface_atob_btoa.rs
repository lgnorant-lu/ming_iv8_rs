#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;


// Integration tests for atob/btoa (Task 64).
// Acceptance criteria:
// - btoa('hello') === 'aGVsbG8='
// - atob('aGVsbG8=') === 'hello'
// - Non-Latin-1 → throws
// - Invalid base64 → throws

use iv8_core::{EmbeddedV8Kernel, EvalOpts, KernelConfig, RustValue};
#[test]
fn btoa_exists() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof btoa"),
        RustValue::String("function".into())
    );
}

#[test]
fn atob_exists() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof atob"),
        RustValue::String("function".into())
    );
}

#[test]
fn btoa_hello() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("btoa('hello')"),
        RustValue::String("aGVsbG8=".into())
    );
}

#[test]
fn atob_hello() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("atob('aGVsbG8=')"),
        RustValue::String("hello".into())
    );
}

#[test]
fn btoa_empty() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("btoa('')"),
        RustValue::String("".into())
    );
}

#[test]
fn atob_empty() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("atob('')"),
        RustValue::String("".into())
    );
}

#[test]
fn btoa_atob_roundtrip() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("atob(btoa('Hello, World!'))"),
        RustValue::String("Hello, World!".into())
    );
}

#[test]
fn btoa_binary_data() {
    let mut kernel = common::make_kernel();
    // String with bytes 0-255
    let result = kernel.eval_to_rust_value(
        r#"
        var s = '';
        for (var i = 0; i < 256; i++) s += String.fromCharCode(i);
        atob(btoa(s)) === s
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn btoa_non_latin1_throws() {
    let mut kernel = common::make_kernel();
    let err = kernel
        .eval("btoa('\\u0100')", EvalOpts::default())
        .unwrap_err();
    match err {
        iv8_core::IV8Error::Js { message, .. } => {
            assert!(
                message.contains("Latin1") || message.contains("InvalidCharacter"),
                "msg: {}",
                message
            );
        }
        other => panic!("expected Js error, got: {:?}", other),
    }
}

#[test]
fn atob_invalid_base64_throws() {
    let mut kernel = common::make_kernel();
    let err = kernel
        .eval("atob('not valid base64!@#$')", EvalOpts::default())
        .unwrap_err();
    match err {
        iv8_core::IV8Error::Js { message, .. } => {
            assert!(
                message.contains("not correctly encoded") || message.contains("InvalidCharacter"),
                "msg: {}",
                message
            );
        }
        other => panic!("expected Js error, got: {:?}", other),
    }
}

#[test]
fn btoa_known_vectors() {
    let mut kernel = common::make_kernel();
    // Standard test vectors
    assert_eq!(
        kernel.eval_to_rust_value("btoa('f')"),
        RustValue::String("Zg==".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("btoa('fo')"),
        RustValue::String("Zm8=".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("btoa('foo')"),
        RustValue::String("Zm9v".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("btoa('foob')"),
        RustValue::String("Zm9vYg==".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("btoa('fooba')"),
        RustValue::String("Zm9vYmE=".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("btoa('foobar')"),
        RustValue::String("Zm9vYmFy".into())
    );
}

#[test]
fn atob_with_whitespace() {
    let mut kernel = common::make_kernel();
    // atob should ignore whitespace per spec
    assert_eq!(
        kernel.eval_to_rust_value("atob('aGVs bG8=')"),
        RustValue::String("hello".into())
    );
}

#[test]
fn atob_without_padding() {
    let mut kernel = common::make_kernel();
    // Some implementations accept base64 without padding
    assert_eq!(
        kernel.eval_to_rust_value("atob('aGVsbG8')"),
        RustValue::String("hello".into())
    );
}
