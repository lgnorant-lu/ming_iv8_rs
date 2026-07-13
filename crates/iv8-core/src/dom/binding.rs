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

/// Extract NodeId from a V8 object.
/// Prefer V8 internal field (DOM FunctionTemplate instances). Own-property
/// `__nodeId__` is fallback only for the plain-object path (no templates).
pub fn extract_node_id(scope: &v8::PinScope<'_, '_>, obj: v8::Local<v8::Object>) -> Option<NodeId> {
    if let Some(nid) = crate::dom::template::extract_node_id_from_internal(scope, obj) {
        return Some(nid);
    }
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
    let isolate: &v8::Isolate = &*scope;
    let state = RuntimeState::get(isolate);

    let root_id = {
        let doc = state.document.borrow();
        doc.as_ref().map(|d| d.root_id())
    };

    let mut used_internal_field = false;
    let doc_obj: v8::Local<v8::Object> = if let Some(root_id) = root_id {
        if let Some(ref templates) = *state.dom_templates.borrow() {
            let doc_tmpl = v8::Local::new(scope, &templates.document_node);
            let inst_tmpl = doc_tmpl.instance_template(scope);
            if let Some(obj) = inst_tmpl.new_instance(scope) {
                let nid_usize = node_id_to_usize(root_id);
                let external = v8::External::new(scope, nid_usize as *mut std::ffi::c_void);
                obj.set_internal_field(crate::dom::template::NODE_ID_FIELD as usize, external.into());
                used_internal_field = true;
                obj
            } else {
                v8::Object::new(scope)
            }
        } else {
            v8::Object::new(scope)
        }
    } else {
        v8::Object::new(scope)
    };

    // Tree-backed Document ops must land on every prototype the document may
    // inherit (FT instance [[Prototype]] and/or global Document.prototype).
    // Codegen no longer installs these (RD-16); dual targets close FT vs global
    // prototype splits after freeze/chain.
    let mut targets: Vec<v8::Local<v8::Object>> = Vec::with_capacity(2);
    if let Some(from_instance) = doc_obj.get_prototype(scope).and_then(|p| {
        if p.is_object() && !p.is_null_or_undefined() {
            p.to_object(scope)
        } else {
            None
        }
    }) {
        targets.push(from_instance);
    }
    if let Some(global_proto) = document_prototype(scope, global) {
        let already = targets
            .iter()
            .any(|t| t.strict_equals(global_proto.into()));
        if !already {
            targets.push(global_proto);
        }
    }
    if targets.is_empty() {
        targets.push(doc_obj);
    }

    for method_target in targets {
        install_method(scope, method_target, "getElementById", get_element_by_id, 1);
        install_method(scope, method_target, "querySelector", query_selector, 1);
        install_method(scope, method_target, "querySelectorAll", query_selector_all, 1);
        install_method(scope, method_target, "getElementsByTagName", get_elements_by_tag_name, 1);
        install_method(scope, method_target, "getElementsByClassName", get_elements_by_class_name, 1);
        install_method(scope, method_target, "createElement", create_element, 1);
        install_method(scope, method_target, "createElementNS", create_element_ns, 2);
        install_method(scope, method_target, "createTextNode", create_text_node, 1);
        install_method(scope, method_target, "createComment", create_comment, 1);
        install_method(scope, method_target, "createDocumentFragment", create_document_fragment, 0);
        install_method(scope, method_target, "addEventListener", add_event_listener_callback, 2);
        install_method(scope, method_target, "removeEventListener", remove_event_listener_callback, 2);
        install_method(scope, method_target, "dispatchEvent", dispatch_event_callback, 1);
        install_method(scope, method_target, "elementFromPoint", element_from_point_cb, 2);
        install_method(scope, method_target, "caretPositionFromPoint", caret_position_from_point_cb, 2);
        install_method(scope, method_target, "exitPictureInPicture", document_resolved_void_promise, 0);
        install_method(scope, method_target, "hasUnpartitionedCookieAccess", document_resolved_false_promise, 0);
        install_method(scope, method_target, "hasStorageAccess", document_resolved_false_promise, 0);
        install_method(scope, method_target, "requestStorageAccess", document_resolved_void_promise, 0);
        install_method(scope, method_target, "hasPrivateToken", document_resolved_false_promise, 1);
        install_method(scope, method_target, "hasRedemptionRecord", document_resolved_false_promise, 1);
        install_method(scope, method_target, "createTreeWalker", create_tree_walker_cb, 1);
        install_method(scope, method_target, "createExpression", create_expression_cb, 1);

        install_doc_accessor(scope, method_target, "documentElement", doc_document_element);
        // body is settable in HTML (Element?); accept Element or no-op on invalid
        install_doc_accessor_rw(scope, method_target, "body", doc_body, Some(doc_body_setter));
        install_doc_accessor(scope, method_target, "head", doc_head);
        install_doc_accessor_rw(scope, method_target, "title", doc_title, Some(doc_title_setter));
        install_doc_accessor(scope, method_target, "URL", doc_url);
        install_doc_accessor(scope, method_target, "documentURI", doc_url);
        install_doc_accessor(scope, method_target, "location", doc_location);
    }

    // Single identity when FT instance is used: internal field only.
    // Own `__nodeId__` only for plain-object fallback (no templates).
    if !used_internal_field {
        let isolate: &v8::Isolate = scope;
        let state = RuntimeState::get(isolate);
        let root_id_opt = state.document.borrow().as_ref().map(|doc| doc.root_id());
        if let Some(root_id) = root_id_opt {
            let id_key = crate::v8_utils::v8_string(scope, "__nodeId__");
            let nz: std::num::NonZeroUsize = unsafe { std::mem::transmute(root_id) };
            let id_val = v8::Number::new(scope, nz.get() as f64);
            doc_obj.define_own_property(
                scope,
                id_key.into(),
                id_val.into(),
                v8::PropertyAttribute::DONT_ENUM | v8::PropertyAttribute::DONT_DELETE,
            );
        }
    }

    let key = crate::v8_utils::v8_string(scope, "document");
    global.set(scope, key.into(), doc_obj.into());

    // TreeWalker.prototype traversal (overrides codegen null stubs).
    install_tree_walker_prototype_ops(scope, global);
    // XPathExpression.evaluate + Document.evaluate (subset over real tree).
    install_xpath_prototype_ops(scope, global);
    // HTMLIFrameElement readonly attrs: no-op setters (H05b Category C).
    install_iframe_readonly_noop_setters(scope, global);
}

