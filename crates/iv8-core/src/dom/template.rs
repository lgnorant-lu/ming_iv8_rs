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
    /// HTMLStyleElement — inherits HTMLElement.
    pub html_style_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLLinkElement — inherits HTMLElement.
    pub html_link_element: v8::Global<v8::FunctionTemplate>,
    /// HTMLMetaElement — inherits HTMLElement.
    pub html_meta_element: v8::Global<v8::FunctionTemplate>,
    /// Text node — inherits Node.
    pub text_node: v8::Global<v8::FunctionTemplate>,
    /// Comment node — inherits Node.
    pub comment_node: v8::Global<v8::FunctionTemplate>,
    /// Document node — inherits Node.
    pub document_node: v8::Global<v8::FunctionTemplate>,
}

/// Helper: create a FunctionTemplate with a class name and internal field count.
fn make_template<'s>(
    scope: &v8::PinScope<'s, '_>,
    class_name: &str,
) -> v8::Local<'s, v8::FunctionTemplate> {
    let tmpl = v8::FunctionTemplate::builder_raw(empty_constructor).build(scope);
    let name = crate::v8_utils::v8_string(scope, class_name);
    tmpl.set_class_name(name);
    // Set internal field count on the instance template
    let inst = tmpl.instance_template(scope);
    inst.set_internal_field_count(INTERNAL_FIELD_COUNT as usize);
    tmpl
}

/// Empty constructor callback — DOM nodes are not constructed from JS.
unsafe extern "C" fn empty_constructor(_info: *const v8::FunctionCallbackInfo) {
    // No-op: DOM nodes are created by Rust, not by `new HTMLElement()`.
}

/// Helper: install a native method on a prototype template.
fn install_proto_method(
    scope: &v8::PinScope<'_, '_>,
    proto: v8::Local<v8::ObjectTemplate>,
    name: &str,
    callback: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
) {
    let fn_tmpl = v8::FunctionTemplate::builder_raw(callback).build(scope);
    let name_str = crate::v8_utils::v8_string(scope, name);
    fn_tmpl.set_class_name(name_str);
    proto.set(name_str.into(), fn_tmpl.into());
}

/// Helper: install a native accessor (getter + optional setter) on a prototype template.
fn install_proto_accessor(
    scope: &v8::PinScope<'_, '_>,
    proto: v8::Local<v8::ObjectTemplate>,
    name: &str,
    getter: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
    setter: Option<unsafe extern "C" fn(*const v8::FunctionCallbackInfo)>,
) {
    let getter_tmpl = v8::FunctionTemplate::builder_raw(getter).build(scope);
    let setter_tmpl = setter.map(|s| v8::FunctionTemplate::builder_raw(s).build(scope));
    let name_str = crate::v8_utils::v8_string(scope, name);
    proto.set_accessor_property(
        name_str.into(),
        Some(getter_tmpl),
        setter_tmpl,
        v8::PropertyAttribute::DONT_DELETE,
    );
}

