//! NavigatorUAData implementation (navigator.userAgentData).
//!
//! Provides the Client Hints JavaScript API:
//! - brands (array of {brand, version})
//! - mobile (bool)
//! - platform (string)
//! - getHighEntropyValues(hints) -> Promise
//! - toJSON() -> {brands, mobile, platform}
//!
//! All values are read from the environment configuration at access time.
//! The brands array is stored as a JSON string in the environment
//! (key: "navigator.userAgentData.brands").

use crate::shims::browser_profile::DEFAULT_PROFILE;
use crate::state::RuntimeState;

/// Install the userAgentData object on the navigator instance.
pub fn install_user_agent_data(scope: &v8::PinScope<'_, '_>, navigator: v8::Local<v8::Object>) {
    let uad_obj = v8::Object::new(scope);

    // brands getter
    let brands_tmpl = v8::FunctionTemplate::builder_raw(uad_brands_getter).build(scope);
    let brands_name = crate::v8_utils::v8_string(scope, "brands");
    brands_tmpl.set_class_name(brands_name);
    brands_tmpl.remove_prototype();

    // mobile getter
    let mobile_tmpl = v8::FunctionTemplate::builder_raw(uad_mobile_getter).build(scope);
    let mobile_name = crate::v8_utils::v8_string(scope, "mobile");
    mobile_tmpl.set_class_name(mobile_name);
    mobile_tmpl.remove_prototype();

    // platform getter
    let platform_tmpl = v8::FunctionTemplate::builder_raw(uad_platform_getter).build(scope);
    let platform_name = crate::v8_utils::v8_string(scope, "platform");
    platform_tmpl.set_class_name(platform_name);
    platform_tmpl.remove_prototype();

    // Install as accessor properties on uad_obj
    // Use defineProperty with getter descriptor
    install_getter(scope, uad_obj, "brands", brands_tmpl);
    install_getter(scope, uad_obj, "mobile", mobile_tmpl);
    install_getter(scope, uad_obj, "platform", platform_tmpl);

    // getHighEntropyValues method
    let ghev_tmpl = v8::FunctionTemplate::builder_raw(uad_get_high_entropy_values).build(scope);
    let ghev_name = crate::v8_utils::v8_string(scope, "getHighEntropyValues");
    ghev_tmpl.set_class_name(ghev_name);
    ghev_tmpl.remove_prototype();
    let ghev_fn = crate::v8_utils::v8_fn(scope, &ghev_tmpl);
    uad_obj.set(scope, ghev_name.into(), ghev_fn.into());

    // toJSON method
    let to_json_tmpl = v8::FunctionTemplate::builder_raw(uad_to_json).build(scope);
    let to_json_name = crate::v8_utils::v8_string(scope, "toJSON");
    to_json_tmpl.set_class_name(to_json_name);
    to_json_tmpl.remove_prototype();
    let to_json_fn = crate::v8_utils::v8_fn(scope, &to_json_tmpl);
    uad_obj.set(scope, to_json_name.into(), to_json_fn.into());

    // Set Symbol.toStringTag = "NavigatorUAData"
    let tag_key = v8::Symbol::get_to_string_tag(scope);
    let tag_val = crate::v8_utils::v8_string(scope, "NavigatorUAData");
    uad_obj.set(scope, tag_key.into(), tag_val.into());

    // Install on navigator
    let uad_key = crate::v8_utils::v8_string(scope, "userAgentData");
    navigator.define_own_property(
        scope,
        uad_key.into(),
        uad_obj.into(),
        v8::PropertyAttribute::DONT_DELETE
            | v8::PropertyAttribute::DONT_ENUM
            | v8::PropertyAttribute::READ_ONLY,
    );
}

