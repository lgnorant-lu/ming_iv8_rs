//! fetch() JS binding: looks up ResourceBundle, returns Promise<Response>.
//!
//! In v0.1 (strict_compat, default offline):
//! - If URL is in ResourceBundle → resolve with Response object
//! - If URL is NOT in ResourceBundle → reject with TypeError("NetworkError")
//!
//! The Response object has: status, ok, headers, text(), json(), arrayBuffer()
//!
//! v0.2 (L-04 fix): fetch() requests are also recorded to `__iv8__.netLog.entries`
//! (was previously XHR-only). Same entry format: { method, url, headers, body }.

use crate::state::RuntimeState;

/// Record a fetch request into __iv8__.netLog.entries.
///
/// Mirrors the JS shim used by XHR (network/xhr.rs) but called from the Rust
/// fetch callback. Silent no-op if __iv8__ or netLog is not yet installed
/// (e.g. very early in context lifetime).
fn record_fetch_in_netlog(
    scope: &v8::PinScope<'_, '_>,
    method: &str,
    url: &str,
    header_pairs: &[(String, String)],
    body: &str,
) {
    let global = scope.get_current_context().global(scope);

    let iv8_key = match v8::String::new(scope, "__iv8__") {
        Some(k) => k,
        None => return,
    };
    let iv8_val = match global.get(scope, iv8_key.into()) {
        Some(v) if v.is_object() => v,
        _ => return,
    };
    let iv8_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(iv8_val) };

    let netlog_key = match v8::String::new(scope, "netLog") {
        Some(k) => k,
        None => return,
    };
    let netlog_val = match iv8_obj.get(scope, netlog_key.into()) {
        Some(v) if v.is_object() => v,
        _ => return,
    };
    let netlog_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(netlog_val) };

    let entries_key = match v8::String::new(scope, "entries") {
        Some(k) => k,
        None => return,
    };
    let entries_val = match netlog_obj.get(scope, entries_key.into()) {
        Some(v) if v.is_array() => v,
        _ => return,
    };
    let entries_arr: v8::Local<v8::Array> = unsafe { v8::Local::cast_unchecked(entries_val) };

    // Build entry object: { method, url, headers, body }
    let entry = v8::Object::new(scope);
    if let Some(k) = v8::String::new(scope, "method") {
        if let Some(v) = v8::String::new(scope, method) {
            entry.set(scope, k.into(), v.into());
        }
    }
    if let Some(k) = v8::String::new(scope, "url") {
        if let Some(v) = v8::String::new(scope, url) {
            entry.set(scope, k.into(), v.into());
        }
    }
    // headers: Array of [name, value] pairs (matches XHR shim format).
    let headers_arr = v8::Array::new(scope, header_pairs.len() as i32);
    for (i, (hk, hv)) in header_pairs.iter().enumerate() {
        let pair = v8::Array::new(scope, 2);
        if let Some(name) = v8::String::new(scope, hk) {
            pair.set_index(scope, 0, name.into());
        }
        if let Some(val) = v8::String::new(scope, hv) {
            pair.set_index(scope, 1, val.into());
        }
        headers_arr.set_index(scope, i as u32, pair.into());
    }
    if let Some(k) = v8::String::new(scope, "headers") {
        entry.set(scope, k.into(), headers_arr.into());
    }
    if let Some(k) = v8::String::new(scope, "body") {
        if let Some(v) = v8::String::new(scope, body) {
            entry.set(scope, k.into(), v.into());
        }
    }

    let len = entries_arr.length();
    entries_arr.set_index(scope, len, entry.into());
}

