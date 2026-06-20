#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// Integration tests for EventTarget (Task 33).
// Tests addEventListener, dispatchEvent, and three-phase dispatch.

use iv8_core::RustValue;

#[test]
fn add_event_listener_and_dispatch() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"target\">hello</div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        var el = document.getElementById('target');
        el.addEventListener('click', function() { fired = true; });
        el.dispatchEvent({type: 'click', bubbles: false});
        fired
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn event_listener_receives_event_object() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"target\"></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var eventType = '';
        var el = document.getElementById('target');
        el.addEventListener('custom', function(e) { eventType = e.type; });
        el.dispatchEvent({type: 'custom', bubbles: false});
        eventType
    "#,
    );
    assert_eq!(result, RustValue::String("custom".into()));
}

#[test]
fn multiple_listeners_same_event() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"target\"></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var count = 0;
        var el = document.getElementById('target');
        el.addEventListener('click', function() { count++; });
        el.addEventListener('click', function() { count++; });
        el.dispatchEvent({type: 'click', bubbles: false});
        count
    "#,
    );
    assert_eq!(result, RustValue::Int(2));
}

#[test]
fn listener_not_fired_for_different_event() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"target\"></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        var el = document.getElementById('target');
        el.addEventListener('click', function() { fired = true; });
        el.dispatchEvent({type: 'mouseover', bubbles: false});
        fired
    "#,
    );
    assert_eq!(result, RustValue::Bool(false));
}

#[test]
fn once_listener_fires_only_once() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"target\"></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var count = 0;
        var el = document.getElementById('target');
        el.addEventListener('click', function() { count++; }, {once: true});
        el.dispatchEvent({type: 'click', bubbles: false});
        el.dispatchEvent({type: 'click', bubbles: false});
        count
    "#,
    );
    assert_eq!(result, RustValue::Int(1));
}

#[test]
fn dispatch_event_returns_true() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"target\"></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var el = document.getElementById('target');
        el.dispatchEvent({type: 'click', bubbles: false})
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn add_event_listener_exists_on_elements() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"target\"></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var el = document.getElementById('target');
        typeof el.addEventListener === 'function' && typeof el.dispatchEvent === 'function'
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn dispatch_string_event() {
    let mut kernel = common::make_kernel_with_doc("<div id=\"target\"></div>");
    let result = kernel.eval_to_rust_value(
        r#"
        var fired = false;
        var el = document.getElementById('target');
        el.addEventListener('test', function() { fired = true; });
        el.dispatchEvent('test');
        fired
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}
