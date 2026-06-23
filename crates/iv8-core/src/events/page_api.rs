//! __iv8__.page.load(snapshot) API
//!
//! Accepts a snapshot object: { baseURL, html, headers, resources }
//! - baseURL: string — the page URL
//! - html: string — the HTML content
//! - headers: [[name, value], ...] — response headers (optional)
//! - resources: { url: content, ... } — pre-fetched JS/CSS resources (optional)
//!
//! This is the primary way iv8 examples load pages with offline resources.

use crate::state::RuntimeState;

/// Helper macro: evaluate a JS string in the given scope, discarding errors.
/// Equivalent to `self.eval(js, EvalOpts::default()).ok()` in embedded_v8.rs.
macro_rules! eval_js {
    ($scope:expr, $js:expr) => {{
        v8::tc_scope!(tc, $scope);
        if let Some(src_str) = v8::String::new(tc, $js) {
            if let Some(script) = v8::Script::compile(tc, src_str, None) {
                script.run(tc);
            }
        }
    }};
}

/// Install __iv8__.page on the __iv8__ tool object.
pub fn install_page_api(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    let js_api_name = {
        let isolate: &v8::Isolate = scope;
        let state = RuntimeState::get(isolate);
        state.js_api_name.clone()
    };

    let api_key = crate::v8_utils::v8_string(scope, &js_api_name);
    let api_obj = match global.get(scope, api_key.into()) {
        Some(v) if v.is_object() => unsafe { v8::Local::<v8::Object>::cast_unchecked(v) },
        _ => return,
    };

    let page_obj = v8::Object::new(scope);

    // __iv8__.page.load(snapshot)
    let load_tmpl = v8::FunctionTemplate::builder_raw(page_load_callback).build(scope);
    let load_fn = crate::v8_utils::v8_fn(scope, &load_tmpl);
    let load_key = crate::v8_utils::v8_string(scope, "load");
    page_obj.set(scope, load_key.into(), load_fn.into());

    let page_key = crate::v8_utils::v8_string(scope, "page");
    api_obj.set(scope, page_key.into(), page_obj.into());
}