/// Ensure HTMLIFrameElement.prototype readonly attrs have no-op setters so
/// assignment does not create shadowing data properties.
/// Must run before `freeze_all_prototypes` (frozen protos reject redefine).
pub fn install_iframe_readonly_noop_setters(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
) {
    let Some(proto) = document_prototype_named(scope, global, "HTMLIFrameElement") else {
        return;
    };
    for name in [
        "contentDocument",
        "contentWindow",
        "permissionsPolicy",
        "sandbox",
        "featurePolicy",
    ] {
        // Keep existing getter if present; always attach no-op setter.
        let key = crate::v8_utils::v8_string(scope, name);
        let existing = object_get_own_property_descriptor(scope, proto, key);
        let getter_fn = existing.and_then(|(g, _)| g);
        let desc = v8::Object::new(scope);
        let get_key = crate::v8_utils::v8_string(scope, "get");
        let set_key = crate::v8_utils::v8_string(scope, "set");
        let enum_key = crate::v8_utils::v8_string(scope, "enumerable");
        let conf_key = crate::v8_utils::v8_string(scope, "configurable");
        if let Some(g) = getter_fn {
            desc.set(scope, get_key.into(), g.into());
        } else {
            let g_tmpl =
                v8::FunctionTemplate::builder_raw(iframe_readonly_null_getter).build(scope);
            let g = crate::v8_utils::v8_fn(scope, &g_tmpl);
            desc.set(scope, get_key.into(), g.into());
        }
        let s_tmpl = v8::FunctionTemplate::builder_raw(doc_readonly_noop_setter).build(scope);
        let s = crate::v8_utils::v8_fn(scope, &s_tmpl);
        desc.set(scope, set_key.into(), s.into());
        desc.set(scope, enum_key.into(), v8::Boolean::new(scope, true).into());
        desc.set(scope, conf_key.into(), v8::Boolean::new(scope, true).into());
        let obj_key = crate::v8_utils::v8_string(scope, "Object");
        if let Some(obj_ctor) = global.get(scope, obj_key.into()) {
            if obj_ctor.is_object() {
                let obj_ctor: v8::Local<v8::Object> =
                    unsafe { v8::Local::cast_unchecked(obj_ctor) };
                let def_key = crate::v8_utils::v8_string(scope, "defineProperty");
                if let Some(def_prop) = obj_ctor.get(scope, def_key.into()) {
                    if def_prop.is_function() {
                        let def_fn: v8::Local<v8::Function> =
                            unsafe { v8::Local::cast_unchecked(def_prop) };
                        let undefined = v8::undefined(scope);
                        def_fn.call(
                            scope,
                            undefined.into(),
                            &[proto.into(), key.into(), desc.into()],
                        );
                    }
                }
            }
        }
    }
}

fn object_get_own_property_descriptor<'s>(
    scope: &v8::PinScope<'s, '_>,
    obj: v8::Local<'s, v8::Object>,
    key: v8::Local<'s, v8::String>,
) -> Option<(
    Option<v8::Local<'s, v8::Function>>,
    Option<v8::Local<'s, v8::Function>>,
)> {
    let global = scope.get_current_context().global(scope);
    let obj_key = crate::v8_utils::v8_string(scope, "Object");
    let obj_ctor = global.get(scope, obj_key.into())?;
    if !obj_ctor.is_object() {
        return None;
    }
    let obj_ctor: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(obj_ctor) };
    let gopd_key = crate::v8_utils::v8_string(scope, "getOwnPropertyDescriptor");
    let gopd = obj_ctor.get(scope, gopd_key.into())?;
    if !gopd.is_function() {
        return None;
    }
    let gopd_fn: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(gopd) };
    let undefined = v8::undefined(scope);
    let desc_val = gopd_fn.call(scope, undefined.into(), &[obj.into(), key.into()])?;
    if !desc_val.is_object() {
        return None;
    }
    let desc: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(desc_val) };
    let get_key = crate::v8_utils::v8_string(scope, "get");
    let set_key = crate::v8_utils::v8_string(scope, "set");
    let getter = desc.get(scope, get_key.into()).and_then(|v| {
        if v.is_function() {
            Some(unsafe { v8::Local::<v8::Function>::cast_unchecked(v) })
        } else {
            None
        }
    });
    let setter = desc.get(scope, set_key.into()).and_then(|v| {
        if v.is_function() {
            Some(unsafe { v8::Local::<v8::Function>::cast_unchecked(v) })
        } else {
            None
        }
    });
    Some((getter, setter))
}

unsafe extern "C" fn iframe_readonly_null_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::null(scope).into());
    }));
}

fn document_prototype<'s>(
    scope: &v8::PinScope<'s, '_>,
    global: v8::Local<'s, v8::Object>,
) -> Option<v8::Local<'s, v8::Object>> {
    let doc_key = crate::v8_utils::v8_string(scope, "Document");
    let ctor_val = global.get(scope, doc_key.into())?;
    if !ctor_val.is_function() {
        return None;
    }
    let ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(ctor_val) };
    let proto_key = crate::v8_utils::v8_string(scope, "prototype");
    let proto_val = ctor.get(scope, proto_key.into())?;
    proto_val.to_object(scope)
}

/// Helper to install a native accessor on an object.
/// `setter = None` → readonly (set is no-op so assignment does not create a
/// shadowing data property — H05b Category C).
fn install_doc_accessor(
    scope: &v8::PinScope<'_, '_>,
    obj: v8::Local<v8::Object>,
    name: &str,
    getter: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
) {
    install_doc_accessor_rw(scope, obj, name, getter, Some(doc_readonly_noop_setter));
}

