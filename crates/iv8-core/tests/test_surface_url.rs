#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]

//! Integration tests for URL/URLSearchParams (Task 72).
//! Acceptance criteria:
//! - new URL('https://example.com/path?a=1') correctly parses
//! - url.hostname / pathname / searchParams correct
//! - new URLSearchParams('a=1&b=2') iterable

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};

fn make_kernel() -> EmbeddedV8Kernel {
    EmbeddedV8Kernel::new(KernelConfig::default()).unwrap()
}

// ─── URL ────────────────────────────────────────────────────────────────────

#[test]
fn url_constructor_exists() {
    let mut kernel = make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof URL"),
        RustValue::String("function".into())
    );
}

#[test]
fn url_parse_full() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value(r#"
        var u = new URL('https://www.example.com:8080/path/page?q=1&r=2#hash');
        u.protocol + '|' + u.hostname + '|' + u.port + '|' + u.pathname + '|' + u.search + '|' + u.hash
    "#);
    assert_eq!(
        result,
        RustValue::String("https:|www.example.com|8080|/path/page|?q=1&r=2|#hash".into())
    );
}

#[test]
fn url_origin() {
    let mut kernel = make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new URL('https://example.com/path').origin"),
        RustValue::String("https://example.com".into())
    );
}

#[test]
fn url_href() {
    let mut kernel = make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new URL('https://example.com/path?x=1').href"),
        RustValue::String("https://example.com/path?x=1".into())
    );
}

#[test]
fn url_to_string() {
    let mut kernel = make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new URL('https://example.com/').toString()"),
        RustValue::String("https://example.com/".into())
    );
}

#[test]
fn url_search_params() {
    let mut kernel = make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new URL('https://x.com/?a=1&b=2').searchParams.get('b')"),
        RustValue::String("2".into())
    );
}

#[test]
fn url_invalid_throws() {
    let mut kernel = make_kernel();
    let err = kernel
        .eval("new URL('not a url')", iv8_core::EvalOpts::default())
        .unwrap_err();
    match err {
        iv8_core::IV8Error::Js { message, .. } => {
            assert!(message.contains("Invalid URL"), "msg: {}", message);
        }
        other => panic!("expected Js error, got: {:?}", other),
    }
}

// ─── URLSearchParams ────────────────────────────────────────────────────────

#[test]
fn url_search_params_constructor_exists() {
    let mut kernel = make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof URLSearchParams"),
        RustValue::String("function".into())
    );
}

#[test]
fn url_search_params_from_string() {
    let mut kernel = make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new URLSearchParams('a=1&b=2').get('a')"),
        RustValue::String("1".into())
    );
}

#[test]
fn url_search_params_has() {
    let mut kernel = make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new URLSearchParams('x=1').has('x')"),
        RustValue::Bool(true)
    );
    assert_eq!(
        kernel.eval_to_rust_value("new URLSearchParams('x=1').has('y')"),
        RustValue::Bool(false)
    );
}

#[test]
fn url_search_params_set() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var p = new URLSearchParams('a=1');
        p.set('a', '2');
        p.get('a')
    "#,
    );
    assert_eq!(result, RustValue::String("2".into()));
}

#[test]
fn url_search_params_append() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var p = new URLSearchParams('a=1');
        p.append('a', '2');
        p.getAll('a').length
    "#,
    );
    assert_eq!(result, RustValue::Int(2));
}

#[test]
fn url_search_params_delete() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var p = new URLSearchParams('a=1&b=2');
        p.delete('a');
        p.has('a')
    "#,
    );
    assert_eq!(result, RustValue::Bool(false));
}

#[test]
fn url_search_params_to_string() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value("new URLSearchParams('a=1&b=2').toString()");
    assert_eq!(result, RustValue::String("a=1&b=2".into()));
}

#[test]
fn url_search_params_size() {
    let mut kernel = make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("new URLSearchParams('a=1&b=2&c=3').size"),
        RustValue::Int(3)
    );
}

#[test]
fn url_search_params_from_object() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var p = new URLSearchParams({foo: 'bar', baz: '42'});
        p.get('foo') + ',' + p.get('baz')
    "#,
    );
    assert_eq!(result, RustValue::String("bar,42".into()));
}

#[test]
fn url_search_params_encoded() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var p = new URLSearchParams();
        p.set('q', 'hello world');
        p.toString()
    "#,
    );
    assert_eq!(result, RustValue::String("q=hello%20world".into()));
}
