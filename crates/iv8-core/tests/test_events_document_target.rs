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

// ─── v0.8.66 (M3): DOMContentLoaded/load automatic dispatch during page_load ─────────

#[test]
fn page_load_dispatches_dom_content_loaded() {
    let mut kernel = common::make_kernel();
    kernel
        .eval_to_rust_value(
            r#"
        window.__dclFired = false;
        document.addEventListener('DOMContentLoaded', function() {
            window.__dclFired = true;
        });
    "#,
        );
    kernel.page_load("<html></html>", None);
    let result = kernel.eval_to_rust_value("window.__dclFired");
    assert_eq!(result, iv8_core::convert::RustValue::Bool(true));
}

#[test]
fn page_load_dispatches_load_after_dom_content_loaded() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        window.__order = [];
        document.addEventListener('DOMContentLoaded', function() {
            window.__order.push('dcl');
        });
        document.addEventListener('load', function() {
            window.__order.push('load');
        });
    "#,
    );
    kernel.page_load("<html></html>", None);
    let result = kernel.eval_to_rust_value("window.__order");
    assert_eq!(
        result,
        iv8_core::convert::RustValue::Array(vec![
            iv8_core::convert::RustValue::String("dcl".into()),
            iv8_core::convert::RustValue::String("load".into()),
        ])
    );
}

#[test]
fn page_load_sets_ready_state_complete() {
    let mut kernel = common::make_kernel();
    kernel.page_load("<html></html>", None);
    let result = kernel.eval_to_rust_value("document.readyState");
    assert_eq!(
        result,
        iv8_core::convert::RustValue::String("complete".into())
    );
}

#[test]
fn page_load_dom_content_loaded_event_properties() {
    let mut kernel = common::make_kernel();
    kernel.eval_to_rust_value(
        r#"
        window.__evtType = null;
        window.__evtTrusted = null;
        document.addEventListener('DOMContentLoaded', function(e) {
            window.__evtType = e.type;
            window.__evtTrusted = e.isTrusted;
        });
    "#,
    );
    kernel.page_load("<html></html>", None);
    let evt_type = kernel.eval_to_rust_value("window.__evtType");
    let evt_trusted = kernel.eval_to_rust_value("window.__evtTrusted");
    assert_eq!(
        evt_type,
        iv8_core::convert::RustValue::String("DOMContentLoaded".into())
    );
    assert_eq!(evt_trusted, iv8_core::convert::RustValue::Bool(true));
}
