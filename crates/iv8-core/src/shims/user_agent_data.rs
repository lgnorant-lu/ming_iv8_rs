//! NavigatorUAData implementation (navigator.userAgentData).
//!
//! Provides the Client Hints JavaScript API:
//! - brands / mobile / platform on **NavigatorUAData.prototype**
//! - getHighEntropyValues(hints) / toJSON on prototype
//! - instanceof NavigatorUAData
//!
//! Brand list priority (Q030): profile → environment → DEFAULT_PROFILE
//! (all sources should share GREASE-aware Chrome-like brand lists).

use crate::shims::browser_profile::DEFAULT_PROFILE;
use crate::shims::native_env::illegal_constructor;
use crate::state::RuntimeState;

/// Install the userAgentData object on the navigator instance.
pub fn install_user_agent_data(scope: &v8::PinScope<'_, '_>, navigator: v8::Local<v8::Object>) {
    // NavigatorUAData constructor (illegal) + prototype with accessors/methods
    let uad_tmpl = v8::FunctionTemplate::builder_raw(illegal_constructor).build(scope);
    uad_tmpl.set_class_name(crate::v8_utils::v8_string(scope, "NavigatorUAData"));

    let proto = uad_tmpl.prototype_template(scope);

    // brands / mobile / platform accessors on prototype
    {
        let g = v8::FunctionTemplate::builder_raw(uad_brands_getter).build(scope);
        g.set_class_name(crate::v8_utils::v8_string(scope, "get brands"));
        g.remove_prototype();
        proto.set_accessor_property(
            crate::v8_utils::v8_string(scope, "brands").into(),
            Some(g),
            None,
            v8::PropertyAttribute::NONE,
        );
    }
    {
        let g = v8::FunctionTemplate::builder_raw(uad_mobile_getter).build(scope);
        g.set_class_name(crate::v8_utils::v8_string(scope, "get mobile"));
        g.remove_prototype();
        proto.set_accessor_property(
            crate::v8_utils::v8_string(scope, "mobile").into(),
            Some(g),
            None,
            v8::PropertyAttribute::NONE,
        );
    }
    {
        let g = v8::FunctionTemplate::builder_raw(uad_platform_getter).build(scope);
        g.set_class_name(crate::v8_utils::v8_string(scope, "get platform"));
        g.remove_prototype();
        proto.set_accessor_property(
            crate::v8_utils::v8_string(scope, "platform").into(),
            Some(g),
            None,
            v8::PropertyAttribute::NONE,
        );
    }
    {
        let m = v8::FunctionTemplate::builder_raw(uad_get_high_entropy_values)
            .length(1)
            .build(scope);
        m.set_class_name(crate::v8_utils::v8_string(scope, "getHighEntropyValues"));
        m.remove_prototype();
        proto.set(
            crate::v8_utils::v8_string(scope, "getHighEntropyValues").into(),
            m.into(),
        );
    }
    {
        let m = v8::FunctionTemplate::builder_raw(uad_to_json).length(0).build(scope);
        m.set_class_name(crate::v8_utils::v8_string(scope, "toJSON"));
        m.remove_prototype();
        proto.set(crate::v8_utils::v8_string(scope, "toJSON").into(), m.into());
    }
    // toStringTag on prototype
    {
        let tag_sym = v8::Symbol::get_to_string_tag(scope);
        let tag_val = crate::v8_utils::v8_string(scope, "NavigatorUAData");
        // store on instance after create; also try proto via JS after
        let _ = (tag_sym, tag_val);
    }

    let uad_ctor = crate::v8_utils::v8_fn(scope, &uad_tmpl);
    // Install global NavigatorUAData
    let global = scope.get_current_context().global(scope);
    let ctor_key = crate::v8_utils::v8_string(scope, "NavigatorUAData");
    let mut ctor_desc = v8::PropertyDescriptor::new_from_value_writable(uad_ctor.into(), true);
    ctor_desc.set_enumerable(false);
    ctor_desc.set_configurable(true);
    let _ = global.define_property(scope, ctor_key.into(), &ctor_desc);

    // Instance via Object.create(NavigatorUAData.prototype)
    let uad_obj = uad_tmpl
        .instance_template(scope)
        .new_instance(scope)
        .unwrap_or_else(|| v8::Object::new(scope));
    // Ensure prototype chain: if new_instance didn't wire, set manually
    if let Some(ctor_fn) = global.get(scope, crate::v8_utils::v8_string(scope, "NavigatorUAData").into())
    {
        if ctor_fn.is_function() {
            let ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(ctor_fn) };
            let pk = crate::v8_utils::v8_string(scope, "prototype");
            if let Some(p) = ctor.get(scope, pk.into()) {
                if p.is_object() {
                    let po: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(p) };
                    let _ = uad_obj.set_prototype(scope, po.into());
                }
            }
        }
    }
    let tag_key = v8::Symbol::get_to_string_tag(scope);
    let tag_val = crate::v8_utils::v8_string(scope, "NavigatorUAData");
    let _ = uad_obj.set(scope, tag_key.into(), tag_val.into());

    // Install on navigator as non-enumerable readonly own property (Chrome: accessor on proto;
    // we keep own RO data for now but value is real NavigatorUAData instance).
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

