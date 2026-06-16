#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]

//! Integration tests for MessageChannel (Task 77).
//! Acceptance criteria:
//! - new MessageChannel() creates port1 + port2
//! - port1.postMessage(data) → port2.onmessage receives {data}
//! - Communication is async (via setTimeout)

use iv8_core::{EmbeddedV8Kernel, KernelConfig, RustValue};

fn make_kernel() -> EmbeddedV8Kernel {
    EmbeddedV8Kernel::new(KernelConfig::default()).unwrap()
}

#[test]
fn message_channel_exists() {
    let mut kernel = make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof MessageChannel"),
        RustValue::String("function".into())
    );
}

#[test]
fn message_channel_has_ports() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var mc = new MessageChannel();
        typeof mc.port1 === 'object' && typeof mc.port2 === 'object'
    "#,
    );
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn message_channel_post_message() {
    let mut kernel = make_kernel();
    kernel.eval_to_rust_value(
        r#"
        var mc = new MessageChannel();
        globalThis.received = null;
        mc.port2.onmessage = function(e) { globalThis.received = e.data; };
        mc.port1.postMessage('hello');
    "#,
    );
    // Message is async (setTimeout 0), need to advance
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(10)");
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.received"),
        RustValue::String("hello".into())
    );
}

#[test]
fn message_channel_bidirectional() {
    let mut kernel = make_kernel();
    kernel.eval_to_rust_value(
        r#"
        var mc = new MessageChannel();
        globalThis.msg1 = null;
        globalThis.msg2 = null;
        mc.port1.onmessage = function(e) { globalThis.msg1 = e.data; };
        mc.port2.onmessage = function(e) { globalThis.msg2 = e.data; };
        mc.port1.postMessage('from1');
        mc.port2.postMessage('from2');
    "#,
    );
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(10)");
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.msg1"),
        RustValue::String("from2".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.msg2"),
        RustValue::String("from1".into())
    );
}

#[test]
fn message_channel_object_data() {
    let mut kernel = make_kernel();
    kernel.eval_to_rust_value(
        r#"
        var mc = new MessageChannel();
        globalThis.receivedObj = null;
        mc.port2.onmessage = function(e) { globalThis.receivedObj = e.data; };
        mc.port1.postMessage({key: 'value', num: 42});
    "#,
    );
    kernel.eval_to_rust_value("__iv8__.eventLoop.advance(10)");
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.receivedObj.key"),
        RustValue::String("value".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.receivedObj.num"),
        RustValue::Int(42)
    );
}

#[test]
fn message_channel_async_not_immediate() {
    let mut kernel = make_kernel();
    let result = kernel.eval_to_rust_value(
        r#"
        var mc = new MessageChannel();
        globalThis.immediate = null;
        mc.port2.onmessage = function(e) { globalThis.immediate = e.data; };
        mc.port1.postMessage('test');
        globalThis.immediate  // Should be null (not yet delivered)
    "#,
    );
    assert_eq!(result, RustValue::Null);
}
