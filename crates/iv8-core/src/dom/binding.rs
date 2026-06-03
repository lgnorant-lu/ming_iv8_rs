//! DOM V8 bindings: expose document query methods to JavaScript.
//!
//! Installs native V8 functions for:
//! - document.getElementById(id)
//! - document.querySelector(selector)
//! - document.querySelectorAll(selector)
//! - document.getElementsByTagName(tag)
//! - document.getElementsByClassName(className)
//!
//! Returns simplified V8 objects for now (Task 29 adds full cppgc wrapper identity).

use crate::dom::{NodeData, NodeId};
use crate::state::RuntimeState;

/// Convert NodeId to a usize for storage in V8 (as f64).
/// NodeId is NonZeroUsize internally, so we transmute.
pub fn node_id_to_usize(id: NodeId) -> usize {
    // Compile-time check: NodeId must be the same size as NonZeroUsize
    const _: () = assert!(
        std::mem::size_of::<NodeId>() == std::mem::size_of::<std::num::NonZeroUsize>(),
        "NodeId layout changed — transmute is unsound"
    );
    // SAFETY: NodeId is a newtype around NonZeroUsize (ego-tree 0.10)
    let nz: std::num::NonZeroUsize = unsafe { std::mem::transmute(id) };
    nz.get()
}

/// Convert a usize back to NodeId.
/// Returns None if the value is 0 (invalid).
pub fn usize_to_node_id(val: usize) -> Option<NodeId> {
    const _: () = assert!(
        std::mem::size_of::<NodeId>() == std::mem::size_of::<std::num::NonZeroUsize>(),
        "NodeId layout changed — transmute is unsound"
    );
    let nz = std::num::NonZeroUsize::new(val)?;
    // SAFETY: NodeId is a newtype around NonZeroUsize (ego-tree 0.10)
    Some(unsafe { std::mem::transmute::<std::num::NonZeroUsize, NodeId>(nz) })
}

/// Extract __nodeId__ from a V8 object, returning the NodeId.
pub fn extract_node_id(scope: &v8::PinScope<'_, '_>, obj: v8::Local<v8::Object>) -> Option<NodeId> {
    let key = v8::String::new(scope, "__nodeId__")?;
    let val = obj.get(scope, key.into())?;
    if val.is_number() {
        let num = val.number_value(scope)? as usize;
        usize_to_node_id(num)
    } else {
        None
    }
}

/// Install DOM query bindings on the `document` global object.
/// Call this after a Document has been set in RuntimeState.
pub fn install_document_bindings(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    // Create the document object
    let doc_obj = v8::Object::new(scope);

    // Install methods
    install_method(scope, doc_obj, "getElementById", get_element_by_id);
    install_method(scope, doc_obj, "querySelector", query_selector);
    install_method(scope, doc_obj, "querySelectorAll", query_selector_all);
    install_method(scope, doc_obj, "getElementsByTagName", get_elements_by_tag_name);
    install_method(scope, doc_obj, "getElementsByClassName", get_elements_by_class_name);
    install_method(scope, doc_obj, "createElement", create_element);

    // EventTarget methods on document (v0.2: L-03 fix).
    //
    // Real DOM exposes addEventListener/removeEventListener/dispatchEvent on
    // document because Document inherits from EventTarget. v0.1 had stub
    // versions installed via document_props.js that did nothing. v0.2 wires
    // them to the EventListenerRegistry using the DOM tree's root NodeId.
    install_method(scope, doc_obj, "addEventListener", add_event_listener_callback);
    install_method(scope, doc_obj, "removeEventListener", remove_event_listener_callback);
    install_method(scope, doc_obj, "dispatchEvent", dispatch_event_callback);

    // Install document.documentElement / document.body / document.head as accessors
    install_doc_accessor(scope, doc_obj, "documentElement", doc_document_element);
    install_doc_accessor(scope, doc_obj, "body", doc_body);
    install_doc_accessor(scope, doc_obj, "head", doc_head);

    // Install document.title as an accessor (reads <title> element text)
    install_doc_accessor(scope, doc_obj, "title", doc_title);

    // Install document.URL / document.documentURI as accessors (= location.href)
    install_doc_accessor(scope, doc_obj, "URL", doc_url);
    install_doc_accessor(scope, doc_obj, "documentURI", doc_url);

    // Install document.location as accessor (= window.location)
    install_doc_accessor(scope, doc_obj, "location", doc_location);

    // Bind document to the DOM tree's root NodeId so that
    // addEventListener/dispatchEvent can locate it via extract_node_id.
    // If no Document is loaded yet (e.g. JSContext init before page.load),
    // skip silently — the binding will be redone when set_document/page_load runs.
    let isolate: &v8::Isolate = scope;
    let state = RuntimeState::get(isolate);
    let root_id_opt = state
        .document
        .borrow()
        .as_ref()
        .map(|doc| doc.root_id());
    if let Some(root_id) = root_id_opt {
        let id_key = crate::v8_utils::v8_string(scope, "__nodeId__");
        let nz: std::num::NonZeroUsize = unsafe { std::mem::transmute(root_id) };
        let id_val = v8::Number::new(scope, nz.get() as f64);
        // Use DontEnum so Object.keys(document) doesn't show __nodeId__.
        doc_obj.define_own_property(
            scope,
            id_key.into(),
            id_val.into(),
            v8::PropertyAttribute::DONT_ENUM | v8::PropertyAttribute::DONT_DELETE,
        );
    }

    // Set document on global
    let key = crate::v8_utils::v8_string(scope, "document");
    global.set(scope, key.into(), doc_obj.into());
}

