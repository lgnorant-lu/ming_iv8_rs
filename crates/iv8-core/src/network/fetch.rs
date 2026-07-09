//! fetch() JS binding: looks up ResourceBundle, returns Promise<Response>.
//!
//! Resolution order:
//! 1. If URL is in ResourceBundle → resolve with Response(status=resource.status)
//! 2. If URL is NOT in ResourceBundle, try Python network_handler → resolve with Response
//! 3. If neither works → resolve with Response(status=404, statusText="Not Found")
//!
//! The Response object has: status, ok, statusText, headers, url,
//! text(), json(), arrayBuffer(), blob()
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
                                    headers.push((
                                        name.to_lowercase(),
                                        val.to_rust_string_lossy(scope),
                                    ));
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
    let func = crate::v8_utils::v8_fn(scope, &tmpl);
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
            let msg = crate::v8_utils::v8_string(
                scope,
                "TypeError: Failed to execute 'fetch': 1 argument required",
            );
            let err = v8::Exception::type_error(scope, msg);
            resolver.reject(scope, err);
            return;
        }

        let url_arg = args.get(0);

        // Support Request object as first argument (like real Chrome)
        // If the argument is an object with a .url property, extract the URL.
        let url_str = if url_arg.is_object() && !url_arg.is_string() {
            let obj = unsafe { v8::Local::<v8::Object>::cast_unchecked(url_arg) };
            let url_key = crate::v8_utils::v8_string(scope, "url");
            let url_val = obj.get(scope, url_key.into());
            if let Some(v) = url_val {
                if v.is_string() {
                    v.to_rust_string_lossy(scope)
                } else {
                    url_arg.to_rust_string_lossy(scope)
                }
            } else {
                url_arg.to_rust_string_lossy(scope)
            }
        } else {
            url_arg.to_rust_string_lossy(scope)
        };

        if url_str.starts_with("chrome-extension://") {
            let msg = crate::v8_utils::v8_string(scope, "TypeError: Failed to fetch");
            let err = v8::Exception::type_error(scope, msg);
            resolver.reject(scope, err);
            return;
        }

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
                        // Neither ResourceBundle nor network handler resolved the URL.
                        // Return a 404 Response (do not reject — matches browser fetch
                        // which resolves with an error Response, not a rejected promise).
                        let not_found = crate::network::Resource::new(
                            Vec::new(),
                            404,
                            None,
                        );
                        let response = build_response_object(scope, &not_found);
                        resolver.resolve(scope, response.into());
                    }
                }
            }
        }
    }));
}

