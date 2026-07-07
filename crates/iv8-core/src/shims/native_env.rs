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

/// Install native-getter versions of navigator and screen on the global.
/// Must be called AFTER env_inject (so the base objects exist) but BEFORE
/// any JS that reads these properties.
pub fn install_native_env(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    install_native_navigator(scope, global);
    install_native_screen(scope, global);
    install_worker_navigator(scope, global);
}

// ─── navigator ────────────────────────────────────────────────────────────────

/// Install accessor getters on an already-instantiated prototype object using
/// `Object.defineProperty`. Each getter is created via FunctionTemplate to
/// preserve `[native code]` toString.
fn install_getters_on_proto(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
    proto: v8::Local<v8::Object>,
    getters: &[(&str, v8::FunctionCallback)],
) {
    let obj_key = crate::v8_utils::v8_string(scope, "Object");
    let obj_val = match global.get(scope, obj_key.into()) {
        Some(v) if v.is_object() => v,
        _ => return,
    };
    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(obj_val) };
    let dop_key = crate::v8_utils::v8_string(scope, "defineProperty");
    let dop_fn: v8::Local<v8::Function> = match obj.get(scope, dop_key.into()) {
        Some(v) if v.is_function() => unsafe { v8::Local::cast_unchecked(v) },
        _ => return,
    };
    let get_key = crate::v8_utils::v8_string(scope, "get");
    let enum_key = crate::v8_utils::v8_string(scope, "enumerable");
    let config_key = crate::v8_utils::v8_string(scope, "configurable");

    for (name, cb) in getters {
        let getter_tmpl = v8::FunctionTemplate::builder_raw(*cb).build(scope);
        let name_str = crate::v8_utils::v8_string(scope, name);
        let class_name = format!("get {}", name);
        let class_name_str = crate::v8_utils::v8_string(scope, &class_name);
        getter_tmpl.set_class_name(class_name_str);
        getter_tmpl.remove_prototype();
        let getter_fn = match getter_tmpl.get_function(scope) {
            Some(f) => f,
            None => continue,
        };
        let name_prop_key = crate::v8_utils::v8_string(scope, "name");
        let _ = getter_fn.define_own_property(
            scope,
            name_prop_key.into(),
            class_name_str.into(),
            v8::PropertyAttribute::DONT_ENUM | v8::PropertyAttribute::READ_ONLY,
        );
        let desc = v8::Object::new(scope);
        let _ = desc.set(scope, get_key.into(), getter_fn.into());
        let _ = desc.set(scope, enum_key.into(), v8::Boolean::new(scope, true).into());
        let _ = desc.set(scope, config_key.into(), v8::Boolean::new(scope, true).into());
        let _ = dop_fn.call(scope, obj.into(), &[proto.into(), name_str.into(), desc.into()]);
    }
}

/// Install method functions on an already-instantiated prototype object.
fn install_methods_on_proto(
    scope: &v8::PinScope<'_, '_>,
    proto: v8::Local<v8::Object>,
    methods: &[(&str, v8::FunctionCallback)],
) {
    for (name, cb) in methods {
        let fn_tmpl = v8::FunctionTemplate::builder_raw(*cb).build(scope);
        let name_str = crate::v8_utils::v8_string(scope, name);
        fn_tmpl.set_class_name(name_str);
        fn_tmpl.remove_prototype();
        if let Some(func) = fn_tmpl.get_function(scope) {
            let _ = proto.set(scope, name_str.into(), func.into());
        }
    }
}

/// Illegal constructor — Navigator and Screen are not constructable from JS.
/// Throws TypeError: Illegal constructor, matching real browser behavior.
unsafe extern "C" fn illegal_constructor(info: *const v8::FunctionCallbackInfo) {
    let info_ref = unsafe { &*info };
    v8::callback_scope!(unsafe scope, info_ref);
    let msg = crate::v8_utils::v8_string(scope, "Illegal constructor");
    let exc = v8::Exception::type_error(scope, msg);
    scope.throw_exception(exc);
}

/// Global getter for `navigator` — returns the stored navigator instance.
/// Reads from globalThis.__iv8NavInst__ (set by JS shim before this getter).
unsafe extern "C" fn navigator_global_getter(info: *const v8::FunctionCallbackInfo) {
    let info_ref = unsafe { &*info };
    v8::callback_scope!(unsafe scope, info_ref);
    let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
    let this = args.this();
    let key = v8::String::new(scope, "__iv8NavInst__").unwrap();
    let val = this.get(scope, key.into());
    let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
    rv.set(val.unwrap_or(v8::undefined(scope).into()));
}