/// __iv8__.page.load(snapshot) callback
/// snapshot: { baseURL?: string, html: string, resources?: {url: content}, headers?: [[k,v]] }
unsafe extern "C" fn page_load_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 1 || !args.get(0).is_object() {
            return;
        }

        let snapshot: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(args.get(0)) };

        // Extract baseURL
        let base_url = get_string_field(scope, snapshot, "baseURL");

        // Extract html
        let html = get_string_field(scope, snapshot, "html").unwrap_or_else(|| {
            "<!DOCTYPE html><html><head></head><body></body></html>".to_string()
        });

        // Extract resources: { url: content, ... }
        let resources = extract_resources(scope, snapshot);

        // Get isolate pointer before any scope borrows
        let isolate_ptr: *const v8::Isolate = {
            let iso: &v8::Isolate = &*scope;
            iso as *const v8::Isolate
        };

        {
            let state = RuntimeState::get(unsafe { &*isolate_ptr });

            // Register all resources in the bundle
            for (url, content) in &resources {
                state.resource_bundle.borrow_mut().add_raw(
                    url,
                    content.as_bytes().to_vec(),
                    200,
                    None,
                );
            }

            // Parse and store the document
            let doc = crate::dom::parse_html(&html, base_url.as_deref());

            // Collect scripts: both inline content and external src URLs
            // For external scripts, look up content in resources map
            let mut scripts: Vec<String> = Vec::new();
            for &nid in doc.get_elements_by_tag_name("script").iter() {
                // Check for src attribute (external script)
                let src_attr = {
                    let node_ref = doc.get(nid);
                    node_ref.and_then(|n| n.value().get_attr("src").map(|s| s.to_string()))
                };

                if let Some(src) = src_attr {
                    // External script: resolve URL and look up in resources
                    let resolved_url = if src.starts_with("http://") || src.starts_with("https://")
                    {
                        src.clone()
                    } else if let Some(ref base) = base_url {
                        // Resolve relative URL against base
                        if src.starts_with('/') {
                            // Absolute path
                            if let Ok(parsed) = url::Url::parse(base) {
                                format!(
                                    "{}://{}{}",
                                    parsed.scheme(),
                                    parsed.host_str().unwrap_or(""),
                                    src
                                )
                            } else {
                                src.clone()
                            }
                        } else {
                            src.clone()
                        }
                    } else {
                        src.clone()
                    };

                    // Look up in resources map
                    if let Some(content) = resources
                        .iter()
                        .find(|(u, _)| u == &resolved_url || u == &src)
                    {
                        scripts.push(content.1.clone());
                    }
                } else {
                    // Inline script
                    let content = doc.text_content_of(nid);
                    if !content.is_empty() {
                        scripts.push(content);
                    }
                }
            }

            // Store document
            *state.document.borrow_mut() = Some(doc);
            state.node_cache.borrow_mut().clear();
            state.style_cache.borrow_mut().clear();

            // Install DOM bindings
            {
                let ctx = scope.get_current_context();
                let global = ctx.global(scope);
                crate::dom::binding::install_document_bindings(scope, global);
            }

            // Pre-populate cookie store from Set-Cookie headers before shims run
            inject_set_cookie_headers(scope, snapshot);

            // 4b. Re-install Canvas2D shim (DOM bindings may have reset HTMLCanvasElement.prototype)
            eval_js!(scope, crate::canvas::binding::CANVAS2D_SHIM_JS);

            // 4c. Install document.write workaround shim
            eval_js!(scope, crate::kernel::embedded_v8::DOCUMENT_WRITE_SHIM);

            // 4d. Re-install document properties (readyState, cookie, etc.)
            eval_js!(scope, crate::shims::document_props::DOCUMENT_PROPS_JS);

            // 4d2. Re-install AudioContext subsystem
            eval_js!(scope, crate::shims::audio_context::AUDIO_CONTEXT_JS);

            // 4d3. Re-install window properties, global constructors, structuredClone, Blob
            eval_js!(scope, crate::shims::window_extras::WINDOW_EXTRAS_JS);

            // 4e. Update location if baseURL was provided (before scripts run,
            // so inline scripts see the correct location.href)
            if let Some(ref url_str) = base_url {
                update_location(scope, url_str);
            }

            // Execute inline scripts
            for script_src in scripts.iter() {
                v8::tc_scope!(tc, scope);
                if let Some(src_str) = v8::String::new(tc, script_src) {
                    if let Some(script) = v8::Script::compile(tc, src_str, None) {
                        script.run(tc);
                    }
                }
            }

            // 6. Set readyState to interactive (Rust + JS side)
            {
                let doc_ref = state.document.borrow();
                if let Some(ref doc) = *doc_ref {
                    doc.set_ready_state(crate::dom::node::DocumentReadyState::Interactive);
                }
                drop(doc_ref);
            }
            eval_js!(scope, "try { document.readyState = 'interactive'; } catch(e) {}");

            // 7. Dispatch DOMContentLoaded event on document root
            {
                let doc_ref = state.document.borrow();
                if let Some(ref document) = *doc_ref {
                    let root_id = document.root_id();
                    let registry = &state.event_listeners;
                    crate::events::target::dispatch_event(
                        scope,
                        registry,
                        document,
                        root_id,
                        "DOMContentLoaded",
                        false,
                    );
                }
            }

            // 8. Set readyState to complete (Rust + JS side)
            {
                let doc_ref = state.document.borrow();
                if let Some(ref doc) = *doc_ref {
                    doc.set_ready_state(crate::dom::node::DocumentReadyState::Complete);
                }
                drop(doc_ref);
            }
            eval_js!(scope, "try { document.readyState = 'complete'; } catch(e) {}");

            // 9. Dispatch load event on document root
            {
                let doc_ref = state.document.borrow();
                if let Some(ref document) = *doc_ref {
                    let root_id = document.root_id();
                    let registry = &state.event_listeners;
                    crate::events::target::dispatch_event(
                        scope,
                        registry,
                        document,
                        root_id,
                        "load",
                        false,
                    );
                }
            }

            // 9b. Re-install cookie accessor after all scripts executed.
            // Inline scripts may have interfered with the cookie accessor via
            // Object.defineProperty. Only re-install cookie (not full
            // DOCUMENT_PROPS_JS) to avoid Intl/Date re-entrancy OOM.
            eval_js!(scope, crate::shims::document_props::COOKIE_REINSTALL_JS);

            // 10. Drain microtasks
            {
                let isolate: &mut v8::Isolate = scope;
                isolate.perform_microtask_checkpoint();
            }
        }
    }));
}

