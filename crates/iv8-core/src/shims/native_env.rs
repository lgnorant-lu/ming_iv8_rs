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

use crate::shims::browser_profile::DEFAULT_PROFILE;
use crate::state::RuntimeState;
use iv8_surface::generated::css_om::create_screen_template;
use iv8_surface::generated::web_apis::create_navigator_template;

/// Install native-getter versions of navigator and screen on the global.
/// Must be called AFTER env_inject (so the base objects exist) but BEFORE
/// any JS that reads these properties.
pub fn install_native_env(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    install_native_navigator(scope, global);
    install_native_screen(scope, global);
    install_worker_navigator(scope, global);
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

/// Build a DOMException object with the given message and name.
/// If DOMException constructor is not available (shim not yet installed),
/// falls back to a plain Error with name property set.
fn build_dom_exception<'s>(
    scope: &v8::PinScope<'s, '_>,
    global: &v8::Local<'s, v8::Object>,
    message: &str,
    name: &str,
) -> v8::Local<'s, v8::Value> {
    // Try DOMException constructor (installed by DOCUMENT_PROPS_JS shim)
    let dom_key = crate::v8_utils::v8_string(scope, "DOMException");
    if let Some(dom_ctor_val) = global.get(scope, dom_key.into()) {
        if dom_ctor_val.is_function() {
            let ctor: v8::Local<v8::Function> =
                unsafe { v8::Local::cast_unchecked(dom_ctor_val) };
            let msg_arg = crate::v8_utils::v8_string(scope, message);
            let name_arg = crate::v8_utils::v8_string(scope, name);
            let undefined = v8::undefined(scope);
            if let Some(result) = ctor.call(scope, undefined.into(), &[msg_arg.into(), name_arg.into()]) {
                return result;
            }
        }
    }
    // Fallback: Error with name property
    let err = v8::Exception::type_error(scope, crate::v8_utils::v8_string(scope, message));
    let name_key = crate::v8_utils::v8_string(scope, "name");
    let name_val = crate::v8_utils::v8_string(scope, name);
    let err_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(err) };
    err_obj.set(scope, name_key.into(), name_val.into());
    err
}