/// Extract method/headers/body from the optional `init` object passed to fetch().
fn parse_fetch_init<'s>(
    scope: &v8::PinScope<'s, '_>,
    init_arg: v8::Local<'s, v8::Value>,
) -> (String, Vec<(String, String)>, String) {
    let mut method = String::from("GET");
    let mut headers = Vec::new();
    let mut body = String::new();

    if !init_arg.is_object() {
        return (method, headers, body);
    }
    let init_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(init_arg) };

    if let Some(method_key) = v8::String::new(scope, "method") {
        if let Some(method_val) = init_obj.get(scope, method_key.into()) {
            if !method_val.is_undefined() && !method_val.is_null() {
                method = method_val.to_rust_string_lossy(scope).to_uppercase();
            }
        }
    }

    if let Some(headers_key) = v8::String::new(scope, "headers") {
        if let Some(headers_val) = init_obj.get(scope, headers_key.into()) {
            if headers_val.is_object() {
                let headers_obj: v8::Local<v8::Object> =
                    unsafe { v8::Local::cast_unchecked(headers_val) };
                let context = scope.get_current_context();
                if let Some(names) = headers_obj.get_own_property_names(scope, Default::default()) {
                    for i in 0..names.length() {
                        if let Some(name_val) = names.get_index(scope, i) {
                            let name = name_val.to_rust_string_lossy(scope);
                            if let Some(val) = headers_obj.get(scope, name_val) {
                                if !val.is_undefined() && !val.is_null() {
                                    headers
                                        .push((name.to_lowercase(), val.to_rust_string_lossy(scope)));
                                }
                            }
                        }
                    }
                    let _ = context; // suppress unused warning if no use of context elsewhere
                }
            }
        }
    }

    if let Some(body_key) = v8::String::new(scope, "body") {
        if let Some(body_val) = init_obj.get(scope, body_key.into()) {
            if !body_val.is_undefined() && !body_val.is_null() {
                body = body_val.to_rust_string_lossy(scope);
            }
        }
    }

    (method, headers, body)
}

/// Install the global fetch() function.
pub fn install_fetch(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    let tmpl = v8::FunctionTemplate::builder_raw(fetch_callback).build(scope);
    let func = crate::v8_utils::v8_fn(scope, &*tmpl);
    let key = crate::v8_utils::v8_string(scope, "fetch");
    func.set_name(key);
    global.set(scope, key.into(), func.into());
}

/// fetch(url, options?) → Promise<Response>
unsafe extern "C" fn fetch_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        // Create a Promise resolver
        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 1 {
            let msg = crate::v8_utils::v8_string(scope, "TypeError: Failed to execute 'fetch': 1 argument required");
            let err = v8::Exception::type_error(scope, msg);
            resolver.reject(scope, err);
            return;
        }

        let url_arg = args.get(0);
        let url_str = url_arg.to_rust_string_lossy(scope);

        // Parse optional init parameter (method/headers/body)
        let (method, headers, body) = if args.length() >= 2 {
            parse_fetch_init(scope, args.get(1))
        } else {
            (String::from("GET"), Vec::new(), String::new())
        };

        // Record into netLog BEFORE attempting the fetch (matches XHR semantics:
        // the request is logged regardless of whether it succeeds).
        record_fetch_in_netlog(scope, &method, &url_str, &headers, &body);

        // Look up in ResourceBundle
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let resource = {
            let bundle = state.resource_bundle.borrow();
            bundle.get(&url_str).cloned()
        };

        match resource {
            Some(res) => {
                // Build Response object
                let response = build_response_object(scope, &res);
                resolver.resolve(scope, response.into());
            }
            None => {
                // Try Python network handler
                let handler_result = {
                    let handler = state.network_handler.borrow();
                    if let Some(ref h) = *handler {
                        h(&url_str, &method)
                    } else {
                        None
                    }
                };

                match handler_result {
                    Some((status, body)) => {
                        let res = crate::network::Resource::new(body, status, None);
                        let response = build_response_object(scope, &res);
                        resolver.resolve(scope, response.into());
                    }
                    None => {
                        // Network error (offline mode)
                        let msg = crate::v8_utils::v8_string(
                            scope,
                            &format!("TypeError: Failed to fetch '{}': NetworkError when attempting to fetch resource.", url_str),
                        );
                        let err = v8::Exception::type_error(scope, msg);
                        resolver.reject(scope, err);
                    }
                }
            }
        }
    }));
}