/// Helper: install a getter-only accessor property via defineProperty.
fn install_getter(
    scope: &v8::PinScope<'_, '_>,
    obj: v8::Local<v8::Object>,
    name: &str,
    getter_tmpl: v8::Local<v8::FunctionTemplate>,
) {
    let getter_fn = crate::v8_utils::v8_fn(scope, &getter_tmpl);
    let name_str = crate::v8_utils::v8_string(scope, name);
    let desc = v8::Object::new(scope);
    let get_key = crate::v8_utils::v8_string(scope, "get");
    let enum_key = crate::v8_utils::v8_string(scope, "enumerable");
    let conf_key = crate::v8_utils::v8_string(scope, "configurable");
    desc.set(scope, get_key.into(), getter_fn.into());
    desc.set(
        scope,
        enum_key.into(),
        v8::Boolean::new(scope, false).into(),
    );
    desc.set(scope, conf_key.into(), v8::Boolean::new(scope, true).into());

    // Call Object.defineProperty(obj, name, desc)
    let global = scope.get_current_context().global(scope);
    let object_key = crate::v8_utils::v8_string(scope, "Object");
    if let Some(object_val) = global.get(scope, object_key.into()) {
        if object_val.is_object() {
            let object_obj: v8::Local<v8::Object> =
                unsafe { v8::Local::cast_unchecked(object_val) };
            let def_prop_key = crate::v8_utils::v8_string(scope, "defineProperty");
            if let Some(def_prop) = object_obj.get(scope, def_prop_key.into()) {
                if def_prop.is_function() {
                    let def_prop_fn: v8::Local<v8::Function> =
                        unsafe { v8::Local::cast_unchecked(def_prop) };
                    let undefined = v8::undefined(scope);
                    def_prop_fn.call(
                        scope,
                        undefined.into(),
                        &[obj.into(), name_str.into(), desc.into()],
                    );
                }
            }
        }
    }
}

// --- Getter callbacks ---

/// brands getter: parse JSON string from environment into V8 array of objects.
unsafe extern "C" fn uad_brands_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let brands_json = state
            .profile
            .map(|p| p.ua_brands_json)
            .or_else(|| state.environment.get_str("navigator.userAgentData.brands"))
            .unwrap_or(DEFAULT_PROFILE.ua_brands_json);

        // Parse JSON and build V8 array
        if let Ok(parsed) = serde_json::from_str::<Vec<serde_json::Value>>(brands_json) {
            let arr = v8::Array::new(scope, parsed.len() as i32);
            for (i, brand_val) in parsed.iter().enumerate() {
                let obj = v8::Object::new(scope);
                if let Some(brand) = brand_val.get("brand").and_then(|v| v.as_str()) {
                    let k = crate::v8_utils::v8_string(scope, "brand");
                    let v = crate::v8_utils::v8_string(scope, brand);
                    obj.set(scope, k.into(), v.into());
                }
                if let Some(version) = brand_val.get("version").and_then(|v| v.as_str()) {
                    let k = crate::v8_utils::v8_string(scope, "version");
                    let v = crate::v8_utils::v8_string(scope, version);
                    obj.set(scope, k.into(), v.into());
                }
                arr.set_index(scope, i as u32, obj.into());
            }
            rv.set(arr.into());
        } else {
            // Fallback: empty array
            rv.set(v8::Array::new(scope, 0).into());
        }
    }));
}

/// mobile getter: read bool from environment.
unsafe extern "C" fn uad_mobile_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let mobile = state
            .profile
            .map(|p| p.ua_mobile)
            .or_else(|| state.environment.get_bool("navigator.userAgentData.mobile"))
            .unwrap_or(DEFAULT_PROFILE.ua_mobile);
        rv.set(v8::Boolean::new(scope, mobile).into());
    }));
}

/// platform getter: read string from environment or profile.
unsafe extern "C" fn uad_platform_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let platform = state
            .profile
            .map(|p| p.ua_platform)
            .or_else(|| {
                state
                    .environment
                    .get_str("navigator.userAgentData.platform")
            })
            .unwrap_or(DEFAULT_PROFILE.ua_platform);
        if let Some(s) = v8::String::new(scope, platform) {
            rv.set(s.into());
        }
    }));
}

