//! v0.8.65: Integration tests for window dimension native accessors.
mod common;

#[test]
fn test_window_inner_width_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof window.innerWidth", "number");
}

#[test]
fn test_window_inner_height_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof window.innerHeight", "number");
}

#[test]
fn test_window_outer_width_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof window.outerWidth", "number");
}

#[test]
fn test_window_outer_height_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof window.outerHeight", "number");
}

#[test]
fn test_window_device_pixel_ratio_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof window.devicePixelRatio", "number");
}

#[test]
fn test_device_pixel_ratio_is_native_getter() {
    let mut k = common::make_kernel();
    let desc = k.eval_to_rust_value(
        "(function(){var d=Object.getOwnPropertyDescriptor(window,'devicePixelRatio');return typeof d.get==='function'&&d.set===undefined})()"
    );
    assert_eq!(desc, iv8_core::convert::RustValue::Bool(true),
        "devicePixelRatio must be a native getter with undefined setter");
}

#[test]
fn test_window_descriptor_getter_is_function() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof Object.getOwnPropertyDescriptor(window, 'innerWidth').get",
        "function",
    );
    common::assert_js_str(
        &mut k,
        "typeof Object.getOwnPropertyDescriptor(window, 'outerWidth').get",
        "function",
    );
}

#[test]
fn test_window_descriptor_setter_is_undefined() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "Object.getOwnPropertyDescriptor(window, 'innerWidth').set === undefined",
        "true",
    );
}

#[test]
fn test_window_outer_gte_inner() {
    let mut k = common::make_kernel();
    let w = k.eval_to_rust_value("window.outerWidth");
    let iw = k.eval_to_rust_value("window.innerWidth");
    let h = k.eval_to_rust_value("window.outerHeight");
    let ih = k.eval_to_rust_value("window.innerHeight");
    let w_val: f64 = common::to_str(&w).parse().unwrap_or(0.0);
    let iw_val: f64 = common::to_str(&iw).parse().unwrap_or(0.0);
    let h_val: f64 = common::to_str(&h).parse().unwrap_or(0.0);
    let ih_val: f64 = common::to_str(&ih).parse().unwrap_or(0.0);
    assert!(
        w_val >= iw_val,
        "outerWidth {} < innerWidth {}",
        w_val,
        iw_val
    );
    assert!(
        h_val >= ih_val,
        "outerHeight {} < innerHeight {}",
        h_val,
        ih_val
    );
}

#[test]
fn test_screen_avail_not_exceed_screen() {
    let mut k = common::make_kernel();
    let w = k.eval_to_rust_value("screen.width");
    let aw = k.eval_to_rust_value("screen.availWidth");
    let w_val: f64 = common::to_str(&w).parse().unwrap_or(0.0);
    let aw_val: f64 = common::to_str(&aw).parse().unwrap_or(0.0);
    assert!(
        aw_val <= w_val,
        "availWidth {} > width {}",
        aw_val,
        w_val
    );
}

#[test]
fn test_window_identity_preserved() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "window === globalThis", "true");
    common::assert_js_str(&mut k, "window === self", "true");
    common::assert_js_str(&mut k, "window.window === window", "true");
}

#[test]
fn test_window_custom_profile() {
    use iv8_core::shims::browser_profile::{BrowserProfile, DEFAULT_PROFILE};
    let profile = BrowserProfile {
        window_inner_width: 800.0,
        window_inner_height: 600.0,
        window_outer_width: 800.0,
        window_outer_height: 640.0,
        device_pixel_ratio: 2.0,
        ..DEFAULT_PROFILE
    };
    let mut k = common::make_kernel_with_profile(profile);
    common::assert_js_str(&mut k, "window.innerWidth", "800");
    common::assert_js_str(&mut k, "window.innerHeight", "600");
    common::assert_js_str(&mut k, "window.outerWidth", "800");
    common::assert_js_str(&mut k, "window.outerHeight", "640");
    common::assert_js_str(&mut k, "window.devicePixelRatio", "2");
}

#[test]
fn test_window_device_pixel_ratio_native_string() {
    let mut k = common::make_kernel();
    let ts = k.eval_to_rust_value(
        "Object.getOwnPropertyDescriptor(window,'devicePixelRatio').get.toString()"
    );
    match ts {
        iv8_core::convert::RustValue::String(s) => {
            assert!(
                s.contains("[native code]"),
                "getter toString should contain '[native code]', got '{}'",
                s
            );
        }
        other => panic!("expected string, got {:?}", other),
    }
}

#[test]
fn test_window_default_values_match_profile_defaults() {
    use iv8_core::shims::browser_profile::DEFAULT_PROFILE;
    let mut k = common::make_kernel();
    let iw: f64 = common::to_str(&k.eval_to_rust_value("window.innerWidth")).parse().unwrap();
    let ih: f64 = common::to_str(&k.eval_to_rust_value("window.innerHeight")).parse().unwrap();
    let ow: f64 = common::to_str(&k.eval_to_rust_value("window.outerWidth")).parse().unwrap();
    let oh: f64 = common::to_str(&k.eval_to_rust_value("window.outerHeight")).parse().unwrap();
    let dpr: f64 = common::to_str(&k.eval_to_rust_value("window.devicePixelRatio")).parse().unwrap();
    assert_eq!(iw, DEFAULT_PROFILE.window_inner_width);
    assert_eq!(ih, DEFAULT_PROFILE.window_inner_height);
    assert_eq!(ow, DEFAULT_PROFILE.window_outer_width);
    assert_eq!(oh, DEFAULT_PROFILE.window_outer_height);
    assert_eq!(dpr, DEFAULT_PROFILE.device_pixel_ratio);
}

#[test]
fn test_window_dimensions_not_writable_in_strict() {
    let mut k = common::make_kernel();
    // Native getters without setter should silently ignore writes in non-strict,
    // throw TypeError in strict mode.
    let result = k.eval_to_rust_value(
        "'use strict'; try { window.innerWidth = 999; 'written' } catch(e) { 'protected' }"
    );
    assert_eq!(result, iv8_core::convert::RustValue::String("protected".into()),
        "innerWidth must be non-writable in strict mode");
}

#[test]
fn test_window_dimensions_deletable() {
    let mut k = common::make_kernel();
    // Chrome native accessors are configurable: true — delete succeeds
    // (unlike env_inject DONT_DELETE own data properties)
    let result = k.eval_to_rust_value(
        "'use strict'; delete window.innerWidth"
    );
    assert_eq!(result, iv8_core::convert::RustValue::Bool(true),
        "innerWidth accessor must be configurable (deletable) in Chrome-compatible mode");
}

#[test]
fn test_window_dimensions_descriptor_shape_matches_chrome() {
    let mut k = common::make_kernel();
    // Chrome: {get: f, set: undefined, enumerable: true, configurable: true}
    let desc = k.eval_to_rust_value(
        "JSON.stringify(Object.getOwnPropertyDescriptor(window, 'innerWidth'), ['enumerable','configurable'])"
    );
    assert_eq!(desc, iv8_core::convert::RustValue::String("{\"enumerable\":true,\"configurable\":true}".into()),
        "innerWidth accessor must match Chrome descriptor shape (enumerable:true, configurable:true)");
}
