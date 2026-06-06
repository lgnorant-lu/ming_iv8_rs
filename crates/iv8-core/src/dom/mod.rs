//! DOM tree implementation based on ego-tree.
//!
//! Provides a mutable DOM tree with five node types:
//! Document, DocumentType, Element, Text, Comment.
//!
//! Uses ego-tree (Vec-backed arena) for O(1) node access and
//! full mutation support (append, prepend, insert_before, insert_after, detach).

pub mod binding;
pub mod navigation;
pub mod node;
pub mod parser;
pub mod selector;
pub mod serialize;
pub mod template;

pub use node::{Document, NodeData, NodeId};
pub use parser::parse_html;
pub use selector::Selector;