fn install_doc_accessor_rw(
    scope: &v8::PinScope<'_, '_>,
    obj: v8::Local<v8::Object>,
    name: &str,
    getter: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
    setter: Option<unsafe extern "C" fn(*const v8::FunctionCallbackInfo)>,
) {
    let getter_tmpl = v8::FunctionTemplate::builder_raw(getter).build(scope);
    let getter_fn = crate::v8_utils::v8_fn(scope, &getter_tmpl);
    let name_str = crate::v8_utils::v8_string(scope, name);
    let desc = v8::Object::new(scope);
    let get_key = crate::v8_utils::v8_string(scope, "get");
    let enum_key = crate::v8_utils::v8_string(scope, "enumerable");
    let conf_key = crate::v8_utils::v8_string(scope, "configurable");
    desc.set(scope, get_key.into(), getter_fn.into());
    if let Some(s) = setter {
        let setter_tmpl = v8::FunctionTemplate::builder_raw(s).build(scope);
        let setter_fn = crate::v8_utils::v8_fn(scope, &setter_tmpl);
        let set_key = crate::v8_utils::v8_string(scope, "set");
        desc.set(scope, set_key.into(), setter_fn.into());
    }
    desc.set(scope, enum_key.into(), v8::Boolean::new(scope, true).into());
    desc.set(scope, conf_key.into(), v8::Boolean::new(scope, true).into());
    let global = scope.get_current_context().global(scope);
    let obj_key = crate::v8_utils::v8_string(scope, "Object");
    if let Some(obj_ctor) = global.get(scope, obj_key.into()) {
        if obj_ctor.is_object() {
            let obj_ctor: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(obj_ctor) };
            let def_prop_key = crate::v8_utils::v8_string(scope, "defineProperty");
            if let Some(def_prop) = obj_ctor.get(scope, def_prop_key.into()) {
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

/// Readonly Document attrs: accept set without changing value (no shadow data prop).
unsafe extern "C" fn doc_readonly_noop_setter(info: *const v8::FunctionCallbackInfo) {
    let _ = info;
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

/// document.body setter — HTML allows assigning an HTMLElement body (or null).
/// Invalid values are ignored (no throw) for L3 fidelity without full HTML parser policy.
unsafe extern "C" fn doc_body_setter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if args.length() < 1 {
            return;
        }
        let v = args.get(0);
        if v.is_null_or_undefined() {
            return;
        }
        if !v.is_object() {
            return;
        }
        let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(v) };
        let Some(nid) = extract_node_id(scope, obj) else {
            return;
        };
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let mut doc = state.document.borrow_mut();
        if let Some(ref mut d) = *doc {
            // Only accept body/frameset-like element tags
            let ok = d
                .get(nid)
                .and_then(|n| n.value().tag_name())
                .map(|t| t.eq_ignore_ascii_case("body") || t.eq_ignore_ascii_case("frameset"))
                .unwrap_or(false);
            if !ok {
                return;
            }
            if let Some(html) = d.document_element() {
                // Detach existing body children under html that are body
                if let Some(old) = d.body() {
                    if old != nid {
                        d.detach(old);
                    }
                }
                // Ensure node is under html
                if d.parent_id(nid) != Some(html) {
                    d.move_to_parent(nid, html);
                }
            }
        }
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
                titles
                    .first()
                    .map(|&nid| doc.text_content_of(nid))
                    .unwrap_or_default()
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

/// document.title setter — creates/updates <title> text content.
unsafe extern "C" fn doc_title_setter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if args.length() < 1 {
            return;
        }
        let val = args.get(0).to_rust_string_lossy(scope);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let mut doc = state.document.borrow_mut();
        if let Some(ref mut d) = *doc {
            let titles = d.get_elements_by_tag_name("title");
            if let Some(&tid) = titles.first() {
                // Replace text content of existing <title>
                if let Some(node) = d.tree.get(tid) {
                    let children: Vec<_> = node.children().map(|c| c.id()).collect();
                    drop(node);
                    for cid in children {
                        d.detach(cid);
                    }
                }
                d.append_child(tid, crate::dom::NodeData::text(&val));
            } else if let Some(head) = d.head() {
                let tid = d.append_child(
                    head,
                    crate::dom::NodeData::element(
                        "title",
                        "http://www.w3.org/1999/xhtml",
                        vec![],
                    ),
                );
                d.append_child(tid, crate::dom::NodeData::text(&val));
            }
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
        let loc_key = match v8::String::new(scope, "location") {
            Some(k) => k,
            None => return,
        };
        if let Some(loc_val) = global.get(scope, loc_key.into()) {
            if loc_val.is_object() && !loc_val.is_null_or_undefined() {
                let loc_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(loc_val) };
                let href_key = match v8::String::new(scope, "href") {
                    Some(k) => k,
                    None => return,
                };
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
        let loc_key = match v8::String::new(scope, "location") {
            Some(k) => k,
            None => return,
        };
        if let Some(loc_val) = global.get(scope, loc_key.into()) {
            rv.set(loc_val);
        }
    }));
}

/// No layout engine: hit-testing returns null (DOM allows this).
unsafe extern "C" fn element_from_point_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::null(scope).into());
    }));
}

unsafe extern "C" fn caret_position_from_point_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(v8::null(scope).into());
    }));
}

fn set_resolved_promise(
    scope: &v8::PinScope<'_, '_>,
    rv: &mut v8::ReturnValue<'_>,
    value: v8::Local<'_, v8::Value>,
) {
    let resolver = crate::v8_utils::v8_resolver(scope);
    let _ = resolver.resolve(scope, value);
    rv.set(resolver.get_promise(scope).into());
}

unsafe extern "C" fn document_resolved_void_promise(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        set_resolved_promise(scope, &mut rv, v8::undefined(scope).into());
    }));
}

unsafe extern "C" fn document_resolved_false_promise(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        set_resolved_promise(scope, &mut rv, v8::Boolean::new(scope, false).into());
    }));
}

/// Install real TreeWalker traversal on TreeWalker.prototype (overrides codegen
/// null stubs). Uses __iv8Root / __iv8CurrentNode hidden keys set by createTreeWalker.
fn install_tree_walker_prototype_ops(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    let Some(proto) = document_prototype_named(scope, global, "TreeWalker") else {
        return;
    };
    install_method(scope, proto, "nextNode", tree_walker_next_node_cb, 0);
    install_method(scope, proto, "previousNode", tree_walker_previous_node_cb, 0);
    install_method(scope, proto, "firstChild", tree_walker_first_child_cb, 0);
    install_method(scope, proto, "lastChild", tree_walker_last_child_cb, 0);
    install_method(scope, proto, "parentNode", tree_walker_parent_node_cb, 0);
    install_method(scope, proto, "nextSibling", tree_walker_next_sibling_cb, 0);
    install_method(scope, proto, "previousSibling", tree_walker_previous_sibling_cb, 0);
}

/// XPathExpression.evaluate + Document.evaluate — subset over real DOM tree.
fn install_xpath_prototype_ops(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    if let Some(proto) = document_prototype_named(scope, global, "XPathExpression") {
        install_method(scope, proto, "evaluate", xpath_expression_evaluate_cb, 1);
    }
    // Document.evaluate is tree-backed; override codegen skeleton.
    if let Some(proto) = document_prototype(scope, global) {
        install_method(scope, proto, "evaluate", document_evaluate_cb, 2);
    }
    // Also install on instance prototype if different from global Document.prototype.
    // (Already handled by dual-target install loop for other methods; evaluate was
    // not in EXCLUDED_OPERATIONS until now — install on both via re-set after freeze.)
}