/// Helper to install a native accessor (getter) on an object.
fn install_doc_accessor(
    scope: &v8::PinScope<'_, '_>,
    obj: v8::Local<v8::Object>,
    name: &str,
    getter: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
) {
    let getter_tmpl = v8::FunctionTemplate::builder_raw(getter).build(scope);
    let getter_fn = crate::v8_utils::v8_fn(scope, &getter_tmpl);
    let name_str = crate::v8_utils::v8_string(scope, name);
    // Use defineProperty to install as a getter
    let desc = v8::Object::new(scope);
    let get_key = crate::v8_utils::v8_string(scope, "get");
    let enum_key = crate::v8_utils::v8_string(scope, "enumerable");
    let conf_key = crate::v8_utils::v8_string(scope, "configurable");
    desc.set(scope, get_key.into(), getter_fn.into());
    desc.set(scope, enum_key.into(), v8::Boolean::new(scope, true).into());
    desc.set(scope, conf_key.into(), v8::Boolean::new(scope, true).into());
    // Use Object.defineProperty via JS
    let global = scope.get_current_context().global(scope);
    let obj_key = crate::v8_utils::v8_string(scope, "Object");
    if let Some(obj_ctor) = global.get(scope, obj_key.into()) {
        if obj_ctor.is_object() {
            let obj_ctor: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(obj_ctor) };
            let def_prop_key = crate::v8_utils::v8_string(scope, "defineProperty");
            if let Some(def_prop) = obj_ctor.get(scope, def_prop_key.into()) {
                if def_prop.is_function() {
                    let def_prop_fn: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(def_prop) };
                    let undefined = v8::undefined(scope);
                    def_prop_fn.call(scope, undefined.into(), &[obj.into(), name_str.into(), desc.into()]);
                }
            }
        }
    }
}

/// document.documentElement getter — returns the <html> element
unsafe extern "C" fn doc_document_element(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let node_id = {
            let doc = state.document.borrow();
            doc.as_ref().and_then(|d| d.document_element())
        };
        if let Some(nid) = node_id {
            if let Some(obj) = crate::dom::template::create_node_object(scope, state, nid) {
                rv.set(obj);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    }));
}

/// document.body getter — returns the <body> element
unsafe extern "C" fn doc_body(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let node_id = {
            let doc = state.document.borrow();
            doc.as_ref().and_then(|d| d.body())
        };
        if let Some(nid) = node_id {
            if let Some(obj) = crate::dom::template::create_node_object(scope, state, nid) {
                rv.set(obj);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    }));
}

/// document.head getter — returns the <head> element
unsafe extern "C" fn doc_head(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let node_id = {
            let doc = state.document.borrow();
            doc.as_ref().and_then(|d| d.head())
        };
        if let Some(nid) = node_id {
            if let Some(obj) = crate::dom::template::create_node_object(scope, state, nid) {
                rv.set(obj);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    }));
}

/// document.title getter — returns text content of <title> element
unsafe extern "C" fn doc_title(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let title = {
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                // Find <title> element and get its text content
                let titles = doc.get_elements_by_tag_name("title");
                titles.first().map(|&nid| doc.text_content_of(nid)).unwrap_or_default()
            } else {
                String::new()
            }
        };
        if let Some(s) = v8::String::new(scope, &title) {
            rv.set(s.into());
        } else {
            rv.set(crate::v8_utils::v8_string(scope, "").into());
        }
    }));
}

/// document.URL / document.documentURI getter — returns location.href
unsafe extern "C" fn doc_url(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        // Read location.href from the global
        let global = scope.get_current_context().global(scope);
        let loc_key = match v8::String::new(scope, "location") { Some(k) => k, None => return };
        if let Some(loc_val) = global.get(scope, loc_key.into()) {
            if loc_val.is_object() && !loc_val.is_null_or_undefined() {
                let loc_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(loc_val) };
                let href_key = match v8::String::new(scope, "href") { Some(k) => k, None => return };
                if let Some(href_val) = loc_obj.get(scope, href_key.into()) {
                    rv.set(href_val);
                    return;
                }
            }
        }
        rv.set(crate::v8_utils::v8_string(scope, "about:blank").into());
    }));
}

/// document.location getter — returns the window.location object itself
/// In real browsers: document.location === window.location
unsafe extern "C" fn doc_location(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let global = scope.get_current_context().global(scope);
        let loc_key = match v8::String::new(scope, "location") { Some(k) => k, None => return };
        if let Some(loc_val) = global.get(scope, loc_key.into()) {
            rv.set(loc_val);
        }
    }));
}

/// Helper to install a native method on an object.
fn install_method(
    scope: &v8::PinScope<'_, '_>,
    obj: v8::Local<v8::Object>,
    name: &str,
    callback: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
) {
    let tmpl = v8::FunctionTemplate::builder_raw(callback).build(scope);
    let func = crate::v8_utils::v8_fn(scope, &tmpl);
    let name_str = crate::v8_utils::v8_string(scope, name);
    func.set_name(name_str);
    obj.set(scope, name_str.into(), func.into());
}

/// Convert a DOM node to a V8 object.
/// Uses the ObjectTemplate-based approach if DOM templates are installed,
/// otherwise falls back to the plain object approach.
pub fn node_to_v8_object<'s>(
    scope: &v8::PinScope<'s, '_>,
    state: &RuntimeState,
    node_id: NodeId,
) -> Option<v8::Local<'s, v8::Value>> {
    // Prefer the template-based approach (ObjectTemplate refactor)
    if state.dom_templates.borrow().is_some() {
        return crate::dom::template::create_node_object(scope, state, node_id);
    }

    // Fallback: plain object approach (used before templates are installed)
    node_to_v8_object_plain(scope, state, node_id)
}