/// Global getter for `screen` — returns the stored screen instance.
unsafe extern "C" fn screen_global_getter(info: *const v8::FunctionCallbackInfo) {
    let info_ref = unsafe { &*info };
    v8::callback_scope!(unsafe scope, info_ref);
    let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
    let this = args.this();
    let key = v8::String::new(scope, "__iv8ScreenInst__").unwrap();
    let val = this.get(scope, key.into());
    let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
    rv.set(val.unwrap_or(v8::undefined(scope).into()));
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
///
/// v0.8.78: Rewritten prototype chain strategy.
/// Instead of creating a gen_tmpl via create_navigator_template(scope, None)
/// and overwriting the global Navigator constructor (which broke
/// `navigator instanceof EventTarget`), we now:
/// 1. Create nav_tmpl WITHOUT inheriting any gen_tmpl.
/// 2. Install native getters on nav_tmpl.prototype_template.
/// 3. Instantiate navigator from nav_tmpl.instance_template.
/// 4. Link nav_tmpl.prototype.__proto__ to install_all's Navigator.prototype.
/// 5. Also install getters/methods on install_all's Navigator.prototype via
///    Object.defineProperty (for descriptor shape detection).
/// 6. Do NOT overwrite the global Navigator constructor.
///
/// Result: navigator instanceof Navigator === true AND
///         navigator instanceof EventTarget === true.
fn install_native_navigator(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    let nav_tmpl = v8::FunctionTemplate::builder_raw(illegal_constructor).build(scope);
    nav_tmpl.set_class_name(crate::v8_utils::v8_string(scope, "Navigator"));

    let proto = nav_tmpl.prototype_template(scope);
    macro_rules! nav_getter {
        ($name:literal, $cb:ident) => {
            let getter = v8::FunctionTemplate::builder_raw($cb).build(scope);
            let name = crate::v8_utils::v8_string(scope, $name);
            let get_name = crate::v8_utils::v8_string(scope, concat!("get ", $name));
            getter.set_class_name(get_name);
            getter.remove_prototype();
            proto.set_accessor_property(
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
    nav_getter!("plugins", nav_plugins);
    nav_getter!("mimeTypes", nav_mime_types);
    nav_getter!("connection", nav_connection);

    // navigator.globalPrivacyControl → false (Chrome default, not enabled)
    nav_getter!("globalPrivacyControl", nav_global_privacy_control);

    // navigator.pdfViewerEnabled is already a profile-backed getter above

    let battery_fn = v8::FunctionTemplate::builder_raw(nav_get_battery).build(scope);
    let battery_name = crate::v8_utils::v8_string(scope, "getBattery");
    battery_fn.set_class_name(battery_name);
    battery_fn.remove_prototype();
    proto.set(battery_name.into(), battery_fn.into());

    let beacon_fn = v8::FunctionTemplate::builder_raw(nav_send_beacon).build(scope);
    let beacon_name = crate::v8_utils::v8_string(scope, "sendBeacon");
    beacon_fn.set_class_name(beacon_name);
    beacon_fn.remove_prototype();
    proto.set(beacon_name.into(), beacon_fn.into());

    nav_getter!("geolocation", nav_geolocation);
    nav_getter!("clipboard", nav_clipboard);
    nav_getter!("credentials", nav_credentials);

    nav_getter!("storageBuckets", nav_storage_buckets);
    nav_getter!("storage", nav_storage);
    nav_getter!("locks", nav_locks);
    nav_getter!("gpu", nav_gpu);
    nav_getter!("mediaCapabilities", nav_media_capabilities);
    nav_getter!("mediaSession", nav_media_session);
    nav_getter!("presentation", nav_presentation);
    nav_getter!("bluetooth", nav_bluetooth);
    nav_getter!("usb", nav_usb);
    nav_getter!("wakeLock", nav_wake_lock);
    nav_getter!("hid", nav_hid);
    nav_getter!("serial", nav_serial);
    nav_getter!("ink", nav_ink);
    nav_getter!("xr", nav_xr);
    nav_getter!("keyboard", nav_keyboard);
    nav_getter!("managed", nav_managed);
    nav_getter!("taintEnabled", nav_taint_enabled);
    nav_getter!("buildID", nav_build_id);
    nav_getter!("protectedAudience", nav_protected_audience);
    nav_getter!("webkitPersistentStorage", nav_webkit_persistent_storage);
    nav_getter!("webkitTemporaryStorage", nav_webkit_temporary_storage);

    let gum_fn = v8::FunctionTemplate::builder_raw(nav_get_user_media).build(scope);
    let gum_name = crate::v8_utils::v8_string(scope, "getUserMedia");
    gum_fn.set_class_name(gum_name);
    gum_fn.remove_prototype();
    proto.set(gum_name.into(), gum_fn.into());

    let wgum_fn = v8::FunctionTemplate::builder_raw(nav_get_user_media).build(scope);
    let wgum_name = crate::v8_utils::v8_string(scope, "webkitGetUserMedia");
    wgum_fn.set_class_name(wgum_name);
    wgum_fn.remove_prototype();
    proto.set(wgum_name.into(), wgum_fn.into());

    let vibrate_fn = v8::FunctionTemplate::builder_raw(nav_vibrate).build(scope);
    let vibrate_name = crate::v8_utils::v8_string(scope, "vibrate");
    vibrate_fn.set_class_name(vibrate_name);
    vibrate_fn.remove_prototype();
    proto.set(vibrate_name.into(), vibrate_fn.into());

    let share_fn = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
    let share_name = crate::v8_utils::v8_string(scope, "share");
    share_fn.set_class_name(share_name);
    share_fn.remove_prototype();
    proto.set(share_name.into(), share_fn.into());

    let canshare_fn = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
    let canshare_name = crate::v8_utils::v8_string(scope, "canShare");
    canshare_fn.set_class_name(canshare_name);
    canshare_fn.remove_prototype();
    proto.set(canshare_name.into(), canshare_fn.into());

    macro_rules! nav_method_stub {
        ($name:literal, $cb:ident) => {
            let f = v8::FunctionTemplate::builder_raw($cb).build(scope);
            let n = crate::v8_utils::v8_string(scope, $name);
            f.set_class_name(n);
            f.remove_prototype();
            proto.set(n.into(), f.into());
        };
        ($name:literal, $cb:ident, $len:expr) => {
            let f = v8::FunctionTemplate::builder_raw($cb).length($len).build(scope);
            let n = crate::v8_utils::v8_string(scope, $name);
            f.set_class_name(n);
            f.remove_prototype();
            proto.set(n.into(), f.into());
        };
    }
    nav_method_stub!("registerProtocolHandler", stub_noop, 2);
    nav_method_stub!("unregisterProtocolHandler", stub_noop, 2);
    nav_method_stub!("runAdAuction", stub_promise_resolve_null);
    nav_method_stub!("joinAdInterestGroup", stub_noop);
    nav_method_stub!("leaveAdInterestGroup", stub_noop);
    nav_method_stub!("updateAdInterestGroups", stub_noop);
    nav_method_stub!("clearOriginJoinedAdInterestGroups", stub_noop);
    nav_method_stub!("createAuctionNonce", nav_create_auction_nonce);
    nav_method_stub!("deprecatedURNToURL", stub_promise_resolve_null);
    nav_method_stub!("deprecatedRunAdAuctionEnforcesKAnonymity", stub_promise_resolve);
    nav_method_stub!("getInterestGroupAdAuctionData", stub_promise_resolve_null);
    nav_method_stub!("canLoadAdAuctionFencedFrame", nav_can_load_ad_auction_fenced_frame);
    nav_method_stub!("setAppBadge", stub_promise_resolve);
    nav_method_stub!("clearAppBadge", stub_promise_resolve);

    let java_fn = v8::FunctionTemplate::builder_raw(nav_java_enabled).build(scope);
    let java_name = crate::v8_utils::v8_string(scope, "javaEnabled");
    java_fn.set_class_name(java_name);
    java_fn.remove_prototype();
    proto.set(java_name.into(), java_fn.into());

    let gamepads_fn = v8::FunctionTemplate::builder_raw(nav_get_gamepads).build(scope);
    let gamepads_name = crate::v8_utils::v8_string(scope, "getGamepads");
    gamepads_fn.set_class_name(gamepads_name);
    gamepads_fn.remove_prototype();
    proto.set(gamepads_name.into(), gamepads_fn.into());

    let eme_fn =
        v8::FunctionTemplate::builder_raw(nav_request_media_key_system_access).build(scope);
    let eme_name = crate::v8_utils::v8_string(scope, "requestMediaKeySystemAccess");
    eme_fn.set_class_name(eme_name);
    eme_fn.remove_prototype();
    proto.set(eme_name.into(), eme_fn.into());

    let midi_fn = v8::FunctionTemplate::builder_raw(nav_request_midi_access).build(scope);
    let midi_name = crate::v8_utils::v8_string(scope, "requestMIDIAccess");
    midi_fn.set_class_name(midi_name);
    midi_fn.remove_prototype();
    proto.set(midi_name.into(), midi_fn.into());

    let inst_tmpl = nav_tmpl.instance_template(scope);
    if let Some(nav_obj) = inst_tmpl.new_instance(scope) {
        // Link nav_tmpl.prototype.__proto__ to install_all's Navigator.prototype.
        if let Some(nav_func) = nav_tmpl.get_function(scope) {
            let proto_key = crate::v8_utils::v8_string(scope, "prototype");
            if let Some(nav_proto_val) = nav_func.get(scope, proto_key.into()) {
                if nav_proto_val.is_object() {
                    let nav_proto: v8::Local<v8::Object> =
                        unsafe { v8::Local::cast_unchecked(nav_proto_val) };
                    let nav_ctor_key = crate::v8_utils::v8_string(scope, "Navigator");
                    if let Some(nav_ctor_val) = global.get(scope, nav_ctor_key.into()) {
                        if nav_ctor_val.is_function() {
                            let nav_ctor: v8::Local<v8::Function> =
                                unsafe { v8::Local::cast_unchecked(nav_ctor_val) };
                            if let Some(install_all_proto_val) = nav_ctor.get(scope, proto_key.into()) {
                                if install_all_proto_val.is_object() {
                                    let install_all_proto: v8::Local<v8::Object> =
                                        unsafe { v8::Local::cast_unchecked(install_all_proto_val) };
                                    let _ = nav_proto.set_prototype(scope, install_all_proto.into());

                                    install_getters_on_proto(scope, global, install_all_proto, &[
                                        ("userAgent", nav_user_agent),
                                        ("appVersion", nav_app_version),
                                        ("platform", nav_platform),
                                        ("vendor", nav_vendor),
                                        ("vendorSub", nav_vendor_sub),
                                        ("product", nav_product),
                                        ("productSub", nav_product_sub),
                                        ("language", nav_language),
                                        ("languages", nav_languages),
                                        ("hardwareConcurrency", nav_hardware_concurrency),
                                        ("deviceMemory", nav_device_memory),
                                        ("maxTouchPoints", nav_max_touch_points),
                                        ("cookieEnabled", nav_cookie_enabled),
                                        ("onLine", nav_online),
                                        ("doNotTrack", nav_do_not_track),
                                        ("webdriver", nav_webdriver),
                                        ("appName", nav_app_name),
                                        ("appCodeName", nav_app_code_name),
                                        ("permissions", nav_permissions),
                                        ("mediaDevices", nav_media_devices),
                                        ("serviceWorker", nav_service_worker),
                                        ("pdfViewerEnabled", nav_pdf_viewer_enabled),
                                        ("plugins", nav_plugins),
                                        ("mimeTypes", nav_mime_types),
                                        ("connection", nav_connection),
                                        ("geolocation", nav_geolocation),
                                        ("clipboard", nav_clipboard),
                                        ("credentials", nav_credentials),
                                        ("storageBuckets", nav_storage_buckets),
                                        ("storage", nav_storage),
                                        ("locks", nav_locks),
                                        ("gpu", nav_gpu),
                                        ("mediaCapabilities", nav_media_capabilities),
                                        ("mediaSession", nav_media_session),
                                        ("presentation", nav_presentation),
                                        ("bluetooth", nav_bluetooth),
                                        ("usb", nav_usb),
                                        ("wakeLock", nav_wake_lock),
                                        ("hid", nav_hid),
                                        ("serial", nav_serial),
                                        ("ink", nav_ink),
                                        ("xr", nav_xr),
                                        ("keyboard", nav_keyboard),
                                        ("managed", nav_managed),
                                        ("taintEnabled", nav_taint_enabled),
                                        ("buildID", nav_build_id),
                                        ("protectedAudience", nav_protected_audience),
                                        ("webkitPersistentStorage", nav_webkit_persistent_storage),
                                        ("webkitTemporaryStorage", nav_webkit_temporary_storage),
                                    ]);
                                    install_methods_on_proto(scope, install_all_proto, &[
                                        ("getBattery", nav_get_battery),
                                        ("sendBeacon", nav_send_beacon),
                                        ("javaEnabled", nav_java_enabled),
                                        ("getGamepads", nav_get_gamepads),
                                        ("requestMediaKeySystemAccess", nav_request_media_key_system_access),
                                        ("requestMIDIAccess", nav_request_midi_access),
                                        ("getUserMedia", nav_get_user_media),
                                        ("webkitGetUserMedia", nav_get_user_media),
                                        ("vibrate", nav_vibrate),
                                        ("share", stub_promise_resolve),
                                        ("canShare", stub_promise_resolve),
                                        ("registerProtocolHandler", stub_noop),
                                        ("unregisterProtocolHandler", stub_noop),
                                        ("runAdAuction", stub_promise_resolve_null),
                                        ("joinAdInterestGroup", stub_noop),
                                        ("leaveAdInterestGroup", stub_noop),
                                        ("updateAdInterestGroups", stub_noop),
                                        ("clearOriginJoinedAdInterestGroups", stub_noop),
                                        ("createAuctionNonce", nav_create_auction_nonce),
                                        ("deprecatedURNToURL", stub_promise_resolve_null),
                                        ("deprecatedRunAdAuctionEnforcesKAnonymity", stub_promise_resolve),
                                        ("getInterestGroupAdAuctionData", stub_promise_resolve_null),
                                        ("canLoadAdAuctionFencedFrame", nav_can_load_ad_auction_fenced_frame),
                                        ("setAppBadge", stub_promise_resolve),
                                        ("clearAppBadge", stub_promise_resolve),
                                    ]);
                                }
                            }
                        }
                    }
                }
            }
        }

        crate::shims::user_agent_data::install_user_agent_data(scope, nav_obj);
        conditionally_hide_properties(scope, nav_obj);

        let key = crate::v8_utils::v8_string(scope, "navigator");
        // Store navigator instance on globalThis.__iv8NavInst__, then install
        // as accessor property so descriptor returns {get: function, set: undefined}
        let store_key = crate::v8_utils::v8_string(scope, "__iv8NavInst__");
        let _ = global.set(scope, store_key.into(), nav_obj.into());
        let getter_tmpl = v8::FunctionTemplate::builder_raw(navigator_global_getter).build(scope);
        getter_tmpl.set_class_name(crate::v8_utils::v8_string(scope, "get navigator"));
        getter_tmpl.remove_prototype();
        if let Some(getter_fn) = getter_tmpl.get_function(scope) {
            let mut desc = v8::PropertyDescriptor::new_from_get_set(
                getter_fn.into(),
                v8::undefined(scope).into(),
            );
            desc.set_enumerable(true);
            desc.set_configurable(true);
            let _ = global.define_property(scope, key.into(), &desc);
        }

        // Overwrite global Navigator constructor with illegal_constructor,
        // but preserve install_all's Navigator.prototype (which has EventTarget
        // inheritance + our native getters). We create a new FunctionTemplate
        // with illegal_constructor, get its function, then set its .prototype
        // to install_all's Navigator.prototype.
        let nav_ctor_tmpl = v8::FunctionTemplate::builder_raw(illegal_constructor).build(scope);
        nav_ctor_tmpl.set_class_name(crate::v8_utils::v8_string(scope, "Navigator"));
        nav_ctor_tmpl.remove_prototype();
        if let Some(nav_ctor_fn) = nav_ctor_tmpl.get_function(scope) {
            // Set the constructor's .prototype to install_all's Navigator.prototype
            let proto_key = crate::v8_utils::v8_string(scope, "prototype");
            let nav_ctor_key = crate::v8_utils::v8_string(scope, "Navigator");
            if let Some(install_all_nav_ctor_val) = global.get(scope, nav_ctor_key.into()) {
                if install_all_nav_ctor_val.is_function() {
                    let install_all_nav_ctor: v8::Local<v8::Function> =
                        unsafe { v8::Local::cast_unchecked(install_all_nav_ctor_val) };
                    if let Some(install_all_proto_val) = install_all_nav_ctor.get(scope, proto_key.into()) {
                        let _ = nav_ctor_fn.set(scope, proto_key.into(), install_all_proto_val);
                    }
                }
            }
            let _ = global.define_own_property(
                scope,
                nav_ctor_key.into(),
                nav_ctor_fn.into(),
                v8::PropertyAttribute::DONT_ENUM,
            );
        }
    }
}

// ─── WorkerNavigator (v0.8.70) ────────────────────────────────────────────

/// Install a native WorkerNavigator constructor that inherits from the
/// generated create_worker_navigator_template and uses illegal_constructor
/// so that `new WorkerNavigator()` throws TypeError.
/// Note: parent=None is used because we can't get EventTarget FunctionTemplate
/// from an already-instantiated function. fix_prototype_chains in
/// embedded_v8.rs patches WorkerNavigator.prototype.__proto__ = EventTarget.prototype.
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

pub fn check_receiver(
    scope: &v8::PinScope<'_, '_>,
    info_ref: &v8::FunctionCallbackInfo,
    iface_name: &str,
) -> bool {
    let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
    let this = args.this();
    let ctx = scope.get_current_context();
    let global = ctx.global(scope);
    let iface_str = match v8::String::new(scope, iface_name) {
        Some(s) => s,
        None => return true,
    };
    let Some(ctor_val) = global.get(scope, iface_str.into()) else {
        return true;
    };
    if !ctor_val.is_function() {
        return true;
    }
    let ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(ctor_val) };
    let proto_key = match v8::String::new(scope, "prototype") {
        Some(s) => s,
        None => return true,
    };
    let Some(proto_val) = ctor.get(scope, proto_key.into()) else {
        return true;
    };
    if !proto_val.is_object() || proto_val.is_null_or_undefined() {
        return true;
    }
    let proto: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(proto_val) };
    if this.strict_equals(proto.into()) {
        let msg = v8::String::new(scope, "Illegal invocation").unwrap();
        let exc = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exc);
        return false;
    }
    let mut current: v8::Local<v8::Value> = this.into();
    for _ in 0..20usize {
        let Some(cur_obj) = current.to_object(scope) else { break; };
        let Some(parent) = cur_obj.get_prototype(scope) else { break; };
        if parent.is_null_or_undefined() || !parent.is_object() {
            break;
        }
        if parent.strict_equals(proto.into()) {
            return true;
        }
        current = parent;
    }
    let msg = v8::String::new(scope, "Illegal invocation").unwrap();
    let exc = v8::Exception::type_error(scope, msg);
    scope.throw_exception(exc);
    false
}

macro_rules! env_str_getter {
    ($name:ident, $path:literal, $field:ident, $default:expr, $iface:literal) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let info_ref = unsafe { &*info };
                v8::callback_scope!(unsafe scope, info_ref);
                if !check_receiver(&scope, info_ref, $iface) {
                    return;
                }
                let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
                let isolate: &v8::Isolate = &*scope;
                let state = RuntimeState::get(isolate);
                // D-111: user_overrides > profile > DEFAULT_PROFILE
                let val = state
                    .environment
                    .get_user_str($path)
                    .unwrap_or_else(|| state.profile.map(|p| p.$field).unwrap_or($default));
                if let Some(s) = v8::String::new(scope, val) {
                    rv.set(s.into());
                }
            }));
        }
    };
}