/// Build a native navigator object with accessor properties.
/// v0.8.60: creates a native Navigator template that inherits from the
/// generated create_navigator_template (46 skeleton properties). Native
/// getters are added to the native template's prototype, shadowing
/// generated skeleton properties via the prototype chain.
/// This unifies BrowserSurface-generated skeleton properties
/// (bluetooth, hid, usb, gpu, etc.) with native profile-backed
/// getters in a single Navigator object.
fn install_native_navigator(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    // After install_all, the global "Navigator" constructor already exists
    // with EventTarget inheritance. We create a nav_tmpl that inherits from
    // the codegen Navigator template to get both native getters AND the
    // EventTarget prototype chain.
    let gen_tmpl = create_navigator_template(scope, None);
    let nav_tmpl = v8::FunctionTemplate::builder_raw(illegal_constructor).build(scope);
    nav_tmpl.set_class_name(crate::v8_utils::v8_string(scope, "Navigator"));
    nav_tmpl.inherit(gen_tmpl);

    // Install native getters on Navigator.prototype template.
    // These shadow generated skeleton getters for overlapping names
    // via prototype chain (native proto → generated proto).
    let proto = nav_tmpl.prototype_template(scope);
    macro_rules! nav_getter {
        ($name:literal, $cb:ident) => {
            let getter = v8::FunctionTemplate::builder_raw($cb).build(scope);
            let name = crate::v8_utils::v8_string(scope, $name);
            getter.set_class_name(name);
            getter.remove_prototype();
            proto.set_accessor_property(
                name.into(),
                Some(getter),
                None,
                v8::PropertyAttribute::DONT_DELETE | v8::PropertyAttribute::DONT_ENUM,
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

    // plugins + mimeTypes: native getters so length is configurable
    nav_getter!("plugins", nav_plugins);
    nav_getter!("mimeTypes", nav_mime_types);

    // connection — NetworkInformation-like accessor getter
    nav_getter!("connection", nav_connection);

    // getBattery: function returning Promise<BatteryManager>
    let battery_fn = v8::FunctionTemplate::builder_raw(nav_get_battery).build(scope);
    let battery_name = crate::v8_utils::v8_string(scope, "getBattery");
    battery_fn.set_class_name(battery_name);
    battery_fn.remove_prototype();
    proto.set(battery_name.into(), battery_fn.into());

    // sendBeacon: function returning true
    let beacon_fn = v8::FunctionTemplate::builder_raw(nav_send_beacon).build(scope);
    let beacon_name = crate::v8_utils::v8_string(scope, "sendBeacon");
    beacon_fn.set_class_name(beacon_name);
    beacon_fn.remove_prototype();
    proto.set(beacon_name.into(), beacon_fn.into());

    // geolocation — accessor getter returning object with stub methods
    nav_getter!("geolocation", nav_geolocation);

    // clipboard — accessor getter returning object with stub methods
    nav_getter!("clipboard", nav_clipboard);

    // credentials — accessor getter returning object with stub methods
    nav_getter!("credentials", nav_credentials);

    // javaEnabled: function that returns false (no Java plugin in V8)
    let java_fn = v8::FunctionTemplate::builder_raw(nav_java_enabled).build(scope);
    let java_name = crate::v8_utils::v8_string(scope, "javaEnabled");
    java_fn.set_class_name(java_name);
    java_fn.remove_prototype();
    proto.set(java_name.into(), java_fn.into());

    // getGamepads: function returning empty array (v0.8.61)
    let gamepads_fn = v8::FunctionTemplate::builder_raw(nav_get_gamepads).build(scope);
    let gamepads_name = crate::v8_utils::v8_string(scope, "getGamepads");
    gamepads_fn.set_class_name(gamepads_name);
    gamepads_fn.remove_prototype();
    proto.set(gamepads_name.into(), gamepads_fn.into());

    // requestMediaKeySystemAccess: function returning rejected Promise (v0.8.61)
    let eme_fn =
        v8::FunctionTemplate::builder_raw(nav_request_media_key_system_access).build(scope);
    let eme_name = crate::v8_utils::v8_string(scope, "requestMediaKeySystemAccess");
    eme_fn.set_class_name(eme_name);
    eme_fn.remove_prototype();
    proto.set(eme_name.into(), eme_fn.into());

    // requestMIDIAccess: function returning rejected Promise (v0.8.61)
    let midi_fn = v8::FunctionTemplate::builder_raw(nav_request_midi_access).build(scope);
    let midi_name = crate::v8_utils::v8_string(scope, "requestMIDIAccess");
    midi_fn.set_class_name(midi_name);
    midi_fn.remove_prototype();
    proto.set(midi_name.into(), midi_fn.into());

    // Instantiate via instance_template (bypasses constructor — we don't want
    // illegal_constructor to block Rust-side instance creation).
    // When JS does `new Navigator()`, the constructor throws TypeError.
    let inst_tmpl = nav_tmpl.instance_template(scope);
    if let Some(nav_obj) = inst_tmpl.new_instance(scope) {
        // Install userAgentData sub-object on navigator instance
        crate::shims::user_agent_data::install_user_agent_data(scope, nav_obj);

        // v0.8.62: conditionally hide platform-dependent properties
        conditionally_hide_properties(scope, nav_obj);

        let key = crate::v8_utils::v8_string(scope, "navigator");
        global.define_own_property(
            scope,
            key.into(),
            nav_obj.into(),
            v8::PropertyAttribute::DONT_DELETE,
        );
    }

    // Do NOT overwrite the global Navigator constructor (from install_all).
    // install_all's Navigator has EventTarget inheritance. Our nav_tmpl is
    // only used to create the navigator instance with native getters.
    // The global Navigator constructor stays as install_all's version,
    // so navigator instanceof EventTarget works through the prototype chain.
    // Note: navigator.__proto__ will be nav_tmpl.prototype (which has native
    // getters), and nav_tmpl.prototype.__proto__ = gen_tmpl.prototype
    // (codegen Navigator.prototype with EventTarget inheritance).
    // navigator instanceof Navigator checks if Navigator.prototype is in
    // navigator's prototype chain — it is, through nav_tmpl → gen_tmpl.

    // Re-register Navigator constructor (overwrites install_all's version).
    // This is necessary because install_native_navigator's gen_tmpl is a
    // DIFFERENT FunctionTemplate than install_all's gen_tmpl. Without
    // overwriting, navigator instanceof Navigator would fail because
    // navigator.__proto__.__proto__ (our gen_tmpl.prototype) !==
    // Navigator.prototype (install_all's gen_tmpl.prototype).
    // Trade-off: navigator instanceof EventTarget = False (our gen_tmpl
    // was created with parent=None, not EventTarget).
    // See TODO-infrastructure.md for full analysis.
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

// ─── WorkerNavigator (v0.8.70) ────────────────────────────────────────────

/// Install a native WorkerNavigator constructor that inherits from the
/// generated create_worker_navigator_template and uses illegal_constructor
/// so that `new WorkerNavigator()` throws TypeError.
fn install_worker_navigator(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
) {
    use iv8_surface::generated::workers::create_worker_navigator_template;

    let gen_tmpl = create_worker_navigator_template(scope, None);
    let wn_tmpl = v8::FunctionTemplate::builder_raw(illegal_constructor)
        .build(scope);
    wn_tmpl.set_class_name(
        crate::v8_utils::v8_string(scope, "WorkerNavigator"),
    );
    wn_tmpl.inherit(gen_tmpl);

    if let Some(func) = wn_tmpl.get_function(scope) {
        let key =
            crate::v8_utils::v8_string(scope, "WorkerNavigator");
        global.define_own_property(
            scope,
            key.into(),
            func.into(),
            v8::PropertyAttribute::DONT_ENUM,
        );
    }
}

// v0.8.62: conditionally hide platform-dependent Navigator properties
// based on BrowserProfile flags (mobile_profile).
// NOTE: webkitGetUserMedia was in original scope but is a global constructor
// (chrome_extensions.rs), not a Navigator property. Removed from conditional set.
// chrome_version flag retained in BrowserProfile for future use.
fn conditionally_hide_properties(scope: &v8::PinScope<'_, '_>, nav_obj: v8::Local<v8::Object>) {
    let isolate: &v8::Isolate = scope;
    let state = RuntimeState::get(isolate);
    let profile = state.profile.unwrap_or(&DEFAULT_PROFILE);

    // Desktop profile: hide mobile-only properties by masking with undefined.
    // Proto-level accessor properties can't be deleted from the instance,
    // so we set own data properties with undefined value to shadow them.
    if !profile.mobile_profile {
        let undef = v8::undefined(scope);
        for prop in &["share", "canShare", "vibrate"] {
            let key = crate::v8_utils::v8_string(scope, prop);
            nav_obj.define_own_property(
                scope,
                key.into(),
                undef.into(),
                v8::PropertyAttribute::DONT_ENUM,
            );
        }
    }
}

// ─── navigator getter callbacks ───────────────────────────────────────────────

macro_rules! env_str_getter {
    ($name:ident, $path:literal, $field:ident, $default:expr) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let info_ref = unsafe { &*info };
                v8::callback_scope!(unsafe scope, info_ref);
                let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
                let isolate: &v8::Isolate = &*scope;
                let state = RuntimeState::get(isolate);
                let val = match state.profile {
                    Some(p) => p.$field,
                    None => state.environment.get_str($path).unwrap_or($default),
                };
                if let Some(s) = v8::String::new(scope, val) {
                    rv.set(s.into());
                }
            }));
        }
    };
}

