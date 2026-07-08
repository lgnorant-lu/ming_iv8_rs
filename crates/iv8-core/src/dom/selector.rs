//! CSS Selector engine integration via the `selectors` crate.
//!
//! Implements `selectors::Element` for our ego-tree DOM nodes,
//! enabling querySelector/querySelectorAll functionality.

use std::fmt;

use cssparser::{
    match_ignore_ascii_case, CowRcStr, ParseError, SourceLocation, ToCss,
};
use ego_tree::NodeRef;
use precomputed_hash::PrecomputedHash;
use selectors::attr::{AttrSelectorOperation, CaseSensitivity, NamespaceConstraint};
use selectors::bloom::BloomFilter;
use selectors::matching::{
    ElementSelectorFlags, MatchingContext, MatchingForInvalidation, MatchingMode,
    NeedsSelectorFlags, QuirksMode, SelectorCaches,
};
use selectors::parser::{self, SelectorParseErrorKind};
use selectors::{self, OpaqueElement};

use super::node::{Document, NodeData, NodeId};

// ─── SelectorImpl ───────────────────────────────────────────────────────────

/// Our custom SelectorImpl — defines the associated types for the selectors crate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Iv8SelectorImpl;

/// A simple string-based type that satisfies all the selector bounds.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CssString(pub String);

impl<'a> From<&'a str> for CssString {
    fn from(s: &'a str) -> Self {
        CssString(s.to_string())
    }
}

impl AsRef<str> for CssString {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::borrow::Borrow<str> for CssString {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl ToCss for CssString {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        cssparser::serialize_identifier(&self.0, dest)
    }
}

impl PrecomputedHash for CssString {
    fn precomputed_hash(&self) -> u32 {
        // Simple FNV-1a hash
        let mut hash: u32 = 2166136261;
        for byte in self.0.bytes() {
            hash ^= byte as u32;
            hash = hash.wrapping_mul(16777619);
        }
        hash
    }
}

/// Non-tree-structural pseudo-classes supported by IV8.
///
/// Tree-structural pseudo-classes (`:first-child`, `:last-child`,
/// `:only-child`, `:nth-child`, `:empty`, `:root`, `:first-of-type`, etc.)
/// are handled internally by the `selectors` crate and do not appear here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoClass {
    Hover,
    Active,
    Focus,
    FocusVisible,
    FocusWithin,
    Visited,
    Link,
    Checked,
    Disabled,
    Enabled,
    PlaceholderShown,
    ReadOnly,
    ReadWrite,
    Required,
    Optional,
    Valid,
    Invalid,
}

impl ToCss for PseudoClass {
    fn to_css<W: fmt::Write>(&self, dest: &mut W) -> fmt::Result {
        let s = match self {
            PseudoClass::Hover => ":hover",
            PseudoClass::Active => ":active",
            PseudoClass::Focus => ":focus",
            PseudoClass::FocusVisible => ":focus-visible",
            PseudoClass::FocusWithin => ":focus-within",
            PseudoClass::Visited => ":visited",
            PseudoClass::Link => ":link",
            PseudoClass::Checked => ":checked",
            PseudoClass::Disabled => ":disabled",
            PseudoClass::Enabled => ":enabled",
            PseudoClass::PlaceholderShown => ":placeholder-shown",
            PseudoClass::ReadOnly => ":read-only",
            PseudoClass::ReadWrite => ":read-write",
            PseudoClass::Required => ":required",
            PseudoClass::Optional => ":optional",
            PseudoClass::Valid => ":valid",
            PseudoClass::Invalid => ":invalid",
        };
        dest.write_str(s)
    }
}

impl parser::NonTSPseudoClass for PseudoClass {
    type Impl = Iv8SelectorImpl;

    fn is_active_or_hover(&self) -> bool {
        matches!(*self, PseudoClass::Hover | PseudoClass::Active)
    }