macro_rules! env_f64_getter {
    ($name:ident, $path:literal, $field:ident, $default:expr, $iface:literal) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let info_ref = unsafe { &*info };
                v8::callback_scope!(unsafe scope, info_ref);
                if !check_receiver(&scope, info_ref, $iface) {
                    return;
                }
                let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
                let isolate: &v8::Isolate = &*scope;
                let state = RuntimeState::get(isolate);
                // D-111: user_overrides > profile > DEFAULT_PROFILE
                let val = state
                    .environment
                    .get_user_f64($path)
                    .unwrap_or_else(|| state.profile.map(|p| p.$field).unwrap_or($default));
                rv.set(v8::Number::new(scope, val).into());
            }));
        }
    };
}

macro_rules! env_bool_getter {
    ($name:ident, $path:literal, $field:ident, $default:expr, $iface:literal) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let info_ref = unsafe { &*info };
                v8::callback_scope!(unsafe scope, info_ref);
                if !check_receiver(&scope, info_ref, $iface) {
                    return;
                }
                let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
                let isolate: &v8::Isolate = &*scope;
                let state = RuntimeState::get(isolate);
                // D-111: user_overrides > profile > DEFAULT_PROFILE
                let val = state
                    .environment
                    .get_user_bool($path)
                    .unwrap_or_else(|| state.profile.map(|p| p.$field).unwrap_or($default));
                rv.set(v8::Boolean::new(scope, val).into());
            }));
        }
    };
}