/// Build a Response-like object with status, ok, headers, text(), json(), arrayBuffer().
fn build_response_object<'s>(
    scope: &v8::PinScope<'s, '_>,
    resource: &crate::network::Resource,
) -> v8::Local<'s, v8::Object> {
    let obj = v8::Object::new(scope);

    // status
    let status_key = crate::v8_utils::v8_string(scope, "status");
    let status_val = v8::Integer::new(scope, resource.status as i32);
    obj.set(scope, status_key.into(), status_val.into());

    // ok (status 200-299)
    let ok_key = crate::v8_utils::v8_string(scope, "ok");
    let ok_val = v8::Boolean::new(scope, resource.status >= 200 && resource.status < 300);
    obj.set(scope, ok_key.into(), ok_val.into());

    // statusText
    let st_key = crate::v8_utils::v8_string(scope, "statusText");
    let st_val = crate::v8_utils::v8_string(scope, if resource.status == 200 { "OK" } else { "" });
    obj.set(scope, st_key.into(), st_val.into());

    // url (empty for now)
    let url_key = crate::v8_utils::v8_string(scope, "url");
    let url_val = crate::v8_utils::v8_string(scope, "");
    obj.set(scope, url_key.into(), url_val.into());

    // headers — build a Headers-like object
    let headers_obj = v8::Object::new(scope);
    for (k, v) in &resource.headers {
        if let (Some(hk), Some(hv)) = (v8::String::new(scope, k), v8::String::new(scope, v)) {
            headers_obj.set(scope, hk.into(), hv.into());
        }
    }
    // Install get() method on headers
    let get_tmpl = v8::FunctionTemplate::builder_raw(headers_get_cb).build(scope);
    let get_fn = crate::v8_utils::v8_fn(scope, &*get_tmpl);
    let get_key = crate::v8_utils::v8_string(scope, "get");
    headers_obj.set(scope, get_key.into(), get_fn.into());
    // Install has() method
    let has_tmpl = v8::FunctionTemplate::builder_raw(headers_has_cb).build(scope);
    let has_fn = crate::v8_utils::v8_fn(scope, &*has_tmpl);
    let has_key = crate::v8_utils::v8_string(scope, "has");
    headers_obj.set(scope, has_key.into(), has_fn.into());
    let headers_key = crate::v8_utils::v8_string(scope, "headers");
    obj.set(scope, headers_key.into(), headers_obj.into());

    // Store body as hidden property for text()/json()/arrayBuffer()
    let body_str = String::from_utf8_lossy(&resource.body);
    let body_key = crate::v8_utils::v8_string(scope, "__body__");
    let body_val = crate::v8_utils::v8_string(scope, &body_str);
    obj.define_own_property(scope, body_key.into(), body_val.into(), v8::PropertyAttribute::DONT_ENUM);

    // Store raw bytes for arrayBuffer
    let store = v8::ArrayBuffer::new_backing_store_from_vec(resource.body.clone());
    let ab = v8::ArrayBuffer::with_backing_store(scope, &store.into());
    let ab_key = crate::v8_utils::v8_string(scope, "__arrayBuffer__");
    obj.define_own_property(scope, ab_key.into(), ab.into(), v8::PropertyAttribute::DONT_ENUM);

    // text() → Promise<string>
    let text_tmpl = v8::FunctionTemplate::builder_raw(response_text).build(scope);
    let text_fn = crate::v8_utils::v8_fn(scope, &*text_tmpl);
    let text_key = crate::v8_utils::v8_string(scope, "text");
    obj.set(scope, text_key.into(), text_fn.into());

    // json() → Promise<object>
    let json_tmpl = v8::FunctionTemplate::builder_raw(response_json).build(scope);
    let json_fn = crate::v8_utils::v8_fn(scope, &*json_tmpl);
    let json_key = crate::v8_utils::v8_string(scope, "json");
    obj.set(scope, json_key.into(), json_fn.into());

    // arrayBuffer() → Promise<ArrayBuffer>
    let ab_tmpl = v8::FunctionTemplate::builder_raw(response_array_buffer).build(scope);
    let ab_fn = crate::v8_utils::v8_fn(scope, &*ab_tmpl);
    let ab_fn_key = crate::v8_utils::v8_string(scope, "arrayBuffer");
    obj.set(scope, ab_fn_key.into(), ab_fn.into());

    obj
}

