//! HTML parser: html5ever TreeSink implementation for ego-tree.
//!
//! Parses HTML strings into our Document/ego-tree DOM structure.
// SAFETY: tree.get_mut/.get expects guarded by parser invariants
#![expect(clippy::expect_used, reason = "parser invariants: node IDs are valid")]

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;

use html5ever::tendril::{StrTendril, TendrilSink};
use html5ever::tree_builder::{ElementFlags, NodeOrText, QuirksMode, TreeSink};
use html5ever::{parse_document, Attribute, ParseOpts, QualName};

use super::node::{Document, NodeData, NodeId};

/// Parse an HTML string into a Document.
pub fn parse_html(html: &str, base_url: Option<&str>) -> Document {
    let sink = EgoTreeSink::new(base_url);
    let parser = parse_document(sink, ParseOpts::default());
    parser.one(html)
}

/// Streaming HTML parse session (tokenizer + tree builder).
///
/// html5ever returns `TokenizerResult::Script` when a classic `</script>` is
/// seen so the host can run the script and `document.write` into the **input
/// stream** (`push_front`) before more tokens are produced.
pub struct StreamingHtmlParser {
    tokenizer:
        html5ever::tokenizer::Tokenizer<html5ever::tree_builder::TreeBuilder<NodeId, EgoTreeSink>>,
    input_buffer: html5ever::buffer_queue::BufferQueue,
    finished: bool,
}

/// Outcome of feeding into the streaming parser.
#[derive(Debug)]
pub enum StreamFeedResult {
    /// Need more input (or idle between scripts).
    NeedMore,
    /// A classic `<script>` finished; run it, optional write, then [`StreamingHtmlParser::resume`].
    Script(NodeId),
    /// Document complete after [`StreamingHtmlParser::finish`].
    Done,
}

impl StreamingHtmlParser {
    pub fn new(base_url: Option<&str>) -> Self {
        let sink = EgoTreeSink::new(base_url);
        let tb = html5ever::tree_builder::TreeBuilder::new(
            sink,
            html5ever::tree_builder::TreeBuilderOpts::default(),
        );
        let tok =
            html5ever::tokenizer::Tokenizer::new(tb, html5ever::tokenizer::TokenizerOpts::default());
        Self {
            tokenizer: tok,
            input_buffer: html5ever::buffer_queue::BufferQueue::default(),
            finished: false,
        }
    }

    /// Push a UTF-8 HTML chunk. May return [`StreamFeedResult::Script`].
    pub fn feed(&mut self, chunk: &str) -> StreamFeedResult {
        if self.finished {
            return StreamFeedResult::Done;
        }
        if !chunk.is_empty() {
            self.input_buffer
                .push_back(html5ever::tendril::StrTendril::from(chunk));
        }
        match self.tokenizer.feed(&self.input_buffer) {
            html5ever::TokenizerResult::Script(node) => StreamFeedResult::Script(node),
            // html5ever 0.39+: encoding label from meta; UTF-8 host logs + continues.
            html5ever::TokenizerResult::EncodingIndicator(label) => {
                let s = label.to_string();
                crate::telemetry::html_encoding_indicator(&s);
                StreamFeedResult::NeedMore
            }
            html5ever::TokenizerResult::Done => StreamFeedResult::NeedMore,
        }
    }

    /// Continue after handling a Script pause (and any document.write injection).
    pub fn resume(&mut self) -> StreamFeedResult {
        self.feed("")
    }

    /// Inject markup at the **current insertion point** (document.write during parse).
    /// Uses `BufferQueue::push_front` so written HTML is tokenized before remaining input.
    pub fn write_at_insertion_point(&mut self, html: &str) -> StreamFeedResult {
        if !html.is_empty() {
            self.input_buffer
                .push_front(html5ever::tendril::StrTendril::from(html));
        }
        self.feed("")
    }

    /// Finish parsing and take the Document.
    pub fn finish(mut self) -> Document {
        loop {
            match self.tokenizer.feed(&self.input_buffer) {
                html5ever::TokenizerResult::Script(_) => {
                    // Unhandled script pause: skip (host should have run scripts).
                    continue;
                }
                html5ever::TokenizerResult::EncodingIndicator(_) => continue,
                html5ever::TokenizerResult::Done => break,
            }
        }
        self.tokenizer.end();
        self.finished = true;
        self.tokenizer.sink.sink.finish()
    }