fn make_xpath_result<'s>(
    scope: &v8::PinScope<'s, '_>,
    state: &RuntimeState,
    global: v8::Local<'s, v8::Object>,
    nodes: &[NodeId],
) -> v8::Local<'s, v8::Object> {
    let obj = if let Some(proto) = document_prototype_named(scope, global, "XPathResult") {
        let o = v8::Object::new(scope);
        let _ = o.set_prototype(scope, proto.into());
        o
    } else {
        v8::Object::new(scope)
    };
    // Snapshot array of node objects
    let arr = v8::Array::new(scope, nodes.len() as i32);
    for (i, nid) in nodes.iter().enumerate() {
        if let Some(nobj) = crate::dom::template::create_node_object(scope, state, *nid) {
            let _ = arr.set_index(scope, i as u32, nobj);
        }
    }
    let snap_key = crate::v8_utils::v8_string(scope, "__iv8Snapshot");
    let _ = obj.define_own_property(
        scope,
        snap_key.into(),
        arr.into(),
        v8::PropertyAttribute::DONT_ENUM,
    );
    let idx_key = crate::v8_utils::v8_string(scope, "__iv8IterIndex");
    let _ = obj.define_own_property(
        scope,
        idx_key.into(),
        v8::Integer::new(scope, 0).into(),
        v8::PropertyAttribute::DONT_ENUM,
    );
    // resultType ORDERED_NODE_SNAPSHOT_TYPE = 7
    let rt_key = crate::v8_utils::v8_string(scope, "resultType");
    let _ = obj.define_own_property(
        scope,
        rt_key.into(),
        v8::Integer::new(scope, 7).into(),
        v8::PropertyAttribute::NONE,
    );
    let snap_len_key = crate::v8_utils::v8_string(scope, "snapshotLength");
    let _ = obj.define_own_property(
        scope,
        snap_len_key.into(),
        v8::Integer::new(scope, nodes.len() as i32).into(),
        v8::PropertyAttribute::NONE,
    );
    let single = if nodes.len() == 1 {
        crate::dom::template::create_node_object(scope, state, nodes[0])
            .unwrap_or_else(|| v8::null(scope).into())
    } else {
        v8::null(scope).into()
    };
    let single_key = crate::v8_utils::v8_string(scope, "singleNodeValue");
    let _ = obj.define_own_property(
        scope,
        single_key.into(),
        single,
        v8::PropertyAttribute::NONE,
    );
    // snapshotItem / iterateNext
    let snap_item = v8::FunctionTemplate::builder_raw(xpath_snapshot_item_cb)
        .length(1)
        .build(scope);
    let snap_fn = crate::v8_utils::v8_fn(scope, &snap_item);
    let snap_item_key = crate::v8_utils::v8_string(scope, "snapshotItem");
    let _ = obj.set(scope, snap_item_key.into(), snap_fn.into());
    let iter = v8::FunctionTemplate::builder_raw(xpath_iterate_next_cb)
        .length(0)
        .build(scope);
    let iter_fn = crate::v8_utils::v8_fn(scope, &iter);
    let iter_key = crate::v8_utils::v8_string(scope, "iterateNext");
    let _ = obj.set(scope, iter_key.into(), iter_fn.into());
    obj
}

unsafe extern "C" fn xpath_snapshot_item_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let Some(this_obj) = this.to_object(scope) else {
            rv.set(v8::null(scope).into());
            return;
        };
        let idx = if args.length() >= 1 {
            args.get(0).uint32_value(scope).unwrap_or(0)
        } else {
            0
        };
        let snap_key = crate::v8_utils::v8_string(scope, "__iv8Snapshot");
        if let Some(arr_val) = this_obj.get(scope, snap_key.into()) {
            if arr_val.is_array() {
                let arr: v8::Local<v8::Array> = unsafe { v8::Local::cast_unchecked(arr_val) };
                if let Some(item) = arr.get_index(scope, idx) {
                    rv.set(item);
                    return;
                }
            }
        }
        rv.set(v8::null(scope).into());
    }));
}

unsafe extern "C" fn xpath_iterate_next_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let Some(this_obj) = this.to_object(scope) else {
            rv.set(v8::null(scope).into());
            return;
        };
        let idx_key = crate::v8_utils::v8_string(scope, "__iv8IterIndex");
        let snap_key = crate::v8_utils::v8_string(scope, "__iv8Snapshot");
        let idx = this_obj
            .get(scope, idx_key.into())
            .and_then(|v| v.uint32_value(scope))
            .unwrap_or(0);
        if let Some(arr_val) = this_obj.get(scope, snap_key.into()) {
            if arr_val.is_array() {
                let arr: v8::Local<v8::Array> = unsafe { v8::Local::cast_unchecked(arr_val) };
                if let Some(item) = arr.get_index(scope, idx) {
                    let _ = this_obj.set(
                        scope,
                        idx_key.into(),
                        v8::Integer::new(scope, (idx + 1) as i32).into(),
                    );
                    rv.set(item);
                    return;
                }
            }
        }
        rv.set(v8::null(scope).into());
    }));
}

unsafe extern "C" fn xpath_expression_evaluate_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let Some(this_obj) = this.to_object(scope) else {
            rv.set(v8::null(scope).into());
            return;
        };
        let expr_key = crate::v8_utils::v8_string(scope, "__iv8Expression");
        let expr = this_obj
            .get(scope, expr_key.into())
            .map(|v| v.to_rust_string_lossy(scope))
            .unwrap_or_default();
        let context_id = if args.length() >= 1 && args.get(0).is_object() {
            let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(args.get(0)) };
            extract_node_id(scope, obj)
        } else {
            None
        };
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let nodes = {
            let doc = state.document.borrow();
            if let Some(ref d) = *doc {
                let ctx = context_id.unwrap_or_else(|| d.root_id());
                d.xpath_evaluate(&expr, ctx)
            } else {
                Vec::new()
            }
        };
        let global = scope.get_current_context().global(scope);
        let result = make_xpath_result(scope, state, global, &nodes);
        rv.set(result.into());
    }));
}

unsafe extern "C" fn document_evaluate_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        if args.length() < 1 {
            let msg = crate::v8_utils::v8_string(
                scope,
                "Failed to execute 'evaluate' on 'Document': 1 argument required, but only 0 present.",
            );
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }
        let expr = args.get(0).to_rust_string_lossy(scope);
        let context_id = if args.length() >= 2 && args.get(1).is_object() {
            let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(args.get(1)) };
            extract_node_id(scope, obj)
        } else {
            None
        };
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let nodes = {
            let doc = state.document.borrow();
            if let Some(ref d) = *doc {
                let ctx = context_id.unwrap_or_else(|| d.root_id());
                d.xpath_evaluate(&expr, ctx)
            } else {
                Vec::new()
            }
        };
        let global = scope.get_current_context().global(scope);
        let result = make_xpath_result(scope, state, global, &nodes);
        rv.set(result.into());
    }));
}