macro_rules! env_f64_getter {
    ($name:ident, $path:literal, $field:ident, $default:expr) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let info_ref = unsafe { &*info };
                v8::callback_scope!(unsafe scope, info_ref);
                let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
                let isolate: &v8::Isolate = &*scope;
                let state = RuntimeState::get(isolate);
                let val = match state.profile {
                    Some(p) => p.$field,
                    None => state.environment.get_f64($path).unwrap_or($default),
                };
                rv.set(v8::Number::new(scope, val).into());
            }));
        }
    };
}

macro_rules! env_bool_getter {
    ($name:ident, $path:literal, $field:ident, $default:expr) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let info_ref = unsafe { &*info };
                v8::callback_scope!(unsafe scope, info_ref);
                let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
                let isolate: &v8::Isolate = &*scope;
                let state = RuntimeState::get(isolate);
                let val = match state.profile {
                    Some(p) => p.$field,
                    None => state.environment.get_bool($path).unwrap_or($default),
                };
                rv.set(v8::Boolean::new(scope, val).into());
            }));
        }
    };
}

env_str_getter!(
    nav_user_agent,
    "navigator.userAgent",
    user_agent,
    DEFAULT_PROFILE.user_agent
);
env_str_getter!(
    nav_app_version,
    "navigator.appVersion",
    app_version,
    DEFAULT_PROFILE.app_version
);
env_str_getter!(
    nav_platform,
    "navigator.platform",
    platform,
    DEFAULT_PROFILE.platform
);
env_str_getter!(
    nav_vendor,
    "navigator.vendor",
    vendor,
    DEFAULT_PROFILE.vendor
);
env_str_getter!(
    nav_vendor_sub,
    "navigator.vendorSub",
    vendor_sub,
    DEFAULT_PROFILE.vendor_sub
);
env_str_getter!(
    nav_product,
    "navigator.product",
    product,
    DEFAULT_PROFILE.product
);
env_str_getter!(
    nav_product_sub,
    "navigator.productSub",
    product_sub,
    DEFAULT_PROFILE.product_sub
);
env_str_getter!(
    nav_language,
    "navigator.language",
    language,
    DEFAULT_PROFILE.language
);
env_str_getter!(
    nav_app_name,
    "navigator.appName",
    app_name,
    DEFAULT_PROFILE.app_name
);
env_str_getter!(
    nav_app_code_name,
    "navigator.appCodeName",
    app_code_name,
    DEFAULT_PROFILE.app_code_name
);
env_f64_getter!(
    nav_hardware_concurrency,
    "navigator.hardwareConcurrency",
    hardware_concurrency,
    DEFAULT_PROFILE.hardware_concurrency
);
env_f64_getter!(
    nav_device_memory,
    "navigator.deviceMemory",
    device_memory,
    DEFAULT_PROFILE.device_memory
);
env_f64_getter!(
    nav_max_touch_points,
    "navigator.maxTouchPoints",
    max_touch_points,
    DEFAULT_PROFILE.max_touch_points
);
env_bool_getter!(
    nav_cookie_enabled,
    "navigator.cookieEnabled",
    cookie_enabled,
    DEFAULT_PROFILE.cookie_enabled
);
env_bool_getter!(
    nav_online,
    "navigator.onLine",
    on_line,
    DEFAULT_PROFILE.on_line
);

