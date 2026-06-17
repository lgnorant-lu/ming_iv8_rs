#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;


// Integration tests for DOM V8 bindings (Task 27).
// Tests document.getElementById, querySelector, querySelectorAll, getElementsByTagName.

use iv8_core::RustValue;

#[test]
fn get_element_by_id_found() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"main\">hello</div>");
    let result = kernel.eval_to_rust_value("document.getElementById('main').tagName");
    assert_eq!(result, RustValue::String("DIV".into()));
}

#[test]
fn get_element_by_id_text_content() {
    let mut kernel = common::make_kernel_with_doc("<p id=\"msg\">Hello World</p>");
    let result = kernel.eval_to_rust_value("document.getElementById('msg').textContent");
    assert_eq!(result, RustValue::String("Hello World".into()));
}

#[test]
fn get_element_by_id_not_found() {
    let mut kernel = common::make_kernel_with_doc("<div>no id</div>");
    let result = kernel.eval_to_rust_value("document.getElementById('missing')");
    assert_eq!(result, RustValue::Null);
}

#[test]
fn query_selector_by_tag() {
    let mut kernel = common::make_kernel_with_doc("<div><p>first</p><p>second</p></div>");
    let result = kernel.eval_to_rust_value("document.querySelector('p').textContent");
    assert_eq!(result, RustValue::String("first".into()));
}

#[test]
fn query_selector_by_class() {
    let mut kernel = common::make_kernel_with_doc("<span class=\"highlight\">text</span>");
    let result = kernel.eval_to_rust_value("document.querySelector('.highlight').tagName");
    assert_eq!(result, RustValue::String("SPAN".into()));
}

#[test]
fn query_selector_not_found() {
    let mut kernel = common::make_kernel_with_doc("<div>hello</div>");
    let result = kernel.eval_to_rust_value("document.querySelector('.missing')");
    assert_eq!(result, RustValue::Null);
}

#[test]
fn query_selector_all_count() {
    let mut kernel = common::make_kernel_with_doc("<ul><li>1</li><li>2</li><li>3</li></ul>");
    let result = kernel.eval_to_rust_value("document.querySelectorAll('li').length");
    assert_eq!(result, RustValue::Int(3));
}

#[test]
fn query_selector_all_access_items() {
    let mut kernel = common::make_kernel_with_doc("<div><p>a</p><p>b</p></div>");
    let result = kernel.eval_to_rust_value("document.querySelectorAll('p')[1].textContent");
    assert_eq!(result, RustValue::String("b".into()));
}

#[test]
fn get_elements_by_tag_name() {
    let mut kernel = common::make_kernel_with_doc("<div><span>1</span><span>2</span></div>");
    let result = kernel.eval_to_rust_value("document.getElementsByTagName('span').length");
    assert_eq!(result, RustValue::Int(2));
}

#[test]
fn get_elements_by_class_name() {
    let mut kernel =
        common::make_kernel_with_doc("<div class=\"a\"><p class=\"b\">1</p><p class=\"b\">2</p></div>");
    let result = kernel.eval_to_rust_value("document.getElementsByClassName('b').length");
    assert_eq!(result, RustValue::Int(2));
}

#[test]
fn get_attribute() {
    let mut kernel =
        common::make_kernel_with_doc("<a href=\"https://example.com\" target=\"_blank\">link</a>");
    let result = kernel.eval_to_rust_value("document.querySelector('a').getAttribute('href')");
    assert_eq!(result, RustValue::String("https://example.com".into()));
}

#[test]
fn get_attribute_missing() {
    let mut kernel = common::make_kernel_with_doc("<div>no attrs</div>");
    let result = kernel.eval_to_rust_value("document.querySelector('div').getAttribute('data-x')");
    assert_eq!(result, RustValue::Null);
}

#[test]
fn element_id_property() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"test\">content</div>");
    let result = kernel.eval_to_rust_value("document.getElementById('test').id");
    assert_eq!(result, RustValue::String("test".into()));
}

#[test]
fn element_class_name_property() {
    let mut kernel = common::make_kernel_with_doc("<div class=\"foo bar\">content</div>");
    let result = kernel.eval_to_rust_value("document.querySelector('div').className");
    assert_eq!(result, RustValue::String("foo bar".into()));
}

#[test]
fn element_node_type() {
    let mut kernel = common::make_kernel_with_doc("<div>content</div>");
    let result = kernel.eval_to_rust_value("document.querySelector('div').nodeType");
    assert_eq!(result, RustValue::Int(1));
}

// ─── Task 28: DOM Mutation Tests ────────────────────────────────────────────

#[test]
fn create_element() {
    let mut kernel = common::make_kernel_with_doc("<body></body>");
    let result = kernel.eval_to_rust_value("document.createElement('div').tagName");
    assert_eq!(result, RustValue::String("DIV".into()));
}

#[test]
fn create_element_node_type() {
    let mut kernel = common::make_kernel_with_doc("<body></body>");
    let result = kernel.eval_to_rust_value("document.createElement('span').nodeType");
    assert_eq!(result, RustValue::Int(1));
}

#[test]
fn append_child_adds_to_dom() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"container\"></div>");
    kernel.eval_to_rust_value(
        r#"
        var container = document.getElementById('container');
        var child = document.createElement('p');
        container.appendChild(child);
    "#,
    );
    // Now querySelectorAll should find the new p inside container
    let result = kernel.eval_to_rust_value("document.querySelectorAll('p').length");
    assert_eq!(result, RustValue::Int(1));
}

#[test]
fn remove_child_removes_from_dom() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"parent\"><p id=\"child\">text</p></div>");
    kernel.eval_to_rust_value(
        r#"
        var parent = document.getElementById('parent');
        var child = document.getElementById('child');
        parent.removeChild(child);
    "#,
    );
    let result = kernel.eval_to_rust_value("document.querySelectorAll('p').length");
    assert_eq!(result, RustValue::Int(0));
}

#[test]
fn set_attribute_updates_dom() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"target\"></div>");
    kernel.eval_to_rust_value(
        r#"
        var el = document.getElementById('target');
        el.setAttribute('data-value', 'hello');
    "#,
    );
    let result = kernel.eval_to_rust_value(
        r#"
        document.getElementById('target').getAttribute('data-value')
    "#,
    );
    assert_eq!(result, RustValue::String("hello".into()));
}

#[test]
fn set_attribute_id_updates_index() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"old\"></div>");
    kernel.eval_to_rust_value(
        r#"
        var el = document.getElementById('old');
        el.setAttribute('id', 'new');
    "#,
    );
    // Should be findable by new id
    let result = kernel.eval_to_rust_value("document.getElementById('new').tagName");
    assert_eq!(result, RustValue::String("DIV".into()));
}

#[test]
fn append_child_returns_child() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"parent\"></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var parent = document.getElementById('parent');
        var child = document.createElement('span');
        var returned = parent.appendChild(child);
        returned.tagName
    "#,
    );
    assert_eq!(result, RustValue::String("SPAN".into()));
}