    /// In-progress document borrow.
    pub fn document(&self) -> std::cell::Ref<'_, Document> {
        self.tokenizer.sink.sink.doc.borrow()
    }

    pub fn document_mut(&self) -> std::cell::RefMut<'_, Document> {
        self.tokenizer.sink.sink.doc.borrow_mut()
    }

    /// Swap the in-progress document with `other` (for publishing to RuntimeState
    /// during script pauses without cloning the tree).
    pub fn swap_document(&self, other: &mut Document) {
        std::mem::swap(&mut *self.document_mut(), other);
    }
}

thread_local! {
    /// Active streaming parser for document.write re-entry (Layer C).
    /// Set only for the duration of a streaming page_load on this thread.
    static ACTIVE_STREAM: std::cell::RefCell<Option<*mut StreamingHtmlParser>> =
        const { std::cell::RefCell::new(None) };
}

/// Install / clear the thread-local stream pointer for write re-entry.
///
/// # Safety
/// `ptr` must remain valid until [`clear_active_stream`] is called.
pub unsafe fn set_active_stream(ptr: *mut StreamingHtmlParser) {
    ACTIVE_STREAM.with(|c| *c.borrow_mut() = Some(ptr));
}

pub fn clear_active_stream() {
    ACTIVE_STREAM.with(|c| *c.borrow_mut() = None);
}

/// document.write during streaming parse: inject into tokenizer input (push_front).
/// Returns true if a stream was active and injection was attempted.
pub fn stream_write_active(html: &str) -> bool {
    ACTIVE_STREAM.with(|c| {
        let mut slot = c.borrow_mut();
        let Some(ptr) = *slot else {
            return false;
        };
        // SAFETY: set_active_stream guarantees ptr is valid for this thread.
        let stream = unsafe { &mut *ptr };
        let mut r = stream.write_at_insertion_point(html);
        // Drain nested script pauses without host JS (written scripts run after finish
        // or via subsequent Script pauses when host loops).
        let mut guard = 0;
        while let StreamFeedResult::Script(_) = r {
            guard += 1;
            if guard > 64 {
                break;
            }
            r = stream.resume();
        }
        true
    })
}

/// Parse HTML with script-pause callbacks (tokenizer re-entry host).
///
/// `on_script(doc, script_node_id)` runs when a classic script is closed; it may
/// call into JS that uses document.write, which should inject via the returned
/// write channel — for offline page_load the host re-runs write through the
/// existing DOCUMENT_WRITE_SHIM after partial tree is published.
pub fn parse_html_with_script_pauses<F>(html: &str, base_url: Option<&str>, mut on_script: F) -> Document
where
    F: FnMut(&Document, NodeId),
{
    let mut stream = StreamingHtmlParser::new(base_url);
    let mut pending = html;
    loop {
        // Feed remaining input in one shot; Script pauses break the feed.
        let result = if !pending.is_empty() {
            let r = stream.feed(pending);
            pending = "";
            r
        } else {
            stream.resume()
        };
        match result {
            StreamFeedResult::Script(nid) => {
                // Publish indices so getElementById works during script.
                {
                    let mut doc = stream.document_mut();
                    doc.invalidate_tag_index();
                    doc.rebuild_id_index();
                }
                on_script(&stream.document(), nid);
                // Continue tokenizer after script (and any write_at_insertion_point calls).
            }
            StreamFeedResult::NeedMore => {
                if pending.is_empty() {
                    break;
                }
            }
            StreamFeedResult::Done => break,
        }
    }
    stream.finish()
}

/// Parse an HTML fragment string and return a flat list of NodeData.
/// Used by innerHTML setter to replace children.
/// This is a simplified implementation that parses the fragment as a full document
/// and extracts the body children.
pub fn parse_html_fragment(html: &str, _context_node: NodeId) -> Vec<NodeData> {
    // Parse as a full document and extract body children
    let doc = parse_html(&format!("<body>{}</body>", html), None);
    let mut result = Vec::new();
    if let Some(body_id) = doc.body() {
        if let Some(body_ref) = doc.get(body_id) {
            for child in body_ref.children() {
                collect_node_data(&doc, child.id(), &mut result);
            }
        }
    }
    result
}

/// Recursively collect NodeData from a subtree (for fragment parsing).
fn collect_node_data(doc: &Document, node_id: NodeId, result: &mut Vec<NodeData>) {
    if let Some(node_ref) = doc.get(node_id) {
        result.push(node_ref.value().clone());
    }
}