    fn is_user_action_state(&self) -> bool {
        matches!(
            *self,
            PseudoClass::Hover
                | PseudoClass::Active
                | PseudoClass::Focus
                | PseudoClass::FocusVisible
                | PseudoClass::FocusWithin
                | PseudoClass::Visited
        )
    }
}

/// Pseudo-element stub.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PseudoElement {
    // Placeholder
    _Placeholder,
}

impl ToCss for PseudoElement {
    fn to_css<W: fmt::Write>(&self, _dest: &mut W) -> fmt::Result {
        Ok(())
    }
}

impl parser::PseudoElement for PseudoElement {
    type Impl = Iv8SelectorImpl;
}

impl selectors::SelectorImpl for Iv8SelectorImpl {
    type ExtraMatchingData<'a> = ();
    type AttrValue = CssString;
    type Identifier = CssString;
    type LocalName = CssString;
    type NamespaceUrl = CssString;
    type NamespacePrefix = CssString;
    type BorrowedNamespaceUrl = str;
    type BorrowedLocalName = str;
    type NonTSPseudoClass = PseudoClass;
    type PseudoElement = PseudoElement;
}

// ─── Element impl ───────────────────────────────────────────────────────────

/// A reference to a DOM element node for selector matching.
/// Wraps an ego-tree NodeRef and only matches Element nodes.
#[derive(Clone, Debug)]
pub struct DomElement<'a> {
    node: NodeRef<'a, NodeData>,
}

impl<'a> DomElement<'a> {
    /// Create a DomElement from a NodeRef. Panics if not an Element.
    pub fn new(node: NodeRef<'a, NodeData>) -> Self {
        debug_assert!(
            node.value().is_element(),
            "DomElement must wrap an Element node"
        );
        Self { node }
    }

    /// Try to create a DomElement, returning None if not an Element.
    pub fn try_new(node: NodeRef<'a, NodeData>) -> Option<Self> {
        if node.value().is_element() {
            Some(Self { node })
        } else {
            None
        }
    }

    /// Get the underlying NodeRef.
    pub fn node_ref(&self) -> &NodeRef<'a, NodeData> {
        &self.node
    }

    /// Get the NodeId.
    pub fn id(&self) -> NodeId {
        self.node.id()
    }

    /// Check if the element is in a "checked" state.
    ///
    /// For `input[type=checkbox]` and `input[type=radio]`, the `checked`
    /// attribute must be present (HTML boolean attribute).
    /// For `option` elements, the `selected` attribute must be present.
    fn is_checked(&self) -> bool {
        let tag = match self.node.value().tag_name() {
            Some(t) => t,
            None => return false,
        };
        let tag = tag.to_ascii_lowercase();
        match tag.as_str() {
            "input" => {
                let input_type = self
                    .node
                    .value()
                    .get_attr("type")
                    .map(|t| t.to_ascii_lowercase())
                    .unwrap_or_else(|| "text".to_string());
                matches!(input_type.as_str(), "checkbox" | "radio")
                    && self.node.value().get_attr("checked").is_some()
            }
            "option" => self.node.value().get_attr("selected").is_some(),
            _ => false,
        }
    }

    /// Check if the element is in a "disabled" state.
    ///
    /// The `disabled` attribute is a boolean attribute for `button`,
    /// `input`, `select`, `textarea`, `optgroup`, `option`, and `fieldset`.
    fn is_disabled(&self) -> bool {
        let tag = match self.node.value().tag_name() {
            Some(t) => t.to_ascii_lowercase(),
            None => return false,
        };
        let is_disableable = matches!(
            tag.as_str(),
            "button"
                | "input"
                | "select"
                | "textarea"
                | "optgroup"
                | "option"
                | "fieldset"
        );
        is_disableable && self.node.value().get_attr("disabled").is_some()
    }
}

impl<'a> selectors::Element for DomElement<'a> {
    type Impl = Iv8SelectorImpl;

    fn opaque(&self) -> OpaqueElement {
        OpaqueElement::new(self.node.value())
    }

    fn parent_element(&self) -> Option<Self> {
        self.node.parent().and_then(DomElement::try_new)
    }

