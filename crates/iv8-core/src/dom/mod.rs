//! DOM tree implementation based on ego-tree.
//!
//! Provides a mutable DOM tree with five node types:
//! Document, DocumentType, Element, Text, Comment.
//!
//! Uses ego-tree (Vec-backed arena) for O(1) node access and
//! full mutation support (append, prepend, insert_before, insert_after, detach).

pub mod binding;
pub mod cookie_jar;
pub mod local_storage;
pub mod node;
pub mod parser;
pub mod selector;
pub mod serialize;
pub mod template;

pub use node::{Document, NodeData, NodeId};
pub use parser::{
    parse_html, parse_html_with_script_pauses, stream_write_active, StreamFeedResult,
    StreamingHtmlParser,
};
pub use selector::Selector;

/// Layer C LC-1: shared ownership handle for the live document tree.
/// Product path still stores `Option<Document>` in RuntimeState; migration to
/// `Option<DocRc>` is tracked in `docs/todo/TODO-layer-c.md`.
pub type DocRc = std::rc::Rc<std::cell::RefCell<Document>>;

/// Create a new shared document handle (Layer C scaffold).
pub fn doc_rc_new(doc: Document) -> DocRc {
    std::rc::Rc::new(std::cell::RefCell::new(doc))
}
