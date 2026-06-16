//! Native environment object installation.
//!
//! Creates `navigator`, `screen`, and other browser environment objects using
//! V8 FunctionTemplate accessors, so that:
//!
//!   Object.getOwnPropertyDescriptor(navigator, 'userAgent')
//!   // → { get: function userAgent() { [native code] }, set: undefined, ... }
//!
//! This is the key difference from static value injection (env_inject.rs):
//! - env_inject.rs: define_own_property → value descriptor (detectable)
//! - native_env.rs: set_accessor_property → getter descriptor (matches real browser)
//!
//! Strategy:
//! - navigator: FunctionTemplate with Navigator.prototype (v0.8.17 Slice 1)
//! - screen: FunctionTemplate with Screen.prototype (v0.8.17 Slice 2)
//! - High-value properties (most commonly fingerprint-checked) → native getter
//! - All values still come from RuntimeState.environment (fully configurable)
//! - env_inject.rs still runs first for the full 393-entry set; we then
//!   OVERRIDE the key objects with native-getter versions.

use crate::state::RuntimeState;

/// Install native-getter versions of navigator and screen on the global.
/// Must be called AFTER env_inject (so the base objects exist) but BEFORE
/// any JS that reads these properties.
pub fn install_native_env(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    install_native_navigator(scope, global);
    install_native_screen(scope, global);
}

// ─── navigator ────────────────────────────────────────────────────────────────

/// Illegal constructor — Navigator and Screen are not constructable from JS.
/// Throws TypeError: Illegal constructor, matching real browser behavior.
unsafe extern "C" fn illegal_constructor(info: *const v8::FunctionCallbackInfo) {
    let info_ref = unsafe { &*info };
    v8::callback_scope!(unsafe scope, info_ref);
    let msg = crate::v8_utils::v8_string(scope, "Illegal constructor");
    let exc = v8::Exception::type_error(scope, msg);
    scope.throw_exception(exc);
}

/// Build a native navigator object with accessor properties.
fn install_native_navigator(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    // Create a FunctionTemplate for Navigator (replaces flat ObjectTemplate)
    let nav_tmpl =
        v8::FunctionTemplate::builder_raw(illegal_constructor).build(scope);
    nav_tmpl.set_class_name(crate::v8_utils::v8_string(scope, "Navigator"));

    // Install native getters on Navigator.prototype (not on the instance template).
    // Each getter reads from RuntimeState.environment at call time.
    macro_rules! nav_getter {
        ($name:literal, $cb:ident) => {
            let getter = v8::FunctionTemplate::builder_raw($cb).build(scope);
            let name = crate::v8_utils::v8_string(scope, $name);
            // Set the getter's name so toString() returns "function <name>() { [native code] }"
            getter.set_class_name(name);
            nav_tmpl.prototype_template(scope).set_accessor_property(
                name.into(),
                Some(getter),
                None,
                v8::PropertyAttribute::DONT_DELETE,
            );
        };
    }

    nav_getter!("userAgent", nav_user_agent);
    nav_getter!("appVersion", nav_app_version);
    nav_getter!("platform", nav_platform);
    nav_getter!("vendor", nav_vendor);
    nav_getter!("vendorSub", nav_vendor_sub);
    nav_getter!("product", nav_product);
    nav_getter!("productSub", nav_product_sub);
    nav_getter!("language", nav_language);
    nav_getter!("languages", nav_languages);
    nav_getter!("hardwareConcurrency", nav_hardware_concurrency);
    nav_getter!("deviceMemory", nav_device_memory);
    nav_getter!("maxTouchPoints", nav_max_touch_points);
    nav_getter!("cookieEnabled", nav_cookie_enabled);
    nav_getter!("onLine", nav_online);
    nav_getter!("doNotTrack", nav_do_not_track);
    nav_getter!("webdriver", nav_webdriver);
    nav_getter!("appName", nav_app_name);
    nav_getter!("appCodeName", nav_app_code_name);
    nav_getter!("permissions", nav_permissions);
    nav_getter!("mediaDevices", nav_media_devices);
    nav_getter!("serviceWorker", nav_service_worker);
    nav_getter!("pdfViewerEnabled", nav_pdf_viewer_enabled);

    // javaEnabled: function that returns false (no Java plugin in V8)
    let java_fn = v8::FunctionTemplate::builder_raw(nav_java_enabled).build(scope);
    let java_name = crate::v8_utils::v8_string(scope, "javaEnabled");
    java_fn.set_class_name(java_name);
    nav_tmpl
        .prototype_template(scope)
        .set(java_name.into(), java_fn.into());

    // Instantiate via instance_template (bypasses constructor — we don't want
    // illegal_constructor to block Rust-side instance creation).
    // When JS does `new Navigator()`, the constructor throws TypeError.
    let inst_tmpl = nav_tmpl.instance_template(scope);
    if let Some(nav_obj) = inst_tmpl.new_instance(scope) {
        // Install userAgentData sub-object on navigator instance
        crate::shims::user_agent_data::install_user_agent_data(scope, nav_obj);

        let key = crate::v8_utils::v8_string(scope, "navigator");
        global.define_own_property(
            scope,
            key.into(),
            nav_obj.into(),
            v8::PropertyAttribute::DONT_DELETE,
        );
    }

    // Install Navigator constructor on global (non-enumerable, like DOM)
    if let Some(func) = nav_tmpl.get_function(scope) {
        let ctor_key = crate::v8_utils::v8_string(scope, "Navigator");
        global.define_own_property(
            scope,
            ctor_key.into(),
            func.into(),
            v8::PropertyAttribute::DONT_ENUM,
        );
    }
}