/// Update the location object after page.load with a new baseURL.
fn update_location(scope: &v8::PinScope<'_, '_>, url_str: &str) {
    // Parse the URL into components
    let parsed = match url::Url::parse(url_str) {
        Ok(u) => u,
        Err(_) => return,
    };

    let href = url_str.to_string();
    let origin = format!("{}://{}", parsed.scheme(), parsed.host_str().unwrap_or(""));
    let protocol = format!("{}:", parsed.scheme());
    let host = parsed.host_str().unwrap_or("").to_string();
    let hostname = host.clone();
    let port = parsed.port().map(|p| p.to_string()).unwrap_or_default();
    let pathname = parsed.path().to_string();
    let search = parsed
        .query()
        .map(|q| format!("?{}", q))
        .unwrap_or_default();
    let hash = parsed
        .fragment()
        .map(|f| format!("#{}", f))
        .unwrap_or_default();

    // Get the location object from global and update its properties directly
    let global = scope.get_current_context().global(scope);
    let loc_key = match v8::String::new(scope, "location") {
        Some(k) => k,
        None => return,
    };
    let loc_val = match global.get(scope, loc_key.into()) {
        Some(v) if v.is_object() && !v.is_null_or_undefined() => v,
        _ => return,
    };
    let loc_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(loc_val) };

    let props = [
        ("href", href.as_str()),
        ("origin", origin.as_str()),
        ("protocol", protocol.as_str()),
        ("host", host.as_str()),
        ("hostname", hostname.as_str()),
        ("port", port.as_str()),
        ("pathname", pathname.as_str()),
        ("search", search.as_str()),
        ("hash", hash.as_str()),
    ];

    for (key, val) in &props {
        if let (Some(k), Some(v)) = (v8::String::new(scope, key), v8::String::new(scope, val)) {
            loc_obj.set(scope, k.into(), v.into());
        }
    }
}

/// Extract a string field from a JS object.
fn get_string_field(
    scope: &v8::PinScope<'_, '_>,
    obj: v8::Local<v8::Object>,
    key: &str,
) -> Option<String> {
    let k = v8::String::new(scope, key)?;
    let val = obj.get(scope, k.into())?;
    if val.is_string() || val.is_string_object() {
        Some(val.to_rust_string_lossy(scope))
    } else {
        None
    }
}

/// Extract resources map from snapshot.resources: { url: content_or_obj, ... }
/// Supports both string values and {body: '...'} object values (iv8 format).
fn extract_resources(
    scope: &v8::PinScope<'_, '_>,
    snapshot: v8::Local<v8::Object>,
) -> Vec<(String, String)> {
    let mut result = Vec::new();

    let res_key = match v8::String::new(scope, "resources") {
        Some(k) => k,
        None => return result,
    };

    let res_val = match snapshot.get(scope, res_key.into()) {
        Some(v) if v.is_object() && !v.is_null_or_undefined() => v,
        _ => return result,
    };

    let res_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(res_val) };

    // Get own property names
    let names = match res_obj.get_own_property_names(scope, v8::GetPropertyNamesArgs::default()) {
        Some(n) => n,
        None => return result,
    };

    for i in 0..names.length() {
        if let Some(name_val) = names.get_index(scope, i) {
            let url = name_val.to_rust_string_lossy(scope);
            if let Some(content_val) = res_obj.get(scope, name_val) {
                if content_val.is_null_or_undefined() {
                    continue;
                }
                // Support both string values and {body: '...'} objects
                let content = if content_val.is_string() || content_val.is_string_object() {
                    content_val.to_rust_string_lossy(scope)
                } else if content_val.is_object() {
                    // Try to get .body property
                    let content_obj: v8::Local<v8::Object> =
                        unsafe { v8::Local::cast_unchecked(content_val) };
                    let body_key = match v8::String::new(scope, "body") {
                        Some(k) => k,
                        None => continue,
                    };
                    match content_obj.get(scope, body_key.into()) {
                        Some(body_val) if !body_val.is_null_or_undefined() => {
                            body_val.to_rust_string_lossy(scope)
                        }
                        _ => continue,
                    }
                } else {
                    continue;
                };
                result.push((url, content));
            }
        }
    }

    result
}