/// TreeSink implementation that builds an ego-tree based Document.
///
/// Uses RefCell for interior mutability since TreeSink methods take &self.
struct EgoTreeSink {
    /// The document being built.
    doc: RefCell<Document>,
    /// Template contents: template element NodeId → fragment NodeId.
    template_contents: RefCell<HashMap<NodeId, NodeId>>,
    /// MathML annotation-xml integration point flags.
    mathml_flags: RefCell<HashMap<NodeId, bool>>,
}

impl EgoTreeSink {
    fn new(base_url: Option<&str>) -> Self {
        Self {
            doc: RefCell::new(Document::new(base_url)),
            template_contents: RefCell::new(HashMap::new()),
            mathml_flags: RefCell::new(HashMap::new()),
        }
    }

    /// Append a node or merge text with previous sibling.
    fn append_common(&self, parent_id: NodeId, child: NodeOrText<NodeId>) {
        match child {
            NodeOrText::AppendNode(node_id) => {
                // Reparent: detach from old parent, append to new parent
                let mut doc = self.doc.borrow_mut();
                doc.detach(node_id);
                // Re-append by moving the subtree
                // ego-tree doesn't support moving nodes directly, so we need
                // to use the low-level tree API
                let mut parent = doc.tree.get_mut(parent_id).expect("parent");
                // Append the orphaned node
                parent.append_id(node_id);
                doc.invalidate_tag_index();
            }
            NodeOrText::AppendText(text) => {
                let mut doc = self.doc.borrow_mut();
                // Try to merge with last child if it's a text node
                let should_merge = doc
                    .tree
                    .get(parent_id)
                    .and_then(|p| p.last_child())
                    .map(|last| last.value().is_text())
                    .unwrap_or(false);

                if should_merge {
                    if let Some(last_child_id) = doc
                        .tree
                        .get(parent_id)
                        .and_then(|parent| parent.last_child())
                        .map(|child| child.id())
                    {
                        if let Some(mut node) = doc.tree.get_mut(last_child_id) {
                            if let NodeData::Text(ref mut existing) = node.value() {
                                existing.push_str(&text);
                            }
                        }
                    }
                } else {
                    let text_str: &str = &text;
                    let mut parent = doc.tree.get_mut(parent_id).expect("parent");
                    parent.append(NodeData::text(text_str));
                }
            }
        }
    }

    /// Insert before a sibling, merging text if needed.
    fn insert_before_common(&self, sibling_id: NodeId, child: NodeOrText<NodeId>) {
        match child {
            NodeOrText::AppendNode(node_id) => {
                let mut doc = self.doc.borrow_mut();
                doc.detach(node_id);
                let mut sibling = doc.tree.get_mut(sibling_id).expect("sibling");
                sibling.insert_id_before(node_id);
                doc.invalidate_tag_index();
            }
            NodeOrText::AppendText(text) => {
                let mut doc = self.doc.borrow_mut();
                // Try to merge with previous sibling if it's text
                let prev_is_text = doc
                    .tree
                    .get(sibling_id)
                    .and_then(|s| s.prev_sibling())
                    .map(|prev| prev.value().is_text())
                    .unwrap_or(false);

                if prev_is_text {
                    if let Some(prev_id) = doc
                        .tree
                        .get(sibling_id)
                        .and_then(|sibling| sibling.prev_sibling())
                        .map(|prev| prev.id())
                    {
                        if let Some(mut node) = doc.tree.get_mut(prev_id) {
                            if let NodeData::Text(ref mut existing) = node.value() {
                                existing.push_str(&text);
                            }
                        }
                    }
                } else {
                    let text_str: &str = &text;
                    let mut sibling = doc.tree.get_mut(sibling_id).expect("sibling");
                    sibling.insert_before(NodeData::text(text_str));
                }
            }
        }
    }
}

impl TreeSink for EgoTreeSink {
    type Handle = NodeId;
    type Output = Document;
    type ElemName<'a> = ExpandedNameRef;

    fn finish(self) -> Document {
        let mut doc = self.doc.into_inner();
        // Rebuild the id index after parsing is complete
        // (ids get lost during the create-as-orphan-then-reparent dance)
        doc.rebuild_id_index();
        doc
    }

    fn parse_error(&self, _msg: Cow<'static, str>) {
        // Silently ignore parse errors (matching browser behavior)
    }

