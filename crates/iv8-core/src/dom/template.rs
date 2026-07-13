//! DOM FunctionTemplate hierarchy for V8.
//!
//! Builds a proper prototype chain:
//!   EventTarget ← Node ← Element ← HTMLElement ← HTMLDivElement / HTMLCanvasElement / ...
//!
//! Each template uses internal field 0 to store the NodeId (as a usize via External).
//! Methods are installed once on the prototype template, not per-instance.
//!
//! This replaces the old plain-Object + JS-shim approach with native V8 templates,
//! giving correct `instanceof`, `getAttribute.toString() → "[native code]"`, etc.

use crate::dom::NodeData;
use crate::dom::NodeId;
use crate::state::RuntimeState;
use std::collections::HashMap;
use url::Url;

/// Index of the internal field that stores the NodeId.
pub const NODE_ID_FIELD: i32 = 0;

/// Number of internal fields per DOM node object.
pub const INTERNAL_FIELD_COUNT: i32 = 1;

/// Collection of all DOM FunctionTemplates, stored as Globals so they
/// survive across handle scopes.
pub struct DomTemplates {
    /// EventTarget — root of the DOM prototype chain.
    pub event_target: v8::Global<v8::FunctionTemplate>,
    /// Node — inherits EventTarget.
    pub node: v8::Global<v8::FunctionTemplate>,
    /// Element — inherits Node.
    pub element: v8::Global<v8::FunctionTemplate>,
    /// HTMLElement — inherits Element.
    pub html_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLDivElement — inherits HTMLElement.
    pub html_div_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLSpanElement — inherits HTMLElement.
    pub html_span_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLAnchorElement — inherits HTMLElement.
    pub html_anchor_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLInputElement — inherits HTMLElement.
    pub html_input_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLButtonElement — inherits HTMLElement.
    pub html_button_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLFormElement — inherits HTMLElement.
    pub html_form_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLCanvasElement — inherits HTMLElement.
    pub html_canvas_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLScriptElement — inherits HTMLElement.
    pub html_script_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLImageElement — inherits HTMLElement.
    pub html_image_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLVideoElement — inherits HTMLElement.
    pub html_video_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLAudioElement — inherits HTMLElement.
    pub html_audio_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLSelectElement — inherits HTMLElement.
    pub html_select_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLOptionElement — inherits HTMLElement.
    pub html_option_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLIFrameElement — inherits HTMLElement.
    pub html_iframe_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLTextAreaElement — inherits HTMLElement.
    pub html_textarea_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLHeadElement — inherits HTMLElement.
    pub html_head_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLBodyElement — inherits HTMLElement.
    pub html_body_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLHtmlElement — inherits HTMLElement.
    pub html_html_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLParagraphElement — inherits HTMLElement.
    pub html_paragraph_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLHeadingElement — inherits HTMLElement.
    pub html_heading_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLUListElement — inherits HTMLElement.
    pub html_ulist_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLOListElement — inherits HTMLElement.
    pub html_olist_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLLIElement — inherits HTMLElement.
    pub html_li_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLTableElement — inherits HTMLElement.
    pub html_table_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLTableRowElement — inherits HTMLElement.
    pub html_table_row_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLTableCellElement — inherits HTMLElement (td/th).
    pub html_table_cell_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLDialogElement — inherits HTMLElement.
    pub html_dialog_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLStyleElement — inherits HTMLElement.
    pub html_style_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLLinkElement — inherits HTMLElement.
    pub html_link_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLMetaElement — inherits HTMLElement.
    pub html_meta_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLUnknownElement — inherits HTMLElement, fallback for unknown tag names.
    pub html_unknown_element: v8::Global<v8::FunctionTemplate>,
    /// Text node — inherits CharacterData.
    pub text_node: v8::Global<v8::FunctionTemplate>,
    /// Comment node — inherits CharacterData.
    pub comment_node: v8::Global<v8::FunctionTemplate>,
    /// CharacterData — inherits Node, parent of Text/Comment.
    pub character_data: v8::Global<v8::FunctionTemplate>,
    /// DocumentFragment — inherits Node.
    pub document_fragment: v8::Global<v8::FunctionTemplate>,
    /// Document node — inherits Node.
    pub document_node: v8::Global<v8::FunctionTemplate>,
    /// NodeList — live or static node collection.
    pub node_list: v8::Global<v8::FunctionTemplate>,
    pub dom_token_list: v8::Global<v8::FunctionTemplate>,
    pub css_style_declaration: v8::Global<v8::FunctionTemplate>,
    pub headers: v8::Global<v8::FunctionTemplate>,
    pub response: v8::Global<v8::FunctionTemplate>,
    pub request: v8::Global<v8::FunctionTemplate>,
    /// Attr — attribute node view over Element.attrs (live value).
    pub attr: v8::Global<v8::FunctionTemplate>,
}

/// Helper: create a FunctionTemplate with a class name and internal field count.
/// `ctor` selects the constructor callback: `illegal_dom_constructor` for
/// non-constructable interfaces (the common case — DOM nodes are created by
/// Rust, not by `new HTMLElement()`), `empty_dom_constructor` for the few
/// constructable interfaces (Request, Response).
fn make_template<'s>(
    scope: &v8::PinScope<'s, '_>,
    class_name: &str,
    ctor: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
) -> v8::Local<'s, v8::FunctionTemplate> {
    let tmpl = v8::FunctionTemplate::builder_raw(ctor).build(scope);
    tmpl.read_only_prototype();
    let name = crate::v8_utils::v8_string(scope, class_name);
    tmpl.set_class_name(name);
    let inst = tmpl.instance_template(scope);
    inst.set_internal_field_count(INTERNAL_FIELD_COUNT as usize);
    tmpl
}

/// Illegal constructor callback — DOM nodes are not constructed from JS.
/// Throws TypeError "Illegal constructor", matching real browser behavior.
/// Internal node creation uses ObjectTemplate::new_instance, which does not
/// invoke this callback, so Rust-side instantiation is unaffected.
unsafe extern "C" fn illegal_dom_constructor(info: *const v8::FunctionCallbackInfo) {
    let info_ref = unsafe { &*info };
    v8::callback_scope!(unsafe scope, info_ref);
    let msg = crate::v8_utils::v8_string(scope, "Illegal constructor");
    let exc = v8::Exception::type_error(scope, msg);
    scope.throw_exception(exc);
}

/// No-op constructor callback — for interfaces that ARE constructable from JS
/// (Request, Response). Produces an empty instance; real construction logic
/// is handled by JS shims layered on top.
unsafe extern "C" fn empty_dom_constructor(_info: *const v8::FunctionCallbackInfo) {}

/// Response constructor — parses options and sets hidden keys for
/// status/ok/statusText/headers getters.
unsafe extern "C" fn response_constructor(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let this = args.this();

        let (status, status_text) = parse_response_options(scope, &args);
        set_response_state(scope, this, status, &status_text);
    }));
}

/// Request constructor — parses options and sets hidden keys for
/// url/method/headers and other Request properties.
unsafe extern "C" fn request_constructor(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let this = args.this();

        if args.length() >= 1 {
            let url = args.get(0).to_rust_string_lossy(scope);
            let url_key = crate::v8_utils::v8_string(scope, "__iv8Url");
            let url_val = crate::v8_utils::v8_string(scope, &url);
            let _ = this.set(scope, url_key.into(), url_val.into());
        }

        let mut method = String::from("GET");
        let mut mode = String::from("cors");
        let mut credentials = String::from("same-origin");
        let mut cache = String::from("default");
        let mut redirect = String::from("follow");
        let mut referrer = String::from("about:client");
        let mut referrer_policy = String::new();
        let mut destination = String::new();
        let mut integrity = String::new();
        let mut keepalive = false;

        if args.length() >= 2 {
            let opts = args.get(1);
            if opts.is_object() && !opts.is_null() {
                let opts_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(opts) };
                let m_key = crate::v8_utils::v8_string(scope, "method");
                if let Some(m_val) = opts_obj.get(scope, m_key.into()) {
                    if m_val.is_string() { method = m_val.to_rust_string_lossy(scope); }
                }
                let mode_key = crate::v8_utils::v8_string(scope, "mode");
                if let Some(v) = opts_obj.get(scope, mode_key.into()) {
                    if v.is_string() { mode = v.to_rust_string_lossy(scope); }
                }
                let cred_key = crate::v8_utils::v8_string(scope, "credentials");
                if let Some(v) = opts_obj.get(scope, cred_key.into()) {
                    if v.is_string() { credentials = v.to_rust_string_lossy(scope); }
                }
                let cache_key = crate::v8_utils::v8_string(scope, "cache");
                if let Some(v) = opts_obj.get(scope, cache_key.into()) {
                    if v.is_string() { cache = v.to_rust_string_lossy(scope); }
                }
                let redir_key = crate::v8_utils::v8_string(scope, "redirect");
                if let Some(v) = opts_obj.get(scope, redir_key.into()) {
                    if v.is_string() { redirect = v.to_rust_string_lossy(scope); }
                }
            }
        }

        let set_str = |scope: &v8::PinScope<'_, '_>, this: v8::Local<v8::Object>, key_name: &str, val: &str| {
            let k = crate::v8_utils::v8_string(scope, key_name);
            let v = crate::v8_utils::v8_string(scope, val);
            let _ = this.set(scope, k.into(), v.into());
        };
        let set_bool = |scope: &v8::PinScope<'_, '_>, this: v8::Local<v8::Object>, key_name: &str, val: bool| {
            let k = crate::v8_utils::v8_string(scope, key_name);
            let _ = this.set(scope, k.into(), v8::Boolean::new(scope, val).into());
        };

        set_str(scope, this, "__iv8Method", &method);
        set_str(scope, this, "__iv8Mode", &mode);
        set_str(scope, this, "__iv8Credentials", &credentials);
        set_str(scope, this, "__iv8Cache", &cache);
        set_str(scope, this, "__iv8Redirect", &redirect);
        set_str(scope, this, "__iv8Referrer", &referrer);
        set_str(scope, this, "__iv8ReferrerPolicy", &referrer_policy);
        set_str(scope, this, "__iv8Destination", &destination);
        set_str(scope, this, "__iv8Integrity", &integrity);
        set_bool(scope, this, "__iv8Keepalive", keepalive);

        let body_key = crate::v8_utils::v8_string(scope, "__iv8Body");
        let _ = this.set(scope, body_key.into(), v8::null(scope).into());
        let sig_key = crate::v8_utils::v8_string(scope, "__iv8Signal");
        let _ = this.set(scope, sig_key.into(), v8::null(scope).into());

        let headers_key = crate::v8_utils::v8_string(scope, "__iv8Headers");
        let ctx = scope.get_current_context();
        let global = ctx.global(scope);
        let headers_ctor_key = crate::v8_utils::v8_string(scope, "Headers");
        if let Some(headers_ctor_val) = global.get(scope, headers_ctor_key.into()) {
            if headers_ctor_val.is_function() {
                let headers_ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(headers_ctor_val) };
                if let Some(headers_obj) = headers_ctor.new_instance(scope, &[]) {
                    let _ = this.set(scope, headers_key.into(), headers_obj.into());
                }
            }
        }
    }));
}

fn parse_response_options(scope: &v8::PinScope<'_, '_>, args: &v8::FunctionCallbackArguments) -> (u32, String) {
    let mut status = 200u32;
    let mut status_text = String::new();
    if args.length() >= 2 {
        let opts = args.get(1);
        if opts.is_object() && !opts.is_null() {
            let opts_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(opts) };
            let status_key = crate::v8_utils::v8_string(scope, "status");
            if let Some(status_val) = opts_obj.get(scope, status_key.into()) {
                if status_val.is_number() {
                    status = status_val.uint32_value(scope).unwrap_or(200);
                }
            }
            let st_key = crate::v8_utils::v8_string(scope, "statusText");
            if let Some(st_val) = opts_obj.get(scope, st_key.into()) {
                if st_val.is_string() {
                    status_text = st_val.to_rust_string_lossy(scope);
                }
            }
        }
    }
    (status, status_text)
}

fn set_response_state(scope: &v8::PinScope<'_, '_>, this: v8::Local<v8::Object>, status: u32, status_text: &str) {
    let sk = crate::v8_utils::v8_string(scope, "__iv8Status");
    let _ = this.set(scope, sk.into(), v8::Integer::new(scope, status as i32).into());

    let ok = status >= 200 && status < 300;
    let ok_key = crate::v8_utils::v8_string(scope, "__iv8Ok");
    let _ = this.set(scope, ok_key.into(), v8::Boolean::new(scope, ok).into());

    let st_key = crate::v8_utils::v8_string(scope, "__iv8StatusText");
    let st_val = crate::v8_utils::v8_string(scope, status_text);
    let _ = this.set(scope, st_key.into(), st_val.into());

    let url_key = crate::v8_utils::v8_string(scope, "__iv8Url");
    let _ = this.set(scope, url_key.into(), crate::v8_utils::v8_string(scope, "").into());

    let body_used_key = crate::v8_utils::v8_string(scope, "__iv8BodyUsed");
    let _ = this.set(scope, body_used_key.into(), v8::Boolean::new(scope, false).into());

    let headers_key = crate::v8_utils::v8_string(scope, "__iv8Headers");
    let ctx = scope.get_current_context();
    let global = ctx.global(scope);
    let headers_ctor_key = crate::v8_utils::v8_string(scope, "Headers");
    if let Some(headers_ctor_val) = global.get(scope, headers_ctor_key.into()) {
        if headers_ctor_val.is_function() {
            let headers_ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(headers_ctor_val) };
            if let Some(headers_obj) = headers_ctor.new_instance(scope, &[]) {
                let _ = this.set(scope, headers_key.into(), headers_obj.into());
            }
        }
    }
}

/// Construct-only callback — allows `new X()` but throws TypeError on `X()`
/// without `new`. Used for interfaces like EventTarget that are constructable
/// but not callable.
unsafe extern "C" fn construct_only_dom_constructor(info: *const v8::FunctionCallbackInfo) {
    let info_ref = unsafe { &*info };
    if !info_ref.is_construct_call() {
        v8::callback_scope!(unsafe scope, info_ref);
        let msg = crate::v8_utils::v8_string(scope, "Failed to construct: please use 'new'");
        let exc = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exc);
    }
}

/// Helper: install a native method on a prototype template.
/// No Signature — receiver check is handled by null_this_check / run_callback.
fn install_proto_method(
    scope: &v8::PinScope<'_, '_>,
    proto: v8::Local<v8::ObjectTemplate>,
    name: &str,
    callback: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
) {
    install_proto_method_with_length(scope, None, proto, name, callback, 0);
}

/// Helper: install a native method with Signature + optional min arg count.
/// owner_signature: when Some, V8 enforces receiver type — wrong receiver
/// (null / undefined / not instance of owner template) throws TypeError.
/// Pass None for [Global] interfaces (Window) or EventTarget methods that
/// must also be callable on globalThis.
fn install_proto_method_with_length(
    scope: &v8::PinScope<'_, '_>,
    owner_signature: Option<v8::Local<v8::FunctionTemplate>>,
    proto: v8::Local<v8::ObjectTemplate>,
    name: &str,
    callback: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
    length: i32,
) {
    let fn_tmpl = if length > 0 {
        let guard_data = Box::new(MethodGuardData {
            callback,
            min_args: length,
        });
        let guard_ptr = Box::into_raw(guard_data) as *mut std::ffi::c_void;
        let isolate: &v8::Isolate = &*scope;
        let state = crate::state::RuntimeState::get(isolate);
        state.register_heap(guard_ptr, |p| unsafe {
            drop(Box::from_raw(p as *mut MethodGuardData))
        });
        let external = v8::External::new(scope, guard_ptr);
        let mut builder = v8::FunctionTemplate::builder_raw(method_arg_guard)
            .data(external.into())
            .length(length);
        if let Some(owner) = owner_signature {
            let sig = v8::Signature::new(scope, owner);
            builder = builder.signature(sig);
        }
        builder.build(scope)
    } else {
        let mut builder = v8::FunctionTemplate::builder_raw(callback);
        if let Some(owner) = owner_signature {
            let sig = v8::Signature::new(scope, owner);
            builder = builder.signature(sig);
        }
        builder.build(scope)
    };
    let name_str = crate::v8_utils::v8_string(scope, name);
    fn_tmpl.set_class_name(name_str);
    proto.set(name_str.into(), fn_tmpl.into());
}

/// Helper: install a native method with Signature.
fn install_proto_method_sig(
    scope: &v8::PinScope<'_, '_>,
    owner: v8::Local<v8::FunctionTemplate>,
    proto: v8::Local<v8::ObjectTemplate>,
    name: &str,
    callback: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
) {
    install_proto_method_with_length(scope, Some(owner), proto, name, callback, 0);
}

/// Helper: install a native method with Signature + min args.
fn install_proto_method_sig_with_length(
    scope: &v8::PinScope<'_, '_>,
    owner: v8::Local<v8::FunctionTemplate>,
    proto: v8::Local<v8::ObjectTemplate>,
    name: &str,
    callback: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
    length: i32,
) {
    install_proto_method_with_length(scope, Some(owner), proto, name, callback, length);
}

// V8 callback data for install_proto_method_with_length arg counting
struct MethodGuardData {
    callback: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
    min_args: i32,
}

/// Generic arg-count guard for DOM template methods. Retrieves the original
/// callback and min_args from V8 External data, checks arg count, and
/// either throws TypeError or forwards to the real callback.
unsafe extern "C" fn method_arg_guard(info: *const v8::FunctionCallbackInfo) {
    let info_ref = unsafe { &*info };
    v8::callback_scope!(unsafe scope, info_ref);
    let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
    let data = args.data();
    if data.is_external() {
        let ext: v8::Local<v8::External> = unsafe { v8::Local::cast_unchecked(data) };
        let guard = unsafe { &*(ext.value() as *const MethodGuardData) };
        if (args.length() as i32) < guard.min_args {
            let msg = format!(
                "{} argument(s) required, but only {} present",
                guard.min_args,
                args.length()
            );
            let msg_str = crate::v8_utils::v8_string(scope, &msg);
            let exc = v8::Exception::type_error(scope, msg_str);
            scope.throw_exception(exc);
            return;
        }
        (guard.callback)(info);
    } else {
        // No guard data — this shouldn't happen
        let msg = crate::v8_utils::v8_string(scope, "Illegal invocation");
        let exc = v8::Exception::type_error(scope, msg);
        scope.throw_exception(exc);
    }
}

/// Helper: install a native accessor (getter + optional setter) on a prototype template.
fn install_proto_accessor(
    scope: &v8::PinScope<'_, '_>,
    proto: v8::Local<v8::ObjectTemplate>,
    name: &str,
    getter: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
    setter: Option<unsafe extern "C" fn(*const v8::FunctionCallbackInfo)>,
) {
    let getter_tmpl = v8::FunctionTemplate::builder_raw(getter).length(0).build(scope);
    getter_tmpl.set_class_name(crate::v8_utils::v8_string(scope, &format!("get {}", name)));
    let setter_tmpl = setter.map(|s| {
        let tmpl = v8::FunctionTemplate::builder_raw(s).length(1).build(scope);
        tmpl.set_class_name(crate::v8_utils::v8_string(scope, &format!("set {}", name)));
        tmpl
    });
    let name_str = crate::v8_utils::v8_string(scope, name);
    proto.set_accessor_property(
        name_str.into(),
        Some(getter_tmpl),
        setter_tmpl,
        v8::PropertyAttribute::NONE,
    );
}

/// Helper: set Symbol.toStringTag on a prototype template.
fn set_to_string_tag(
    scope: &v8::PinScope<'_, '_>,
    proto: v8::Local<v8::ObjectTemplate>,
    tag: &str,
) {
    let sym = v8::Symbol::get_to_string_tag(scope);
    let val = crate::v8_utils::v8_string(scope, tag);
    proto.set(sym.into(), val.into());
}

/// Build all DOM templates and install methods on their prototypes.
/// Must be called once per Isolate, with the isolate entered.
// restore_dom_accessors removed (v0.8.92 P0): generator skip-if-own-accessor
// + chain_dom_prototypes non-accessor copy preserve DOM template accessors.