    fn parent_node_is_shadow_root(&self) -> bool {
        false
    }

    fn containing_shadow_host(&self) -> Option<Self> {
        None
    }

    fn is_pseudo_element(&self) -> bool {
        false
    }

    fn prev_sibling_element(&self) -> Option<Self> {
        let mut sibling = self.node.prev_sibling();
        while let Some(s) = sibling {
            if let Some(elem) = DomElement::try_new(s) {
                return Some(elem);
            }
            sibling = s.prev_sibling();
        }
        None
    }

    fn next_sibling_element(&self) -> Option<Self> {
        let mut sibling = self.node.next_sibling();
        while let Some(s) = sibling {
            if let Some(elem) = DomElement::try_new(s) {
                return Some(elem);
            }
            sibling = s.next_sibling();
        }
        None
    }

    fn first_element_child(&self) -> Option<Self> {
        self.node.children().find_map(DomElement::try_new)
    }

    fn is_html_element_in_html_document(&self) -> bool {
        true
    }

    fn has_local_name(&self, local_name: &str) -> bool {
        self.node
            .value()
            .tag_name()
            .map(|t| t.eq_ignore_ascii_case(local_name))
            .unwrap_or(false)
    }

    fn has_namespace(&self, ns: &str) -> bool {
        match self.node.value() {
            NodeData::Element { namespace, .. } => ns.is_empty() || namespace == ns,
            _ => false,
        }
    }

    fn is_same_type(&self, other: &Self) -> bool {
        self.node.value().tag_name() == other.node.value().tag_name()
    }

    fn attr_matches(
        &self,
        ns: &NamespaceConstraint<&CssString>,
        local_name: &CssString,
        operation: &AttrSelectorOperation<&CssString>,
    ) -> bool {
        let attrs = self.node.value().attrs();
        let _ = ns; // ignore namespace for HTML attributes

        attrs.iter().any(|(name, value)| {
            if !name.eq_ignore_ascii_case(&local_name.0) {
                return false;
            }
            operation.eval_str(value)
        })
    }

    fn match_non_ts_pseudo_class(
        &self,
        pc: &PseudoClass,
        _context: &mut MatchingContext<Iv8SelectorImpl>,
    ) -> bool {
        match *pc {
            PseudoClass::Hover
            | PseudoClass::Active
            | PseudoClass::Focus
            | PseudoClass::FocusVisible
            | PseudoClass::FocusWithin
            | PseudoClass::Visited => false,
            PseudoClass::Link => self.is_link(),
            PseudoClass::Checked => self.is_checked(),
            PseudoClass::Disabled => self.is_disabled(),
            PseudoClass::Enabled => !self.is_disabled(),
            PseudoClass::PlaceholderShown
            | PseudoClass::ReadOnly
            | PseudoClass::ReadWrite
            | PseudoClass::Required
            | PseudoClass::Optional
            | PseudoClass::Valid
            | PseudoClass::Invalid => false,
        }
    }

    fn match_pseudo_element(
        &self,
        _pe: &PseudoElement,
        _context: &mut MatchingContext<Iv8SelectorImpl>,
    ) -> bool {
        false
    }

    fn apply_selector_flags(&self, _flags: ElementSelectorFlags) {}

    fn is_link(&self) -> bool {
        matches!(
            self.node.value().tag_name(),
            Some("a") | Some("area") | Some("link")
        ) && self.node.value().get_attr("href").is_some()
    }

    fn is_html_slot_element(&self) -> bool {
        self.node.value().tag_name() == Some("slot")
    }

    fn has_id(&self, id: &CssString, case_sensitivity: CaseSensitivity) -> bool {
        self.node
            .value()
            .element_id()
            .map(|elem_id| case_sensitivity.eq(elem_id.as_bytes(), id.0.as_bytes()))
            .unwrap_or(false)
    }