/// Build all DOM templates and install methods on their prototypes.
/// Must be called once per Isolate, with the isolate entered.
pub fn build_dom_templates(scope: &v8::PinScope<'_, '_>) -> DomTemplates {
    // ── 1. EventTarget ──────────────────────────────────────────────────────
    let event_target = make_template(scope, "EventTarget");
    {
        let proto = event_target.prototype_template(scope);
        install_proto_method(scope, proto, "addEventListener", add_event_listener_cb);
        install_proto_method(
            scope,
            proto,
            "removeEventListener",
            remove_event_listener_cb,
        );
        install_proto_method(scope, proto, "dispatchEvent", dispatch_event_cb);
    }

    // ── 2. Node (inherits EventTarget) ──────────────────────────────────────
    let node = make_template(scope, "Node");
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
        install_proto_method(scope, proto, "appendChild", append_child_cb);
        install_proto_method(scope, proto, "removeChild", remove_child_cb);
        install_proto_method(scope, proto, "insertBefore", insert_before_cb);
        install_proto_method(scope, proto, "cloneNode", clone_node_cb);
        install_proto_method(scope, proto, "contains", contains_cb);
        install_proto_method(scope, proto, "hasChildNodes", has_child_nodes_cb);
        install_proto_method(scope, proto, "normalize", normalize_cb);
    }

    // ── 3. Element (inherits Node) ──────────────────────────────────────────
    let element = make_template(scope, "Element");
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
        install_proto_method(scope, proto, "getAttribute", get_attribute_cb);
        install_proto_method(scope, proto, "setAttribute", set_attribute_cb);
        install_proto_method(scope, proto, "removeAttribute", remove_attribute_cb);
        install_proto_method(scope, proto, "hasAttribute", has_attribute_cb);
        install_proto_method(scope, proto, "getAttributeNames", get_attribute_names_cb);
        // DOM mutation methods
        install_proto_method(scope, proto, "replaceChild", replace_child_cb);
        install_proto_method(scope, proto, "insertBefore", insert_before_cb);
        install_proto_method(scope, proto, "insertAdjacentHTML", insert_adjacent_html_cb);
        install_proto_method(
            scope,
            proto,
            "insertAdjacentElement",
            insert_adjacent_element_cb,
        );
        install_proto_method(scope, proto, "insertAdjacentText", insert_adjacent_text_cb);
        install_proto_method(scope, proto, "cloneNode", clone_node_cb);
        install_proto_method(scope, proto, "contains", contains_cb);
        // Query methods
        install_proto_method(scope, proto, "querySelector", query_selector_cb);
        install_proto_method(scope, proto, "querySelectorAll", query_selector_all_cb);
        install_proto_method(
            scope,
            proto,
            "getElementsByTagName",
            get_elements_by_tag_name_cb,
        );
        install_proto_method(
            scope,
            proto,
            "getElementsByClassName",
            get_elements_by_class_name_cb,
        );
        install_proto_method(scope, proto, "matches", matches_cb);
        install_proto_method(scope, proto, "closest", closest_cb);
        // Geometry
        install_proto_method(
            scope,
            proto,
            "getBoundingClientRect",
            get_bounding_client_rect_cb,
        );
        install_proto_accessor(scope, proto, "offsetWidth", offset_width_getter, None);
        install_proto_accessor(scope, proto, "offsetHeight", offset_height_getter, None);
        install_proto_accessor(scope, proto, "offsetTop", offset_top_getter, None);
        install_proto_accessor(scope, proto, "offsetLeft", offset_left_getter, None);
        install_proto_accessor(scope, proto, "clientWidth", client_width_getter, None);
        install_proto_accessor(scope, proto, "clientHeight", client_height_getter, None);
        install_proto_accessor(scope, proto, "scrollWidth", scroll_width_getter, None);
        install_proto_accessor(scope, proto, "scrollHeight", scroll_height_getter, None);
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
        install_proto_method(scope, proto, "scrollIntoView", scroll_into_view_cb);
        install_proto_method(scope, proto, "getClientRects", get_client_rects_cb);
        // Event methods (also on EventTarget, but Element overrides for convenience)
        install_proto_method(scope, proto, "addEventListener", add_event_listener_cb);
        install_proto_method(
            scope,
            proto,
            "removeEventListener",
            remove_event_listener_cb,
        );
        install_proto_method(scope, proto, "dispatchEvent", dispatch_event_cb);
    }

    // ── 4. HTMLElement (inherits Element) ───────────────────────────────────
    let html_element = make_template(scope, "HTMLElement");
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
        install_proto_method(scope, proto, "focus", focus_cb);
        install_proto_method(scope, proto, "blur", blur_cb);
        install_proto_method(scope, proto, "click", click_cb);
    }

    // ── 5. Specific HTML element types ──────────────────────────────────────
    let html_div_element = make_template(scope, "HTMLDivElement");
    html_div_element.inherit(html_element);

    let html_span_element = make_template(scope, "HTMLSpanElement");
    html_span_element.inherit(html_element);

    let html_anchor_element = make_template(scope, "HTMLAnchorElement");
    html_anchor_element.inherit(html_element);
    {
        let proto = html_anchor_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "href", href_getter, Some(href_setter));
        install_proto_accessor(scope, proto, "target", target_getter, Some(target_setter));
        install_proto_accessor(scope, proto, "rel", rel_getter, Some(rel_setter));
        // Computed URL properties (read-only, parsed from href)
        install_proto_accessor(scope, proto, "pathname", anchor_pathname_getter, None);
        install_proto_accessor(scope, proto, "hostname", anchor_hostname_getter, None);
        install_proto_accessor(scope, proto, "protocol", anchor_protocol_getter, None);
        install_proto_accessor(scope, proto, "host", anchor_host_getter, None);
        install_proto_accessor(scope, proto, "port", anchor_port_getter, None);
        install_proto_accessor(scope, proto, "search", anchor_search_getter, None);
        install_proto_accessor(scope, proto, "hash", anchor_hash_getter, None);
        install_proto_accessor(scope, proto, "origin", anchor_origin_getter, None);
    }

    let html_input_element = make_template(scope, "HTMLInputElement");
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
        install_proto_method(scope, proto, "focus", focus_cb);
        install_proto_method(scope, proto, "blur", blur_cb);
        install_proto_method(scope, proto, "select", select_cb);
        install_proto_method(scope, proto, "click", click_cb);
    }

    let html_button_element = make_template(scope, "HTMLButtonElement");
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
        install_proto_method(scope, proto, "click", click_cb);
    }

    let html_form_element = make_template(scope, "HTMLFormElement");
    html_form_element.inherit(html_element);
    {
        let proto = html_form_element.prototype_template(scope);
        install_proto_method(scope, proto, "submit", submit_cb);
        install_proto_method(scope, proto, "reset", reset_cb);
        install_proto_method(scope, proto, "checkValidity", check_validity_cb);
    }

    let html_canvas_element = make_template(scope, "HTMLCanvasElement");
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
        install_proto_method(scope, proto, "getContext", get_context_cb);
        install_proto_method(scope, proto, "toDataURL", to_data_url_cb);
        install_proto_method(scope, proto, "toBlob", to_blob_cb);
        install_proto_method(scope, proto, "captureStream", capture_stream_cb);
        install_proto_method(scope, proto, "webkitCaptureStream", capture_stream_cb);
    }

    let html_script_element = make_template(scope, "HTMLScriptElement");
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

    let html_image_element = make_template(scope, "HTMLImageElement");
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
    }

    let html_video_element = make_template(scope, "HTMLVideoElement");
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
        install_proto_method(scope, proto, "play", media_play_cb);
        install_proto_method(scope, proto, "pause", media_pause_cb);
        install_proto_method(scope, proto, "load", media_load_cb);
        install_proto_method(scope, proto, "canPlayType", can_play_type_cb);
        install_proto_method(scope, proto, "captureStream", capture_stream_cb);
        install_proto_method(scope, proto, "mozCaptureStream", capture_stream_cb);
        install_proto_method(scope, proto, "webkitCaptureStream", capture_stream_cb);
    }

    let html_audio_element = make_template(scope, "HTMLAudioElement");
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
        install_proto_method(scope, proto, "play", media_play_cb);
        install_proto_method(scope, proto, "pause", media_pause_cb);
        install_proto_method(scope, proto, "load", media_load_cb);
        install_proto_method(scope, proto, "canPlayType", can_play_type_cb);
    }

    let html_select_element = make_template(scope, "HTMLSelectElement");
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
    }

    let html_textarea_element = make_template(scope, "HTMLTextAreaElement");
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
        install_proto_method(scope, proto, "select", select_cb);
    }

    let html_head_element = make_template(scope, "HTMLHeadElement");
    html_head_element.inherit(html_element);

    let html_body_element = make_template(scope, "HTMLBodyElement");
    html_body_element.inherit(html_element);

    let html_html_element = make_template(scope, "HTMLHtmlElement");
    html_html_element.inherit(html_element);

    let html_paragraph_element = make_template(scope, "HTMLParagraphElement");
    html_paragraph_element.inherit(html_element);

    let html_heading_element = make_template(scope, "HTMLHeadingElement");
    html_heading_element.inherit(html_element);

    let html_ulist_element = make_template(scope, "HTMLUListElement");
    html_ulist_element.inherit(html_element);

    let html_olist_element = make_template(scope, "HTMLOListElement");
    html_olist_element.inherit(html_element);

    let html_li_element = make_template(scope, "HTMLLIElement");
    html_li_element.inherit(html_element);

    let html_table_element = make_template(scope, "HTMLTableElement");
    html_table_element.inherit(html_element);

    let html_style_element = make_template(scope, "HTMLStyleElement");
    html_style_element.inherit(html_element);

    let html_link_element = make_template(scope, "HTMLLinkElement");
    html_link_element.inherit(html_element);
    {
        let proto = html_link_element.prototype_template(scope);
        install_proto_accessor(scope, proto, "href", href_getter, Some(href_setter));
        install_proto_accessor(scope, proto, "rel", rel_getter, Some(rel_setter));
    }

    let html_meta_element = make_template(scope, "HTMLMetaElement");
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

    // ── 6. Text node (inherits Node) ────────────────────────────────────────
    let text_node = make_template(scope, "Text");
    text_node.inherit(node);

    // ── 7. Comment node (inherits Node) ─────────────────────────────────────
    let comment_node = make_template(scope, "Comment");
    comment_node.inherit(node);

    // ── 8. Document node (inherits Node) ────────────────────────────────────
    let document_node = make_template(scope, "Document");
    document_node.inherit(node);

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
        html_style_element: v8::Global::new(scope, html_style_element),
        html_link_element: v8::Global::new(scope, html_link_element),
        html_meta_element: v8::Global::new(scope, html_meta_element),
        text_node: v8::Global::new(scope, text_node),
        comment_node: v8::Global::new(scope, comment_node),
        document_node: v8::Global::new(scope, document_node),
    }
}

