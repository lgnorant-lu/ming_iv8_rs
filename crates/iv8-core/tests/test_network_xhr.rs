#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]

//! Integration tests for XMLHttpRequest (Task 47).

use iv8_core::{EmbeddedV8Kernel, EvalOpts, KernelConfig, RustValue};

fn make_kernel() -> EmbeddedV8Kernel {
    EmbeddedV8Kernel::new(KernelConfig::default()).unwrap()
}

#[test]
fn xhr_class_exists() {
    let mut kernel = make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof XMLHttpRequest"),
        RustValue::String("function".into())
    );
}

#[test]
fn xhr_sync_get_from_bundle() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/data", b"hello xhr".to_vec(), 200, None);

    let result = kernel.eval_to_rust_value(
        r#"
        var xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://api.com/data', false);
        xhr.send();
        xhr.responseText
    "#,
    );
    assert_eq!(result, RustValue::String("hello xhr".into()));
}

#[test]
fn xhr_status_code() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/ok", b"".to_vec(), 200, None);

    let result = kernel.eval_to_rust_value(
        r#"
        var xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://api.com/ok', false);
        xhr.send();
        xhr.status
    "#,
    );
    assert_eq!(result, RustValue::Int(200));
}

#[test]
fn xhr_ready_state_done() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/x", b"x".to_vec(), 200, None);

    let result = kernel.eval_to_rust_value(
        r#"
        var xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://api.com/x', false);
        xhr.send();
        xhr.readyState
    "#,
    );
    assert_eq!(result, RustValue::Int(4)); // DONE
}

#[test]
fn xhr_onload_fires() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/cb", b"data".to_vec(), 200, None);

    let result = kernel.eval_to_rust_value(
        r#"
        var loaded = false;
        var xhr = new XMLHttpRequest();
        xhr.onload = function() { loaded = true; };
        xhr.open('GET', 'https://api.com/cb', false);
        xhr.send();
        loaded
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn xhr_onerror_on_missing_resource() {
    let mut kernel = make_kernel();

    let result = kernel.eval_to_rust_value(
        r#"
        var errored = false;
        var xhr = new XMLHttpRequest();
        xhr.onerror = function() { errored = true; };
        xhr.open('GET', 'https://missing.com/nope', false);
        xhr.send();
        errored
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn xhr_json_response() {
    let mut kernel = make_kernel();
    kernel.add_resource(
        "https://api.com/json",
        br#"{"key":"val"}"#.to_vec(),
        200,
        None,
    );

    let result = kernel.eval_to_rust_value(
        r#"
        var xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://api.com/json', false);
        xhr.send();
        JSON.parse(xhr.responseText).key
    "#,
    );
    assert_eq!(result, RustValue::String("val".into()));
}

#[test]
fn xhr_constants() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        XMLHttpRequest.DONE === 4 && XMLHttpRequest.OPENED === 1
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

// ─── Async XHR (Task 80) ────────────────────────────────────────────────────

#[test]
fn xhr_async_not_loaded_immediately() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/async", b"async data".to_vec(), 200, None);

    let result = kernel.eval_to_rust_value(
        r#"
        var xhr = new XMLHttpRequest();
        globalThis.asyncLoaded = false;
        xhr.onload = function() { globalThis.asyncLoaded = true; };
        xhr.open('GET', 'https://api.com/async', true);
        xhr.send();
        globalThis.asyncLoaded
    "#,
    );
    assert_eq!(result, RustValue::Bool(false)); // Not loaded yet
}

#[test]
fn xhr_async_loads_after_advance() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/async", b"async data".to_vec(), 200, None);

    kernel.eval_to_rust_value(
        r#"
        var xhr = new XMLHttpRequest();
        globalThis.asyncLoaded = false;
        globalThis.asyncData = '';
        xhr.onload = function() {
            globalThis.asyncLoaded = true;
            globalThis.asyncData = xhr.responseText;
        };
        xhr.open('GET', 'https://api.com/async', true);
        xhr.send();
    "#,
    );

    // Advance event loop to trigger the async callback
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(10)");

    assert_eq!(
        kernel.eval_to_rust_value("globalThis.asyncLoaded"),
        RustValue::Bool(true)
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.asyncData"),
        RustValue::String("async data".into())
    );
}

#[test]
fn xhr_async_status_zero_before_advance() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/x", b"x".to_vec(), 200, None);

    let result = kernel.eval_to_rust_value(
        r#"
        var xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://api.com/x', true);
        xhr.send();
        xhr.status
    "#,
    );
    assert_eq!(result, RustValue::Int(0)); // Status 0 before completion
}

#[test]
fn xhr_sync_still_works() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/sync", b"sync data".to_vec(), 200, None);

    let result = kernel.eval_to_rust_value(
        r#"
        var xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://api.com/sync', false);
        xhr.send();
        xhr.responseText
    "#,
    );
    assert_eq!(result, RustValue::String("sync data".into()));
}

#[test]
fn xhr_async_error_after_advance() {
    let mut kernel = make_kernel();
    // No resource registered for this URL

    kernel.eval_to_rust_value(
        r#"
        var xhr = new XMLHttpRequest();
        globalThis.asyncError = false;
        xhr.onerror = function() { globalThis.asyncError = true; };
        xhr.open('GET', 'https://missing.com/nope', true);
        xhr.send();
    "#,
    );

    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(10)");
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.asyncError"),
        RustValue::Bool(true)
    );
}