    fn has_class(&self, name: &CssString, case_sensitivity: CaseSensitivity) -> bool {
        self.node
            .value()
            .class_list()
            .iter()
            .any(|c| case_sensitivity.eq(c.as_bytes(), name.0.as_bytes()))
    }

    fn has_custom_state(&self, _name: &CssString) -> bool {
        false
    }

    fn imported_part(&self, _name: &CssString) -> Option<CssString> {
        None
    }

    fn is_part(&self, _name: &CssString) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        !self.node.children().any(|child| match child.value() {
            NodeData::Element { .. } => true,
            NodeData::Text(t) => !t.is_empty(),
            _ => false,
        })
    }

    fn is_root(&self) -> bool {
        self.node
            .parent()
            .map(|p| matches!(p.value(), NodeData::Document))
            .unwrap_or(false)
    }

    fn add_element_unique_hashes(&self, _filter: &mut BloomFilter) -> bool {
        false
    }
}

// ─── Selector parsing + matching API ────────────────────────────────────────

/// Our CSS selector parser.
struct Iv8Parser;

/// Custom error type that wraps SelectorParseErrorKind.
#[derive(Debug)]
pub struct SelectorError<'i>(pub SelectorParseErrorKind<'i>);

impl<'i> From<SelectorParseErrorKind<'i>> for SelectorError<'i> {
    fn from(e: SelectorParseErrorKind<'i>) -> Self {
        SelectorError(e)
    }
}

impl<'i> parser::Parser<'i> for Iv8Parser {
    type Impl = Iv8SelectorImpl;
    type Error = SelectorError<'i>;

    fn parse_non_ts_pseudo_class(
        &self,
        location: SourceLocation,
        name: CowRcStr<'i>,
    ) -> Result<PseudoClass, ParseError<'i, SelectorError<'i>>> {
        match_ignore_ascii_case! { &name,
            "hover" => return Ok(PseudoClass::Hover),
            "active" => return Ok(PseudoClass::Active),
            "focus" => return Ok(PseudoClass::Focus),
            "focus-visible" => return Ok(PseudoClass::FocusVisible),
            "focus-within" => return Ok(PseudoClass::FocusWithin),
            "visited" => return Ok(PseudoClass::Visited),
            "link" => return Ok(PseudoClass::Link),
            "checked" => return Ok(PseudoClass::Checked),
            "disabled" => return Ok(PseudoClass::Disabled),
            "enabled" => return Ok(PseudoClass::Enabled),
            "placeholder-shown" => return Ok(PseudoClass::PlaceholderShown),
            "read-only" => return Ok(PseudoClass::ReadOnly),
            "read-write" => return Ok(PseudoClass::ReadWrite),
            "required" => return Ok(PseudoClass::Required),
            "optional" => return Ok(PseudoClass::Optional),
            "valid" => return Ok(PseudoClass::Valid),
            "invalid" => return Ok(PseudoClass::Invalid),
            _ => {}
        }
        Err(
            location.new_custom_error(SelectorParseErrorKind::UnsupportedPseudoClassOrElement(
                name,
            )),
        )
    }
}

/// A parsed CSS selector list.
pub struct Selector(selectors::SelectorList<Iv8SelectorImpl>);

impl Selector {
    /// Parse a CSS selector string.
    pub fn parse(selector: &str) -> Result<Self, String> {
        let mut parser_input = cssparser::ParserInput::new(selector);
        let mut parser = cssparser::Parser::new(&mut parser_input);
        selectors::SelectorList::parse(&Iv8Parser, &mut parser, parser::ParseRelative::No)
            .map(Selector)
            .map_err(|e| format!("invalid selector: {:?}", e))
    }

    /// Check if an element matches this selector.
    pub fn matches(&self, element: &DomElement) -> bool {
        let mut caches = SelectorCaches::default();
        let mut context = MatchingContext::new(
            MatchingMode::Normal,
            None,
            &mut caches,
            QuirksMode::NoQuirks,
            NeedsSelectorFlags::No,
            MatchingForInvalidation::No,
        );
        selectors::matching::matches_selector_list(&self.0, element, &mut context)
    }
}