/// Fallback: create a plain V8 object for a DOM node (used before templates are installed).
fn node_to_v8_object_plain<'s>(
    scope: &v8::PinScope<'s, '_>,
    state: &RuntimeState,
    node_id: NodeId,
) -> Option<v8::Local<'s, v8::Value>> {
    // Identity cache: return same object for same NodeId
    {
        let cache = state.node_cache.borrow();
        if let Some(global) = cache.get(&node_id) {
            return Some(v8::Local::new(scope, global).into());
        }
    }

    let doc = state.document.borrow();
    let doc = doc.as_ref()?;
    let node_ref = doc.get(node_id)?;
    let data = node_ref.value();

    let obj = v8::Object::new(scope);

    // Store hidden node ID for mutation methods
    let nid_key = crate::v8_utils::v8_string(scope, "__nodeId__");
    // ego-tree NodeId is a NonZeroUsize, store as integer
    let nid_val = v8::Number::new(scope, node_id_to_usize(node_id) as f64);
    obj.define_own_property(
        scope,
        nid_key.into(),
        nid_val.into(),
        v8::PropertyAttribute::DONT_ENUM,
    );

    // nodeType
    let node_type_key = crate::v8_utils::v8_string(scope, "nodeType");
    let node_type_val = v8::Integer::new(scope, data.node_type() as i32);
    obj.set(scope, node_type_key.into(), node_type_val.into());

    // nodeName
    let node_name_key = crate::v8_utils::v8_string(scope, "nodeName");
    let node_name_val = crate::v8_utils::v8_string(scope, data.node_name());
    obj.set(scope, node_name_key.into(), node_name_val.into());

    match data {
        NodeData::Element { tag_name, attrs, id, classes, .. } => {
            // tagName (uppercase for HTML)
            let tag_key = crate::v8_utils::v8_string(scope, "tagName");
            let tag_val = crate::v8_utils::v8_string(scope, &tag_name.to_ascii_uppercase());
            obj.set(scope, tag_key.into(), tag_val.into());

            // id
            let id_key = crate::v8_utils::v8_string(scope, "id");
            let id_val = crate::v8_utils::v8_string(scope, id.as_deref().unwrap_or(""));
            obj.set(scope, id_key.into(), id_val.into());

            // className
            let class_key = crate::v8_utils::v8_string(scope, "className");
            let class_val = crate::v8_utils::v8_string(scope, &classes.join(" "));
            obj.set(scope, class_key.into(), class_val.into());

            // textContent
            let text_key = crate::v8_utils::v8_string(scope, "textContent");
            let text_content = doc.text_content_of(node_id);
            let text_val = crate::v8_utils::v8_string(scope, &text_content);
            obj.set(scope, text_key.into(), text_val.into());

            // getAttribute method
            let _get_attr_data = attrs.clone();
            // For simplicity, store attrs as a hidden property and provide getAttribute
            let attrs_obj = v8::Object::new(scope);
            for (k, v) in attrs {
                if let (Some(ak), Some(av)) = (v8::String::new(scope, k), v8::String::new(scope, v)) {
                    attrs_obj.set(scope, ak.into(), av.into());
                }
            }
            let attrs_key = crate::v8_utils::v8_string(scope, "__attrs__");
            obj.set(scope, attrs_key.into(), attrs_obj.into());

            // Install getAttribute as a native function
            let get_attr_tmpl = v8::FunctionTemplate::builder_raw(get_attribute_callback).build(scope);
            let get_attr_fn = crate::v8_utils::v8_fn(scope, &get_attr_tmpl);
            let get_attr_key = crate::v8_utils::v8_string(scope, "getAttribute");
            obj.set(scope, get_attr_key.into(), get_attr_fn.into());

            // Install setAttribute
            let set_attr_tmpl = v8::FunctionTemplate::builder_raw(set_attribute_callback).build(scope);
            let set_attr_fn = crate::v8_utils::v8_fn(scope, &set_attr_tmpl);
            let set_attr_key = crate::v8_utils::v8_string(scope, "setAttribute");
            obj.set(scope, set_attr_key.into(), set_attr_fn.into());

            // Install removeAttribute
            let rm_attr_tmpl = v8::FunctionTemplate::builder_raw(remove_attribute_callback).build(scope);
            let rm_attr_fn = crate::v8_utils::v8_fn(scope, &rm_attr_tmpl);
            let rm_attr_key = crate::v8_utils::v8_string(scope, "removeAttribute");
            obj.set(scope, rm_attr_key.into(), rm_attr_fn.into());

            // Install hasAttribute
            let has_attr_tmpl = v8::FunctionTemplate::builder_raw(has_attribute_callback).build(scope);
            let has_attr_fn = crate::v8_utils::v8_fn(scope, &has_attr_tmpl);
            let has_attr_key = crate::v8_utils::v8_string(scope, "hasAttribute");
            obj.set(scope, has_attr_key.into(), has_attr_fn.into());

            // Install appendChild
            let append_tmpl = v8::FunctionTemplate::builder_raw(append_child_callback).build(scope);
            let append_fn = crate::v8_utils::v8_fn(scope, &append_tmpl);
            let append_key = crate::v8_utils::v8_string(scope, "appendChild");
            obj.set(scope, append_key.into(), append_fn.into());

            // Install removeChild
            let remove_tmpl = v8::FunctionTemplate::builder_raw(remove_child_callback).build(scope);
            let remove_fn = crate::v8_utils::v8_fn(scope, &remove_tmpl);
            let remove_key = crate::v8_utils::v8_string(scope, "removeChild");
            obj.set(scope, remove_key.into(), remove_fn.into());

            // Install replaceChild
            let replace_tmpl = v8::FunctionTemplate::builder_raw(replace_child_callback).build(scope);
            let replace_fn = crate::v8_utils::v8_fn(scope, &replace_tmpl);
            let replace_key = crate::v8_utils::v8_string(scope, "replaceChild");
            obj.set(scope, replace_key.into(), replace_fn.into());

            // Install insertBefore
            let ib_tmpl = v8::FunctionTemplate::builder_raw(insert_before_callback).build(scope);
            let ib_fn = crate::v8_utils::v8_fn(scope, &ib_tmpl);
            let ib_key = crate::v8_utils::v8_string(scope, "insertBefore");
            obj.set(scope, ib_key.into(), ib_fn.into());

            // Install addEventListener
            let ael_tmpl = v8::FunctionTemplate::builder_raw(add_event_listener_callback).build(scope);
            let ael_fn = crate::v8_utils::v8_fn(scope, &ael_tmpl);
            let ael_key = crate::v8_utils::v8_string(scope, "addEventListener");
            obj.set(scope, ael_key.into(), ael_fn.into());

            // Install removeEventListener
            let rel_tmpl = v8::FunctionTemplate::builder_raw(remove_event_listener_callback).build(scope);
            let rel_fn = crate::v8_utils::v8_fn(scope, &rel_tmpl);
            let rel_key = crate::v8_utils::v8_string(scope, "removeEventListener");
            obj.set(scope, rel_key.into(), rel_fn.into());

            // Install dispatchEvent
            let de_tmpl = v8::FunctionTemplate::builder_raw(dispatch_event_callback).build(scope);
            let de_fn = crate::v8_utils::v8_fn(scope, &de_tmpl);
            let de_key = crate::v8_utils::v8_string(scope, "dispatchEvent");
            obj.set(scope, de_key.into(), de_fn.into());

            // Install innerHTML getter (as a method for now — proper getter needs accessor)
            let ih_tmpl = v8::FunctionTemplate::builder_raw(inner_html_getter_callback).build(scope);
            let ih_fn = crate::v8_utils::v8_fn(scope, &ih_tmpl);
            let ih_key = crate::v8_utils::v8_string(scope, "__getInnerHTML__");
            obj.set(scope, ih_key.into(), ih_fn.into());

            // Install innerHTML setter
            let ihs_tmpl = v8::FunctionTemplate::builder_raw(inner_html_setter_callback).build(scope);
            let ihs_fn = crate::v8_utils::v8_fn(scope, &ihs_tmpl);
            let ihs_key = crate::v8_utils::v8_string(scope, "__setInnerHTML__");
            obj.set(scope, ihs_key.into(), ihs_fn.into());

            // Install insertAdjacentHTML
            let iah_tmpl = v8::FunctionTemplate::builder_raw(insert_adjacent_html_callback).build(scope);
            let iah_fn = crate::v8_utils::v8_fn(scope, &iah_tmpl);
            let iah_key = crate::v8_utils::v8_string(scope, "insertAdjacentHTML");
            obj.set(scope, iah_key.into(), iah_fn.into());

            // Install outerHTML getter
            let oh_tmpl = v8::FunctionTemplate::builder_raw(outer_html_getter_callback).build(scope);
            let oh_fn = crate::v8_utils::v8_fn(scope, &oh_tmpl);
            let oh_key = crate::v8_utils::v8_string(scope, "__getOuterHTML__");
            obj.set(scope, oh_key.into(), oh_fn.into());

            // Install textContent setter (as method)
            let tcs_tmpl = v8::FunctionTemplate::builder_raw(text_content_setter_callback).build(scope);
            let tcs_fn = crate::v8_utils::v8_fn(scope, &tcs_tmpl);
            let tcs_key = crate::v8_utils::v8_string(scope, "__setTextContent__");
            obj.set(scope, tcs_key.into(), tcs_fn.into());
        }
        NodeData::Text(text) => {
            let text_key = crate::v8_utils::v8_string(scope, "textContent");
            let text_val = crate::v8_utils::v8_string(scope, text);
            obj.set(scope, text_key.into(), text_val.into());
        }
        _ => {}
    }

    // Set prototype chain (if __setNodePrototype__ is available)
    let global = scope.get_current_context().global(scope);
    let proto_key = crate::v8_utils::v8_string(scope, "__setNodePrototype__");
    if let Some(proto_fn) = global.get(scope, proto_key.into()) {
        if proto_fn.is_function() && !proto_fn.is_undefined() {
            let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(proto_fn) };
            let undefined = v8::undefined(scope);
            func.call(scope, undefined.into(), &[obj.into()]);
        }
    }

    // Store in identity cache
    let global_obj = v8::Global::new(scope, obj);
    state.node_cache.borrow_mut().insert(node_id, global_obj);

    Some(obj.into())}

