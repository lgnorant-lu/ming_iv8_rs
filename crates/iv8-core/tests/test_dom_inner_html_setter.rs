#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]

//! Integration tests for innerHTML setter (Task 79).
//! Acceptance criteria:
//! - element.innerHTML = '<p>new</p>' replaces children
//! - document.documentElement.innerHTML = html (h5st pattern)
//! - After setting, querySelector finds new elements
//! - After setting, old elements are gone

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};

fn make_kernel() -> EmbeddedV8Kernel {
    EmbeddedV8Kernel::new(KernelConfig::default()).unwrap()
}

fn make_kernel_with_doc(html: &str) -> EmbeddedV8Kernel {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
    kernel.set_document(html, None);
    kernel
}

#[test]
fn inner_html_setter_basic() {
    let mut kernel = make_kernel_with_doc("<div id='target'><p>old</p></div>");
    kernel.eval_to_rust_value(
        r#"
        document.getElementById('target').innerHTML = '<span>new</span>';
    "#,
    );
    let result = kernel.eval_to_rust_value("document.querySelector('#target span').textContent");
    assert_eq!(result, RustValue::String("new".into()));
}

#[test]
fn inner_html_setter_clears_old() {
    let mut kernel = make_kernel_with_doc("<div id='x'><p>1</p><p>2</p><p>3</p></div>");
    kernel.eval_to_rust_value("document.getElementById('x').innerHTML = '<span>only</span>'");
    assert_eq!(
        kernel.eval_to_rust_value("document.querySelectorAll('#x p').length"),
        RustValue::Int(0)
    );
    assert_eq!(
        kernel.eval_to_rust_value("document.querySelectorAll('#x span').length"),
        RustValue::Int(1)
    );
}

#[test]
fn inner_html_setter_empty_string() {
    let mut kernel = make_kernel_with_doc("<div id='x'><p>content</p></div>");
    kernel.eval_to_rust_value("document.getElementById('x').innerHTML = ''");
    assert_eq!(
        kernel.eval_to_rust_value("document.getElementById('x').innerHTML"),
        RustValue::String("".into())
    );
}

#[test]
fn inner_html_setter_complex_html() {
    let mut kernel = make_kernel_with_doc("<div id='root'></div>");
    kernel.eval_to_rust_value(
        r#"
        document.getElementById('root').innerHTML = '<ul><li id="a">A</li><li id="b">B</li></ul>';
    "#,
    );
    assert_eq!(
        kernel.eval_to_rust_value("document.getElementById('a').textContent"),
        RustValue::String("A".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("document.getElementById('b').textContent"),
        RustValue::String("B".into())
    );
}

#[test]
fn inner_html_setter_document_element() {
    // h5st pattern: document.documentElement.innerHTML = full_html
    let mut kernel = make_kernel();
    kernel.eval_to_rust_value(r#"
        document.querySelector('html').innerHTML = '<head><title>Test</title></head><body><div id="app">loaded</div></body>';
    "#);
    let result = kernel.eval_to_rust_value("document.getElementById('app').textContent");
    assert_eq!(result, RustValue::String("loaded".into()));
}

#[test]
fn inner_html_setter_with_attributes() {
    let mut kernel = make_kernel_with_doc("<div id='x'></div>");
    kernel.eval_to_rust_value(r#"
        document.getElementById('x').innerHTML = '<a href="https://test.com" class="link">click</a>';
    "#);
    assert_eq!(
        kernel.eval_to_rust_value("document.querySelector('.link').getAttribute('href')"),
        RustValue::String("https://test.com".into())
    );
}