    fn get_document(&self) -> NodeId {
        self.doc.borrow().root_id()
    }

    fn elem_name<'a>(&'a self, target: &'a NodeId) -> Self::ElemName<'a> {
        // We store the QualName info in a side table since we can't return
        // a reference into the RefCell. Use a wrapper that implements ElemName.
        let doc = self.doc.borrow();
        let node = doc.tree.get(*target).expect("node exists");
        match node.value() {
            NodeData::Element {
                tag_name,
                namespace,
                ..
            } => ExpandedNameRef {
                tag_name: tag_name.clone(),
                namespace: namespace.clone(),
            },
            _ => ExpandedNameRef {
                tag_name: String::new(),
                namespace: String::new(),
            },
        }
    }

    fn create_element(&self, name: QualName, attrs: Vec<Attribute>, flags: ElementFlags) -> NodeId {
        let tag_name = name.local.to_string();
        let namespace = name.ns.to_string();
        let attr_vec: Vec<(String, String)> = attrs
            .iter()
            .map(|a| (a.name.local.to_string(), a.value.to_string()))
            .collect();

        let node_data = NodeData::element(&tag_name, &namespace, attr_vec);

        let mut doc = self.doc.borrow_mut();
        // Create as orphan (append to root temporarily, then detach)
        let root_id = doc.root_id();
        let node_id = doc.append_child(root_id, node_data);
        // Immediately detach — the tree builder will place it correctly
        doc.detach(node_id);

        // Handle template elements
        if flags.template {
            let fragment_id = doc.append_child(root_id, NodeData::Document);
            doc.detach(fragment_id);
            self.template_contents
                .borrow_mut()
                .insert(node_id, fragment_id);
        }

        // Store mathml flag
        if flags.mathml_annotation_xml_integration_point {
            self.mathml_flags.borrow_mut().insert(node_id, true);
        }

        node_id
    }

    fn create_comment(&self, text: StrTendril) -> NodeId {
        let mut doc = self.doc.borrow_mut();
        let root_id = doc.root_id();
        let node_id = doc.append_child(root_id, NodeData::comment(&text));
        doc.detach(node_id);
        node_id
    }

    fn create_pi(&self, target: StrTendril, data: StrTendril) -> NodeId {
        // Processing instructions are treated as comments in HTML
        let content = format!("?{} {}", target, data);
        let mut doc = self.doc.borrow_mut();
        let root_id = doc.root_id();
        let node_id = doc.append_child(root_id, NodeData::comment(&content));
        doc.detach(node_id);
        node_id
    }

    fn append(&self, parent: &NodeId, child: NodeOrText<NodeId>) {
        self.append_common(*parent, child);
    }

    fn append_before_sibling(&self, sibling: &NodeId, child: NodeOrText<NodeId>) {
        self.insert_before_common(*sibling, child);
    }

    fn append_based_on_parent_node(
        &self,
        element: &NodeId,
        prev_element: &NodeId,
        child: NodeOrText<NodeId>,
    ) {
        let has_parent = {
            let doc = self.doc.borrow();
            doc.tree.get(*element).and_then(|n| n.parent()).is_some()
        };

        if has_parent {
            self.append_before_sibling(element, child);
        } else {
            self.append_common(*prev_element, child);
        }
    }

    fn append_doctype_to_document(
        &self,
        name: StrTendril,
        public_id: StrTendril,
        system_id: StrTendril,
    ) {
        let mut doc = self.doc.borrow_mut();
        let root_id = doc.root_id();
        let mut root = doc.tree.get_mut(root_id).expect("root");
        root.append(NodeData::doctype(&name, &public_id, &system_id));
    }

    fn get_template_contents(&self, target: &NodeId) -> NodeId {
        self.template_contents
            .borrow()
            .get(target)
            .copied()
            .expect("template contents not found")
    }

    fn same_node(&self, x: &NodeId, y: &NodeId) -> bool {
        *x == *y
    }

    fn set_quirks_mode(&self, _mode: QuirksMode) {
        // We don't track quirks mode for now
    }