/// Convert a list of NodeIds to a V8 array of node objects.
fn node_list_to_v8_array<'s>(
    scope: &v8::PinScope<'s, '_>,
    state: &RuntimeState,
    node_ids: &[NodeId],
) -> v8::Local<'s, v8::Value> {
    let arr = v8::Array::new(scope, node_ids.len() as i32);
    for (i, &nid) in node_ids.iter().enumerate() {
        if let Some(obj) = node_to_v8_object(scope, state, nid) {
            arr.set_index(scope, i as u32, obj);
        }
    }
    arr.into()
}

// ─── V8 Callbacks ───────────────────────────────────────────────────────────

/// document.getElementById(id) callback
unsafe extern "C" fn get_element_by_id(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            return;
        }

        let id_arg = args.get(0);
        if !id_arg.is_string() {
            return;
        }
        let id_str = id_arg.to_rust_string_lossy(scope);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        // Borrow, find node_id, then release borrow before calling node_to_v8_object
        let node_id = {
            let doc = state.document.borrow();
            doc.as_ref().and_then(|d| d.get_element_by_id(&id_str))
        };

        if let Some(nid) = node_id {
            if let Some(obj) = node_to_v8_object(scope, state, nid) {
                rv.set(obj);
            }
        }
        // If not found, return value stays undefined → caller gets null via JS semantics
        // Actually we should set null explicitly
        else {
            rv.set(v8::null(scope).into());
        }
    }));
    if result.is_err() {
        tracing::error!("panic in getElementById callback");
    }
}