// ─── navigator getter callbacks ───────────────────────────────────────────────

macro_rules! env_str_getter {
    ($name:ident, $path:literal, $default:literal) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let info_ref = unsafe { &*info };
                v8::callback_scope!(unsafe scope, info_ref);
                let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
                let isolate: &v8::Isolate = &*scope;
                let state = RuntimeState::get(isolate);
                let val = state.environment.get_str($path).unwrap_or($default);
                if let Some(s) = v8::String::new(scope, val) {
                    rv.set(s.into());
                }
            }));
        }
    };
}

macro_rules! env_f64_getter {
    ($name:ident, $path:literal, $default:expr) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let info_ref = unsafe { &*info };
                v8::callback_scope!(unsafe scope, info_ref);
                let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
                let isolate: &v8::Isolate = &*scope;
                let state = RuntimeState::get(isolate);
                let val = state.environment.get_f64($path).unwrap_or($default);
                rv.set(v8::Number::new(scope, val).into());
            }));
        }
    };
}

macro_rules! env_bool_getter {
    ($name:ident, $path:literal, $default:expr) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let info_ref = unsafe { &*info };
                v8::callback_scope!(unsafe scope, info_ref);
                let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
                let isolate: &v8::Isolate = &*scope;
                let state = RuntimeState::get(isolate);
                let val = state.environment.get_bool($path).unwrap_or($default);
                rv.set(v8::Boolean::new(scope, val).into());
            }));
        }
    };
}

env_str_getter!(nav_user_agent, "navigator.userAgent", "Mozilla/5.0");
env_str_getter!(nav_app_version, "navigator.appVersion", "5.0");
env_str_getter!(nav_platform, "navigator.platform", "Win32");
env_str_getter!(nav_vendor, "navigator.vendor", "Google Inc.");
env_str_getter!(nav_vendor_sub, "navigator.vendorSub", "");
env_str_getter!(nav_product, "navigator.product", "Gecko");
env_str_getter!(nav_product_sub, "navigator.productSub", "20030107");
env_str_getter!(nav_language, "navigator.language", "en-US");
env_str_getter!(nav_app_name, "navigator.appName", "Netscape");
env_str_getter!(nav_app_code_name, "navigator.appCodeName", "Mozilla");
env_f64_getter!(
    nav_hardware_concurrency,
    "navigator.hardwareConcurrency",
    8.0
);
env_f64_getter!(nav_device_memory, "navigator.deviceMemory", 8.0);
env_f64_getter!(nav_max_touch_points, "navigator.maxTouchPoints", 0.0);
env_bool_getter!(nav_cookie_enabled, "navigator.cookieEnabled", true);
env_bool_getter!(nav_online, "navigator.onLine", true);