fn document_prototype_named<'s>(
    scope: &v8::PinScope<'s, '_>,
    global: v8::Local<'s, v8::Object>,
    name: &str,
) -> Option<v8::Local<'s, v8::Object>> {
    let key = crate::v8_utils::v8_string(scope, name);
    let ctor_val = global.get(scope, key.into())?;
    if !ctor_val.is_function() {
        return None;
    }
    let ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(ctor_val) };
    let proto_key = crate::v8_utils::v8_string(scope, "prototype");
    let proto_val = ctor.get(scope, proto_key.into())?;
    proto_val.to_object(scope)
}

fn tree_walker_hidden_node(
    scope: &v8::PinScope<'_, '_>,
    this: v8::Local<v8::Object>,
    key: &str,
) -> Option<NodeId> {
    let k = crate::v8_utils::v8_string(scope, key);
    let val = this.get(scope, k.into())?;
    if !val.is_object() {
        return None;
    }
    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(val) };
    extract_node_id(scope, obj)
}

fn tree_walker_set_current(
    scope: &v8::PinScope<'_, '_>,
    this: v8::Local<v8::Object>,
    node_obj: v8::Local<v8::Value>,
) {
    let k = crate::v8_utils::v8_string(scope, "__iv8CurrentNode");
    let _ = this.set(scope, k.into(), node_obj);
}

fn tree_walker_what_to_show(
    scope: &v8::PinScope<'_, '_>,
    this: v8::Local<v8::Object>,
) -> u32 {
    let k = crate::v8_utils::v8_string(scope, "__iv8WhatToShow");
    this.get(scope, k.into())
        .and_then(|v| v.number_value(scope))
        .map(|n| n as u32)
        .unwrap_or(0xFFFF_FFFF)
}

fn tree_walker_move(
    info: *const v8::FunctionCallbackInfo,
    step: fn(&crate::dom::Document, NodeId, NodeId, u32) -> Option<NodeId>,
) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let Some(this_obj) = this.to_object(scope) else {
            rv.set(v8::null(scope).into());
            return;
        };
        let Some(root_id) = tree_walker_hidden_node(scope, this_obj, "__iv8Root") else {
            rv.set(v8::null(scope).into());
            return;
        };
        let Some(cur_id) = tree_walker_hidden_node(scope, this_obj, "__iv8CurrentNode") else {
            rv.set(v8::null(scope).into());
            return;
        };
        let what = tree_walker_what_to_show(scope, this_obj);
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let next_id = {
            let doc = state.document.borrow();
            doc.as_ref().and_then(|d| step(d, root_id, cur_id, what))
        };
        match next_id {
            Some(nid) => {
                if let Some(obj) = crate::dom::template::create_node_object(scope, state, nid) {
                    tree_walker_set_current(scope, this_obj, obj);
                    rv.set(obj);
                } else {
                    rv.set(v8::null(scope).into());
                }
            }
            None => rv.set(v8::null(scope).into()),
        }
    }));
}

unsafe extern "C" fn tree_walker_next_node_cb(info: *const v8::FunctionCallbackInfo) {
    tree_walker_move(info, |d, root, cur, what| d.tree_walker_next(root, cur, what));
}
unsafe extern "C" fn tree_walker_previous_node_cb(info: *const v8::FunctionCallbackInfo) {
    tree_walker_move(info, |d, root, cur, what| {
        d.tree_walker_previous(root, cur, what)
    });
}
unsafe extern "C" fn tree_walker_first_child_cb(info: *const v8::FunctionCallbackInfo) {
    tree_walker_move(info, |d, root, cur, what| {
        let _ = root;
        let mut c = d.first_child_id(cur)?;
        loop {
            if d.matches_what_to_show(c, what) {
                return Some(c);
            }
            c = d.next_sibling_id(c)?;
        }
    });
}
unsafe extern "C" fn tree_walker_last_child_cb(info: *const v8::FunctionCallbackInfo) {
    tree_walker_move(info, |d, root, cur, what| {
        let _ = root;
        let mut c = d.last_child_id(cur)?;
        loop {
            if d.matches_what_to_show(c, what) {
                return Some(c);
            }
            c = d.previous_sibling_id(c)?;
        }
    });
}
unsafe extern "C" fn tree_walker_parent_node_cb(info: *const v8::FunctionCallbackInfo) {
    tree_walker_move(info, |d, root, cur, what| {
        if cur == root {
            return None;
        }
        let mut p = d.parent_id(cur)?;
        loop {
            if p == root {
                return if d.matches_what_to_show(p, what) {
                    Some(p)
                } else {
                    None
                };
            }
            if d.matches_what_to_show(p, what) {
                return Some(p);
            }
            p = d.parent_id(p)?;
        }
    });
}
unsafe extern "C" fn tree_walker_next_sibling_cb(info: *const v8::FunctionCallbackInfo) {
    tree_walker_move(info, |d, root, cur, what| {
        let _ = root;
        let mut s = d.next_sibling_id(cur)?;
        loop {
            if d.matches_what_to_show(s, what) {
                return Some(s);
            }
            s = d.next_sibling_id(s)?;
        }
    });
}
unsafe extern "C" fn tree_walker_previous_sibling_cb(info: *const v8::FunctionCallbackInfo) {
    tree_walker_move(info, |d, root, cur, what| {
        let _ = root;
        let mut s = d.previous_sibling_id(cur)?;
        loop {
            if d.matches_what_to_show(s, what) {
                return Some(s);
            }
            s = d.previous_sibling_id(s)?;
        }
    });
}

/// document.createTreeWalker(root, whatToShow?, filter?) — real TreeWalker shell.
unsafe extern "C" fn create_tree_walker_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        if args.length() < 1 {
            let msg = crate::v8_utils::v8_string(
                scope,
                "Failed to execute 'createTreeWalker' on 'Document': 1 argument required, but only 0 present.",
            );
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }
        let root_arg = args.get(0);
        if !root_arg.is_object() {
            let msg = crate::v8_utils::v8_string(
                scope,
                "Failed to execute 'createTreeWalker' on 'Document': parameter 1 is not of type 'Node'.",
            );
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }
        let what_to_show = if args.length() >= 2 && args.get(1).is_number() {
            args.get(1).number_value(scope).unwrap_or(4294967295.0) as u32
        } else {
            0xFFFFFFFFu32
        };
        let global = scope.get_current_context().global(scope);
        let ctor_key = crate::v8_utils::v8_string(scope, "TreeWalker");
        let walker = if let Some(ctor_val) = global.get(scope, ctor_key.into()) {
            if ctor_val.is_function() {
                let ctor: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(ctor_val) };
                let proto_key = crate::v8_utils::v8_string(scope, "prototype");
                if let Some(proto) = ctor.get(scope, proto_key.into()) {
                    let obj = v8::Object::new(scope);
                    let _ = obj.set_prototype(scope, proto);
                    obj
                } else {
                    v8::Object::new(scope)
                }
            } else {
                v8::Object::new(scope)
            }
        } else {
            v8::Object::new(scope)
        };
        let set_hidden = |scope: &v8::PinScope<'_, '_>, key: &str, val: v8::Local<v8::Value>| {
            let k = crate::v8_utils::v8_string(scope, key);
            let _ = walker.define_own_property(
                scope,
                k.into(),
                val,
                v8::PropertyAttribute::DONT_ENUM,
            );
        };
        set_hidden(scope, "__iv8Root", root_arg);
        set_hidden(scope, "__iv8CurrentNode", root_arg);
        set_hidden(
            scope,
            "__iv8WhatToShow",
            v8::Number::new(scope, what_to_show as f64).into(),
        );
        if args.length() >= 3 {
            set_hidden(scope, "__iv8Filter", args.get(2));
        } else {
            set_hidden(scope, "__iv8Filter", v8::null(scope).into());
        }
        // Wire common TreeWalker accessors if still default stubs (read hidden keys).
        // Prefer existing codegen accessors when they read __iv8*.
        rv.set(walker.into());
    }));
}