/// document.querySelector(selector) callback
unsafe extern "C" fn query_selector(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            return;
        }

        let sel_arg = args.get(0);
        let sel_str = sel_arg.to_rust_string_lossy(scope);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let node_id = {
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                doc.query_selector(&sel_str).ok().flatten()
            } else {
                None
            }
        };

        if let Some(nid) = node_id {
            if let Some(obj) = node_to_v8_object(scope, state, nid) {
                rv.set(obj);
            }
        } else {
            rv.set(v8::null(scope).into());
        }
    }));
    if result.is_err() {
        tracing::error!("panic in querySelector callback");
    }
}

/// document.querySelectorAll(selector) callback
unsafe extern "C" fn query_selector_all(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            rv.set(v8::Array::new(scope, 0).into());
            return;
        }

        let sel_arg = args.get(0);
        let sel_str = sel_arg.to_rust_string_lossy(scope);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let node_ids = {
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                doc.query_selector_all(&sel_str).unwrap_or_default()
            } else {
                vec![]
            }
        };

        let arr = node_list_to_v8_array(scope, state, &node_ids);
        rv.set(arr);
    }));
    if result.is_err() {
        tracing::error!("panic in querySelectorAll callback");
    }
}

/// document.getElementsByTagName(tag) callback
unsafe extern "C" fn get_elements_by_tag_name(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            rv.set(v8::Array::new(scope, 0).into());
            return;
        }

        let tag_arg = args.get(0);
        let tag_str = tag_arg.to_rust_string_lossy(scope);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let node_ids = {
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                doc.get_elements_by_tag_name(&tag_str)
            } else {
                vec![]
            }
        };

        let arr = node_list_to_v8_array(scope, state, &node_ids);
        rv.set(arr);
    }));
    if result.is_err() {
        tracing::error!("panic in getElementsByTagName callback");
    }
}

/// document.getElementsByClassName(className) callback
unsafe extern "C" fn get_elements_by_class_name(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            rv.set(v8::Array::new(scope, 0).into());
            return;
        }

        let class_arg = args.get(0);
        let class_str = class_arg.to_rust_string_lossy(scope);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let node_ids = {
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                // Direct class matching — avoid CSS selector injection
                // Split input by whitespace (multiple classes = AND match)
                let target_classes: Vec<&str> = class_str.split_whitespace().collect();
                if target_classes.is_empty() {
                    vec![]
                } else {
                    let mut results = Vec::new();
                    for node_ref in doc.tree.root().descendants() {
                        let classes = node_ref.value().class_list();
                        if target_classes.iter().all(|tc| classes.iter().any(|c| c == tc)) {
                            results.push(node_ref.id());
                        }
                    }
                    results
                }
            } else {
                vec![]
            }
        };

        let arr = node_list_to_v8_array(scope, state, &node_ids);
        rv.set(arr);
    }));
    if result.is_err() {
        tracing::error!("panic in getElementsByClassName callback");
    }
}

/// getAttribute(name) callback — reads from __attrs__ hidden property on `this`.
unsafe extern "C" fn get_attribute_callback(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            rv.set(v8::null(scope).into());
            return;
        }

        let attr_name = args.get(0);
        let this = args.this();

        let attrs_key = crate::v8_utils::v8_string(scope, "__attrs__");
        if let Some(attrs_val) = this.get(scope, attrs_key.into()) {
            if attrs_val.is_object() {
                let attrs_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(attrs_val) };
                if let Some(val) = attrs_obj.get(scope, attr_name) {
                    if !val.is_undefined() {
                        rv.set(val);
                        return;
                    }
                }
            }
        }
        rv.set(v8::null(scope).into());
    }));
    if result.is_err() {
        tracing::error!("panic in getAttribute callback");
    }
}


/// document.createElement(tag) callback
unsafe extern "C" fn create_element(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            return;
        }

        let tag_arg = args.get(0);
        let tag_str = tag_arg.to_rust_string_lossy(scope);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let node_id = {
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                let root_id = doc.root_id();
                let data = NodeData::element(&tag_str, "http://www.w3.org/1999/xhtml", vec![]);
                let nid = doc.append_child(root_id, data);
                // Detach immediately — it's an orphan until appendChild
                doc.detach(nid);
                Some(nid)
            } else {
                None
            }
        };

        if let Some(nid) = node_id {
            if let Some(obj) = node_to_v8_object(scope, state, nid) {
                rv.set(obj);
            }
        }
    }));
    if result.is_err() {
        tracing::error!("panic in createElement callback");
    }
}