// navigator.languages → array from environment
unsafe extern "C" fn nav_languages(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        // Try to get languages array from environment
        let langs: Vec<String> = if let Some(val) = state.environment.get("navigator.languages") {
            if let Some(arr) = val.as_array() {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            } else {
                vec!["en-US".to_string(), "en".to_string()]
            }
        } else {
            // Fall back to single language
            let lang = state
                .environment
                .get_str("navigator.language")
                .unwrap_or("en-US");
            vec![lang.to_string()]
        };

        let arr = v8::Array::new(scope, langs.len() as i32);
        for (i, lang) in langs.iter().enumerate() {
            if let Some(s) = v8::String::new(scope, lang) {
                arr.set_index(scope, i as u32, s.into());
            }
        }
        rv.set(arr.into());
    }));
}

// navigator.permissions stub — returns object with query() method
unsafe extern "C" fn nav_permissions(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        // query(descriptor) → Promise<{state: 'granted'|'denied'|'prompt'}>
        let query_tmpl = v8::FunctionTemplate::builder_raw(permissions_query_cb).build(scope);
        let query_fn = crate::v8_utils::v8_fn(scope, &query_tmpl);
        let query_key = crate::v8_utils::v8_string(scope, "query");
        obj.set(scope, query_key.into(), query_fn.into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn permissions_query_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        // Return Promise.resolve({state: 'prompt'})
        let global = scope.get_current_context().global(scope);
        let promise_key = crate::v8_utils::v8_string(scope, "Promise");
        if let Some(promise_ctor) = global.get(scope, promise_key.into()) {
            if promise_ctor.is_function() {
                let ctor: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(promise_ctor) };
                let resolve_key = crate::v8_utils::v8_string(scope, "resolve");
                if let Some(resolve_fn) = ctor.get(scope, resolve_key.into()) {
                    if resolve_fn.is_function() {
                        let resolve: v8::Local<v8::Function> =
                            unsafe { v8::Local::cast_unchecked(resolve_fn) };
                        let result_obj = v8::Object::new(scope);
                        let state_key = crate::v8_utils::v8_string(scope, "state");
                        let state_val = crate::v8_utils::v8_string(scope, "prompt");
                        result_obj.set(scope, state_key.into(), state_val.into());
                        let _undefined = v8::undefined(scope);
                        if let Some(promise) =
                            resolve.call(scope, ctor.into(), &[result_obj.into()])
                        {
                            rv.set(promise);
                            return;
                        }
                    }
                }
            }
        }
        rv.set(v8::undefined(scope).into());
    }));
}

// navigator.mediaDevices stub
unsafe extern "C" fn nav_media_devices(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        // enumerateDevices() → Promise<[]>
        let enum_tmpl = v8::FunctionTemplate::builder_raw(media_devices_enumerate_cb).build(scope);
        let enum_fn = crate::v8_utils::v8_fn(scope, &enum_tmpl);
        let enum_key = crate::v8_utils::v8_string(scope, "enumerateDevices");
        obj.set(scope, enum_key.into(), enum_fn.into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn media_devices_enumerate_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let global = scope.get_current_context().global(scope);
        let promise_key = crate::v8_utils::v8_string(scope, "Promise");
        if let Some(promise_ctor) = global.get(scope, promise_key.into()) {
            if promise_ctor.is_function() {
                let ctor: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(promise_ctor) };
                let resolve_key = crate::v8_utils::v8_string(scope, "resolve");
                if let Some(resolve_fn) = ctor.get(scope, resolve_key.into()) {
                    if resolve_fn.is_function() {
                        let resolve: v8::Local<v8::Function> =
                            unsafe { v8::Local::cast_unchecked(resolve_fn) };
                        let empty_arr = v8::Array::new(scope, 0);
                        let _undefined = v8::undefined(scope);
                        if let Some(promise) = resolve.call(scope, ctor.into(), &[empty_arr.into()])
                        {
                            rv.set(promise);
                            return;
                        }
                    }
                }
            }
        }
        rv.set(v8::undefined(scope).into());
    }));
}