/// Install all DOM constructor functions on the global object.
/// This makes `HTMLDivElement`, `HTMLElement`, etc. available in JS.
pub fn install_dom_constructors(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
    templates: &DomTemplates,
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
        ("HTMLStyleElement", &templates.html_style_element),
        ("HTMLLinkElement", &templates.html_link_element),
        ("HTMLMetaElement", &templates.html_meta_element),
        ("Text", &templates.text_node),
        ("Comment", &templates.comment_node),
    ];

    for (name, tmpl_global) in pairs {
        let tmpl = v8::Local::new(scope, *tmpl_global);
        if let Some(func) = tmpl.get_function(scope) {
            let key = crate::v8_utils::v8_string(scope, name);
            global.define_own_property(
                scope,
                key.into(),
                func.into(),
                v8::PropertyAttribute::DONT_ENUM,
            );
        }
    }
}

/// Select the correct FunctionTemplate for a given tag name.
/// Returns the most specific template available.
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
        "textarea" => &templates.html_textarea_element,
        "head" => &templates.html_head_element,
        "body" => &templates.html_body_element,
        "html" => &templates.html_html_element,
        "p" => &templates.html_paragraph_element,
        "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => &templates.html_heading_element,
        "ul" => &templates.html_ulist_element,
        "ol" => &templates.html_olist_element,
        "li" => &templates.html_li_element,
        "table" | "thead" | "tbody" | "tfoot" | "tr" | "td" | "th" => &templates.html_table_element,
        "style" => &templates.html_style_element,
        "link" => &templates.html_link_element,
        "meta" => &templates.html_meta_element,
        _ => &templates.html_element,
    };
    v8::Local::new(scope, global)
}