/// getHighEntropyValues(hints): returns a resolved Promise with requested fields.
unsafe extern "C" fn uad_get_high_entropy_values(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        // Build result object with requested hints
        let result = v8::Object::new(scope);

        // Always include brands, mobile, platform (low entropy)
        // Parse brands
        let brands_json = state
            .profile
            .map(|p| p.ua_brands_json)
            .or_else(|| state.environment.get_str("navigator.userAgentData.brands"))
            .unwrap_or(DEFAULT_PROFILE.ua_brands_json);
        if let Ok(parsed) = serde_json::from_str::<Vec<serde_json::Value>>(brands_json) {
            let arr = v8::Array::new(scope, parsed.len() as i32);
            for (i, brand_val) in parsed.iter().enumerate() {
                let obj = v8::Object::new(scope);
                if let Some(brand) = brand_val.get("brand").and_then(|v| v.as_str()) {
                    let k = crate::v8_utils::v8_string(scope, "brand");
                    let v = crate::v8_utils::v8_string(scope, brand);
                    obj.set(scope, k.into(), v.into());
                }
                if let Some(version) = brand_val.get("version").and_then(|v| v.as_str()) {
                    let k = crate::v8_utils::v8_string(scope, "version");
                    let v = crate::v8_utils::v8_string(scope, version);
                    obj.set(scope, k.into(), v.into());
                }
                arr.set_index(scope, i as u32, obj.into());
            }
            let brands_key = crate::v8_utils::v8_string(scope, "brands");
            result.set(scope, brands_key.into(), arr.into());
        }

        let mobile = state
            .profile
            .map(|p| p.ua_mobile)
            .or_else(|| state.environment.get_bool("navigator.userAgentData.mobile"))
            .unwrap_or(DEFAULT_PROFILE.ua_mobile);
        let mobile_key = crate::v8_utils::v8_string(scope, "mobile");
        result.set(
            scope,
            mobile_key.into(),
            v8::Boolean::new(scope, mobile).into(),
        );

        // Align with uad_platform_getter: profile > env > DEFAULT (v0.8.93 B)
        let platform = state
            .profile
            .map(|p| p.ua_platform)
            .or_else(|| {
                state
                    .environment
                    .get_str("navigator.userAgentData.platform")
            })
            .unwrap_or(DEFAULT_PROFILE.ua_platform);
        let platform_key = crate::v8_utils::v8_string(scope, "platform");
        if let Some(s) = v8::String::new(scope, platform) {
            result.set(scope, platform_key.into(), s.into());
        }

        // High entropy fields (only if requested in hints array)
        if args.length() > 0 && args.get(0).is_object() {
            let hints_val = args.get(0);
            // Try to iterate the hints array
            if let Ok(hints_arr) = v8::Local::<v8::Array>::try_from(hints_val) {
                let len = hints_arr.length();
                for i in 0..len {
                    if let Some(hint_val) = hints_arr.get_index(scope, i) {
                        let hint = hint_val.to_rust_string_lossy(scope);
                        match hint.as_str() {
                            "architecture" => {
                                let val = state
                                    .profile
                                    .map(|p| p.ua_architecture)
                                    .or_else(|| {
                                        state
                                            .environment
                                            .get_str("navigator.userAgentData.architecture")
                                    })
                                    .unwrap_or(DEFAULT_PROFILE.ua_architecture);
                                let k = crate::v8_utils::v8_string(scope, "architecture");
                                let v = crate::v8_utils::v8_string(scope, val);
                                result.set(scope, k.into(), v.into());
                            }
                            "bitness" => {
                                let val = state
                                    .profile
                                    .map(|p| p.ua_bitness)
                                    .or_else(|| {
                                        state.environment.get_str("navigator.userAgentData.bitness")
                                    })
                                    .unwrap_or(DEFAULT_PROFILE.ua_bitness);
                                let k = crate::v8_utils::v8_string(scope, "bitness");
                                let v = crate::v8_utils::v8_string(scope, val);
                                result.set(scope, k.into(), v.into());
                            }
                            "model" => {
                                let val = state
                                    .profile
                                    .map(|p| p.ua_model)
                                    .or_else(|| {
                                        state.environment.get_str("navigator.userAgentData.model")
                                    })
                                    .unwrap_or(DEFAULT_PROFILE.ua_model);
                                let k = crate::v8_utils::v8_string(scope, "model");
                                let v = crate::v8_utils::v8_string(scope, val);
                                result.set(scope, k.into(), v.into());
                            }
                            "platformVersion" => {
                                let val = state
                                    .profile
                                    .map(|p| p.ua_platform_version)
                                    .or_else(|| {
                                        state
                                            .environment
                                            .get_str("navigator.userAgentData.platformVersion")
                                    })
                                    .unwrap_or(DEFAULT_PROFILE.ua_platform_version);
                                let k = crate::v8_utils::v8_string(scope, "platformVersion");
                                let v = crate::v8_utils::v8_string(scope, val);
                                result.set(scope, k.into(), v.into());
                            }
                            "wow64" => {
                                let val = state
                                    .profile
                                    .map(|p| p.ua_wow64)
                                    .or_else(|| {
                                        state.environment.get_bool("navigator.userAgentData.wow64")
                                    })
                                    .unwrap_or(DEFAULT_PROFILE.ua_wow64);
                                let k = crate::v8_utils::v8_string(scope, "wow64");
                                result.set(scope, k.into(), v8::Boolean::new(scope, val).into());
                            }
                            "fullVersionList" => {
                                // Same format as brands but with full version numbers
                                let fvl_json = state
                                    .profile
                                    .map(|p| p.ua_full_version_list_json)
                                    .or_else(|| {
                                        state
                                            .environment
                                            .get_str("navigator.userAgentData.fullVersionList")
                                    })
                                    .unwrap_or(DEFAULT_PROFILE.ua_full_version_list_json);
                                if let Ok(parsed) =
                                    serde_json::from_str::<Vec<serde_json::Value>>(fvl_json)
                                {
                                    let arr = v8::Array::new(scope, parsed.len() as i32);
                                    for (j, brand_val) in parsed.iter().enumerate() {
                                        let obj = v8::Object::new(scope);
                                        if let Some(brand) =
                                            brand_val.get("brand").and_then(|v| v.as_str())
                                        {
                                            let bk = crate::v8_utils::v8_string(scope, "brand");
                                            let bv = crate::v8_utils::v8_string(scope, brand);
                                            obj.set(scope, bk.into(), bv.into());
                                        }
                                        if let Some(version) =
                                            brand_val.get("version").and_then(|v| v.as_str())
                                        {
                                            let vk = crate::v8_utils::v8_string(scope, "version");
                                            let vv = crate::v8_utils::v8_string(scope, version);
                                            obj.set(scope, vk.into(), vv.into());
                                        }
                                        arr.set_index(scope, j as u32, obj.into());
                                    }
                                    let k = crate::v8_utils::v8_string(scope, "fullVersionList");
                                    result.set(scope, k.into(), arr.into());
                                }
                            }
                            _ => {} // Unknown hint, ignore
                        }
                    }
                }
            }
        }

        // Return a resolved Promise
        let resolver = crate::v8_utils::v8_resolver(scope);
        resolver.resolve(scope, result.into());
        rv.set(resolver.get_promise(scope).into());
    }));
}

