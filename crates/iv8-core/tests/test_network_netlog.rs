#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;


// Integration tests for __iv8__.netLog (Task 49).
// Acceptance criteria:
// - __iv8__.netLog.entries is an array
// - XHR requests are recorded with method, url, headers, body
// - Multiple requests accumulate

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};
#[test]
fn netlog_exists() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof __iv8__.netLog"),
        RustValue::String("object".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("Array.isArray(__iv8__.netLog.entries)"),
        RustValue::Bool(true)
    );
}

#[test]
fn netlog_initially_empty() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries.length"),
        RustValue::Int(0)
    );
}

#[test]
fn netlog_records_xhr() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://api.com/test", b"ok".to_vec(), 200, None);

    kernel.eval_to_rust_value(
        r#"
        var xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://api.com/test?q=hello', false);
        xhr.send();
    "#,
    );

    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries.length"),
        RustValue::Int(1)
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].method"),
        RustValue::String("GET".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].url"),
        RustValue::String("https://api.com/test?q=hello".into())
    );
}

#[test]
fn netlog_records_headers() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://api.com/h", b"".to_vec(), 200, None);

    kernel.eval_to_rust_value(
        r#"
        var xhr = new XMLHttpRequest();
        xhr.open('POST', 'https://api.com/h', false);
        xhr.setRequestHeader('Content-Type', 'application/json');
        xhr.setRequestHeader('X-Token', 'abc123');
        xhr.send('{"data":1}');
    "#,
    );

    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].method"),
        RustValue::String("POST".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].headers.length"),
        RustValue::Int(2)
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].headers[0][0]"),
        RustValue::String("content-type".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].body"),
        RustValue::String("{\"data\":1}".into())
    );
}

#[test]
fn netlog_multiple_requests() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://a.com/1", b"".to_vec(), 200, None);
    kernel.add_resource("https://a.com/2", b"".to_vec(), 200, None);

    kernel.eval_to_rust_value(
        r#"
        var xhr1 = new XMLHttpRequest();
        xhr1.open('GET', 'https://a.com/1', false);
        xhr1.send();
        var xhr2 = new XMLHttpRequest();
        xhr2.open('GET', 'https://a.com/2', false);
        xhr2.send();
    "#,
    );

    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries.length"),
        RustValue::Int(2)
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].url"),
        RustValue::String("https://a.com/1".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[1].url"),
        RustValue::String("https://a.com/2".into())
    );
}

#[test]
fn netlog_async_xhr_recorded_immediately() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://api.com/async", b"data".to_vec(), 200, None);

    kernel.eval_to_rust_value(
        r#"
        var xhr = new XMLHttpRequest();
        xhr.open('GET', 'https://api.com/async', true);
        xhr.send();
    "#,
    );

    // netLog should record immediately on send(), not after advance
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries.length"),
        RustValue::Int(1)
    );
    assert_eq!(
        kernel.eval_to_rust_value("__iv8__.netLog.entries[0].url"),
        RustValue::String("https://api.com/async".into())
    );
}
