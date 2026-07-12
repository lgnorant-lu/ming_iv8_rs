//! NodeData and Document — the core DOM tree types.
//!
//! ego-tree stores nodes in a Vec<Node<T>> where T is our NodeData enum.
//! Each node has parent/first_child/last_child/prev_sibling/next_sibling indices.
//! NodeId is a typed wrapper around the arena index.
#![expect(
    clippy::expect_used,
    reason = "tree.get_mut expects: node IDs validated at call sites"
)]

use std::cell::RefCell;
use std::collections::HashMap;

/// Re-export ego_tree::NodeId for convenience.
pub type NodeId = ego_tree::NodeId;

/// The five DOM node types supported by iv8-rs.
#[derive(Clone, Debug, PartialEq)]
pub enum NodeData {
    /// The document root node (nodeType = 9).
    Document,

    /// DOCTYPE declaration (nodeType = 10).
    DocumentType {
        name: String,
        public_id: String,
        system_id: String,
    },

    /// An element node (nodeType = 1).
    Element {
        /// Tag name, lowercased (e.g. "div", "script", "html").
        tag_name: String,
        /// Namespace URI (e.g. "http://www.w3.org/1999/xhtml").
        namespace: String,
        /// Attributes as (name, value) pairs, preserving order.
        attrs: Vec<(String, String)>,
        /// Cached id attribute (for O(1) getElementById).
        id: Option<String>,
        /// Cached class list (split from class attribute).
        classes: Vec<String>,
        /// Cached inline style properties (kebab-case → value).
        style_map: HashMap<String, String>,
    },

    /// A text node (nodeType = 3).
    Text(String),

    /// A comment node (nodeType = 8).
    Comment(String),

    /// A document fragment (nodeType = 11).
    /// Acts as a lightweight container for grouping nodes.
    DocumentFragment,
}

impl NodeData {
    /// Create a new Element node.
    pub fn element(tag_name: &str, namespace: &str, attrs: Vec<(String, String)>) -> Self {
        let id = attrs
            .iter()
            .find(|(k, _)| k == "id")
            .map(|(_, v)| v.clone());

        let classes = attrs
            .iter()
            .find(|(k, _)| k == "class")
            .map(|(_, v)| {
                v.split_whitespace()
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        NodeData::Element {
            tag_name: tag_name.to_ascii_lowercase(),
            namespace: namespace.to_string(),
            attrs,
            id,
            classes,
            style_map: HashMap::new(),
        }
    }

    /// Create a new Text node.
    pub fn text(content: &str) -> Self {
        NodeData::Text(content.to_string())
    }

    /// Create a new Comment node.
    pub fn comment(content: &str) -> Self {
        NodeData::Comment(content.to_string())
    }

    /// Create a new DocumentType node.
    pub fn doctype(name: &str, public_id: &str, system_id: &str) -> Self {
        NodeData::DocumentType {
            name: name.to_string(),
            public_id: public_id.to_string(),
            system_id: system_id.to_string(),
        }
    }

    /// Get the DOM nodeType number.
    pub fn node_type(&self) -> u16 {
        match self {
            NodeData::Document => 9,
            NodeData::DocumentType { .. } => 10,
            NodeData::Element { .. } => 1,
            NodeData::Text(_) => 3,
            NodeData::Comment(_) => 8,
            NodeData::DocumentFragment => 11,
        }
    }

    /// Get the DOM nodeName.
    pub fn node_name(&self) -> &str {
        match self {
            NodeData::Document => "#document",
            NodeData::DocumentType { name, .. } => name,
            NodeData::Element { tag_name, .. } => tag_name,
            NodeData::Text(_) => "#text",
            NodeData::Comment(_) => "#comment",
            NodeData::DocumentFragment => "#document-fragment",
        }
    }

    /// Check if this is an Element node.
    pub fn is_element(&self) -> bool {
        matches!(self, NodeData::Element { .. })
    }

    /// Check if this is a Text node.
    pub fn is_text(&self) -> bool {
        matches!(self, NodeData::Text(_))
    }

    /// Get the tag name (only for Element nodes).
    pub fn tag_name(&self) -> Option<&str> {
        match self {
            NodeData::Element { tag_name, .. } => Some(tag_name),
            _ => None,
        }
    }

    /// Get the id attribute (only for Element nodes).
    pub fn element_id(&self) -> Option<&str> {
        match self {
            NodeData::Element { id, .. } => id.as_deref(),
            _ => None,
        }
    }

    /// Get the class list (only for Element nodes).
    pub fn class_list(&self) -> &[String] {
        match self {
            NodeData::Element { classes, .. } => classes,
            _ => &[],
        }
    }

    /// Get attributes (only for Element nodes).
    pub fn attrs(&self) -> &[(String, String)] {
        match self {
            NodeData::Element { attrs, .. } => attrs,
            _ => &[],
        }
    }

    /// Get an attribute value by name (only for Element nodes).
    pub fn get_attr(&self, name: &str) -> Option<&str> {
        match self {
            NodeData::Element { attrs, .. } => attrs
                .iter()
                .find(|(k, _)| k == name)
                .map(|(_, v)| v.as_str()),
            _ => None,
        }
    }

    /// Get text content (for Text and Comment nodes).
    pub fn text_content(&self) -> Option<&str> {
        match self {
            NodeData::Text(s) | NodeData::Comment(s) => Some(s),
            _ => None,
        }
    }
}

/// Document ready state (mirrors document.readyState).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentReadyState {
    Loading,
    Interactive,
    Complete,
}

/// The Document — owns the ego-tree and provides DOM query/mutation APIs.
pub struct Document {
    /// The underlying tree. Root node is always NodeData::Document.
    pub(crate) tree: ego_tree::Tree<NodeData>,