// navigator.languages → array from environment
unsafe extern "C" fn nav_languages(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        // Try to get languages array from environment
        let langs: Vec<String> = if let Some(p) = &state.profile {
            p.languages.iter().map(|s| s.to_string()).collect()
        } else if let Some(val) = state.environment.get("navigator.languages") {
            if let Some(arr) = val.as_array() {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            } else {
                DEFAULT_PROFILE
                    .languages
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            }
        } else {
            let lang = state
                .environment
                .get_str("navigator.language")
                .unwrap_or(DEFAULT_PROFILE.language);
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
                        // Check descriptor name: 'notifications' → 'default',
                        // everything else → 'prompt'
                        let mut state_str = "prompt";
                        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
                        if args.length() > 0 {
                            let arg0 = args.get(0);
                            if arg0.is_object() {
                                let desc_obj: v8::Local<v8::Object> =
                                    unsafe { v8::Local::cast_unchecked(arg0) };
                                let name_key = crate::v8_utils::v8_string(scope, "name");
                                if let Some(name_val) = desc_obj.get(scope, name_key.into()) {
                                    let name = name_val.to_rust_string_lossy(scope);
                                    if name == "notifications" {
                                        state_str = "default";
                                    }
                                }
                            }
                        }
                        let state_val = crate::v8_utils::v8_string(scope, state_str);
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

// navigator.doNotTrack → null (standard default), profile-injectable
unsafe extern "C" fn nav_do_not_track(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let val = state
            .profile
            .and_then(|p| p.do_not_track)
            .or_else(|| state.environment.get_str("navigator.doNotTrack"));
        if let Some(s) = val {
            if let Some(v) = v8::String::new(scope, s) {
                rv.set(v.into());
                return;
            }
        }
        rv.set(v8::null(scope).into());
    }));
}

