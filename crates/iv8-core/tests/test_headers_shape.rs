//! v0.8.72 Track C: Integration tests for Headers shape edge cases.
mod common;

use iv8_core::{EvalOpts, RustValue};

fn eval_result(k: &mut iv8_core::EmbeddedV8Kernel, expr: &str) -> RustValue {
    k.eval_to_rust_value(expr)
}

fn assert_str(k: &mut iv8_core::EmbeddedV8Kernel, expr: &str, expected: &str) {
    let v = eval_result(k, expr);
    let got = match &v {
        RustValue::String(s) => s.as_str(),
        _ => "",
    };
    assert_eq!(got, expected, "expr='{}' expected='{}' got='{:?}'", expr, expected, v);
}

fn make_kernel_with_headers() -> iv8_core::EmbeddedV8Kernel {
    let mut k = iv8_core::EmbeddedV8Kernel::new(Default::default()).unwrap();
    let mut headers = std::collections::HashMap::new();
    headers.insert("content-type".to_string(), "text/plain".to_string());
    k.add_resource("http://t/1", b"hello".to_vec(), 200, Some(headers));
    // Fetch and store response with headers on global
    k.eval("globalThis.r = undefined; fetch('http://t/1').then(function(resp){globalThis.r = resp;});", EvalOpts::default()).unwrap();
    k.drain_microtasks();
    k
}

#[test]
fn test_headers_case_insensitive_get() {
    let mut k = make_kernel_with_headers();
    k.eval("r.headers.set('X-Custom', 'val');", EvalOpts::default()).unwrap();
    assert_str(&mut k, "r.headers.get('x-custom')", "val");
    assert_str(&mut k, "r.headers.get('X-CUSTOM')", "val");
    assert_str(&mut k, "r.headers.get('X-Custom')", "val");
}

#[test]
fn test_headers_append_duplicate_comma_joined() {
    let mut k = make_kernel_with_headers();
    k.eval(
        "r.headers.append('X-List', 'a'); r.headers.append('X-List', 'b'); r.headers.append('X-List', 'c');",
        EvalOpts::default(),
    ).unwrap();
    assert_str(&mut k, "r.headers.get('x-list')", "a, b, c");
}

#[test]
fn test_headers_set_replaces_all() {
    let mut k = make_kernel_with_headers();
    k.eval(
        "r.headers.append('X-Dup', 'a'); r.headers.append('X-Dup', 'b'); r.headers.set('X-Dup', 'replaced');",
        EvalOpts::default(),
    ).unwrap();
    assert_str(&mut k, "r.headers.get('x-dup')", "replaced");
}

#[test]
fn test_headers_has_case_insensitive() {
    let mut k = make_kernel_with_headers();
    k.eval("r.headers.set('X-Has', 'yes');", EvalOpts::default()).unwrap();
    assert_str(&mut k, "String(r.headers.has('x-has'))", "true");
    assert_str(&mut k, "String(r.headers.has('X-HAS'))", "true");
    assert_str(&mut k, "String(r.headers.has('x-missing'))", "false");
}

#[test]
fn test_headers_delete_removes_all_duplicates() {
    let mut k = make_kernel_with_headers();
    k.eval(
        "r.headers.append('X-Del', 'a'); r.headers.append('X-Del', 'b'); r.headers.delete('X-Del');",
        EvalOpts::default(),
    ).unwrap();
    assert_str(&mut k, "String(r.headers.has('x-del'))", "false");
}

#[test]
fn test_headers_get_nonexistent_returns_null() {
    let mut k = make_kernel_with_headers();
    assert_str(&mut k, "String(r.headers.get('nonexistent'))", "null");
}

#[test]
fn test_headers_content_type_default() {
    let mut k = make_kernel_with_headers();
    assert_str(&mut k, "r.headers.get('content-type')", "text/plain");
}

#[test]
fn test_headers_content_type_case_insensitive() {
    let mut k = make_kernel_with_headers();
    assert_str(&mut k, "r.headers.get('Content-Type')", "text/plain");
    assert_str(&mut k, "r.headers.get('CONTENT-TYPE')", "text/plain");
}