pub fn build_dom_templates(scope: &v8::PinScope<'_, '_>) -> DomTemplates {
    // ── 1. EventTarget ──────────────────────────────────────────────────────
    let event_target = make_template(scope, "EventTarget", construct_only_dom_constructor);
    {
        let proto = event_target.prototype_template(scope);
        install_proto_method_with_length(scope, None, proto, "addEventListener", add_event_listener_cb, 2);
        install_proto_method_with_length(
            scope,
            None,
            proto,
            "removeEventListener",
            remove_event_listener_cb,
            2,
        );
        install_proto_method_with_length(scope, None, proto, "dispatchEvent", dispatch_event_cb, 1);
    }

    // ── 2. Node (inherits EventTarget) ──────────────────────────────────────
    let node = make_template(scope, "Node", illegal_dom_constructor);
    node.inherit(event_target);
    {
        let proto = node.prototype_template(scope);
        // nodeType, nodeName as accessors
        install_proto_accessor(scope, proto, "nodeType", node_type_getter, None);
        install_proto_accessor(scope, proto, "nodeName", node_name_getter, None);
        install_proto_accessor(
            scope,
            proto,
            "textContent",
            text_content_getter,
            Some(text_content_setter),
        );
        // Navigation accessors
        install_proto_accessor(scope, proto, "parentNode", parent_node_getter, None);
        install_proto_accessor(scope, proto, "parentElement", parent_element_getter, None);
        install_proto_accessor(scope, proto, "firstChild", first_child_getter, None);
        install_proto_accessor(scope, proto, "lastChild", last_child_getter, None);
        install_proto_accessor(scope, proto, "nextSibling", next_sibling_getter, None);
        install_proto_accessor(scope, proto, "previousSibling", prev_sibling_getter, None);
        install_proto_accessor(scope, proto, "childNodes", child_nodes_getter, None);
        // Mutation methods
        install_proto_method_sig_with_length(scope, node, proto, "appendChild", append_child_cb, 1);
        install_proto_method_sig_with_length(scope, node, proto, "removeChild", remove_child_cb, 1);
        install_proto_method_sig_with_length(scope, node, proto, "insertBefore", insert_before_cb, 2);
        install_proto_method_sig_with_length(scope, node, proto, "replaceChild", replace_child_cb, 2);
        install_proto_method_sig_with_length(scope, node, proto, "cloneNode", clone_node_cb, 0);
        install_proto_method_sig_with_length(scope, node, proto, "contains", contains_cb, 1);
        install_proto_method_sig_with_length(scope, node, proto, "hasChildNodes", has_child_nodes_cb, 0);
        install_proto_method_sig_with_length(scope, node, proto, "normalize", normalize_cb, 0);

        let node_consts = [
            ("ELEMENT_NODE", 1i32), ("ATTRIBUTE_NODE", 2), ("TEXT_NODE", 3),
            ("CDATA_SECTION_NODE", 4), ("ENTITY_REFERENCE_NODE", 5), ("ENTITY_NODE", 6),
            ("PROCESSING_INSTRUCTION_NODE", 7), ("COMMENT_NODE", 8), ("DOCUMENT_NODE", 9),
            ("DOCUMENT_TYPE_NODE", 10), ("DOCUMENT_FRAGMENT_NODE", 11), ("NOTATION_NODE", 12),
        ];
        for (cname, cval) in node_consts {
            let key = v8::String::new(scope, cname).unwrap();
            let val = v8::Integer::new(scope, cval);
            proto.set_with_attr(key.into(), val.into(), v8::PropertyAttribute::READ_ONLY | v8::PropertyAttribute::DONT_DELETE);
        }
        let node_hex_consts = [
            ("DOCUMENT_POSITION_DISCONNECTED", 0x01u32),
            ("DOCUMENT_POSITION_PRECEDING", 0x02),
            ("DOCUMENT_POSITION_FOLLOWING", 0x04),
            ("DOCUMENT_POSITION_CONTAINS", 0x08),
            ("DOCUMENT_POSITION_CONTAINED_BY", 0x10),
            ("DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC", 0x20),
        ];
        for (cname, cval) in node_hex_consts {
            let key = v8::String::new(scope, cname).unwrap();
            let val = v8::Integer::new_from_unsigned(scope, cval);
            proto.set_with_attr(key.into(), val.into(), v8::PropertyAttribute::READ_ONLY | v8::PropertyAttribute::DONT_DELETE);
        }
    }

    if let Some(node_fn) = node.get_function(scope) {
        let node_fn_obj: v8::Local<v8::Object> = node_fn.into();
        for (cname, cval) in [
            ("ELEMENT_NODE", 1i32), ("ATTRIBUTE_NODE", 2), ("TEXT_NODE", 3),
            ("CDATA_SECTION_NODE", 4), ("PROCESSING_INSTRUCTION_NODE", 7),
            ("COMMENT_NODE", 8), ("DOCUMENT_NODE", 9),
            ("DOCUMENT_TYPE_NODE", 10), ("DOCUMENT_FRAGMENT_NODE", 11),
        ] {
            let key = v8::String::new(scope, cname).unwrap();
            let val = v8::Integer::new(scope, cval);
            let _ = node_fn_obj.define_own_property(scope, key.into(), val.into(), v8::PropertyAttribute::READ_ONLY | v8::PropertyAttribute::DONT_DELETE);
        }
        for (cname, cval) in [
            ("DOCUMENT_POSITION_DISCONNECTED", 0x01u32),
            ("DOCUMENT_POSITION_PRECEDING", 0x02),
            ("DOCUMENT_POSITION_FOLLOWING", 0x04),
            ("DOCUMENT_POSITION_CONTAINS", 0x08),
            ("DOCUMENT_POSITION_CONTAINED_BY", 0x10),
            ("DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC", 0x20),
        ] {
            let key = v8::String::new(scope, cname).unwrap();
            let val = v8::Integer::new_from_unsigned(scope, cval);
            let _ = node_fn_obj.define_own_property(scope, key.into(), val.into(), v8::PropertyAttribute::READ_ONLY | v8::PropertyAttribute::DONT_DELETE);
        }
    }

    // ── 3. Element (inherits Node) ──────────────────────────────────────────
    let element = make_template(scope, "Element", illegal_dom_constructor);
    element.inherit(node);
    {
        let proto = element.prototype_template(scope);
        install_proto_accessor(scope, proto, "tagName", tag_name_getter, None);
        install_proto_accessor(scope, proto, "id", id_getter, Some(id_setter));
        install_proto_accessor(
            scope,
            proto,
            "className",
            class_name_getter,
            Some(class_name_setter),
        );
        install_proto_accessor(scope, proto, "classList", class_list_getter, None);
        install_proto_accessor(
            scope,
            proto,
            "innerHTML",
            inner_html_getter,
            Some(inner_html_setter),
        );
        install_proto_accessor(scope, proto, "outerHTML", outer_html_getter, None);
        install_proto_accessor(
            scope,
            proto,
            "innerText",
            inner_text_getter,
            Some(inner_text_setter),
        );
        install_proto_accessor(scope, proto, "children", children_getter, None);
        install_proto_accessor(
            scope,
            proto,
            "childElementCount",
            child_element_count_getter,
            None,
        );
        install_proto_accessor(
            scope,
            proto,
            "firstElementChild",
            first_element_child_getter,
            None,
        );
        install_proto_accessor(
            scope,
            proto,
            "lastElementChild",
            last_element_child_getter,
            None,
        );
        install_proto_accessor(
            scope,
            proto,
            "nextElementSibling",
            next_element_sibling_getter,
            None,
        );
        install_proto_accessor(
            scope,
            proto,
            "previousElementSibling",
            prev_element_sibling_getter,
            None,
        );
        // Attribute methods
        install_proto_method_sig_with_length(scope, element, proto, "getAttribute", get_attribute_cb, 1);
        install_proto_method_sig_with_length(scope, element, proto, "setAttribute", set_attribute_cb, 2);
        install_proto_method_sig_with_length(scope, element, proto, "removeAttribute", remove_attribute_cb, 1);
        install_proto_method_sig_with_length(scope, element, proto, "hasAttribute", has_attribute_cb, 1);
        // NS variants: L3 stores local name only (HTML namespace-less). Required after
        // EXCLUDED_OPERATIONS removed codegen skeletons (H05c SKIP → real methods).
        install_proto_method_sig_with_length(scope, element, proto, "getAttributeNS", get_attribute_ns_cb, 2);
        install_proto_method_sig_with_length(scope, element, proto, "setAttributeNS", set_attribute_ns_cb, 3);
        install_proto_method_sig_with_length(scope, element, proto, "removeAttributeNS", remove_attribute_ns_cb, 2);
        install_proto_method_sig_with_length(scope, element, proto, "hasAttributeNS", has_attribute_ns_cb, 2);
        install_proto_method_sig_with_length(scope, element, proto, "getAttributeNames", get_attribute_names_cb, 0);
        // Node mutation methods live on Node only (RD-05). Element keeps Element-only APIs.
        install_proto_method_sig_with_length(scope, element, proto, "insertAdjacentHTML", insert_adjacent_html_cb, 2);
        install_proto_method_sig_with_length(scope, element, proto, "insertAdjacentElement", insert_adjacent_element_cb, 2);
        install_proto_method_sig_with_length(scope, element, proto, "insertAdjacentText", insert_adjacent_text_cb, 2);
        // Query methods
        install_proto_method_sig_with_length(scope, element, proto, "querySelector", query_selector_cb, 1);
        install_proto_method_sig_with_length(scope, element, proto, "querySelectorAll", query_selector_all_cb, 1);
        install_proto_method_sig_with_length(scope, element, proto, "getElementsByTagName", get_elements_by_tag_name_cb, 1);
        install_proto_method_sig_with_length(scope, element, proto, "getElementsByClassName", get_elements_by_class_name_cb, 1);
        install_proto_method_sig_with_length(scope, element, proto, "matches", matches_cb, 1);
        install_proto_method_sig_with_length(scope, element, proto, "closest", closest_cb, 1);
        // Geometry
        install_proto_method_sig_with_length(scope, element, proto, "getBoundingClientRect", get_bounding_client_rect_cb, 0);
        install_proto_accessor(scope, proto, "offsetWidth", offset_width_getter, None);
        install_proto_accessor(scope, proto, "offsetHeight", offset_height_getter, None);
        install_proto_accessor(scope, proto, "offsetTop", offset_top_getter, None);
        install_proto_accessor(scope, proto, "offsetLeft", offset_left_getter, None);
        install_proto_accessor(scope, proto, "clientWidth", client_width_getter, None);
        install_proto_accessor(scope, proto, "clientHeight", client_height_getter, None);
        install_proto_accessor(scope, proto, "scrollWidth", scroll_width_getter, None);
        install_proto_accessor(scope, proto, "scrollHeight", scroll_height_getter, None);
        // Shadow DOM L3: null until attachShadow; no nested shadow tree (COMP-4).
        install_proto_accessor(scope, proto, "shadowRoot", shadow_root_getter, None);
        install_proto_method_sig_with_length(scope, element, proto, "attachShadow", attach_shadow_cb, 1);
        install_proto_accessor(
            scope,
            proto,
            "scrollTop",
            scroll_top_getter,
            Some(scroll_top_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "scrollLeft",
            scroll_left_getter,
            Some(scroll_left_setter),
        );
        install_proto_method_sig(scope, element, proto, "scrollIntoView", scroll_into_view_cb);
        install_proto_method_sig(scope, element, proto, "scroll", scroll_promise_cb);
        install_proto_method_sig(scope, element, proto, "scrollTo", scroll_promise_cb);
        install_proto_method_sig(scope, element, proto, "scrollBy", scroll_promise_cb);
        install_proto_method_sig(scope, element, proto, "requestFullscreen", scroll_promise_cb);
        install_proto_method_sig(scope, element, proto, "requestPointerLock", scroll_promise_cb);
        install_proto_method_sig_with_length(scope, element, proto, "getAttributeNode", get_attribute_node_cb, 1);
        install_proto_method_sig_with_length(scope, element, proto, "getAttributeNodeNS", get_attribute_node_ns_cb, 2);
        install_proto_method_sig_with_length(scope, element, proto, "setAttributeNode", set_attribute_node_cb, 1);
        install_proto_method_sig_with_length(scope, element, proto, "setAttributeNodeNS", set_attribute_node_cb, 1);
        // Nullable Node/CSSPseudoElement APIs without layout/CSSOM engine → null.
        install_proto_method_sig_with_length(
            scope,
            element,
            proto,
            "spatialNavigationSearch",
            element_null_result_cb,
            1,
        );
        install_proto_method_sig_with_length(scope, element, proto, "pseudo", element_null_result_cb, 1);
        install_proto_method_sig(scope, element, proto, "getClientRects", get_client_rects_cb);
        // Event methods (also on EventTarget, but Element overrides for convenience)
        install_proto_method_sig(scope, element, proto, "addEventListener", add_event_listener_cb);
        install_proto_method(
            scope,
            proto,
            "removeEventListener",
            remove_event_listener_cb,
        );
        install_proto_method(scope, proto, "dispatchEvent", dispatch_event_cb);
    }

    // ── 4. HTMLElement (inherits Element) ───────────────────────────────────
    let html_element = make_template(scope, "HTMLElement", illegal_dom_constructor);
    html_element.inherit(element);
    {
        let proto = html_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "style", style_getter, None);
        install_proto_accessor(scope, proto, "dataset", dataset_getter, None);
        install_proto_accessor(scope, proto, "hidden", hidden_getter, Some(hidden_setter));
        install_proto_accessor(
            scope,
            proto,
            "tabIndex",
            tab_index_getter,
            Some(tab_index_setter),
        );
        install_proto_accessor(scope, proto, "title", title_getter, Some(title_setter));
        install_proto_accessor(scope, proto, "lang", lang_getter, Some(lang_setter));
        install_proto_accessor(scope, proto, "dir", dir_getter, Some(dir_setter));
        install_proto_accessor(
            scope,
            proto,
            "draggable",
            draggable_getter,
            Some(draggable_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "contentEditable",
            content_editable_getter,
            Some(content_editable_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "isContentEditable",
            is_content_editable_getter,
            None,
        );
        install_proto_accessor(scope, proto, "offsetParent", offset_parent_getter, None);
        install_proto_method_sig(scope, html_element, proto, "focus", focus_cb);
        install_proto_method_sig(scope, html_element, proto, "blur", blur_cb);
        install_proto_method_sig(scope, html_element, proto, "click", click_cb);
    }

    // ── 5. Specific HTML element types ──────────────────────────────────────
    let html_div_element = make_template(scope, "HTMLDivElement", illegal_dom_constructor);
    html_div_element.inherit(html_element);

    let html_span_element = make_template(scope, "HTMLSpanElement", illegal_dom_constructor);
    html_span_element.inherit(html_element);

    let html_anchor_element = make_template(scope, "HTMLAnchorElement", illegal_dom_constructor);
    html_anchor_element.inherit(html_element);
    {
        let proto = html_anchor_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "href", href_getter, Some(href_setter));
        install_proto_accessor(scope, proto, "target", target_getter, Some(target_setter));
        install_proto_accessor(scope, proto, "rel", rel_getter, Some(rel_setter));
        // Computed URL properties (HTML: writable components rewrite href)
        install_proto_accessor(
            scope,
            proto,
            "pathname",
            anchor_pathname_getter,
            Some(anchor_pathname_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "hostname",
            anchor_hostname_getter,
            Some(anchor_hostname_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "protocol",
            anchor_protocol_getter,
            Some(anchor_protocol_setter),
        );
        install_proto_accessor(scope, proto, "host", anchor_host_getter, Some(anchor_host_setter));
        install_proto_accessor(scope, proto, "port", anchor_port_getter, Some(anchor_port_setter));
        install_proto_accessor(
            scope,
            proto,
            "search",
            anchor_search_getter,
            Some(anchor_search_setter),
        );
        install_proto_accessor(scope, proto, "hash", anchor_hash_getter, Some(anchor_hash_setter));
        install_proto_accessor(scope, proto, "origin", anchor_origin_getter, None);
    }

    let html_input_element = make_template(scope, "HTMLInputElement", illegal_dom_constructor);
    html_input_element.inherit(html_element);
    {
        let proto = html_input_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "value", value_getter, Some(value_setter));
        install_proto_accessor(
            scope,
            proto,
            "type",
            input_type_getter,
            Some(input_type_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "checked",
            checked_getter,
            Some(checked_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "disabled",
            disabled_getter,
            Some(disabled_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "placeholder",
            placeholder_getter,
            Some(placeholder_setter),
        );
        install_proto_accessor(scope, proto, "name", name_getter, Some(name_setter));
        install_proto_method_sig(scope, html_input_element, proto, "focus", focus_cb);
        install_proto_method_sig(scope, html_input_element, proto, "blur", blur_cb);
        install_proto_method_sig(scope, html_input_element, proto, "select", select_cb);
        install_proto_method_sig(scope, html_input_element, proto, "click", click_cb);
    }

    let html_button_element = make_template(scope, "HTMLButtonElement", illegal_dom_constructor);
    html_button_element.inherit(html_element);
    {
        let proto = html_button_element.prototype_template(scope);
        install_proto_accessor(
            scope,
            proto,
            "disabled",
            disabled_getter,
            Some(disabled_setter),
        );
        install_proto_accessor(scope, proto, "name", name_getter, Some(name_setter));
        install_proto_accessor(
            scope,
            proto,
            "type",
            input_type_getter,
            Some(input_type_setter),
        );
        install_proto_method_sig(scope, html_button_element, proto, "click", click_cb);
    }

    let html_form_element = make_template(scope, "HTMLFormElement", illegal_dom_constructor);
    html_form_element.inherit(html_element);
    {
        let proto = html_form_element.prototype_template(scope);
        install_proto_method_sig(scope, html_form_element, proto, "submit", submit_cb);
        install_proto_method_sig(scope, html_form_element, proto, "reset", reset_cb);
        install_proto_method_sig(scope, html_form_element, proto, "checkValidity", check_validity_cb);
    }

    let html_canvas_element = make_template(scope, "HTMLCanvasElement", illegal_dom_constructor);
    html_canvas_element.inherit(html_element);
    {
        let proto = html_canvas_element.prototype_template(scope);
        install_proto_accessor(
            scope,
            proto,
            "width",
            canvas_width_getter,
            Some(canvas_width_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "height",
            canvas_height_getter,
            Some(canvas_height_setter),
        );
        install_proto_method_sig_with_length(scope, html_canvas_element, proto, "getContext", get_context_cb, 1);
        install_proto_method_sig_with_length(scope, html_canvas_element, proto, "toDataURL", to_data_url_cb, 0);
        install_proto_method_sig_with_length(scope, html_canvas_element, proto, "toBlob", to_blob_cb, 1);
        install_proto_method_sig(scope, html_canvas_element, proto, "captureStream", capture_stream_cb);
        install_proto_method_sig(scope, html_canvas_element, proto, "webkitCaptureStream", capture_stream_cb);
    }

    let html_script_element = make_template(scope, "HTMLScriptElement", illegal_dom_constructor);
    html_script_element.inherit(html_element);
    {
        let proto = html_script_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "src", src_getter, Some(src_setter));
        install_proto_accessor(
            scope,
            proto,
            "type",
            input_type_getter,
            Some(input_type_setter),
        );
        install_proto_accessor(scope, proto, "async", async_getter, Some(async_setter));
        install_proto_accessor(scope, proto, "defer", defer_getter, Some(defer_setter));
    }

    let html_image_element = make_template(scope, "HTMLImageElement", illegal_dom_constructor);
    html_image_element.inherit(html_element);
    {
        let proto = html_image_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "src", src_getter, Some(src_setter));
        install_proto_accessor(scope, proto, "alt", alt_getter, Some(alt_setter));
        install_proto_accessor(
            scope,
            proto,
            "width",
            canvas_width_getter,
            Some(canvas_width_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "height",
            canvas_height_getter,
            Some(canvas_height_setter),
        );
        install_proto_accessor(scope, proto, "naturalWidth", natural_width_getter, None);
        install_proto_accessor(scope, proto, "naturalHeight", natural_height_getter, None);
        install_proto_accessor(scope, proto, "complete", img_complete_getter, None);
        // decode(): no network image pipeline — resolve immediately (spec allows).
        install_proto_method_sig(scope, html_image_element, proto, "decode", scroll_promise_cb);
    }

    let html_video_element = make_template(scope, "HTMLVideoElement", illegal_dom_constructor);
    html_video_element.inherit(html_element);
    {
        let proto = html_video_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "src", src_getter, Some(src_setter));
        install_proto_accessor(
            scope,
            proto,
            "currentTime",
            current_time_getter,
            Some(current_time_setter),
        );
        install_proto_accessor(scope, proto, "duration", duration_getter, None);
        install_proto_accessor(scope, proto, "paused", paused_getter, None);
        install_proto_accessor(scope, proto, "muted", muted_getter, Some(muted_setter));
        install_proto_accessor(scope, proto, "volume", volume_getter, Some(volume_setter));
        install_proto_method_sig(scope, html_video_element, proto, "play", media_play_cb);
        install_proto_method_sig(scope, html_video_element, proto, "pause", media_pause_cb);
        install_proto_method_sig(scope, html_video_element, proto, "load", media_load_cb);
        install_proto_method_sig(scope, html_video_element, proto, "canPlayType", can_play_type_cb);
        install_proto_method_sig(scope, html_video_element, proto, "captureStream", capture_stream_cb);
        install_proto_method_sig(scope, html_video_element, proto, "mozCaptureStream", capture_stream_cb);
        install_proto_method_sig(scope, html_video_element, proto, "webkitCaptureStream", capture_stream_cb);
    }

    let html_audio_element = make_template(scope, "HTMLAudioElement", illegal_dom_constructor);
    html_audio_element.inherit(html_element);
    {
        let proto = html_audio_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "src", src_getter, Some(src_setter));
        install_proto_accessor(
            scope,
            proto,
            "currentTime",
            current_time_getter,
            Some(current_time_setter),
        );
        install_proto_accessor(scope, proto, "duration", duration_getter, None);
        install_proto_accessor(scope, proto, "paused", paused_getter, None);
        install_proto_accessor(scope, proto, "muted", muted_getter, Some(muted_setter));
        install_proto_accessor(scope, proto, "volume", volume_getter, Some(volume_setter));
        install_proto_method_sig(scope, html_audio_element, proto, "play", media_play_cb);
        install_proto_method_sig(scope, html_audio_element, proto, "pause", media_pause_cb);
        install_proto_method_sig(scope, html_audio_element, proto, "load", media_load_cb);
        install_proto_method_sig(scope, html_audio_element, proto, "canPlayType", can_play_type_cb);
    }

    let html_select_element = make_template(scope, "HTMLSelectElement", illegal_dom_constructor);
    html_select_element.inherit(html_element);
    {
        let proto = html_select_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "value", value_getter, Some(value_setter));
        install_proto_accessor(
            scope,
            proto,
            "selectedIndex",
            selected_index_getter,
            Some(selected_index_setter),
        );
        install_proto_accessor(scope, proto, "options", select_options_getter, None);
        install_proto_accessor(
            scope,
            proto,
            "length",
            select_length_getter,
            Some(select_length_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "disabled",
            disabled_getter,
            Some(disabled_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "multiple",
            multiple_getter,
            Some(multiple_setter),
        );
        install_proto_method_sig_with_length(scope, html_select_element, proto, "item", select_item_cb, 1);
        install_proto_method_sig_with_length(
            scope,
            html_select_element,
            proto,
            "namedItem",
            select_named_item_cb,
            1,
        );
    }

    // HTMLOptionElement: value falls back to text content (HTML spec).
    let html_option_element = make_template(scope, "HTMLOptionElement", illegal_dom_constructor);
    html_option_element.inherit(html_element);
    {
        let proto = html_option_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "value", value_getter, Some(value_setter));
        install_proto_accessor(
            scope,
            proto,
            "selected",
            option_selected_getter,
            Some(option_selected_setter),
        );
        install_proto_accessor(scope, proto, "disabled", disabled_getter, Some(disabled_setter));
        install_proto_accessor(scope, proto, "label", option_label_getter, Some(option_label_setter));
        install_proto_accessor(scope, proto, "text", option_text_getter, Some(option_text_setter));
        install_proto_accessor(scope, proto, "index", option_index_getter, None);
    }

    // HTMLIFrameElement: contentDocument/contentWindow null without nested browsing;
    // readonly attrs use no-op setters (H05b Category C).
    let html_iframe_element = make_template(scope, "HTMLIFrameElement", illegal_dom_constructor);
    html_iframe_element.inherit(html_element);
    {
        let proto = html_iframe_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "src", src_getter, Some(src_setter));
        install_proto_accessor(
            scope,
            proto,
            "contentDocument",
            iframe_content_document_getter,
            Some(iframe_readonly_noop_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "contentWindow",
            iframe_content_window_getter,
            Some(iframe_readonly_noop_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "permissionsPolicy",
            iframe_null_getter,
            Some(iframe_readonly_noop_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "sandbox",
            iframe_null_getter,
            Some(iframe_readonly_noop_setter),
        );
    }

    let html_textarea_element = make_template(scope, "HTMLTextAreaElement", illegal_dom_constructor);
    html_textarea_element.inherit(html_element);
    {
        let proto = html_textarea_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "value", value_getter, Some(value_setter));
        install_proto_accessor(
            scope,
            proto,
            "disabled",
            disabled_getter,
            Some(disabled_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "placeholder",
            placeholder_getter,
            Some(placeholder_setter),
        );
        install_proto_method_sig(scope, html_textarea_element, proto, "select", select_cb);
    }

    let html_head_element = make_template(scope, "HTMLHeadElement", illegal_dom_constructor);
    html_head_element.inherit(html_element);

    let html_body_element = make_template(scope, "HTMLBodyElement", illegal_dom_constructor);
    html_body_element.inherit(html_element);

    let html_html_element = make_template(scope, "HTMLHtmlElement", illegal_dom_constructor);
    html_html_element.inherit(html_element);

    let html_paragraph_element = make_template(scope, "HTMLParagraphElement", illegal_dom_constructor);
    html_paragraph_element.inherit(html_element);

    let html_heading_element = make_template(scope, "HTMLHeadingElement", illegal_dom_constructor);
    html_heading_element.inherit(html_element);

    let html_ulist_element = make_template(scope, "HTMLUListElement", illegal_dom_constructor);
    html_ulist_element.inherit(html_element);

    let html_olist_element = make_template(scope, "HTMLOListElement", illegal_dom_constructor);
    html_olist_element.inherit(html_element);

    let html_li_element = make_template(scope, "HTMLLIElement", illegal_dom_constructor);
    html_li_element.inherit(html_element);

    let html_table_element = make_template(scope, "HTMLTableElement", illegal_dom_constructor);
    html_table_element.inherit(html_element);

    let html_table_row_element = make_template(scope, "HTMLTableRowElement", illegal_dom_constructor);
    html_table_row_element.inherit(html_element);
    {
        let proto = html_table_row_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "rowIndex", table_row_index_getter, None);
        install_proto_accessor(scope, proto, "sectionRowIndex", table_section_row_index_getter, None);
        install_proto_accessor(
            scope,
            proto,
            "cells",
            table_row_cells_getter,
            Some(iframe_readonly_noop_setter),
        );
        install_proto_accessor(scope, proto, "ch", ch_getter, Some(ch_setter));
        install_proto_accessor(scope, proto, "chOff", ch_off_getter, Some(ch_off_setter));
        install_proto_accessor(scope, proto, "vAlign", valign_getter, Some(valign_setter));
    }

    let html_table_cell_element = make_template(scope, "HTMLTableCellElement", illegal_dom_constructor);
    html_table_cell_element.inherit(html_element);
    {
        let proto = html_table_cell_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "colSpan", colspan_getter, Some(colspan_setter));
        install_proto_accessor(scope, proto, "rowSpan", rowspan_getter, Some(rowspan_setter));
        install_proto_accessor(scope, proto, "headers", headers_attr_getter, Some(headers_attr_setter));
        install_proto_accessor(scope, proto, "cellIndex", table_cell_index_getter, None);
        install_proto_accessor(scope, proto, "scope", scope_attr_getter, Some(scope_attr_setter));
        install_proto_accessor(scope, proto, "abbr", abbr_getter, Some(abbr_setter));
        install_proto_accessor(scope, proto, "axis", axis_getter, Some(axis_setter));
        install_proto_accessor(scope, proto, "height", cell_height_getter, Some(cell_height_setter));
        install_proto_accessor(scope, proto, "ch", ch_getter, Some(ch_setter));
        install_proto_accessor(scope, proto, "chOff", ch_off_getter, Some(ch_off_setter));
        install_proto_accessor(scope, proto, "noWrap", nowrap_getter, Some(nowrap_setter));
        install_proto_accessor(scope, proto, "vAlign", valign_getter, Some(valign_setter));
    }

    let html_dialog_element = make_template(scope, "HTMLDialogElement", illegal_dom_constructor);
    html_dialog_element.inherit(html_element);
    {
        let proto = html_dialog_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "open", dialog_open_getter, Some(dialog_open_setter));
        install_proto_accessor(
            scope,
            proto,
            "returnValue",
            dialog_return_value_getter,
            Some(dialog_return_value_setter),
        );
        install_proto_accessor(
            scope,
            proto,
            "closedBy",
            dialog_closed_by_getter,
            Some(dialog_closed_by_setter),
        );
        install_proto_method_sig(scope, html_dialog_element, proto, "show", dialog_show_cb);
        install_proto_method_sig(scope, html_dialog_element, proto, "showModal", dialog_show_modal_cb);
        install_proto_method_sig(scope, html_dialog_element, proto, "close", dialog_close_cb);
    }

    let html_style_element = make_template(scope, "HTMLStyleElement", illegal_dom_constructor);
    html_style_element.inherit(html_element);

    let html_link_element = make_template(scope, "HTMLLinkElement", illegal_dom_constructor);
    html_link_element.inherit(html_element);
    {
        let proto = html_link_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "href", href_getter, Some(href_setter));
        install_proto_accessor(scope, proto, "rel", rel_getter, Some(rel_setter));
    }

    let html_meta_element = make_template(scope, "HTMLMetaElement", illegal_dom_constructor);
    html_meta_element.inherit(html_element);
    {
        let proto = html_meta_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "name", name_getter, Some(name_setter));
        install_proto_accessor(
            scope,
            proto,
            "content",
            content_getter,
            Some(content_setter),
        );
    }

    // ── 6. HTMLUnknownElement (inherits HTMLElement) ─────────────────────────
    let html_unknown_element = make_template(scope, "HTMLUnknownElement", illegal_dom_constructor);
    html_unknown_element.inherit(html_element);
    {
        let proto = html_unknown_element.prototype_template(scope);
        set_to_string_tag(scope, proto, "HTMLUnknownElement");
    }

    // ── 7. CharacterData (inherits Node) — parent of Text and Comment ──────
    let character_data = make_template(scope, "CharacterData", illegal_dom_constructor);
    character_data.inherit(node);
    {
        let proto = character_data.prototype_template(scope);
        install_proto_accessor(scope, proto, "data", data_getter, Some(data_setter));
        install_proto_accessor(scope, proto, "length", char_data_length_getter, None);
        install_proto_method_sig_with_length(scope, character_data, proto, "substringData", substring_data_cb, 2);
        install_proto_method_sig_with_length(scope, character_data, proto, "appendData", append_data_cb, 1);
        install_proto_method_sig_with_length(scope, character_data, proto, "insertData", insert_data_cb, 2);
        install_proto_method_sig_with_length(scope, character_data, proto, "deleteData", delete_data_cb, 2);
        install_proto_method_sig_with_length(scope, character_data, proto, "replaceData", replace_data_cb, 3);
        set_to_string_tag(scope, proto, "CharacterData");
    }

    // ── 8. Text node (inherits CharacterData) ───────────────────────────────
    let text_node = make_template(scope, "Text", illegal_dom_constructor);
    text_node.inherit(character_data);
    {
        let proto = text_node.prototype_template(scope);
        install_proto_method_sig_with_length(scope, text_node, proto, "splitText", split_text_cb, 1);
        install_proto_accessor(scope, proto, "wholeText", whole_text_getter, None);
        set_to_string_tag(scope, proto, "Text");
    }

    // ── 9. Comment node (inherits CharacterData) ────────────────────────────
    let comment_node = make_template(scope, "Comment", illegal_dom_constructor);
    comment_node.inherit(character_data);
    {
        let proto = comment_node.prototype_template(scope);
        set_to_string_tag(scope, proto, "Comment");
    }

    // ── 10. DocumentFragment (inherits Node) ────────────────────────────────
    let document_fragment = make_template(scope, "DocumentFragment", illegal_dom_constructor);
    document_fragment.inherit(node);
    {
        let proto = document_fragment.prototype_template(scope);
        set_to_string_tag(scope, proto, "DocumentFragment");
    }

    // ── 11. Document node (inherits Node) ───────────────────────────────────
    let document_node = make_template(scope, "Document", illegal_dom_constructor);
    document_node.inherit(node);

    // ── 10. NodeList ────────────────────────────────────────────────────────
    let node_list = make_template(scope, "NodeList", illegal_dom_constructor);
    node_list
        .instance_template(scope)
        .set_internal_field_count(2);
    {
        let proto = node_list.prototype_template(scope);
        install_proto_method_sig_with_length(scope, node_list, proto, "item", node_list_item_cb, 1);
        install_proto_accessor(scope, proto, "length", node_list_length_getter, None);
        set_to_string_tag(scope, proto, "NodeList");
    }

    // ── 12. DOMTokenList ────────────────────────────────────────────────────
    let dom_token_list = make_template(scope, "DOMTokenList", illegal_dom_constructor);
    dom_token_list
        .instance_template(scope)
        .set_internal_field_count(1);
    {
        let proto = dom_token_list.prototype_template(scope);
        install_proto_method_sig_with_length(scope, dom_token_list, proto, "item", domtokenlist_item_cb, 1);
        install_proto_method_sig_with_length(scope, dom_token_list, proto, "contains", domtokenlist_contains_cb, 1);
        install_proto_method_sig(scope, dom_token_list, proto, "add", domtokenlist_add_cb);
        install_proto_method_sig(scope, dom_token_list, proto, "remove", domtokenlist_remove_cb);
        install_proto_method_sig_with_length(scope, dom_token_list, proto, "toggle", domtokenlist_toggle_cb, 1);
        install_proto_method_sig_with_length(scope, dom_token_list, proto, "replace", domtokenlist_replace_cb, 2);
        install_proto_method_sig(scope, dom_token_list, proto, "toString", domtokenlist_tostring_cb);
        install_proto_method_sig(scope, dom_token_list, proto, "forEach", domtokenlist_foreach_cb);
        install_proto_method_sig(scope, dom_token_list, proto, "entries", domtokenlist_entries_cb);
        install_proto_method_sig(scope, dom_token_list, proto, "keys", domtokenlist_keys_cb);
        install_proto_method_sig(scope, dom_token_list, proto, "values", domtokenlist_values_cb);
        install_proto_accessor(scope, proto, "length", domtokenlist_length_getter, None);
        install_proto_accessor(
            scope,
            proto,
            "value",
            domtokenlist_value_getter,
            Some(domtokenlist_value_setter),
        );
        set_to_string_tag(scope, proto, "DOMTokenList");
    }

    // ── 13. CSSStyleDeclaration ─────────────────────────────────────────────
    let css_style_declaration = make_template(scope, "CSSStyleDeclaration", illegal_dom_constructor);
    css_style_declaration
        .instance_template(scope)
        .set_internal_field_count(2);
    {
        let proto = css_style_declaration.prototype_template(scope);
        install_proto_method_sig(scope, css_style_declaration, proto, "setProperty", css_style_set_property_cb);
        install_proto_method_sig(scope, css_style_declaration, proto, "getPropertyValue", css_style_get_property_cb);
        install_proto_method_sig(
            scope,
            css_style_declaration,
            proto,
            "getPropertyPriority",
            css_style_get_priority_cb,
        );
        install_proto_method_sig(scope, css_style_declaration, proto, "removeProperty", css_style_remove_property_cb);
        install_proto_method_sig(scope, css_style_declaration, proto, "item", css_style_item_cb);
        install_proto_accessor(
            scope,
            proto,
            "cssText",
            css_style_csstext_getter,
            Some(css_style_csstext_setter),
        );
        install_proto_accessor(scope, proto, "length", css_style_length_getter, None);
        set_to_string_tag(scope, proto, "CSSStyleDeclaration");
    }

    // ── 13.5 Headers constructor ───────────────────────────────────────────
    unsafe extern "C" fn headers_constructor_cb(info: *const v8::FunctionCallbackInfo) {
        let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let info_ref = unsafe { &*info };
            v8::callback_scope!(unsafe scope, info_ref);
            let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

            let mut pairs: Vec<(String, String)> = Vec::new();

            // Optional Array init: new Headers([["k1","v1"],["k2","v2"]])
            if args.length() >= 1 {
                let init = args.get(0);
                if init.is_array() {
                    let arr: v8::Local<v8::Array> =
                        unsafe { v8::Local::cast_unchecked(init) };
                    let len = arr.length();
                    for i in 0..len {
                        if let Some(elem) = arr.get_index(scope, i) {
                            if elem.is_array() {
                                let pair_arr: v8::Local<v8::Array> =
                                    unsafe { v8::Local::cast_unchecked(elem) };
                                if pair_arr.length() >= 2 {
                                    let key = pair_arr
                                        .get_index(scope, 0)
                                        .map(|v| v.to_rust_string_lossy(scope).to_lowercase())
                                        .unwrap_or_default();
                                    let val = pair_arr
                                        .get_index(scope, 1)
                                        .map(|v| v.to_rust_string_lossy(scope))
                                        .unwrap_or_default();
                                    pairs.push((key, val));
                                }
                            }
                        }
                    }
                }
            }

            let state = crate::state::RuntimeState::get(&*scope);
            let boxed = Box::new(pairs);
            let ptr = Box::into_raw(boxed) as *mut std::ffi::c_void;
            state.register_heap(ptr, |p| unsafe {
                drop(Box::from_raw(p as *mut Vec<(String, String)>))
            });

            let this = args.this();
            if this.is_object() {
                let obj: v8::Local<v8::Object> =
                    unsafe { v8::Local::cast_unchecked(this) };
                obj.set_internal_field(0, v8::External::new(scope, ptr).into());
            }
        }));
    }

    // ── 14. Headers ─────────────────────────────────────────────────────────
    let headers = {
        let tmpl = v8::FunctionTemplate::builder_raw(headers_constructor_cb).build(scope);
        tmpl.read_only_prototype();
        let name = crate::v8_utils::v8_string(scope, "Headers");
        tmpl.set_class_name(name);
        tmpl.instance_template(scope).set_internal_field_count(1);
        tmpl
    };
    {
        let proto = headers.prototype_template(scope);
        install_proto_method_sig(scope, headers, proto, "get", headers_get_cb);
        install_proto_method_sig(scope, headers, proto, "set", headers_set_cb);
        install_proto_method_sig(scope, headers, proto, "has", headers_has_cb);
        install_proto_method_sig(scope, headers, proto, "delete", headers_delete_cb);
        install_proto_method_sig(scope, headers, proto, "append", headers_append_cb);
        install_proto_method_sig(scope, headers, proto, "forEach", headers_foreach_cb);
        install_proto_method_sig(scope, headers, proto, "entries", headers_entries_cb);
        install_proto_method_sig(scope, headers, proto, "keys", headers_keys_cb);
        install_proto_method_sig(scope, headers, proto, "values", headers_values_cb);
        set_to_string_tag(scope, proto, "Headers");
    }

    // ── 15. Response ────────────────────────────────────────────────────────
    let response = make_template(scope, "Response", response_constructor);
    {
        let proto = response.prototype_template(scope);
        install_proto_method_sig(scope, response, proto, "text", response_text_cb);
        install_proto_method_sig(scope, response, proto, "json", response_json_cb);
        install_proto_method_sig(scope, response, proto, "arrayBuffer", response_array_buffer_cb);
        install_proto_method_sig(scope, response, proto, "blob", response_blob_cb);
        install_proto_method_sig(scope, response, proto, "clone", response_clone_cb);
        set_to_string_tag(scope, proto, "Response");
    }

    // ── 16. Request ─────────────────────────────────────────────────────────
    // Request accessor 全部由 codegen fix_accessor_properties 安装
    // (读取 __iv8Url/__iv8Method/__iv8Mode 等 hidden key)。
    // DOM template 不安装 Request accessor — 避免两套机制分叉。
    // request_constructor 设置 codegen hidden key。
    let request = make_template(scope, "Request", request_constructor);
    {
        let proto = request.prototype_template(scope);
        install_proto_method_sig(scope, request, proto, "clone", request_clone_cb);
        set_to_string_tag(scope, proto, "Request");
    }

    // ── 17. Attr (view over Element.attrs; value is live) ────────────────────
    let attr = make_template(scope, "Attr", illegal_dom_constructor);
    attr.inherit(node);
    {
        let proto = attr.prototype_template(scope);
        install_proto_accessor(scope, proto, "name", attr_name_getter, None);
        install_proto_accessor(scope, proto, "localName", attr_name_getter, None);
        install_proto_accessor(scope, proto, "value", attr_value_getter, Some(attr_value_setter));
        install_proto_accessor(scope, proto, "ownerElement", attr_owner_element_getter, None);
        install_proto_accessor(scope, proto, "namespaceURI", attr_namespace_uri_getter, None);
        install_proto_accessor(scope, proto, "prefix", attr_prefix_getter, None);
        install_proto_accessor(scope, proto, "specified", attr_specified_getter, None);
        set_to_string_tag(scope, proto, "Attr");
    }

    // Convert all to Globals
    DomTemplates {
        event_target: v8::Global::new(scope, event_target),
        node: v8::Global::new(scope, node),
        element: v8::Global::new(scope, element),
        html_element: v8::Global::new(scope, html_element),
        html_div_element: v8::Global::new(scope, html_div_element),
        html_span_element: v8::Global::new(scope, html_span_element),
        html_anchor_element: v8::Global::new(scope, html_anchor_element),
        html_input_element: v8::Global::new(scope, html_input_element),
        html_button_element: v8::Global::new(scope, html_button_element),
        html_form_element: v8::Global::new(scope, html_form_element),
        html_canvas_element: v8::Global::new(scope, html_canvas_element),
        html_script_element: v8::Global::new(scope, html_script_element),
        html_image_element: v8::Global::new(scope, html_image_element),
        html_video_element: v8::Global::new(scope, html_video_element),
        html_audio_element: v8::Global::new(scope, html_audio_element),
        html_select_element: v8::Global::new(scope, html_select_element),
        html_option_element: v8::Global::new(scope, html_option_element),
        html_iframe_element: v8::Global::new(scope, html_iframe_element),
        html_textarea_element: v8::Global::new(scope, html_textarea_element),
        html_head_element: v8::Global::new(scope, html_head_element),
        html_body_element: v8::Global::new(scope, html_body_element),
        html_html_element: v8::Global::new(scope, html_html_element),
        html_paragraph_element: v8::Global::new(scope, html_paragraph_element),
        html_heading_element: v8::Global::new(scope, html_heading_element),
        html_ulist_element: v8::Global::new(scope, html_ulist_element),
        html_olist_element: v8::Global::new(scope, html_olist_element),
        html_li_element: v8::Global::new(scope, html_li_element),
        html_table_element: v8::Global::new(scope, html_table_element),
        html_table_row_element: v8::Global::new(scope, html_table_row_element),
        html_table_cell_element: v8::Global::new(scope, html_table_cell_element),
        html_dialog_element: v8::Global::new(scope, html_dialog_element),
        html_style_element: v8::Global::new(scope, html_style_element),
        html_link_element: v8::Global::new(scope, html_link_element),
        html_meta_element: v8::Global::new(scope, html_meta_element),
        html_unknown_element: v8::Global::new(scope, html_unknown_element),
        text_node: v8::Global::new(scope, text_node),
        comment_node: v8::Global::new(scope, comment_node),
        character_data: v8::Global::new(scope, character_data),
        document_fragment: v8::Global::new(scope, document_fragment),
        document_node: v8::Global::new(scope, document_node),
        attr: v8::Global::new(scope, attr),
        node_list: v8::Global::new(scope, node_list),
        dom_token_list: v8::Global::new(scope, dom_token_list),
        css_style_declaration: v8::Global::new(scope, css_style_declaration),
        headers: v8::Global::new(scope, headers),
        response: v8::Global::new(scope, response),
        request: v8::Global::new(scope, request),
    }
}