/// Create a V8 object for a DOM node using the appropriate template.
/// Stores the NodeId in internal field 0.
/// Uses the identity cache to return the same object for the same NodeId.
pub fn create_node_object<'s>(
    scope: &v8::PinScope<'s, '_>,
    state: &RuntimeState,
    node_id: NodeId,
) -> Option<v8::Local<'s, v8::Value>> {
    // Check identity cache first
    {
        let cache = state.node_cache.borrow();
        if let Some(global) = cache.get(&node_id) {
            return Some(v8::Local::new(scope, global).into());
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
    };

    // Instantiate from the FunctionTemplate (not instance_template directly)
    // This ensures the prototype chain is correct for instanceof checks
    let func = tmpl_local.get_function(scope)?;
    let obj = func.new_instance(scope, &[])?;

    // Store NodeId in internal field 0 as a usize via External
    let nid_usize = super::binding::node_id_to_usize(node_id);
    // We store the usize directly as a pointer value (no heap allocation needed)
    // SAFETY: we only read this back as a usize, never dereference it
    let external = v8::External::new(scope, nid_usize as *mut std::ffi::c_void);
    obj.set_internal_field(NODE_ID_FIELD as usize, external.into());

    // Cache it
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
        if let Some(node_id) = extract_node_id_from_internal(scope, this) {
            f(scope, &mut rv, state, node_id);
        }
    }));
}

