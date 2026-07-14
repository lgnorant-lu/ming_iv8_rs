//! v0.8.98 S6: high-signal behavior/consistency (beyond typeof-only).
mod common;

use iv8_core::RustValue;

#[test]
fn event_phase_constants_on_ctor_and_prototype() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "String(Event.NONE)", "0");
    common::assert_js_str(&mut k, "String(Event.CAPTURING_PHASE)", "1");
    common::assert_js_str(&mut k, "String(Event.AT_TARGET)", "2");
    common::assert_js_str(&mut k, "String(Event.BUBBLING_PHASE)", "3");
    common::assert_js_str(
        &mut k,
        "String(Object.prototype.hasOwnProperty.call(Event.prototype,'NONE'))",
        "true",
    );
    common::assert_js_str(
        &mut k,
        "String(Event.prototype.CAPTURING_PHASE === Event.CAPTURING_PHASE)",
        "true",
    );
}

#[test]
fn event_target_is_constructable_and_dom_inherits() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "(function(){ try { return typeof new EventTarget(); } catch(e) { return e.name; } })()",
        "object",
    );
    common::assert_js_str(
        &mut k,
        "String(document.createElement('div') instanceof EventTarget)",
        "true",
    );
    common::assert_js_str(
        &mut k,
        "String(document instanceof Node && document instanceof EventTarget)",
        "true",
    );
}

#[test]
fn navigator_permission_geo_battery_connection_shape() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof navigator.permissions", "object");
    common::assert_js_str(&mut k, "typeof navigator.permissions.query", "function");
    common::assert_js_str(&mut k, "typeof navigator.geolocation", "object");
    common::assert_js_str(
        &mut k,
        "typeof navigator.geolocation.getCurrentPosition",
        "function",
    );
    common::assert_js_str(&mut k, "typeof navigator.getBattery", "function");
    common::assert_js_str(&mut k, "typeof navigator.connection", "object");
    // connection effectiveType is string-ish when present
    let et = k.eval_to_rust_value("typeof navigator.connection.effectiveType");
    assert!(
        matches!(et, RustValue::String(ref s) if s == "string" || s == "undefined"),
        "effectiveType type: {:?}",
        et
    );
}

#[test]
fn canvas_webgl_unmasked_and_audio_sample_rate_consistency() {
    let mut k = common::make_kernel();
    // WebGL parameter path
    common::assert_js_str(
        &mut k,
        r#"(function(){
            var c=document.createElement('canvas');
            var g=c.getContext('webgl')||c.getContext('experimental-webgl');
            if(!g) return 'no-webgl';
            var ext=g.getExtension('WEBGL_debug_renderer_info');
            if(!ext) return 'no-ext';
            var v=g.getParameter(ext.UNMASKED_VENDOR_WEBGL);
            return (typeof v==='string' && v.length>0) ? 'ok' : 'bad';
        })()"#,
        "ok",
    );
    common::assert_js_str(
        &mut k,
        r#"(function(){
            var a=new AudioContext();
            return (typeof a.sampleRate==='number' && a.sampleRate>0) ? 'ok' : 'bad';
        })()"#,
        "ok",
    );
    common::assert_js_str(
        &mut k,
        r#"(function(){
            var c=document.createElement('canvas');
            var x=c.getContext('2d');
            x.fillStyle='#f00';
            x.fillRect(0,0,1,1);
            var d=c.toDataURL();
            return (typeof d==='string' && d.indexOf('data:image')===0) ? 'ok' : 'bad';
        })()"#,
        "ok",
    );
}

#[test]
fn storage_and_crypto_subtle_shape() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof localStorage.setItem", "function");
    common::assert_js_str(&mut k, "typeof sessionStorage.getItem", "function");
    common::assert_js_str(&mut k, "typeof crypto.subtle", "object");
    common::assert_js_str(&mut k, "typeof crypto.subtle.digest", "function");
    common::assert_js_str(&mut k, "typeof crypto.getRandomValues", "function");
}

#[test]
fn node_constants_on_ctor_and_prototype() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "String(Node.ELEMENT_NODE)", "1");
    common::assert_js_str(
        &mut k,
        "String(Object.prototype.hasOwnProperty.call(Node.prototype,'ELEMENT_NODE'))",
        "true",
    );
    common::assert_js_str(
        &mut k,
        "String(Node.prototype.TEXT_NODE === Node.TEXT_NODE && Node.TEXT_NODE === 3)",
        "true",
    );
}

#[test]
fn event_type_accessor_and_customevent_inheritance() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "String(typeof Object.getOwnPropertyDescriptor(Event.prototype,'type').get)",
        "function",
    );
    common::assert_js_str(
        &mut k,
        "String(Object.getPrototypeOf(CustomEvent.prototype) === Event.prototype)",
        "true",
    );
    common::assert_js_str(
        &mut k,
        "String(Object.getPrototypeOf(Node.prototype) === EventTarget.prototype)",
        "true",
    );
}