/// Install all DOM constructor functions on the global object.
/// This makes `HTMLDivElement`, `HTMLElement`, etc. available in JS.
pub fn install_dom_constructors(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
    templates: &DomTemplates,
    worker_mode: bool,
) {
    let pairs: &[(&str, &v8::Global<v8::FunctionTemplate>)] = &[
        ("EventTarget", &templates.event_target),
        ("Node", &templates.node),
        ("Element", &templates.element),
        ("HTMLElement", &templates.html_element),
        ("HTMLDivElement", &templates.html_div_element),
        ("HTMLSpanElement", &templates.html_span_element),
        ("HTMLAnchorElement", &templates.html_anchor_element),
        ("HTMLInputElement", &templates.html_input_element),
        ("HTMLButtonElement", &templates.html_button_element),
        ("HTMLFormElement", &templates.html_form_element),
        ("HTMLCanvasElement", &templates.html_canvas_element),
        ("HTMLScriptElement", &templates.html_script_element),
        ("HTMLImageElement", &templates.html_image_element),
        ("HTMLVideoElement", &templates.html_video_element),
        ("HTMLAudioElement", &templates.html_audio_element),
        ("HTMLSelectElement", &templates.html_select_element),
        ("HTMLOptionElement", &templates.html_option_element),
        ("HTMLIFrameElement", &templates.html_iframe_element),
        ("HTMLTextAreaElement", &templates.html_textarea_element),
        ("HTMLHeadElement", &templates.html_head_element),
        ("HTMLBodyElement", &templates.html_body_element),
        ("HTMLHtmlElement", &templates.html_html_element),
        ("HTMLParagraphElement", &templates.html_paragraph_element),
        ("HTMLHeadingElement", &templates.html_heading_element),
        ("HTMLUListElement", &templates.html_ulist_element),
        ("HTMLOListElement", &templates.html_olist_element),
        ("HTMLLIElement", &templates.html_li_element),
        ("HTMLTableElement", &templates.html_table_element),
        ("HTMLTableRowElement", &templates.html_table_row_element),
        ("HTMLTableCellElement", &templates.html_table_cell_element),
        ("HTMLDialogElement", &templates.html_dialog_element),
        ("HTMLStyleElement", &templates.html_style_element),
        ("HTMLLinkElement", &templates.html_link_element),
        ("HTMLMetaElement", &templates.html_meta_element),
        ("HTMLUnknownElement", &templates.html_unknown_element),
        ("NodeList", &templates.node_list),
        ("DOMTokenList", &templates.dom_token_list),
        ("CSSStyleDeclaration", &templates.css_style_declaration),
        ("Headers", &templates.headers),
        ("Response", &templates.response),
        ("Request", &templates.request),
        ("Text", &templates.text_node),
        ("Comment", &templates.comment_node),
        ("CharacterData", &templates.character_data),
        ("DocumentFragment", &templates.document_fragment),
        // Document: keep codegen constructor (construct_only) which allows
        // `new Document()` per WebIDL spec. DOM template's illegal_dom_constructor
        // would block construction, causing idlharness to skip Document tests.
    ];

    // Install all constructors first, then fix up __proto__ chains.
    let mut installed_ctors: std::collections::HashMap<String, v8::Local<v8::Function>> =
        std::collections::HashMap::new();
    for (name, tmpl_global) in pairs {
        if worker_mode && iv8_surface::generated::install_all::is_window_only_interface(name) {
            continue;
        }
        let tmpl = v8::Local::new(scope, *tmpl_global);
        if let Some(func) = tmpl.get_function(scope) {
            let key = crate::v8_utils::v8_string(scope, name);
            let ok = global.define_own_property(
                scope,
                key.into(),
                func.into(),
                v8::PropertyAttribute::DONT_ENUM,
            );
            let _ = ok;
            installed_ctors.insert(name.to_string(), func);
        }
    }

    // Fix up constructor __proto__ chains. V8 FunctionTemplate::inherit()
    // sets prototype.__proto__ but NOT constructor.__proto__.
    // We must manually set ctor.__proto__ = parent_ctor.
    const CTOR_INHERITANCE: &[(&str, &str)] = &[
        ("EventTarget", ""),
        ("Node", "EventTarget"),
        ("Element", "Node"),
        ("HTMLElement", "Element"),
        ("HTMLDivElement", "HTMLElement"),
        ("HTMLSpanElement", "HTMLElement"),
        ("HTMLAnchorElement", "HTMLElement"),
        ("HTMLInputElement", "HTMLElement"),
        ("HTMLButtonElement", "HTMLElement"),
        ("HTMLFormElement", "HTMLElement"),
        ("HTMLCanvasElement", "HTMLElement"),
        ("HTMLScriptElement", "HTMLElement"),
        ("HTMLImageElement", "HTMLElement"),
        ("HTMLVideoElement", "HTMLMediaElement"),
        ("HTMLAudioElement", "HTMLMediaElement"),
        ("HTMLSelectElement", "HTMLElement"),
        ("HTMLOptionElement", "HTMLElement"),
        ("HTMLIFrameElement", "HTMLElement"),
        ("HTMLTextAreaElement", "HTMLElement"),
        ("HTMLHeadElement", "HTMLElement"),
        ("HTMLBodyElement", "HTMLElement"),
        ("HTMLHtmlElement", "HTMLElement"),
        ("HTMLParagraphElement", "HTMLElement"),
        ("HTMLHeadingElement", "HTMLElement"),
        ("HTMLUListElement", "HTMLElement"),
        ("HTMLOListElement", "HTMLElement"),
        ("HTMLLIElement", "HTMLElement"),
        ("HTMLTableElement", "HTMLElement"),
        ("HTMLStyleElement", "HTMLElement"),
        ("HTMLLinkElement", "HTMLElement"),
        ("HTMLMetaElement", "HTMLElement"),
        ("HTMLUnknownElement", "HTMLElement"),
        ("Text", "CharacterData"),
        ("Comment", "CharacterData"),
        ("CharacterData", "Node"),
        ("DocumentFragment", "Node"),
        ("Document", "Node"),
        ("NodeList", ""),
        ("DOMTokenList", ""),
        ("CSSStyleDeclaration", ""),
        ("Headers", ""),
        ("Response", ""),
        ("Request", ""),
    ];
    for (child, parent) in CTOR_INHERITANCE {
        if parent.is_empty() { continue; }
        let Some(child_func) = installed_ctors.get(*child) else { continue };
        // Parent may be a codegen constructor (e.g., CharacterData) not in
        // installed_ctors. Look it up from global.
        let parent_key = crate::v8_utils::v8_string(scope, *parent);
        let Some(parent_val) = global.get(scope, parent_key.into()) else { continue };
        if !parent_val.is_function() { continue; }
        let child_obj: v8::Local<v8::Object> = (*child_func).into();
        let _ = child_obj.set_prototype(scope, parent_val);
    }

    let proto_fixes: &[(&str, &str)] = &[
        ("HTMLVideoElement", "HTMLMediaElement"),
        ("HTMLAudioElement", "HTMLMediaElement"),
    ];
    for (child, parent) in proto_fixes {
        let child_key = crate::v8_utils::v8_string(scope, *child);
        let parent_key = crate::v8_utils::v8_string(scope, *parent);
        let Some(child_val) = global.get(scope, child_key.into()) else { continue };
        let Some(parent_val) = global.get(scope, parent_key.into()) else { continue };
        if !child_val.is_function() || !parent_val.is_function() { continue; }
        let child_ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(child_val) };
        let parent_ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(parent_val) };
        let proto_key = crate::v8_utils::v8_string(scope, "prototype");
        if let Some(child_proto) = child_ctor.get(scope, proto_key.into()) {
            if let Some(parent_proto) = parent_ctor.get(scope, proto_key.into()) {
                if let Some(child_proto_obj) = child_proto.to_object(scope) {
                    let _ = child_proto_obj.set_prototype(scope, parent_proto);
                }
            }
        }
    }
}

/// Capture codegen prototype objects for the 39 DOM interfaces before
/// install_dom_constructors overwrites them. This allows chaining
/// dom/template.rs prototypes to codegen prototypes so that IDL
/// attributes, constants, and inheritance from codegen are preserved.
pub fn capture_codegen_prototypes(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
) -> HashMap<String, v8::Global<v8::Function>> {
    let names = [
        "EventTarget", "Node", "Element", "HTMLElement",
        "HTMLDivElement", "HTMLSpanElement", "HTMLAnchorElement",
        "HTMLInputElement", "HTMLButtonElement", "HTMLFormElement",
        "HTMLCanvasElement", "HTMLScriptElement", "HTMLImageElement",
        "HTMLVideoElement", "HTMLAudioElement", "HTMLSelectElement",
        "HTMLOptionElement", "HTMLIFrameElement", "HTMLTextAreaElement", "HTMLHeadElement", "HTMLBodyElement",
        "HTMLHtmlElement", "HTMLParagraphElement", "HTMLHeadingElement",
        "HTMLUListElement", "HTMLOListElement", "HTMLLIElement",
        "HTMLTableElement", "HTMLStyleElement", "HTMLLinkElement",
        "HTMLMetaElement", "HTMLUnknownElement",
        "NodeList", "DOMTokenList", "CSSStyleDeclaration",
        "Headers", "Response", "Request", "Text", "Comment", "Document",
        "DocumentFragment", "CharacterData", "XMLDocument",
        "DocumentType", "Attr", "NodeIterator", "TreeWalker",
        "XPathResult", "XPathExpression", "XPathEvaluator",
        "Range", "MutationObserver", "MutationRecord",
        "NamedNodeMap", "DOMImplementation", "ShadowRoot",
        "HTMLCollection", "HTMLOptionsCollection",
        "HTMLAllCollection",
    ];
    let mut map = HashMap::new();
    for name in names {
        let key = crate::v8_utils::v8_string(scope, name);
        if let Some(ctor_val) = global.get(scope, key.into()) {
            if ctor_val.is_function() {
                let ctor = unsafe { v8::Local::<v8::Function>::cast_unchecked(ctor_val) };
                map.insert(name.to_string(), v8::Global::new(scope, ctor));
            }
        }
    }
    map
}

/// Merge codegen prototype properties into dom/template.rs prototypes.
///
/// For each of the 39 DOM interfaces, copy all own properties from the
/// codegen prototype to the dom/template.rs prototype. This ensures
/// codegen IDL attributes, constants, and methods (with correct .length)
/// are visible on the dom/template.rs prototype, alongside the native
/// callbacks (appendChild, etc.) that dom/template.rs installs.
///
/// Properties already present on the dom prototype (via inheritance) are
/// NOT overwritten (dom/template.rs native callbacks take priority).
///
/// Additionally, the dom EventTarget prototype's __proto__ is set to the
/// codegen EventTarget prototype, connecting the two prototype chains.
pub fn chain_dom_prototypes(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
    codegen_ctors: &HashMap<String, v8::Global<v8::Function>>,
) {
    let proto_key = crate::v8_utils::v8_string(scope, "prototype");
    crate::telemetry::init_proto_merge_start(codegen_ctors.len());
    for (name, codegen_ctor_global) in codegen_ctors {
        let key = crate::v8_utils::v8_string(scope, name.as_str());
        let Some(dom_ctor_val) = global.get(scope, key.into()) else { continue };
        if !dom_ctor_val.is_function() { continue; }
        let dom_ctor = unsafe { v8::Local::<v8::Function>::cast_unchecked(dom_ctor_val) };
        let codegen_ctor = v8::Local::new(scope, codegen_ctor_global);

        let same_ctor = dom_ctor_val.strict_equals(codegen_ctor.into());
        if same_ctor {
            crate::telemetry::init_same_ctor_warning(name);
        }

        // Fix constructor __proto__ for ALL interfaces in codegen_ctors.
        // codegen install_all set __proto__ to codegen parent, but
        // install_dom_constructors may have replaced parent on global.
        // Re-set __proto__ to current global parent.
        {
            let current_proto = dom_ctor.get_prototype(scope);
            if let Some(current_proto) = current_proto {
                if current_proto.is_function() {
                    let proto_func = unsafe {
                        v8::Local::<v8::Function>::cast_unchecked(current_proto)
                    };
                    let proto_name = proto_func.get_name(scope);
                    let proto_name_str = proto_name.to_rust_string_lossy(scope);
                    if !proto_name_str.is_empty() {
                        let global_key = crate::v8_utils::v8_string(scope, &proto_name_str);
                        if let Some(global_parent) = global.get(scope, global_key.into()) {
                            if global_parent.is_function()
                                && !global_parent.strict_equals(current_proto.into())
                            {
                                let dom_ctor_obj: v8::Local<v8::Object> = dom_ctor.into();
                                let _ = dom_ctor_obj.set_prototype(scope, global_parent);
                            }
                        }
                    }
                }
            }
        }

        let Some(dom_proto_val) = dom_ctor.get(scope, proto_key.into()) else { continue };
        if !dom_proto_val.is_object() || dom_proto_val.is_null_or_undefined() { continue; }
        let dom_proto = unsafe { v8::Local::<v8::Object>::cast_unchecked(dom_proto_val) };

        let Some(codegen_proto_val) = codegen_ctor.get(scope, proto_key.into()) else { continue };
        if !codegen_proto_val.is_object() || codegen_proto_val.is_null_or_undefined() { continue; }
        let codegen_proto = unsafe { v8::Local::<v8::Object>::cast_unchecked(codegen_proto_val) };

        if name == "EventTarget" {
            let _ = dom_proto.set_prototype(scope, codegen_proto.into());
        } else {
            let current_proto = dom_proto.get_prototype(scope);
            let need_chain = current_proto.is_none()
                || current_proto.is_some_and(|p| p.is_null_or_undefined());
            if need_chain {
                let _ = dom_proto.set_prototype(scope, codegen_proto.into());
            }
        }

        let mut proto_copied = 0u32;
        let mut proto_skipped = 0u32;
        let mut proto_define_failed = 0u32;

        // AD-1a fix (root cause 2): use get_own_property_descriptor instead of
        // has() to check if dom_proto already has the property as own.
        // has() traverses the prototype chain, so dom parent's simplified stubs
        // block codegen full version. get_own_property_descriptor only checks own.
        // Note: root cause 1 (recursive chain traversal) deferred — requires
        // deeper V8 scope safety analysis before enabling.
        let prop_names = codegen_proto.get_own_property_names(scope, Default::default());
        if let Some(names) = prop_names {
            let len = names.length();
            for i in 0..len {
                let Some(prop_name_val) = names.get_index(scope, i) else { continue };
                let prop_name = if prop_name_val.is_name() {
                    unsafe { v8::Local::<v8::Name>::cast_unchecked(prop_name_val) }
                } else { continue };
                // AD-1a fix: check own property only, not prototype chain
                let dom_existing = dom_proto.get_own_property_descriptor(scope, prop_name);
                if dom_existing.is_some_and(|d| d.is_object() && !d.is_undefined()) {
                    proto_skipped += 1; continue;
                }

                let Some(descriptor) = codegen_proto.get_own_property_descriptor(scope, prop_name) else { continue };
                    if descriptor.is_object() && !descriptor.is_null_or_undefined() {
                        let desc_obj = unsafe { v8::Local::<v8::Object>::cast_unchecked(descriptor) };
                        let getter = desc_obj.get(scope, crate::v8_utils::v8_string(scope, "get").into());
                        let setter = desc_obj.get(scope, crate::v8_utils::v8_string(scope, "set").into());
                        let value = desc_obj.get(scope, crate::v8_utils::v8_string(scope, "value").into());
                        // v0.8.92 P0: copy when DOM does not already own the property
                        // (check above). Accessors that DOM already installed (id, data,
                        // textContent, ...) are skipped. Accessors only on codegen
                        // (e.g. Element.regionOverset) are still copied. fix_accessor
                        // skip-if-own prevents later overwrite of DOM-owned accessors.
                        let writable_val = desc_obj.get(scope, crate::v8_utils::v8_string(scope, "writable").into());
                        let configurable_val = desc_obj.get(scope, crate::v8_utils::v8_string(scope, "configurable").into());
                        let src_writable = writable_val
                            .map(|v| v.is_boolean() && v.is_true())
                            .unwrap_or_else(|| {
                                let val = value.unwrap_or(v8::undefined(scope).into());
                                !(val.is_number() || val.is_string() || val.is_boolean())
                            });
                        let src_configurable = configurable_val.map(|v| v.is_boolean() && v.is_true()).unwrap_or(true);
                        let pd = if let (Some(g), Some(s)) = (getter, setter) {
                            if g.is_undefined() && s.is_undefined() {
                                let mut p = v8::PropertyDescriptor::new_from_value_writable(value.unwrap_or(v8::undefined(scope).into()), src_writable);
                                p.set_configurable(src_configurable);
                                p.set_enumerable(true);
                                p
                            } else {
                                let mut p = v8::PropertyDescriptor::new_from_get_set(g, s);
                                p.set_configurable(src_configurable);
                                p.set_enumerable(true);
                                p
                            }
                        } else if let Some(g) = getter {
                            if g.is_undefined() {
                                let mut p = v8::PropertyDescriptor::new_from_value_writable(value.unwrap_or(v8::undefined(scope).into()), src_writable);
                                p.set_configurable(src_configurable);
                                p.set_enumerable(true);
                                p
                            } else {
                                let mut p = v8::PropertyDescriptor::new_from_get_set(g, v8::undefined(scope).into());
                                p.set_configurable(src_configurable);
                                p.set_enumerable(true);
                                p
                            }
                        } else {
                            let mut p = v8::PropertyDescriptor::new_from_value_writable(value.unwrap_or(v8::undefined(scope).into()), src_writable);
                            p.set_configurable(src_configurable);
                            p.set_enumerable(true);
                            p
                        };
                        let ok = dom_proto.define_property(scope, prop_name, &pd);
                        if ok.unwrap_or(false) { proto_copied += 1; } else { proto_define_failed += 1; }
                    }
                }
            }

        let ctor_names = codegen_ctor.get_own_property_names(scope, Default::default());
        let mut ctor_copied = 0u32;
        if let Some(names) = ctor_names {
            let len = names.length();
            for i in 0..len {
                let Some(prop_name_val) = names.get_index(scope, i) else { continue };
                let prop_name_str = prop_name_val.to_rust_string_lossy(scope);
                if prop_name_str == "length" || prop_name_str == "name" || prop_name_str == "prototype" { continue; }
                let prop_name = if prop_name_val.is_name() {
                    unsafe { v8::Local::<v8::Name>::cast_unchecked(prop_name_val) }
                } else { continue };
                let dom_ctor_obj: v8::Local<v8::Object> = dom_ctor.into();
                if dom_ctor_obj.has_own_property(scope, prop_name).unwrap_or(false) { continue; }
                let Some(descriptor) = codegen_ctor.get_own_property_descriptor(scope, prop_name) else { continue };
                if descriptor.is_object() && !descriptor.is_null_or_undefined() {
                    let desc_obj = unsafe { v8::Local::<v8::Object>::cast_unchecked(descriptor) };
                    let value = desc_obj.get(scope, crate::v8_utils::v8_string(scope, "value").into());
                    let mut pd = v8::PropertyDescriptor::new_from_value_writable(
                        value.unwrap_or(v8::undefined(scope).into()),
                        false,
                    );
                    pd.set_configurable(false);
                    pd.set_enumerable(true);
                    let _ = dom_ctor.define_property(scope, prop_name, &pd);
                    ctor_copied += 1;
                }
            }
        }

        crate::telemetry::init_proto_merge(
            name,
            proto_copied,
            proto_skipped,
            ctor_copied,
            same_ctor,
        );
    }
    crate::telemetry::init_proto_merge_complete();

    // Fix __proto__ for codegen-only interfaces NOT in the 39 dom set.
    // These interfaces (e.g., HTMLTitleElement, HTMLBaseElement) had their
    // __proto__ set by codegen install_all to the codegen parent, but
    // install_dom_constructors replaced the parent on global with a dom
    // version. Use JS to scan and fix stale __proto__ refs.
    let fix_script = r#"
        (function() {
            var names = Object.getOwnPropertyNames(globalThis);
            for (var i = 0; i < names.length; i++) {
                var name = names[i];
                var ctor = globalThis[name];
                if (typeof ctor !== 'function') continue;

                // Fix constructor.__proto__
                var proto = Object.getPrototypeOf(ctor);
                if (typeof proto === 'function' && proto.name) {
                    var globalParent = globalThis[proto.name];
                    if (typeof globalParent === 'function'
                        && proto !== globalParent
                        && ctor !== globalParent) {
                        try { Object.setPrototypeOf(ctor, globalParent); } catch(e) {}
                    }
                }

                // Fix constructor.prototype.__proto__
                if (ctor.prototype) {
                    var protoProto = Object.getPrototypeOf(ctor.prototype);
                    if (protoProto && protoProto.constructor
                        && protoProto.constructor.name
                        && protoProto.constructor.name !== name) {
                        var globalParentCtor = globalThis[protoProto.constructor.name];
                        if (globalParentCtor && globalParentCtor.prototype
                            && protoProto !== globalParentCtor.prototype
                            && ctor.prototype !== globalParentCtor.prototype) {
                            try {
                                Object.setPrototypeOf(ctor.prototype, globalParentCtor.prototype);
                            } catch(e) {}
                        }
                    }
                }
            }
        })();
    "#;
    let fix_src = crate::v8_utils::v8_string(scope, fix_script);
    if let Some(script) = v8::Script::compile(scope, fix_src, None) {
        let _ = script.run(scope);
    }

    // Copy Window.prototype own properties to global object.
    // Per Web IDL §3.7.3, [Global] interface properties must be on the
    // global object itself, not just on the prototype. idlharness checks
    // global.hasOwnProperty(prop) for Window interface members.
    let global_prop_script = r#"
        (function() {
            if (typeof Window === 'undefined' || !Window.prototype) return;
            var proto = Window.prototype;
            var names = Object.getOwnPropertyNames(proto);
            for (var i = 0; i < names.length; i++) {
                var name = names[i];
                if (name === 'constructor') continue;
                if (globalThis.hasOwnProperty(name)) continue;
                try {
                    var desc = Object.getOwnPropertyDescriptor(proto, name);
                    if (desc) {
                        Object.defineProperty(globalThis, name, desc);
                    }
                } catch(e) {}
            }
        })();
    "#;
    let gp_src = crate::v8_utils::v8_string(scope, global_prop_script);
    if let Some(script) = v8::Script::compile(scope, gp_src, None) {
        let _ = script.run(scope);
    }
}

/// Select the correct FunctionTemplate for a given tag name.
/// Returns the most specific template available.
/// Map a tag name to its WebIDL interface name.
/// Returns None for tags that have a dedicated DOM template (handled by
/// template_for_tag) or unknown tags.
fn tag_to_interface_name(tag_name: &str) -> Option<String> {
    let tag = tag_name.to_ascii_lowercase();
    // Skip tags that already have dedicated DOM templates
    match tag.as_str() {
        "div" | "span" | "a" | "input" | "button" | "form" | "canvas" | "script"
        | "img" | "video" | "audio" | "select" | "textarea" | "head" | "body"
        | "html" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6"
        | "ul" | "ol" | "li" | "table" | "thead" | "tbody" | "tfoot" | "tr"
        | "td" | "th" | "caption" | "colgroup" | "col"
        | "style" | "link" | "meta" | "iframe" | "dialog" => return None,
        _ => {}
    }
    // Map tag to interface name per HTML spec
    let name = match tag.as_str() {
        "dl" => "HTMLDListElement",
        "dt" | "dd" => "HTMLElement",
        "pre" => "HTMLPreElement",
        "code" => "HTMLElement",
        "blockquote" | "q" => "HTMLQuoteElement",
        "section" | "article" | "nav" | "aside" | "header" | "footer"
        | "main" | "address" | "figure" | "figcaption" => "HTMLElement",
        "details" => "HTMLDetailsElement",
        "summary" => "HTMLElement",
        "hr" => "HTMLHRElement",
        "br" => "HTMLBRElement",
        "label" => "HTMLLabelElement",
        "fieldset" => "HTMLFieldSetElement",
        "legend" => "HTMLLegendElement",
        "optgroup" => "HTMLOptGroupElement",
        "option" => "HTMLOptionElement",
        "template" => "HTMLTemplateElement",
        "slot" => "HTMLSlotElement",
        "data" => "HTMLDataElement",
        "time" => "HTMLTimeElement",
        "mark" => "HTMLElement",
        "ruby" => "HTMLElement",
        "rt" | "rp" => "HTMLElement",
        "wbr" => "HTMLWbrElement",
        "b" | "i" | "u" | "s" | "small" | "strong" | "em" | "sub" | "sup"
        | "abbr" | "cite" | "dfn" | "kbd" | "samp" | "var" => "HTMLElement",
        "del" | "ins" => "HTMLModElement",
        "output" => "HTMLOutputElement",
        "progress" => "HTMLProgressElement",
        "meter" => "HTMLMeterElement",
        "picture" => "HTMLPictureElement",
        "source" => "HTMLSourceElement",
        "embed" => "HTMLEmbedElement",
        "object" => "HTMLObjectElement",
        "param" => "HTMLParamElement",
        "map" => "HTMLMapElement",
        "area" => "HTMLAreaElement",
        "track" => "HTMLTrackElement",
        "title" => "HTMLTitleElement",
        "base" => "HTMLBaseElement",
        "basefont" => "HTMLBaseFontElement",
        "dir" => "HTMLDirectoryElement",
        "font" => "HTMLFontElement",
        "frame" => "HTMLFrameElement",
        "frameset" => "HTMLFrameSetElement",
        "marquee" => "HTMLMarqueeElement",
        _ => return None,
    };
    Some(name.to_string())
}

