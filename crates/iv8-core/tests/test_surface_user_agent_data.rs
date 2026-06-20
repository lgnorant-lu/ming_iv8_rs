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

#[test]
fn test_uadata_custom_profile() {
    use iv8_core::shims::browser_profile::{BrowserProfile, DEFAULT_PROFILE};
    let profile = BrowserProfile {
        ua_platform: &*Box::leak("Android".to_string().into_boxed_str()),
        ua_mobile: true,
        ua_architecture: &*Box::leak("arm".to_string().into_boxed_str()),
        ua_bitness: &*Box::leak("32".to_string().into_boxed_str()),
        ua_model: &*Box::leak("Pixel 9".to_string().into_boxed_str()),
        ..DEFAULT_PROFILE
    };
    let mut k = common::make_kernel_with_profile(profile);
    common::assert_js_str(&mut k, "navigator.userAgentData.platform", "Android");
    common::assert_js_str(&mut k, "navigator.userAgentData.mobile", "true");
}
