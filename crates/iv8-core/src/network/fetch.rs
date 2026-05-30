//! fetch() JS binding: looks up ResourceBundle, returns Promise<Response>.
//!
//! In v0.1 (strict_compat, default offline):
//! - If URL is in ResourceBundle → resolve with Response object
//! - If URL is NOT in ResourceBundle → reject with TypeError("NetworkError")
//!
//! The Response object has: status, ok, headers, text(), json(), arrayBuffer()

use crate::state::RuntimeState;

/// Install the global fetch() function.
pub fn install_fetch(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    let tmpl = v8::FunctionTemplate::builder_raw(fetch_callback).build(scope);
    let func = tmpl.get_function(scope).expect("fn");
    let key = v8::String::new(scope, "fetch").expect("key");
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
        let resolver = v8::PromiseResolver::new(scope).expect("resolver");
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 1 {
            let msg = v8::String::new(scope, "TypeError: Failed to execute 'fetch': 1 argument required").expect("msg");
            let err = v8::Exception::type_error(scope, msg);
            resolver.reject(scope, err);
            return;
        }

        let url_arg = args.get(0);
        let url_str = url_arg.to_rust_string_lossy(scope);

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
                        h(&url_str, "GET")
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
                        let msg = v8::String::new(
                            scope,
                            &format!("TypeError: Failed to fetch '{}': NetworkError when attempting to fetch resource.", url_str),
                        ).expect("msg");
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
    let status_key = v8::String::new(scope, "status").expect("key");
    let status_val = v8::Integer::new(scope, resource.status as i32);
    obj.set(scope, status_key.into(), status_val.into());

    // ok (status 200-299)
    let ok_key = v8::String::new(scope, "ok").expect("key");
    let ok_val = v8::Boolean::new(scope, resource.status >= 200 && resource.status < 300);
    obj.set(scope, ok_key.into(), ok_val.into());

    // statusText
    let st_key = v8::String::new(scope, "statusText").expect("key");
    let st_val = v8::String::new(scope, if resource.status == 200 { "OK" } else { "" }).expect("val");
    obj.set(scope, st_key.into(), st_val.into());

    // url (empty for now)
    let url_key = v8::String::new(scope, "url").expect("key");
    let url_val = v8::String::new(scope, "").expect("val");
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
    let get_fn = get_tmpl.get_function(scope).expect("fn");
    let get_key = v8::String::new(scope, "get").expect("key");
    headers_obj.set(scope, get_key.into(), get_fn.into());
    // Install has() method
    let has_tmpl = v8::FunctionTemplate::builder_raw(headers_has_cb).build(scope);
    let has_fn = has_tmpl.get_function(scope).expect("fn");
    let has_key = v8::String::new(scope, "has").expect("key");
    headers_obj.set(scope, has_key.into(), has_fn.into());
    let headers_key = v8::String::new(scope, "headers").expect("key");
    obj.set(scope, headers_key.into(), headers_obj.into());

    // Store body as hidden property for text()/json()/arrayBuffer()
    let body_str = String::from_utf8_lossy(&resource.body);
    let body_key = v8::String::new(scope, "__body__").expect("key");
    let body_val = v8::String::new(scope, &body_str).expect("val");
    obj.define_own_property(scope, body_key.into(), body_val.into(), v8::PropertyAttribute::DONT_ENUM);

    // Store raw bytes for arrayBuffer
    let store = v8::ArrayBuffer::new_backing_store_from_vec(resource.body.clone());
    let ab = v8::ArrayBuffer::with_backing_store(scope, &store.into());
    let ab_key = v8::String::new(scope, "__arrayBuffer__").expect("key");
    obj.define_own_property(scope, ab_key.into(), ab.into(), v8::PropertyAttribute::DONT_ENUM);

    // text() → Promise<string>
    let text_tmpl = v8::FunctionTemplate::builder_raw(response_text).build(scope);
    let text_fn = text_tmpl.get_function(scope).expect("fn");
    let text_key = v8::String::new(scope, "text").expect("key");
    obj.set(scope, text_key.into(), text_fn.into());

    // json() → Promise<object>
    let json_tmpl = v8::FunctionTemplate::builder_raw(response_json).build(scope);
    let json_fn = json_tmpl.get_function(scope).expect("fn");
    let json_key = v8::String::new(scope, "json").expect("key");
    obj.set(scope, json_key.into(), json_fn.into());

    // arrayBuffer() → Promise<ArrayBuffer>
    let ab_tmpl = v8::FunctionTemplate::builder_raw(response_array_buffer).build(scope);
    let ab_fn = ab_tmpl.get_function(scope).expect("fn");
    let ab_fn_key = v8::String::new(scope, "arrayBuffer").expect("key");
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
        if let Some(val) = this.get(scope, v8::String::new(scope, &name).expect("key").into()) {
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

        let has = if let Some(val) = this.get(scope, v8::String::new(scope, &name).expect("key").into()) {
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
        let resolver = v8::PromiseResolver::new(scope).expect("resolver");
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        let body_key = v8::String::new(scope, "__body__").expect("key");
        if let Some(body) = this.get(scope, body_key.into()) {
            resolver.resolve(scope, body);
        } else {
            let empty = v8::String::new(scope, "").expect("empty");
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
        let resolver = v8::PromiseResolver::new(scope).expect("resolver");
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        let body_key = v8::String::new(scope, "__body__").expect("key");
        if let Some(body_val) = this.get(scope, body_key.into()) {
            let body_str = body_val.to_rust_string_lossy(scope);
            // Parse JSON using V8's JSON.parse
            let json_key = v8::String::new(scope, "JSON").expect("key");
            let global = scope.get_current_context().global(scope);
            if let Some(json_obj) = global.get(scope, json_key.into()) {
                if json_obj.is_object() {
                    let json_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(json_obj) };
                    let parse_key = v8::String::new(scope, "parse").expect("key");
                    if let Some(parse_fn) = json_obj.get(scope, parse_key.into()) {
                        if parse_fn.is_function() {
                            let parse_fn: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(parse_fn) };
                            let body_v8 = v8::String::new(scope, &body_str).expect("body");
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
        let resolver = v8::PromiseResolver::new(scope).expect("resolver");
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        let ab_key = v8::String::new(scope, "__arrayBuffer__").expect("key");
        if let Some(ab) = this.get(scope, ab_key.into()) {
            resolver.resolve(scope, ab);
        } else {
            let empty = v8::ArrayBuffer::new(scope, 0);
            resolver.resolve(scope, empty.into());
        }
    }));
}