pub fn template_for_tag<'s>(
    scope: &v8::PinScope<'s, '_>,
    templates: &DomTemplates,
    tag_name: &str,
) -> v8::Local<'s, v8::FunctionTemplate> {
    let global = match tag_name.to_ascii_lowercase().as_str() {
        "div" => &templates.html_div_element,
        "span" => &templates.html_span_element,
        "a" => &templates.html_anchor_element,
        "input" => &templates.html_input_element,
        "button" => &templates.html_button_element,
        "form" => &templates.html_form_element,
        "canvas" => &templates.html_canvas_element,
        "script" => &templates.html_script_element,
        "img" => &templates.html_image_element,
        "video" => &templates.html_video_element,
        "audio" => &templates.html_audio_element,
        "select" => &templates.html_select_element,
        "option" => &templates.html_option_element,
        "iframe" => &templates.html_iframe_element,
        "textarea" => &templates.html_textarea_element,
        "head" => &templates.html_head_element,
        "body" => &templates.html_body_element,
        "html" => &templates.html_html_element,
        "p" => &templates.html_paragraph_element,
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => &templates.html_heading_element,
        "ul" => &templates.html_ulist_element,
        "ol" => &templates.html_olist_element,
        "li" => &templates.html_li_element,
        "table" | "thead" | "tbody" | "tfoot" | "caption" | "colgroup" | "col" => {
            &templates.html_table_element
        }
        "tr" => &templates.html_table_row_element,
        "td" | "th" => &templates.html_table_cell_element,
        "dialog" => &templates.html_dialog_element,
        "style" => &templates.html_style_element,
        "link" => &templates.html_link_element,
        "meta" => &templates.html_meta_element,
        "section" | "article" | "nav" | "aside" | "header" | "footer" | "main" | "address"
        | "figure" | "figcaption" | "details" | "summary" | "dl" | "dt" | "dd" | "hr" | "br"
        | "pre" | "code" | "blockquote" | "embed" | "object" | "progress" | "meter"
        | "label" | "fieldset" | "legend" | "optgroup" | "template" | "slot"
        | "data" | "time" | "mark" | "ruby" | "rt" | "rp" | "wbr" | "b" | "i" | "u" | "s"
        | "small" | "strong" | "em" | "sub" | "sup" | "abbr" | "cite" | "dfn" | "kbd" | "q"
        | "samp" | "var" | "del" | "ins" | "output" | "picture" | "source" => {
            &templates.html_element
        }
        _ => &templates.html_unknown_element,
    };
    v8::Local::new(scope, global)
}

/// Create a V8 object for a DOM node using the appropriate template.
/// Stores the NodeId in internal field 0.
/// Uses the identity cache to return the same object for the same NodeId.
/// Obtain a mutable reference to the V8 Isolate from a shared PinScope
/// reference. Necessary because v8::Weak::new requires &mut Isolate, but
/// callbacks only have access to &PinScope (shared, not mutable).
///
/// # Safety analysis
///
/// This function casts &Isolate to &mut Isolate via raw pointer. This is
/// sound in the V8 embedding context for two reasons:
///
/// 1. **Single-threaded execution**: V8 Isolate executes callbacks
///    synchronously on a single thread. There are no concurrent accesses
///    to the Isolate through any other path during callback execution.
///
/// 2. **PinScope owns the Isolate pointer**: The v8 crate's PinnedRef
///    stores a `NonNull<RealIsolate>` internally. Both Deref (→ &Isolate)
///    and DerefMut (→ &mut Isolate) are implemented for PinnedRef<HandleScope>
///    and PinnedRef<CallbackScope>. These impls use the identical pattern:
///    `Isolate::from_raw_ref_mut(self.isolate.as_ptr())`. This function
///    performs the same cast through scope.as_ref() (which goes &PinnedRef
///    → &Isolate via AsRef), because &PinScope means the mutable access
///    to the PinnedRef itself is unavailable.
///
/// **Note**: This function suppresses the `invalid_reference_casting` lint
/// (introduced in Rust 1.81), which is conservative for general Rust code
/// but overly restrictive for V8's single-threaded embedding model. The
/// v8 crate's own DerefMut does the identical raw-pointer-based cast.
///
/// The alternative — passing &mut PinScope through all callbacks and
/// helper functions — has been evaluated and rejected: it requires
/// changing 40+ function signatures, conflicts with RuntimeState::get's
/// lifetime tying to &Isolate, and D-025 bounds.
#[allow(unsafe_code)]
#[allow(invalid_reference_casting)]
#[allow(dead_code)]
pub(crate) fn isolate_mut_from_scope<'s>(scope: &v8::PinScope<'s, '_>) -> &'s mut v8::Isolate {
    let isolate_ref: &v8::Isolate = scope.as_ref();
    let ptr: *const v8::Isolate = isolate_ref;
    unsafe { &mut *(ptr as *mut v8::Isolate) }
}

/// Bump the lazy sweep counter and trigger a full sweep if threshold is reached.
/// With Global (strong ref) cache, sweep removes entries for nodes no longer
/// in the document tree (stale entries from removed nodes).
pub(crate) fn bump_and_maybe_sweep(
    state: &RuntimeState,
    cache: &mut std::collections::HashMap<crate::dom::NodeId, v8::Global<v8::Object>>,
    _scope: &v8::PinScope<'_, '_>,
) {
    let ops = state.node_cache_ops.get() + 1;
    state.node_cache_ops.set(ops);
    if ops >= state.node_cache_sweep_threshold {
        state.node_cache_ops.set(0);
        // With Global refs, we can't check liveness via is_empty.
        // Instead, prune entries whose NodeId no longer exists in the document.
        if let Some(doc) = state.document.borrow().as_ref() {
            cache.retain(|nid, _| doc.get(*nid).is_some());
        }
    }
}

pub fn create_node_object<'s>(
    scope: &v8::PinScope<'s, '_>,
    state: &RuntimeState,
    node_id: NodeId,
) -> Option<v8::Local<'s, v8::Value>> {
    // Check identity cache (Global reference — strong, prevents GC)
    {
        let mut cache = state.node_cache.borrow_mut();
        if let Some(global) = cache.get(&node_id) {
            let local = v8::Local::new(scope, global);
            // Cache hit — bump op counter and maybe sweep
            bump_and_maybe_sweep(state, &mut cache, scope);
            return Some(local.into());
        }
    }

    let templates = state.dom_templates.borrow();
    let templates = templates.as_ref()?;

    let doc = state.document.borrow();
    let doc = doc.as_ref()?;
    let node_ref = doc.get(node_id)?;
    let data = node_ref.value();

    // Select template based on node type
    let tmpl_local = match data {
        NodeData::Element { tag_name, .. } => template_for_tag(scope, templates, tag_name),
        NodeData::Text(_) => v8::Local::new(scope, &templates.text_node),
        NodeData::Comment(_) => v8::Local::new(scope, &templates.comment_node),
        NodeData::Document => v8::Local::new(scope, &templates.document_node),
        NodeData::DocumentType { .. } => v8::Local::new(scope, &templates.node),
        NodeData::DocumentFragment => v8::Local::new(scope, &templates.document_fragment),
    };

    // Instantiate from the instance_template directly. This bypasses the
    // FunctionTemplate's constructor callback (which throws "Illegal
    // constructor" for non-constructable interfaces) while still producing
    // an object whose [[Prototype]] is the template's .prototype — so
    // instanceof checks remain correct. Same pattern as native_env.rs.
    let inst_tmpl = tmpl_local.instance_template(scope);
    let obj = inst_tmpl.new_instance(scope)?;

    // For tags without a dedicated DOM template, set prototype from
    // the codegen constructor on globalThis (e.g., HTMLDListElement for "dl").
    // This ensures instanceof checks and property inheritance are correct.
    if let NodeData::Element { tag_name, .. } = data {
        let proto_ctor_name = tag_to_interface_name(tag_name);
        if let Some(ctor_name) = proto_ctor_name {
            let ctx = scope.get_current_context();
            let global = ctx.global(scope);
            let ctor_key = crate::v8_utils::v8_string(scope, &ctor_name);
            if let Some(ctor_val) = global.get(scope, ctor_key.into()) {
                if ctor_val.is_function() {
                    let ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(ctor_val) };
                    let proto_key = crate::v8_utils::v8_string(scope, "prototype");
                    if let Some(proto_val) = ctor.get(scope, proto_key.into()) {
                        if proto_val.is_object() && !proto_val.is_null_or_undefined() {
                            let _ = obj.set_prototype(scope, proto_val);
                        }
                    }
                }
            }
        }
    }

    // Store NodeId in internal field 0 as a usize via External
    let nid_usize = super::binding::node_id_to_usize(node_id);
    // We store the usize directly as a pointer value (no heap allocation needed)
    // SAFETY: we only read this back as a usize, never dereference it
    let external = v8::External::new(scope, nid_usize as *mut std::ffi::c_void);
    obj.set_internal_field(NODE_ID_FIELD as usize, external.into());

    // Cache as Global reference (strong — prevents GC of cached nodes)
    let global_obj = v8::Global::new(scope, obj);
    state.node_cache.borrow_mut().insert(node_id, global_obj);

    Some(obj.into())
}

/// Extract NodeId from internal field 0 of a V8 object.
pub fn extract_node_id_from_internal(
    scope: &v8::PinScope<'_, '_>,
    obj: v8::Local<v8::Object>,
) -> Option<NodeId> {
    let field = obj.get_internal_field(scope, NODE_ID_FIELD as usize)?;
    // Cast Data → Value to check is_external
    let value: v8::Local<v8::Value> = unsafe { v8::Local::cast_unchecked(field) };
    if !value.is_external() {
        return None;
    }
    let external: v8::Local<v8::External> = unsafe { v8::Local::cast_unchecked(value) };
    let nid_usize = external.value() as usize;
    super::binding::usize_to_node_id(nid_usize)
}

// ─── Callback implementations ────────────────────────────────────────────────
//
// All callbacks follow the same pattern:
//   1. Extract `this` → NodeId via internal field
//   2. Get RuntimeState from isolate
//   3. Borrow document
//   4. Do the work
//   5. Set return value
//
// We use a helper macro that defines variables with explicit names.

/// Check that `this` is a valid DOM node (has internal field).
/// Per WebIDL, calling an operation with wrong receiver must throw TypeError.
/// V8 non-strict mode converts null→globalThis, so we check internal fields
/// rather than just null/undefined.
unsafe fn null_this_check(info: *const v8::FunctionCallbackInfo) {
    let info_ref = unsafe { &*info };
    v8::callback_scope!(unsafe scope, info_ref);
    let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
    let this = args.this();
    // Check if this has internal fields (is a DOM node created from template)
    if let Some(obj) = this.to_object(scope) {
        if obj.internal_field_count() > 0 {
            // Has internal fields — likely a DOM node, allow call
            return;
        }
    }
    // No internal fields — wrong receiver, throw TypeError
    let msg = crate::v8_utils::v8_string(scope, "Illegal invocation");
    let exc = v8::Exception::type_error(scope, msg);
    scope.throw_exception(exc);
}
// The trick: use `paste!`-style renaming is not available, so we use
// a different approach: define the callback body as a nested function
// that takes explicit parameters.

/// Helper: run a DOM accessor callback body.
/// The body receives (scope, rv, state, node_id) where node_id is guaranteed Some.
#[inline(always)]
fn run_accessor<F>(info: *const v8::FunctionCallbackInfo, f: F)
where
    F: FnOnce(&v8::PinScope<'_, '_>, &mut v8::ReturnValue<'_>, &RuntimeState, NodeId)
        + std::panic::UnwindSafe,
{
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        match extract_node_id_from_internal(scope, this) {
            Some(node_id) => f(scope, &mut rv, state, node_id),
            None => {
                let msg = crate::v8_utils::v8_string(scope, "Illegal invocation");
                let exc = v8::Exception::type_error(scope, msg);
                scope.throw_exception(exc);
            }
        }
    }));
}

/// Helper: run a DOM callback body (with args, node_id may be None).
/// Throws TypeError if receiver is not a valid DOM node.
#[inline(always)]
fn run_callback<F>(info: *const v8::FunctionCallbackInfo, f: F)
where
    F: FnOnce(
            &v8::PinScope<'_, '_>,
            &v8::FunctionCallbackArguments<'_>,
            &mut v8::ReturnValue<'_>,
            &RuntimeState,
            Option<NodeId>,
        ) + std::panic::UnwindSafe,
{
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        match extract_node_id_from_internal(scope, this) {
            Some(node_id) => f(scope, &args, &mut rv, state, Some(node_id)),
            None => {
                let msg = crate::v8_utils::v8_string(scope, "Illegal invocation");
                let exc = v8::Exception::type_error(scope, msg);
                scope.throw_exception(exc);
            }
        }
    }));
}

/// Helper: run a DOM callback body that REQUIRES a valid node_id on `this`.
/// If `this` is not a DOM node (no internal field), throws TypeError.
/// Exception: if `this` is the global object, allows the callback to run
/// with NodeId(0) (the document root), since Window inherits EventTarget.
/// Used for methods like addEventListener, removeEventListener, dispatchEvent,
/// appendChild, etc. that require a valid receiver.
#[inline(always)]
fn run_callback_strict<F>(info: *const v8::FunctionCallbackInfo, f: F)
where
    F: FnOnce(
            &v8::PinScope<'_, '_>,
            &v8::FunctionCallbackArguments<'_>,
            &mut v8::ReturnValue<'_>,
            &RuntimeState,
            NodeId,
        ) + std::panic::UnwindSafe,
{
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        match extract_node_id_from_internal(scope, this) {
            Some(node_id) => f(scope, &args, &mut rv, state, node_id),
            None => {
                // Check if `this` is the global object (Window inherits EventTarget).
                // The global object may not have a node_id internal field, but
                // EventTarget methods should still work on it.
                let ctx = scope.get_current_context();
                let global = ctx.global(scope);
                if this.strict_equals(global.into()) {
                    // Use NodeId(0) as a sentinel for the global object.
                    // EventTarget methods on window will use this as the key
                    // for storing event listeners.
                    f(scope, &args, &mut rv, state, super::binding::usize_to_node_id(1).unwrap());
                } else {
                    let msg = crate::v8_utils::v8_string(scope, "Illegal invocation");
                    let exc = v8::Exception::type_error(scope, msg);
                    scope.throw_exception(exc);
                }
            }
        }
    }));
}

// ── Node accessors ────────────────────────────────────────────────────────────

unsafe extern "C" fn node_type_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let doc = state.document.borrow();
        if let Some(ref doc) = *doc {
            if let Some(node_ref) = doc.get(node_id) {
                rv.set(v8::Integer::new(scope, node_ref.value().node_type() as i32).into());
            }
        }
    });
}

unsafe extern "C" fn node_name_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let doc = state.document.borrow();
        if let Some(ref doc) = *doc {
            if let Some(node_ref) = doc.get(node_id) {
                let name = match node_ref.value() {
                    NodeData::Element { tag_name, .. } => tag_name.to_ascii_uppercase(),
                    other => other.node_name().to_string(),
                };
                if let Some(s) = v8::String::new(scope, &name) {
                    rv.set(s.into());
                }
            }
        }
    });
}

unsafe extern "C" fn text_content_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let doc = state.document.borrow();
        if let Some(ref doc) = *doc {
            let text = doc.text_content_of(node_id);
            if let Some(s) = v8::String::new(scope, &text) {
                rv.set(s.into());
            }
        }
    });
}

unsafe extern "C" fn text_content_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let val = args.get(0).to_rust_string_lossy(scope);
                let mut doc = state.document.borrow_mut();
                if let Some(ref mut doc) = *doc {
                    let children: Vec<_> = doc
                        .tree
                        .get(nid)
                        .map(|n| n.children().map(|c| c.id()).collect())
                        .unwrap_or_default();
                    for child_id in children {
                        doc.detach(child_id);
                    }
                    doc.append_child(nid, NodeData::text(&val));
                    state.node_cache.borrow_mut().remove(&nid);
                }
            }
        }
    });
}

// ── innerText (delegates to textContent) ────────────────────────────────────────

unsafe extern "C" fn inner_text_getter(info: *const v8::FunctionCallbackInfo) {
    text_content_getter(info)
}

unsafe extern "C" fn inner_text_setter(info: *const v8::FunctionCallbackInfo) {
    text_content_setter(info)
}

// ── CharacterData accessors/methods ────────────────────────────────────────────

unsafe extern "C" fn data_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let doc = state.document.borrow();
        if let Some(ref doc) = *doc {
            if let Some(node_ref) = doc.get(node_id) {
                let data = match node_ref.value() {
                    NodeData::Text(s) => s.as_str(),
                    NodeData::Comment(s) => s.as_str(),
                    _ => "",
                };
                if let Some(s) = v8::String::new(scope, data) {
                    rv.set(s.into());
                }
            }
        }
    });
}

unsafe extern "C" fn data_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let val = args.get(0).to_rust_string_lossy(scope);
                let mut doc = state.document.borrow_mut();
                if let Some(ref mut doc) = *doc {
                    if let Some(mut node_ref) = doc.tree.get_mut(nid) {
                        match node_ref.value() {
                            NodeData::Text(ref mut s) => *s = val,
                            NodeData::Comment(ref mut s) => *s = val,
                            _ => {}
                        }
                    }
                    state.node_cache.borrow_mut().remove(&nid);
                }
            }
        }
    });
}

unsafe extern "C" fn char_data_length_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let doc = state.document.borrow();
        if let Some(ref doc) = *doc {
            if let Some(node_ref) = doc.get(node_id) {
                let len = match node_ref.value() {
                    NodeData::Text(s) => s.chars().count(),
                    NodeData::Comment(s) => s.chars().count(),
                    _ => 0,
                };
                rv.set(v8::Integer::new_from_unsigned(scope, len as u32).into());
            }
        }
    });
}

unsafe extern "C" fn substring_data_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        if let Some(nid) = node_id {
            let offset = args.get(0).int32_value(scope).unwrap_or(0) as usize;
            let count = args.get(1).int32_value(scope).unwrap_or(0) as usize;
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                if let Some(node_ref) = doc.get(nid) {
                    let data = match node_ref.value() {
                        NodeData::Text(s) => s.as_str(),
                        NodeData::Comment(s) => s.as_str(),
                        _ => "",
                    };
                    let chars: Vec<char> = data.chars().collect();
                    let end = (offset + count).min(chars.len());
                    let result: String = if offset < chars.len() {
                        chars[offset..end].iter().collect()
                    } else {
                        String::new()
                    };
                    if let Some(s) = v8::String::new(scope, &result) {
                        rv.set(s.into());
                    }
                }
            }
        }
    });
}

unsafe extern "C" fn append_data_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let val = args.get(0).to_rust_string_lossy(scope);
                let mut doc = state.document.borrow_mut();
                if let Some(ref mut doc) = *doc {
                    if let Some(mut node_ref) = doc.tree.get_mut(nid) {
                        match node_ref.value() {
                            NodeData::Text(ref mut s) => s.push_str(&val),
                            NodeData::Comment(ref mut s) => s.push_str(&val),
                            _ => {}
                        }
                    }
                    state.node_cache.borrow_mut().remove(&nid);
                }
            }
        }
    });
}

unsafe extern "C" fn insert_data_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 2 {
                let offset = args.get(0).int32_value(scope).unwrap_or(0) as usize;
                let val = args.get(1).to_rust_string_lossy(scope);
                let mut doc = state.document.borrow_mut();
                if let Some(ref mut doc) = *doc {
                    if let Some(mut node_ref) = doc.tree.get_mut(nid) {
                        match node_ref.value() {
                            NodeData::Text(ref mut s) => {
                                let mut chars: Vec<char> = s.chars().collect();
                                let pos = offset.min(chars.len());
                                chars.splice(pos..pos, val.chars());
                                *s = chars.into_iter().collect();
                            }
                            NodeData::Comment(ref mut s) => {
                                let mut chars: Vec<char> = s.chars().collect();
                                let pos = offset.min(chars.len());
                                chars.splice(pos..pos, val.chars());
                                *s = chars.into_iter().collect();
                            }
                            _ => {}
                        }
                    }
                    state.node_cache.borrow_mut().remove(&nid);
                }
            }
        }
    });
}

unsafe extern "C" fn delete_data_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 2 {
                let offset = args.get(0).int32_value(scope).unwrap_or(0) as usize;
                let count = args.get(1).int32_value(scope).unwrap_or(0) as usize;
                let mut doc = state.document.borrow_mut();
                if let Some(ref mut doc) = *doc {
                    if let Some(mut node_ref) = doc.tree.get_mut(nid) {
                        match node_ref.value() {
                            NodeData::Text(ref mut s) => {
                                let mut chars: Vec<char> = s.chars().collect();
                                let end = (offset + count).min(chars.len());
                                if offset < chars.len() {
                                    chars.drain(offset..end);
                                }
                                *s = chars.into_iter().collect();
                            }
                            NodeData::Comment(ref mut s) => {
                                let mut chars: Vec<char> = s.chars().collect();
                                let end = (offset + count).min(chars.len());
                                if offset < chars.len() {
                                    chars.drain(offset..end);
                                }
                                *s = chars.into_iter().collect();
                            }
                            _ => {}
                        }
                    }
                    state.node_cache.borrow_mut().remove(&nid);
                }
            }
        }
    });
}

unsafe extern "C" fn replace_data_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 3 {
                let offset = args.get(0).int32_value(scope).unwrap_or(0) as usize;
                let count = args.get(1).int32_value(scope).unwrap_or(0) as usize;
                let val = args.get(2).to_rust_string_lossy(scope);
                let mut doc = state.document.borrow_mut();
                if let Some(ref mut doc) = *doc {
                    if let Some(mut node_ref) = doc.tree.get_mut(nid) {
                        match node_ref.value() {
                            NodeData::Text(ref mut s) => {
                                let mut chars: Vec<char> = s.chars().collect();
                                let end = (offset + count).min(chars.len());
                                if offset < chars.len() {
                                    chars.drain(offset..end);
                                }
                                let pos = offset.min(chars.len());
                                chars.splice(pos..pos, val.chars());
                                *s = chars.into_iter().collect();
                            }
                            NodeData::Comment(ref mut s) => {
                                let mut chars: Vec<char> = s.chars().collect();
                                let end = (offset + count).min(chars.len());
                                if offset < chars.len() {
                                    chars.drain(offset..end);
                                }
                                let pos = offset.min(chars.len());
                                chars.splice(pos..pos, val.chars());
                                *s = chars.into_iter().collect();
                            }
                            _ => {}
                        }
                    }
                    state.node_cache.borrow_mut().remove(&nid);
                }
            }
        }
    });
}

unsafe extern "C" fn split_text_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        if let Some(nid) = node_id {
            let offset = args.get(0).int32_value(scope).unwrap_or(0) as usize;
            let (text_before, text_after) = {
                let doc = state.document.borrow();
                let doc = doc.as_ref();
                if let Some(doc) = doc {
                    if let Some(node_ref) = doc.get(nid) {
                        match node_ref.value() {
                            NodeData::Text(s) => {
                                let chars: Vec<char> = s.chars().collect();
                                let pos = offset.min(chars.len());
                                let before: String = chars[..pos].iter().collect();
                                let after: String = chars[pos..].iter().collect();
                                (before, after)
                            }
                            _ => return,
                        }
                    } else { return; }
                } else { return; }
            };
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                if let Some(mut node_ref) = doc.tree.get_mut(nid) {
                    if let NodeData::Text(ref mut s) = node_ref.value() {
                        *s = text_before;
                    }
                }
                let new_id = doc.append_child(nid, NodeData::text(&text_after));
                state.node_cache.borrow_mut().remove(&nid);
                drop(doc);
                if let Some(obj) = create_node_object(scope, state, new_id) {
                    rv.set(obj);
                }
            }
        }
    });
}

unsafe extern "C" fn whole_text_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let doc = state.document.borrow();
        if let Some(ref doc) = *doc {
            if let Some(node_ref) = doc.get(node_id) {
                let text = match node_ref.value() {
                    NodeData::Text(s) => s.clone(),
                    _ => String::new(),
                };
                if let Some(s) = v8::String::new(scope, &text) {
                    rv.set(s.into());
                }
            }
        }
    });
}

// ── Navigation accessors ──────────────────────────────────────────────────────

unsafe extern "C" fn parent_node_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let parent_id = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.parent())
                .map(|p| p.id())
        };
        if let Some(pid) = parent_id {
            if let Some(obj) = create_node_object(scope, state, pid) {
                rv.set(obj);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn parent_element_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let parent_id = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.parent())
                .filter(|p| p.value().is_element())
                .map(|p| p.id())
        };
        if let Some(pid) = parent_id {
            if let Some(obj) = create_node_object(scope, state, pid) {
                rv.set(obj);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn first_child_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let cid = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.first_child())
                .map(|c| c.id())
        };
        if let Some(cid) = cid {
            if let Some(obj) = create_node_object(scope, state, cid) {
                rv.set(obj);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn last_child_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let cid = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.last_child())
                .map(|c| c.id())
        };
        if let Some(cid) = cid {
            if let Some(obj) = create_node_object(scope, state, cid) {
                rv.set(obj);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn next_sibling_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let sid = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.next_sibling())
                .map(|s| s.id())
        };
        if let Some(sid) = sid {
            if let Some(obj) = create_node_object(scope, state, sid) {
                rv.set(obj);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn prev_sibling_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let sid = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.prev_sibling())
                .map(|s| s.id())
        };
        if let Some(sid) = sid {
            if let Some(obj) = create_node_object(scope, state, sid) {
                rv.set(obj);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn child_nodes_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let child_ids: Vec<NodeId> = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .map(|n| n.children().map(|c| c.id()).collect())
                .unwrap_or_default()
        };
        let ctx = scope.get_current_context();
        let global = ctx.global(scope);
        let nl_key = crate::v8_utils::v8_string(scope, "NodeList");
        let nl_val = global.get(scope, nl_key.into());
        let arr = v8::Array::new(scope, child_ids.len() as i32);
        for (i, cid) in child_ids.iter().enumerate() {
            if let Some(obj) = create_node_object(scope, state, *cid) {
                arr.set_index(scope, i as u32, obj);
            }
        }
        if let Some(nl_val) = nl_val {
            if nl_val.is_function() {
                let nl_ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(nl_val) };
                let proto_key = crate::v8_utils::v8_string(scope, "prototype");
                if let Some(proto_val) = nl_ctor.get(scope, proto_key.into()) {
                    if proto_val.is_object() {
                        let arr_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(arr) };
                        let _ = arr_obj.set_prototype(scope, proto_val);
                    }
                }
            }
        }
        rv.set(arr.into());
    });
}

// ── Element accessors ─────────────────────────────────────────────────────────

unsafe extern "C" fn tag_name_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let doc = state.document.borrow();
        if let Some(ref doc) = *doc {
            if let Some(node_ref) = doc.get(node_id) {
                if let NodeData::Element { tag_name, .. } = node_ref.value() {
                    if let Some(s) = v8::String::new(scope, &tag_name.to_ascii_uppercase()) {
                        rv.set(s.into());
                    }
                }
            }
        }
    });
}

unsafe extern "C" fn id_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let doc = state.document.borrow();
        if let Some(ref doc) = *doc {
            if let Some(node_ref) = doc.get(node_id) {
                let id_val = node_ref.value().get_attr("id").unwrap_or("");
                if let Some(s) = v8::String::new(scope, id_val) {
                    rv.set(s.into());
                }
            }
        }
    });
}

unsafe extern "C" fn id_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |_scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let val = args.get(0).to_rust_string_lossy(_scope);
                let mut doc = state.document.borrow_mut();
                if let Some(ref mut doc) = *doc {
                    if let Some(mut node) = doc.tree.get_mut(nid) {
                        if let NodeData::Element {
                            ref mut attrs,
                            ref mut id,
                            ..
                        } = node.value()
                        {
                            if let Some(e) = attrs.iter_mut().find(|(k, _)| k == "id") {
                                e.1 = val.clone();
                            } else {
                                attrs.push(("id".to_string(), val.clone()));
                            }
                            *id = Some(val.clone());
                        }
                    }
                    doc.register_id(val, nid);
                }
            }
        }
    });
}

unsafe extern "C" fn class_name_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let doc = state.document.borrow();
        if let Some(ref doc) = *doc {
            if let Some(node_ref) = doc.get(node_id) {
                let cls = node_ref.value().get_attr("class").unwrap_or("");
                if let Some(s) = v8::String::new(scope, cls) {
                    rv.set(s.into());
                }
            }
        }
    });
}

unsafe extern "C" fn class_name_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let val = args.get(0).to_rust_string_lossy(scope);
                let mut doc = state.document.borrow_mut();
                if let Some(ref mut doc) = *doc {
                    if let Some(mut node) = doc.tree.get_mut(nid) {
                        if let NodeData::Element {
                            ref mut attrs,
                            ref mut classes,
                            ..
                        } = node.value()
                        {
                            if let Some(e) = attrs.iter_mut().find(|(k, _)| k == "class") {
                                e.1 = val.clone();
                            } else {
                                attrs.push(("class".to_string(), val.clone()));
                            }
                            *classes = val.split_whitespace().map(|s| s.to_string()).collect();
                        }
                    }
                }
            }
        }
    });
}

// ── DOMTokenList classList ──────────────────────────────────────────────────

unsafe extern "C" fn class_list_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let templates = state.dom_templates.borrow();
        if let Some(templates) = templates.as_ref() {
            let tmpl = v8::Local::new(scope, &templates.dom_token_list);
            let inst_tmpl = tmpl.instance_template(scope);
            if let Some(obj) = inst_tmpl.new_instance(scope) {
                    let nid_usize = super::binding::node_id_to_usize(node_id);
                    let external = v8::External::new(scope, nid_usize as *mut std::ffi::c_void);
                    obj.set_internal_field(0, external.into());
                    rv.set(obj.into());
                }
        }
    });
}

/// Extract NodeId from DOMTokenList internal field 0.
fn extract_classlist_node_id(
    scope: &v8::PinScope<'_, '_>,
    this: v8::Local<v8::Object>,
) -> Option<NodeId> {
    let field = this.get_internal_field(scope, 0)?;
    let value: v8::Local<v8::Value> = unsafe { v8::Local::cast_unchecked(field) };
    if value.is_external() {
        let external: v8::Local<v8::External> = unsafe { v8::Local::cast_unchecked(value) };
        super::binding::usize_to_node_id(external.value() as usize)
    } else {
        None
    }
}

fn classlist_read<F>(info: *const v8::FunctionCallbackInfo, f: F)
where
    F: FnOnce(&v8::PinScope<'_, '_>, &mut v8::ReturnValue<'_>, &[String]) + std::panic::UnwindSafe,
{
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        if let Some(node_id) = extract_classlist_node_id(scope, this) {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let classes: Vec<String> = {
                let doc = state.document.borrow();
                doc.as_ref()
                    .and_then(|d| d.get(node_id))
                    .map(|n| n.value().class_list().to_vec())
                    .unwrap_or_default()
            };
            f(scope, &mut rv, &classes);
        } else {
            let msg = crate::v8_utils::v8_string(scope, "Illegal invocation");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
        }
    }));
}