    /// O(1) id → NodeId index. Updated on parse and setAttribute("id", ...).
    id_index: HashMap<String, NodeId>,

    /// Lazily-built tag name → NodeId index (invalidated on mutation).
    tag_index: RefCell<Option<HashMap<String, Vec<NodeId>>>>,

    /// Base URL for resolving relative URLs.
    base_url: Option<url::Url>,

    /// Document ready state.
    ready_state: RefCell<DocumentReadyState>,
}

impl Document {
    /// Create a new empty Document with a root Document node.
    pub fn new(base_url: Option<&str>) -> Self {
        let tree = ego_tree::Tree::new(NodeData::Document);
        let parsed_url = base_url.and_then(|u| url::Url::parse(u).ok());

        Self {
            tree,
            id_index: HashMap::new(),
            tag_index: RefCell::new(None),
            base_url: parsed_url,
            ready_state: RefCell::new(DocumentReadyState::Loading),
        }
    }

    /// Get the root NodeId (always the Document node).
    pub fn root_id(&self) -> NodeId {
        self.tree.root().id()
    }

    /// Get a node reference by NodeId.
    pub fn get(&self, id: NodeId) -> Option<ego_tree::NodeRef<'_, NodeData>> {
        self.tree.get(id)
    }

    /// Get a mutable node reference by NodeId.
    pub fn get_mut(&mut self, id: NodeId) -> Option<ego_tree::NodeMut<'_, NodeData>> {
        self.tree.get_mut(id)
    }

    /// Look up an element by id (O(1) via HashMap).
    pub fn get_element_by_id(&self, id: &str) -> Option<NodeId> {
        self.id_index.get(id).copied()
    }

    /// Get all elements with a given tag name.
    pub fn get_elements_by_tag_name(&self, tag: &str) -> Vec<NodeId> {
        let tag_lower = tag.to_ascii_lowercase();

        // Try cached index first
        {
            let cache = self.tag_index.borrow();
            if let Some(ref index) = *cache {
                return index.get(&tag_lower).cloned().unwrap_or_default();
            }
        }

        // Build the tag index
        self.rebuild_tag_index();

        let cache = self.tag_index.borrow();
        cache
            .as_ref()
            .and_then(|idx| idx.get(&tag_lower))
            .cloned()
            .unwrap_or_default()
    }

    /// Rebuild the tag name index by traversing the tree.
    fn rebuild_tag_index(&self) {
        let mut index: HashMap<String, Vec<NodeId>> = HashMap::new();
        for node_ref in self.tree.root().descendants() {
            if let NodeData::Element { ref tag_name, .. } = node_ref.value() {
                index
                    .entry(tag_name.clone())
                    .or_default()
                    .push(node_ref.id());
            }
        }
        *self.tag_index.borrow_mut() = Some(index);
    }

    /// Rebuild the id index by traversing the entire tree.
    /// Called after parsing to fix up ids that were lost during reparenting.
    pub fn rebuild_id_index(&mut self) {
        self.id_index.clear();
        for node_ref in self.tree.root().descendants() {
            if let Some(id) = node_ref.value().element_id() {
                self.id_index.insert(id.to_string(), node_ref.id());
            }
        }
    }

    /// Invalidate the tag index (call after any tree mutation).
    pub fn invalidate_tag_index(&self) {
        *self.tag_index.borrow_mut() = None;
    }

    /// Register a node's id in the index.
    pub fn register_id(&mut self, id: String, node_id: NodeId) {
        self.id_index.insert(id, node_id);
    }

    /// Unregister a node's id from the index.
    pub fn unregister_id(&mut self, id: &str) {
        self.id_index.remove(id);
    }

    /// Append a child node to a parent. Returns the new NodeId.
    pub fn append_child(&mut self, parent_id: NodeId, data: NodeData) -> NodeId {
        // Register id if it's an element with an id
        let new_id = {
            // SAFETY: parent_id is validated before append_child is called
            let mut parent = self.tree.get_mut(parent_id).expect("parent not found");
            parent.append(data).id()
        };

        // Register id in index
        if let Some(node_ref) = self.tree.get(new_id) {
            if let Some(id) = node_ref.value().element_id() {
                self.id_index.insert(id.to_string(), new_id);
            }
        }

        self.invalidate_tag_index();
        new_id
    }

    /// Prepend a child node to a parent. Returns the new NodeId.
    pub fn prepend_child(&mut self, parent_id: NodeId, data: NodeData) -> NodeId {
        let new_id = {
            // SAFETY: parent_id is validated before prepend_child is called
            let mut parent = self.tree.get_mut(parent_id).expect("parent not found");
            parent.prepend(data).id()
        };

        if let Some(node_ref) = self.tree.get(new_id) {
            if let Some(id) = node_ref.value().element_id() {
                self.id_index.insert(id.to_string(), new_id);
            }
        }

        self.invalidate_tag_index();
        new_id
    }

    /// Insert a node before a sibling. Returns the new NodeId.
    pub fn insert_before(&mut self, sibling_id: NodeId, data: NodeData) -> NodeId {
        let new_id = {
            // SAFETY: sibling_id is validated before insert_before is called
            let mut sibling = self.tree.get_mut(sibling_id).expect("sibling not found");
            sibling.insert_before(data).id()
        };

        if let Some(node_ref) = self.tree.get(new_id) {
            if let Some(id) = node_ref.value().element_id() {
                self.id_index.insert(id.to_string(), new_id);
            }
        }

        self.invalidate_tag_index();
        new_id
    }

    /// Insert a node after a sibling. Returns the new NodeId.
    pub fn insert_after(&mut self, sibling_id: NodeId, data: NodeData) -> NodeId {
        let new_id = {
            // SAFETY: sibling_id is validated before insert_after is called
            let mut sibling = self.tree.get_mut(sibling_id).expect("sibling not found");
            sibling.insert_after(data).id()
        };

        if let Some(node_ref) = self.tree.get(new_id) {
            if let Some(id) = node_ref.value().element_id() {
                self.id_index.insert(id.to_string(), new_id);
            }
        }

        self.invalidate_tag_index();
        new_id
    }

    /// Detach a node (and its subtree) from the tree.
    /// The node remains in the arena but is no longer reachable from root.
    pub fn detach(&mut self, node_id: NodeId) {
        // Unregister ids in the subtree
        if let Some(node_ref) = self.tree.get(node_id) {
            let ids_to_remove: Vec<String> = node_ref
                .descendants()
                .filter_map(|n| n.value().element_id().map(|s| s.to_string()))
                .collect();
            for id in ids_to_remove {
                self.id_index.remove(&id);
            }
        }

        if let Some(mut node) = self.tree.get_mut(node_id) {
            node.detach();
        }

        self.invalidate_tag_index();
    }

    /// Move a node (with its subtree) to be inserted before a sibling.
    pub fn move_before_sibling(&mut self, node_to_move: NodeId, sibling_id: NodeId) {
        // Collect the subtree data
        let (data, children) = self.collect_subtree_recursive(node_to_move);
        // Detach original
        self.detach(node_to_move);
        // Insert before sibling
        let new_id = self.insert_before(sibling_id, data);
        // Restore children
        self.restore_subtree_recursive(new_id, children);
        self.invalidate_tag_index();
        self.rebuild_id_index();
    }

    /// Move a node (with its subtree) to be appended to a parent.
    pub fn move_to_parent(&mut self, node_to_move: NodeId, parent_id: NodeId) {
        let (data, children) = self.collect_subtree_recursive(node_to_move);
        self.detach(node_to_move);
        let new_id = self.append_child(parent_id, data);
        self.restore_subtree_recursive(new_id, children);
        self.invalidate_tag_index();
        self.rebuild_id_index();
    }

    /// Collect a subtree as (data, children) recursively.
    fn collect_subtree_recursive(
        &self,
        node_id: NodeId,
    ) -> (NodeData, Vec<(NodeData, Vec<NodeData>)>) {
        let data = self
            .tree
            .get(node_id)
            .map(|n| n.value().clone())
            .unwrap_or(NodeData::Document);
        let children = if let Some(node_ref) = self.tree.get(node_id) {
            node_ref
                .children()
                .map(|c| {
                    let child_data = c.value().clone();
                    let grandchildren: Vec<NodeData> =
                        c.children().map(|gc| gc.value().clone()).collect();
                    (child_data, grandchildren)
                })
                .collect()
        } else {
            vec![]
        };
        (data, children)
    }

    fn restore_subtree_recursive(
        &mut self,
        parent_id: NodeId,
        children: Vec<(NodeData, Vec<NodeData>)>,
    ) {
        for (data, grandchildren) in children {
            let child_id = self.append_child(parent_id, data);
            for gc_data in grandchildren {
                self.append_child(child_id, gc_data);
            }
        }
    }

    /// Get the base URL.
    pub fn base_url(&self) -> Option<&url::Url> {
        self.base_url.as_ref()
    }

    /// Set the base URL.
    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = url::Url::parse(url).ok();
    }

    /// Get the document ready state.
    pub fn ready_state(&self) -> DocumentReadyState {
        *self.ready_state.borrow()
    }

    /// Set the document ready state.
    pub fn set_ready_state(&self, state: DocumentReadyState) {
        *self.ready_state.borrow_mut() = state;
    }

    /// Get the number of nodes in the tree.
    pub fn node_count(&self) -> usize {
        self.tree.root().descendants().count()
    }

    /// Get the <html> element (first child element of document).
    pub fn document_element(&self) -> Option<NodeId> {
        self.tree
            .root()
            .children()
            .find(|n| matches!(n.value(), NodeData::Element { tag_name, .. } if tag_name == "html"))
            .map(|n| n.id())
    }

    /// Get the <head> element.
    pub fn head(&self) -> Option<NodeId> {
        self.document_element().and_then(|html_id| {
            self.tree.get(html_id).and_then(|html| {
                html.children()
                    .find(|n| {
                        matches!(n.value(), NodeData::Element { tag_name, .. } if tag_name == "head")
                    })
                    .map(|n| n.id())
            })
        })
    }

    /// Get the <body> element.
    pub fn body(&self) -> Option<NodeId> {
        self.document_element().and_then(|html_id| {
            self.tree.get(html_id).and_then(|html| {
                html.children()
                    .find(|n| {
                        matches!(n.value(), NodeData::Element { tag_name, .. } if tag_name == "body")
                    })
                    .map(|n| n.id())
            })
        })
    }

    /// Collect text content of a node and all its descendants.
    pub fn text_content_of(&self, node_id: NodeId) -> String {
        let mut result = String::new();
        if let Some(node_ref) = self.tree.get(node_id) {
            match node_ref.value() {
                NodeData::Text(s) | NodeData::Comment(s) => return s.clone(),
                _ => {}
            }
            for descendant in node_ref.descendants() {
                if let NodeData::Text(ref text) = descendant.value() {
                    result.push_str(text);
                }
            }
        }
        result
    }
}