env_str_getter!(
    nav_user_agent,
    "navigator.userAgent",
    user_agent,
    DEFAULT_PROFILE.user_agent,
    "Navigator"
);
env_str_getter!(
    nav_app_version,
    "navigator.appVersion",
    app_version,
    DEFAULT_PROFILE.app_version,
    "Navigator"
);
env_str_getter!(
    nav_platform,
    "navigator.platform",
    platform,
    DEFAULT_PROFILE.platform,
    "Navigator"
);
env_str_getter!(
    nav_vendor,
    "navigator.vendor",
    vendor,
    DEFAULT_PROFILE.vendor,
    "Navigator"
);
env_str_getter!(
    nav_vendor_sub,
    "navigator.vendorSub",
    vendor_sub,
    DEFAULT_PROFILE.vendor_sub,
    "Navigator"
);
env_str_getter!(
    nav_product,
    "navigator.product",
    product,
    DEFAULT_PROFILE.product,
    "Navigator"
);
env_str_getter!(
    nav_product_sub,
    "navigator.productSub",
    product_sub,
    DEFAULT_PROFILE.product_sub,
    "Navigator"
);
env_str_getter!(
    nav_language,
    "navigator.language",
    language,
    DEFAULT_PROFILE.language,
    "Navigator"
);
env_str_getter!(
    nav_app_name,
    "navigator.appName",
    app_name,
    DEFAULT_PROFILE.app_name,
    "Navigator"
);
env_str_getter!(
    nav_app_code_name,
    "navigator.appCodeName",
    app_code_name,
    DEFAULT_PROFILE.app_code_name,
    "Navigator"
);
env_f64_getter!(
    nav_hardware_concurrency,
    "navigator.hardwareConcurrency",
    hardware_concurrency,
    DEFAULT_PROFILE.hardware_concurrency,
    "Navigator"
);
env_f64_getter!(
    nav_device_memory,
    "navigator.deviceMemory",
    device_memory,
    DEFAULT_PROFILE.device_memory,
    "Navigator"
);
env_f64_getter!(
    nav_max_touch_points,
    "navigator.maxTouchPoints",
    max_touch_points,
    DEFAULT_PROFILE.max_touch_points,
    "Navigator"
);
env_bool_getter!(
    nav_cookie_enabled,
    "navigator.cookieEnabled",
    cookie_enabled,
    DEFAULT_PROFILE.cookie_enabled,
    "Navigator"
);
env_bool_getter!(
    nav_online,
    "navigator.onLine",
    on_line,
    DEFAULT_PROFILE.on_line,
    "Navigator"
);

