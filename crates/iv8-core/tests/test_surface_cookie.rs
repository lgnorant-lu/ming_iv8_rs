//! v0.8.51 S3: Integration tests for document.cookie surface.
mod common;

#[test]
fn test_cookie_set_and_get() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("document.cookie = 'a=1'");
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("a=1"), "cookie missing a=1: {}", val);
}

#[test]
fn test_cookie_multiple_values() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("document.cookie = 'x=hello'");
    k.eval_to_rust_value("document.cookie = 'y=world'");
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("x=hello"), "missing x=hello: {}", val);
    assert!(val.contains("y=world"), "missing y=world: {}", val);
}

#[test]
fn test_cookie_max_age_zero_removes() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("document.cookie = 'temp=1'");
    k.eval_to_rust_value("document.cookie = 'temp=; Max-Age=0'");
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(!val.contains("temp=1"), "cookie not removed: {}", val);
}

#[test]
fn test_cookie_readback_probe() {
    let mut k = common::make_kernel();
    // Simulate RS enable_xxx test cookie probe
    k.eval_to_rust_value("document.cookie = 'enable_Test=true'");
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("enable_Test=true"), "probe failed: {}", val);
}

#[test]
fn test_cookie_empty_after_clear() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("document.cookie = 'a=1'");
    k.eval_to_rust_value("document.cookie = 'b=2'");
    k.eval_to_rust_value("document.cookie = 'a=; Max-Age=0'");
    k.eval_to_rust_value("document.cookie = 'b=; Max-Age=0'");
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(!val.contains("=1"), "should be empty, got: {}", val);
}
