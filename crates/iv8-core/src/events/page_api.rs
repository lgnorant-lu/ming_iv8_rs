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

            // Execute inline scripts
            for script_src in scripts.iter() {
                v8::tc_scope!(tc, scope);
                if let Some(src_str) = v8::String::new(tc, script_src) {
                    if let Some(script) = v8::Script::compile(tc, src_str, None) {
                        script.run(tc);
                    }
                }
            }

            // Set readyState to interactive
            let doc_ref = state.document.borrow();
            if let Some(ref doc) = *doc_ref {
                doc.set_ready_state(crate::dom::node::DocumentReadyState::Interactive);
            }
            drop(doc_ref);
        }

        // Update location object if baseURL was provided
        if let Some(ref url_str) = base_url {
            update_location(scope, url_str);
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
