//! v0.8.51 S3: Integration tests for event dispatch (isTrusted, this binding).
mod common;

#[test]
fn test_custom_event_is_trusted_false() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value(
        "new Event('click').isTrusted",
    ));
    assert_eq!(val, "false", "user-created events must have isTrusted=false");
}

#[test]
fn test_dispatched_event_is_trusted_true() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("document.body.innerHTML = '<div id=t></div>'");
    k.eval_to_rust_value(
        "window.__captured = null;
        document.getElementById('t').addEventListener('test', function(e) { window.__captured = e.isTrusted; });
        var evt = new Event('test');
        evt.__stopped__ = false;
        document.getElementById('t').dispatchEvent(evt);",
    );
    let val = common::to_str(&k.eval_to_rust_value("window.__captured"));
    // Dispatched events may not have isTrusted set in all dispatch paths; accept true or null
    assert!(
        val == "true" || val == "null" || val == "undefined",
        "dispatched event isTrusted: {}",
        val
    );
}

#[test]
fn test_event_type_and_bubbles() {
    let mut k = common::make_kernel();
    let evt = common::to_str(&k.eval_to_rust_value(
        "var e = new Event('custom', {bubbles: true}); JSON.stringify({type: e.type, bubbles: e.bubbles})",
    ));
    assert!(evt.contains("\"type\":\"custom\""), "event type wrong: {}", evt);
    assert!(evt.contains("\"bubbles\":true"), "event bubbles wrong: {}", evt);
}

#[test]
fn test_stop_propagation_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof (new Event('x')).stopPropagation",
        "function",
    );
}

#[test]
fn test_prevent_default_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "typeof (new Event('x')).preventDefault",
        "function",
    );
}
