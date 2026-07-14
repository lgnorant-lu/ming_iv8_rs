#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// Integration tests for fetch() JS binding (Task 46).

use iv8_core::{EmbeddedV8Kernel, EvalOpts, KernelConfig, RustValue};
#[test]
fn fetch_exists() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof fetch"),
        RustValue::String("function".into())
    );
}

#[test]
fn fetch_returns_promise() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("fetch('http://x.com') instanceof Promise");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn fetch_resolves_with_resource() {
    let mut kernel = common::make_kernel();
    kernel.add_resource(
        "https://api.example.com/data",
        b"hello world".to_vec(),
        200,
        None,
    );

    // Use .then() to capture the resolved value
    kernel
        .eval(
            r#"
        globalThis.fetchResult = null;
        fetch('https://api.example.com/data').then(function(response) {
            globalThis.fetchResult = response.status;
        });
    "#,
            EvalOpts::default(),
        )
        .unwrap();

    // Drain microtasks to resolve the promise
    kernel.drain_microtasks();

    let result = kernel.eval_to_rust_value("globalThis.fetchResult");
    assert_eq!(result, RustValue::Int(200));
}

#[test]
fn fetch_response_ok() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://x.com/ok", b"".to_vec(), 200, None);

    kernel
        .eval(
            r#"
        globalThis.isOk = null;
        fetch('https://x.com/ok').then(function(r) { globalThis.isOk = r.ok; });
    "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();

    assert_eq!(
        kernel.eval_to_rust_value("globalThis.isOk"),
        RustValue::Bool(true)
    );
}

#[test]
fn fetch_response_text() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://x.com/text", b"hello".to_vec(), 200, None);

    kernel
        .eval(
            r#"
        globalThis.body = null;
        fetch('https://x.com/text')
            .then(function(r) { return r.text(); })
            .then(function(t) { globalThis.body = t; });
    "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();

    assert_eq!(
        kernel.eval_to_rust_value("globalThis.body"),
        RustValue::String("hello".into())
    );
}

#[test]
fn fetch_response_json() {
    let mut kernel = common::make_kernel();
    kernel.add_resource(
        "https://x.com/json",
        br#"{"key":"value"}"#.to_vec(),
        200,
        None,
    );

    kernel
        .eval(
            r#"
        globalThis.parsed = null;
        fetch('https://x.com/json')
            .then(function(r) { return r.json(); })
            .then(function(j) { globalThis.parsed = j.key; });
    "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();

    assert_eq!(
        kernel.eval_to_rust_value("globalThis.parsed"),
        RustValue::String("value".into())
    );
}

#[test]
fn fetch_missing_url_resolves_404_not_reject() {
    // Browser fetch: network failure can reject; offline ResourceBundle path
    // resolves with Response(status=404) when URL not registered (fetch.rs).
    let mut kernel = common::make_kernel();

    kernel
        .eval(
            r#"
        globalThis.missingStatus = -1;
        globalThis.missingOk = null;
        globalThis.missingRejected = false;
        fetch('https://unknown.com/missing')
            .then(function(r) {
                globalThis.missingStatus = r.status;
                globalThis.missingOk = r.ok;
            })
            .catch(function() { globalThis.missingRejected = true; });
    "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();

    assert_eq!(
        kernel.eval_to_rust_value("globalThis.missingRejected"),
        RustValue::Bool(false)
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.missingStatus"),
        RustValue::Int(404)
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.missingOk"),
        RustValue::Bool(false)
    );
}

#[test]
fn fetch_404_resolves_not_rejects() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://x.com/404", b"not found".to_vec(), 404, None);

    kernel
        .eval(
            r#"
        globalThis.got404 = false;
        fetch('https://x.com/404').then(function(r) {
            globalThis.got404 = (r.status === 404 && r.ok === false);
        });
    "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();

    assert_eq!(
        kernel.eval_to_rust_value("globalThis.got404"),
        RustValue::Bool(true)
    );
}

#[test]
fn fetch_response_ok_status_no_recursion() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://x.com/regr", b"body".to_vec(), 200, None);
    kernel
        .eval(
            r#"
        globalThis.regrOk = null;
        globalThis.regrStatus = -1;
        fetch('https://x.com/regr').then(function(r) {
            globalThis.regrOk = r.ok;
            globalThis.regrStatus = r.status;
        });
        "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.regrOk"),
        RustValue::Bool(true)
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.regrStatus"),
        RustValue::Int(200)
    );
}

#[test]
fn fetch_response_404_ok_status_no_recursion() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://x.com/nope", b"not found".to_vec(), 404, None);
    kernel
        .eval(
            r#"
        globalThis.r4ok = null;
        globalThis.r4st = -1;
        fetch('https://x.com/nope').then(function(r) {
            globalThis.r4ok = r.ok;
            globalThis.r4st = r.status;
        });
        "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.r4ok"),
        RustValue::Bool(false)
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.r4st"),
        RustValue::Int(404)
    );
}