// navigator.languages → array from environment
unsafe extern "C" fn nav_languages(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
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
                state
                    .profile
                    .map(|p| p.languages.iter().map(|s| s.to_string()).collect())
                    .unwrap_or_else(|| {
                        DEFAULT_PROFILE
                            .languages
                            .iter()
                            .map(|s| s.to_string())
                            .collect()
                    })
            }
        } else {
            state
                .profile
                .map(|p| p.languages.iter().map(|s| s.to_string()).collect())
                .unwrap_or_else(|| {
                    DEFAULT_PROFILE
                        .languages
                        .iter()
                        .map(|s| s.to_string())
                        .collect()
                })
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
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
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
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
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
                        // Permission state mapping aligned with Chrome defaults.
                        // Auto-granted: sensors/media APIs (Chrome grants by default).
                        // Prompt: user-decision APIs.
                        // Denied: deprecated/non-standard.
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
                                    // Read from environment map (profile-driven).
                                    // Falls back to "prompt" for any permission
                                    // not in the profile.
                                    let env_key = format!("permissions.{}", name);
                                    state_str = state
                                        .environment
                                        .get_str(&env_key)
                                        .unwrap_or_else(|| {
                                            // Chrome default: sensors/media APIs auto-granted.
                                            match name.as_str() {
                                                "accelerometer" | "gyroscope" | "magnetometer"
                                                | "ambient-light-sensor" | "background-sync"
                                                | "midi" | "clipboard-write"
                                                | "screen-wake-lock" => "granted",
                                                _ => "prompt",
                                            }
                                        });
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
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
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
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        obj.set(scope, s("controller").into(), v8::null(scope).into());
        obj.set(scope, s("ready").into(), v8::null(scope).into());
        // register(scriptURL) → Promise<ServiceWorkerRegistration>
        let register_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve_null).build(scope);
            tmpl.set_class_name(s("register"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("register").into(), register_fn.into());
        // getRegistration → Promise<undefined>
        let get_reg_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
            tmpl.set_class_name(s("getRegistration"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("getRegistration").into(), get_reg_fn.into());
        // getRegistrations → Promise<[]>
        let get_regs_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve_empty_array).build(scope);
            tmpl.set_class_name(s("getRegistrations"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("getRegistrations").into(), get_regs_fn.into());
        rv.set(obj.into());
    }));
}

// navigator.webdriver → undefined in real browsers (anti-detection)
// In strict_compat mode: false (matching iv8 0.1.2 behavior)
unsafe extern "C" fn nav_webdriver(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
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
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let val = state
            .environment
            .get_str("navigator.doNotTrack")
            .map(|s| s.to_string())
            .or_else(|| state.profile.and_then(|p| p.do_not_track.map(|s| s.to_string())));
        if let Some(s) = val {
            if let Some(v) = v8::String::new(scope, &s) {
                rv.set(v.into());
                return;
            }
        }
        rv.set(v8::null(scope).into());
    }));
}

// navigator.globalPrivacyControl → false (Chrome default)
unsafe extern "C" fn nav_global_privacy_control(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let val = state
            .environment
            .get_str("navigator.globalPrivacyControl");
        match val {
            Some("true") => rv.set(v8::Boolean::new(scope, true).into()),
            _ => rv.set(v8::Boolean::new(scope, false).into()),
        }
    }));
}
unsafe extern "C" fn nav_connection(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let env = &state.environment;
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let eff_type = env.get_str("network.effectiveType").unwrap_or("4g");
        obj.set(scope, s("effectiveType").into(), s(eff_type).into());
        let downlink = env.get_f64("network.downlink").unwrap_or(10.0);
        obj.set(scope, s("downlink").into(), v8::Number::new(scope, downlink).into());
        let rtt = env.get_f64("network.rtt").unwrap_or(50.0);
        obj.set(scope, s("rtt").into(), v8::Number::new(scope, rtt).into());
        let save_data = env.get_str("network.saveData").map(|v| v == "true").unwrap_or(false);
        obj.set(scope, s("saveData").into(), v8::Boolean::new(scope, save_data).into());
        let net_type = env.get_str("network.type").unwrap_or("wifi");
        obj.set(scope, s("type").into(), s(net_type).into());
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
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let env = &state.environment;
        let result = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let charging = env.get_str("battery.charging").map(|v| v == "true").unwrap_or(true);
        result.set(scope, s("charging").into(), v8::Boolean::new(scope, charging).into());
        let charging_time = env.get_f64("battery.chargingTime").unwrap_or(0.0);
        result.set(scope, s("chargingTime").into(), v8::Number::new(scope, charging_time).into());
        let discharging_time = env.get_f64("battery.dischargingTime").unwrap_or(f64::INFINITY);
        result.set(scope, s("dischargingTime").into(), v8::Number::new(scope, discharging_time).into());
        let level = env.get_f64("battery.level").unwrap_or(1.0);
        result.set(scope, s("level").into(), v8::Number::new(scope, level).into());
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

// Stub that returns Promise.resolve() — for APIs that should return Promises
unsafe extern "C" fn stub_promise_resolve(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let resolver = crate::v8_utils::v8_resolver(scope);
        resolver.resolve(scope, v8::undefined(scope).into());
        rv.set(resolver.get_promise(scope).into());
    }));
}