/// document.createExpression(expression, resolver?) — XPathExpression shell.
unsafe extern "C" fn create_expression_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        if args.length() < 1 {
            let msg = crate::v8_utils::v8_string(
                scope,
                "Failed to execute 'createExpression' on 'Document': 1 argument required, but only 0 present.",
            );
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }
        let expr = args.get(0).to_rust_string_lossy(scope);
        let global = scope.get_current_context().global(scope);
        let ctor_key = crate::v8_utils::v8_string(scope, "XPathExpression");
        let obj = if let Some(ctor_val) = global.get(scope, ctor_key.into()) {
            if ctor_val.is_function() {
                let ctor: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(ctor_val) };
                let proto_key = crate::v8_utils::v8_string(scope, "prototype");
                if let Some(proto) = ctor.get(scope, proto_key.into()) {
                    let o = v8::Object::new(scope);
                    let _ = o.set_prototype(scope, proto);
                    o
                } else {
                    v8::Object::new(scope)
                }
            } else {
                v8::Object::new(scope)
            }
        } else {
            v8::Object::new(scope)
        };
        let k = crate::v8_utils::v8_string(scope, "__iv8Expression");
        let v = crate::v8_utils::v8_string(scope, &expr);
        let _ = obj.define_own_property(
            scope,
            k.into(),
            v.into(),
            v8::PropertyAttribute::DONT_ENUM,
        );
        rv.set(obj.into());
    }));
}