// navigator.serviceWorker stub
unsafe extern "C" fn nav_service_worker(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let state_key = crate::v8_utils::v8_string(scope, "controller");
        obj.set(scope, state_key.into(), v8::null(scope).into());
        rv.set(obj.into());
    }));
}

// navigator.webdriver → undefined in real browsers (anti-detection)
// In strict_compat mode: false (matching iv8 0.1.2 behavior)
unsafe extern "C" fn nav_webdriver(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        if state.strict_compat {
            // iv8 0.1.2 returns false (bug: real browsers return undefined)
            rv.set(v8::Boolean::new(scope, false).into());
        } else {
            // Correct browser behavior: undefined when not automated
            rv.set(v8::undefined(scope).into());
        }
    }));
}

// navigator.doNotTrack → null (standard default)
unsafe extern "C" fn nav_do_not_track(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::null(scope).into());
    }));
}

// ─── screen ───────────────────────────────────────────────────────────────────

fn install_native_screen(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    // Create a FunctionTemplate for Screen (replaces flat ObjectTemplate)
    let screen_tmpl =
        v8::FunctionTemplate::builder_raw(illegal_constructor).build(scope);
    screen_tmpl.set_class_name(crate::v8_utils::v8_string(scope, "Screen"));

    macro_rules! screen_getter {
        ($name:literal, $cb:ident) => {
            let getter = v8::FunctionTemplate::builder_raw($cb).build(scope);
            let name = crate::v8_utils::v8_string(scope, $name);
            getter.set_class_name(name);
            screen_tmpl.prototype_template(scope).set_accessor_property(
                name.into(),
                Some(getter),
                None,
                v8::PropertyAttribute::DONT_DELETE,
            );
        };
    }

    screen_getter!("width", screen_width);
    screen_getter!("height", screen_height);
    screen_getter!("availWidth", screen_avail_width);
    screen_getter!("availHeight", screen_avail_height);
    screen_getter!("colorDepth", screen_color_depth);
    screen_getter!("pixelDepth", screen_pixel_depth);
    screen_getter!("availLeft", screen_avail_left);
    screen_getter!("availTop", screen_avail_top);

    // Instantiate via instance_template (bypasses constructor).
    // When JS does `new Screen()`, the constructor throws TypeError.
    let inst_tmpl = screen_tmpl.instance_template(scope);
    if let Some(screen_obj) = inst_tmpl.new_instance(scope) {
        let key = crate::v8_utils::v8_string(scope, "screen");
        global.define_own_property(
            scope,
            key.into(),
            screen_obj.into(),
            v8::PropertyAttribute::DONT_DELETE,
        );
    }

    // Install Screen constructor on global (non-enumerable, like DOM)
    if let Some(func) = screen_tmpl.get_function(scope) {
        let ctor_key = crate::v8_utils::v8_string(scope, "Screen");
        global.define_own_property(
            scope,
            ctor_key.into(),
            func.into(),
            v8::PropertyAttribute::DONT_ENUM,
        );
    }
}

env_bool_getter!(nav_pdf_viewer_enabled, "navigator.pdfViewerEnabled", true);

// javaEnabled() → always returns false (no Java plugin in V8 context)
unsafe extern "C" fn nav_java_enabled(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::Boolean::new(scope, false).into());
    }));
}

env_f64_getter!(screen_width, "screen.width", 1920.0);
env_f64_getter!(screen_height, "screen.height", 1080.0);
env_f64_getter!(screen_avail_width, "screen.availWidth", 1920.0);
env_f64_getter!(screen_avail_height, "screen.availHeight", 1040.0);
env_f64_getter!(screen_color_depth, "screen.colorDepth", 24.0);
env_f64_getter!(screen_pixel_depth, "screen.pixelDepth", 24.0);
env_f64_getter!(screen_avail_left, "screen.availLeft", 0.0);
env_f64_getter!(screen_avail_top, "screen.availTop", 0.0);
