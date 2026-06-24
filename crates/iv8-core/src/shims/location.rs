//! location object: window.location with native accessor properties.
//!
//! Uses a FunctionTemplate so that Object.getOwnPropertyDescriptor returns
//! { get: f, set: f } shapes matching real browser behavior.

use crate::state::RuntimeState;

pub fn install_location(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    let loc_tmpl = v8::FunctionTemplate::builder_raw(illegal_constructor).build(scope);
    loc_tmpl.set_class_name(crate::v8_utils::v8_string(scope, "Location"));
    loc_tmpl
        .instance_template(scope)
        .set_internal_field_count(1);

    macro_rules! loc_accessor {
        ($name:literal, $getter:ident, $setter:ident) => {
            let getter = v8::FunctionTemplate::builder_raw($getter).build(scope);
            getter.set_class_name(crate::v8_utils::v8_string(scope, $name));
            getter.remove_prototype();
            let setter = v8::FunctionTemplate::builder_raw($setter).build(scope);
            setter.set_class_name(crate::v8_utils::v8_string(scope, $name));
            setter.remove_prototype();
            let name = crate::v8_utils::v8_string(scope, $name);
            loc_tmpl.prototype_template(scope).set_accessor_property(
                name.into(),
                Some(getter),
                Some(setter),
                v8::PropertyAttribute::DONT_DELETE | v8::PropertyAttribute::DONT_ENUM,
            );
        };
        ($name:literal, $getter:ident) => {
            let getter = v8::FunctionTemplate::builder_raw($getter).build(scope);
            getter.set_class_name(crate::v8_utils::v8_string(scope, $name));
            getter.remove_prototype();
            let name = crate::v8_utils::v8_string(scope, $name);
            loc_tmpl.prototype_template(scope).set_accessor_property(
                name.into(),
                Some(getter),
                None,
                v8::PropertyAttribute::DONT_DELETE | v8::PropertyAttribute::DONT_ENUM,
            );
        };
    }

    loc_accessor!("href", loc_href_getter, loc_href_setter);
    loc_accessor!("origin", loc_origin_getter);
    loc_accessor!("protocol", loc_protocol_getter, loc_protocol_setter);
    loc_accessor!("host", loc_host_getter, loc_host_setter);
    loc_accessor!("hostname", loc_hostname_getter, loc_hostname_setter);
    loc_accessor!("port", loc_port_getter, loc_port_setter);
    loc_accessor!("pathname", loc_pathname_getter, loc_pathname_setter);
    loc_accessor!("search", loc_search_getter, loc_search_setter);
    loc_accessor!("hash", loc_hash_getter, loc_hash_setter);

    // toString / valueOf → href
    let ts_tmpl = v8::FunctionTemplate::builder_raw(loc_to_string).build(scope);
    loc_tmpl.prototype_template(scope).set(
        crate::v8_utils::v8_string(scope, "toString").into(),
        ts_tmpl.into(),
    );
    let ts_tmpl2 = v8::FunctionTemplate::builder_raw(loc_to_string).build(scope);
    loc_tmpl.prototype_template(scope).set(
        crate::v8_utils::v8_string(scope, "valueOf").into(),
        ts_tmpl2.into(),
    );

    // assign/replace/reload → no-op
    for name in &["assign", "replace", "reload"] {
        let noop = v8::FunctionTemplate::builder_raw(noop_callback).build(scope);
        loc_tmpl
            .prototype_template(scope)
            .set(crate::v8_utils::v8_string(scope, name).into(), noop.into());
    }

    let obj = loc_tmpl
        .get_function(scope)
        .expect("Location")
        .new_instance(scope, &[])
        .expect("Location instance");

    // v0.8.78: Link loc_tmpl.prototype.__proto__ to install_all's
    // Location.prototype (created by the surface codegen). This makes
    // `location instanceof Location` and `location instanceof EventTarget`
    // evaluate to true, matching real browser prototype chains.
    if let Some(loc_func) = loc_tmpl.get_function(scope) {
        let proto_key = crate::v8_utils::v8_string(scope, "prototype");
        if let Some(loc_proto_val) = loc_func.get(scope, proto_key.into()) {
            if loc_proto_val.is_object() {
                let loc_proto: v8::Local<v8::Object> =
                    unsafe { v8::Local::cast_unchecked(loc_proto_val) };
                let loc_ctor_key = crate::v8_utils::v8_string(scope, "Location");
                if let Some(loc_ctor_val) = global.get(scope, loc_ctor_key.into()) {
                    if loc_ctor_val.is_function() {
                        let loc_ctor: v8::Local<v8::Function> =
                            unsafe { v8::Local::cast_unchecked(loc_ctor_val) };
                        if let Some(install_all_proto_val) = loc_ctor.get(scope, proto_key.into()) {
                            if install_all_proto_val.is_object() {
                                let install_all_proto: v8::Local<v8::Object> =
                                    unsafe { v8::Local::cast_unchecked(install_all_proto_val) };
                                let _ = loc_proto.set_prototype(scope, install_all_proto.into());

                                // Re-install accessor getters on install_all's
                                // Location.prototype via Object.defineProperty so
                                // that the getters are reachable through the linked
                                // prototype chain.
                                install_loc_getters_on_proto(scope, global, install_all_proto);
                            }
                        }
                    }
                }
            }
        }
    }

    global.set(
        scope,
        crate::v8_utils::v8_string(scope, "location").into(),
        obj.into(),
    );

    // Overwrite global Location constructor with illegal_constructor,
    // but preserve install_all's Location.prototype (which has EventTarget
    // inheritance + our native getters).
    let loc_ctor_tmpl = v8::FunctionTemplate::builder_raw(illegal_constructor).build(scope);
    loc_ctor_tmpl.set_class_name(crate::v8_utils::v8_string(scope, "Location"));
    loc_ctor_tmpl.remove_prototype();
    if let Some(loc_ctor_fn) = loc_ctor_tmpl.get_function(scope) {
        let proto_key = crate::v8_utils::v8_string(scope, "prototype");
        let loc_ctor_key = crate::v8_utils::v8_string(scope, "Location");
        if let Some(install_all_loc_ctor_val) = global.get(scope, loc_ctor_key.into()) {
            if install_all_loc_ctor_val.is_function() {
                let install_all_loc_ctor: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(install_all_loc_ctor_val) };
                if let Some(install_all_proto_val) =
                    install_all_loc_ctor.get(scope, proto_key.into())
                {
                    let _ = loc_ctor_fn.set(scope, proto_key.into(), install_all_proto_val);
                }
            }
        }
        let _ = global.define_own_property(
            scope,
            loc_ctor_key.into(),
            loc_ctor_fn.into(),
            v8::PropertyAttribute::DONT_ENUM,
        );
    }
}