/// Helper: run a DOM callback body (with args, node_id may be None).
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
        let node_id = extract_node_id_from_internal(scope, this);
        f(scope, &args, &mut rv, state, node_id);
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
        let arr = v8::Array::new(scope, child_ids.len() as i32);
        for (i, cid) in child_ids.iter().enumerate() {
            if let Some(obj) = create_node_object(scope, state, *cid) {
                arr.set_index(scope, i as u32, obj);
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

unsafe extern "C" fn class_list_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, state, node_id| {
        let classes: Vec<String> = {
            let doc = state.document.borrow();
            doc.as_ref()
                .and_then(|d| d.get(node_id))
                .map(|n| n.value().class_list().to_vec())
                .unwrap_or_default()
        };
        let obj = v8::Object::new(scope);
        let len_key = crate::v8_utils::v8_string(scope, "length");
        obj.set(
            scope,
            len_key.into(),
            v8::Integer::new(scope, classes.len() as i32).into(),
        );
        // Store nodeId for mutation methods
        let nid_key = crate::v8_utils::v8_string(scope, "__nodeId__");
        let nid_val = v8::Number::new(scope, super::binding::node_id_to_usize(node_id) as f64);
        obj.define_own_property(
            scope,
            nid_key.into(),
            nid_val.into(),
            v8::PropertyAttribute::DONT_ENUM,
        );
        for (name, cb) in &[
            (
                "item",
                class_list_item_cb as unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
            ),
            ("contains", class_list_contains_cb),
            ("add", class_list_add_cb),
            ("remove", class_list_remove_cb),
            ("toggle", class_list_toggle_cb),
            ("toString", class_list_tostring_cb),
        ] {
            let fn_tmpl = v8::FunctionTemplate::builder_raw(*cb).build(scope);
            let fn_obj = crate::v8_utils::v8_fn(scope, &fn_tmpl);
            let key = crate::v8_utils::v8_string(scope, name);
            obj.set(scope, key.into(), fn_obj.into());
        }
        rv.set(obj.into());
    });
}

// classList helpers
unsafe extern "C" fn class_list_item_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let nid_key = crate::v8_utils::v8_string(scope, "__nodeId__");
        if let Some(nid_val) = this.get(scope, nid_key.into()) {
            if nid_val.is_number() {
                let nid_usize = nid_val.number_value(scope).unwrap_or(0.0) as usize;
                if let Some(node_id) = super::binding::usize_to_node_id(nid_usize) {
                    let idx = if args.length() >= 1 {
                        args.get(0).number_value(scope).unwrap_or(-1.0) as i32
                    } else {
                        -1
                    };
                    let isolate: &v8::Isolate = &*scope;
                    let state = RuntimeState::get(isolate);
                    let doc = state.document.borrow();
                    if let Some(ref doc) = *doc {
                        if let Some(node_ref) = doc.get(node_id) {
                            let classes = node_ref.value().class_list();
                            if idx >= 0 && (idx as usize) < classes.len() {
                                if let Some(s) = v8::String::new(scope, &classes[idx as usize]) {
                                    rv.set(s.into());
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

unsafe extern "C" fn class_list_contains_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        if args.length() < 1 {
            rv.set(v8::Boolean::new(scope, false).into());
            return;
        }
        let cls = args.get(0).to_rust_string_lossy(scope);
        let this = args.this();
        let nid_key = crate::v8_utils::v8_string(scope, "__nodeId__");
        let mut found = false;
        if let Some(nid_val) = this.get(scope, nid_key.into()) {
            if nid_val.is_number() {
                let nid_usize = nid_val.number_value(scope).unwrap_or(0.0) as usize;
                if let Some(node_id) = super::binding::usize_to_node_id(nid_usize) {
                    let isolate: &v8::Isolate = &*scope;
                    let state = RuntimeState::get(isolate);
                    let doc = state.document.borrow();
                    if let Some(ref doc) = *doc {
                        if let Some(node_ref) = doc.get(node_id) {
                            found = node_ref.value().class_list().iter().any(|c| c == &cls);
                        }
                    }
                }
            }
        }
        rv.set(v8::Boolean::new(scope, found).into());
    }));
}

unsafe extern "C" fn class_list_add_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if args.length() < 1 {
            return;
        }
        let cls = args.get(0).to_rust_string_lossy(scope);
        let this = args.this();
        let nid_key = crate::v8_utils::v8_string(scope, "__nodeId__");
        if let Some(nid_val) = this.get(scope, nid_key.into()) {
            if nid_val.is_number() {
                let nid_usize = nid_val.number_value(scope).unwrap_or(0.0) as usize;
                if let Some(node_id) = super::binding::usize_to_node_id(nid_usize) {
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
                                if !classes.contains(&cls) {
                                    classes.push(cls.clone());
                                    let new_class = classes.join(" ");
                                    if let Some(e) = attrs.iter_mut().find(|(k, _)| k == "class") {
                                        e.1 = new_class;
                                    } else {
                                        attrs.push(("class".to_string(), new_class));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }));
}

unsafe extern "C" fn class_list_remove_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if args.length() < 1 {
            return;
        }
        let cls = args.get(0).to_rust_string_lossy(scope);
        let this = args.this();
        let nid_key = crate::v8_utils::v8_string(scope, "__nodeId__");
        if let Some(nid_val) = this.get(scope, nid_key.into()) {
            if nid_val.is_number() {
                let nid_usize = nid_val.number_value(scope).unwrap_or(0.0) as usize;
                if let Some(node_id) = super::binding::usize_to_node_id(nid_usize) {
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
                                classes.retain(|c| c != &cls);
                                let new_class = classes.join(" ");
                                if let Some(e) = attrs.iter_mut().find(|(k, _)| k == "class") {
                                    e.1 = new_class;
                                }
                            }
                        }
                    }
                }
            }
        }
    }));
}

