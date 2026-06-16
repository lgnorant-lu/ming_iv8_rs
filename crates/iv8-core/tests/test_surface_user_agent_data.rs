//! v0.8.51: Integration tests for NavigatorUAData surface.
mod common;

#[test]
fn test_user_agent_data_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.userAgentData", "object");
}

#[test]
fn test_user_agent_data_platform() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.userAgentData.platform", "string");
}

#[test]
fn test_user_agent_data_mobile() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.userAgentData.mobile", "boolean");
}

#[test]
fn test_user_agent_data_brands() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "Array.isArray(navigator.userAgentData.brands)", "true");
    let len = common::to_str(&k.eval_to_rust_value("navigator.userAgentData.brands.length"));
    assert!(len != "0", "brands should not be empty, got {}", len);
}

#[test]
fn test_user_agent_data_brands_has_brand_and_version() {
    let mut k = common::make_kernel();
    let b = k.eval_to_rust_value("navigator.userAgentData.brands[0].brand");
    let v = k.eval_to_rust_value("navigator.userAgentData.brands[0].version");
    assert!(!common::to_str(&b).is_empty(), "brand name empty");
    assert!(!common::to_str(&v).is_empty(), "brand version empty");
}

#[test]
fn test_user_agent_data_get_high_entropy_values_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof navigator.userAgentData.getHighEntropyValues",
        "function",
    );
}