// ─── Document query methods ─────────────────────────────────────────────────

impl Document {
    /// querySelector — returns the first element matching the CSS selector.
    pub fn query_selector(&self, selector_str: &str) -> Result<Option<NodeId>, String> {
        let selector = Selector::parse(selector_str)?;

        for node_ref in self.tree.root().descendants() {
            if let Some(elem) = DomElement::try_new(node_ref) {
                if selector.matches(&elem) {
                    return Ok(Some(elem.id()));
                }
            }
        }
        Ok(None)
    }

    /// querySelectorAll — returns all elements matching the CSS selector.
    pub fn query_selector_all(&self, selector_str: &str) -> Result<Vec<NodeId>, String> {
        let selector = Selector::parse(selector_str)?;
        let mut results = Vec::new();

        for node_ref in self.tree.root().descendants() {
            if let Some(elem) = DomElement::try_new(node_ref) {
                if selector.matches(&elem) {
                    results.push(elem.id());
                }
            }
        }
        Ok(results)
    }

    /// querySelector scoped to a subtree rooted at `context_node`.
    pub fn query_selector_from(
        &self,
        selector_str: &str,
        context_node: NodeId,
    ) -> Result<Option<NodeId>, String> {
        let selector = Selector::parse(selector_str)?;
        if let Some(node_ref) = self.tree.get(context_node) {
            for descendant in node_ref.descendants() {
                if let Some(elem) = DomElement::try_new(descendant) {
                    if selector.matches(&elem) {
                        return Ok(Some(elem.id()));
                    }
                }
            }
        }
        Ok(None)
    }

    /// querySelectorAll scoped to a subtree rooted at `context_node`.
    pub fn query_selector_all_from(
        &self,
        selector_str: &str,
        context_node: NodeId,
    ) -> Result<Vec<NodeId>, String> {
        let selector = Selector::parse(selector_str)?;
        let mut results = Vec::new();
        if let Some(node_ref) = self.tree.get(context_node) {
            for descendant in node_ref.descendants() {
                if let Some(elem) = DomElement::try_new(descendant) {
                    if selector.matches(&elem) {
                        results.push(elem.id());
                    }
                }
            }
        }
        Ok(results)
    }

    /// Get elements by tag name scoped to a subtree.
    pub fn get_elements_by_tag_name_from(&self, tag: &str, context_node: NodeId) -> Vec<NodeId> {
        let tag_lower = tag.to_ascii_lowercase();
        let mut results = Vec::new();
        if let Some(node_ref) = self.tree.get(context_node) {
            for descendant in node_ref.descendants() {
                if let NodeData::Element { tag_name, .. } = descendant.value() {
                    if tag_lower == "*" || tag_name == &tag_lower {
                        results.push(descendant.id());
                    }
                }
            }
        }
        results
    }

    /// Check if an element matches a CSS selector.
    pub fn element_matches(&self, node_id: NodeId, selector_str: &str) -> bool {
        let selector = match Selector::parse(selector_str) {
            Ok(s) => s,
            Err(_) => return false,
        };
        if let Some(node_ref) = self.tree.get(node_id) {
            if let Some(elem) = DomElement::try_new(node_ref) {
                return selector.matches(&elem);
            }
        }
        false
    }

