// v0.8.51 S3: Integration tests for document.cookie surface.
// v0.8.72 Track B: + cookie security attribute tests.
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

// ── v0.8.72 Track B: cookie security attributes ──

#[test]
fn test_cookie_secure_attribute_stored() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("document.cookie = 'sec=1; Secure'");
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("sec=1"), "Secure cookie should be visible: {}", val);
}

#[test]
fn test_cookie_path_attribute_stored() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("document.cookie = 'p=1; Path=/'");
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("p=1"), "Path=/ cookie should be visible: {}", val);
}

#[test]
fn test_cookie_samesite_attribute_stored() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("document.cookie = 'ss=1; SameSite=Lax'");
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("ss=1"));
}

#[test]
fn test_cookie_max_age_positive() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("document.cookie = 'ma=1; Max-Age=3600'");
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("ma=1"), "Positive Max-Age should store: {}", val);
}

#[test]
fn test_cookie_expires_attribute_stored() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value(
        "document.cookie = 'exp_cookie=1; Expires=Fri, 31 Dec 2099 23:59:59 GMT'",
    );
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("exp_cookie=1"));
}

#[test]
fn test_cookie_path_filtering_root() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("document.cookie = 'private=secret; Path=/app'");
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(
        !val.contains("private=secret"),
        "Path=/app cookie should be hidden at root: got '{}'",
        val
    );
}

#[test]
fn test_cookie_multiple_attributes() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value(
        "document.cookie = 'multi=val; Path=/; Secure; SameSite=Strict; Max-Age=7200'",
    );
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("multi=val"));
}

#[test]
fn test_cookie_value_with_equals() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("document.cookie = 'token=abc=def=ghi'");
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("token=abc=def=ghi"));
}

#[test]
fn test_cookie_backward_compat_plain_value() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("document.cookie = 'simple=42'");
    let val = common::to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("simple=42"));
}
