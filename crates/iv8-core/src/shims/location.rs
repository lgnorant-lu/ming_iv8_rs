//! location object: window.location with href/origin/protocol/host/pathname/search/hash.
//!
//! Reads from environment dot-paths (location.href, location.origin, etc.)
//! or constructs from base_url if available.

use crate::state::RuntimeState;

/// Install the location object on the global scope.
pub fn install_location(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    let isolate: &v8::Isolate = scope;
    let state = RuntimeState::get(isolate);
    let env = &state.environment;

    let location_obj = v8::Object::new(scope);

    // Read from environment or use defaults
    let href = env.get_str("location.href").unwrap_or("about:blank");
    let origin = env.get_str("location.origin").unwrap_or("null");
    let protocol = env.get_str("location.protocol").unwrap_or("about:");
    let host = env.get_str("location.host").unwrap_or("");
    let hostname = env.get_str("location.hostname").unwrap_or("");
    let port = env.get_str("location.port").unwrap_or("");
    let pathname = env.get_str("location.pathname").unwrap_or("blank");
    let search = env.get_str("location.search").unwrap_or("");
    let hash = env.get_str("location.hash").unwrap_or("");

    set_str_prop(scope, location_obj, "href", href);
    set_str_prop(scope, location_obj, "origin", origin);
    set_str_prop(scope, location_obj, "protocol", protocol);
    set_str_prop(scope, location_obj, "host", host);
    set_str_prop(scope, location_obj, "hostname", hostname);
    set_str_prop(scope, location_obj, "port", port);
    set_str_prop(scope, location_obj, "pathname", pathname);
    set_str_prop(scope, location_obj, "search", search);
    set_str_prop(scope, location_obj, "hash", hash);

    // toString() and valueOf() return href
    let to_string_tmpl = v8::FunctionTemplate::builder_raw(location_to_string).build(scope);
    let to_string_fn = crate::v8_utils::v8_fn(scope, &*to_string_tmpl);
    let ts_key = crate::v8_utils::v8_string(scope, "toString");
    location_obj.set(scope, ts_key.into(), to_string_fn.into());

    let vo_key = crate::v8_utils::v8_string(scope, "valueOf");
    location_obj.set(scope, vo_key.into(), to_string_fn.into());

    // assign/replace/reload are no-ops in v0.1
    let noop_tmpl = v8::FunctionTemplate::builder_raw(location_noop).build(scope);
    let noop_fn = crate::v8_utils::v8_fn(scope, &*noop_tmpl);
    for name in &["assign", "replace", "reload"] {
        let key = crate::v8_utils::v8_string(scope, name);
        location_obj.set(scope, key.into(), noop_fn.into());
    }

    // Set on global as 'location'
    let loc_key = crate::v8_utils::v8_string(scope, "location");
    global.set(scope, loc_key.into(), location_obj.into());
}

fn set_str_prop(scope: &v8::PinScope<'_, '_>, obj: v8::Local<v8::Object>, name: &str, value: &str) {
    let key = crate::v8_utils::v8_string(scope, name);
    let val = crate::v8_utils::v8_string(scope, value);
    obj.set(scope, key.into(), val.into());
}

/// location.toString() → href
unsafe extern "C" fn location_to_string(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let this = args.this();
        let href_key = crate::v8_utils::v8_string(scope, "href");
        if let Some(href) = this.get(scope, href_key.into()) {
            rv.set(href);
        }
    }));
}

/// No-op for assign/replace/reload in offline mode.
unsafe extern "C" fn location_noop(_info: *const v8::FunctionCallbackInfo) {}