/// element.appendChild(child) callback
unsafe extern "C" fn append_child_callback(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            return;
        }

        let this = args.this();
        let child_arg = args.get(0);

        if !child_arg.is_object() {
            return;
        }
        let child_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(child_arg) };

        let parent_id = extract_node_id(scope, this);
        let child_id = extract_node_id(scope, child_obj);

        if let (Some(pid), Some(cid)) = (parent_id, child_id) {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                // Detach child from current parent (if any), then append to new parent
                doc.detach(cid);
                if let Some(mut parent) = doc.tree.get_mut(pid) {
                    parent.append_id(cid);
                }
                doc.invalidate_tag_index();
                // Rebuild id index to pick up any id attributes in the appended subtree
                doc.rebuild_id_index();
            }
        }

        // Return the child (per DOM spec)
        rv.set(child_arg);
    }));
    if result.is_err() {
        tracing::error!("panic in appendChild callback");
    }
}

/// element.removeChild(child) callback
unsafe extern "C" fn remove_child_callback(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            return;
        }

        let child_arg = args.get(0);
        if !child_arg.is_object() {
            return;
        }
        let child_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(child_arg) };
        let child_id = extract_node_id(scope, child_obj);

        if let Some(cid) = child_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                doc.detach(cid);
            }
        }

        // Return the removed child (per DOM spec)
        rv.set(child_arg);
    }));
    if result.is_err() {
        tracing::error!("panic in removeChild callback");
    }
}

/// element.replaceChild(newChild, oldChild) callback
unsafe extern "C" fn replace_child_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 2 { return; }

        let new_child_arg = args.get(0);
        let old_child_arg = args.get(1);

        if !new_child_arg.is_object() || !old_child_arg.is_object() { return; }

        let new_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(new_child_arg) };
        let old_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(old_child_arg) };

        let new_id = extract_node_id(scope, new_obj);
        let old_id = extract_node_id(scope, old_obj);

        if let (Some(nid), Some(oid)) = (new_id, old_id) {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                // Move new node before old node, then detach old node
                doc.move_before_sibling(nid, oid);
                doc.detach(oid);
            }
        }

        // Return the old child (per DOM spec)
        rv.set(old_child_arg);
    }));
}

/// element.insertBefore(newNode, referenceNode) callback
unsafe extern "C" fn insert_before_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 { return; }

        let this = args.this();
        let new_child_arg = args.get(0);
        if !new_child_arg.is_object() { return; }

        let new_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(new_child_arg) };
        let parent_id = extract_node_id(scope, this);
        let new_id = extract_node_id(scope, new_obj);

        if let (Some(pid), Some(nid)) = (parent_id, new_id) {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);

            // Reference node (2nd arg) — if null/undefined, append to end
            if args.length() >= 2 && args.get(1).is_object() {
                let ref_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(args.get(1)) };
                if let Some(ref_id) = extract_node_id(scope, ref_obj) {
                    let mut doc = state.document.borrow_mut();
                    if let Some(ref mut doc) = *doc {
                        doc.move_before_sibling(nid, ref_id);
                    }
                }
            } else {
                // No reference node → append to parent
                let mut doc = state.document.borrow_mut();
                if let Some(ref mut doc) = *doc {
                    doc.move_to_parent(nid, pid);
                }
            }
        }

        rv.set(new_child_arg);
    }));
}

/// element.removeAttribute(name) callback
unsafe extern "C" fn remove_attribute_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 1 { return; }

        let this = args.this();
        let name_arg = args.get(0).to_rust_string_lossy(scope);
        let node_id = extract_node_id(scope, this);

        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                if let Some(mut node) = doc.tree.get_mut(nid) {
                    if let NodeData::Element { ref mut attrs, ref mut id, ref mut classes, .. } = node.value() {
                        attrs.retain(|(k, _)| k != &name_arg);
                        if name_arg == "id" { *id = None; }
                        if name_arg == "class" { classes.clear(); }
                    }
                }
                if name_arg == "id" {
                    // Remove from id index
                    doc.rebuild_id_index();
                }
            }
        }

        // Also update __attrs__ on the JS object
        let attrs_key = crate::v8_utils::v8_string(scope, "__attrs__");
        if let Some(attrs_val) = this.get(scope, attrs_key.into()) {
            if attrs_val.is_object() {
                let attrs_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(attrs_val) };
                if let Some(k) = v8::String::new(scope, &name_arg) {
                    attrs_obj.delete(scope, k.into());
                }
            }
        }
    }));
}

/// element.hasAttribute(name) callback
unsafe extern "C" fn has_attribute_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            rv.set(v8::Boolean::new(scope, false).into());
            return;
        }

        let this = args.this();
        let name_arg = args.get(0).to_rust_string_lossy(scope);
        let node_id = extract_node_id(scope, this);

        let has = if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                doc.get(nid).map(|n| n.value().get_attr(&name_arg).is_some()).unwrap_or(false)
            } else { false }
        } else { false };

        rv.set(v8::Boolean::new(scope, has).into());
    }));
}

