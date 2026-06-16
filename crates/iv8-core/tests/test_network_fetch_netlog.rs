#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]

//! Test L-04 fix: fetch() requests are recorded to __iv8__.netLog.entries.
//!
//! v0.1: only XHR was recorded; fetch was invisible to netLog.
//! v0.2: fetch() also records, with the same { method, url, headers, body } shape.

use iv8_core::convert::RustValue;
use iv8_core::kernel::{EvalOpts, KernelConfig};
use iv8_core::EmbeddedV8Kernel;

fn make_kernel() -> EmbeddedV8Kernel {
    EmbeddedV8Kernel::new(KernelConfig::default()).unwrap()
}

#[test]
fn fetch_get_records_url_in_netlog() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/x", b"hello".to_vec(), 200, None);
    kernel
        .eval(
            r#"fetch('https://api.com/x').then(r => r.text())"#,
            EvalOpts::default(),
        )
        .unwrap();

    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries.length"),
        RustValue::Int(1)
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].url"),
        RustValue::String("https://api.com/x".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].method"),
        RustValue::String("GET".into())
    );
}

#[test]
fn fetch_post_records_method_and_body() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/post", b"ok".to_vec(), 200, None);
    kernel
        .eval(
            r#"
            fetch('https://api.com/post', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({ a: 1 })
            })
        "#,
            EvalOpts::default(),
        )
        .unwrap();

    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].method"),
        RustValue::String("POST".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].body"),
        RustValue::String(r#"{"a":1}"#.into())
    );
}

#[test]
fn fetch_records_headers_as_lowercase_pairs() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/h", b"".to_vec(), 200, None);
    kernel
        .eval(
            r#"
            fetch('https://api.com/h', {
                method: 'POST',
                headers: { 'Content-Type': 'text/plain', 'X-Custom': 'abc' }
            })
        "#,
            EvalOpts::default(),
        )
        .unwrap();

    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].headers.length"),
        RustValue::Int(2)
    );
    // Headers are stored as [name_lowercase, value] pairs
    let result = kernel.eval_to_rust_value(
        r#"
        var pairs = __iv8__.netLog.entries[0].headers;
        var found_ct = false, found_xc = false;
        for (var i = 0; i < pairs.length; i++) {
            if (pairs[i][0] === 'content-type' && pairs[i][1] === 'text/plain') found_ct = true;
            if (pairs[i][0] === 'x-custom' && pairs[i][1] === 'abc') found_xc = true;
        }
        found_ct && found_xc
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn fetch_records_even_on_network_error() {
    // Even when the URL is offline (rejects), the request is still logged.
    let mut kernel = make_kernel();
    let _ = kernel.eval(
        r#"fetch('https://offline.example/missing').catch(() => {})"#,
        EvalOpts::default(),
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries.length"),
        RustValue::Int(1)
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].url"),
        RustValue::String("https://offline.example/missing".into())
    );
}

#[test]
fn fetch_and_xhr_share_same_netlog() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/a", b"".to_vec(), 200, None);
    kernel.add_resource("https://api.com/b", b"".to_vec(), 200, None);

    // Mix one fetch + one XHR
    kernel
        .eval(
            r#"
            fetch('https://api.com/a');
            var xhr = new XMLHttpRequest();
            xhr.open('GET', 'https://api.com/b');
            xhr.send();
        "#,
            EvalOpts::default(),
        )
        .unwrap();

    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries.length"),
        RustValue::Int(2)
    );
    // Order matches call order
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].url"),
        RustValue::String("https://api.com/a".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[1].url"),
        RustValue::String("https://api.com/b".into())
    );
}

#[test]
fn fetch_default_method_is_get() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/d", b"".to_vec(), 200, None);
    kernel
        .eval(r#"fetch('https://api.com/d')"#, EvalOpts::default())
        .unwrap();
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].method"),
        RustValue::String("GET".into())
    );
}

#[test]
fn fetch_method_normalized_to_uppercase() {
    let mut kernel = make_kernel();
    kernel.add_resource("https://api.com/u", b"".to_vec(), 200, None);
    kernel
        .eval(
            r#"fetch('https://api.com/u', { method: 'put' })"#,
            EvalOpts::default(),
        )
        .unwrap();
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].method"),
        RustValue::String("PUT".into())
    );
}
