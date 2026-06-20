#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// Integration tests for page.load (Task 35).
// Tests HTML parsing + inline script execution + DOM availability.

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};
#[test]
fn page_load_basic_html() {
    let mut kernel = common::make_kernel();
    kernel.page_load(
        "<html><body><div id=\"app\">Hello</div></body></html>",
        None,
    );
    let result = kernel.eval_to_rust_value("document.getElementById('app').textContent");
    assert_eq!(result, RustValue::String("Hello".into()));
}

#[test]
fn page_load_executes_inline_script() {
    let mut kernel = common::make_kernel();
    kernel.page_load(
        "<html><body><script>globalThis.loaded = true;</script></body></html>",
        None,
    );
    let result = kernel.eval_to_rust_value("globalThis.loaded");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn page_load_multiple_scripts_in_order() {
    let mut kernel = common::make_kernel();
    kernel.page_load(
        r#"<html><body>
            <script>globalThis.order = [];</script>
            <script>globalThis.order.push(1);</script>
            <script>globalThis.order.push(2);</script>
        </body></html>"#,
        None,
    );
    let result = kernel.eval_to_rust_value("globalThis.order");
    assert_eq!(
        result,
        RustValue::Array(vec![RustValue::Int(1), RustValue::Int(2)])
    );
}

#[test]
fn page_load_script_can_access_dom() {
    let mut kernel = common::make_kernel();
    kernel.page_load(
        r#"<html><body>
            <div id="target">original</div>
            <script>
                var el = document.getElementById('target');
                globalThis.found = el ? el.textContent : 'not found';
            </script>
        </body></html>"#,
        None,
    );
    let result = kernel.eval_to_rust_value("globalThis.found");
    assert_eq!(result, RustValue::String("original".into()));
}

#[test]
fn page_load_script_error_does_not_abort() {
    let mut kernel = common::make_kernel();
    kernel.page_load(
        r#"<html><body>
            <script>throw new Error('oops');</script>
            <script>globalThis.afterError = true;</script>
        </body></html>"#,
        None,
    );
    // Second script should still execute despite first throwing
    let result = kernel.eval_to_rust_value("globalThis.afterError");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn page_load_with_base_url() {
    let mut kernel = common::make_kernel();
    kernel.page_load("<html></html>", Some("https://example.com/page"));
    // Document should have the base URL (not directly testable from JS in v0.1,
    // but we can verify it doesn't crash)
    let result = kernel.eval_to_rust_value("typeof document");
    assert_eq!(result, RustValue::String("object".into()));
}

#[test]
fn page_load_empty_html() {
    let mut kernel = common::make_kernel();
    kernel.page_load("", None);
    // html5ever creates html/head/body even for empty input
    let result = kernel.eval_to_rust_value("document.querySelector('body') !== null");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn page_load_complex_structure() {
    let mut kernel = common::make_kernel();
    kernel.page_load(
        r#"<!DOCTYPE html>
        <html>
        <head><title>Test Page</title></head>
        <body>
            <header><nav><a href="/">Home</a></nav></header>
            <main>
                <article id="post">
                    <h1>Title</h1>
                    <p class="content">Paragraph 1</p>
                    <p class="content">Paragraph 2</p>
                </article>
            </main>
            <script>
                globalThis.paragraphs = document.querySelectorAll('.content').length;
                globalThis.title = document.querySelector('h1').textContent;
            </script>
        </body>
        </html>"#,
        None,
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.paragraphs"),
        RustValue::Int(2)
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.title"),
        RustValue::String("Title".into())
    );
}

#[test]
fn page_load_set_timeout_in_script() {
    let mut kernel = common::make_kernel();
    kernel.page_load(
        r#"<html><body><script>
            globalThis.timerFired = false;
            setTimeout(function() { globalThis.timerFired = true; }, 100);
        </script></body></html>"#,
        None,
    );
    // Timer hasn't fired yet
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.timerFired"),
        RustValue::Bool(false)
    );
    // Advance time
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(200)");
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.timerFired"),
        RustValue::Bool(true)
    );
}