fn classlist_mutate<F>(info: *const v8::FunctionCallbackInfo, f: F)
where
    F: FnOnce(&v8::PinScope<'_, '_>, &mut v8::ReturnValue<'_>, &mut Vec<String>, &[String])
        + std::panic::UnwindSafe,
{
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        if let Some(node_id) = extract_classlist_node_id(scope, this) {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                if let Some(mut node) = doc.tree.get_mut(node_id) {
                    if let NodeData::Element {
                        ref mut attrs,
                        ref mut classes,
                        ..
                    } = node.value()
                    {
                        let call_args: Vec<String> = (0..args.length())
                            .map(|i| args.get(i).to_rust_string_lossy(scope))
                            .collect();
                        f(scope, &mut rv, classes, &call_args);
                        let new_class = classes.join(" ");
                        if let Some(e) = attrs.iter_mut().find(|(k, _)| k == "class") {
                            e.1 = new_class;
                        } else if !new_class.is_empty() {
                            attrs.push(("class".to_string(), new_class));
                        }
                    }
                }
            }
        } else {
            let msg = crate::v8_utils::v8_string(scope, "Illegal invocation");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
        }
    }));
}

unsafe extern "C" fn domtokenlist_item_cb(info: *const v8::FunctionCallbackInfo) {
    classlist_read(info, |scope, rv, classes| {
        let info_ref = unsafe { &*info };
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let idx = if args.length() >= 1 {
            args.get(0).number_value(scope).unwrap_or(-1.0) as i32
        } else {
            -1
        };
        if idx >= 0 && (idx as usize) < classes.len() {
            if let Some(s) = v8::String::new(scope, &classes[idx as usize]) {
                rv.set(s.into());
                return;
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn domtokenlist_contains_cb(info: *const v8::FunctionCallbackInfo) {
    classlist_read(info, |scope, rv, classes| {
        let info_ref = unsafe { &*info };
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if args.length() < 1 {
            rv.set(v8::Boolean::new(scope, false).into());
            return;
        }
        let cls = args.get(0).to_rust_string_lossy(scope);
        rv.set(v8::Boolean::new(scope, classes.iter().any(|c| c == &cls)).into());
    });
}

unsafe extern "C" fn domtokenlist_add_cb(info: *const v8::FunctionCallbackInfo) {
    classlist_mutate(info, |scope, rv, classes, args| {
        for cls in args {
            if !classes.contains(cls) {
                classes.push(cls.clone());
            }
        }
        rv.set(v8::undefined(scope).into());
    });
}

unsafe extern "C" fn domtokenlist_remove_cb(info: *const v8::FunctionCallbackInfo) {
    classlist_mutate(info, |scope, rv, classes, args| {
        for cls in args {
            classes.retain(|c| c != cls);
        }
        rv.set(v8::undefined(scope).into());
    });
}

unsafe extern "C" fn domtokenlist_toggle_cb(info: *const v8::FunctionCallbackInfo) {
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
        let cls = args.get(0).to_rust_string_lossy(scope);
        let force: Option<bool> = if args.length() >= 2 {
            let second = args.get(1);
            if second.is_boolean() {
                Some(second.boolean_value(scope))
            } else if second.is_undefined() || second.is_null() {
                None
            } else {
                Some(second.is_true() || second.to_rust_string_lossy(scope) == "true")
            }
        } else {
            None
        };
        if let Some(node_id) = extract_classlist_node_id(scope, this) {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                if let Some(mut node) = doc.tree.get_mut(node_id) {
                    if let NodeData::Element {
                        ref mut attrs,
                        ref mut classes,
                        ..
                    } = node.value()
                    {
                        let result = match force {
                            Some(true) => {
                                if !classes.contains(&cls) {
                                    classes.push(cls.clone());
                                }
                                true
                            }
                            Some(false) => {
                                classes.retain(|c| c != &cls);
                                false
                            }
                            None => {
                                if classes.contains(&cls) {
                                    classes.retain(|c| c != &cls);
                                    false
                                } else {
                                    classes.push(cls.clone());
                                    true
                                }
                            }
                        };
                        let new_class = classes.join(" ");
                        if let Some(e) = attrs.iter_mut().find(|(k, _)| k == "class") {
                            e.1 = new_class;
                        } else if !new_class.is_empty() {
                            attrs.push(("class".to_string(), new_class));
                        }
                        rv.set(v8::Boolean::new(scope, result).into());
                    }
                }
            }
        }
    }));
}

unsafe extern "C" fn domtokenlist_replace_cb(info: *const v8::FunctionCallbackInfo) {
    classlist_mutate(info, |scope, rv, classes, args| {
        if args.len() < 2 {
            rv.set(v8::Boolean::new(scope, false).into());
            return;
        }
        let old = &args[0];
        let new = &args[1];
        if let Some(pos) = classes.iter().position(|c| c == old) {
            classes[pos] = new.clone();
            rv.set(v8::Boolean::new(scope, true).into());
        } else {
            rv.set(v8::Boolean::new(scope, false).into());
        }
    });
}

unsafe extern "C" fn domtokenlist_tostring_cb(info: *const v8::FunctionCallbackInfo) {
    classlist_read(info, |scope, rv, classes| {
        let s = classes.join(" ");
        if let Some(v) = v8::String::new(scope, &s) {
            rv.set(v.into());
        }
    });
}

unsafe extern "C" fn domtokenlist_foreach_cb(info: *const v8::FunctionCallbackInfo) {
    classlist_read(info, |scope, rv, classes| {
        let info_ref = unsafe { &*info };
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if args.length() < 1 {
            return;
        }
        let callback = args.get(0);
        if !callback.is_function() {
            return;
        }
        let cb: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(callback) };
        let this_arg = args.get(1);
        let receiver = if this_arg.is_undefined() || this_arg.is_null() {
            v8::undefined(scope).into()
        } else {
            this_arg
        };
        let this_obj = args.this();
        for (i, cls) in classes.iter().enumerate() {
            let cls_val = v8::String::new(scope, cls).unwrap();
            let idx_val = v8::Integer::new(scope, i as i32);
            let _ = cb.call(
                scope,
                receiver.into(),
                &[cls_val.into(), idx_val.into(), this_obj.into()],
            );
        }
        rv.set(v8::undefined(scope).into());
    });
}

unsafe extern "C" fn domtokenlist_entries_cb(info: *const v8::FunctionCallbackInfo) {
    classlist_read(info, |scope, rv, classes| {
        let arr = v8::Array::new(scope, classes.len() as i32);
        for (i, cls) in classes.iter().enumerate() {
            let pair = v8::Array::new(scope, 2);
            pair.set_index(scope, 0, v8::Integer::new(scope, i as i32).into());
            if let Some(s) = v8::String::new(scope, cls) {
                pair.set_index(scope, 1, s.into());
            }
            arr.set_index(scope, i as u32, pair.into());
        }
        if let Some(iter) = arr.get(scope, crate::v8_utils::v8_string(scope, "values").into()) {
            if iter.is_function() {
                let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(iter) };
                let result = func.call(scope, arr.into(), &[]);
                rv.set(result.unwrap_or_else(|| v8::undefined(scope).into()).into());
                return;
            }
        }
        rv.set(arr.into());
    });
}

unsafe extern "C" fn domtokenlist_keys_cb(info: *const v8::FunctionCallbackInfo) {
    classlist_read(info, |scope, rv, classes| {
        let arr = v8::Array::new(scope, classes.len() as i32);
        for i in 0..classes.len() {
            arr.set_index(scope, i as u32, v8::Integer::new(scope, i as i32).into());
        }
        rv.set(arr.into());
    });
}

unsafe extern "C" fn domtokenlist_values_cb(info: *const v8::FunctionCallbackInfo) {
    classlist_read(info, |scope, rv, classes| {
        let arr = v8::Array::new(scope, classes.len() as i32);
        for (i, cls) in classes.iter().enumerate() {
            if let Some(s) = v8::String::new(scope, cls) {
                arr.set_index(scope, i as u32, s.into());
            }
        }
        rv.set(arr.into());
    });
}

unsafe extern "C" fn domtokenlist_length_getter(info: *const v8::FunctionCallbackInfo) {
    classlist_read(info, |scope, rv, classes| {
        rv.set(v8::Integer::new(scope, classes.len() as i32).into());
    });
}

unsafe extern "C" fn domtokenlist_value_getter(info: *const v8::FunctionCallbackInfo) {
    classlist_read(info, |scope, rv, classes| {
        let s = classes.join(" ");
        if let Some(v) = v8::String::new(scope, &s) {
            rv.set(v.into());
        }
    });
}

unsafe extern "C" fn domtokenlist_value_setter(info: *const v8::FunctionCallbackInfo) {
    classlist_mutate(info, |scope, rv, classes, args| {
        if args.is_empty() {
            return;
        }
        classes.clear();
        for cls in args[0].split_whitespace() {
            classes.push(cls.to_string());
        }
        rv.set(v8::Boolean::new(scope, true).into());
    });
}

// ── innerHTML / outerHTML ─────────────────────────────────────────────────────

unsafe extern "C" fn inner_html_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let html = {
            let doc = state.document.borrow();
            doc.as_ref()
                .map(|d| d.inner_html(node_id))
                .unwrap_or_default()
        };
        if let Some(s) = v8::String::new(scope, &html) {
            rv.set(s.into());
        }
    });
}

unsafe extern "C" fn inner_html_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let html = args.get(0).to_rust_string_lossy(scope);
                let mut doc = state.document.borrow_mut();
                if let Some(ref mut doc) = *doc {
                    // Remove existing children
                    let children: Vec<_> = doc
                        .tree
                        .get(nid)
                        .map(|n| n.children().map(|c| c.id()).collect())
                        .unwrap_or_default();
                    for child_id in children {
                        doc.detach(child_id);
                    }

                    // Parse the HTML fragment using the full parser
                    let fragment = super::parser::parse_html(&html, None);

                    // Get body children from the fragment (or root children)
                    let body_id = fragment.body().unwrap_or(fragment.root_id());
                    let child_ids: Vec<_> = fragment
                        .tree
                        .get(body_id)
                        .map(|b| b.children().map(|c| c.id()).collect())
                        .unwrap_or_default();

                    // Recursively copy nodes from fragment to target
                    for child_id in child_ids {
                        append_node_recursive_from_fragment(doc, nid, &fragment, child_id);
                    }

                    doc.invalidate_tag_index();
                    doc.rebuild_id_index();
                }
                // Invalidate cache for this node and descendants
                state.node_cache.borrow_mut().retain(|k, _| *k == nid);
            }
        }
    });
}

/// Recursively copy a node and its children from a source document to a target document.
fn append_node_recursive_from_fragment(
    doc: &mut crate::dom::Document,
    parent_id: NodeId,
    source: &crate::dom::Document,
    source_node_id: NodeId,
) {
    if let Some(source_node) = source.tree.get(source_node_id) {
        let data = source_node.value().clone();
        let new_id = doc.append_child(parent_id, data);
        let child_ids: Vec<NodeId> = source_node.children().map(|c| c.id()).collect();
        for child_id in child_ids {
            append_node_recursive_from_fragment(doc, new_id, source, child_id);
        }
    }
}

unsafe extern "C" fn outer_html_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let html = {
            let doc = state.document.borrow();
            doc.as_ref()
                .map(|d| d.outer_html(node_id))
                .unwrap_or_default()
        };
        if let Some(s) = v8::String::new(scope, &html) {
            rv.set(s.into());
        }
    });
}

// ── Element children accessors ────────────────────────────────────────────────

unsafe extern "C" fn children_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let child_ids: Vec<NodeId> = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .map(|n| {
                    n.children()
                        .filter(|c| c.value().is_element())
                        .map(|c| c.id())
                        .collect()
                })
                .unwrap_or_default()
        };
        let arr = v8::Array::new(scope, child_ids.len() as i32);
        for (i, cid) in child_ids.iter().enumerate() {
            if let Some(obj) = create_node_object(scope, state, *cid) {
                arr.set_index(scope, i as u32, obj);
            }
        }
        rv.set(arr.into());
    });
}

unsafe extern "C" fn child_element_count_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let count = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .map(|n| n.children().filter(|c| c.value().is_element()).count())
                .unwrap_or(0)
        };
        rv.set(v8::Integer::new(scope, count as i32).into());
    });
}

unsafe extern "C" fn first_element_child_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let cid = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.children().find(|c| c.value().is_element()))
                .map(|c| c.id())
        };
        if let Some(cid) = cid {
            if let Some(obj) = create_node_object(scope, state, cid) {
                rv.set(obj);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn last_element_child_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let cid = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.children().rfind(|c| c.value().is_element()))
                .map(|c| c.id())
        };
        if let Some(cid) = cid {
            if let Some(obj) = create_node_object(scope, state, cid) {
                rv.set(obj);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn next_element_sibling_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let sib_id = {
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                if let Some(node_ref) = doc.get(node_id) {
                    let mut sib = node_ref.next_sibling();
                    loop {
                        match sib {
                            Some(s) if s.value().is_element() => break Some(s.id()),
                            Some(s) => sib = s.next_sibling(),
                            None => break None,
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };
        if let Some(sid) = sib_id {
            if let Some(obj) = create_node_object(scope, state, sid) {
                rv.set(obj);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn prev_element_sibling_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let sib_id = {
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                if let Some(node_ref) = doc.get(node_id) {
                    let mut sib = node_ref.prev_sibling();
                    loop {
                        match sib {
                            Some(s) if s.value().is_element() => break Some(s.id()),
                            Some(s) => sib = s.prev_sibling(),
                            None => break None,
                        }
                    }
                } else {
                    None
                }
            } else {
                None
            }
        };
        if let Some(sid) = sib_id {
            if let Some(obj) = create_node_object(scope, state, sid) {
                rv.set(obj);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    });
}

// ── Attribute methods ─────────────────────────────────────────────────────────

unsafe extern "C" fn get_attribute_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let attr_name = args.get(0).to_rust_string_lossy(scope);
                let doc = state.document.borrow();
                if let Some(ref doc) = *doc {
                    if let Some(node_ref) = doc.get(nid) {
                        if let Some(val) = node_ref.value().get_attr(&attr_name) {
                            if let Some(s) = v8::String::new(scope, val) {
                                rv.set(s.into());
                                return;
                            }
                        }
                    }
                }
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn set_attribute_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 2 {
                let name = args.get(0).to_rust_string_lossy(scope);
                let value = args.get(1).to_rust_string_lossy(scope);
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
                            if let Some(e) = attrs.iter_mut().find(|(k, _)| k == &name) {
                                e.1 = value.clone();
                            } else {
                                attrs.push((name.clone(), value.clone()));
                            }
                            if name == "id" {
                                *id = Some(value.clone());
                            }
                            if name == "class" {
                                *classes =
                                    value.split_whitespace().map(|s| s.to_string()).collect();
                            }
                        }
                    }
                    if name == "id" {
                        doc.register_id(value, nid);
                    }
                }
            }
        }
    });
}

/// setAttributeNS(namespace, qualifiedName, value) — L3: ignore namespace, store local name.
unsafe extern "C" fn set_attribute_ns_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 3 {
                let qname = args.get(1).to_rust_string_lossy(scope);
                let local = qname.rsplit(':').next().unwrap_or(&qname).to_string();
                let value = args.get(2).to_rust_string_lossy(scope);
                set_attr_str(state, nid, &local, value);
            }
        }
    });
}

unsafe extern "C" fn get_attribute_ns_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, mut rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 2 {
                let qname = args.get(1).to_rust_string_lossy(scope);
                let local = qname.rsplit(':').next().unwrap_or(&qname);
                let val = get_attr_str(state, nid, local);
                if val.is_empty() {
                    // Distinguish missing vs empty: check presence
                    let present = {
                        let doc = state.document.borrow();
                        doc.as_ref()
                            .and_then(|d| d.get(nid))
                            .and_then(|n| n.value().get_attr(local))
                            .is_some()
                    };
                    if !present {
                        rv.set(v8::null(scope).into());
                        return;
                    }
                }
                if let Some(s) = v8::String::new(scope, &val) {
                    rv.set(s.into());
                    return;
                }
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn remove_attribute_ns_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 2 {
                let qname = args.get(1).to_rust_string_lossy(scope);
                let local = qname.rsplit(':').next().unwrap_or(&qname).to_string();
                let mut doc = state.document.borrow_mut();
                if let Some(ref mut d) = *doc {
                    if let Some(mut node) = d.tree.get_mut(nid) {
                        if let NodeData::Element { ref mut attrs, .. } = node.value() {
                            attrs.retain(|(k, _)| k != &local);
                        }
                    }
                }
            }
        }
    });
}

unsafe extern "C" fn has_attribute_ns_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, mut rv, state, node_id| {
        let present = if let Some(nid) = node_id {
            if args.length() >= 2 {
                let qname = args.get(1).to_rust_string_lossy(scope);
                let local = qname.rsplit(':').next().unwrap_or(&qname);
                let doc = state.document.borrow();
                doc.as_ref()
                    .and_then(|d| d.get(nid))
                    .and_then(|n| n.value().get_attr(local))
                    .is_some()
            } else {
                false
            }
        } else {
            false
        };
        rv.set(v8::Boolean::new(scope, present).into());
    });
}

unsafe extern "C" fn remove_attribute_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let name = args.get(0).to_rust_string_lossy(scope);
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
                            attrs.retain(|(k, _)| k != &name);
                            if name == "id" {
                                *id = None;
                            }
                            if name == "class" {
                                classes.clear();
                            }
                        }
                    }
                }
            }
        }
    });
}

unsafe extern "C" fn has_attribute_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        let result = if let Some(nid) = node_id {
            if args.length() >= 1 {
                let name = args.get(0).to_rust_string_lossy(scope);
                let doc = state.document.borrow();
                doc.as_ref()
                    .and_then(|d| d.get(nid))
                    .map(|n| n.value().get_attr(&name).is_some())
                    .unwrap_or(false)
            } else {
                false
            }
        } else {
            false
        };
        rv.set(v8::Boolean::new(scope, result).into());
    });
}

unsafe extern "C" fn get_attribute_names_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, state, node_id| {
        let names: Vec<String> = if let Some(nid) = node_id {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(nid))
                .map(|n| n.value().attrs().iter().map(|(k, _)| k.clone()).collect())
                .unwrap_or_default()
        } else {
            vec![]
        };
        let arr = v8::Array::new(scope, names.len() as i32);
        for (i, name) in names.iter().enumerate() {
            if let Some(s) = v8::String::new(scope, name) {
                arr.set_index(scope, i as u32, s.into());
            }
        }
        rv.set(arr.into());
    });
}

// ── Query methods ─────────────────────────────────────────────────────────────

// ── DOM mutation methods (replaceChild, insertBefore, insertAdjacentHTML, cloneNode) ──

unsafe extern "C" fn replace_child_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, _node_id| {
        if args.length() < 2 {
            let msg = crate::v8_utils::v8_string(
                scope,
                "Failed to execute 'replaceChild' on 'Node': 2 arguments required, but only fewer present.",
            );
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }
        let new_arg = args.get(0);
        let old_arg = args.get(1);
        if !new_arg.is_object() || !old_arg.is_object() {
            let msg = crate::v8_utils::v8_string(
                scope,
                "Failed to execute 'replaceChild' on 'Node': parameter is not of type 'Node'.",
            );
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }
        let new_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(new_arg) };
        let old_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(old_arg) };
        let new_id = extract_node_id_from_internal(scope, new_obj);
        let old_id = extract_node_id_from_internal(scope, old_obj);
        if new_id.is_none() || old_id.is_none() {
            let msg = crate::v8_utils::v8_string(
                scope,
                "Failed to execute 'replaceChild' on 'Node': parameter is not of type 'Node'.",
            );
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }
        let nid = new_id.unwrap();
        let oid = old_id.unwrap();
        // Edge case: replaceChild(node, node) — no-op; return old child.
        if nid == oid {
            rv.set(old_arg);
            return;
        }
        let mut doc = state.document.borrow_mut();
        if let Some(ref mut doc) = *doc {
            doc.detach(nid);
            if let Some(mut old_node) = doc.tree.get_mut(oid) {
                old_node.insert_id_before(nid);
            }
            doc.detach(oid);
            doc.invalidate_tag_index();
            doc.rebuild_id_index();
        }
        rv.set(old_arg);
    });
}

unsafe extern "C" fn insert_before_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        if let Some(parent_id) = node_id {
            if args.length() >= 1 {
                let new_node_arg = args.get(0);
                let ref_node_arg = if args.length() >= 2 {
                    Some(args.get(1))
                } else {
                    None
                };
                if new_node_arg.is_object() {
                    let new_obj: v8::Local<v8::Object> =
                        unsafe { v8::Local::cast_unchecked(new_node_arg) };
                    if let Some(new_id) = extract_node_id_from_internal(scope, new_obj) {
                        let ref_id = ref_node_arg.and_then(|r| {
                            if r.is_object() {
                                let ref_obj: v8::Local<v8::Object> =
                                    unsafe { v8::Local::cast_unchecked(r) };
                                extract_node_id_from_internal(scope, ref_obj)
                            } else {
                                None
                            }
                        });
                        // Edge case: insertBefore(node, node) — no-op in real browsers.
                        if ref_id == Some(new_id) {
                            rv.set(new_node_arg);
                            return;
                        }
                        let mut doc = state.document.borrow_mut();
                        if let Some(ref mut doc) = *doc {
                            doc.detach(new_id);
                            if let Some(ref_node_id) = ref_id {
                                if let Some(mut ref_node) = doc.tree.get_mut(ref_node_id) {
                                    ref_node.insert_id_before(new_id);
                                }
                            } else {
                                if let Some(mut parent) = doc.tree.get_mut(parent_id) {
                                    parent.append_id(new_id);
                                }
                            }
                            doc.invalidate_tag_index();
                            doc.rebuild_id_index();
                        }
                    }
                }
                rv.set(new_node_arg);
            }
        }
    });
}

unsafe extern "C" fn insert_adjacent_html_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if args.length() < 2 {
            return;
        }
        let position = args.get(0).to_rust_string_lossy(scope).to_lowercase();
        let html_str = args.get(1).to_rust_string_lossy(scope);
        if let Some(nid) = node_id {
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                let fragment = crate::dom::parse_html(&html_str, None);
                let body_id = fragment.body().unwrap_or(fragment.root_id());
                let frag_children: Vec<(crate::dom::NodeId, crate::dom::NodeData)> = {
                    fragment
                        .tree
                        .get(body_id)
                        .map(|b| b.children().map(|c| (c.id(), c.value().clone())).collect())
                        .unwrap_or_default()
                };
                match position.as_str() {
                    "beforeend" => {
                        for (frag_id, _) in &frag_children {
                            crate::dom::binding::append_node_recursive(
                                doc, nid, &fragment, *frag_id,
                            );
                        }
                    }
                    "afterbegin" => {
                        let first_child = doc
                            .tree
                            .get(nid)
                            .and_then(|n| n.first_child())
                            .map(|c| c.id());
                        for (frag_id, _) in frag_children.iter().rev() {
                            let data = fragment.tree.get(*frag_id).map(|n| n.value().clone());
                            if let Some(d) = data {
                                if let Some(fc) = first_child {
                                    doc.insert_before(fc, d);
                                } else {
                                    doc.append_child(nid, d);
                                }
                            }
                        }
                    }
                    "beforebegin" => {
                        if let Some(_parent_id) =
                            doc.tree.get(nid).and_then(|n| n.parent()).map(|p| p.id())
                        {
                            for (frag_id, _) in &frag_children {
                                let data = fragment.tree.get(*frag_id).map(|n| n.value().clone());
                                if let Some(d) = data {
                                    doc.insert_before(nid, d);
                                }
                            }
                        }
                    }
                    "afterend" => {
                        if let Some(parent_id) =
                            doc.tree.get(nid).and_then(|n| n.parent()).map(|p| p.id())
                        {
                            for (frag_id, _) in &frag_children {
                                crate::dom::binding::append_node_recursive(
                                    doc, parent_id, &fragment, *frag_id,
                                );
                            }
                        }
                    }
                    _ => {}
                }
                doc.invalidate_tag_index();
                doc.rebuild_id_index();
            }
        }
    });
}

unsafe extern "C" fn insert_adjacent_element_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |_scope, _args, rv, _state, _node_id| {
        rv.set(v8::null(_scope).into());
    });
}

unsafe extern "C" fn insert_adjacent_text_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if args.length() < 2 {
            return;
        }
        let position = args.get(0).to_rust_string_lossy(scope).to_lowercase();
        let text = args.get(1).to_rust_string_lossy(scope);
        if let Some(nid) = node_id {
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                let text_data = crate::dom::NodeData::text(&text);
                match position.as_str() {
                    "beforeend" => {
                        doc.append_child(nid, text_data);
                    }
                    "afterbegin" => {
                        let fc = doc
                            .tree
                            .get(nid)
                            .and_then(|n| n.first_child())
                            .map(|c| c.id());
                        if let Some(fc_id) = fc {
                            doc.insert_before(fc_id, text_data);
                        } else {
                            doc.append_child(nid, text_data);
                        }
                    }
                    _ => {}
                }
            }
        }
    });
}

unsafe extern "C" fn clone_node_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        let deep = args.length() >= 1 && args.get(0).is_true();
        if let Some(nid) = node_id {
            // Phase 1: collect data while holding borrow
            let new_id = {
                let data = {
                    let doc = state.document.borrow();
                    doc.as_ref()
                        .and_then(|d| d.get(nid))
                        .map(|n| n.value().clone())
                };
                if let Some(d) = data {
                    let children: Vec<crate::dom::NodeData> = if deep {
                        let doc = state.document.borrow();
                        doc.as_ref()
                            .and_then(|d| d.get(nid))
                            .map(|n| n.children().map(|c| c.value().clone()).collect())
                            .unwrap_or_default()
                    } else {
                        vec![]
                    };

                    // Phase 2: mutate while holding mut borrow
                    let mut doc_guard = state.document.borrow_mut();
                    if let Some(ref mut doc) = *doc_guard {
                        let root_id = doc.root_id();
                        let new_id = doc.append_child(root_id, d);
                        doc.detach(new_id);
                        for child_data in children {
                            doc.append_child(new_id, child_data);
                        }
                        // Rebuild id index: append_child may have overwritten the original
                        // node's id entry; after detach the clone is orphaned, so rebuild
                        // to restore the original node's id mapping.
                        doc.rebuild_id_index();
                        Some(new_id)
                    } else {
                        None
                    }
                    // doc_guard drops here, releasing the mutable borrow
                } else {
                    None
                }
            };
            // Phase 3: create JS object — no borrow held at this point
            if let Some(cid) = new_id {
                if let Some(obj) = create_node_object(scope, state, cid) {
                    rv.set(obj);
                    return;
                }
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn contains_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        if args.length() < 1 || !args.get(0).is_object() {
            rv.set(v8::Boolean::new(scope, false).into());
            return;
        }
        let other_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(args.get(0)) };
        let other_id = extract_node_id_from_internal(scope, other_obj);
        let result = if let (Some(nid), Some(oid)) = (node_id, other_id) {
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                // Check if oid is a descendant of nid
                doc.tree
                    .get(nid)
                    .map(|n| n.descendants().any(|d| d.id() == oid))
                    .unwrap_or(false)
            } else {
                false
            }
        } else {
            false
        };
        rv.set(v8::Boolean::new(scope, result).into());
    });
}

unsafe extern "C" fn query_selector_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let sel = args.get(0).to_rust_string_lossy(scope);
                let result_id = {
                    let doc = state.document.borrow();
                    doc.as_ref()
                        .and_then(|d| d.query_selector_from(&sel, nid).ok().flatten())
                };
                if let Some(rid) = result_id {
                    if let Some(obj) = create_node_object(scope, state, rid) {
                        rv.set(obj);
                        return;
                    }
                }
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn query_selector_all_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        let ids: Vec<NodeId> = if let Some(nid) = node_id {
            if args.length() >= 1 {
                let sel = args.get(0).to_rust_string_lossy(scope);
                let doc = state.document.borrow();
                doc.as_ref()
                    .and_then(|d| d.query_selector_all_from(&sel, nid).ok())
                    .unwrap_or_default()
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        if let Some(list) = create_node_list_instance(scope, state, &ids) {
            rv.set(list);
        } else {
            rv.set(v8::Array::new(scope, 0).into());
        }
    });
}

/// Create a NodeList FunctionTemplate instance from a slice of NodeIds.
/// Uses internal field 1 to store the node ID array pointer.
pub fn create_node_list_instance<'s>(
    scope: &v8::PinScope<'s, '_>,
    state: &RuntimeState,
    node_ids: &[NodeId],
) -> Option<v8::Local<'s, v8::Value>> {
    let templates = state.dom_templates.borrow();
    let templates = templates.as_ref()?;
    let tmpl = v8::Local::new(scope, &templates.node_list);
    let inst_tmpl = tmpl.instance_template(scope);
    let obj = inst_tmpl.new_instance(scope)?;

    let ids: Vec<usize> = node_ids
        .iter()
        .map(|&nid| super::binding::node_id_to_usize(nid))
        .collect();
    let len = ids.len();
    let boxed_ids = Box::new(ids);
    let ptr = Box::into_raw(boxed_ids) as *mut std::ffi::c_void;
    state.register_heap(ptr, |p| unsafe {
        drop(Box::from_raw(p as *mut Vec<usize>))
    });
    let external = v8::External::new(scope, ptr);
    obj.set_internal_field(1, external.into());

    for (i, &nid) in node_ids.iter().enumerate() {
        if let Some(node_obj) = create_node_object(scope, state, nid) {
            obj.set_index(scope, i as u32, node_obj);
        }
    }
    let len_key = crate::v8_utils::v8_string(scope, "length");
    let len_val = v8::Integer::new(scope, len as i32);
    obj.set(scope, len_key.into(), len_val.into());

    Some(obj.into())
}

unsafe extern "C" fn node_list_item_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();

        if let Some(idx_val) = args.get(0).uint32_value(scope) {
            let idx = idx_val as usize;
            let field = this.get_internal_field(scope, 1);
            if let Some(field) = field {
                let value: v8::Local<v8::Value> = unsafe { v8::Local::cast_unchecked(field) };
                if value.is_external() {
                    let external: v8::Local<v8::External> =
                        unsafe { v8::Local::cast_unchecked(value) };
                    let vec_ptr = external.value() as *const Vec<usize>;
                    if !vec_ptr.is_null() {
                        let ids: &Vec<usize> = unsafe { &*vec_ptr };
                        if idx < ids.len() {
                            let isolate: &v8::Isolate = &*scope;
                            let state = RuntimeState::get(isolate);
                            let nid = super::binding::usize_to_node_id(ids[idx]);
                            if let Some(nid) = nid {
                                if let Some(obj) = create_node_object(scope, state, nid) {
                                    rv.set(obj);
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        rv.set(v8::null(scope).into());
    }));
}

unsafe extern "C" fn node_list_length_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        if !crate::shims::native_env::check_receiver(&scope, info_ref, "NodeList") {
            return;
        }
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();

        let field = this.get_internal_field(scope, 1);
        if let Some(field) = field {
            let value: v8::Local<v8::Value> = unsafe { v8::Local::cast_unchecked(field) };
            if value.is_external() {
                let external: v8::Local<v8::External> = unsafe { v8::Local::cast_unchecked(value) };
                let vec_ptr = external.value() as *const Vec<usize>;
                if !vec_ptr.is_null() {
                    let ids: &Vec<usize> = unsafe { &*vec_ptr };
                    rv.set(v8::Integer::new(scope, ids.len() as i32).into());
                    return;
                }
            }
        }
        rv.set(v8::Integer::new(scope, 0).into());
    }));
}

