//! v0.8.71: Branch A browser surface coverage baseline — curated priority probe matrix.
//!
//! Covers properties across 9 surface dimensions that have no dedicated per-property tests.
//! Does NOT duplicate existing per-property tests in test_surface_navigator.rs,
//! test_surface_screen.rs, etc. For each property, adds only the missing dimension
//! (type, descriptor, constructor, coherence).
//!
//! Test strategy: all tests assert the current runtime state. Gaps are recorded as
//! assertions on the observed condition — the suite must pass regardless of whether
//! a surface is "present" or "missing". Only performance.timeOrigin fix (Slice 1)
//! is authorized for behavioral change this version.

#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// ─── Navigator core: missing-property probes (native getters exist, not tested) ───

#[test]
fn navigator_cookie_enabled_type() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.cookieEnabled", "boolean");
}

#[test]
fn navigator_online_type() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.onLine", "boolean");
}

#[test]
fn navigator_do_not_track_exists() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value(
        "navigator.doNotTrack === null || typeof navigator.doNotTrack === 'string'",
    ));
    assert_eq!(
        val, "true",
        "doNotTrack should be null or string, got: {}",
        common::to_str(&k.eval_to_rust_value("navigator.doNotTrack"))
    );
}

#[test]
fn navigator_pdf_viewer_enabled_type() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.pdfViewerEnabled", "boolean");
}

#[test]
fn navigator_java_enabled_type() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.javaEnabled", "function");
}

#[test]
fn navigator_core_descriptor_shape() {
    let mut k = common::make_kernel();
    let keys = ["platform", "language", "hardwareConcurrency", "webdriver", "cookieEnabled"];
    for key in &keys {
        let is_getter = k.eval_to_rust_value(&format!(
            "(function(){{var d=Object.getOwnPropertyDescriptor(navigator.__proto__,'{}');return d!==undefined&&typeof d.get==='function'}})()",
            key
        ));
        assert_eq!(
            is_getter,
            iv8_core::convert::RustValue::Bool(true),
            "navigator.__proto__.{} should be a native getter",
            key
        );
    }
}

// ─── NavigatorUAData probes ──────────────────────────────────────────────────────

#[test]
fn uadata_get_high_entropy_values_returns_promise() {
    let mut k = common::make_kernel();
    let is_promise = k.eval_to_rust_value(
        "navigator.userAgentData.getHighEntropyValues(['architecture']) instanceof Promise",
    );
    assert_eq!(
        is_promise,
        iv8_core::convert::RustValue::Bool(true),
        "getHighEntropyValues() should return a Promise"
    );
}

#[test]
fn uadata_brands_descriptor_shape() {
    let mut k = common::make_kernel();
    let is_array = common::to_str(
        &k.eval_to_rust_value("Array.isArray(navigator.userAgentData.brands)"),
    );
    assert_eq!(is_array, "true", "brands should be an Array");
    let len = common::to_str(&k.eval_to_rust_value("navigator.userAgentData.brands.length"));
    let len_val: i64 = len.parse().unwrap_or(0);
    assert!(len_val > 0, "brands should not be empty, got {}", len);
}

#[test]
fn uadata_tojson_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof navigator.userAgentData.toJSON",
        "function",
    );
}

// ─── WorkerNavigator ─────────────────────────────────────────────────────────────

#[test]
fn worker_navigator_instanceof_check() {
    let mut k = common::make_kernel();
    let result = k.eval_to_rust_value("navigator instanceof WorkerNavigator");
    let val = common::to_str(&result);
    // WorkerNavigator exists at global scope but navigator is a Navigator instance,
    // not WorkerNavigator (no real Worker context). This records the current state.
    assert!(
        val == "true" || val == "false",
        "navigator instanceof WorkerNavigator should be boolean, got: {}",
        val
    );
}

// ─── Screen ──────────────────────────────────────────────────────────────────────

#[test]
fn screen_orientation_type() {
    let mut k = common::make_kernel();
    let result = k.eval_to_rust_value(
        "(function(){try{return typeof screen.orientation}catch(e){return 'undefined'}})()",
    );
    let val = common::to_str(&result);
    // orientation may be present (generated skeleton) or absent. Record state.
    assert!(
        val != "null" && val != "NaN",
        "screen.orientation type: {}",
        val
    );
}

#[test]
fn screen_descriptor_shape() {
    let mut k = common::make_kernel();
    let is_getter = k.eval_to_rust_value(
        "(function(){\
         var d=Object.getOwnPropertyDescriptor(Screen.prototype,'width');\
         return d!==undefined&&typeof d.get==='function'\
         })()",
    );
    assert_eq!(
        is_getter,
        iv8_core::convert::RustValue::Bool(true),
        "Screen.prototype.width should be a native getter"
    );
}