/// Helper to install a native method on an object.
/// `length` is the IDL required argument count (Function.length).
fn install_method(
    scope: &v8::PinScope<'_, '_>,
    obj: v8::Local<v8::Object>,
    name: &str,
    callback: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
    length: i32,
) {
    let tmpl = if length > 0 {
        v8::FunctionTemplate::builder_raw(callback).length(length).build(scope)
    } else {
        v8::FunctionTemplate::builder_raw(callback).build(scope)
    };
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
    // Identity cache: return same object for same NodeId (Global reference)
    {
        let mut cache = state.node_cache.borrow_mut();
        if let Some(global) = cache.get(&node_id) {
            let local = v8::Local::new(scope, global);
            crate::dom::template::bump_and_maybe_sweep(state, &mut cache, scope);
            return Some(local.into());
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
        NodeData::Element {
            tag_name,
            attrs,
            id,
            classes,
            ..
        } => {
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
                if let (Some(ak), Some(av)) = (v8::String::new(scope, k), v8::String::new(scope, v))
                {
                    attrs_obj.set(scope, ak.into(), av.into());
                }
            }
            let attrs_key = crate::v8_utils::v8_string(scope, "__attrs__");
            obj.set(scope, attrs_key.into(), attrs_obj.into());

            // Install getAttribute as a native function
            let get_attr_tmpl =
                v8::FunctionTemplate::builder_raw(get_attribute_callback).build(scope);
            let get_attr_fn = crate::v8_utils::v8_fn(scope, &get_attr_tmpl);
            let get_attr_key = crate::v8_utils::v8_string(scope, "getAttribute");
            obj.set(scope, get_attr_key.into(), get_attr_fn.into());

            // Install setAttribute
            let set_attr_tmpl =
                v8::FunctionTemplate::builder_raw(set_attribute_callback).build(scope);
            let set_attr_fn = crate::v8_utils::v8_fn(scope, &set_attr_tmpl);
            let set_attr_key = crate::v8_utils::v8_string(scope, "setAttribute");
            obj.set(scope, set_attr_key.into(), set_attr_fn.into());

            // Install removeAttribute
            let rm_attr_tmpl =
                v8::FunctionTemplate::builder_raw(remove_attribute_callback).build(scope);
            let rm_attr_fn = crate::v8_utils::v8_fn(scope, &rm_attr_tmpl);
            let rm_attr_key = crate::v8_utils::v8_string(scope, "removeAttribute");
            obj.set(scope, rm_attr_key.into(), rm_attr_fn.into());

            // Install hasAttribute
            let has_attr_tmpl =
                v8::FunctionTemplate::builder_raw(has_attribute_callback).build(scope);
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
            let replace_tmpl =
                v8::FunctionTemplate::builder_raw(replace_child_callback).build(scope);
            let replace_fn = crate::v8_utils::v8_fn(scope, &replace_tmpl);
            let replace_key = crate::v8_utils::v8_string(scope, "replaceChild");
            obj.set(scope, replace_key.into(), replace_fn.into());

            // Install insertBefore
            let ib_tmpl = v8::FunctionTemplate::builder_raw(insert_before_callback).build(scope);
            let ib_fn = crate::v8_utils::v8_fn(scope, &ib_tmpl);
            let ib_key = crate::v8_utils::v8_string(scope, "insertBefore");
            obj.set(scope, ib_key.into(), ib_fn.into());

            // Install addEventListener
            let ael_tmpl =
                v8::FunctionTemplate::builder_raw(add_event_listener_callback).build(scope);
            let ael_fn = crate::v8_utils::v8_fn(scope, &ael_tmpl);
            let ael_key = crate::v8_utils::v8_string(scope, "addEventListener");
            obj.set(scope, ael_key.into(), ael_fn.into());

            // Install removeEventListener
            let rel_tmpl =
                v8::FunctionTemplate::builder_raw(remove_event_listener_callback).build(scope);
            let rel_fn = crate::v8_utils::v8_fn(scope, &rel_tmpl);
            let rel_key = crate::v8_utils::v8_string(scope, "removeEventListener");
            obj.set(scope, rel_key.into(), rel_fn.into());

            // Install dispatchEvent
            let de_tmpl = v8::FunctionTemplate::builder_raw(dispatch_event_callback).build(scope);
            let de_fn = crate::v8_utils::v8_fn(scope, &de_tmpl);
            let de_key = crate::v8_utils::v8_string(scope, "dispatchEvent");
            obj.set(scope, de_key.into(), de_fn.into());

            // Install innerHTML getter (as a method for now — proper getter needs accessor)
            let ih_tmpl =
                v8::FunctionTemplate::builder_raw(inner_html_getter_callback).build(scope);
            let ih_fn = crate::v8_utils::v8_fn(scope, &ih_tmpl);
            let ih_key = crate::v8_utils::v8_string(scope, "__getInnerHTML__");
            obj.set(scope, ih_key.into(), ih_fn.into());

            // Install innerHTML setter
            let ihs_tmpl =
                v8::FunctionTemplate::builder_raw(inner_html_setter_callback).build(scope);
            let ihs_fn = crate::v8_utils::v8_fn(scope, &ihs_tmpl);
            let ihs_key = crate::v8_utils::v8_string(scope, "__setInnerHTML__");
            obj.set(scope, ihs_key.into(), ihs_fn.into());

            // Install insertAdjacentHTML
            let iah_tmpl =
                v8::FunctionTemplate::builder_raw(insert_adjacent_html_callback).build(scope);
            let iah_fn = crate::v8_utils::v8_fn(scope, &iah_tmpl);
            let iah_key = crate::v8_utils::v8_string(scope, "insertAdjacentHTML");
            obj.set(scope, iah_key.into(), iah_fn.into());

            // Install outerHTML getter
            let oh_tmpl =
                v8::FunctionTemplate::builder_raw(outer_html_getter_callback).build(scope);
            let oh_fn = crate::v8_utils::v8_fn(scope, &oh_tmpl);
            let oh_key = crate::v8_utils::v8_string(scope, "__getOuterHTML__");
            obj.set(scope, oh_key.into(), oh_fn.into());

            // Install textContent setter (as method)
            let tcs_tmpl =
                v8::FunctionTemplate::builder_raw(text_content_setter_callback).build(scope);
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

    // Store in identity cache as Global reference (strong — K-018 fix)
    let global_obj = v8::Global::new(scope, obj);
    state.node_cache.borrow_mut().insert(node_id, global_obj);

    Some(obj.into())
}

/// Convert a list of NodeIds to a V8 array of node objects.
// ─── V8 Callbacks ───────────────────────────────────────────────────────────

/// document.getElementById(id) callback
unsafe extern "C" fn get_element_by_id(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            rv.set(v8::null(scope).into());
            return;
        }

        // WebIDL DOMString: ToString on the argument (elements stringify to tags).
        let id_str = args.get(0).to_rust_string_lossy(scope);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let node_id = {
            let doc = state.document.borrow();
            doc.as_ref().and_then(|d| d.get_element_by_id(&id_str))
        };

        if let Some(nid) = node_id {
            if let Some(obj) = node_to_v8_object(scope, state, nid) {
                rv.set(obj);
            } else {
                rv.set(v8::null(scope).into());
            }
        } else {
            // Single id path: NodeData id_index only (DOM Element.id setter).
            rv.set(v8::null(scope).into());
        }
    }));
    if result.is_err() {
        crate::telemetry::dom_binding_panic("getElementById");
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
        crate::telemetry::dom_binding_panic("querySelector");
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

        let list = crate::dom::template::create_node_list_instance(scope, state, &node_ids)
            .unwrap_or_else(|| v8::Array::new(scope, 0).into());
        rv.set(list);
    }));
    if result.is_err() {
        crate::telemetry::dom_binding_panic("querySelectorAll");
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

        let list = crate::dom::template::create_node_list_instance(scope, state, &node_ids)
            .unwrap_or_else(|| v8::Array::new(scope, 0).into());
        rv.set(list);
    }));
    if result.is_err() {
        crate::telemetry::dom_binding_panic("getElementsByTagName");
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
                        if target_classes
                            .iter()
                            .all(|tc| classes.iter().any(|c| c == tc))
                        {
                            results.push(node_ref.id());
                        }
                    }
                    results
                }
            } else {
                vec![]
            }
        };

        let list = crate::dom::template::create_node_list_instance(scope, state, &node_ids)
            .unwrap_or_else(|| v8::Array::new(scope, 0).into());
        rv.set(list);
    }));
    if result.is_err() {
        crate::telemetry::dom_binding_panic("getElementsByClassName");
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
                let attrs_obj: v8::Local<v8::Object> =
                    unsafe { v8::Local::cast_unchecked(attrs_val) };
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
        crate::telemetry::dom_binding_panic("getAttribute");
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
        let tag_str = tag_str.to_ascii_lowercase();

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
        crate::telemetry::dom_binding_panic("createElement");
    }
}

/// document.createTextNode(data) callback
unsafe extern "C" fn create_text_node(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            return;
        }

        let data = args.get(0).to_rust_string_lossy(scope);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let node_id = {
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                let root_id = doc.root_id();
                let nid = doc.append_child(root_id, NodeData::text(&data));
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
        crate::telemetry::dom_binding_panic("createTextNode");
    }
}

/// document.createComment(data) callback
unsafe extern "C" fn create_comment(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            return;
        }

        let data = args.get(0).to_rust_string_lossy(scope);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let node_id = {
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                let root_id = doc.root_id();
                let nid = doc.append_child(root_id, NodeData::comment(&data));
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
        crate::telemetry::dom_binding_panic("createComment");
    }
}

/// document.createDocumentFragment() callback
unsafe extern "C" fn create_document_fragment(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let node_id = {
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                let root_id = doc.root_id();
                let nid = doc.append_child(root_id, NodeData::DocumentFragment);
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
        crate::telemetry::dom_binding_panic("createDocumentFragment");
    }
}