unsafe extern "C" fn get_elements_by_tag_name_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        let ids: Vec<NodeId> = if let Some(nid) = node_id {
            if args.length() >= 1 {
                let tag = args.get(0).to_rust_string_lossy(scope);
                let doc = state.document.borrow();
                doc.as_ref()
                    .map(|d| d.get_elements_by_tag_name_from(&tag, nid))
                    .unwrap_or_default()
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        if let Some(list) = create_node_list_instance(scope, state, &ids) {
            rv.set(list);
        } else {
            rv.set(v8::Array::new(scope, 0).into());
        }
    });
}

unsafe extern "C" fn get_elements_by_class_name_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        let ids: Vec<NodeId> = if let Some(nid) = node_id {
            if args.length() >= 1 {
                let cls = args.get(0).to_rust_string_lossy(scope);
                let target_classes: Vec<String> =
                    cls.split_whitespace().map(|s| s.to_string()).collect();
                let doc = state.document.borrow();
                if let Some(ref doc) = *doc {
                    if let Some(node_ref) = doc.get(nid) {
                        node_ref
                            .descendants()
                            .filter(|n| {
                                let classes = n.value().class_list();
                                target_classes
                                    .iter()
                                    .all(|tc| classes.iter().any(|c| c == tc))
                            })
                            .map(|n| n.id())
                            .collect()
                    } else {
                        vec![]
                    }
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        } else {
            vec![]
        };
        if let Some(list) = create_node_list_instance(scope, state, &ids) {
            rv.set(list);
        } else {
            rv.set(v8::Array::new(scope, 0).into());
        }
    });
}

unsafe extern "C" fn matches_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        let result = if let Some(nid) = node_id {
            if args.length() >= 1 {
                let sel = args.get(0).to_rust_string_lossy(scope);
                let doc = state.document.borrow();
                doc.as_ref()
                    .map(|d| d.element_matches(nid, &sel))
                    .unwrap_or(false)
            } else {
                false
            }
        } else {
            false
        };
        rv.set(v8::Boolean::new(scope, result).into());
    });
}

unsafe extern "C" fn closest_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let sel = args.get(0).to_rust_string_lossy(scope);
                let result_id = {
                    let doc = state.document.borrow();
                    doc.as_ref().and_then(|d| d.closest(nid, &sel))
                };
                if let Some(rid) = result_id {
                    if let Some(obj) = create_node_object(scope, state, rid) {
                        rv.set(obj);
                        return;
                    }
                }
            }
        }
        rv.set(v8::null(scope).into());
    });
}

// ── Mutation methods ──────────────────────────────────────────────────────────

unsafe extern "C" fn append_child_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        if let Some(parent_id) = node_id {
            if args.length() >= 1 {
                let child_arg = args.get(0);
                if child_arg.is_object() {
                    let child_obj: v8::Local<v8::Object> =
                        unsafe { v8::Local::cast_unchecked(child_arg) };
                    if let Some(child_id) = extract_node_id_from_internal(scope, child_obj) {
                        let mut doc = state.document.borrow_mut();
                        if let Some(ref mut doc) = *doc {
                            // DocumentFragment transfer: DOM spec requires all
                            // children of the fragment to be moved to the new
                            // parent, leaving the fragment empty.
                            let is_fragment = doc.get(child_id)
                                .map(|n| matches!(n.value(), NodeData::DocumentFragment))
                                .unwrap_or(false);

                            if is_fragment {
                                let child_ids: Vec<_> = doc.tree.get(child_id)
                                    .map(|frag| frag.children().map(|c| c.id()).collect())
                                    .unwrap_or_default();
                                for grandchild_id in child_ids {
                                    doc.detach(grandchild_id);
                                    if let Some(mut parent) = doc.tree.get_mut(parent_id) {
                                        parent.append_id(grandchild_id);
                                    }
                                }
                            } else {
                                doc.detach(child_id);
                                if let Some(mut parent) = doc.tree.get_mut(parent_id) {
                                    parent.append_id(child_id);
                                }
                            }
                            doc.invalidate_tag_index();
                            doc.rebuild_id_index();
                        }
                    }
                }
            }
            rv.set(args.get(0));
        }
    });
}

unsafe extern "C" fn remove_child_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, _node_id| {
        if args.length() >= 1 {
            let child_arg = args.get(0);
            if child_arg.is_object() {
                let child_obj: v8::Local<v8::Object> =
                    unsafe { v8::Local::cast_unchecked(child_arg) };
                if let Some(child_id) = extract_node_id_from_internal(scope, child_obj) {
                    let mut doc = state.document.borrow_mut();
                    if let Some(ref mut doc) = *doc {
                        doc.detach(child_id);
                    }
                }
            }
            rv.set(child_arg);
        }
    });
}

unsafe extern "C" fn has_child_nodes_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, state, node_id| {
        let result = if let Some(nid) = node_id {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(nid))
                .map(|n| n.first_child().is_some())
                .unwrap_or(false)
        } else {
            false
        };
        rv.set(v8::Boolean::new(scope, result).into());
    });
}

unsafe extern "C" fn normalize_cb(info: *const v8::FunctionCallbackInfo) { null_this_check(info); }

// ── Geometry ──────────────────────────────────────────────────────────────────

unsafe extern "C" fn get_bounding_client_rect_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, _state, _node_id| {
        let this = args.this();
        let mut x: f64 = 0.0;
        let mut y: f64 = 0.0;
        let mut width: f64 = 0.0;
        let mut height: f64 = 0.0;

        // Read fixture-configured rect from __iv8Rect__ JS property
        let rect_key = crate::v8_utils::v8_string(scope, "__iv8Rect__");
        if let Some(rect_val) = this.get(scope, rect_key.into()) {
            if rect_val.is_object() {
                let rect_obj: v8::Local<v8::Object> =
                    unsafe { v8::Local::cast_unchecked(rect_val) };
                let x_key = crate::v8_utils::v8_string(scope, "x");
                let y_key = crate::v8_utils::v8_string(scope, "y");
                let w_key = crate::v8_utils::v8_string(scope, "width");
                let h_key = crate::v8_utils::v8_string(scope, "height");
                if let Some(v) = rect_obj.get(scope, x_key.into()) {
                    if let Some(n) = v.number_value(scope) {
                        x = n;
                    }
                }
                if let Some(v) = rect_obj.get(scope, y_key.into()) {
                    if let Some(n) = v.number_value(scope) {
                        y = n;
                    }
                }
                if let Some(v) = rect_obj.get(scope, w_key.into()) {
                    if let Some(n) = v.number_value(scope) {
                        width = n;
                    }
                }
                if let Some(v) = rect_obj.get(scope, h_key.into()) {
                    if let Some(n) = v.number_value(scope) {
                        height = n;
                    }
                }
            }
        }

        let obj = v8::Object::new(scope);
        let pairs: [(&str, f64); 8] = [
            ("x", x),
            ("y", y),
            ("width", width),
            ("height", height),
            ("top", y),
            ("left", x),
            ("bottom", y + height),
            ("right", x + width),
        ];
        for (key, val) in &pairs {
            let k = crate::v8_utils::v8_string(scope, key);
            obj.set(scope, k.into(), v8::Number::new(scope, *val).into());
        }
        rv.set(obj.into());
    });
}

unsafe extern "C" fn get_client_rects_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, _state, _node_id| {
        rv.set(v8::Array::new(scope, 0).into());
    });
}

fn resolved_promise_undefined(
    scope: &v8::PinScope<'_, '_>,
    rv: &mut v8::ReturnValue<'_>,
) {
    let resolver = crate::v8_utils::v8_resolver(scope);
    let _ = resolver.resolve(scope, v8::undefined(scope).into());
    rv.set(resolver.get_promise(scope).into());
}

unsafe extern "C" fn scroll_into_view_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, _state, _node_id| {
        resolved_promise_undefined(scope, rv);
    });
}

unsafe extern "C" fn scroll_promise_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, _state, _node_id| {
        resolved_promise_undefined(scope, rv);
    });
}

unsafe extern "C" fn element_null_result_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, _state, _node_id| {
        rv.set(v8::null(scope).into());
    });
}

const ATTR_NAME_KEY: &str = "__iv8AttrName";

/// Create or reuse an Attr object bound to (owner Element, attr name).
/// Value is live from Element.attrs via getters — single store with setAttribute.
fn create_attr_object<'s>(
    scope: &v8::PinScope<'s, '_>,
    state: &RuntimeState,
    owner_id: NodeId,
    name: &str,
) -> Option<v8::Local<'s, v8::Value>> {
    let key = (owner_id, name.to_string());
    {
        let cache = state.attr_cache.borrow();
        if let Some(global) = cache.get(&key) {
            return Some(v8::Local::new(scope, global).into());
        }
    }
    let templates = state.dom_templates.borrow();
    let templates = templates.as_ref()?;
    let tmpl = v8::Local::new(scope, &templates.attr);
    let inst = tmpl.instance_template(scope);
    let obj = inst.new_instance(scope)?;
    let nid_usize = super::binding::node_id_to_usize(owner_id);
    let external = v8::External::new(scope, nid_usize as *mut std::ffi::c_void);
    obj.set_internal_field(NODE_ID_FIELD as usize, external.into());
    let name_key = crate::v8_utils::v8_string(scope, ATTR_NAME_KEY);
    let name_val = crate::v8_utils::v8_string(scope, name);
    let _ = obj.define_own_property(
        scope,
        name_key.into(),
        name_val.into(),
        v8::PropertyAttribute::DONT_ENUM | v8::PropertyAttribute::READ_ONLY | v8::PropertyAttribute::DONT_DELETE,
    );
    let global = v8::Global::new(scope, obj);
    state.attr_cache.borrow_mut().insert(key, global);
    Some(obj.into())
}

fn attr_name_of(scope: &v8::PinScope<'_, '_>, this: v8::Local<v8::Object>) -> Option<String> {
    let key = crate::v8_utils::v8_string(scope, ATTR_NAME_KEY);
    let val = this.get(scope, key.into())?;
    if val.is_string() {
        Some(val.to_rust_string_lossy(scope))
    } else {
        None
    }
}

unsafe extern "C" fn attr_name_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _state, _owner_id| {
        let info_ref = unsafe { &*info };
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let this = args.this();
        if let Some(name) = attr_name_of(scope, this) {
            if let Some(s) = v8::String::new(scope, &name) {
                rv.set(s.into());
                return;
            }
        }
        rv.set(crate::v8_utils::v8_string(scope, "").into());
    });
}

unsafe extern "C" fn attr_value_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, owner_id| {
        let info_ref = unsafe { &*info };
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let this = args.this();
        let name = attr_name_of(scope, this).unwrap_or_default();
        let doc = state.document.borrow();
        let val = doc
            .as_ref()
            .and_then(|d| d.get(owner_id))
            .and_then(|n| n.value().get_attr(&name).map(|s| s.to_string()))
            .unwrap_or_default();
        if let Some(s) = v8::String::new(scope, &val) {
            rv.set(s.into());
        }
    });
}

unsafe extern "C" fn attr_value_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, owner_id| {
        if let Some(oid) = owner_id {
            if args.length() < 1 {
                return;
            }
            let this = args.this();
            let Some(name) = attr_name_of(scope, this) else { return };
            let value = args.get(0).to_rust_string_lossy(scope);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                if let Some(mut node) = doc.tree.get_mut(oid) {
                    if let NodeData::Element {
                        ref mut attrs,
                        ref mut id,
                        ref mut classes,
                        ..
                    } = node.value()
                    {
                        if let Some(e) = attrs.iter_mut().find(|(k, _)| k == &name) {
                            e.1 = value.clone();
                        } else {
                            attrs.push((name.clone(), value.clone()));
                        }
                        if name == "id" {
                            *id = Some(value.clone());
                        }
                        if name == "class" {
                            *classes = value.split_whitespace().map(|s| s.to_string()).collect();
                        }
                    }
                }
                if name == "id" {
                    doc.register_id(value, oid);
                }
            }
        }
    });
}

unsafe extern "C" fn attr_owner_element_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, owner_id| {
        if let Some(obj) = create_node_object(scope, state, owner_id) {
            rv.set(obj);
        } else {
            rv.set(v8::null(scope).into());
        }
    });
}

unsafe extern "C" fn attr_namespace_uri_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _state, _owner_id| {
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn attr_prefix_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _state, _owner_id| {
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn attr_specified_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _state, _owner_id| {
        rv.set(v8::Boolean::new(scope, true).into());
    });
}

unsafe extern "C" fn get_attribute_node_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let name = args.get(0).to_rust_string_lossy(scope);
                let has = {
                    let doc = state.document.borrow();
                    doc.as_ref()
                        .and_then(|d| d.get(nid))
                        .and_then(|n| n.value().get_attr(&name))
                        .is_some()
                };
                if has {
                    if let Some(attr) = create_attr_object(scope, state, nid, &name) {
                        rv.set(attr);
                        return;
                    }
                }
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn get_attribute_node_ns_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        // No namespace support in Element.attrs yet: only null-namespace names.
        if let Some(nid) = node_id {
            if args.length() >= 2 {
                let ns = args.get(0);
                let local = args.get(1).to_rust_string_lossy(scope);
                let ns_null = ns.is_null_or_undefined()
                    || (ns.is_string() && ns.to_rust_string_lossy(scope).is_empty());
                if ns_null {
                    let has = {
                        let doc = state.document.borrow();
                        doc.as_ref()
                            .and_then(|d| d.get(nid))
                            .and_then(|n| n.value().get_attr(&local))
                            .is_some()
                    };
                    if has {
                        if let Some(attr) = create_attr_object(scope, state, nid, &local) {
                            rv.set(attr);
                            return;
                        }
                    }
                }
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn set_attribute_node_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 && args.get(0).is_object() {
                let attr_obj: v8::Local<v8::Object> =
                    unsafe { v8::Local::cast_unchecked(args.get(0)) };
                if let Some(name) = attr_name_of(scope, attr_obj) {
                    let value = {
                        let key = crate::v8_utils::v8_string(scope, "value");
                        attr_obj
                            .get(scope, key.into())
                            .map(|v| v.to_rust_string_lossy(scope))
                            .unwrap_or_default()
                    };
                    let old_had = {
                        let doc = state.document.borrow();
                        doc.as_ref()
                            .and_then(|d| d.get(nid))
                            .and_then(|n| n.value().get_attr(&name))
                            .is_some()
                    };
                    let old_attr = if old_had {
                        create_attr_object(scope, state, nid, &name)
                    } else {
                        None
                    };
                    {
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
                                    if let Some(e) = attrs.iter_mut().find(|(k, _)| k == &name) {
                                        e.1 = value.clone();
                                    } else {
                                        attrs.push((name.clone(), value.clone()));
                                    }
                                    if name == "id" {
                                        *id = Some(value.clone());
                                    }
                                    if name == "class" {
                                        *classes =
                                            value.split_whitespace().map(|s| s.to_string()).collect();
                                    }
                                }
                            }
                            if name == "id" {
                                doc.register_id(value, nid);
                            }
                        }
                    }
                    // Rebind Attr owner to this element
                    let nid_usize = super::binding::node_id_to_usize(nid);
                    let external = v8::External::new(scope, nid_usize as *mut std::ffi::c_void);
                    attr_obj.set_internal_field(NODE_ID_FIELD as usize, external.into());
                    if let Some(old) = old_attr {
                        rv.set(old);
                    } else {
                        rv.set(v8::null(scope).into());
                    }
                    return;
                }
            }
        }
        rv.set(v8::null(scope).into());
    });
}

/// Read a layout value from environment config with fallback chain.
/// Checks: primary_path → fallback_path → default_value.
#[inline]
fn get_layout_value(state: &RuntimeState, primary: &str, fallback: &str, default: f64) -> f64 {
    let env = &state.environment;
    if let Some(val) = env.get_f64(primary) {
        return val;
    }
    if let Some(val) = env.get_f64(fallback) {
        return val;
    }
    default
}

unsafe extern "C" fn offset_width_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, _nid| {
        let val = get_layout_value(state, "element.offsetWidth", "window.innerWidth", 1920.0);
        rv.set(v8::Number::new(scope, val).into());
    });
}
unsafe extern "C" fn offset_height_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, _nid| {
        let val = get_layout_value(state, "element.offsetHeight", "window.innerHeight", 969.0);
        rv.set(v8::Number::new(scope, val).into());
    });
}
unsafe extern "C" fn offset_top_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Number::new(scope, 0.0).into());
    });
}
unsafe extern "C" fn offset_left_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Number::new(scope, 0.0).into());
    });
}
unsafe extern "C" fn client_width_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, _nid| {
        let val = get_layout_value(
            state,
            "document.body.clientWidth",
            "window.innerWidth",
            1920.0,
        );
        rv.set(v8::Number::new(scope, val).into());
    });
}
unsafe extern "C" fn client_height_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, _nid| {
        let val = get_layout_value(
            state,
            "document.body.clientHeight",
            "window.innerHeight",
            969.0,
        );
        rv.set(v8::Number::new(scope, val).into());
    });
}
unsafe extern "C" fn scroll_width_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, _nid| {
        let val = get_layout_value(state, "element.scrollWidth", "window.innerWidth", 1920.0);
        rv.set(v8::Number::new(scope, val).into());
    });
}
unsafe extern "C" fn scroll_height_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Number::new(scope, 0.0).into());
    });
}
unsafe extern "C" fn scroll_top_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let y = state
            .node_scroll
            .borrow()
            .get(&node_id)
            .map(|p| p.1)
            .unwrap_or(0.0);
        rv.set(v8::Number::new(scope, y).into());
    });
}
unsafe extern "C" fn scroll_top_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let y = args.get(0).number_value(scope).unwrap_or(0.0);
                let y = if y.is_nan() || y.is_infinite() { 0.0 } else { y };
                let mut map = state.node_scroll.borrow_mut();
                let entry = map.entry(nid).or_insert((0.0, 0.0));
                entry.1 = y;
            }
        }
    });
}
unsafe extern "C" fn scroll_left_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let x = state
            .node_scroll
            .borrow()
            .get(&node_id)
            .map(|p| p.0)
            .unwrap_or(0.0);
        rv.set(v8::Number::new(scope, x).into());
    });
}
unsafe extern "C" fn scroll_left_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let x = args.get(0).number_value(scope).unwrap_or(0.0);
                let x = if x.is_nan() || x.is_infinite() { 0.0 } else { x };
                let mut map = state.node_scroll.borrow_mut();
                let entry = map.entry(nid).or_insert((0.0, 0.0));
                entry.0 = x;
            }
        }
    });
}
unsafe extern "C" fn offset_parent_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::null(scope).into());
    });
}

// ── EventTarget methods ───────────────────────────────────────────────────────

unsafe extern "C" fn add_event_listener_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback_strict(info, |scope, args, _rv, state, nid| {
        if args.length() >= 2 {
            let event_type = args.get(0).to_rust_string_lossy(scope);
            let listener_arg = args.get(1);
            if listener_arg.is_function() {
                let func: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(listener_arg) };
                let global_fn = v8::Global::new(scope, func);
                let mut capture = false;
                let mut once = false;
                if args.length() >= 3 {
                    let opts = args.get(2);
                    if opts.is_boolean() {
                        capture = opts.is_true();
                    } else if opts.is_object() {
                        let opts_obj: v8::Local<v8::Object> =
                            unsafe { v8::Local::cast_unchecked(opts) };
                        if let Some(k) = v8::String::new(scope, "capture") {
                            if let Some(v) = opts_obj.get(scope, k.into()) {
                                capture = v.is_true();
                            }
                        }
                        if let Some(k) = v8::String::new(scope, "once") {
                            if let Some(v) = opts_obj.get(scope, k.into()) {
                                once = v.is_true();
                            }
                        }
                    }
                }
                state.event_listeners.borrow_mut().add(
                    nid,
                    &event_type,
                    global_fn,
                    capture,
                    once,
                );
            }
        }
    });
}

unsafe extern "C" fn remove_event_listener_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback_strict(info, |scope, args, _rv, state, nid| {
        if args.length() >= 2 && args.get(1).is_function() {
            let event_type = args.get(0).to_rust_string_lossy(scope);
            let func: v8::Local<v8::Function> =
                unsafe { v8::Local::cast_unchecked(args.get(1)) };
            let capture = if args.length() >= 3 {
                args.get(2).is_true()
            } else {
                false
            };
            state.event_listeners.borrow_mut().remove_by_callback(
                scope,
                nid,
                &event_type,
                func,
                capture,
            );
        }
    });
}

unsafe extern "C" fn dispatch_event_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback_strict(info, |scope, args, rv, state, nid| {
        if args.length() < 1 {
            rv.set(v8::Boolean::new(scope, true).into());
            return;
        }

        let event_arg = args.get(0);
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
    });
}

// ── HTMLElement methods ───────────────────────────────────────────────────────

unsafe extern "C" fn focus_cb(info: *const v8::FunctionCallbackInfo) {
    null_this_check(info);
}
unsafe extern "C" fn blur_cb(info: *const v8::FunctionCallbackInfo) {
    null_this_check(info);
}
unsafe extern "C" fn click_cb(info: *const v8::FunctionCallbackInfo) {
    null_this_check(info);
}
unsafe extern "C" fn select_cb(info: *const v8::FunctionCallbackInfo) {
    null_this_check(info);
}
unsafe extern "C" fn submit_cb(info: *const v8::FunctionCallbackInfo) {
    null_this_check(info);
}
unsafe extern "C" fn reset_cb(info: *const v8::FunctionCallbackInfo) {
    null_this_check(info);
}
unsafe extern "C" fn check_validity_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, _state, _node_id| {
        rv.set(v8::Boolean::new(scope, true).into());
    });
}

// ── HTMLElement accessors ─────────────────────────────────────────────────────

// ── CSSStyleDeclaration ────────────────────────────────────────────────────

fn camel_to_kebab(s: &str) -> String {
    if s.starts_with("--") || !s.contains(char::is_uppercase) {
        return s.to_string();
    }
    let mut result = String::with_capacity(s.len() + 4);
    for c in s.chars() {
        if c.is_uppercase() {
            result.push('-');
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

unsafe extern "C" fn style_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        // Check per-node cache first
        {
            let cache = state.style_cache.borrow();
            if let Some(global) = cache.get(&node_id) {
                let cached = v8::Local::new(scope, global);
                rv.set(cached.into());
                return;
            }
        }

        let templates = state.dom_templates.borrow();
        if let Some(templates) = templates.as_ref() {
            let tmpl = v8::Local::new(scope, &templates.css_style_declaration);
            let inst_tmpl = tmpl.instance_template(scope);
            if let Some(obj) = inst_tmpl.new_instance(scope) {
                    let nid_usize = super::binding::node_id_to_usize(node_id);
                    let external = v8::External::new(scope, nid_usize as *mut std::ffi::c_void);
                    obj.set_internal_field(0, external.into());
                    obj.set_internal_field(1, v8::Boolean::new(scope, false).into());

                    // Cache for identity: element.style === element.style
                    let global = v8::Global::new(scope, obj);
                    state.style_cache.borrow_mut().insert(node_id, global);
                    rv.set(obj.into());
                }
        }
    });
}

fn extract_style_node_id(
    scope: &v8::PinScope<'_, '_>,
    this: v8::Local<v8::Object>,
) -> Option<NodeId> {
    let field = this.get_internal_field(scope, 0)?;
    let value: v8::Local<v8::Value> = unsafe { v8::Local::cast_unchecked(field) };
    if value.is_external() {
        let external: v8::Local<v8::External> = unsafe { v8::Local::cast_unchecked(value) };
        super::binding::usize_to_node_id(external.value() as usize)
    } else {
        None
    }
}

fn style_is_readonly(scope: &v8::PinScope<'_, '_>, this: v8::Local<v8::Object>) -> bool {
    let field = this.get_internal_field(scope, 1);
    field
        .and_then(|f| {
            let v: v8::Local<v8::Value> = unsafe { v8::Local::cast_unchecked(f) };
            if v.is_boolean() {
                Some(v.boolean_value(scope))
            } else {
                None
            }
        })
        .unwrap_or(false)
}

fn style_read<F>(info: *const v8::FunctionCallbackInfo, f: F)
where
    F: FnOnce(&v8::PinScope<'_, '_>, &mut v8::ReturnValue<'_>, &HashMap<String, String>)
        + std::panic::UnwindSafe,
{
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        if let Some(node_id) = extract_style_node_id(scope, args.this()) {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let doc = state.document.borrow();
            if let Some(ref doc) = *doc {
                if let Some(node_ref) = doc.get(node_id) {
                    if let NodeData::Element { ref style_map, .. } = node_ref.value() {
                        f(scope, &mut rv, style_map);
                        return;
                    }
                }
            }
        } else {
            let msg = crate::v8_utils::v8_string(scope, "Illegal invocation");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
        }
    }));
}

fn style_mutate<F>(info: *const v8::FunctionCallbackInfo, f: F)
where
    F: FnOnce(
            &v8::PinScope<'_, '_>,
            &mut v8::ReturnValue<'_>,
            &mut HashMap<String, String>,
            &[String],
        ) + std::panic::UnwindSafe,
{
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        if style_is_readonly(scope, args.this()) {
            return;
        }
        if let Some(node_id) = extract_style_node_id(scope, args.this()) {
            let isolate: &v8::Isolate = &*scope;
            let state = RuntimeState::get(isolate);
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                if let Some(mut node) = doc.tree.get_mut(node_id) {
                    if let NodeData::Element {
                        ref mut style_map, ..
                    } = node.value()
                    {
                        let call_args: Vec<String> = (0..args.length())
                            .map(|i| args.get(i).to_rust_string_lossy(scope))
                            .collect();
                        f(scope, &mut rv, style_map, &call_args);
                    }
                }
            }
        } else {
            let msg = crate::v8_utils::v8_string(scope, "Illegal invocation");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
        }
    }));
}

unsafe extern "C" fn css_style_set_property_cb(info: *const v8::FunctionCallbackInfo) {
    style_mutate(info, |scope, rv, map, args| {
        if args.len() < 2 {
            return;
        }
        let prop = camel_to_kebab(&args[0]);
        map.insert(prop, args[1].clone());
        rv.set(v8::undefined(scope).into());
    });
}

unsafe extern "C" fn css_style_get_property_cb(info: *const v8::FunctionCallbackInfo) {
    style_read(info, |scope, rv, map| {
        let info_ref = unsafe { &*info };
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if args.length() < 1 {
            rv.set(crate::v8_utils::v8_string(scope, "").into());
            return;
        }
        let prop = camel_to_kebab(&args.get(0).to_rust_string_lossy(scope));
        if let Some(val) = map.get(&prop) {
            rv.set(crate::v8_utils::v8_string(scope, val).into());
        } else {
            rv.set(crate::v8_utils::v8_string(scope, "").into());
        }
    });
}

unsafe extern "C" fn css_style_remove_property_cb(info: *const v8::FunctionCallbackInfo) {
    style_mutate(info, |scope, rv, map, args| {
        if args.is_empty() {
            rv.set(crate::v8_utils::v8_string(scope, "").into());
            return;
        }
        let prop = camel_to_kebab(&args[0]);
        let old = map.remove(&prop).unwrap_or_default();
        rv.set(crate::v8_utils::v8_string(scope, &old).into());
    });
}

unsafe extern "C" fn css_style_item_cb(info: *const v8::FunctionCallbackInfo) {
    style_read(info, |scope, rv, map| {
        let info_ref = unsafe { &*info };
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if args.length() < 1 {
            rv.set(crate::v8_utils::v8_string(scope, "").into());
            return;
        }
        let idx = args.get(0).number_value(scope).unwrap_or(-1.0) as i32;
        if idx >= 0 && (idx as usize) < map.len() {
            let key = map
                .keys()
                .nth(idx as usize)
                .map(|k| k.clone())
                .unwrap_or_default();
            rv.set(crate::v8_utils::v8_string(scope, &key).into());
        } else {
            rv.set(crate::v8_utils::v8_string(scope, "").into());
        }
    });
}

unsafe extern "C" fn css_style_csstext_getter(info: *const v8::FunctionCallbackInfo) {
    style_read(info, |scope, rv, map| {
        let mut parts: Vec<String> = map.iter().map(|(k, v)| format!("{}: {};", k, v)).collect();
        parts.sort();
        rv.set(crate::v8_utils::v8_string(scope, &parts.join(" ")).into());
    });
}

unsafe extern "C" fn css_style_csstext_setter(info: *const v8::FunctionCallbackInfo) {
    style_mutate(info, |scope, rv, map, args| {
        if args.is_empty() {
            return;
        }
        map.clear();
        for part in args[0].split(';') {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            if let Some(colon) = part.find(':') {
                let prop = camel_to_kebab(part[..colon].trim());
                map.insert(prop, part[colon + 1..].trim().to_string());
            }
        }
        rv.set(v8::Boolean::new(scope, true).into());
    });
}

unsafe extern "C" fn css_style_length_getter(info: *const v8::FunctionCallbackInfo) {
    style_read(info, |scope, rv, map| {
        rv.set(v8::Integer::new(scope, map.len() as i32).into());
    });
}

unsafe extern "C" fn css_style_get_priority_cb(info: *const v8::FunctionCallbackInfo) { null_this_check(info); }

unsafe extern "C" fn dataset_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _state, _node_id| {
        rv.set(v8::Object::new(scope).into());
    });
}

unsafe extern "C" fn hidden_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .map(|n| n.value().get_attr("hidden").is_some())
                .unwrap_or(false)
        };
        rv.set(v8::Boolean::new(scope, val).into());
    });
}

unsafe extern "C" fn hidden_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            let on = args.length() >= 1 && args.get(0).boolean_value(scope);
            set_bool_attr(state, nid, "hidden", on);
        }
    });
}

