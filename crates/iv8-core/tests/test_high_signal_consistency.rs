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
fn high_signal_value_consistency_nav_screen_plugins() {
    let mut k = common::make_kernel();
    // Soft identity consistency: non-empty strings, plugins length number, uaData brands array-ish
    common::assert_js_str(
        &mut k,
        r#"(function(){
            if (typeof navigator.userAgent !== 'string' || !navigator.userAgent) return 'ua';
            if (typeof navigator.platform !== 'string' || !navigator.platform) return 'plat';
            if (typeof screen.width !== 'number' || screen.width <= 0) return 'sw';
            if (typeof screen.height !== 'number' || screen.height <= 0) return 'sh';
            if (typeof navigator.plugins === 'undefined') return 'plugins-missing';
            if (typeof navigator.plugins.length !== 'number') return 'plugins-len';
            if (navigator.userAgentData) {
                var b = navigator.userAgentData.brands;
                if (b && typeof b.length !== 'number') return 'brands';
            }
            return 'ok';
        })()"#,
        "ok",
    );
}

#[test]
fn html_all_collection_named_item_name_and_receiver() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        r#"(function(){
            var p = HTMLAllCollection && HTMLAllCollection.prototype;
            if (!p || typeof p.namedItem !== 'function') return 'no-fn';
            if (p.namedItem.name !== 'namedItem') return 'name:' + p.namedItem.name;
            try { p.namedItem.call(p, 'x'); return 'no-throw-proto'; } catch (e) {
                return (e && e.name === 'TypeError') ? 'ok' : ('throw:' + (e && e.name));
            }
        })()"#,
        "ok",
    );
}

#[test]
fn permissions_query_returns_promise_like() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value(
        r#"
        globalThis._pq = null;
        globalThis._pqErr = null;
        try {
            var p = navigator.permissions.query({name:'notifications'});
            globalThis._pq = p && typeof p.then === 'function';
            if (p && p.then) {
                p.then(function(r){ globalThis._pqState = r && r.state; })
                 .catch(function(e){ globalThis._pqErr = String(e); });
            }
        } catch (e) {
            globalThis._pqErr = String(e);
        }
        "#,
    );
    for _ in 0..8 {
        k.drain_microtasks();
    }
    common::assert_js_str(&mut k, "String(globalThis._pq)", "true");
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
fn event_istrusted_is_own_accessor_not_data() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        r#"(function(){
            var e = new Event('x');
            var d = Object.getOwnPropertyDescriptor(e, 'isTrusted');
            if (!d) return 'no-own';
            if (typeof d.get !== 'function') return 'not-getter:' + JSON.stringify(d);
            if ('value' in d) return 'has-value';
            if (d.get.name !== 'get isTrusted') return 'name:' + d.get.name;
            try { d.get.call({}); return 'no-throw'; } catch (err) {
                if (err && err.name === 'TypeError') return 'ok';
                return 'throw:' + (err && err.name);
            }
        })()"#,
        "ok",
    );
}

#[test]
fn worker_constructor_length_is_one() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "String(Worker.length)", "1");
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
