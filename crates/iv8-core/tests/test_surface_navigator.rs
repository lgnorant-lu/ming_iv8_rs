// v0.8.51 S3: Integration tests for Navigator surface.
mod common;

#[test]
fn test_navigator_user_agent() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("navigator.userAgent"));
    assert!(val.contains("Chrome"), "UA missing Chrome: {}", val);
    assert!(val.contains("Windows"), "UA missing Windows: {}", val);
}

#[test]
fn test_navigator_platform() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.platform", "string");
}

#[test]
fn test_navigator_webdriver_false() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("navigator.webdriver"));
    assert_eq!(val, "false");
}

#[test]
fn test_navigator_java_enabled_method() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.javaEnabled", "function");
    let val = common::to_str(&k.eval_to_rust_value("navigator.javaEnabled()"));
    assert_eq!(val, "false");
}

#[test]
fn test_navigator_plugins_empty() {
    let mut k = common::make_kernel();
    let len = common::to_str(&k.eval_to_rust_value("navigator.plugins.length"));
    assert_eq!(len, "5");
    common::assert_js_str(&mut k, "typeof navigator.plugins[0]", "object");
}

#[test]
fn test_navigator_mime_types_empty() {
    let mut k = common::make_kernel();
    let len = common::to_str(&k.eval_to_rust_value("navigator.mimeTypes.length"));
    assert_eq!(len, "2");
}

#[test]
fn test_navigator_hardware_concurrency() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.hardwareConcurrency", "number");
}

#[test]
fn test_navigator_language() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.language", "string");
}

#[test]
fn test_navigator_connection_getter() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.connection", "object");
    common::assert_js_str(
        &mut k,
        "typeof navigator.connection.effectiveType",
        "string",
    );
    common::assert_js_str(&mut k, "typeof navigator.connection.downlink", "number");
    common::assert_js_str(&mut k, "typeof navigator.connection.rtt", "number");
    common::assert_js_str(&mut k, "typeof navigator.connection.saveData", "boolean");
    common::assert_js_str(&mut k, "typeof navigator.connection.type", "string");
}

#[test]
fn test_navigator_connection_descriptor_on_prototype() {
    let mut k = common::make_kernel();
    let own = k.eval_to_rust_value("Object.getOwnPropertyDescriptor(navigator, 'connection')");
    let own_str = common::to_str(&own);
    assert!(
        own_str == "undefined" || own_str == "null",
        "connection should not be own property: {}",
        own_str
    );

    let has = k.eval_to_rust_value("'connection' in navigator");
    assert_eq!(
        common::to_str(&has),
        "true",
        "connection should be in navigator"
    );
}

#[test]
fn test_navigator_get_battery_method() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.getBattery", "function");
}

#[test]
fn test_navigator_get_battery_descriptor_on_prototype() {
    let mut k = common::make_kernel();
    let proto_check = k.eval_to_rust_value("'prototype' in navigator.getBattery");
    assert_eq!(
        common::to_str(&proto_check),
        "false",
        "getBattery should not have prototype property"
    );
}

#[test]
fn test_navigator_send_beacon_method() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.sendBeacon", "function");
    let val = common::to_str(&k.eval_to_rust_value("navigator.sendBeacon('http://x', 'a')"));
    assert_eq!(val, "true");
}

#[test]
fn test_navigator_geolocation_getter() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.geolocation", "object");
    common::assert_js_str(
        &mut k,
        "typeof navigator.geolocation.getCurrentPosition",
        "function",
    );
    common::assert_js_str(
        &mut k,
        "typeof navigator.geolocation.watchPosition",
        "function",
    );
    common::assert_js_str(
        &mut k,
        "typeof navigator.geolocation.clearWatch",
        "function",
    );
}

#[test]
fn test_navigator_clipboard_getter() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.clipboard", "object");
    common::assert_js_str(&mut k, "typeof navigator.clipboard.readText", "function");
    common::assert_js_str(&mut k, "typeof navigator.clipboard.writeText", "function");
}

#[test]
fn test_navigator_credentials_getter() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.credentials", "object");
    common::assert_js_str(&mut k, "typeof navigator.credentials.get", "function");
    common::assert_js_str(&mut k, "typeof navigator.credentials.create", "function");
    common::assert_js_str(&mut k, "typeof navigator.credentials.store", "function");
    common::assert_js_str(
        &mut k,
        "typeof navigator.credentials.preventSilentAccess",
        "function",
    );
}

#[test]
fn test_navigator_new_properties_not_enumerable() {
    let mut k = common::make_kernel();
    let checks = ["connection", "geolocation"];
    for prop in &checks {
        let js = format!(
            "Object.prototype.propertyIsEnumerable.call(navigator, '{}')",
            prop
        );
        common::assert_js_str(&mut k, &js, "false");
    }
}

