//! v0.8.51 S3: Integration tests for location surface.
mod common;

#[test]
fn test_location_href_getter() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof location.href", "string");
}

#[test]
fn test_location_href_setter_persists() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("location.href = 'https://example.com/path'");
    let val = common::to_str(&k.eval_to_rust_value("location.href"));
    assert_eq!(val, "https://example.com/path");
}

#[test]
fn test_location_protocol_host_hostname() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("location.protocol = 'https:'");
    k.eval_to_rust_value("location.hostname = 'example.com'");
    k.eval_to_rust_value("location.host = 'example.com:443'");
    assert_eq!(
        common::to_str(&k.eval_to_rust_value("location.protocol")),
        "https:"
    );
    assert_eq!(
        common::to_str(&k.eval_to_rust_value("location.hostname")),
        "example.com"
    );
}

#[test]
fn test_location_to_string_returns_href() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("location.href = 'https://a.com/'");
    let val = common::to_str(&k.eval_to_rust_value("'' + location"));
    assert_eq!(val, "https://a.com/");
}

#[test]
fn test_location_assign_replace_reload_exist() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof location.assign", "function");
    common::assert_js_str(&mut k, "typeof location.replace", "function");
    common::assert_js_str(&mut k, "typeof location.reload", "function");
}

#[test]
fn test_location_native_descriptor_on_prototype() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value(
        "'get' in Object.getOwnPropertyDescriptor(Object.getPrototypeOf(location), 'href')",
    ));
    assert_eq!(val, "true", "location.href missing native getter");
}
