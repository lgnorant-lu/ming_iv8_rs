#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]

//! Integration tests for DOM navigation properties (Task 65).
//! Acceptance criteria:
//! - element.childNodes returns array-like with length + indexing
//! - element.parentNode returns parent node object
//! - element.firstChild / lastChild correct
//! - element.nextSibling / previousSibling correct
//! - element.children only returns Element children

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};

fn make_kernel_with_doc(html: &str) -> EmbeddedV8Kernel {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    kernel.set_document(html, None);
    kernel
}

#[test]
fn parent_node_exists() {
    let mut kernel = make_kernel_with_doc("<div id='parent'><p id='child'>text</p></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var child = document.getElementById('child');
        var parent = child.parentNode;
        parent !== null && parent.tagName === 'DIV'
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn first_child() {
    let mut kernel = make_kernel_with_doc("<div id='parent'><span>first</span><p>second</p></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var parent = document.getElementById('parent');
        parent.firstChild !== null && parent.firstChild.tagName === 'SPAN'
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn last_child() {
    let mut kernel = make_kernel_with_doc("<div id='parent'><span>first</span><p>second</p></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var parent = document.getElementById('parent');
        parent.lastChild !== null && parent.lastChild.tagName === 'P'
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn next_sibling() {
    let mut kernel = make_kernel_with_doc("<div><span id='a'>1</span><p id='b'>2</p></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var a = document.getElementById('a');
        a.nextSibling !== null && a.nextSibling.tagName === 'P'
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn previous_sibling() {
    let mut kernel = make_kernel_with_doc("<div><span id='a'>1</span><p id='b'>2</p></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var b = document.getElementById('b');
        b.previousSibling !== null && b.previousSibling.tagName === 'SPAN'
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn child_nodes_length() {
    let mut kernel = make_kernel_with_doc("<ul id='list'><li>1</li><li>2</li><li>3</li></ul>");
    let result = kernel.eval_to_rust_value(
        r#"
        var list = document.getElementById('list');
        list.childNodes.length
    "#,
    );
    assert_eq!(result, RustValue::Int(3));
}

#[test]
fn child_nodes_indexing() {
    let mut kernel = make_kernel_with_doc("<ul id='list'><li>a</li><li>b</li></ul>");
    let result = kernel.eval_to_rust_value(
        r#"
        var list = document.getElementById('list');
        list.childNodes[1].textContent
    "#,
    );
    assert_eq!(result, RustValue::String("b".into()));
}

#[test]
fn children_only_elements() {
    let mut kernel = make_kernel_with_doc("<div id='mixed'><span>elem</span></div>");
    // html5ever may add text nodes for whitespace, children should only have elements
    let result = kernel.eval_to_rust_value(
        r#"
        var el = document.getElementById('mixed');
        var allElements = true;
        var kids = el.children;
        for (var i = 0; i < kids.length; i++) {
            if (kids[i].nodeType !== 1) allElements = false;
        }
        allElements
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn child_element_count() {
    let mut kernel = make_kernel_with_doc("<div id='parent'><p>1</p><p>2</p><p>3</p></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        document.getElementById('parent').childElementCount
    "#,
    );
    assert_eq!(result, RustValue::Int(3));
}

#[test]
fn parent_node_null_for_root() {
    let mut kernel = make_kernel_with_doc("<div id='x'></div>");
    // The html element's parent is the document node (not an element)
    // parentElement should be null for html element
    let result = kernel.eval_to_rust_value(
        r#"
        var html = document.querySelector('html');
        html.parentElement === null
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn navigation_chain() {
    let mut kernel = make_kernel_with_doc("<div id='root'><a id='link'>click</a></div>");
    // Navigate: getElementById → parentNode → firstChild → textContent
    let result = kernel.eval_to_rust_value(
        r#"
        var link = document.getElementById('link');
        var parent = link.parentNode;
        parent.id === 'root' && parent.firstChild.id === 'link'
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}