    fn add_attrs_if_missing(&self, target: &NodeId, attrs: Vec<Attribute>) {
        let mut doc = self.doc.borrow_mut();
        let mut node = doc.tree.get_mut(*target).expect("node");
        if let NodeData::Element {
            attrs: ref mut existing_attrs,
            ref mut id,
            ref mut classes,
            ..
        } = node.value()
        {
            for attr in attrs {
                let name = attr.name.local.to_string();
                if !existing_attrs.iter().any(|(k, _)| k == &name) {
                    let value = attr.value.to_string();
                    // Update cached id/classes if needed
                    if name == "id" {
                        *id = Some(value.clone());
                    }
                    if name == "class" {
                        *classes = value.split_whitespace().map(|s| s.to_string()).collect();
                    }
                    existing_attrs.push((name, value));
                }
            }
        }
    }

    fn remove_from_parent(&self, target: &NodeId) {
        let mut doc = self.doc.borrow_mut();
        doc.detach(*target);
    }

    fn reparent_children(&self, node: &NodeId, new_parent: &NodeId) {
        let mut doc = self.doc.borrow_mut();
        // Collect children first to avoid borrow issues
        let children: Vec<NodeId> = doc
            .tree
            .get(*node)
            .map(|n| n.children().map(|c| c.id()).collect())
            .unwrap_or_default();

        for child_id in children {
            doc.detach(child_id);
            let mut parent = doc.tree.get_mut(*new_parent).expect("new_parent");
            parent.append_id(child_id);
        }
        doc.invalidate_tag_index();
    }

    fn is_mathml_annotation_xml_integration_point(&self, handle: &NodeId) -> bool {
        self.mathml_flags
            .borrow()
            .get(handle)
            .copied()
            .unwrap_or(false)
    }
}

/// Wrapper for elem_name that implements the ElemName trait.
/// Owns the data since we can't return references into RefCell.
#[derive(Debug)]
pub struct ExpandedNameRef {
    tag_name: String,
    namespace: String,
}

impl html5ever::interface::ElemName for ExpandedNameRef {
    fn ns(&self) -> &html5ever::Namespace {
        // This is a bit of a hack — we need to return a reference to a Namespace
        // but we only have a String. We'll use a static for the common case.
        // For html5ever's purposes, it only checks equality.
        static HTML_NS: std::sync::LazyLock<html5ever::Namespace> =
            std::sync::LazyLock::new(|| html5ever::ns!(html));
        static MATHML_NS: std::sync::LazyLock<html5ever::Namespace> =
            std::sync::LazyLock::new(|| html5ever::ns!(mathml));
        static SVG_NS: std::sync::LazyLock<html5ever::Namespace> =
            std::sync::LazyLock::new(|| html5ever::ns!(svg));

        if self.namespace == "http://www.w3.org/1999/xhtml" {
            &HTML_NS
        } else if self.namespace == "http://www.w3.org/1998/Math/MathML" {
            &MATHML_NS
        } else if self.namespace == "http://www.w3.org/2000/svg" {
            &SVG_NS
        } else {
            &HTML_NS
        }
    }