unsafe extern "C" fn class_list_toggle_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        if args.length() < 1 {
            rv.set(v8::Boolean::new(scope, false).into());
            return;
        }
        let cls = args.get(0).to_rust_string_lossy(scope);
        let this = args.this();
        let nid_key = crate::v8_utils::v8_string(scope, "__nodeId__");
        let mut result = false;
        if let Some(nid_val) = this.get(scope, nid_key.into()) {
            if nid_val.is_number() {
                let nid_usize = nid_val.number_value(scope).unwrap_or(0.0) as usize;
                if let Some(node_id) = super::binding::usize_to_node_id(nid_usize) {
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
                                if classes.contains(&cls) {
                                    classes.retain(|c| c != &cls);
                                    result = false;
                                } else {
                                    classes.push(cls.clone());
                                    result = true;
                                }
                                let new_class = classes.join(" ");
                                if let Some(e) = attrs.iter_mut().find(|(k, _)| k == "class") {
                                    e.1 = new_class;
                                } else {
                                    attrs.push(("class".to_string(), new_class));
                                }
                            }
                        }
                    }
                }
            }
        }
        rv.set(v8::Boolean::new(scope, result).into());
    }));
}

unsafe extern "C" fn class_list_tostring_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        let this = args.this();
        let nid_key = crate::v8_utils::v8_string(scope, "__nodeId__");
        if let Some(nid_val) = this.get(scope, nid_key.into()) {
            if nid_val.is_number() {
                let nid_usize = nid_val.number_value(scope).unwrap_or(0.0) as usize;
                if let Some(node_id) = super::binding::usize_to_node_id(nid_usize) {
                    let isolate: &v8::Isolate = &*scope;
                    let state = RuntimeState::get(isolate);
                    let doc = state.document.borrow();
                    if let Some(ref doc) = *doc {
                        if let Some(node_ref) = doc.get(node_id) {
                            let cls = node_ref.value().class_list().join(" ");
                            if let Some(s) = v8::String::new(scope, &cls) {
                                rv.set(s.into());
                                return;
                            }
                        }
                    }
                }
            }
        }
        rv.set(crate::v8_utils::v8_string(scope, "").into());
    }));
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
            return;
        }
        let new_arg = args.get(0);
        let old_arg = args.get(1);
        if !new_arg.is_object() || !old_arg.is_object() {
            return;
        }
        let new_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(new_arg) };
        let old_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(old_arg) };
        let new_id = extract_node_id_from_internal(scope, new_obj);
        let old_id = extract_node_id_from_internal(scope, old_obj);
        if let (Some(nid), Some(oid)) = (new_id, old_id) {
            // Edge case: replaceChild(node, node) — replacing a node with itself.
            // Real browsers treat this as a no-op. Return the old child reference.
            if nid == oid {
                rv.set(old_arg);
                return;
            }
            let mut doc = state.document.borrow_mut();
            if let Some(ref mut doc) = *doc {
                // Detach new node from wherever it currently is
                doc.detach(nid);
                // Insert new node before old node
                if let Some(mut old_node) = doc.tree.get_mut(oid) {
                    old_node.insert_id_before(nid);
                }
                // Remove old node
                doc.detach(oid);
                doc.invalidate_tag_index();
                doc.rebuild_id_index();
            }
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
        let arr = v8::Array::new(scope, ids.len() as i32);
        for (i, id) in ids.iter().enumerate() {
            if let Some(obj) = create_node_object(scope, state, *id) {
                arr.set_index(scope, i as u32, obj);
            }
        }
        rv.set(arr.into());
    });
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
        let arr = v8::Array::new(scope, ids.len() as i32);
        for (i, id) in ids.iter().enumerate() {
            if let Some(obj) = create_node_object(scope, state, *id) {
                arr.set_index(scope, i as u32, obj);
            }
        }
        rv.set(arr.into());
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
        let arr = v8::Array::new(scope, ids.len() as i32);
        for (i, id) in ids.iter().enumerate() {
            if let Some(obj) = create_node_object(scope, state, *id) {
                arr.set_index(scope, i as u32, obj);
            }
        }
        rv.set(arr.into());
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
                            doc.detach(child_id);
                            if let Some(mut parent) = doc.tree.get_mut(parent_id) {
                                parent.append_id(child_id);
                            }
                            doc.invalidate_tag_index();
                            // Rebuild id index to pick up id attributes in appended subtree
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