/// Extract Set-Cookie headers from snapshot.headers and pre-populate
/// the cookie store before DOCUMENT_PROPS_JS runs.
///
/// headers format: [[name, value], ...]
/// Set-Cookie entries are parsed into { v: value, path: "/", ... } records
/// and stored in window._iv8CookieStore.
fn inject_set_cookie_headers(scope: &v8::PinScope<'_, '_>, snapshot: v8::Local<v8::Object>) {
    let headers_key = match v8::String::new(scope, "headers") {
        Some(k) => k,
        None => return,
    };
    let headers_val = match snapshot.get(scope, headers_key.into()) {
        Some(v) if v.is_array() => v,
        _ => return,
    };
    let headers_arr: v8::Local<v8::Array> = unsafe { v8::Local::cast_unchecked(headers_val) };
    let len = headers_arr.length();

    // Get or create window._iv8CookieStore
    let global = scope.get_current_context().global(scope);
    let store_key = match v8::String::new(scope, "_iv8CookieStore") {
        Some(k) => k,
        None => return,
    };
    let store_val = match global.get(scope, store_key.into()) {
        Some(v) if v.is_object() && !v.is_null_or_undefined() => v,
        _ => {
            // Create the store
            let obj = v8::Object::new(scope);
            global.set(scope, store_key.into(), obj.into());
            obj.into()
        }
    };
    let store: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(store_val) };

    for i in 0..len {
        let entry = match headers_arr.get_index(scope, i) {
            Some(v) if v.is_array() => v,
            _ => continue,
        };
        let entry_arr: v8::Local<v8::Array> = unsafe { v8::Local::cast_unchecked(entry) };
        let name = match entry_arr.get_index(scope, 0) {
            Some(v) => v.to_rust_string_lossy(scope),
            None => continue,
        };
        if !name.eq_ignore_ascii_case("set-cookie") {
            continue;
        }
        let value = match entry_arr.get_index(scope, 1) {
            Some(v) => v.to_rust_string_lossy(scope),
            None => continue,
        };

        // Parse "name=val; path=/; secure; ..." format
        let parts: Vec<&str> = value.split(';').collect();
        if parts.is_empty() {
            continue;
        }
        let kv: Vec<&str> = parts[0].splitn(2, '=').collect();
        if kv.len() < 2 {
            continue;
        }
        let cookie_name = kv[0].trim();
        let cookie_val = kv[1].trim();

        // Build record object { v: value, path: "/", ... }
        let rec = v8::Object::new(scope);
        let v_key = v8::String::new(scope, "v").unwrap();
        let v_val = v8::String::new(scope, cookie_val).unwrap();
        rec.set(scope, v_key.into(), v_val.into());

        let path_key = v8::String::new(scope, "path").unwrap();
        let path_val = v8::String::new(scope, "/").unwrap();
        rec.set(scope, path_key.into(), path_val.into());

        // Parse additional attributes
        for attr_part in &parts[1..] {
            let attr = attr_part.trim();
            let lower = attr.to_lowercase();
            if let Some(val) = lower.strip_prefix("path=") {
                if let Some(p) = v8::String::new(scope, val) {
                    rec.set(scope, path_key.into(), p.into());
                }
            } else if let Some(val) = lower.strip_prefix("domain=") {
                let dk = v8::String::new(scope, "domain").unwrap();
                if let Some(dv) = v8::String::new(scope, val) {
                    rec.set(scope, dk.into(), dv.into());
                }
            } else if lower == "secure" {
                let sk = v8::String::new(scope, "secure").unwrap();
                rec.set(scope, sk.into(), v8::Boolean::new(scope, true).into());
            } else if lower == "httponly" {
                let hk = v8::String::new(scope, "httpOnly").unwrap();
                rec.set(scope, hk.into(), v8::Boolean::new(scope, true).into());
            }
        }

        let cn = v8::String::new(scope, cookie_name).unwrap();
        store.set(scope, cn.into(), rec.into());
    }
}