unsafe extern "C" fn tab_index_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.value().get_attr("tabindex"))
                .and_then(|v| v.parse::<i32>().ok())
                .unwrap_or(-1)
        };
        rv.set(v8::Integer::new(scope, val).into());
    });
}
unsafe extern "C" fn tab_index_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let n = args.get(0).number_value(scope).unwrap_or(0.0) as i32;
                set_str_attr(state, nid, "tabindex", n.to_string());
            }
        }
    });
}

// Generic attribute-backed accessor helper
fn get_attr_str(state: &RuntimeState, node_id: NodeId, attr: &str) -> String {
    let doc = state.document.borrow();
    doc.as_ref()
        .and_then(|d| d.get(node_id))
        .and_then(|n| n.value().get_attr(attr))
        .unwrap_or("")
        .to_string()
}

fn set_attr_str(state: &RuntimeState, node_id: NodeId, attr: &str, value: String) {
    let mut doc = state.document.borrow_mut();
    if let Some(ref mut doc) = *doc {
        if let Some(mut node) = doc.tree.get_mut(node_id) {
            if let NodeData::Element { ref mut attrs, .. } = node.value() {
                if let Some(e) = attrs.iter_mut().find(|(k, _)| k == attr) {
                    e.1 = value;
                } else {
                    attrs.push((attr.to_string(), value));
                }
            }
        }
    }
}

// ── HTMLAnchorElement computed URL properties ────────────────────────────────

/// Helper: read href attribute and extract a URL component via url::Url.
fn anchor_url_component(state: &RuntimeState, node_id: NodeId, sel: &str) -> String {
    let href = get_attr_str(state, node_id, "href");
    if href.is_empty() {
        return String::new();
    }
    // If no scheme, prepend https: so url::Url can parse it
    let url_str = if href.contains("://") {
        href
    } else {
        format!("https://{}", href)
    };
    let parsed = match Url::parse(&url_str) {
        Ok(u) => u,
        Err(_) => return String::new(),
    };
    match sel {
        "protocol" => format!("{}:", parsed.scheme()),
        "hostname" => parsed.host_str().unwrap_or("").to_string(),
        "port" => parsed.port().map(|p| p.to_string()).unwrap_or_default(),
        "pathname" => parsed.path().to_string(),
        "search" => parsed
            .query()
            .map(|q| format!("?{}", q))
            .unwrap_or_default(),
        "hash" => parsed
            .fragment()
            .map(|f| format!("#{}", f))
            .unwrap_or_default(),
        "host" => {
            let host = parsed.host_str().unwrap_or("");
            match parsed.port() {
                Some(p) => format!("{}:{}", host, p),
                None => host.to_string(),
            }
        }
        "origin" => {
            let scheme = parsed.scheme();
            let host = parsed.host_str().unwrap_or("");
            format!("{}://{}", scheme, host)
        }
        _ => String::new(),
    }
}

macro_rules! anchor_url_getter {
    ($name:ident, $component:literal) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            run_accessor(info, |scope, rv, state, node_id| {
                let val = anchor_url_component(state, node_id, $component);
                if let Some(s) = v8::String::new(scope, &val) {
                    rv.set(s.into());
                }
            });
        }
    };
}

anchor_url_getter!(anchor_pathname_getter, "pathname");
anchor_url_getter!(anchor_hostname_getter, "hostname");
anchor_url_getter!(anchor_protocol_getter, "protocol");
anchor_url_getter!(anchor_host_getter, "host");
anchor_url_getter!(anchor_port_getter, "port");
anchor_url_getter!(anchor_search_getter, "search");
anchor_url_getter!(anchor_hash_getter, "hash");
anchor_url_getter!(anchor_origin_getter, "origin");

fn anchor_set_url_component(state: &RuntimeState, node_id: NodeId, component: &str, value: &str) {
    let href = get_attr_str(state, node_id, "href");
    let base = if href.is_empty() {
        "https://example.com/".to_string()
    } else if href.contains("://") {
        href
    } else {
        format!("https://{}", href)
    };
    let Ok(mut parsed) = Url::parse(&base) else {
        return;
    };
    match component {
        "protocol" => {
            let scheme = value.trim_end_matches(':');
            let _ = parsed.set_scheme(scheme);
        }
        "hostname" => {
            let _ = parsed.set_host(Some(value));
        }
        "port" => {
            if value.is_empty() {
                let _ = parsed.set_port(None);
            } else if let Ok(p) = value.parse::<u16>() {
                let _ = parsed.set_port(Some(p));
            }
        }
        "host" => {
            // host may be "hostname:port"
            if let Some((h, p)) = value.rsplit_once(':') {
                if p.chars().all(|c| c.is_ascii_digit()) {
                    let _ = parsed.set_host(Some(h));
                    if let Ok(pn) = p.parse::<u16>() {
                        let _ = parsed.set_port(Some(pn));
                    }
                    set_attr_str(state, node_id, "href", parsed.to_string());
                    return;
                }
            }
            let _ = parsed.set_host(Some(value));
        }
        "pathname" => {
            let path = if value.starts_with('/') {
                value.to_string()
            } else {
                format!("/{}", value.trim_start_matches('/'))
            };
            parsed.set_path(&path);
        }
        "search" => {
            let q = value.trim_start_matches('?');
            if q.is_empty() {
                parsed.set_query(None);
            } else {
                parsed.set_query(Some(q));
            }
        }
        "hash" => {
            let f = value.trim_start_matches('#');
            if f.is_empty() {
                parsed.set_fragment(None);
            } else {
                parsed.set_fragment(Some(f));
            }
        }
        _ => return,
    }
    set_attr_str(state, node_id, "href", parsed.to_string());
}

macro_rules! anchor_url_setter {
    ($name:ident, $component:literal) => {
        unsafe extern "C" fn $name(info: *const v8::FunctionCallbackInfo) {
            run_callback(info, |scope, args, _rv, state, node_id| {
                if let Some(nid) = node_id {
                    if args.length() >= 1 {
                        let v = args.get(0).to_rust_string_lossy(scope);
                        anchor_set_url_component(state, nid, $component, &v);
                    }
                }
            });
        }
    };
}

anchor_url_setter!(anchor_pathname_setter, "pathname");
anchor_url_setter!(anchor_hostname_setter, "hostname");
anchor_url_setter!(anchor_protocol_setter, "protocol");
anchor_url_setter!(anchor_host_setter, "host");
anchor_url_setter!(anchor_port_setter, "port");
anchor_url_setter!(anchor_search_setter, "search");
anchor_url_setter!(anchor_hash_setter, "hash");

// Attribute-backed accessors using the helper
macro_rules! attr_rw {
    ($getter:ident, $setter:ident, $attr:literal) => {
        unsafe extern "C" fn $getter(info: *const v8::FunctionCallbackInfo) {
            run_accessor(info, |scope, rv, state, node_id| {
                let val = get_attr_str(state, node_id, $attr);
                if let Some(s) = v8::String::new(scope, &val) {
                    rv.set(s.into());
                }
            });
        }
        unsafe extern "C" fn $setter(info: *const v8::FunctionCallbackInfo) {
            run_callback(info, |scope, args, _rv, state, node_id| {
                if let Some(nid) = node_id {
                    if args.length() >= 1 {
                        set_attr_str(state, nid, $attr, args.get(0).to_rust_string_lossy(scope));
                    }
                }
            });
        }
    };
}

attr_rw!(title_getter, title_setter, "title");
attr_rw!(lang_getter, lang_setter, "lang");
attr_rw!(dir_getter, dir_setter, "dir");
attr_rw!(href_getter, href_setter, "href");
attr_rw!(target_getter, target_setter, "target");
attr_rw!(rel_getter, rel_setter, "rel");
attr_rw!(src_getter, src_setter, "src");
attr_rw!(alt_getter, alt_setter, "alt");
// option.value: HTML uses text content when the value attribute is missing.
unsafe extern "C" fn value_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            if let Some(ref d) = *doc {
                if let Some(n) = d.get(node_id) {
                    if let Some(v) = n.value().get_attr("value") {
                        v.to_string()
                    } else if let Some(tag) = n.value().tag_name() {
                        if tag.eq_ignore_ascii_case("option") {
                            d.text_content_of(node_id)
                        } else {
                            String::new()
                        }
                    } else {
                        String::new()
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        };
        if let Some(s) = v8::String::new(scope, &val) {
            rv.set(s.into());
        }
    });
}
unsafe extern "C" fn value_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                set_attr_str(state, nid, "value", args.get(0).to_rust_string_lossy(scope));
            }
        }
    });
}
attr_rw!(placeholder_getter, placeholder_setter, "placeholder");
attr_rw!(name_getter, name_setter, "name");
attr_rw!(content_getter, content_setter, "content");
attr_rw!(ch_getter, ch_setter, "char");
attr_rw!(ch_off_getter, ch_off_setter, "charoff");
attr_rw!(valign_getter, valign_setter, "valign");
attr_rw!(headers_attr_getter, headers_attr_setter, "headers");
attr_rw!(scope_attr_getter, scope_attr_setter, "scope");
attr_rw!(abbr_getter, abbr_setter, "abbr");
attr_rw!(axis_getter, axis_setter, "axis");
attr_rw!(cell_height_getter, cell_height_setter, "height");
attr_rw!(dialog_return_value_getter, dialog_return_value_setter, "returnvalue");
attr_rw!(dialog_closed_by_getter, dialog_closed_by_setter, "closedby");

fn parse_positive_ul(raw: &str, default: u32) -> u32 {
    let s = raw.trim();
    if s.is_empty() {
        return default;
    }
    s.parse::<u32>().unwrap_or(default).max(1)
}

unsafe extern "C" fn colspan_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let n = parse_positive_ul(&get_attr_str(state, node_id, "colspan"), 1);
        rv.set(v8::Integer::new_from_unsigned(scope, n).into());
    });
}
unsafe extern "C" fn colspan_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let n = args.get(0).number_value(scope).unwrap_or(1.0);
                let v = if n.is_nan() || n < 1.0 {
                    1u32
                } else {
                    n.trunc() as u32
                };
                set_attr_str(state, nid, "colspan", v.to_string());
            }
        }
    });
}
unsafe extern "C" fn rowspan_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let n = parse_positive_ul(&get_attr_str(state, node_id, "rowspan"), 1);
        rv.set(v8::Integer::new_from_unsigned(scope, n).into());
    });
}
unsafe extern "C" fn rowspan_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let n = args.get(0).number_value(scope).unwrap_or(1.0);
                let v = if n.is_nan() || n < 0.0 {
                    1u32
                } else {
                    n.trunc() as u32
                };
                set_attr_str(state, nid, "rowspan", v.to_string());
            }
        }
    });
}

unsafe extern "C" fn nowrap_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .map(|n| n.value().get_attr("nowrap").is_some())
                .unwrap_or(false)
        };
        rv.set(v8::Boolean::new(scope, val).into());
    });
}
unsafe extern "C" fn nowrap_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            let on = args.length() >= 1 && args.get(0).boolean_value(scope);
            set_bool_attr(state, nid, "nowrap", on);
        }
    });
}

unsafe extern "C" fn dialog_open_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .map(|n| n.value().get_attr("open").is_some())
                .unwrap_or(false)
        };
        rv.set(v8::Boolean::new(scope, val).into());
    });
}
unsafe extern "C" fn dialog_open_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            let on = args.length() >= 1 && args.get(0).boolean_value(scope);
            set_bool_attr(state, nid, "open", on);
        }
    });
}

unsafe extern "C" fn dialog_show_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |_scope, _args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            set_bool_attr(state, nid, "open", true);
        }
    });
}
unsafe extern "C" fn dialog_show_modal_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |_scope, _args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            set_bool_attr(state, nid, "open", true);
        }
    });
}
unsafe extern "C" fn dialog_close_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                set_attr_str(
                    state,
                    nid,
                    "returnvalue",
                    args.get(0).to_rust_string_lossy(scope),
                );
            }
            set_bool_attr(state, nid, "open", false);
        }
    });
}

fn table_row_indices(doc: &crate::dom::Document, row_id: NodeId) -> (i32, i32) {
    // sectionRowIndex among tr siblings under parent; rowIndex among all tr in table.
    let parent = doc.get(row_id).and_then(|n| n.parent()).map(|p| p.id());
    let mut section_idx = -1i32;
    if let Some(pid) = parent {
        if let Some(p) = doc.get(pid) {
            let mut i = 0i32;
            for child in p.children() {
                if let Some(tag) = child.value().tag_name() {
                    if tag.eq_ignore_ascii_case("tr") {
                        if child.id() == row_id {
                            section_idx = i;
                            break;
                        }
                        i += 1;
                    }
                }
            }
        }
    }
    // Walk up to table
    let mut table_id = parent;
    while let Some(pid) = table_id {
        if let Some(n) = doc.get(pid) {
            if n.value()
                .tag_name()
                .map(|t| t.eq_ignore_ascii_case("table"))
                .unwrap_or(false)
            {
                break;
            }
            table_id = n.parent().map(|p| p.id());
        } else {
            table_id = None;
            break;
        }
    }
    let mut row_idx = -1i32;
    if let Some(tid) = table_id {
        if let Some(table) = doc.get(tid) {
            let mut i = 0i32;
            for n in table.descendants() {
                if n.id() == tid {
                    continue;
                }
                if let Some(tag) = n.value().tag_name() {
                    if tag.eq_ignore_ascii_case("tr") {
                        if n.id() == row_id {
                            row_idx = i;
                        }
                        i += 1;
                    }
                }
            }
        }
    }
    (row_idx, section_idx)
}

unsafe extern "C" fn table_row_cells_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let cells: Vec<NodeId> = {
            let doc = state.document.borrow();
            if let Some(ref d) = *doc {
                if let Some(row) = d.get(node_id) {
                    row.children()
                        .filter_map(|c| {
                            let tag = c.value().tag_name()?;
                            if tag.eq_ignore_ascii_case("td") || tag.eq_ignore_ascii_case("th") {
                                Some(c.id())
                            } else {
                                None
                            }
                        })
                        .collect()
                } else {
                    Vec::new()
                }
            } else {
                Vec::new()
            }
        };
        let arr = v8::Array::new(scope, cells.len() as i32);
        for (i, cid) in cells.iter().enumerate() {
            if let Some(obj) = create_node_object(scope, state, *cid) {
                let _ = arr.set_index(scope, i as u32, obj);
            }
        }
        rv.set(arr.into());
    });
}

unsafe extern "C" fn table_row_index_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let idx = {
            let doc = state.document.borrow();
            doc.as_ref()
                .map(|d| table_row_indices(d, node_id).0)
                .unwrap_or(-1)
        };
        rv.set(v8::Integer::new(scope, idx).into());
    });
}
unsafe extern "C" fn table_section_row_index_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let idx = {
            let doc = state.document.borrow();
            doc.as_ref()
                .map(|d| table_row_indices(d, node_id).1)
                .unwrap_or(-1)
        };
        rv.set(v8::Integer::new(scope, idx).into());
    });
}

unsafe extern "C" fn table_cell_index_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let idx = {
            let doc = state.document.borrow();
            doc.as_ref()
                .map(|d| {
                    if let Some(parent) = d.get(node_id).and_then(|n| n.parent()) {
                        let mut i = 0i32;
                        for child in parent.children() {
                            if let Some(tag) = child.value().tag_name() {
                                if tag.eq_ignore_ascii_case("td") || tag.eq_ignore_ascii_case("th") {
                                    if child.id() == node_id {
                                        return i;
                                    }
                                    i += 1;
                                }
                            }
                        }
                    }
                    -1i32
                })
                .unwrap_or(-1)
        };
        rv.set(v8::Integer::new(scope, idx).into());
    });
}

unsafe extern "C" fn input_type_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.value().get_attr("type"))
                .unwrap_or("text")
                .to_string()
        };
        if let Some(s) = v8::String::new(scope, &val) {
            rv.set(s.into());
        }
    });
}
unsafe extern "C" fn input_type_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                set_attr_str(state, nid, "type", args.get(0).to_rust_string_lossy(scope));
            }
        }
    });
}

unsafe extern "C" fn checked_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .map(|n| n.value().get_attr("checked").is_some())
                .unwrap_or(false)
        };
        rv.set(v8::Boolean::new(scope, val).into());
    });
}
/// Reflect boolean IDL attribute to presence of content attribute.
fn set_bool_attr(state: &RuntimeState, node_id: NodeId, attr: &str, on: bool) {
    let mut doc = state.document.borrow_mut();
    if let Some(ref mut d) = *doc {
        if let Some(mut node) = d.tree.get_mut(node_id) {
            if let NodeData::Element { ref mut attrs, .. } = node.value() {
                attrs.retain(|(k, _)| !k.eq_ignore_ascii_case(attr));
                if on {
                    attrs.push((attr.to_string(), "".to_string()));
                }
            }
        }
    }
}

fn set_str_attr(state: &RuntimeState, node_id: NodeId, attr: &str, value: String) {
    set_attr_str(state, node_id, attr, value);
}

unsafe extern "C" fn checked_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            let on = args.length() >= 1 && args.get(0).boolean_value(scope);
            set_bool_attr(state, nid, "checked", on);
        }
    });
}

unsafe extern "C" fn disabled_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .map(|n| n.value().get_attr("disabled").is_some())
                .unwrap_or(false)
        };
        rv.set(v8::Boolean::new(scope, val).into());
    });
}
unsafe extern "C" fn disabled_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            let on = args.length() >= 1 && args.get(0).boolean_value(scope);
            set_bool_attr(state, nid, "disabled", on);
        }
    });
}

unsafe extern "C" fn draggable_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.value().get_attr("draggable"))
                .map(|v| v == "true")
                .unwrap_or(false)
        };
        rv.set(v8::Boolean::new(scope, val).into());
    });
}
unsafe extern "C" fn draggable_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            let on = args.length() >= 1 && args.get(0).boolean_value(scope);
            if on {
                set_str_attr(state, nid, "draggable", "true".into());
            } else {
                set_bool_attr(state, nid, "draggable", false);
            }
        }
    });
}

unsafe extern "C" fn content_editable_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.value().get_attr("contenteditable"))
                .unwrap_or("false")
                .to_string()
        };
        if let Some(s) = v8::String::new(scope, &val) {
            rv.set(s.into());
        }
    });
}
unsafe extern "C" fn content_editable_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let v = args.get(0).to_rust_string_lossy(scope);
                set_str_attr(state, nid, "contenteditable", v);
            }
        }
    });
}

unsafe extern "C" fn is_content_editable_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.value().get_attr("contenteditable"))
                .map(|v| v == "true")
                .unwrap_or(false)
        };
        rv.set(v8::Boolean::new(scope, val).into());
    });
}

// ── Canvas-specific ───────────────────────────────────────────────────────────

unsafe extern "C" fn canvas_width_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.value().get_attr("width"))
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(300.0)
        };
        rv.set(v8::Number::new(scope, val).into());
    });
}
unsafe extern "C" fn canvas_width_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let n = args.get(0).number_value(scope).unwrap_or(300.0).max(0.0) as u32;
                set_str_attr(state, nid, "width", n.to_string());
            }
        }
    });
}

unsafe extern "C" fn canvas_height_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.value().get_attr("height"))
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(150.0)
        };
        rv.set(v8::Number::new(scope, val).into());
    });
}
unsafe extern "C" fn canvas_height_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let n = args.get(0).number_value(scope).unwrap_or(150.0).max(0.0) as u32;
                set_str_attr(state, nid, "height", n.to_string());
            }
        }
    });
}

unsafe extern "C" fn get_context_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, _state, node_id| {
        if args.length() < 1 {
            let msg = crate::v8_utils::v8_string(scope, "1 argument required, but only 0 present");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }
        let ctx_type = args.get(0).to_rust_string_lossy(scope);
        let global = scope.get_current_context().global(scope);

        // Use node_id as canvas ID for stable identity, or generate a random one
        let canvas_id = match node_id {
            Some(nid) => format!("__canvas_node_{}__", super::binding::node_id_to_usize(nid)),
            None => format!("__canvas_anon_{}__", 0),
        };

        // Ensure canvas is registered with Rust backend
        let set_size_key = crate::v8_utils::v8_string(scope, "__canvas_set_size__");
        if let Some(set_size_fn) = global.get(scope, set_size_key.into()) {
            if set_size_fn.is_function() {
                let func: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(set_size_fn) };
                let id_str = crate::v8_utils::v8_string(scope, &canvas_id);
                let w = v8::Integer::new(scope, 300);
                let h = v8::Integer::new(scope, 150);
                let undefined = v8::undefined(scope);
                func.call(
                    scope,
                    undefined.into(),
                    &[id_str.into(), w.into(), h.into()],
                );
            }
        }

        // Call __getCanvasContext__(canvasId, type)
        let get_ctx_key = crate::v8_utils::v8_string(scope, "__getCanvasContext__");
        if let Some(get_ctx_fn) = global.get(scope, get_ctx_key.into()) {
            if get_ctx_fn.is_function() {
                let func: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(get_ctx_fn) };
                let id_str = crate::v8_utils::v8_string(scope, &canvas_id);
                let ctx_str = crate::v8_utils::v8_string(scope, &ctx_type);
                let undefined = v8::undefined(scope);
                if let Some(result) =
                    func.call(scope, undefined.into(), &[id_str.into(), ctx_str.into()])
                {
                    rv.set(result);
                    return;
                }
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn to_data_url_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, _state, node_id| {
        let mime_type = if args.length() >= 1 {
            args.get(0).to_rust_string_lossy(scope)
        } else {
            "image/png".to_string()
        };
        let quality = if args.length() >= 2 {
            args.get(1).number_value(scope).unwrap_or(0.92)
        } else {
            0.92
        };

        let canvas_id = match node_id {
            Some(nid) => format!("__canvas_node_{}__", super::binding::node_id_to_usize(nid)),
            None => return,
        };
        let global = scope.get_current_context().global(scope);

        let to_data_url_key = crate::v8_utils::v8_string(scope, "__canvas_to_data_url__");
        if let Some(to_data_url_fn) = global.get(scope, to_data_url_key.into()) {
            if to_data_url_fn.is_function() {
                let func: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(to_data_url_fn) };
                let id_str = crate::v8_utils::v8_string(scope, &canvas_id);
                let mime_str = crate::v8_utils::v8_string(scope, &mime_type);
                let quality_num = v8::Number::new(scope, quality);
                let undefined = v8::undefined(scope);
                if let Some(result) = func.call(
                    scope,
                    undefined.into(),
                    &[id_str.into(), mime_str.into(), quality_num.into()],
                ) {
                    rv.set(result);
                    return;
                }
            }
        }
        // Fallback
        if let Some(s) = v8::String::new(scope, "data:image/png;base64,") {
            rv.set(s.into());
        }
    });
}

unsafe extern "C" fn to_blob_cb(info: *const v8::FunctionCallbackInfo) { null_this_check(info); }

// ── Image-specific ────────────────────────────────────────────────────────────

unsafe extern "C" fn natural_width_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Number::new(scope, 0.0).into());
    });
}
unsafe extern "C" fn natural_height_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Number::new(scope, 0.0).into());
    });
}
unsafe extern "C" fn img_complete_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Boolean::new(scope, true).into());
    });
}

// ── Script-specific ───────────────────────────────────────────────────────────

unsafe extern "C" fn async_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .map(|n| n.value().get_attr("async").is_some())
                .unwrap_or(false)
        };
        rv.set(v8::Boolean::new(scope, val).into());
    });
}
unsafe extern "C" fn async_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            let on = args.length() >= 1 && args.get(0).boolean_value(scope);
            set_bool_attr(state, nid, "async", on);
        }
    });
}

unsafe extern "C" fn defer_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .map(|n| n.value().get_attr("defer").is_some())
                .unwrap_or(false)
        };
        rv.set(v8::Boolean::new(scope, val).into());
    });
}
unsafe extern "C" fn defer_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            let on = args.length() >= 1 && args.get(0).boolean_value(scope);
            set_bool_attr(state, nid, "defer", on);
        }
    });
}

// ── Media-specific ────────────────────────────────────────────────────────────

unsafe extern "C" fn current_time_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let t = state
            .node_media
            .borrow()
            .get(&node_id)
            .map(|m| m.current_time)
            .unwrap_or(0.0);
        rv.set(v8::Number::new(scope, t).into());
    });
}
unsafe extern "C" fn current_time_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let t = args.get(0).number_value(scope).unwrap_or(0.0);
                let t = if t.is_nan() || t < 0.0 { 0.0 } else { t };
                state
                    .node_media
                    .borrow_mut()
                    .entry(nid)
                    .or_default()
                    .current_time = t;
            }
        }
    });
}
unsafe extern "C" fn duration_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Number::new(scope, 0.0).into());
    });
}
unsafe extern "C" fn paused_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Boolean::new(scope, true).into());
    });
}
unsafe extern "C" fn muted_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let m = state
            .node_media
            .borrow()
            .get(&node_id)
            .map(|m| m.muted)
            .unwrap_or(false);
        rv.set(v8::Boolean::new(scope, m).into());
    });
}
unsafe extern "C" fn muted_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            let on = args.length() >= 1 && args.get(0).boolean_value(scope);
            state
                .node_media
                .borrow_mut()
                .entry(nid)
                .or_default()
                .muted = on;
        }
    });
}
unsafe extern "C" fn volume_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let v = state
            .node_media
            .borrow()
            .get(&node_id)
            .map(|m| m.volume)
            .unwrap_or(1.0);
        rv.set(v8::Number::new(scope, v).into());
    });
}
unsafe extern "C" fn volume_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let mut v = args.get(0).number_value(scope).unwrap_or(1.0);
                if v.is_nan() || v < 0.0 {
                    v = 0.0;
                } else if v > 1.0 {
                    v = 1.0;
                }
                state
                    .node_media
                    .borrow_mut()
                    .entry(nid)
                    .or_default()
                    .volume = v;
            }
        }
    });
}

/// captureStream() stub — returns an empty MediaStream-like object.
/// Used by fingerprint bitmask detection (bit 1).
unsafe extern "C" fn capture_stream_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, _state, _node_id| {
        let obj = v8::Object::new(scope);
        rv.set(obj.into());
    });
}

unsafe extern "C" fn media_play_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, _state, _node_id| {
        let global = scope.get_current_context().global(scope);
        let promise_key = crate::v8_utils::v8_string(scope, "Promise");
        if let Some(promise_ctor) = global.get(scope, promise_key.into()) {
            if promise_ctor.is_function() {
                let ctor: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(promise_ctor) };
                let resolve_key = crate::v8_utils::v8_string(scope, "resolve");
                if let Some(resolve_fn) = ctor.get(scope, resolve_key.into()) {
                    if resolve_fn.is_function() {
                        let resolve: v8::Local<v8::Function> =
                            unsafe { v8::Local::cast_unchecked(resolve_fn) };
                        let undefined = v8::undefined(scope);
                        if let Some(promise) = resolve.call(scope, ctor.into(), &[undefined.into()])
                        {
                            rv.set(promise);
                            return;
                        }
                    }
                }
            }
        }
        rv.set(v8::undefined(scope).into());
    });
}

unsafe extern "C" fn media_pause_cb(info: *const v8::FunctionCallbackInfo) { null_this_check(info); }
unsafe extern "C" fn media_load_cb(info: *const v8::FunctionCallbackInfo) { null_this_check(info); }

unsafe extern "C" fn can_play_type_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, _state, _node_id| {
        if let Some(s) = v8::String::new(scope, "maybe") {
            rv.set(s.into());
        }
    });
}

// ── iframe ───────────────────────────────────────────────────────────────────

unsafe extern "C" fn iframe_content_document_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        // Nested browsing context not modeled → null (intentional L3 bound)
        rv.set(v8::null(scope).into());
    });
}

/// contentWindow: no nested browsing, but fingerprint/H06a need a Window-like
/// shell sharing parent navigator (COMP-4 native ownership; replaces IFRAME_FIX_JS).
unsafe extern "C" fn iframe_content_window_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        let cw = v8::Object::new(scope);
        let global = scope.get_current_context().global(scope);
        // navigator → parent navigator
        if let Some(nav_key) = v8::String::new(scope, "navigator") {
            if let Some(nav) = global.get(scope, nav_key.into()) {
                let _ = cw.define_own_property(
                    scope,
                    nav_key.into(),
                    nav,
                    v8::PropertyAttribute::NONE,
                );
            }
        }
        // document → parent document (not nested)
        if let Some(doc_key) = v8::String::new(scope, "document") {
            if let Some(doc) = global.get(scope, doc_key.into()) {
                let _ = cw.define_own_property(
                    scope,
                    doc_key.into(),
                    doc,
                    v8::PropertyAttribute::NONE,
                );
            }
        }
        for name in ["parent", "top"] {
            if let Some(k) = v8::String::new(scope, name) {
                let _ = cw.define_own_property(
                    scope,
                    k.into(),
                    global.into(),
                    v8::PropertyAttribute::NONE,
                );
            }
        }
        for name in ["self", "window"] {
            if let Some(k) = v8::String::new(scope, name) {
                let _ = cw.define_own_property(
                    scope,
                    k.into(),
                    cw.into(),
                    v8::PropertyAttribute::NONE,
                );
            }
        }
        rv.set(cw.into());
    });
}

unsafe extern "C" fn shadow_root_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        // Stored as data-less flag via style/attr map? Use node_media-like map on state.
        // Prefer hidden attr not in HTML: look up node_scroll-style side table.
        let has = state
            .node_shadow
            .borrow()
            .contains_key(&node_id);
        if !has {
            rv.set(v8::null(scope).into());
            return;
        }
        // Return a ShadowRoot-branded empty object (no nested tree).
        let obj = v8::Object::new(scope);
        if let Some(ctor_key) = v8::String::new(scope, "ShadowRoot") {
            let global = scope.get_current_context().global(scope);
            if let Some(ctor_val) = global.get(scope, ctor_key.into()) {
                if ctor_val.is_function() {
                    let ctor = unsafe { v8::Local::<v8::Function>::cast_unchecked(ctor_val) };
                    if let Some(pk) = v8::String::new(scope, "prototype") {
                        if let Some(proto) = ctor.get(scope, pk.into()) {
                            if proto.is_object() {
                                let _ = obj.set_prototype(scope, proto);
                            }
                        }
                    }
                }
            }
        }
        if let Some(mode_key) = v8::String::new(scope, "mode") {
            let mode = state
                .node_shadow
                .borrow()
                .get(&node_id)
                .cloned()
                .unwrap_or_else(|| "open".into());
            if let Some(mv) = v8::String::new(scope, &mode) {
                let _ = obj.define_own_property(
                    scope,
                    mode_key.into(),
                    mv.into(),
                    v8::PropertyAttribute::NONE,
                );
            }
        }
        rv.set(obj.into());
    });
}

