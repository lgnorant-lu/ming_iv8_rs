#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;


// Integration tests for innerHTML/outerHTML/textContent (Task 66).
// Acceptance criteria:
// - element.innerHTML returns child HTML string
// - element.outerHTML includes self tag
// - element.textContent = 'x' clears children and sets text

use iv8_core::RustValue;

#[test]
fn inner_html_basic() {
    let mut kernel = common::make_kernel_with_doc("<div id='x'><p>hello</p></div>");
    let result = kernel.eval_to_rust_value("document.getElementById('x').innerHTML");
    assert_eq!(result, RustValue::String("<p>hello</p>".into()));
}

#[test]
fn inner_html_nested() {
    let mut kernel = common::make_kernel_with_doc("<div id='x'><ul><li>1</li><li>2</li></ul></div>");
    let result = kernel.eval_to_rust_value("document.getElementById('x').innerHTML");
    assert_eq!(
        result,
        RustValue::String("<ul><li>1</li><li>2</li></ul>".into())
    );
}

#[test]
fn inner_html_with_attributes() {
    let mut kernel =
        common::make_kernel_with_doc("<div id='x'><a href=\"/link\" class=\"btn\">click</a></div>");
    let result = kernel.eval_to_rust_value("document.getElementById('x').innerHTML");
    match result {
        RustValue::String(s) => {
            assert!(s.contains("href=\"/link\""), "html: {}", s);
            assert!(s.contains("class=\"btn\""), "html: {}", s);
            assert!(s.contains("click"), "html: {}", s);
        }
        other => panic!("expected String, got: {:?}", other),
    }
}

#[test]
fn inner_html_empty() {
    let mut kernel = common::make_kernel_with_doc("<div id='x'></div>");
    let result = kernel.eval_to_rust_value("document.getElementById('x').innerHTML");
    assert_eq!(result, RustValue::String("".into()));
}

#[test]
fn outer_html_basic() {
    let mut kernel = common::make_kernel_with_doc("<div id='x'><p>hi</p></div>");
    let result = kernel.eval_to_rust_value("document.getElementById('x').outerHTML");
    assert_eq!(
        result,
        RustValue::String("<div id=\"x\"><p>hi</p></div>".into())
    );
}

#[test]
fn text_content_setter() {
    let mut kernel = common::make_kernel_with_doc("<div id='x'><p>old</p><span>stuff</span></div>");
    kernel.eval_to_rust_value("document.getElementById('x').textContent = 'new text'");
    // After setting textContent, innerHTML should just be the text (escaped)
    let result = kernel.eval_to_rust_value("document.getElementById('x').innerHTML");
    assert_eq!(result, RustValue::String("new text".into()));
}

#[test]
fn text_content_setter_clears_children() {
    let mut kernel = common::make_kernel_with_doc("<ul id='list'><li>1</li><li>2</li><li>3</li></ul>");
    kernel.eval_to_rust_value("document.getElementById('list').textContent = 'cleared'");
    let result = kernel.eval_to_rust_value("document.querySelectorAll('li').length");
    assert_eq!(result, RustValue::Int(0));
}

#[test]
fn inner_html_void_elements() {
    let mut kernel = common::make_kernel_with_doc("<div id='x'><br><img src=\"a.png\"></div>");
    let result = kernel.eval_to_rust_value("document.getElementById('x').innerHTML");
    match result {
        RustValue::String(s) => {
            assert!(s.contains("<br>"), "html: {}", s);
            assert!(
                !s.contains("</br>"),
                "void element should not have closing tag: {}",
                s
            );
        }
        other => panic!("expected String, got: {:?}", other),
    }
}