unsafe extern "C" fn normalize_cb(_info: *const v8::FunctionCallbackInfo) {}

// ── Geometry ──────────────────────────────────────────────────────────────────

unsafe extern "C" fn get_bounding_client_rect_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, _state, _node_id| {
        let obj = v8::Object::new(scope);
        for key in &[
            "x", "y", "width", "height", "top", "left", "bottom", "right",
        ] {
            let k = crate::v8_utils::v8_string(scope, key);
            obj.set(scope, k.into(), v8::Number::new(scope, 0.0).into());
        }
        rv.set(obj.into());
    });
}

unsafe extern "C" fn get_client_rects_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, _state, _node_id| {
        rv.set(v8::Array::new(scope, 0).into());
    });
}

unsafe extern "C" fn scroll_into_view_cb(_info: *const v8::FunctionCallbackInfo) {}

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
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Number::new(scope, 0.0).into());
    });
}
unsafe extern "C" fn scroll_top_setter(_info: *const v8::FunctionCallbackInfo) {}
unsafe extern "C" fn scroll_left_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Number::new(scope, 0.0).into());
    });
}
unsafe extern "C" fn scroll_left_setter(_info: *const v8::FunctionCallbackInfo) {}
unsafe extern "C" fn offset_parent_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::null(scope).into());
    });
}

// ── EventTarget methods ───────────────────────────────────────────────────────

unsafe extern "C" fn add_event_listener_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
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
        }
    });
}

unsafe extern "C" fn remove_event_listener_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
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
        }
    });
}

unsafe extern "C" fn dispatch_event_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, state, node_id| {
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

        if let Some(nid) = node_id {
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
    });
}

// ── HTMLElement methods ───────────────────────────────────────────────────────

unsafe extern "C" fn focus_cb(_info: *const v8::FunctionCallbackInfo) {}
unsafe extern "C" fn blur_cb(_info: *const v8::FunctionCallbackInfo) {}
unsafe extern "C" fn click_cb(_info: *const v8::FunctionCallbackInfo) {}
unsafe extern "C" fn select_cb(_info: *const v8::FunctionCallbackInfo) {}
unsafe extern "C" fn submit_cb(_info: *const v8::FunctionCallbackInfo) {}
unsafe extern "C" fn reset_cb(_info: *const v8::FunctionCallbackInfo) {}
unsafe extern "C" fn check_validity_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, _state, _node_id| {
        rv.set(v8::Boolean::new(scope, true).into());
    });
}

// ── HTMLElement accessors ─────────────────────────────────────────────────────

unsafe extern "C" fn style_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _state, _node_id| {
        let obj = v8::Object::new(scope);
        for (name, cb) in &[
            (
                "setProperty",
                style_set_property_cb as unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
            ),
            ("getPropertyValue", style_get_property_cb),
            ("removeProperty", style_remove_property_cb),
        ] {
            let fn_tmpl = v8::FunctionTemplate::builder_raw(*cb).build(scope);
            let fn_obj = crate::v8_utils::v8_fn(scope, &fn_tmpl);
            let key = crate::v8_utils::v8_string(scope, name);
            obj.set(scope, key.into(), fn_obj.into());
        }
        rv.set(obj.into());
    });
}