impl std::fmt::Debug for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Document")
            .field("node_count", &self.node_count())
            .field("base_url", &self.base_url)
            .field("ready_state", &self.ready_state())
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_document_has_root() {
        let doc = Document::new(None);
        assert_eq!(doc.node_count(), 1);
        let root = doc.tree.root();
        assert_eq!(root.value().node_type(), 9);
        assert_eq!(root.value().node_name(), "#document");
    }

    #[test]
    fn append_child_basic() {
        let mut doc = Document::new(None);
        let root = doc.root_id();

        let html_id = doc.append_child(
            root,
            NodeData::element("html", "http://www.w3.org/1999/xhtml", vec![]),
        );
        let body_id = doc.append_child(
            html_id,
            NodeData::element("body", "http://www.w3.org/1999/xhtml", vec![]),
        );
        let text_id = doc.append_child(body_id, NodeData::text("Hello, world!"));

        assert_eq!(doc.node_count(), 4); // document + html + body + text
        assert_eq!(
            doc.get(text_id).unwrap().value().text_content(),
            Some("Hello, world!")
        );
    }

    #[test]
    fn get_element_by_id() {
        let mut doc = Document::new(None);
        let root = doc.root_id();

        let div_id = doc.append_child(
            root,
            NodeData::element(
                "div",
                "http://www.w3.org/1999/xhtml",
                vec![("id".to_string(), "main".to_string())],
            ),
        );

        assert_eq!(doc.get_element_by_id("main"), Some(div_id));
        assert_eq!(doc.get_element_by_id("nonexistent"), None);
    }

    #[test]
    fn get_elements_by_tag_name() {
        let mut doc = Document::new(None);
        let root = doc.root_id();

        let html_id = doc.append_child(
            root,
            NodeData::element("html", "http://www.w3.org/1999/xhtml", vec![]),
        );
        doc.append_child(
            html_id,
            NodeData::element("div", "http://www.w3.org/1999/xhtml", vec![]),
        );
        doc.append_child(
            html_id,
            NodeData::element("div", "http://www.w3.org/1999/xhtml", vec![]),
        );
        doc.append_child(
            html_id,
            NodeData::element("span", "http://www.w3.org/1999/xhtml", vec![]),
        );

        let divs = doc.get_elements_by_tag_name("div");
        assert_eq!(divs.len(), 2);

        let spans = doc.get_elements_by_tag_name("span");
        assert_eq!(spans.len(), 1);

        let ps = doc.get_elements_by_tag_name("p");
        assert_eq!(ps.len(), 0);
    }

    #[test]
    fn detach_removes_from_tree() {
        let mut doc = Document::new(None);
        let root = doc.root_id();

        let div_id = doc.append_child(
            root,
            NodeData::element(
                "div",
                "http://www.w3.org/1999/xhtml",
                vec![("id".to_string(), "target".to_string())],
            ),
        );
        doc.append_child(div_id, NodeData::text("child text"));

        assert_eq!(doc.node_count(), 3); // document + div + text
        assert_eq!(doc.get_element_by_id("target"), Some(div_id));

        doc.detach(div_id);

        // After detach, descendants from root no longer include the detached subtree
        let root_descendants: Vec<_> = doc.tree.root().descendants().collect();
        assert_eq!(root_descendants.len(), 1); // only document root
        assert_eq!(doc.get_element_by_id("target"), None);
    }

    #[test]
    fn insert_before_and_after() {
        let mut doc = Document::new(None);
        let root = doc.root_id();

        let first = doc.append_child(
            root,
            NodeData::element("first", "http://www.w3.org/1999/xhtml", vec![]),
        );
        let _before = doc.insert_before(
            first,
            NodeData::element("before", "http://www.w3.org/1999/xhtml", vec![]),
        );
        let _after = doc.insert_after(
            first,
            NodeData::element("after", "http://www.w3.org/1999/xhtml", vec![]),
        );

        let children: Vec<_> = doc
            .tree
            .root()
            .children()
            .map(|n| n.value().node_name().to_string())
            .collect();
        assert_eq!(children, vec!["before", "first", "after"]);
    }

    #[test]
    fn prepend_child() {
        let mut doc = Document::new(None);
        let root = doc.root_id();

        doc.append_child(
            root,
            NodeData::element("second", "http://www.w3.org/1999/xhtml", vec![]),
        );
        doc.prepend_child(
            root,
            NodeData::element("first", "http://www.w3.org/1999/xhtml", vec![]),
        );

        let children: Vec<_> = doc
            .tree
            .root()
            .children()
            .map(|n| n.value().node_name().to_string())
            .collect();
        assert_eq!(children, vec!["first", "second"]);
    }

    #[test]
    fn document_element_and_body() {
        let mut doc = Document::new(None);
        let root = doc.root_id();

        let html_id = doc.append_child(
            root,
            NodeData::element("html", "http://www.w3.org/1999/xhtml", vec![]),
        );
        doc.append_child(
            html_id,
            NodeData::element("head", "http://www.w3.org/1999/xhtml", vec![]),
        );
        let body_id = doc.append_child(
            html_id,
            NodeData::element("body", "http://www.w3.org/1999/xhtml", vec![]),
        );

        assert_eq!(doc.document_element(), Some(html_id));
        assert!(doc.head().is_some());
        assert_eq!(doc.body(), Some(body_id));
    }

    #[test]
    fn text_content_of_subtree() {
        let mut doc = Document::new(None);
        let root = doc.root_id();

        let div_id = doc.append_child(
            root,
            NodeData::element("div", "http://www.w3.org/1999/xhtml", vec![]),
        );
        doc.append_child(div_id, NodeData::text("Hello "));
        let span_id = doc.append_child(
            div_id,
            NodeData::element("span", "http://www.w3.org/1999/xhtml", vec![]),
        );
        doc.append_child(span_id, NodeData::text("world"));

        assert_eq!(doc.text_content_of(div_id), "Hello world");
    }

    #[test]
    fn node_data_constructors() {
        let elem = NodeData::element(
            "DIV",
            "http://www.w3.org/1999/xhtml",
            vec![
                ("id".to_string(), "main".to_string()),
                ("class".to_string(), "foo bar".to_string()),
            ],
        );

        assert_eq!(elem.tag_name(), Some("div")); // lowercased
        assert_eq!(elem.element_id(), Some("main"));
        assert_eq!(elem.class_list(), &["foo", "bar"]);
        assert_eq!(elem.get_attr("id"), Some("main"));
        assert_eq!(elem.get_attr("class"), Some("foo bar"));
        assert_eq!(elem.node_type(), 1);
    }

    #[test]
    fn base_url_parsing() {
        let doc = Document::new(Some("https://example.com/page"));
        assert!(doc.base_url().is_some());
        assert_eq!(doc.base_url().unwrap().as_str(), "https://example.com/page");

        let doc2 = Document::new(Some("not a url"));
        assert!(doc2.base_url().is_none());
    }

    #[test]
    fn ready_state_transitions() {
        let doc = Document::new(None);
        assert_eq!(doc.ready_state(), DocumentReadyState::Loading);

        doc.set_ready_state(DocumentReadyState::Interactive);
        assert_eq!(doc.ready_state(), DocumentReadyState::Interactive);

        doc.set_ready_state(DocumentReadyState::Complete);
        assert_eq!(doc.ready_state(), DocumentReadyState::Complete);
    }

    #[test]
    fn tag_index_invalidation() {
        let mut doc = Document::new(None);
        let root = doc.root_id();

        doc.append_child(
            root,
            NodeData::element("div", "http://www.w3.org/1999/xhtml", vec![]),
        );

        // First call builds the index
        assert_eq!(doc.get_elements_by_tag_name("div").len(), 1);

        // Add another div
        doc.append_child(
            root,
            NodeData::element("div", "http://www.w3.org/1999/xhtml", vec![]),
        );

        // Index was invalidated by append_child, so it rebuilds
        assert_eq!(doc.get_elements_by_tag_name("div").len(), 2);
    }
}