// Stub that returns Promise.resolve(null) — for APIs that resolve to null
unsafe extern "C" fn stub_promise_resolve_null(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let resolver = crate::v8_utils::v8_resolver(scope);
        resolver.resolve(scope, v8::null(scope).into());
        rv.set(resolver.get_promise(scope).into());
    }));
}

// Stub that returns Promise.resolve([]) — for APIs that resolve to empty array
unsafe extern "C" fn stub_promise_resolve_empty_array(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let resolver = crate::v8_utils::v8_resolver(scope);
        resolver.resolve(scope, v8::Array::new(scope, 0).into());
        rv.set(resolver.get_promise(scope).into());
    }));
}

// Build the Geolocation error object: {code:1, message:"...", PERMISSION_DENIED:1}
fn build_geo_permission_denied<'s>(
    scope: &'s v8::PinScope<'s, '_>,
) -> v8::Local<'s, v8::Object> {
    let obj = v8::Object::new(scope);
    let s = |k: &str| crate::v8_utils::v8_string(scope, k);
    obj.set(scope, s("code").into(), v8::Integer::new(scope, 1).into());
    obj.set(
        scope,
        s("message").into(),
        crate::v8_utils::v8_string(scope, "User denied Geolocation").into(),
    );
    obj.set(
        scope,
        s("PERMISSION_DENIED").into(),
        v8::Integer::new(scope, 1).into(),
    );
    obj
}

// Call setTimeout(errorCb, 0, errorObj) on the global object.
// If errorCb is missing or not a function, does nothing (no hang).
fn schedule_geo_error(
    scope: &v8::PinScope<'_, '_>,
    error_cb: Option<v8::Local<v8::Function>>,
) {
    let Some(cb) = error_cb else {
        return;
    };
    let global = scope.get_current_context().global(scope);
    let set_timeout_key = crate::v8_utils::v8_string(scope, "setTimeout");
    if let Some(set_timeout_val) = global.get(scope, set_timeout_key.into()) {
        if set_timeout_val.is_function() {
            let set_timeout: v8::Local<v8::Function> =
                unsafe { v8::Local::cast_unchecked(set_timeout_val) };
            let err_obj = build_geo_permission_denied(scope);
            let delay = v8::Integer::new(scope, 0);
            set_timeout.call(
                scope,
                global.into(),
                &[cb.into(), delay.into(), err_obj.into()],
            );
        }
    }
}

// navigator.geolocation.getCurrentPosition(success, error, options)
// → calls error callback with PERMISSION_DENIED (code=1) via setTimeout
unsafe extern "C" fn geo_get_current_position(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let error_cb = if args.length() >= 2 && args.get(1).is_function() {
            Some(unsafe { v8::Local::<v8::Function>::cast_unchecked(args.get(1)) })
        } else {
            None
        };
        schedule_geo_error(&scope, error_cb);
    }));
}

// navigator.geolocation.watchPosition(success, error, options)
// → same as getCurrentPosition but returns a fake watch ID
unsafe extern "C" fn geo_watch_position(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let error_cb = if args.length() >= 2 && args.get(1).is_function() {
            Some(unsafe { v8::Local::<v8::Function>::cast_unchecked(args.get(1)) })
        } else {
            None
        };
        schedule_geo_error(&scope, error_cb);
        rv.set(v8::Integer::new(scope, 1).into());
    }));
}

// navigator.geolocation → object with methods
unsafe extern "C" fn nav_geolocation(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let build_fn = |name: &str,
                        cb: unsafe extern "C" fn(*const v8::FunctionCallbackInfo)| {
            let tmpl = v8::FunctionTemplate::builder_raw(cb).build(scope);
            let name_str = crate::v8_utils::v8_string(scope, name);
            tmpl.set_class_name(name_str);
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(
            scope,
            s("getCurrentPosition").into(),
            build_fn("getCurrentPosition", geo_get_current_position).into(),
        );
        obj.set(
            scope,
            s("watchPosition").into(),
            build_fn("watchPosition", geo_watch_position).into(),
        );
        obj.set(
            scope,
            s("clearWatch").into(),
            build_fn("clearWatch", stub_noop).into(),
        );
        rv.set(obj.into());
    }));
}

// navigator.clipboard → object with Promise-returning methods
unsafe extern "C" fn nav_clipboard(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let build_promise_stub = |name: &str, cb: unsafe extern "C" fn(*const v8::FunctionCallbackInfo)| {
            let tmpl = v8::FunctionTemplate::builder_raw(cb).build(scope);
            let name_str = crate::v8_utils::v8_string(scope, name);
            tmpl.set_class_name(name_str);
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        // readText → Promise<String> (returns empty string)
        // writeText → Promise<void>
        // read → Promise<Array> (returns empty array)
        // write → Promise<void>
        obj.set(scope, s("readText").into(), build_promise_stub("readText", stub_promise_resolve).into());
        obj.set(scope, s("writeText").into(), build_promise_stub("writeText", stub_promise_resolve).into());
        obj.set(scope, s("read").into(), build_promise_stub("read", stub_promise_resolve_empty_array).into());
        obj.set(scope, s("write").into(), build_promise_stub("write", stub_promise_resolve).into());
        rv.set(obj.into());
    }));
}

// navigator.credentials → object with Promise-returning methods
unsafe extern "C" fn nav_credentials(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let build_promise_stub = |name: &str, cb: unsafe extern "C" fn(*const v8::FunctionCallbackInfo)| {
            let tmpl = v8::FunctionTemplate::builder_raw(cb).build(scope);
            let name_str = crate::v8_utils::v8_string(scope, name);
            tmpl.set_class_name(name_str);
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        // get → Promise<Credential|null> (resolves null — no credentials)
        // create → Promise<Credential|null> (resolves null)
        // store → Promise<Credential|null> (resolves null)
        // preventSilentAccess → Promise<void>
        obj.set(scope, s("get").into(), build_promise_stub("get", stub_promise_resolve_null).into());
        obj.set(scope, s("create").into(), build_promise_stub("create", stub_promise_resolve_null).into());
        obj.set(scope, s("store").into(), build_promise_stub("store", stub_promise_resolve_null).into());
        obj.set(scope, s("preventSilentAccess").into(), build_promise_stub("preventSilentAccess", stub_promise_resolve).into());
        rv.set(obj.into());
    }));
}

// ─── navigator.plugins / navigator.mimeTypes ──────────────────────────────────