unsafe extern "C" fn illegal_constructor(_info: *const v8::FunctionCallbackInfo) {}

/// Install accessor getter/setter pairs on an already-instantiated prototype
/// object using `Object.defineProperty`, mirroring the loc_accessor! macro but
/// for install_all's Location.prototype (so getters are reachable through the
/// linked prototype chain).
fn install_loc_getters_on_proto(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
    proto: v8::Local<v8::Object>,
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
    let set_key = crate::v8_utils::v8_string(scope, "set");
    let enum_key = crate::v8_utils::v8_string(scope, "enumerable");
    let config_key = crate::v8_utils::v8_string(scope, "configurable");

    // (name, getter, setter) — setter is None for read-only accessors
    let accessors: &[(&str, v8::FunctionCallback, Option<v8::FunctionCallback>)] = &[
        ("href", loc_href_getter, Some(loc_href_setter)),
        ("origin", loc_origin_getter, None),
        ("protocol", loc_protocol_getter, Some(loc_protocol_setter)),
        ("host", loc_host_getter, Some(loc_host_setter)),
        ("hostname", loc_hostname_getter, Some(loc_hostname_setter)),
        ("port", loc_port_getter, Some(loc_port_setter)),
        ("pathname", loc_pathname_getter, Some(loc_pathname_setter)),
        ("search", loc_search_getter, Some(loc_search_setter)),
        ("hash", loc_hash_getter, Some(loc_hash_setter)),
    ];

    for (name, getter_cb, setter_opt) in accessors {
        let getter_tmpl = v8::FunctionTemplate::builder_raw(*getter_cb).build(scope);
        let name_str = crate::v8_utils::v8_string(scope, name);
        getter_tmpl.set_class_name(name_str);
        getter_tmpl.remove_prototype();
        let getter_fn = match getter_tmpl.get_function(scope) {
            Some(f) => f,
            None => continue,
        };
        let desc = v8::Object::new(scope);
        let _ = desc.set(scope, get_key.into(), getter_fn.into());
        if let Some(setter_cb) = setter_opt {
            let setter_tmpl = v8::FunctionTemplate::builder_raw(*setter_cb).build(scope);
            setter_tmpl.set_class_name(name_str);
            setter_tmpl.remove_prototype();
            if let Some(setter_fn) = setter_tmpl.get_function(scope) {
                let _ = desc.set(scope, set_key.into(), setter_fn.into());
            }
        }
        let _ = desc.set(scope, enum_key.into(), v8::Boolean::new(scope, false).into());
        let _ = desc.set(scope, config_key.into(), v8::Boolean::new(scope, true).into());
        let _ = dop_fn.call(scope, obj.into(), &[proto.into(), name_str.into(), desc.into()]);
    }
}