unsafe extern "C" fn style_set_property_cb(_info: *const v8::FunctionCallbackInfo) {}
unsafe extern "C" fn style_remove_property_cb(_info: *const v8::FunctionCallbackInfo) {}
unsafe extern "C" fn style_get_property_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
        rv.set(crate::v8_utils::v8_string(scope, "").into());
    }));
}

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
    run_callback(info, |_scope, args, _rv, state, node_id| {
        if let Some(nid) = node_id {
            if args.length() >= 1 {
                let hidden = args.get(0).is_true();
                let mut doc = state.document.borrow_mut();
                if let Some(ref mut doc) = *doc {
                    if let Some(mut node) = doc.tree.get_mut(nid) {
                        if let NodeData::Element { ref mut attrs, .. } = node.value() {
                            if hidden {
                                if !attrs.iter().any(|(k, _)| k == "hidden") {
                                    attrs.push(("hidden".to_string(), "".to_string()));
                                }
                            } else {
                                attrs.retain(|(k, _)| k != "hidden");
                            }
                        }
                    }
                }
            }
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
unsafe extern "C" fn tab_index_setter(_info: *const v8::FunctionCallbackInfo) {}

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
attr_rw!(value_getter, value_setter, "value");
attr_rw!(placeholder_getter, placeholder_setter, "placeholder");
attr_rw!(name_getter, name_setter, "name");
attr_rw!(content_getter, content_setter, "content");

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
unsafe extern "C" fn checked_setter(_info: *const v8::FunctionCallbackInfo) {}

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
unsafe extern "C" fn disabled_setter(_info: *const v8::FunctionCallbackInfo) {}

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
unsafe extern "C" fn draggable_setter(_info: *const v8::FunctionCallbackInfo) {}

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
unsafe extern "C" fn content_editable_setter(_info: *const v8::FunctionCallbackInfo) {}

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
unsafe extern "C" fn canvas_width_setter(_info: *const v8::FunctionCallbackInfo) {}

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
unsafe extern "C" fn canvas_height_setter(_info: *const v8::FunctionCallbackInfo) {}

unsafe extern "C" fn get_context_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, args, rv, _state, node_id| {
        let ctx_type = if args.length() >= 1 {
            args.get(0).to_rust_string_lossy(scope)
        } else {
            "2d".to_string()
        };
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

unsafe extern "C" fn to_blob_cb(_info: *const v8::FunctionCallbackInfo) {}

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
unsafe extern "C" fn async_setter(_info: *const v8::FunctionCallbackInfo) {}

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
unsafe extern "C" fn defer_setter(_info: *const v8::FunctionCallbackInfo) {}

// ── Media-specific ────────────────────────────────────────────────────────────

unsafe extern "C" fn current_time_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Number::new(scope, 0.0).into());
    });
}
unsafe extern "C" fn current_time_setter(_info: *const v8::FunctionCallbackInfo) {}
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
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Boolean::new(scope, false).into());
    });
}
unsafe extern "C" fn muted_setter(_info: *const v8::FunctionCallbackInfo) {}
unsafe extern "C" fn volume_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Number::new(scope, 1.0).into());
    });
}
unsafe extern "C" fn volume_setter(_info: *const v8::FunctionCallbackInfo) {}

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

unsafe extern "C" fn media_pause_cb(_info: *const v8::FunctionCallbackInfo) {}
unsafe extern "C" fn media_load_cb(_info: *const v8::FunctionCallbackInfo) {}

unsafe extern "C" fn can_play_type_cb(info: *const v8::FunctionCallbackInfo) {
    run_callback(info, |scope, _args, rv, _state, _node_id| {
        if let Some(s) = v8::String::new(scope, "maybe") {
            rv.set(s.into());
        }
    });
}

// ── Select-specific ───────────────────────────────────────────────────────────

unsafe extern "C" fn selected_index_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Integer::new(scope, -1).into());
    });
}
unsafe extern "C" fn selected_index_setter(_info: *const v8::FunctionCallbackInfo) {}
unsafe extern "C" fn multiple_getter(info: *const v8::FunctionCallbackInfo) {
    run_accessor(info, |scope, rv, _, _| {
        rv.set(v8::Boolean::new(scope, false).into());
    });
}
unsafe extern "C" fn multiple_setter(_info: *const v8::FunctionCallbackInfo) {}