// navigator.connection → NetworkInformation-like accessor getter
unsafe extern "C" fn nav_connection(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        obj.set(scope, s("effectiveType").into(), s("4g").into());
        obj.set(
            scope,
            s("downlink").into(),
            v8::Number::new(scope, 10.0).into(),
        );
        obj.set(scope, s("rtt").into(), v8::Number::new(scope, 50.0).into());
        obj.set(
            scope,
            s("saveData").into(),
            v8::Boolean::new(scope, false).into(),
        );
        obj.set(scope, s("type").into(), s("wifi").into());
        let ts = v8::Symbol::get_to_string_tag(scope);
        obj.set(scope, ts.into(), s("NetworkInformation").into());
        rv.set(obj.into());
    }));
}

// navigator.getBattery() → Promise<{charging, chargingTime, dischargingTime, level}>
unsafe extern "C" fn nav_get_battery(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let result = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        result.set(
            scope,
            s("charging").into(),
            v8::Boolean::new(scope, true).into(),
        );
        result.set(
            scope,
            s("chargingTime").into(),
            v8::Number::new(scope, 0.0).into(),
        );
        result.set(
            scope,
            s("dischargingTime").into(),
            v8::Number::new(scope, f64::INFINITY).into(),
        );
        result.set(scope, s("level").into(), v8::Number::new(scope, 1.0).into());
        // Set Symbol.toStringTag = "BatteryManager" for fingerprint fidelity
        let tag_sym = v8::Symbol::get_to_string_tag(scope);
        let tag_val = crate::v8_utils::v8_string(scope, "BatteryManager");
        result.set(scope, tag_sym.into(), tag_val.into());
        let resolver = crate::v8_utils::v8_resolver(scope);
        resolver.resolve(scope, result.into());
        rv.set(resolver.get_promise(scope).into());
    }));
}

// navigator.sendBeacon(url, data) → true
unsafe extern "C" fn nav_send_beacon(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::Boolean::new(scope, true).into());
    }));
}

// Single no-op callback shared by stub methods (geolocation, clipboard, credentials)
unsafe extern "C" fn stub_noop(_info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // intentionally empty — stub method, returns undefined
    }));
}

// navigator.geolocation → object with stub methods
unsafe extern "C" fn nav_geolocation(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let build_stub = |name: &str| {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_noop).build(scope);
            let name_str = crate::v8_utils::v8_string(scope, name);
            tmpl.set_class_name(name_str);
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(
            scope,
            s("getCurrentPosition").into(),
            build_stub("getCurrentPosition").into(),
        );
        obj.set(
            scope,
            s("watchPosition").into(),
            build_stub("watchPosition").into(),
        );
        obj.set(
            scope,
            s("clearWatch").into(),
            build_stub("clearWatch").into(),
        );
        rv.set(obj.into());
    }));
}

// navigator.clipboard → object with stub methods
unsafe extern "C" fn nav_clipboard(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let build_stub = |name: &str| {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_noop).build(scope);
            let name_str = crate::v8_utils::v8_string(scope, name);
            tmpl.set_class_name(name_str);
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("readText").into(), build_stub("readText").into());
        obj.set(scope, s("writeText").into(), build_stub("writeText").into());
        rv.set(obj.into());
    }));
}

// navigator.credentials → object with stub methods
unsafe extern "C" fn nav_credentials(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let build_stub = |name: &str| {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_noop).build(scope);
            let name_str = crate::v8_utils::v8_string(scope, name);
            tmpl.set_class_name(name_str);
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("get").into(), build_stub("get").into());
        obj.set(scope, s("create").into(), build_stub("create").into());
        obj.set(scope, s("store").into(), build_stub("store").into());
        obj.set(
            scope,
            s("preventSilentAccess").into(),
            build_stub("preventSilentAccess").into(),
        );
        rv.set(obj.into());
    }));
}