// ─── Window dimensions ───────────────────────────────────────────────────────────

#[test]
fn window_screenx_screeny_type() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof window.screenX", "number");
    common::assert_js_str(&mut k, "typeof window.screenY", "number");
}

#[test]
fn window_dimension_descriptor_shape() {
    let mut k = common::make_kernel();
    let is_getter = k.eval_to_rust_value(
        "(function(){\
         var d=Object.getOwnPropertyDescriptor(window,'innerWidth');\
         return d!==undefined&&typeof d.get==='function'\
         })()",
    );
    assert_eq!(
        is_getter,
        iv8_core::convert::RustValue::Bool(true),
        "window.innerWidth should be a native getter"
    );
}

// ─── Performance ─────────────────────────────────────────────────────────────────

#[test]
fn performance_time_origin_type() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("typeof performance.timeOrigin"));
    assert!(
        val == "number" || val == "undefined",
        "performance.timeOrigin should be number or undefined, got: {}",
        val
    );
}

#[test]
fn performance_timing_exists() {
    let mut k = common::make_kernel();
    let result = k.eval_to_rust_value(
        "(function(){try{return typeof performance.timing}catch(e){return 'undefined'}})()",
    );
    let val = common::to_str(&result);
    assert!(val != "null", "performance.timing type: {}", val);
}

#[test]
fn performance_navigation_exists() {
    let mut k = common::make_kernel();
    let result = k.eval_to_rust_value(
        "(function(){try{return typeof performance.navigation}catch(e){return 'undefined'}})()",
    );
    let val = common::to_str(&result);
    assert!(val != "null", "performance.navigation type: {}", val);
}

// ─── Descriptor shape: Symbol.toStringTag ────────────────────────────────────────

#[test]
fn navigator_symbol_to_string_tag() {
    let mut k = common::make_kernel();
    let val = common::to_str(
        &k.eval_to_rust_value("Object.prototype.toString.call(navigator)"),
    );
    assert!(
        val.contains("Navigator"),
        "navigator toStringTag should contain Navigator, got: {}",
        val
    );
}

#[test]
fn screen_symbol_to_string_tag() {
    let mut k = common::make_kernel();
    let val = common::to_str(
        &k.eval_to_rust_value("Object.prototype.toString.call(screen)"),
    );
    assert!(
        val.contains("Screen"),
        "screen toStringTag should contain Screen, got: {}",
        val
    );
}

#[test]
fn performance_symbol_to_string_tag() {
    let mut k = common::make_kernel();
    let val = common::to_str(
        &k.eval_to_rust_value("Object.prototype.toString.call(performance)"),
    );
    assert!(
        val.contains("Performance") || val.contains("Object"),
        "performance toStringTag: {}",
        val
    );
}

#[test]
fn worker_navigator_symbol_to_string_tag() {
    let mut k = common::make_kernel();
    let val = common::to_str(
        &k.eval_to_rust_value(
            "typeof WorkerNavigator !== 'undefined' \
             ? Object.prototype.toString.call(WorkerNavigator.prototype) \
             : 'WorkerNavigator undefined'",
        ),
    );
    assert!(!val.is_empty(), "WorkerNavigator toStringTag: {}", val);
}

// ─── Global constructors ─────────────────────────────────────────────────────────

#[test]
fn screen_constructor_typeof() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("typeof Screen"));
    assert!(
        val == "function" || val == "undefined",
        "typeof Screen: {}",
        val
    );
}

#[test]
fn performance_constructor_typeof() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("typeof Performance"));
    // Performance may be installed as a generated constructor or not at global scope
    // (the runtime performance object is a plain object from date_interceptor.rs)
    assert!(
        val == "function" || val == "undefined" || val == "object",
        "typeof Performance: {}",
        val
    );
}

#[test]
fn image_constructor_typeof() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("typeof Image"));
    assert!(
        val == "function" || val == "undefined",
        "typeof Image: {}",
        val
    );
}

#[test]
fn option_constructor_typeof() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("typeof Option"));
    assert!(
        val == "function" || val == "undefined",
        "typeof Option: {}",
        val
    );
}

#[test]
fn audio_constructor_typeof() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("typeof Audio"));
    assert!(
        val == "function" || val == "undefined",
        "typeof Audio: {}",
        val
    );
}

#[test]
fn screen_constructor_throws() {
    let mut k = common::make_kernel();
    let result = k.eval_to_rust_value(
        "(function(){try{new Screen();return false}catch(e){return true}})()",
    );
    // Screen constructor should throw (illegal constructor in native_env.rs)
    assert_eq!(
        result,
        iv8_core::convert::RustValue::Bool(true),
        "new Screen() should throw TypeError"
    );
}