/// Build a Response object using the Response FunctionTemplate.
fn build_response_object<'s>(
    scope: &v8::PinScope<'s, '_>,
    resource: &crate::network::Resource,
) -> v8::Local<'s, v8::Object> {
    let state = RuntimeState::get(&*scope);
    let templates = state.dom_templates.borrow();
    let templates = match templates.as_ref() {
        Some(t) => t,
        None => {
            // Fallback: plain object (no DomTemplates available)
            let obj = v8::Object::new(scope);
            let sk = crate::v8_utils::v8_string(scope, "status");
            obj.set(
                scope,
                sk.into(),
                v8::Integer::new(scope, resource.status as i32).into(),
            );
            return obj;
        }
    };

    let tmpl = v8::Local::new(scope, &templates.response);
    let func = match tmpl.get_function(scope) {
        Some(f) => f,
        None => return v8::Object::new(scope),
    };
    let obj = match func.new_instance(scope, &[]) {
        Some(o) => o,
        None => return v8::Object::new(scope),
    };

    // Set backing values under hidden keys. The Response prototype installs
    // accessor properties (status/ok/statusText/url/headers) with no setter,
    // so a plain `obj.set("status", …)` would silently fail and the getter
    // would recurse into itself. Store under "__name__" keys that the getters
    // read from instead.
    let status_key = crate::v8_utils::v8_string(scope, "__status__");
    obj.set(
        scope,
        status_key.into(),
        v8::Integer::new(scope, resource.status as i32).into(),
    );

    let ok_key = crate::v8_utils::v8_string(scope, "__ok__");
    obj.set(
        scope,
        ok_key.into(),
        v8::Boolean::new(scope, resource.status >= 200 && resource.status < 300).into(),
    );

    let st_key = crate::v8_utils::v8_string(scope, "__statusText__");
    let status_text = match resource.status {
        200 => "OK",
        404 => "Not Found",
        _ => "",
    };
    obj.set(
        scope,
        st_key.into(),
        crate::v8_utils::v8_string(scope, status_text).into(),
    );

    let url_key = crate::v8_utils::v8_string(scope, "__url__");
    obj.set(
        scope,
        url_key.into(),
        crate::v8_utils::v8_string(scope, "").into(),
    );

    // Build Headers object using Headers FunctionTemplate
    let header_pairs: Vec<(String, String)> = resource
        .headers
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect();
    let headers_obj = if let Some(ho) =
        crate::dom::template::create_headers_instance(scope, state, templates, header_pairs)
    {
        ho
    } else {
        v8::Object::new(scope)
    };
    let headers_key = crate::v8_utils::v8_string(scope, "__headers__");
    obj.set(scope, headers_key.into(), headers_obj.into());

    // Process Set-Cookie headers: inject into window._iv8CookieStore
    for (k, v) in &resource.headers {
        if k.eq_ignore_ascii_case("set-cookie") {
            // Get or create cookie store
            let global = scope.get_current_context().global(scope);
            let store_key = crate::v8_utils::v8_string(scope, "_iv8CookieStore");
            let store_val = match global.get(scope, store_key.into()) {
                Some(v) if v.is_object() && !v.is_null_or_undefined() => v,
                _ => {
                    let obj = v8::Object::new(scope);
                    global.set(scope, store_key.into(), obj.into());
                    obj.into()
                }
            };
            let store: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(store_val) };

            // Parse "name=val; path=/; secure; ..." format
            let parts: Vec<&str> = v.split(';').collect();
            if parts.is_empty() { continue; }
            let kv: Vec<&str> = parts[0].splitn(2, '=').collect();
            if kv.len() < 2 { continue; }
            let cookie_name = kv[0].trim();
            let cookie_val = kv[1].trim();

            let rec = v8::Object::new(scope);
            let vk = crate::v8_utils::v8_string(scope, "v");
            let vv = crate::v8_utils::v8_string(scope, cookie_val);
            rec.set(scope, vk.into(), vv.into());
            let pk = crate::v8_utils::v8_string(scope, "path");
            let pv = crate::v8_utils::v8_string(scope, "/");
            rec.set(scope, pk.into(), pv.into());

            for attr_part in &parts[1..] {
                let attr = attr_part.trim();
                let lower = attr.to_lowercase();
                if let Some(val) = lower.strip_prefix("path=") {
                    if let Some(p) = v8::String::new(scope, val) {
                        rec.set(scope, pk.into(), p.into());
                    }
                } else if lower == "secure" {
                    let sk = crate::v8_utils::v8_string(scope, "secure");
                    rec.set(scope, sk.into(), v8::Boolean::new(scope, true).into());
                } else if lower == "httponly" {
                    let hk = crate::v8_utils::v8_string(scope, "httpOnly");
                    rec.set(scope, hk.into(), v8::Boolean::new(scope, true).into());
                }
            }

            let cn = crate::v8_utils::v8_string(scope, cookie_name);
            store.set(scope, cn.into(), rec.into());
        }
    }

    // Store body as hidden property
    let body_str = String::from_utf8_lossy(&resource.body);
    let body_key = crate::v8_utils::v8_string(scope, "__body__");
    obj.define_own_property(
        scope,
        body_key.into(),
        crate::v8_utils::v8_string(scope, &body_str).into(),
        v8::PropertyAttribute::DONT_ENUM,
    );

    // Store raw bytes for arrayBuffer
    let store = v8::ArrayBuffer::new_backing_store_from_vec(resource.body.clone());
    let ab = v8::ArrayBuffer::with_backing_store(scope, &store.into());
    let ab_key = crate::v8_utils::v8_string(scope, "__arrayBuffer__");
    obj.define_own_property(
        scope,
        ab_key.into(),
        ab.into(),
        v8::PropertyAttribute::DONT_ENUM,
    );

    obj
}
