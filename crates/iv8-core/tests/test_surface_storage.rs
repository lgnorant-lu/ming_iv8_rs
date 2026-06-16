//! v0.8.51: Integration tests for storage surface (localStorage, sessionStorage).
mod common;

#[test]
fn test_local_storage_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof localStorage", "object");
}

#[test]
fn test_session_storage_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof sessionStorage", "object");
}

#[test]
fn test_local_storage_set_get() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("localStorage.setItem('k', 'v')");
    let val = common::to_str(&k.eval_to_rust_value("localStorage.getItem('k')"));
    assert_eq!(val, "v");
}

#[test]
fn test_local_storage_remove() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("localStorage.setItem('x', '1')");
    k.eval_to_rust_value("localStorage.removeItem('x')");
    let val = common::to_str(&k.eval_to_rust_value("localStorage.getItem('x')"));
    assert_eq!(val, "null");
}

#[test]
fn test_local_storage_length() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof localStorage.length", "number");
}

#[test]
fn test_local_storage_clear() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("localStorage.setItem('a', '1')");
    k.eval_to_rust_value("localStorage.setItem('b', '2')");
    k.eval_to_rust_value("localStorage.clear()");
    let len = common::to_str(&k.eval_to_rust_value("localStorage.length"));
    assert_eq!(len, "0");
}