/// element.insertAdjacentHTML(position, html) callback
unsafe extern "C" fn insert_adjacent_html_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 2 { return; }

        let this = args.this();
        let position = args.get(0).to_rust_string_lossy(scope).to_lowercase();
        let html_str = args.get(1).to_rust_string_lossy(scope);
        let node_id = extract_node_id(scope, this);

        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                // Parse the HTML fragment
                let fragment = crate::dom::parse_html(&html_str, None);
                let body_id = fragment.body().unwrap_or(fragment.root_id());

                let frag_children: Vec<(crate::dom::NodeId, crate::dom::NodeData)> = {
                    fragment.tree.get(body_id)
                        .map(|body| body.children().map(|c| (c.id(), c.value().clone())).collect())
                        .unwrap_or_default()
                };

                match position.as_str() {
                    "beforebegin" => {
                        // Insert before this element (as previous sibling)
                        if let Some(parent_id) = doc.tree.get(nid).and_then(|n| n.parent()).map(|p| p.id()) {
                            for (frag_id, _) in &frag_children {
                                let frag_data = fragment.tree.get(*frag_id).map(|n| n.value().clone());
                                if let Some(data) = frag_data {
                                    doc.insert_before(nid, data);
                                }
                            }
                            let _ = parent_id; // used for context validation
                        }
                    }
                    "afterbegin" => {
                        // Insert as first children of this element
                        // Get current first child
                        let first_child = doc.tree.get(nid).and_then(|n| n.first_child()).map(|c| c.id());
                        for (frag_id, _) in frag_children.iter().rev() {
                            let frag_data = fragment.tree.get(*frag_id).map(|n| n.value().clone());
                            if let Some(data) = frag_data {
                                if let Some(fc) = first_child {
                                    // Insert before first child
                                    doc.insert_before(fc, data);
                                } else {
                                    // No children, just append
                                    doc.append_child(nid, data);
                                }
                            }
                        }
                    }
                    "beforeend" => {
                        // Append as last children of this element
                        for (frag_id, _) in &frag_children {
                            append_node_recursive(doc, nid, &fragment, *frag_id);
                        }
                    }
                    "afterend" => {
                        // Insert after this element (before next sibling)
                        // Simplified: append to parent
                        if let Some(parent_id) = doc.tree.get(nid).and_then(|n| n.parent()).map(|p| p.id()) {
                            for (frag_id, _) in &frag_children {
                                append_node_recursive(doc, parent_id, &fragment, *frag_id);
                            }
                        }
                    }
                    _ => {}
                }

                doc.invalidate_tag_index();
                doc.rebuild_id_index();
            }
        }
    }));
}

/// element.setAttribute(name, value) callback
unsafe extern "C" fn set_attribute_callback(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 2 {
            return;
        }

        let this = args.this();
        let name_arg = args.get(0).to_rust_string_lossy(scope);
        let value_arg = args.get(1).to_rust_string_lossy(scope);

        let node_id = extract_node_id(scope, this);
        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                if let Some(mut node) = doc.tree.get_mut(nid) {
                    if let NodeData::Element { ref mut attrs, ref mut id, ref mut classes, .. } = node.value() {
                        // Update or add the attribute
                        if let Some(existing) = attrs.iter_mut().find(|(k, _)| k == &name_arg) {
                            existing.1 = value_arg.clone();
                        } else {
                            attrs.push((name_arg.clone(), value_arg.clone()));
                        }
                        // Update cached id/classes
                        if name_arg == "id" {
                            *id = Some(value_arg.clone());
                        }
                        if name_arg == "class" {
                            *classes = value_arg.split_whitespace().map(|s| s.to_string()).collect();
                        }
                    }
                }
                // Re-register id if changed
                if name_arg == "id" {
                    doc.register_id(value_arg.clone(), nid);
                }
            }
        }

        // Also update the __attrs__ object on `this` for getAttribute consistency
        let attrs_key = crate::v8_utils::v8_string(scope, "__attrs__");
        if let Some(attrs_val) = this.get(scope, attrs_key.into()) {
            if attrs_val.is_object() {
                let attrs_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(attrs_val) };
                if let Some(k) = v8::String::new(scope, &name_arg) {
                    if let Some(v) = v8::String::new(scope, &value_arg) {
                        attrs_obj.set(scope, k.into(), v.into());
                    }
                }
            }
        }
    }));
    if result.is_err() {
        tracing::error!("panic in setAttribute callback");
    }
}


/// element.addEventListener(type, listener, options?) callback
unsafe extern "C" fn add_event_listener_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 2 {
            return;
        }

        let this = args.this();
        let event_type = args.get(0).to_rust_string_lossy(scope);
        let listener_arg = args.get(1);

        if !listener_arg.is_function() {
            return;
        }

        let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(listener_arg) };
        let global_fn = v8::Global::new(scope, func);

        // Parse options (3rd arg): boolean (capture) or object {capture, once}
        let mut capture = false;
        let mut once = false;
        if args.length() >= 3 {
            let opts = args.get(2);
            if opts.is_boolean() {
                capture = opts.is_true();
            } else if opts.is_object() {
                let opts_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(opts) };
                if let Some(cap_key) = v8::String::new(scope, "capture") {
                    if let Some(cap_val) = opts_obj.get(scope, cap_key.into()) {
                        capture = cap_val.is_true();
                    }
                }
                if let Some(once_key) = v8::String::new(scope, "once") {
                    if let Some(once_val) = opts_obj.get(scope, once_key.into()) {
                        once = once_val.is_true();
                    }
                }
            }
        }

        let node_id = extract_node_id(scope, this);
        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            state
                .event_listeners
                .borrow_mut()
                .add(nid, &event_type, global_fn, capture, once);
        }
    }));
}