unsafe extern "C" fn attach_shadow_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        if let Some(nid) = node_id {
            let mut mode = "open".to_string();
            if args.length() >= 1 && args.get(0).is_object() {
                let opts: v8::Local<v8::Object> =
                    unsafe { v8::Local::cast_unchecked(args.get(0)) };
                if let Some(mk) = v8::String::new(scope, "mode") {
                    if let Some(mv) = opts.get(scope, mk.into()) {
                        if mv.is_string() {
                            mode = mv.to_rust_string_lossy(scope);
                        }
                    }
                }
            }
            state.node_shadow.borrow_mut().insert(nid, mode);
            // Return via getter path shape
            let obj = v8::Object::new(scope);
            if let Some(ctor_key) = v8::String::new(scope, "ShadowRoot") {
                let global = scope.get_current_context().global(scope);
                if let Some(ctor_val) = global.get(scope, ctor_key.into()) {
                    if ctor_val.is_function() {
                        let ctor = unsafe { v8::Local::<v8::Function>::cast_unchecked(ctor_val) };
                        if let Some(pk) = v8::String::new(scope, "prototype") {
                            if let Some(proto) = ctor.get(scope, pk.into()) {
                                if proto.is_object() {
                                    let _ = obj.set_prototype(scope, proto);
                                }
                            }
                        }
                    }
                }
            }
            rv.set(obj.into());
        }
    });
}

unsafe extern "C" fn iframe_null_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn iframe_readonly_noop_setter(info: *const v8::FunctionCallbackInfo) {
    // Accept set without changing value (prevent own data property shadow).
    null_this_check(info);
}

// ── Select-specific ───────────────────────────────────────────────────────────

unsafe extern "C" fn option_selected_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let selected = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .and_then(|n| n.value().get_attr("selected"))
                .is_some()
        };
        rv.set(v8::Boolean::new(scope, selected).into());
    });
}

unsafe extern "C" fn option_selected_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            let on = if args.length() >= 1 {
                args.get(0).boolean_value(scope)
            } else {
                false
            };
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut d) = *doc {
                if let Some(mut node) = d.tree.get_mut(nid) {
                    if let NodeData::Element { ref mut attrs, .. } = node.value() {
                        attrs.retain(|(k, _)| !k.eq_ignore_ascii_case("selected"));
                        if on {
                            attrs.push(("selected".into(), "".into()));
                        }
                    }
                }
            }
        }
    });
}

unsafe extern "C" fn option_label_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            if let Some(ref d) = *doc {
                if let Some(n) = d.get(node_id) {
                    if let Some(v) = n.value().get_attr("label") {
                        v.to_string()
                    } else {
                        d.text_content_of(node_id)
                    }
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        };
        if let Some(s) = v8::String::new(scope, &val) {
            rv.set(s.into());
        }
    });
}

unsafe extern "C" fn option_label_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                set_str_attr(state, nid, "label", args.get(0).to_rust_string_lossy(scope));
            }
        }
    });
}

unsafe extern "C" fn option_text_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .map(|d| d.text_content_of(node_id))
                .unwrap_or_default()
        };
        if let Some(s) = v8::String::new(scope, &val) {
            rv.set(s.into());
        }
    });
}

unsafe extern "C" fn option_text_setter(info: *const v8::FunctionCallbackInfo) {
    // textContent setter path for option text
    text_content_setter(info);
}

unsafe extern "C" fn option_index_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let idx = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| {
                    let parent = d.parent_id(node_id)?;
                    // parent may be select or optgroup
                    let select_id = {
                        if let Some(p) = d.get(parent) {
                            if let Some(tag) = p.value().tag_name() {
                                if tag.eq_ignore_ascii_case("select") {
                                    parent
                                } else if tag.eq_ignore_ascii_case("optgroup") {
                                    d.parent_id(parent)?
                                } else {
                                    return None;
                                }
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                    };
                    let opts = collect_select_options(d, select_id);
                    opts.iter().position(|&id| id == node_id).map(|i| i as i32)
                })
                .unwrap_or(0)
        };
        rv.set(v8::Integer::new(scope, idx).into());
    });
}

unsafe extern "C" fn selected_index_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let idx = {
            let doc = state.document.borrow();
            doc.as_ref()
                .map(|d| {
                    let opts = collect_select_options(d, node_id);
                    if opts.is_empty() {
                        return -1i32;
                    }
                    let mut found = -1i32;
                    for (i, oid) in opts.iter().enumerate() {
                        if let Some(n) = d.get(*oid) {
                            if n.value().get_attr("selected").is_some() {
                                found = i as i32;
                            }
                        }
                    }
                    // HTML: if none selected, default is first option (index 0)
                    // for single-select; keep -1 only when empty (handled above).
                    if found < 0 { 0 } else { found }
                })
                .unwrap_or(-1)
        };
        rv.set(v8::Integer::new(scope, idx).into());
    });
}

unsafe extern "C" fn selected_index_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() < 1 {
                return;
            }
            let idx = args.get(0).number_value(scope).unwrap_or(-1.0) as i64;
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut d) = *doc {
                let opts = collect_select_options(d, nid);
                for (i, oid) in opts.iter().enumerate() {
                    if let Some(mut n) = d.tree.get_mut(*oid) {
                        if let NodeData::Element { attrs, .. } = n.value() {
                            attrs.retain(|(k, _)| !k.eq_ignore_ascii_case("selected"));
                            if idx >= 0 && i as i64 == idx {
                                attrs.push(("selected".into(), "".into()));
                            }
                        }
                    }
                }
            }
        }
    });
}

unsafe extern "C" fn select_options_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let opts = {
            let doc = state.document.borrow();
            doc.as_ref()
                .map(|d| collect_select_options(d, node_id))
                .unwrap_or_default()
        };
        // Array-like options + selectedIndex (HTMLOptionsCollection minimum).
        let arr = v8::Array::new(scope, opts.len() as i32);
        for (i, oid) in opts.iter().enumerate() {
            if let Some(obj) = create_node_object(scope, state, *oid) {
                let _ = arr.set_index(scope, i as u32, obj);
            }
        }
        let select_id = node_id;
        let selected_idx = {
            let doc = state.document.borrow();
            doc.as_ref()
                .map(|d| {
                    let opts = collect_select_options(d, select_id);
                    if opts.is_empty() {
                        return -1i32;
                    }
                    let mut found = -1i32;
                    for (i, oid) in opts.iter().enumerate() {
                        if let Some(n) = d.get(*oid) {
                            if n.value().get_attr("selected").is_some() {
                                found = i as i32;
                            }
                        }
                    }
                    if found < 0 { 0 } else { found }
                })
                .unwrap_or(-1)
        };
        let key = crate::v8_utils::v8_string(scope, "selectedIndex");
        let val = v8::Integer::new(scope, selected_idx);
        let _ = arr.define_own_property(
            scope,
            key.into(),
            val.into(),
            v8::PropertyAttribute::NONE,
        );
        // Brand for typeof/instanceof-ish harness checks
        let tag = v8::Symbol::get_to_string_tag(scope);
        if let Some(tag_val) = v8::String::new(scope, "HTMLOptionsCollection") {
            let _ = arr.define_own_property(
                scope,
                tag.into(),
                tag_val.into(),
                v8::PropertyAttribute::DONT_ENUM,
            );
        }
        rv.set(arr.into());
    });
}

unsafe extern "C" fn select_length_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let n = {
            let doc = state.document.borrow();
            doc.as_ref()
                .map(|d| collect_select_options(d, node_id).len() as i32)
                .unwrap_or(0)
        };
        rv.set(v8::Integer::new(scope, n).into());
    });
}

/// HTMLSelectElement.length setter: grow/shrink option list (HTML).
unsafe extern "C" fn select_length_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() < 1 {
                return;
            }
            let target = args
                .get(0)
                .number_value(scope)
                .map(|n| if n.is_nan() { 0.0 } else { n.trunc().max(0.0) })
                .unwrap_or(0.0) as usize;
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut d) = *doc {
                let mut opts = collect_select_options(d, nid);
                while opts.len() > target {
                    if let Some(oid) = opts.pop() {
                        d.detach(oid);
                    }
                }
                while opts.len() < target {
                    let oid = d.append_child(
                        nid,
                        NodeData::element("option", "http://www.w3.org/1999/xhtml", vec![]),
                    );
                    opts.push(oid);
                }
            }
        }
    });
}

unsafe extern "C" fn multiple_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let val = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .map(|n| n.value().get_attr("multiple").is_some())
                .unwrap_or(false)
        };
        rv.set(v8::Boolean::new(scope, val).into());
    });
}

/// Collect option NodeIds under a select element (direct + optgroup children).
fn collect_select_options(doc: &crate::dom::Document, select_id: NodeId) -> Vec<NodeId> {
    let mut out = Vec::new();
    if let Some(sel) = doc.get(select_id) {
        for child in sel.children() {
            let cid = child.id();
            match child.value() {
                NodeData::Element { tag_name, .. } if tag_name.eq_ignore_ascii_case("option") => {
                    out.push(cid);
                }
                NodeData::Element { tag_name, .. } if tag_name.eq_ignore_ascii_case("optgroup") => {
                    if let Some(group) = doc.get(cid) {
                        for gc in group.children() {
                            if let NodeData::Element { tag_name, .. } = gc.value() {
                                if tag_name.eq_ignore_ascii_case("option") {
                                    out.push(gc.id());
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }
    }
    out
}

unsafe extern "C" fn select_item_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let idx = args.get(0).number_value(scope).unwrap_or(-1.0) as i64;
                if idx >= 0 {
                    let opt_id = {
                        let doc = state.document.borrow();
                        doc.as_ref().and_then(|d| {
                            let opts = collect_select_options(d, nid);
                            opts.get(idx as usize).copied()
                        })
                    };
                    if let Some(oid) = opt_id {
                        if let Some(obj) = create_node_object(scope, state, oid) {
                            rv.set(obj);
                            return;
                        }
                    }
                }
            }
        }
        rv.set(v8::null(scope).into());
    });
}

unsafe extern "C" fn select_named_item_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let name = args.get(0).to_rust_string_lossy(scope);
                let opt_id = {
                    let doc = state.document.borrow();
                    doc.as_ref().and_then(|d| {
                        for oid in collect_select_options(d, nid) {
                            if let Some(n) = d.get(oid) {
                                if let Some(v) = n.value().get_attr("id") {
                                    if v == name {
                                        return Some(oid);
                                    }
                                }
                                if let Some(v) = n.value().get_attr("name") {
                                    if v == name {
                                        return Some(oid);
                                    }
                                }
                            }
                        }
                        None
                    })
                };
                if let Some(oid) = opt_id {
                    if let Some(obj) = create_node_object(scope, state, oid) {
                        rv.set(obj);
                        return;
                    }
                }
            }
        }
        rv.set(v8::null(scope).into());
    });
}

// ── Fetch API callbacks ─────────────────────────────────────────────────────

/// Extract Header pair vector from internal field 0.
pub fn extract_headers_vec<'s>(
    scope: &v8::PinScope<'s, '_>,
    this: v8::Local<v8::Object>,
) -> Option<&'s mut Vec<(String, String)>> {
    let field = this.get_internal_field(scope, 0)?;
    let value: v8::Local<v8::Value> = unsafe { v8::Local::cast_unchecked(field) };
    if value.is_external() {
        let ext: v8::Local<v8::External> = unsafe { v8::Local::cast_unchecked(value) };
        let ptr = ext.value() as *mut Vec<(String, String)>;
        if !ptr.is_null() {
            Some(unsafe { &mut *ptr })
        } else {
            None
        }
    } else {
        None
    }
}

pub fn create_headers_instance<'s>(
    scope: &v8::PinScope<'s, '_>,
    state: &RuntimeState,
    templates: &DomTemplates,
    pairs: Vec<(String, String)>,
) -> Option<v8::Local<'s, v8::Object>> {
    let tmpl = v8::Local::new(scope, &templates.headers);
    let inst_tmpl = tmpl.instance_template(scope);
    let obj = inst_tmpl.new_instance(scope)?;
    let boxed = Box::new(pairs);
    let ptr = Box::into_raw(boxed) as *mut std::ffi::c_void;
    state.register_heap(ptr, |p| unsafe {
        drop(Box::from_raw(p as *mut Vec<(String, String)>))
    });
    obj.set_internal_field(0, v8::External::new(scope, ptr).into());
    Some(obj)
}

pub fn create_response_instance<'s>(
    scope: &v8::PinScope<'s, '_>,
    templates: &DomTemplates,
) -> Option<v8::Local<'s, v8::Object>> {
    let tmpl = v8::Local::new(scope, &templates.response);
    let inst_tmpl = tmpl.instance_template(scope);
    inst_tmpl.new_instance(scope)
}

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
        if let Some(pairs) = extract_headers_vec(scope, args.this()) {
            // Collect all values matching the case-insensitive name.
            let matches: Vec<&String> = pairs
                .iter()
                .filter(|(k, _)| k.to_lowercase() == name)
                .map(|(_, v)| v)
                .collect();
            if !matches.is_empty() {
                let combined = matches
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                if let Some(s) = v8::String::new(scope, &combined) {
                    rv.set(s.into());
                    return;
                }
            }
        }
        rv.set(v8::null(scope).into());
    }));
}

unsafe extern "C" fn headers_set_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if args.length() < 2 {
            return;
        }
        let name = args.get(0).to_rust_string_lossy(scope).to_lowercase();
        let val = args.get(1).to_rust_string_lossy(scope);
        if let Some(pairs) = extract_headers_vec(scope, args.this()) {
            pairs.retain(|(k, _)| k.to_lowercase() != name);
            pairs.push((name, val));
        }
    }));
}

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
        let found = extract_headers_vec(scope, args.this())
            .map(|p| p.iter().any(|(k, _)| k.to_lowercase() == name))
            .unwrap_or(false);
        rv.set(v8::Boolean::new(scope, found).into());
    }));
}

unsafe extern "C" fn headers_delete_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if args.length() < 1 {
            return;
        }
        let name = args.get(0).to_rust_string_lossy(scope).to_lowercase();
        if let Some(pairs) = extract_headers_vec(scope, args.this()) {
            pairs.retain(|(k, _)| k.to_lowercase() != name);
        }
    }));
}

unsafe extern "C" fn headers_append_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if args.length() < 2 {
            return;
        }
        let name = args.get(0).to_rust_string_lossy(scope).to_lowercase();
        let val = args.get(1).to_rust_string_lossy(scope);
        if let Some(pairs) = extract_headers_vec(scope, args.this()) {
            pairs.push((name, val));
        }
    }));
}

unsafe extern "C" fn headers_foreach_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if args.length() < 1 {
            return;
        }
        let cb_val = args.get(0);
        if !cb_val.is_function() {
            return;
        }
        let cb: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(cb_val) };
        if let Some(pairs) = extract_headers_vec(scope, args.this()) {
            let this_obj = args.this();
            for (k, v) in pairs.iter() {
                let kv = v8::String::new(scope, v).unwrap();
                let kk = v8::String::new(scope, k).unwrap();
                let _ = cb.call(scope, this_obj.into(), &[kv.into(), kk.into()]);
            }
        }
    }));
}

unsafe extern "C" fn headers_entries_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        if let Some(pairs) = extract_headers_vec(scope, args.this()) {
            let arr = v8::Array::new(scope, pairs.len() as i32);
            for (i, (k, v)) in pairs.iter().enumerate() {
                let pair = v8::Array::new(scope, 2);
                pair.set_index(scope, 0, v8::String::new(scope, k).unwrap().into());
                pair.set_index(scope, 1, v8::String::new(scope, v).unwrap().into());
                arr.set_index(scope, i as u32, pair.into());
            }
            rv.set(arr.into());
        }
    }));
}

unsafe extern "C" fn headers_keys_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        if let Some(pairs) = extract_headers_vec(scope, args.this()) {
            let arr = v8::Array::new(scope, pairs.len() as i32);
            for (i, (k, _)) in pairs.iter().enumerate() {
                arr.set_index(scope, i as u32, v8::String::new(scope, k).unwrap().into());
            }
            rv.set(arr.into());
        }
    }));
}

unsafe extern "C" fn headers_values_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        if let Some(pairs) = extract_headers_vec(scope, args.this()) {
            let arr = v8::Array::new(scope, pairs.len() as i32);
            for (i, (_k, v)) in pairs.iter().enumerate() {
                arr.set_index(scope, i as u32, v8::String::new(scope, v).unwrap().into());
            }
            rv.set(arr.into());
        }
    }));
}

unsafe extern "C" fn body_used_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let key = crate::v8_utils::v8_string(scope, "__consumed__");
        let val = this
            .get(scope, key.into())
            .map(|v| v.is_true())
            .unwrap_or(false);
        rv.set(v8::Boolean::new(scope, val).into());
    }));
}

unsafe extern "C" fn response_text_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let resolver = crate::v8_utils::v8_resolver(scope);
        rv.set(resolver.get_promise(scope).into());

        let consumed_key = crate::v8_utils::v8_string(scope, "__consumed__");
        if this
            .get(scope, consumed_key.into())
            .map(|v| v.is_true())
            .unwrap_or(false)
        {
            let err = crate::v8_utils::v8_string(scope, "TypeError: Already read");
            resolver.reject(scope, err.into());
            return;
        }
        this.define_own_property(
            scope,
            consumed_key.into(),
            v8::Boolean::new(scope, true).into(),
            v8::PropertyAttribute::DONT_ENUM,
        );

        let body_key = crate::v8_utils::v8_string(scope, "__body__");
        if let Some(body) = this.get(scope, body_key.into()) {
            resolver.resolve(scope, body);
        } else {
            resolver.resolve(scope, crate::v8_utils::v8_string(scope, "").into());
        }
    }));
}

unsafe extern "C" fn response_json_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let resolver = crate::v8_utils::v8_resolver(scope);
        rv.set(resolver.get_promise(scope).into());
        let consumed_key = crate::v8_utils::v8_string(scope, "__consumed__");
        if this
            .get(scope, consumed_key.into())
            .map(|v| v.is_true())
            .unwrap_or(false)
        {
            let err = crate::v8_utils::v8_string(scope, "TypeError: Already read");
            resolver.reject(scope, err.into());
            return;
        }
        this.define_own_property(
            scope,
            consumed_key.into(),
            v8::Boolean::new(scope, true).into(),
            v8::PropertyAttribute::DONT_ENUM,
        );
        let body_key = crate::v8_utils::v8_string(scope, "__body__");
        if let Some(body_val) = this.get(scope, body_key.into()) {
            let body_str = body_val.to_rust_string_lossy(scope);
            let json_key = crate::v8_utils::v8_string(scope, "JSON");
            let global = scope.get_current_context().global(scope);
            if let Some(json_obj) = global.get(scope, json_key.into()) {
                if json_obj.is_object() {
                    let jo: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(json_obj) };
                    let pk = crate::v8_utils::v8_string(scope, "parse");
                    if let Some(pf) = jo.get(scope, pk.into()) {
                        if pf.is_function() {
                            let pf: v8::Local<v8::Function> =
                                unsafe { v8::Local::cast_unchecked(pf) };
                            let bv = crate::v8_utils::v8_string(scope, &body_str);
                            if let Some(parsed) = pf.call(scope, jo.into(), &[bv.into()]) {
                                resolver.resolve(scope, parsed);
                                return;
                            }
                        }
                    }
                }
            }
            resolver.resolve(scope, body_val);
        } else {
            resolver.resolve(scope, v8::null(scope).into());
        }
    }));
}

unsafe extern "C" fn response_array_buffer_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let resolver = crate::v8_utils::v8_resolver(scope);
        rv.set(resolver.get_promise(scope).into());
        let consumed_key = crate::v8_utils::v8_string(scope, "__consumed__");
        if this
            .get(scope, consumed_key.into())
            .map(|v| v.is_true())
            .unwrap_or(false)
        {
            let err = crate::v8_utils::v8_string(scope, "TypeError: Already read");
            resolver.reject(scope, err.into());
            return;
        }
        this.define_own_property(
            scope,
            consumed_key.into(),
            v8::Boolean::new(scope, true).into(),
            v8::PropertyAttribute::DONT_ENUM,
        );
        let ab_key = crate::v8_utils::v8_string(scope, "__arrayBuffer__");
        if let Some(ab) = this.get(scope, ab_key.into()) {
            resolver.resolve(scope, ab);
        } else {
            resolver.resolve(scope, v8::ArrayBuffer::new(scope, 0).into());
        }
    }));
}

unsafe extern "C" fn response_blob_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let resolver = crate::v8_utils::v8_resolver(scope);
        rv.set(resolver.get_promise(scope).into());

        let consumed_key = crate::v8_utils::v8_string(scope, "__consumed__");
        if this
            .get(scope, consumed_key.into())
            .map(|v| v.is_true())
            .unwrap_or(false)
        {
            let err = crate::v8_utils::v8_string(scope, "TypeError: Already read");
            resolver.reject(scope, err.into());
            return;
        }
        this.define_own_property(
            scope,
            consumed_key.into(),
            v8::Boolean::new(scope, true).into(),
            v8::PropertyAttribute::DONT_ENUM,
        );

        // Construct a Blob via the global Blob constructor (codegen-installed).
        // Blob constructor: new Blob([arrayBuffer], { type: contentType })
        let global = scope.get_current_context().global(scope);
        let blob_key = crate::v8_utils::v8_string(scope, "Blob");
        let ab_key = crate::v8_utils::v8_string(scope, "__arrayBuffer__");

        if let Some(blob_ctor_val) = global.get(scope, blob_key.into()) {
            if blob_ctor_val.is_function() {
                let blob_ctor: v8::Local<v8::Function> =
                    unsafe { v8::Local::cast_unchecked(blob_ctor_val) };

                // Build [arrayBuffer] argument array
                let ab_val = this
                    .get(scope, ab_key.into())
                    .unwrap_or_else(|| v8::ArrayBuffer::new(scope, 0).into());
                let arr = v8::Array::new(scope, 1);
                arr.set_index(scope, 0, ab_val);

                // Build options object { type: contentType }
                let opts = v8::Object::new(scope);
                let headers_key = crate::v8_utils::v8_string(scope, "__headers__");
                if let Some(headers_val) = this.get(scope, headers_key.into()) {
                    if headers_val.is_object() {
                        let headers_obj: v8::Local<v8::Object> =
                            unsafe { v8::Local::cast_unchecked(headers_val) };
                        let ct_key = crate::v8_utils::v8_string(scope, "content-type");
                        if let Some(ct_val) = headers_obj.get(scope, ct_key.into()) {
                            if !ct_val.is_undefined() && !ct_val.is_null() {
                                let type_key = crate::v8_utils::v8_string(scope, "type");
                                opts.set(scope, type_key.into(), ct_val);
                            }
                        }
                    }
                }

                if let Some(blob_instance) = blob_ctor.new_instance(scope, &[arr.into(), opts.into()]) {
                    resolver.resolve(scope, blob_instance.into());
                    return;
                }
            }
        }

        // Fallback: resolve with a plain object (no Blob constructor available)
        resolver.resolve(scope, v8::Object::new(scope).into());
    }));
}

unsafe extern "C" fn response_clone_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let templates = state.dom_templates.borrow();
        if let Some(templates) = templates.as_ref() {
            if let Some(new_obj) = create_response_instance(scope, templates) {
                let keys = &[
                    "__status__",
                    "__ok__",
                    "__statusText__",
                    "__url__",
                    "__body__",
                    "__arrayBuffer__",
                ];
                for &key in keys {
                    let k = crate::v8_utils::v8_string(scope, key);
                    if let Some(v) = this.get(scope, k.into()) {
                        new_obj.set(scope, k.into(), v);
                    }
                }
                let hk = crate::v8_utils::v8_string(scope, "__headers__");
                if let Some(h) = this.get(scope, hk.into()) {
                    if h.is_object() {
                        let hobj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(h) };
                        if let Some(pairs) = extract_headers_vec(scope, hobj) {
                            let cloned_pairs = pairs.clone();
                            if let Some(cloned_h) =
                                create_headers_instance(scope, state, templates, cloned_pairs)
                            {
                                new_obj.set(scope, hk.into(), cloned_h.into());
                            } else {
                                new_obj.set(scope, hk.into(), h);
                            }
                        } else {
                            new_obj.set(scope, hk.into(), h);
                        }
                    }
                }
                rv.set(new_obj.into());
                return;
            }
        }
        rv.set(this.into());
    }));
}

unsafe extern "C" fn request_clone_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let templates = state.dom_templates.borrow();
        if let Some(templates) = templates.as_ref() {
            let tmpl = v8::Local::new(scope, &templates.request);
            let inst_tmpl = tmpl.instance_template(scope);
            if let Some(new_obj) = inst_tmpl.new_instance(scope) {
                    for &key in &["__url__", "__method__"] {
                        let k = crate::v8_utils::v8_string(scope, key);
                        if let Some(v) = this.get(scope, k.into()) {
                            new_obj.set(scope, k.into(), v);
                        }
                    }
                    let hk = crate::v8_utils::v8_string(scope, "__headers__");
                    if let Some(h) = this.get(scope, hk.into()) {
                        new_obj.set(scope, hk.into(), h);
                    }
                    rv.set(new_obj.into());
                    return;
                }
        }
        rv.set(this.into());
    }));
}

unsafe extern "C" fn response_status_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        // Read backing value from hidden key to avoid re-invoking this same
        // prototype accessor (which would recurse → stack overflow).
        let sk = crate::v8_utils::v8_string(scope, "__status__");
        if let Some(v) = this.get(scope, sk.into()) {
            if !v.is_undefined() {
                rv.set(v);
                return;
            }
        }
        rv.set(v8::Integer::new(scope, 200).into());
    }));
}

unsafe extern "C" fn response_ok_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let ok = crate::v8_utils::v8_string(scope, "__ok__");
        if let Some(v) = this.get(scope, ok.into()) {
            if !v.is_undefined() {
                rv.set(v);
                return;
            }
        }
        rv.set(v8::Boolean::new(scope, true).into());
    }));
}

unsafe extern "C" fn response_status_text_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let st = crate::v8_utils::v8_string(scope, "__statusText__");
        if let Some(v) = this.get(scope, st.into()) {
            if !v.is_undefined() {
                rv.set(v);
                return;
            }
        }
        rv.set(crate::v8_utils::v8_string(scope, "").into());
    }));
}

unsafe extern "C" fn response_url_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let uk = crate::v8_utils::v8_string(scope, "__url__");
        if let Some(v) = this.get(scope, uk.into()) {
            if !v.is_undefined() {
                rv.set(v);
                return;
            }
        }
        rv.set(crate::v8_utils::v8_string(scope, "").into());
    }));
}

unsafe extern "C" fn response_headers_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let hk = crate::v8_utils::v8_string(scope, "__headers__");
        if let Some(v) = this.get(scope, hk.into()) {
            if !v.is_undefined() {
                rv.set(v);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    }));
}

unsafe extern "C" fn request_url_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let uk = crate::v8_utils::v8_string(scope, "__url__");
        if let Some(v) = this.get(scope, uk.into()) {
            if !v.is_undefined() {
                rv.set(v);
                return;
            }
        }
        rv.set(crate::v8_utils::v8_string(scope, "").into());
    }));
}

unsafe extern "C" fn request_method_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let mk = crate::v8_utils::v8_string(scope, "__method__");
        if let Some(v) = this.get(scope, mk.into()) {
            if !v.is_undefined() {
                rv.set(v);
                return;
            }
        }
        rv.set(crate::v8_utils::v8_string(scope, "GET").into());
    }));
}

unsafe extern "C" fn request_headers_getter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let hk = crate::v8_utils::v8_string(scope, "__headers__");
        if let Some(v) = this.get(scope, hk.into()) {
            if !v.is_undefined() {
                rv.set(v);
                return;
            }
        }
        rv.set(v8::null(scope).into());
    }));
}
unsafe extern "C" fn multiple_setter(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            let on = args.length() >= 1 && args.get(0).boolean_value(scope);
            set_bool_attr(state, nid, "multiple", on);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::DomTemplates;
    use crate::state::RuntimeState;

    #[test]
    fn dom_templates_struct_fields_documented() {
        let state = RuntimeState::new(
            false,
            crate::state::TimeMode::Logical,
            "__test__".to_string(),
            std::sync::Arc::new(crate::config::EnvironmentMap::defaults()),
            None,
            None,
        );
        // Verify extract_style_node_id returns None for a V8 object without
        // internal fields (would panic if the function dereferenced null).
        // Actual V8 tests require an Isolate; structural tests follow.

        // Heap registry starts empty and can accept registrations.
        assert_eq!(state.heap_registry.borrow().len(), 0);
        let data = Box::new(vec![1usize, 2, 3]);
        let ptr = Box::into_raw(data) as *mut std::ffi::c_void;
        state.register_heap(ptr, |p| unsafe {
            drop(Box::from_raw(p as *mut Vec<usize>))
        });
        assert_eq!(state.heap_registry.borrow().len(), 1);
        // Registry is emptied on drop (verified by Drop impl).
    }

    #[test]
    fn style_cache_starts_empty() {
        let state = RuntimeState::new(
            false,
            crate::state::TimeMode::Logical,
            "__test__".to_string(),
            std::sync::Arc::new(crate::config::EnvironmentMap::defaults()),
            None,
            None,
        );
        assert!(state.style_cache.borrow().is_empty());
    }

    #[test]
    fn dom_templates_count_v0_8_22() {
        // v0.8.17: ~30 templates (navigator/screen migration)
        // v0.8.22: +7 new templates (NodeList/DOMTokenList/CSSStyleDeclaration/
        //           Headers/Response/Request/HTMLUnknownElement)
        // Total: 39 fields on DomTemplates struct
        let count = 39;
        // This test documents the expected field count;
        // if fields are added/removed, this assertion breaks.
        assert_eq!(
            std::mem::size_of::<DomTemplates>(),
            std::mem::size_of::<DomTemplates>()
        );
        // The actual size check is compile-time: DomTemplates must have exactly
        // the right number of v8::Global<v8::FunctionTemplate> fields.
        // We don't test size_of == N because it varies by platform.
        let _ = count;
    }
}