/// document.createElementNS(namespace, qualifiedName) callback
unsafe extern "C" fn create_element_ns(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 2 {
            return;
        }

        let ns_arg = args.get(0);
        let ns_str = ns_arg.to_rust_string_lossy(scope);
        let tag_arg = args.get(1);
        let tag_str = tag_arg.to_rust_string_lossy(scope);
        let tag_str = tag_str.to_ascii_lowercase();

        let ns = if ns_str.is_empty() || ns_str == "null" {
            "http://www.w3.org/1999/xhtml"
        } else {
            Box::leak(ns_str.into_boxed_str())
        };

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let node_id = {
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                let root_id = doc.root_id();
                let data = NodeData::element(&tag_str, ns, vec![]);
                let nid = doc.append_child(root_id, data);
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
        crate::telemetry::dom_binding_panic("createElementNS");
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
                // Check if child is a DocumentFragment — DOM spec requires
                // transferring all children to the new parent, leaving the
                // fragment empty.
                let is_fragment = doc.get(cid)
                    .map(|n| matches!(n.value(), NodeData::DocumentFragment))
                    .unwrap_or(false);

                if is_fragment {
                    let child_ids: Vec<_> = doc.tree.get(cid)
                        .map(|frag| frag.children().map(|c| c.id()).collect())
                        .unwrap_or_default();
                    for grandchild_id in child_ids {
                        doc.detach(grandchild_id);
                        if let Some(mut parent) = doc.tree.get_mut(pid) {
                            parent.append_id(grandchild_id);
                        }
                    }
                } else {
                    // Normal append: detach child from current parent, append to new parent
                    doc.detach(cid);
                    if let Some(mut parent) = doc.tree.get_mut(pid) {
                        parent.append_id(cid);
                    }
                }
                doc.invalidate_tag_index();
                doc.rebuild_id_index();
            }
        }

        // Return the child (per DOM spec)
        rv.set(child_arg);
    }));
    if result.is_err() {
        crate::telemetry::dom_binding_panic("appendChild");
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
        crate::telemetry::dom_binding_panic("removeChild");
    }
}

/// element.replaceChild(newChild, oldChild) callback
unsafe extern "C" fn replace_child_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 2 {
            return;
        }

        let new_child_arg = args.get(0);
        let old_child_arg = args.get(1);

        if !new_child_arg.is_object() || !old_child_arg.is_object() {
            return;
        }

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

        if args.length() < 1 {
            return;
        }

        let this = args.this();
        let new_child_arg = args.get(0);
        if !new_child_arg.is_object() {
            return;
        }

        let new_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(new_child_arg) };
        let parent_id = extract_node_id(scope, this);
        let new_id = extract_node_id(scope, new_obj);

        if let (Some(pid), Some(nid)) = (parent_id, new_id) {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);

            // Reference node (2nd arg) — if null/undefined, append to end
            if args.length() >= 2 && args.get(1).is_object() {
                let ref_obj: v8::Local<v8::Object> =
                    unsafe { v8::Local::cast_unchecked(args.get(1)) };
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

        if args.length() < 1 {
            return;
        }

        let this = args.this();
        let name_arg = args.get(0).to_rust_string_lossy(scope);
        let node_id = extract_node_id(scope, this);

        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                if let Some(mut node) = doc.tree.get_mut(nid) {
                    if let NodeData::Element {
                        ref mut attrs,
                        ref mut id,
                        ref mut classes,
                        ..
                    } = node.value()
                    {
                        attrs.retain(|(k, _)| k != &name_arg);
                        if name_arg == "id" {
                            *id = None;
                        }
                        if name_arg == "class" {
                            classes.clear();
                        }
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
                let attrs_obj: v8::Local<v8::Object> =
                    unsafe { v8::Local::cast_unchecked(attrs_val) };
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
                doc.get(nid)
                    .map(|n| n.value().get_attr(&name_arg).is_some())
                    .unwrap_or(false)
            } else {
                false
            }
        } else {
            false
        };

        rv.set(v8::Boolean::new(scope, has).into());
    }));
}

/// element.insertAdjacentHTML(position, html) callback
unsafe extern "C" fn insert_adjacent_html_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 2 {
            return;
        }

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
                    fragment
                        .tree
                        .get(body_id)
                        .map(|body| {
                            body.children()
                                .map(|c| (c.id(), c.value().clone()))
                                .collect()
                        })
                        .unwrap_or_default()
                };

                match position.as_str() {
                    "beforebegin" => {
                        // Insert before this element (as previous sibling)
                        if let Some(parent_id) =
                            doc.tree.get(nid).and_then(|n| n.parent()).map(|p| p.id())
                        {
                            for (frag_id, _) in &frag_children {
                                let frag_data =
                                    fragment.tree.get(*frag_id).map(|n| n.value().clone());
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
                        let first_child = doc
                            .tree
                            .get(nid)
                            .and_then(|n| n.first_child())
                            .map(|c| c.id());
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
                        if let Some(parent_id) =
                            doc.tree.get(nid).and_then(|n| n.parent()).map(|p| p.id())
                        {
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
                    if let NodeData::Element {
                        ref mut attrs,
                        ref mut id,
                        ref mut classes,
                        ..
                    } = node.value()
                    {
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
                            *classes = value_arg
                                .split_whitespace()
                                .map(|s| s.to_string())
                                .collect();
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
                let attrs_obj: v8::Local<v8::Object> =
                    unsafe { v8::Local::cast_unchecked(attrs_val) };
                if let Some(k) = v8::String::new(scope, &name_arg) {
                    if let Some(v) = v8::String::new(scope, &value_arg) {
                        attrs_obj.set(scope, k.into(), v.into());
                    }
                }
            }
        }
    }));
    if result.is_err() {
        crate::telemetry::dom_binding_panic("setAttribute");
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
        let capture = if args.length() >= 3 {
            args.get(2).is_true()
        } else {
            false
        };

        let node_id = extract_node_id(scope, this);
        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            state.event_listeners.borrow_mut().remove_by_callback(
                scope,
                nid,
                &event_type,
                func,
                capture,
            );
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

        if args.length() < 1 {
            return;
        }

        let this = args.this();
        let text_val = args.get(0).to_rust_string_lossy(scope);
        let node_id = extract_node_id(scope, this);

        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                // Remove all children
                let children: Vec<crate::dom::NodeId> = doc
                    .tree
                    .get(nid)
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

        if args.length() < 1 {
            return;
        }

        let this = args.this();
        let html_str = args.get(0).to_rust_string_lossy(scope);
        let node_id = extract_node_id(scope, this);

        if let Some(nid) = node_id {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                // 1. Remove all existing children
                let children: Vec<crate::dom::NodeId> = doc
                    .tree
                    .get(nid)
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
                    fragment
                        .tree
                        .get(body_id)
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
        let child_ids: Vec<crate::dom::NodeId> = source_node.children().map(|c| c.id()).collect();
        for child_id in child_ids {
            append_node_recursive(doc, new_id, source, child_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_node_id_to_usize_and_back() {
        let id = usize_to_node_id(42).unwrap();
        assert_eq!(node_id_to_usize(id), 42);
    }

    #[test]
    fn test_usize_to_node_id_zero_returns_none() {
        assert!(usize_to_node_id(0).is_none());
    }

    #[test]
    fn test_usize_to_node_id_valid() {
        let id = usize_to_node_id(1).unwrap();
        assert_eq!(node_id_to_usize(id), 1);
    }

    #[test]
    fn test_node_id_roundtrip_large() {
        let id = usize_to_node_id(1_000_000).unwrap();
        assert_eq!(node_id_to_usize(id), 1_000_000);
    }
}
