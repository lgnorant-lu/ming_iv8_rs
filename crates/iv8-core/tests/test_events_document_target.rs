#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// Test L-03 fix: document is a real EventTarget.
//
// v0.1: addEventListener/dispatchEvent on document silently no-op'd because
// document had no NodeId.
//
// v0.2: document is bound to the DOM tree's root NodeId, so listeners attach
// to the EventListenerRegistry and dispatchEvent fires them.

use iv8_core::kernel::{EvalOpts, KernelConfig};
use iv8_core::EmbeddedV8Kernel;
#[test]
fn document_addeventlistener_is_real_function() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("typeof document.addEventListener");
    // Real function (not the document_props.js stub)
    assert_eq!(
        result,
        iv8_core::convert::RustValue::String("function".into())
    );
}

#[test]
fn document_dispatchevent_fires_listener() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = 0;
        document.addEventListener('foo', function(e) { fired += 1; });
        document.dispatchEvent({ type: 'foo', bubbles: false });
        fired
    "#,
    );
    assert_eq!(result, iv8_core::convert::RustValue::Int(1));
}

#[test]
fn document_dispatchevent_string_type_fires_listener() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        document.addEventListener('click', function() { fired = true; });
        document.dispatchEvent('click');
        fired
    "#,
    );
    assert_eq!(result, iv8_core::convert::RustValue::Bool(true));
}

#[test]
fn document_removeeventlistener_works() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var count = 0;
        var listener = function() { count += 1; };
        document.addEventListener('test', listener);
        document.dispatchEvent({ type: 'test' });
        document.removeEventListener('test', listener);
        document.dispatchEvent({ type: 'test' });
        count
    "#,
    );
    // After remove, second dispatch should not fire
    assert_eq!(result, iv8_core::convert::RustValue::Int(1));
}

#[test]
fn document_event_bubbles_from_child_to_document() {
    let mut kernel = common::make_kernel();
    kernel.set_document(
        r#"<html><body><div id="child"></div></body></html>"#,
        Some("https://example.com/"),
    );
    let result = kernel.eval_to_rust_value(
        r#"
        var caught = 'none';
        document.addEventListener('bubble-test', function(e) { caught = 'document'; });
        var child = document.getElementById('child');
        child.dispatchEvent({ type: 'bubble-test', bubbles: true });
        caught
    "#,
    );
    assert_eq!(
        result,
        iv8_core::convert::RustValue::String("document".into())
    );
}

#[test]
fn document_dom_content_loaded_pattern() {
    // Common reverse-engineering pattern: anti-bot scripts listen on document
    // for DOMContentLoaded.
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var loaded = false;
        document.addEventListener('DOMContentLoaded', function() {
            loaded = true;
        });
        // Simulate the DOMContentLoaded firing
        document.dispatchEvent({ type: 'DOMContentLoaded', bubbles: false });
        loaded
    "#,
    );
    assert_eq!(result, iv8_core::convert::RustValue::Bool(true));
}

#[test]
fn document_multiple_listeners_all_fire() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var calls = [];
        document.addEventListener('m', function() { calls.push('a'); });
        document.addEventListener('m', function() { calls.push('b'); });
        document.addEventListener('m', function() { calls.push('c'); });
        document.dispatchEvent({ type: 'm' });
        calls.join(',')
    "#,
    );
    assert_eq!(result, iv8_core::convert::RustValue::String("a,b,c".into()));
}

#[test]
fn document_once_option_listener_fires_only_once() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var count = 0;
        document.addEventListener('o', function() { count += 1; }, { once: true });
        document.dispatchEvent({ type: 'o' });
        document.dispatchEvent({ type: 'o' });
        document.dispatchEvent({ type: 'o' });
        count
    "#,
    );
    assert_eq!(result, iv8_core::convert::RustValue::Int(1));
}

#[test]
fn document_no_listener_dispatch_returns_true() {
    // dispatchEvent should return true when no listener prevents default.
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(r#"document.dispatchEvent({ type: 'never-listened' })"#);
    assert_eq!(result, iv8_core::convert::RustValue::Bool(true));
}