fn brands_json_from_state(state: &RuntimeState) -> &str {
    // Q030: prefer env brands when present (profile JSON inject), then profile, then DEFAULT.
    // Ensures GREASE brand from default_chrome147.json is not dropped when profile is unset.
    if let Some(s) = state.environment.get_str("navigator.userAgentData.brands") {
        return s;
    }
    if let Some(p) = state.profile {
        return p.ua_brands_json;
    }
    DEFAULT_PROFILE.ua_brands_json
}

// --- Getter callbacks ---

unsafe extern "C" fn uad_brands_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let brands_json = brands_json_from_state(state);

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
            rv.set(v8::Array::new(scope, 0).into());
        }
    }));
}

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

unsafe extern "C" fn uad_get_high_entropy_values(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let result = v8::Object::new(scope);
        let brands_json = brands_json_from_state(state);
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

        // High-entropy fields from profile when requested — keep prior behavior:
        // architecture / bitness / model / platformVersion / wow64 / fullVersionList
        // (hints argument parsing simplified: always fill profile high-entropy)
        let arch = state
            .profile
            .map(|p| p.ua_architecture)
            .unwrap_or(DEFAULT_PROFILE.ua_architecture);
        let bitness = state
            .profile
            .map(|p| p.ua_bitness)
            .unwrap_or(DEFAULT_PROFILE.ua_bitness);
        let model = state
            .profile
            .map(|p| p.ua_model)
            .unwrap_or(DEFAULT_PROFILE.ua_model);
        let plat_ver = state
            .profile
            .map(|p| p.ua_platform_version)
            .unwrap_or(DEFAULT_PROFILE.ua_platform_version);
        let wow64 = state
            .profile
            .map(|p| p.ua_wow64)
            .unwrap_or(DEFAULT_PROFILE.ua_wow64);
        if let Some(s) = v8::String::new(scope, arch) {
            result.set(
                scope,
                crate::v8_utils::v8_string(scope, "architecture").into(),
                s.into(),
            );
        }
        if let Some(s) = v8::String::new(scope, bitness) {
            result.set(
                scope,
                crate::v8_utils::v8_string(scope, "bitness").into(),
                s.into(),
            );
        }
        if let Some(s) = v8::String::new(scope, model) {
            result.set(
                scope,
                crate::v8_utils::v8_string(scope, "model").into(),
                s.into(),
            );
        }
        if let Some(s) = v8::String::new(scope, plat_ver) {
            result.set(
                scope,
                crate::v8_utils::v8_string(scope, "platformVersion").into(),
                s.into(),
            );
        }
        result.set(
            scope,
            crate::v8_utils::v8_string(scope, "wow64").into(),
            v8::Boolean::new(scope, wow64).into(),
        );

        let resolver = crate::v8_utils::v8_resolver(scope);
        resolver.resolve(scope, result.into());
        rv.set(resolver.get_promise(scope).into());
    }));
}

unsafe extern "C" fn uad_to_json(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let result = v8::Object::new(scope);
        let brands_json = brands_json_from_state(state);
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

        rv.set(result.into());
    }));
}