/// headers.get(name) → string or null
unsafe extern "C" fn headers_get_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            rv.set(v8::null(scope).into());
            return;
        }

        let name = args.get(0).to_rust_string_lossy(scope).to_lowercase();
        let this = args.this();

        // Look up the header by name (case-insensitive)
        if let Some(val) = this.get(scope, crate::v8_utils::v8_string(scope, &name).into()) {
            if !val.is_undefined() && !val.is_null() {
                rv.set(val);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    }));
}

/// headers.has(name) → boolean
unsafe extern "C" fn headers_has_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            rv.set(v8::Boolean::new(scope, false).into());
            return;
        }

        let name = args.get(0).to_rust_string_lossy(scope).to_lowercase();
        let this = args.this();

        let has = if let Some(val) = this.get(scope, crate::v8_utils::v8_string(scope, &name).into()) {
            !val.is_undefined() && !val.is_null()
        } else { false };

        rv.set(v8::Boolean::new(scope, has).into());
    }));
}

/// response.text() → Promise<string>
unsafe extern "C" fn response_text(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let this = args.this();
        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        let body_key = crate::v8_utils::v8_string(scope, "__body__");
        if let Some(body) = this.get(scope, body_key.into()) {
            resolver.resolve(scope, body);
        } else {
            let empty = crate::v8_utils::v8_string(scope, "");
            resolver.resolve(scope, empty.into());
        }
    }));
}

/// response.json() → Promise<object>
unsafe extern "C" fn response_json(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let this = args.this();
        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        let body_key = crate::v8_utils::v8_string(scope, "__body__");
        if let Some(body_val) = this.get(scope, body_key.into()) {
            let body_str = body_val.to_rust_string_lossy(scope);
            // Parse JSON using V8's JSON.parse
            let json_key = crate::v8_utils::v8_string(scope, "JSON");
            let global = scope.get_current_context().global(scope);
            if let Some(json_obj) = global.get(scope, json_key.into()) {
                if json_obj.is_object() {
                    let json_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(json_obj) };
                    let parse_key = crate::v8_utils::v8_string(scope, "parse");
                    if let Some(parse_fn) = json_obj.get(scope, parse_key.into()) {
                        if parse_fn.is_function() {
                            let parse_fn: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(parse_fn) };
                            let body_v8 = crate::v8_utils::v8_string(scope, &body_str);
                            if let Some(parsed) = parse_fn.call(scope, json_obj.into(), &[body_v8.into()]) { resolver.resolve(scope, parsed); return; }
                        }
                    }
                }
            }
            // Fallback: resolve with the string
            resolver.resolve(scope, body_val);
        } else {
            resolver.resolve(scope, v8::null(scope).into());
        }
    }));
}

/// response.arrayBuffer() → Promise<ArrayBuffer>
unsafe extern "C" fn response_array_buffer(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let this = args.this();
        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        let ab_key = crate::v8_utils::v8_string(scope, "__arrayBuffer__");
        if let Some(ab) = this.get(scope, ab_key.into()) {
            resolver.resolve(scope, ab);
        } else {
            let empty = v8::ArrayBuffer::new(scope, 0);
            resolver.resolve(scope, empty.into());
        }
    }));
}
