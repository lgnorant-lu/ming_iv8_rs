#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

use iv8_core::RustValue;

// Simplified idlharness proto_throw test: accessing attribute on prototype
// must throw TypeError.
#[test]
fn proto_throw_abort_controller_signal() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(r#"
        try {
            AbortController.prototype.signal;
            "NO_THROW"
        } catch (e) {
            e instanceof TypeError ? "TYPE_ERROR" : "OTHER:" + e
        }
    "#);
    match result {
        RustValue::String(s) if s == "TYPE_ERROR" => {}
        other => panic!("Expected TYPE_ERROR, got {:?}", other),
    }
}

#[test]
fn proto_throw_abort_signal_aborted() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(r#"
        try {
            AbortSignal.prototype.aborted;
            "NO_THROW"
        } catch (e) {
            e instanceof TypeError ? "TYPE_ERROR" : "OTHER:" + e
        }
    "#);
    match result {
        RustValue::String(s) if s == "TYPE_ERROR" => {}
        other => panic!("Expected TYPE_ERROR, got {:?}", other),
    }
}

// this_throw: calling operation with this={} must throw TypeError
// Note: EventTarget methods are dom-native (install_proto_method_with_length),
// not codegen-generated. They don't have the prototype chain check.
// AbortController methods are codegen-generated and should throw.
#[test]
fn this_throw_abort_controller_abort() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(r#"
        try {
            AbortController.prototype.abort.call({});
            "NO_THROW"
        } catch (e) {
            e instanceof TypeError ? "TYPE_ERROR" : "OTHER:" + e
        }
    "#);
    match result {
        RustValue::String(s) if s == "TYPE_ERROR" => {}
        other => panic!("Expected TYPE_ERROR, got {:?}", other),
    }
}

// Normal usage must still work (no false positive)
#[test]
fn normal_usage_abort_controller_signal() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(r#"
        var ac = new AbortController();
        typeof ac.signal
    "#);
    assert_eq!(result, RustValue::String("object".into()));
}

#[test]
fn normal_usage_abort_controller_abort() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(r#"
        var ac = new AbortController();
        ac.abort();
        "OK"
    "#);
    assert_eq!(result, RustValue::String("OK".into()));
}

// null-this must still throw
#[test]
fn null_this_abort_controller_signal() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value(r#"
        try {
            AbortController.prototype.signal.call(null);
            "NO_THROW"
        } catch (e) {
            e instanceof TypeError ? "TYPE_ERROR" : "OTHER:" + e
        }
    "#);
    match result {
        RustValue::String(s) if s == "TYPE_ERROR" => {}
        other => panic!("Expected TYPE_ERROR, got {:?}", other),
    }
}

// DOM instance must still work (chain_dom_prototypes compatibility)
#[test]
fn dom_instance_tag_name_works() {
    let mut kernel = common::make_kernel_with_doc("<div id='test'>hello</div>");
    let result = kernel.eval_to_rust_value("document.getElementById('test').tagName");
    assert_eq!(result, RustValue::String("DIV".into()));
}

#[test]
fn dom_instance_get_attribute_works() {
    let mut kernel = common::make_kernel_with_doc("<div id='test' class='foo'>hello</div>");
    let result = kernel.eval_to_rust_value("document.getElementById('test').getAttribute('class')");
    assert_eq!(result, RustValue::String("foo".into()));
}

// Codegen accessor properties (baseURI, ownerDocument) must be visible
// in the DOM prototype chain after chain_dom_prototypes + reapply.
#[test]
fn codegen_attrs_visible_on_dom_prototype() {
    let mut kernel = common::make_kernel_with_doc("<div>hello</div>");
    let result = kernel.eval_to_rust_value(r#"
        (function() {
            var el = document.createElement('div');
            var results = [];
            results.push('baseURI' in el ? 'PASS' : 'FAIL:baseURI not in div');
            results.push('ownerDocument' in el ? 'PASS' : 'FAIL:ownerDocument not in div');
            results.push('appendChild' in el ? 'PASS' : 'FAIL:appendChild not in div');
            results.push('addEventListener' in el ? 'PASS' : 'FAIL:addEventListener not in div');
            return results.join(';');
        })()
    "#);
    match &result {
        RustValue::String(s) => {
            for part in s.split(';') {
                if part.starts_with("FAIL") {
                    panic!("{}", part);
                }
            }
        }
        other => panic!("Expected string, got {:?}", other),
    }
}

// DOM prototype access must throw — but only for codegen-generated attributes.
// Dom-native attributes (tagName, parentNode, etc.) use run_accessor which
// doesn't have receiver check yet. This is a known limitation.
// Element.prototype.tagName is dom-native, so it won't throw (yet).
// Document.prototype.documentElement is codegen-generated, so it should throw.
#[test]
fn dom_proto_throw_document_element() {
    let mut kernel = common::make_kernel_with_doc("<div>hello</div>");
    let result = kernel.eval_to_rust_value(r#"
        try {
            Document.prototype.documentElement;
            "NO_THROW"
        } catch (e) {
            e instanceof TypeError ? "TYPE_ERROR" : "OTHER:" + e
        }
    "#);
    match result {
        RustValue::String(s) if s == "TYPE_ERROR" => {}
        other => panic!("Expected TYPE_ERROR, got {:?}", other),
    }
}