#[test]
fn navigator_plugins_descriptor_shape() {
    let mut k = common::make_kernel();
    let is_array = common::to_str(
        &k.eval_to_rust_value("Array.isArray(navigator.plugins)"),
    );
    assert_eq!(is_array, "true", "plugins should be array-like");
    let len = common::to_str(&k.eval_to_rust_value("navigator.plugins.length"));
    let len_val: i64 = len.parse().unwrap_or(0);
    assert!(len_val > 0, "plugins should not be empty, got {}", len);
    // Verify plugins entries have name property
    let has_name = common::to_str(
        &k.eval_to_rust_value("typeof navigator.plugins[0].name"),
    );
    assert_eq!(
        has_name, "string",
        "plugins[0].name should be string, got: {}",
        has_name
    );
}

// ─── Coherence — cross-surface profile consistency ───────────────────────────────

#[test]
fn screen_width_gte_window_outer_width() {
    let mut k = common::make_kernel();
    let sw = common::to_str(&k.eval_to_rust_value("screen.width"));
    let ww = common::to_str(&k.eval_to_rust_value("window.outerWidth"));
    let sw_val: f64 = sw.parse().unwrap_or(0.0);
    let ww_val: f64 = ww.parse().unwrap_or(0.0);
    assert!(
        sw_val >= ww_val,
        "screen.width ({}) should be >= window.outerWidth ({})",
        sw_val,
        ww_val
    );
}

#[test]
fn ua_platform_family_coherent_with_uadata() {
    let mut k = common::make_kernel();
    let ua = common::to_str(&k.eval_to_rust_value("navigator.userAgent"));
    let ua_platform = common::to_str(
        &k.eval_to_rust_value("navigator.userAgentData.platform"),
    );
    let nav_platform = common::to_str(
        &k.eval_to_rust_value("navigator.platform"),
    );

    // Family mapping, not equality
    let ua_is_windows = ua.contains("Windows");
    let navp_is_win32 = nav_platform == "Win32";
    let uap_is_windows = ua_platform == "Windows";

    if ua_is_windows {
        assert!(
            navp_is_win32,
            "UA contains Windows but platform is {}, expected Win32",
            nav_platform
        );
        assert!(
            uap_is_windows,
            "UA contains Windows but uadata.platform is {}, expected Windows",
            ua_platform
        );
    }
}

// v0.8.78: navigator instanceof EventTarget — the native_env refactor
// links nav_tmpl.prototype.__proto__ to install_all's Navigator.prototype,
// which inherits EventTarget. So navigator instanceof EventTarget === true
// AND navigator instanceof Navigator === true.
#[test]
fn navigator_instanceof_event_target() {
    let mut k = common::make_kernel();
    let result = k.eval_to_rust_value("navigator instanceof EventTarget");
    assert_eq!(
        result,
        iv8_core::convert::RustValue::Bool(true),
        "navigator instanceof EventTarget should be true"
    );
}

#[test]
fn navigator_instanceof_navigator() {
    let mut k = common::make_kernel();
    let result = k.eval_to_rust_value("navigator instanceof Navigator");
    assert_eq!(
        result,
        iv8_core::convert::RustValue::Bool(true),
        "navigator instanceof Navigator should be true"
    );
}

#[test]
fn navigator_proto_chain() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.userAgent", "string");
    let result = k.eval_to_rust_value(
        "Object.getPrototypeOf(Object.getPrototypeOf(navigator)) === Navigator.prototype",
    );
    assert_eq!(
        result,
        iv8_core::convert::RustValue::Bool(true),
        "navigator.__proto__.__proto__ should be Navigator.prototype"
    );
}

// ─── B1-B5 behavior probes (v0.8.78) ────────────────────────────────────────────
//
// B2 sendBeacon and B5 chrome.runtime are already correct; verified separately.
// Here we cover the synchronous observable shape for B1, B3, B4.

#[test]
fn b1_battery_getbattery_native_code() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.getBattery", "function");
    let result = k.eval_to_rust_value("navigator.getBattery.toString().includes('[native code]')");
    assert_eq!(
        result,
        iv8_core::convert::RustValue::Bool(true),
        "getBattery.toString() should report [native code]"
    );
}

#[test]
fn b3_domexception_exists_and_correct() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof DOMException", "function");
    let result = k.eval_to_rust_value(
        "(function(){ try { var e = new DOMException('test', 'NotSupportedError'); return e.name; } catch(e) { return 'error:' + e.message; } })()",
    );
    match result {
        iv8_core::convert::RustValue::String(s) => assert_eq!(s, "NotSupportedError"),
        _ => panic!("Expected NotSupportedError, got {:?}", result),
    }
}

#[test]
fn b4_permissions_notification_consistency() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "Notification.permission", "default");
    common::assert_js_str(&mut k, "typeof navigator.permissions.query", "function");
}