/// element.removeEventListener(type, listener, options?) callback
unsafe extern "C" fn remove_event_listener_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 2 || !args.get(1).is_function() {
            return;
        }

        let this = args.this();
        let event_type = args.get(0).to_rust_string_lossy(scope);
        let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(args.get(1)) };
        let capture = if args.length() >= 3 { args.get(2).is_true() } else { false };

        let node_id = extract_node_id(scope, this);
        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            state.event_listeners.borrow_mut().remove_by_callback(scope, nid, &event_type, func, capture);
        }
    }));
}

/// element.dispatchEvent(event) callback
/// event can be a string (event type) or an object with {type, bubbles}
unsafe extern "C" fn dispatch_event_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            rv.set(v8::Boolean::new(scope, true).into());
            return;
        }

        let this = args.this();
        let event_arg = args.get(0);

        // Extract event type and bubbles
        let (event_type, bubbles) = if event_arg.is_string() {
            (event_arg.to_rust_string_lossy(scope), true)
        } else if event_arg.is_object() {
            let evt_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(event_arg) };
            let type_key = crate::v8_utils::v8_string(scope, "type");
            let event_type = evt_obj
                .get(scope, type_key.into())
                .map(|v| v.to_rust_string_lossy(scope))
                .unwrap_or_default();
            let bubbles_key = crate::v8_utils::v8_string(scope, "bubbles");
            let bubbles = evt_obj
                .get(scope, bubbles_key.into())
                .map(|v| v.is_true())
                .unwrap_or(true);
            (event_type, bubbles)
        } else {
            rv.set(v8::Boolean::new(scope, true).into());
            return;
        };

        let node_id = extract_node_id(scope, this);
        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);

            let result = {
                let doc = state.document.borrow();
                if let Some(ref doc) = *doc {
                    crate::events::target::dispatch_event(
                        scope,
                        &state.event_listeners,
                        doc,
                        nid,
                        &event_type,
                        bubbles,
                    )
                } else {
                    true
                }
            };

            rv.set(v8::Boolean::new(scope, result).into());
        } else {
            rv.set(v8::Boolean::new(scope, true).into());
        }
    }));
}


/// element.innerHTML getter callback
unsafe extern "C" fn inner_html_getter_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let this = args.this();
        let node_id = extract_node_id(scope, this);
        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                let html = doc.inner_html(nid);
                if let Some(s) = v8::String::new(scope, &html) {
                    rv.set(s.into());
                    return;
                }
            }
        }
        let empty = crate::v8_utils::v8_string(scope, "");
        rv.set(empty.into());
    }));
}

/// element.outerHTML getter callback
unsafe extern "C" fn outer_html_getter_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let this = args.this();
        let node_id = extract_node_id(scope, this);
        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                let html = doc.outer_html(nid);
                if let Some(s) = v8::String::new(scope, &html) {
                    rv.set(s.into());
                    return;
                }
            }
        }
        let empty = crate::v8_utils::v8_string(scope, "");
        rv.set(empty.into());
    }));
}

/// element.textContent setter callback — clears children and sets text
unsafe extern "C" fn text_content_setter_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 1 { return; }

        let this = args.this();
        let text_val = args.get(0).to_rust_string_lossy(scope);
        let node_id = extract_node_id(scope, this);

        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                // Remove all children
                let children: Vec<crate::dom::NodeId> = doc.tree.get(nid)
                    .map(|n| n.children().map(|c| c.id()).collect())
                    .unwrap_or_default();
                for child_id in children {
                    doc.detach(child_id);
                }
                // Add new text node
                doc.append_child(nid, NodeData::text(&text_val));
            }
        }
    }));
}


/// element.innerHTML setter callback — parses HTML and replaces children.
unsafe extern "C" fn inner_html_setter_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 1 { return; }

        let this = args.this();
        let html_str = args.get(0).to_rust_string_lossy(scope);
        let node_id = extract_node_id(scope, this);

        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                // 1. Remove all existing children
                let children: Vec<crate::dom::NodeId> = doc.tree.get(nid)
                    .map(|n| n.children().map(|c| c.id()).collect())
                    .unwrap_or_default();
                for child_id in children {
                    doc.detach(child_id);
                }

                // 2. Parse the HTML fragment
                let fragment = crate::dom::parse_html(&html_str, None);

                // 3. Move parsed nodes into the target
                // The parsed fragment has html/head/body structure.
                // For innerHTML, we want the body's children (or all root children).
                let body_children: Vec<(crate::dom::NodeId, crate::dom::NodeData)> = {
                    // Find body in the fragment, get its children's data
                    let body_id = fragment.body().unwrap_or(fragment.root_id());
                    fragment.tree.get(body_id)
                        .map(|body| {
                            body.children()
                                .map(|c| (c.id(), c.value().clone()))
                                .collect()
                        })
                        .unwrap_or_default()
                };

                // 4. Append cloned nodes to target
                for (_frag_id, _node_data) in body_children {
                    append_node_recursive(doc, nid, &fragment, _frag_id);
                }

                doc.invalidate_tag_index();
                doc.rebuild_id_index();
            }
        }
    }));
}

/// Recursively append a node and its children from a source document to a target.
pub fn append_node_recursive(
    doc: &mut crate::dom::Document,
    parent_id: crate::dom::NodeId,
    source: &crate::dom::Document,
    source_node_id: crate::dom::NodeId,
) {
    if let Some(source_node) = source.tree.get(source_node_id) {
        let data = source_node.value().clone();
        let new_id = doc.append_child(parent_id, data);

        // Recursively append children
        let child_ids: Vec<crate::dom::NodeId> = source_node
            .children()
            .map(|c| c.id())
            .collect();
        for child_id in child_ids {
            append_node_recursive(doc, new_id, source, child_id);
        }
    }
}