unsafe extern "C" fn nav_mime_types(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
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
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
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
    let screen_tmpl = v8::FunctionTemplate::builder_raw(illegal_constructor).build(scope);
    screen_tmpl.set_class_name(crate::v8_utils::v8_string(scope, "Screen"));

    let proto = screen_tmpl.prototype_template(scope);

    macro_rules! screen_getter {
        ($name:literal, $cb:ident) => {
            let getter = v8::FunctionTemplate::builder_raw($cb).build(scope);
            let name = crate::v8_utils::v8_string(scope, $name);
            let get_name = crate::v8_utils::v8_string(scope, concat!("get ", $name));
            getter.set_class_name(get_name);
            getter.remove_prototype();
            proto.set_accessor_property(
                name.into(),
                Some(getter),
                None,
                v8::PropertyAttribute::NONE,
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
    screen_getter!("isExtended", screen_is_extended);
    screen_getter!("orientation", screen_orientation);
    screen_getter!("onchange", screen_onchange);

    let et_add = v8::FunctionTemplate::builder_raw(stub_noop).build(scope);
    let et_add_name = crate::v8_utils::v8_string(scope, "addEventListener");
    et_add.set_class_name(et_add_name);
    et_add.remove_prototype();
    proto.set(et_add_name.into(), et_add.into());

    let et_disp = v8::FunctionTemplate::builder_raw(screen_dispatch_event).build(scope);
    let et_disp_name = crate::v8_utils::v8_string(scope, "dispatchEvent");
    et_disp.set_class_name(et_disp_name);
    et_disp.remove_prototype();
    proto.set(et_disp_name.into(), et_disp.into());

    let et_rem = v8::FunctionTemplate::builder_raw(stub_noop).build(scope);
    let et_rem_name = crate::v8_utils::v8_string(scope, "removeEventListener");
    et_rem.set_class_name(et_rem_name);
    et_rem.remove_prototype();
    proto.set(et_rem_name.into(), et_rem.into());

    let when_fn = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
    let when_name = crate::v8_utils::v8_string(scope, "when");
    when_fn.set_class_name(when_name);
    when_fn.remove_prototype();
    proto.set(when_name.into(), when_fn.into());

    let inst_tmpl = screen_tmpl.instance_template(scope);
    if let Some(screen_obj) = inst_tmpl.new_instance(scope) {
        // Link screen_tmpl.prototype.__proto__ to install_all's Screen.prototype.
        if let Some(screen_func) = screen_tmpl.get_function(scope) {
            let proto_key = crate::v8_utils::v8_string(scope, "prototype");
            if let Some(screen_proto_val) = screen_func.get(scope, proto_key.into()) {
                if screen_proto_val.is_object() {
                    let screen_proto: v8::Local<v8::Object> =
                        unsafe { v8::Local::cast_unchecked(screen_proto_val) };
                    let screen_ctor_key = crate::v8_utils::v8_string(scope, "Screen");
                    if let Some(screen_ctor_val) = global.get(scope, screen_ctor_key.into()) {
                        if screen_ctor_val.is_function() {
                            let screen_ctor: v8::Local<v8::Function> =
                                unsafe { v8::Local::cast_unchecked(screen_ctor_val) };
                            if let Some(install_all_proto_val) = screen_ctor.get(scope, proto_key.into()) {
                                if install_all_proto_val.is_object() {
                                    let install_all_proto: v8::Local<v8::Object> =
                                        unsafe { v8::Local::cast_unchecked(install_all_proto_val) };
                                    let _ = screen_proto.set_prototype(scope, install_all_proto.into());
                                }
                            }
                        }
                    }
                }
            }
        }

        let key = crate::v8_utils::v8_string(scope, "screen");
        // Store screen instance on globalThis.__iv8ScreenInst__, then install
        // as accessor property
        let store_key = crate::v8_utils::v8_string(scope, "__iv8ScreenInst__");
        let _ = global.set(scope, store_key.into(), screen_obj.into());
        let getter_tmpl = v8::FunctionTemplate::builder_raw(screen_global_getter).build(scope);
        getter_tmpl.set_class_name(crate::v8_utils::v8_string(scope, "get screen"));
        getter_tmpl.remove_prototype();
        if let Some(getter_fn) = getter_tmpl.get_function(scope) {
            let mut desc = v8::PropertyDescriptor::new_from_get_set(
                getter_fn.into(),
                v8::undefined(scope).into(),
            );
            desc.set_enumerable(true);
            desc.set_configurable(true);
            let _ = global.define_property(scope, key.into(), &desc);
        }
    }

    // Overwrite global Screen constructor with illegal_constructor version.
    if let Some(func) = screen_tmpl.get_function(scope) {
        let ctor_key = crate::v8_utils::v8_string(scope, "Screen");
        let _ = global.define_own_property(
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
    DEFAULT_PROFILE.pdf_viewer_enabled,
    "Navigator"
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

unsafe extern "C" fn nav_storage_buckets(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let open_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
            tmpl.set_class_name(s("open"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("open").into(), open_fn.into());
        let ts = v8::Symbol::get_to_string_tag(scope);
        obj.set(scope, ts.into(), s("StorageBucketManager").into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_storage(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let estimate_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(storage_estimate_cb).build(scope);
            tmpl.set_class_name(s("estimate"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("estimate").into(), estimate_fn.into());
        obj.set(scope, s("persist").into(), {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
            tmpl.set_class_name(s("persist"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        }.into());
        obj.set(scope, s("persisted").into(), {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
            tmpl.set_class_name(s("persisted"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        }.into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn storage_estimate_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let global = scope.get_current_context().global(scope);
        if let Some(promise_ctor) = global.get(scope, v8_str(scope, "Promise").into()) {
            if promise_ctor.is_function() {
                let ctor: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(promise_ctor) };
                let resolve_key = v8_str(scope, "resolve");
                if let Some(resolve_fn) = ctor.get(scope, resolve_key.into()) {
                    if resolve_fn.is_function() {
                        let resolve: v8::Local<v8::Function> =
                            unsafe { v8::Local::cast_unchecked(resolve_fn) };
                        let result = v8::Object::new(scope);
                        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
                        result.set(scope, s("quota").into(), v8::Number::new(scope, 2997060666368.0).into());
                        result.set(scope, s("usage").into(), v8::Number::new(scope, 0.0).into());
                        let _undefined = v8::undefined(scope);
                        if let Some(promise) = resolve.call(scope, ctor.into(), &[result.into()]) {
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

unsafe extern "C" fn nav_locks(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let request_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
            tmpl.set_class_name(s("request"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("request").into(), request_fn.into());
        let query_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
            tmpl.set_class_name(s("query"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("query").into(), query_fn.into());
        let ts = v8::Symbol::get_to_string_tag(scope);
        obj.set(scope, ts.into(), s("LockManager").into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_gpu(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let request_adapter_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve_null).build(scope);
            tmpl.set_class_name(s("requestAdapter"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("requestAdapter").into(), request_adapter_fn.into());
        let ts = v8::Symbol::get_to_string_tag(scope);
        obj.set(scope, ts.into(), s("GPU").into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_media_capabilities(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let decoding_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
            tmpl.set_class_name(s("decodingInfo"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("decodingInfo").into(), decoding_fn.into());
        let encoding_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
            tmpl.set_class_name(s("encodingInfo"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("encodingInfo").into(), encoding_fn.into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_media_session(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        obj.set(scope, s("metadata").into(), v8::null(scope).into());
        obj.set(scope, s("playbackState").into(), s("none").into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_presentation(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        obj.set(scope, s("defaultRequest").into(), v8::null(scope).into());
        obj.set(scope, s("receiver").into(), v8::null(scope).into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_bluetooth(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let getavail_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
            tmpl.set_class_name(s("getAvailability"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("getAvailability").into(), getavail_fn.into());
        let request_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve_null).build(scope);
            tmpl.set_class_name(s("requestDevice"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("requestDevice").into(), request_fn.into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_usb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let getdevices_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve_empty_array).build(scope);
            tmpl.set_class_name(s("getDevices"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("getDevices").into(), getdevices_fn.into());
        let request_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve_null).build(scope);
            tmpl.set_class_name(s("requestDevice"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("requestDevice").into(), request_fn.into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_wake_lock(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let request_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve_null).build(scope);
            tmpl.set_class_name(s("request"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("request").into(), request_fn.into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_hid(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let getdevices_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve_empty_array).build(scope);
            tmpl.set_class_name(s("getDevices"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("getDevices").into(), getdevices_fn.into());
        let request_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve_null).build(scope);
            tmpl.set_class_name(s("requestDevice"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("requestDevice").into(), request_fn.into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_serial(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let getports_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve_empty_array).build(scope);
            tmpl.set_class_name(s("getPorts"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("getPorts").into(), getports_fn.into());
        let request_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve_null).build(scope);
            tmpl.set_class_name(s("requestPort"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("requestPort").into(), request_fn.into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_ink(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let request_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve_null).build(scope);
            tmpl.set_class_name(s("requestPresenter"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("requestPresenter").into(), request_fn.into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_xr(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let issession_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
            tmpl.set_class_name(s("isSessionSupported"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("isSessionSupported").into(), issession_fn.into());
        let request_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve_null).build(scope);
            tmpl.set_class_name(s("requestSession"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("requestSession").into(), request_fn.into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_keyboard(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let lock_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_promise_resolve).build(scope);
            tmpl.set_class_name(s("lock"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("lock").into(), lock_fn.into());
        let unlock_fn = {
            let tmpl = v8::FunctionTemplate::builder_raw(stub_noop).build(scope);
            tmpl.set_class_name(s("unlock"));
            tmpl.remove_prototype();
            crate::v8_utils::v8_fn(scope, &tmpl)
        };
        obj.set(scope, s("unlock").into(), unlock_fn.into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_managed(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_taint_enabled(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::Boolean::new(scope, false).into());
    }));
}

unsafe extern "C" fn nav_build_id(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::undefined(scope).into());
    }));
}

unsafe extern "C" fn nav_get_user_media(info: *const v8::FunctionCallbackInfo) {
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
                        let err_obj = build_dom_exception(scope, &global, "Permission denied", "NotAllowedError");
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

unsafe extern "C" fn nav_vibrate(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::Boolean::new(scope, false).into());
    }));
}

env_f64_getter!(
    screen_width,
    "screen.width",
    screen_width,
    DEFAULT_PROFILE.screen_width,
    "Screen"
);
env_f64_getter!(
    screen_height,
    "screen.height",
    screen_height,
    DEFAULT_PROFILE.screen_height,
    "Screen"
);
env_f64_getter!(
    screen_avail_width,
    "screen.availWidth",
    screen_avail_width,
    DEFAULT_PROFILE.screen_avail_width,
    "Screen"
);
env_f64_getter!(
    screen_avail_height,
    "screen.availHeight",
    screen_avail_height,
    DEFAULT_PROFILE.screen_avail_height,
    "Screen"
);
env_f64_getter!(
    screen_color_depth,
    "screen.colorDepth",
    screen_color_depth,
    DEFAULT_PROFILE.screen_color_depth,
    "Screen"
);
env_f64_getter!(
    screen_pixel_depth,
    "screen.pixelDepth",
    screen_pixel_depth,
    DEFAULT_PROFILE.screen_pixel_depth,
    "Screen"
);
env_f64_getter!(
    screen_avail_left,
    "screen.availLeft",
    screen_avail_left,
    DEFAULT_PROFILE.screen_avail_left,
    "Screen"
);
env_f64_getter!(
    screen_avail_top,
    "screen.availTop",
    screen_avail_top,
    DEFAULT_PROFILE.screen_avail_top,
    "Screen"
);

unsafe extern "C" fn screen_is_extended(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Screen") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::Boolean::new(scope, false).into());
    }));
}

unsafe extern "C" fn screen_orientation(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Screen") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let env = &state.environment;
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        let otype = env.get_str("screen.orientation.type").unwrap_or("landscape-primary");
        obj.set(scope, s("type").into(), s(otype).into());
        let angle = env.get_f64("screen.orientation.angle").unwrap_or(0.0);
        obj.set(scope, s("angle").into(), v8::Number::new(scope, angle).into());
        obj.set(scope, s("onchange").into(), v8::null(scope).into());
        let ts = v8::Symbol::get_to_string_tag(scope);
        obj.set(scope, ts.into(), s("ScreenOrientation").into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn screen_onchange(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Screen") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::null(scope).into());
    }));
}

unsafe extern "C" fn nav_protected_audience(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_webkit_persistent_storage(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        obj.set(scope, s("queryUsageAndQuota").into(), v8::null(scope).into());
        obj.set(scope, s("requestQuota").into(), v8::null(scope).into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_webkit_temporary_storage(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Navigator") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let obj = v8::Object::new(scope);
        let s = |k: &str| crate::v8_utils::v8_string(scope, k);
        obj.set(scope, s("queryUsageAndQuota").into(), v8::null(scope).into());
        obj.set(scope, s("requestQuota").into(), v8::null(scope).into());
        rv.set(obj.into());
    }));
}

unsafe extern "C" fn nav_create_auction_nonce(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let s = crate::v8_utils::v8_string(scope, "");
        rv.set(s.into());
    }));
}

unsafe extern "C" fn nav_can_load_ad_auction_fenced_frame(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::Boolean::new(scope, false).into());
    }));
}

unsafe extern "C" fn screen_dispatch_event(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !check_receiver(&scope, info_ref, "Screen") {
            return;
        }
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::Boolean::new(scope, true).into());
    }));
}