/// toJSON(): returns {brands, mobile, platform} as a plain object.
unsafe extern "C" fn uad_to_json(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let result = v8::Object::new(scope);

        // brands
        let brands_json = state
            .profile
            .map(|p| p.ua_brands_json)
            .or_else(|| state.environment.get_str("navigator.userAgentData.brands"))
            .unwrap_or(DEFAULT_PROFILE.ua_brands_json);
        if let Ok(parsed) = serde_json::from_str::<Vec<serde_json::Value>>(brands_json) {
            let arr = v8::Array::new(scope, parsed.len() as i32);
            for (i, brand_val) in parsed.iter().enumerate() {
                let obj = v8::Object::new(scope);
                if let Some(brand) = brand_val.get("brand").and_then(|v| v.as_str()) {
                    let k = crate::v8_utils::v8_string(scope, "brand");
                    let v = crate::v8_utils::v8_string(scope, brand);
                    obj.set(scope, k.into(), v.into());
                }
                if let Some(version) = brand_val.get("version").and_then(|v| v.as_str()) {
                    let k = crate::v8_utils::v8_string(scope, "version");
                    let v = crate::v8_utils::v8_string(scope, version);
                    obj.set(scope, k.into(), v.into());
                }
                arr.set_index(scope, i as u32, obj.into());
            }
            let brands_key = crate::v8_utils::v8_string(scope, "brands");
            result.set(scope, brands_key.into(), arr.into());
        }

        // mobile
        let mobile = state
            .environment
            .get_bool("navigator.userAgentData.mobile")
            .unwrap_or(false);
        let mobile_key = crate::v8_utils::v8_string(scope, "mobile");
        result.set(
            scope,
            mobile_key.into(),
            v8::Boolean::new(scope, mobile).into(),
        );

        // platform
        let platform = state
            .environment
            .get_str("navigator.userAgentData.platform")
            .unwrap_or("Windows");
        let platform_key = crate::v8_utils::v8_string(scope, "platform");
        if let Some(s) = v8::String::new(scope, platform) {
            result.set(scope, platform_key.into(), s.into());
        }

        rv.set(result.into());
    }));
}