    fn local_name(&self) -> &html5ever::LocalName {
        // Same issue — we need a &LocalName but have a String.
        // We'll leak a small amount of memory for uncommon tag names.
        // In practice, html5ever only calls this during parsing for a finite set of tags.
        // A better approach would be to store QualName directly, but that changes NodeData.
        // For now, use a thread-local cache.
        thread_local! {
            static CACHE: RefCell<HashMap<String, &'static html5ever::LocalName>> =
                RefCell::new(HashMap::new());
        }

        CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            if let Some(name) = cache.get(&self.tag_name) {
                return *name;
            }
            let local: html5ever::LocalName = self.tag_name.as_str().into();
            let leaked: &'static html5ever::LocalName = Box::leak(Box::new(local));
            cache.insert(self.tag_name.clone(), leaked);
            leaked
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_html() {
        let doc = parse_html(
            "<html><head><title>Test</title></head><body><p>Hello</p></body></html>",
            None,
        );
        assert!(doc.document_element().is_some());
        assert!(doc.head().is_some());
        assert!(doc.body().is_some());
    }

    #[test]
    fn parse_extracts_elements() {
        let doc = parse_html(
            "<div id=\"main\"><span class=\"foo bar\">text</span></div>",
            None,
        );

        // Should have the div with id
        let main_id = doc.get_element_by_id("main");
        assert!(main_id.is_some(), "should find element with id='main'");

        // Check span
        let spans = doc.get_elements_by_tag_name("span");
        assert_eq!(spans.len(), 1);

        let span_ref = doc.get(spans[0]).unwrap();
        assert_eq!(span_ref.value().class_list(), &["foo", "bar"]);
    }

    #[test]
    fn parse_text_content() {
        let doc = parse_html("<body><p>Hello <b>world</b></p></body>", None);
        let body_id = doc.body().unwrap();
        let text = doc.text_content_of(body_id);
        assert_eq!(text, "Hello world");
    }

    #[test]
    fn parse_comment() {
        let doc = parse_html("<body><!-- a comment --><p>text</p></body>", None);
        let body_id = doc.body().unwrap();
        let body_ref = doc.get(body_id).unwrap();

        let has_comment = body_ref
            .children()
            .any(|c| matches!(c.value(), NodeData::Comment(s) if s.contains("a comment")));
        assert!(has_comment, "should have a comment node");
    }

    #[test]
    fn parse_doctype() {
        let doc = parse_html("<!DOCTYPE html><html><body></body></html>", None);
        let root = doc.tree.root();

        let has_doctype = root
            .children()
            .any(|c| matches!(c.value(), NodeData::DocumentType { name, .. } if name == "html"));
        assert!(has_doctype, "should have DOCTYPE node");
    }

    #[test]
    fn parse_attributes() {
        let doc = parse_html(
            "<a href=\"https://example.com\" target=\"_blank\">link</a>",
            None,
        );
        let links = doc.get_elements_by_tag_name("a");
        assert_eq!(links.len(), 1);

        let link_ref = doc.get(links[0]).unwrap();
        assert_eq!(
            link_ref.value().get_attr("href"),
            Some("https://example.com")
        );
        assert_eq!(link_ref.value().get_attr("target"), Some("_blank"));
    }

    #[test]
    fn parse_multiple_elements_same_tag() {
        let doc = parse_html("<ul><li>1</li><li>2</li><li>3</li></ul>", None);
        let lis = doc.get_elements_by_tag_name("li");
        assert_eq!(lis.len(), 3);
    }

    #[test]
    fn parse_nested_structure() {
        let doc = parse_html("<div><div><div><span>deep</span></div></div></div>", None);
        let spans = doc.get_elements_by_tag_name("span");
        assert_eq!(spans.len(), 1);
        assert_eq!(doc.text_content_of(spans[0]), "deep");
    }

    #[test]
    fn parse_empty_document() {
        let doc = parse_html("", None);
        // html5ever always creates html/head/body even for empty input
        assert!(doc.document_element().is_some());
    }

    #[test]
    fn parse_script_tag() {
        let doc = parse_html("<body><script>var x = 1;</script></body>", None);
        let scripts = doc.get_elements_by_tag_name("script");
        assert_eq!(scripts.len(), 1);
        assert_eq!(doc.text_content_of(scripts[0]), "var x = 1;");
    }

    #[test]
    fn parse_with_base_url() {
        let doc = parse_html("<html></html>", Some("https://example.com/page"));
        assert_eq!(doc.base_url().unwrap().as_str(), "https://example.com/page");
    }

    #[test]
    fn streaming_script_pause_and_write_front() {
        // Feed HTML that ends a script; host injects markup via push_front.
        let mut stream = StreamingHtmlParser::new(None);
        let html = "<!DOCTYPE html><html><body><script>/*host runs*/</script><p id='after'>A</p></body></html>";
        let mut saw_script = false;
        let mut r = stream.feed(html);
        loop {
            match r {
                StreamFeedResult::Script(nid) => {
                    saw_script = true;
                    // document.write during parse → front of input stream
                    r = stream.write_at_insertion_point("<div id='mid'>M</div>");
                    let _ = nid;
                }
                StreamFeedResult::NeedMore => break,
                StreamFeedResult::Done => break,
            }
        }
        assert!(saw_script, "tokenizer should pause on script");
        let doc = stream.finish();
        assert!(
            doc.get_element_by_id("mid").is_some(),
            "write_at_insertion_point should insert mid"
        );
        assert!(
            doc.get_element_by_id("after").is_some(),
            "rest of document should still parse"
        );
    }

    #[test]
    fn parse_html_with_script_pauses_invokes_callback() {
        let mut count = 0;
        let doc = parse_html_with_script_pauses(
            "<body><script>1</script><script>2</script></body>",
            None,
            |_d, _nid| {
                count += 1;
            },
        );
        assert_eq!(count, 2);
        assert_eq!(doc.get_elements_by_tag_name("script").len(), 2);
    }
}