    /// Find the closest ancestor (or self) matching a CSS selector.
    pub fn closest(&self, node_id: NodeId, selector_str: &str) -> Option<NodeId> {
        let selector = Selector::parse(selector_str).ok()?;
        let mut current = self.tree.get(node_id)?;
        loop {
            if let Some(elem) = DomElement::try_new(current) {
                if selector.matches(&elem) {
                    return Some(elem.id());
                }
            }
            current = current.parent()?;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::dom::parser::parse_html;

    #[test]
    fn selector_by_tag() {
        let doc = parse_html("<div><p>hello</p><span>world</span></div>", None);
        let results = doc.query_selector_all("p").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_by_id() {
        let doc = parse_html("<div id=\"main\"><p id=\"target\">text</p></div>", None);
        let result = doc.query_selector("#target").unwrap();
        assert!(result.is_some());
        let node = doc.get(result.unwrap()).unwrap();
        assert_eq!(node.value().tag_name(), Some("p"));
    }

    #[test]
    fn selector_by_class() {
        let doc = parse_html(
            "<div class=\"a\"><p class=\"b c\">1</p><p class=\"b\">2</p></div>",
            None,
        );
        let results = doc.query_selector_all(".b").unwrap();
        assert_eq!(results.len(), 2);

        let results_c = doc.query_selector_all(".c").unwrap();
        assert_eq!(results_c.len(), 1);
    }

    #[test]
    fn selector_descendant() {
        let doc = parse_html("<div><ul><li>1</li><li>2</li></ul></div>", None);
        let results = doc.query_selector_all("div li").unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn selector_child_combinator() {
        let doc = parse_html("<div><p>direct</p><span><p>nested</p></span></div>", None);
        let results = doc.query_selector_all("div > p").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_attribute() {
        let doc = parse_html(
            "<a href=\"https://example.com\">link</a><a>no href</a>",
            None,
        );
        let results = doc.query_selector_all("a[href]").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_attribute_value() {
        let doc = parse_html("<input type=\"text\"><input type=\"hidden\">", None);
        let results = doc.query_selector_all("input[type=\"text\"]").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_multiple() {
        let doc = parse_html("<div><p>1</p><span>2</span><p>3</p></div>", None);
        let results = doc.query_selector_all("p, span").unwrap();
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn selector_first_child() {
        let doc = parse_html("<ul><li>1</li><li>2</li><li>3</li></ul>", None);
        let results = doc.query_selector_all("li:first-child").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_empty() {
        let doc = parse_html("<div></div><div>text</div>", None);
        let results = doc.query_selector_all("div:empty").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_invalid_returns_error() {
        let doc = parse_html("<div></div>", None);
        let result = doc.query_selector("[[[invalid");
        assert!(result.is_err());
    }

    #[test]
    fn query_selector_returns_first() {
        let doc = parse_html("<p>1</p><p>2</p><p>3</p>", None);
        let result = doc.query_selector("p").unwrap().unwrap();
        assert_eq!(doc.text_content_of(result), "1");
    }

    #[test]
    fn selector_no_match_returns_none() {
        let doc = parse_html("<div>hello</div>", None);
        let result = doc.query_selector("span").unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn selector_hover_returns_nothing() {
        let doc = parse_html("<div>hello</div>", None);
        let results = doc.query_selector_all("div:hover").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn selector_focus_returns_nothing() {
        let doc = parse_html("<input><input>", None);
        let results = doc.query_selector_all("input:focus").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn selector_active_returns_nothing() {
        let doc = parse_html("<a href=\"#\">link</a>", None);
        let results = doc.query_selector_all("a:active").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn selector_visited_returns_nothing() {
        let doc = parse_html("<a href=\"#\">link</a>", None);
        let results = doc.query_selector_all("a:visited").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn selector_focus_visible_returns_nothing() {
        let doc = parse_html("<button>btn</button>", None);
        let results = doc.query_selector_all("button:focus-visible").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn selector_focus_within_returns_nothing() {
        let doc = parse_html("<div><input></div>", None);
        let results = doc.query_selector_all("div:focus-within").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn selector_link_matches_anchor_with_href() {
        let doc = parse_html(
            "<a href=\"https://example.com\">link</a><a>no href</a>",
            None,
        );
        let results = doc.query_selector_all("a:link").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_link_matches_area_with_href() {
        let doc = parse_html(
            "<area href=\"https://example.com\"><area>",
            None,
        );
        let results = doc.query_selector_all("area:link").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_checked_checkbox() {
        let doc = parse_html(
            "<input type=\"checkbox\" checked><input type=\"checkbox\">",
            None,
        );
        let results = doc.query_selector_all("input:checked").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_checked_radio() {
        let doc = parse_html(
            "<input type=\"radio\" checked><input type=\"radio\">",
            None,
        );
        let results = doc.query_selector_all("input:checked").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_checked_option() {
        let doc = parse_html(
            "<select><option>1</option><option selected>2</option></select>",
            None,
        );
        let results = doc.query_selector_all("option:checked").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_checked_text_input_not_matched() {
        let doc = parse_html("<input type=\"text\" checked>", None);
        let results = doc.query_selector_all("input:checked").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn selector_disabled_input() {
        let doc = parse_html(
            "<input disabled><input>",
            None,
        );
        let results = doc.query_selector_all("input:disabled").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_disabled_button() {
        let doc = parse_html(
            "<button disabled>btn</button><button>btn2</button>",
            None,
        );
        let results = doc.query_selector_all("button:disabled").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_disabled_select_textarea() {
        let doc = parse_html(
            "<select disabled></select><textarea disabled></textarea>",
            None,
        );
        let results = doc.query_selector_all("select:disabled, textarea:disabled").unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn selector_enabled_input() {
        let doc = parse_html(
            "<input disabled><input>",
            None,
        );
        let results = doc.query_selector_all("input:enabled").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_enabled_not_disableable_tag() {
        let doc = parse_html("<div disabled></div>", None);
        let results = doc.query_selector_all("div:enabled").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_disabled_not_disableable_tag() {
        let doc = parse_html("<div disabled></div>", None);
        let results = doc.query_selector_all("div:disabled").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn selector_last_child() {
        let doc = parse_html("<ul><li>1</li><li>2</li><li>3</li></ul>", None);
        let results = doc.query_selector_all("li:last-child").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_only_child() {
        let doc = parse_html("<div><p>only</p></div><div><p>1</p><p>2</p></div>", None);
        let results = doc.query_selector_all("p:only-child").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_nth_child() {
        let doc = parse_html("<ul><li>1</li><li>2</li><li>3</li></ul>", None);
        let results = doc.query_selector_all("li:nth-child(2)").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_nth_child_odd() {
        let doc = parse_html("<ul><li>1</li><li>2</li><li>3</li><li>4</li></ul>", None);
        let results = doc.query_selector_all("li:nth-child(odd)").unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn selector_root() {
        let doc = parse_html("<html><body></body></html>", None);
        let results = doc.query_selector_all("html:root").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_first_of_type() {
        let doc = parse_html("<div><p>1</p><span>2</span><p>3</p></div>", None);
        let results = doc.query_selector_all("p:first-of-type").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_last_of_type() {
        let doc = parse_html("<div><p>1</p><span>2</span><p>3</p></div>", None);
        let results = doc.query_selector_all("p:last-of-type").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_nth_of_type() {
        let doc = parse_html("<div><p>1</p><span>2</span><p>3</p><p>4</p></div>", None);
        let results = doc.query_selector_all("p:nth-of-type(2)").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_nth_of_type_odd() {
        let doc = parse_html(
            "<div><p>1</p><span>s</span><p>2</p><span>s</span><p>3</p></div>",
            None,
        );
        let results = doc.query_selector_all("p:nth-of-type(odd)").unwrap();
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn selector_only_of_type() {
        let doc = parse_html(
            "<div><p>only</p><span>1</span><span>2</span></div>",
            None,
        );
        let results = doc.query_selector_all("p:only-of-type").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_only_of_type_none_when_multiple() {
        let doc = parse_html("<div><p>1</p><p>2</p></div>", None);
        let results = doc.query_selector_all("p:only-of-type").unwrap();
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn selector_pseudo_class_with_combinator() {
        let doc = parse_html(
            "<form><input type=\"checkbox\" checked></form>",
            None,
        );
        let results = doc.query_selector_all("form input:checked").unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn selector_unknown_pseudo_class_returns_error() {
        let doc = parse_html("<div></div>", None);
        let result = doc.query_selector("div:nonexistent");
        assert!(result.is_err());
    }
}