#[test]
fn test_navigator_new_properties_tostring_native_code() {
    let mut k = common::make_kernel();
    let checks = ["getBattery", "sendBeacon"];
    for prop in &checks {
        let js = format!("navigator.{}.toString().includes('[native code]')", prop);
        common::assert_js_str(&mut k, &js, "true");
    }
}

#[test]
fn test_navigator_custom_profile() {
    use iv8_core::shims::browser_profile::{BrowserProfile, DEFAULT_PROFILE};
    let profile = BrowserProfile {
        user_agent: &*Box::leak("TestAgent/9.9".to_string().into_boxed_str()),
        platform: &*Box::leak("TestOS".to_string().into_boxed_str()),
        language: &*Box::leak("test-LANG".to_string().into_boxed_str()),
        ..DEFAULT_PROFILE
    };
    let mut k = common::make_kernel_with_profile(profile);
    common::assert_js_str(&mut k, "navigator.userAgent", "TestAgent/9.9");
    common::assert_js_str(&mut k, "navigator.platform", "TestOS");
    common::assert_js_str(&mut k, "navigator.language", "test-LANG");
    common::assert_js_str(&mut k, "navigator.appVersion", DEFAULT_PROFILE.app_version);
}

#[test]
fn test_navigator_default_profile_equivalence() {
    let mut k = common::make_kernel();
    let ua = common::to_str(&k.eval_to_rust_value("navigator.userAgent"));
    assert!(ua.contains("Chrome"), "default UA missing Chrome: {}", ua);
    assert!(ua.contains("Windows"), "default UA missing Windows: {}", ua);
    let plat = common::to_str(&k.eval_to_rust_value("navigator.platform"));
    assert!(!plat.is_empty(), "default platform is empty");
    let lang = common::to_str(&k.eval_to_rust_value("navigator.language"));
    assert!(!lang.is_empty(), "default language is empty");
}

// v0.8.60: generated Navigator template unification.
// Generated skeleton properties (bluetooth, hid, usb) are now
// visible in JS runtime — native Navigator inherits from generated
// create_navigator_template via FunctionTemplate::inherit().
// Note: gpu is not a Navigator attribute in W3C WebIDL webref;
// it is on WorkerNavigator (not yet generated).

#[test]
fn test_navigator_generated_skeleton_visible() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.bluetooth", "object");
    common::assert_js_str(&mut k, "typeof navigator.hid", "object");
    common::assert_js_str(&mut k, "typeof navigator.usb", "object");
    // Native getters still take precedence over generated skeletons
    common::assert_js_str(&mut k, "typeof navigator.userAgent", "string");
}

// v0.8.61: native stubs for high-signal Navigator methods.

#[test]
fn test_navigator_get_gamepads_returns_empty_array() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.getGamepads", "function");
    common::assert_js_str(&mut k, "navigator.getGamepads().length", "0");
    common::assert_js_str(&mut k, "Array.isArray(navigator.getGamepads())", "true");
}

#[test]
fn test_navigator_request_media_key_system_access_returns_promise() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof navigator.requestMediaKeySystemAccess",
        "function",
    );
    common::assert_js_str(
        &mut k,
        "navigator.requestMediaKeySystemAccess('com.widevine.alpha').constructor.name",
        "Promise",
    );
}

#[test]
fn test_navigator_request_midi_access_returns_promise() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.requestMIDIAccess", "function");
    common::assert_js_str(
        &mut k,
        "navigator.requestMIDIAccess().constructor.name",
        "Promise",
    );
}

// v0.8.62: conditional Navigator property exposure
// Desktop profile (mobile_profile=false, chrome_version=131)
// hides mobile-only and legacy properties.

#[test]
fn test_conditional_share_hidden_on_desktop() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.share", "undefined");
    common::assert_js_str(&mut k, "typeof navigator.canShare", "undefined");
}

#[test]
fn test_conditional_vibrate_hidden_on_desktop() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.vibrate", "undefined");
}

// Mobile profile: share/canShare visible
#[test]
fn test_conditional_share_visible_on_mobile() {
    use iv8_core::shims::browser_profile::BrowserProfile;
    let profile = BrowserProfile {
        mobile_profile: true,
        ..iv8_core::shims::browser_profile::DEFAULT_PROFILE.clone()
    };
    let mut k = common::make_kernel_with_profile(profile);
    // share/canShare are generated as methods (typeof "function")
    common::assert_js_str(&mut k, "typeof navigator.share", "function");
    common::assert_js_str(&mut k, "typeof navigator.canShare", "function");
}