// ─── navigator.plugins / navigator.mimeTypes ──────────────────────────────────

unsafe extern "C" fn nav_mime_types(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let arr = v8::Array::new(scope, 2);
        let ts = v8::Symbol::get_to_string_tag(scope);
        let st = |k: &str| crate::v8_utils::v8_string(scope, k);

        let make_mt = |typ: &str, suffixes: &str, desc: &str| {
            let obj = v8::Object::new(scope);
            obj.set(scope, st("type").into(), st(typ).into());
            obj.set(scope, st("suffixes").into(), st(suffixes).into());
            obj.set(scope, st("description").into(), st(desc).into());
            obj.set(scope, st("enabledPlugin").into(), v8::null(scope).into());
            obj.set(scope, ts.into(), st("MimeType").into());
            obj
        };

        let m1 = make_mt("application/pdf", "pdf", "Portable Document Format");
        let m2 = make_mt("text/pdf", "pdf", "Portable Document Format");
        arr.set_index(scope, 0, m1.into());
        arr.set_index(scope, 1, m2.into());
        arr.set(scope, ts.into(), st("MimeTypeArray").into());
        rv.set(arr.into());
    }));
}

unsafe extern "C" fn nav_plugins(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let arr = v8::Array::new(scope, 5);
        let ts = v8::Symbol::get_to_string_tag(scope);

        let make_mt = |typ: &str, suffixes: &str, desc: &str| {
            let obj = v8::Object::new(scope);
            let s = |k: &str| crate::v8_utils::v8_string(scope, k);
            obj.set(scope, s("type").into(), s(typ).into());
            obj.set(scope, s("suffixes").into(), s(suffixes).into());
            obj.set(scope, s("description").into(), s(desc).into());
            obj.set(scope, s("enabledPlugin").into(), v8::null(scope).into());
            obj.set(scope, ts.into(), s("MimeType").into());
            obj
        };

        let m1 = make_mt("application/pdf", "pdf", "Portable Document Format");
        let m2 = make_mt("text/pdf", "pdf", "Portable Document Format");

        let plugin_names = [
            "PDF Viewer",
            "Chrome PDF Viewer",
            "Chromium PDF Viewer",
            "Microsoft Edge PDF Viewer",
            "WebKit built-in PDF",
        ];

        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        for (i, name) in plugin_names.iter().enumerate() {
            let p = v8::Object::new(scope);
            p.set(scope, s("name").into(), s(name).into());
            p.set(scope, s("filename").into(), s("internal-pdf-viewer").into());
            p.set(
                scope,
                s("description").into(),
                s("Portable Document Format").into(),
            );
            p.set(
                scope,
                s("length").into(),
                v8::Number::new(scope, 2.0).into(),
            );
            p.set(scope, v8::Integer::new(scope, 0).into(), m1.into());
            p.set(scope, v8::Integer::new(scope, 1).into(), m2.into());
            p.set(scope, ts.into(), s("Plugin").into());
            arr.set_index(scope, i as u32, p.into());
        }
        arr.set(scope, ts.into(), s("PluginArray").into());
        rv.set(arr.into());
    }));
}

// ─── screen ───────────────────────────────────────────────────────────────────

fn install_native_screen(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    // v0.8.61: inherit from generated Screen template (9 skeleton properties)
    let gen_tmpl = create_screen_template(scope, None);
    let screen_tmpl = v8::FunctionTemplate::builder_raw(illegal_constructor).build(scope);
    screen_tmpl.set_class_name(crate::v8_utils::v8_string(scope, "Screen"));
    screen_tmpl.inherit(gen_tmpl);

    let proto = screen_tmpl.prototype_template(scope);

    macro_rules! screen_getter {
        ($name:literal, $cb:ident) => {
            let getter = v8::FunctionTemplate::builder_raw($cb).build(scope);
            let name = crate::v8_utils::v8_string(scope, $name);
            getter.set_class_name(name);
            getter.remove_prototype();
            proto.set_accessor_property(
                name.into(),
                Some(getter),
                None,
                v8::PropertyAttribute::DONT_DELETE | v8::PropertyAttribute::DONT_ENUM,
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

env_bool_getter!(
    nav_pdf_viewer_enabled,
    "navigator.pdfViewerEnabled",
    pdf_viewer_enabled,
    DEFAULT_PROFILE.pdf_viewer_enabled
);

// javaEnabled() → always returns false (no Java plugin in V8 context)
unsafe extern "C" fn nav_java_enabled(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::Boolean::new(scope, false).into());
    }));
}

