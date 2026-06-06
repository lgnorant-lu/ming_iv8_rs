//! DOM HTML serialization: innerHTML / outerHTML generation.
//!
//! Converts a DOM subtree back to an HTML string.

use super::node::{Document, NodeData, NodeId};

impl Document {
    /// Serialize the inner HTML of a node (children only, not the node itself).
    pub fn inner_html(&self, node_id: NodeId) -> String {
        let mut output = String::new();
        if let Some(node_ref) = self.tree.get(node_id) {
            for child in node_ref.children() {
                serialize_node(self, child.id(), &mut output);
            }
        }
        output
    }

    /// Serialize the outer HTML of a node (including the node itself).
    pub fn outer_html(&self, node_id: NodeId) -> String {
        let mut output = String::new();
        serialize_node(self, node_id, &mut output);
        output
    }
}

/// Serialize a single node and its descendants to HTML.
fn serialize_node(doc: &Document, node_id: NodeId, output: &mut String) {
    let node_ref = match doc.get(node_id) {
        Some(n) => n,
        None => return,
    };

    match node_ref.value() {
        NodeData::Document => {
            // Serialize all children
            for child in node_ref.children() {
                serialize_node(doc, child.id(), output);
            }
        }
        NodeData::DocumentType {
            name,
            public_id,
            system_id,
        } => {
            output.push_str("<!DOCTYPE ");
            output.push_str(name);
            if !public_id.is_empty() {
                output.push_str(" PUBLIC \"");
                output.push_str(public_id);
                output.push('"');
                if !system_id.is_empty() {
                    output.push_str(" \"");
                    output.push_str(system_id);
                    output.push('"');
                }
            } else if !system_id.is_empty() {
                output.push_str(" SYSTEM \"");
                output.push_str(system_id);
                output.push('"');
            }
            output.push('>');
        }
        NodeData::Element {
            tag_name, attrs, ..
        } => {
            // Opening tag
            output.push('<');
            output.push_str(tag_name);
            for (name, value) in attrs {
                output.push(' ');
                output.push_str(name);
                output.push_str("=\"");
                output.push_str(&escape_attr(value));
                output.push('"');
            }
            output.push('>');

            // Void elements don't have closing tags
            if !is_void_element(tag_name) {
                // Children
                for child in node_ref.children() {
                    serialize_node(doc, child.id(), output);
                }
                // Closing tag
                output.push_str("</");
                output.push_str(tag_name);
                output.push('>');
            }
        }
        NodeData::Text(text) => {
            output.push_str(&escape_text(text));
        }
        NodeData::Comment(text) => {
            output.push_str("<!--");
            output.push_str(text);
            output.push_str("-->");
        }
    }
}

/// Free function: serialize inner HTML of a node (for use from template.rs).
pub fn serialize_inner_html(doc: &Document, node_id: NodeId) -> String {
    doc.inner_html(node_id)
}

/// Free function: serialize outer HTML of a node (for use from template.rs).
pub fn serialize_outer_html(doc: &Document, node_id: NodeId) -> String {
    doc.outer_html(node_id)
}

/// Escape HTML text content.
fn escape_text(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Escape HTML attribute value.
fn escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Check if a tag is a void element (no closing tag).
fn is_void_element(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dom::parse_html;

    #[test]
    fn inner_html_simple() {
        let doc = parse_html("<div id='x'><p>hello</p></div>", None);
        let div_id = doc.get_element_by_id("x").unwrap();
        assert_eq!(doc.inner_html(div_id), "<p>hello</p>");
    }

    #[test]
    fn outer_html_simple() {
        let doc = parse_html("<div id='x'><p>hello</p></div>", None);
        let div_id = doc.get_element_by_id("x").unwrap();
        assert_eq!(doc.outer_html(div_id), "<div id=\"x\"><p>hello</p></div>");
    }

    #[test]
    fn inner_html_with_attributes() {
        let doc = parse_html(
            "<a id='link' href='https://x.com' class='btn'>click</a>",
            None,
        );
        let body = doc.body().unwrap();
        let html = doc.inner_html(body);
        assert!(html.contains("href=\"https://x.com\""));
        assert!(html.contains("class=\"btn\""));
        assert!(html.contains("click"));
    }

    #[test]
    fn inner_html_escapes_text() {
        let doc = parse_html("<div id='x'></div>", None);
        // Manually add text with special chars
        let div_id = doc.get_element_by_id("x").unwrap();
        // The text "<script>" should be escaped in innerHTML
        // Since we can't easily add raw text with < via parse, test via outer_html
        let doc2 = parse_html("<div id='y'>&lt;script&gt;</div>", None);
        let y_id = doc2.get_element_by_id("y").unwrap();
        let html = doc2.inner_html(y_id);
        assert!(html.contains("&lt;script&gt;"), "html: {}", html);
    }

    #[test]
    fn inner_html_void_elements() {
        let doc = parse_html("<div id='x'><br><img src='a.png'></div>", None);
        let div_id = doc.get_element_by_id("x").unwrap();
        let html = doc.inner_html(div_id);
        assert!(html.contains("<br>"), "html: {}", html);
        assert!(html.contains("<img"), "html: {}", html);
        // Void elements should NOT have closing tags
        assert!(!html.contains("</br>"), "html: {}", html);
        assert!(!html.contains("</img>"), "html: {}", html);
    }

    #[test]
    fn inner_html_nested() {
        let doc = parse_html("<div id='x'><ul><li>1</li><li>2</li></ul></div>", None);
        let div_id = doc.get_element_by_id("x").unwrap();
        assert_eq!(doc.inner_html(div_id), "<ul><li>1</li><li>2</li></ul>");
    }

    #[test]
    fn inner_html_comment() {
        let doc = parse_html("<div id='x'><!-- comment --><p>text</p></div>", None);
        let div_id = doc.get_element_by_id("x").unwrap();
        let html = doc.inner_html(div_id);
        assert!(html.contains("<!-- comment -->"), "html: {}", html);
        assert!(html.contains("<p>text</p>"), "html: {}", html);
    }
}