// ── Environment read helper ──

fn env_str(scope: &v8::PinScope<'_, '_>, key: &str, default: &str) -> String {
    let isolate: &v8::Isolate = scope;
    if let Some(state) = isolate.get_slot::<RuntimeState>() {
        state
            .environment
            .get_str(key)
            .unwrap_or(default)
            .to_string()
    } else {
        default.to_string()
    }
}

// ── Getter / setter macros ──

macro_rules! loc_getter {
    ($name:ident, $key:expr, $default:expr) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let info_ref = unsafe { &*info };
                v8::callback_scope!(unsafe scope, info_ref);
                let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
                let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
                // Check hidden property first (set by setter), then environment
                let this = args.this();
                if this.is_object() {
                    let obj: v8::Local<v8::Object> = unsafe {
                        v8::Local::cast_unchecked(this)
                    };
                    let hidden_key = crate::v8_utils::v8_string(
                        scope,
                        &format!("__loc_{}", $key),
                    );
                    if let Some(hidden_val) = obj.get(scope, hidden_key.into()) {
                        if !hidden_val.is_undefined() {
                            rv.set(hidden_val);
                            return;
                        }
                    }
                }
                let s = env_str(scope, $key, $default);
                if let Some(v) = v8::String::new(scope, &s) {
                    rv.set(v.into());
                }
            }));
        }
    };
}

macro_rules! loc_setter {
    ($name:ident, $key:expr) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let info_ref = unsafe { &*info };
                v8::callback_scope!(unsafe scope, info_ref);
                let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
                if args.length() > 0 {
                    if let Some(val) = args.get(0).to_string(scope) {
                        let s = val.to_rust_string_lossy(scope);
                        let this = args.this();
                        if this.is_object() {
                            let obj: v8::Local<v8::Object> = unsafe {
                                v8::Local::cast_unchecked(this)
                            };
                            let k = crate::v8_utils::v8_string(scope, &format!("__loc_{}", $key));
                            let v = crate::v8_utils::v8_string(scope, &s);
                            obj.set(scope, k.into(), v.into());
                        }
                    }
                }
            }));
        }
    };
}

loc_getter!(loc_href_getter, "location.href", "about:blank");
loc_setter!(loc_href_setter, "location.href");
loc_getter!(loc_origin_getter, "location.origin", "null");
loc_getter!(loc_protocol_getter, "location.protocol", "about:");
loc_setter!(loc_protocol_setter, "location.protocol");
loc_getter!(loc_host_getter, "location.host", "");
loc_setter!(loc_host_setter, "location.host");
loc_getter!(loc_hostname_getter, "location.hostname", "");
loc_setter!(loc_hostname_setter, "location.hostname");
loc_getter!(loc_port_getter, "location.port", "");
loc_setter!(loc_port_setter, "location.port");
loc_getter!(loc_pathname_getter, "location.pathname", "blank");
loc_setter!(loc_pathname_setter, "location.pathname");
loc_getter!(loc_search_getter, "location.search", "");
loc_setter!(loc_search_setter, "location.search");
loc_getter!(loc_hash_getter, "location.hash", "");
loc_setter!(loc_hash_setter, "location.hash");

unsafe extern "C" fn loc_to_string(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        // Check hidden property first (set by setter), then environment
        let this = args.this();
        if this.is_object() {
            let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(this) };
            let hidden_key = crate::v8_utils::v8_string(scope, "__loc_location.href");
            if let Some(hidden_val) = obj.get(scope, hidden_key.into()) {
                if !hidden_val.is_undefined() {
                    rv.set(hidden_val);
                    return;
                }
            }
        }
        let s = env_str(scope, "location.href", "about:blank");
        if let Some(v) = v8::String::new(scope, &s) {
            rv.set(v.into());
        }
    }));
}

unsafe extern "C" fn noop_callback(_info: *const v8::FunctionCallbackInfo) {}