// ─── v0.8.61 native stubs ────────────────────────────────────────────────────────

// navigator.getGamepads() → empty array []
unsafe extern "C" fn nav_get_gamepads(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let arr = v8::Array::new(scope, 0);
        rv.set(arr.into());
    }));
}

// navigator.requestMediaKeySystemAccess() → Promise.reject(TypeError) (M1 approximation)
unsafe extern "C" fn nav_request_media_key_system_access(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let global = scope.get_current_context().global(scope);
        if let Some(promise_ctor) = global.get(scope, v8_str(scope, "Promise").into()) {
            if promise_ctor.is_function() {
                let ctor: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(promise_ctor) };
                let reject_key = v8_str(scope, "reject");
                if let Some(reject_fn) = ctor.get(scope, reject_key.into()) {
                    if reject_fn.is_function() {
                        let reject: v8::Local<v8::Function> =
                            unsafe { v8::Local::cast_unchecked(reject_fn) };
                        // Use DOMException(NotSupportedError) instead of TypeError
                        let err_obj = build_dom_exception(scope, &global, "Unsupported keySystem", "NotSupportedError");
                        if let Some(promise) = reject.call(scope, ctor.into(), &[err_obj.into()]) {
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

// navigator.requestMIDIAccess() → Promise.reject(DOMException)
unsafe extern "C" fn nav_request_midi_access(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let global = scope.get_current_context().global(scope);
        if let Some(promise_ctor) = global.get(scope, v8_str(scope, "Promise").into()) {
            if promise_ctor.is_function() {
                let ctor: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(promise_ctor) };
                let reject_key = v8_str(scope, "reject");
                if let Some(reject_fn) = ctor.get(scope, reject_key.into()) {
                    if reject_fn.is_function() {
                        let reject: v8::Local<v8::Function> =
                            unsafe { v8::Local::cast_unchecked(reject_fn) };
                        let err_obj = build_dom_exception(scope, &global, "MIDI access not supported", "NotSupportedError");
                        if let Some(promise) = reject.call(scope, ctor.into(), &[err_obj.into()]) {
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

fn v8_str<'s>(scope: &'s v8::PinScope<'s, '_>, s: &str) -> v8::Local<'s, v8::String> {
    crate::v8_utils::v8_string(scope, s)
}

env_f64_getter!(
    screen_width,
    "screen.width",
    screen_width,
    DEFAULT_PROFILE.screen_width
);
env_f64_getter!(
    screen_height,
    "screen.height",
    screen_height,
    DEFAULT_PROFILE.screen_height
);
env_f64_getter!(
    screen_avail_width,
    "screen.availWidth",
    screen_avail_width,
    DEFAULT_PROFILE.screen_avail_width
);
env_f64_getter!(
    screen_avail_height,
    "screen.availHeight",
    screen_avail_height,
    DEFAULT_PROFILE.screen_avail_height
);
env_f64_getter!(
    screen_color_depth,
    "screen.colorDepth",
    screen_color_depth,
    DEFAULT_PROFILE.screen_color_depth
);
env_f64_getter!(
    screen_pixel_depth,
    "screen.pixelDepth",
    screen_pixel_depth,
    DEFAULT_PROFILE.screen_pixel_depth
);
env_f64_getter!(
    screen_avail_left,
    "screen.availLeft",
    screen_avail_left,
    DEFAULT_PROFILE.screen_avail_left
);
env_f64_getter!(
    screen_avail_top,
    "screen.availTop",
    screen_avail_top,
    DEFAULT_PROFILE.screen_avail_top
);
