//! K-008 V8 accessor property workaround tests.
//!
//! V8 set_accessor_property getters cannot be called via JS .call().
//! Workarounds create return values directly instead of calling
//! origGet.call(this). These tests validate the workarounds.

mod common;

use common::*;

#[test]
fn test_dataset_returns_domstringmap() {
    let mut k = make_kernel_with_doc("<div data-foo='bar' data-test='123'></div>");
    assert_js_str(&mut k, "document.querySelector('div').dataset.foo", "bar");
    assert_js_str(&mut k, "document.querySelector('div').dataset.test", "123");
}

#[test]
fn test_dataset_stringification_is_domstringmap() {
    let mut k = make_kernel_with_doc("<div data-x='y'></div>");
    assert_js_str(
        &mut k,
        "Object.prototype.toString.call(document.querySelector('div').dataset)",
        "[object DOMStringMap]",
    );
}

#[test]
fn test_dataset_instanceof_domstringmap() {
    let mut k = make_kernel_with_doc("<div data-x='y'></div>");
    let val = to_str(&k.eval_to_rust_value(
        "document.querySelector('div').dataset instanceof DOMStringMap"
    ));
    assert_eq!(val, "true", "dataset should be instanceof DOMStringMap");
}

#[test]
fn test_dataset_getter_throws_on_wrong_receiver() {
    let mut k = make_kernel();
    assert_js_error(&mut k, "(function(){ var g = Object.getOwnPropertyDescriptor(HTMLElement.prototype, 'dataset').get; g.call({}); })()");
}

#[test]
fn test_children_returns_htmlcollection() {
    let mut k = make_kernel_with_doc("<div><span></span><p></p></div>");
    assert_js_str(&mut k, "document.querySelector('div').children.length", "2");
}

#[test]
fn test_children_stringification_is_htmlcollection() {
    let mut k = make_kernel_with_doc("<div><span></span></div>");
    assert_js_str(
        &mut k,
        "Object.prototype.toString.call(document.querySelector('div').children)",
        "[object HTMLCollection]",
    );
}

#[test]
fn test_children_item_method_works() {
    let mut k = make_kernel_with_doc("<div><span></span><p></p></div>");
    let val = to_str(&k.eval_to_rust_value(
        "document.querySelector('div').children.item(0).tagName"
    ));
    assert_eq!(val, "SPAN", "children.item(0) should return first child");
}

#[test]
fn test_children_nameditem_method_works() {
    let mut k = make_kernel_with_doc("<div><span id='myid'></span></div>");
    assert_js_str(&mut k,
        "typeof document.querySelector('div').children.namedItem('myid')",
        "object");
}

#[test]
fn test_children_getter_throws_on_wrong_receiver() {
    let mut k = make_kernel();
    assert_js_error(&mut k, "(function(){ var g = Object.getOwnPropertyDescriptor(Element.prototype, 'children').get; g.call({}); })()");
}

#[test]
fn test_event_handler_getter_works_on_element() {
    let mut k = make_kernel();
    let val = to_str(&k.eval_to_rust_value(
        "document.createElement('div').onmouseenter"
    ));
    assert_eq!(val, "null", "onmouseenter should return null on fresh element");
}

#[test]
fn test_event_handler_getter_throws_on_wrong_receiver() {
    let mut k = make_kernel();
    assert_js_error(&mut k, "(function(){ var g = Object.getOwnPropertyDescriptor(HTMLElement.prototype, 'onmouseenter').get; g.call({}); })()");
}
