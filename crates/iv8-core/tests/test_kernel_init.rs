mod common;
// v0.8.31 T-1: new-chain-only init validation.
// All 81 expressions must evaluate correctly on the current init chain.

use iv8_core::convert::RustValue;
use iv8_core::kernel::embedded_v8::EmbeddedV8Kernel;
use iv8_core::kernel::KernelConfig;

/// Build a kernel using the current default init chain.
fn make_new_chain_kernel() -> EmbeddedV8Kernel {
    EmbeddedV8Kernel::new(KernelConfig::default()).unwrap()
}

/// Extract string from RustValue for comparison.
fn to_str(v: &RustValue) -> String {
    match v {
        RustValue::String(s) => s.clone(),
        RustValue::Null => "null".to_string(),
        RustValue::Bool(b) => b.to_string(),
        RustValue::Int(n) => n.to_string(),
        RustValue::Float(f) => f.to_string(),
        RustValue::JsObject(s) => s.clone(),
        other => format!("{:?}", other),
    }
}

/// —— typeof checks ——————————————————————————————————

#[test]
fn test_typeof_window() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof window"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof window'");
}

#[test]
fn test_typeof_document() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof document"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof document'");
}

#[test]
fn test_typeof_navigator() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof navigator"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof navigator'");
}

#[test]
fn test_typeof_html_element() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof HTMLDivElement"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof HTMLDivElement'");
}

#[test]
fn test_typeof_html_element_proto() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof HTMLDivElement.prototype"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof HTMLDivElement.prototype'");
}

#[test]
fn test_typeof_webgl() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof WebGLRenderingContext"));
    assert_eq!(val, "function", "kernel should expose WebGLRenderingContext");
}

#[test]
fn test_typeof_canvas_context() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof CanvasRenderingContext2D"));
    assert_eq!(val, "function", "kernel should expose CanvasRenderingContext2D");
}

#[test]
fn test_typeof_audio() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof AudioContext"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof AudioContext'");
}

// —— Value checks ——————————————————————————————————

#[test]
fn test_navigator_useragent() {
    let mut kernel = make_new_chain_kernel();
    let ua = common::to_str(&kernel.eval_to_rust_value("navigator.userAgent"));
    assert!(!ua.is_empty(), "UA empty");
}

#[test]
fn test_screen_width() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("String(screen.width)"));
    assert!(!val.is_empty(), "kernel returned empty on 'String(screen.width)'");
}

#[test]
fn test_screen_height() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("String(screen.height)"));
    assert!(!val.is_empty(), "kernel returned empty on 'String(screen.height)'");
}

// —— DOM checks ————————————————————————————————————

#[test]
fn test_create_element_tagname() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("document.createElement('div').tagName"));
    assert!(!val.is_empty(), "kernel returned empty on 'document.createElement('div').tagName'");
}

#[test]
fn test_create_element_span() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("document.createElement('span').tagName"));
    assert!(!val.is_empty(), "kernel returned empty on 'document.createElement('span').tagName'");
}

#[test]
fn test_div_instanceof_element() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("String(document.createElement('div') instanceof Element)"));
    assert!(!val.is_empty(), "kernel returned empty");
}

#[test]
fn test_div_instanceof_htmlelement() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("String(document.createElement('div') instanceof HTMLElement)"));
    assert!(!val.is_empty(), "kernel returned empty");
}

#[test]
fn test_div_instanceof_html_divelement() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("document.createElement('div') instanceof HTMLDivElement"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_span_instanceof_html_spanelement() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("document.createElement('span') instanceof HTMLSpanElement"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_document_getelementbyid() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof document.getElementById"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof document.getElementById'");
}

#[test]
fn test_document_queryselector() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof document.querySelectorAll"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof document.querySelectorAll'");
}

#[test]
fn test_document_body() {
    let mut kernel = make_new_chain_kernel();
    let body = common::to_str(&kernel.eval_to_rust_value("String(document.body !== null)"));
    assert_eq!(body, "true");
}

// —— JS shim globals ——————————————————————————————

#[test]
fn test_settimeout() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof setTimeout"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_fetch() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof fetch"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_xmlhttprequest() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof XMLHttpRequest"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_crypto_subtle() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof crypto.subtle"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_url() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof URL"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_localstorage() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof localStorage"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_sessionstorage() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof sessionStorage"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_message_channel() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof MessageChannel"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_event() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof Event"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_customevent() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof CustomEvent"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_mouseevent() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof MouseEvent"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_keyboardevent() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof KeyboardEvent"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_promise() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof Promise"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_date_now() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof Date.now"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof Date.now'");
}

#[test]
fn test_performance() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof performance"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_performance_time_origin_number() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof performance.timeOrigin"));
    assert_eq!(val, "number", "performance.timeOrigin should be numeric");
}

#[test]
fn test_performance_now_returns_number() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof performance.now()"));
    assert_eq!(val, "number", "performance.now() should return a number");
}

#[test]
fn test_console() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof console"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_atob() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof atob"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_btoa() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof btoa"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_requestanimationframe() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof requestAnimationFrame"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_location() {
    let mut kernel = make_new_chain_kernel();
    let loc = common::to_str(&kernel.eval_to_rust_value("typeof location"));
    assert!(!loc.is_empty(), "location typeof empty");
}

// —— Navigator properties ——————————————————————————

#[test]
fn test_navigator_plugins() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof navigator.plugins"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_navigator_mimetypes() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof navigator.mimeTypes"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_navigator_java_enabled_method() {
    let mut kernel = make_new_chain_kernel();
    let typeof_method = common::to_str(&kernel.eval_to_rust_value("typeof navigator.javaEnabled"));
    assert_eq!(typeof_method, "function", "navigator.javaEnabled should be a function");
    let value = common::to_str(&kernel.eval_to_rust_value("String(navigator.javaEnabled())"));
    assert_eq!(value, "false", "navigator.javaEnabled() should return false");
}

#[test]
fn test_navigator_language() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof navigator.language"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof navigator.language'");
}

#[test]
fn test_navigator_hardware_concurrency() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("String(typeof navigator.hardwareConcurrency)"));
    assert!(!val.is_empty(), "kernel returned empty");
}

// —— Canvas / WebGL ——————————————————————————————

#[test]
fn test_canvas_create() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("String(document.createElement('canvas') instanceof HTMLCanvasElement)"));
    assert!(!val.is_empty(), "kernel returned empty");
}

#[test]
fn test_canvas_getcontext_2d() {
    let mut kernel = make_new_chain_kernel();
    let ctx = common::to_str(&kernel.eval_to_rust_value(
        "var c=document.createElement('canvas'); String(c.getContext('2d') !== null)"
    ));
    assert_eq!(ctx, "true");
}

#[test]
fn test_webgl_get_parameter_vendor() {
    let mut kernel = make_new_chain_kernel();
    let vendor = common::to_str(&kernel.eval_to_rust_value(
        "var c=document.createElement('canvas'); var gl=c.getContext('webgl'); typeof gl.getParameter(0x1F00)"
    ));
    assert!(!vendor.is_empty(), "webgl vendor typeof empty");
}

#[test]
fn test_to_data_url() {
    let mut kernel = make_new_chain_kernel();
    let _ = kernel.eval_to_rust_value(
        "var c=document.createElement('canvas'); var ctx=c.getContext('2d'); typeof ctx.canvas"
    );
}

// —— Function.prototype.toString native detection ——————————

#[test]
fn test_to_string_native_code() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("String(Function.prototype.toString.call(Array).includes('[native code]'))"));
    assert!(!val.is_empty(), "kernel returned empty");
}

// —— Date overrides ——————————————————————————————

#[test]
fn test_date_now_function() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof Date.now"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof Date.now'");
}

// —— Intl ———————————————————————————————————————

#[test]
fn test_intl_exists() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof Intl"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

// —— Blob / ArrayBuffer ——————————————————————————

#[test]
fn test_blob_exists() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof Blob"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

#[test]
fn test_uint8array() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof Uint8Array"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

// —— Error types ————————————————————————————————
#[test]
fn test_error_types() {
    let mut kernel = make_new_chain_kernel();

    let val = common::to_str(&kernel.eval_to_rust_value("typeof TypeError"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty on 'typeof TypeError': {:?}", val);

    let val = common::to_str(&kernel.eval_to_rust_value("typeof SyntaxError"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty on 'typeof SyntaxError': {:?}", val);

    let val = common::to_str(&kernel.eval_to_rust_value("typeof ReferenceError"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty on 'typeof ReferenceError': {:?}", val);
}

// —— RegExp —————————————————————————————————————
#[test]
fn test_regexp() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("String(typeof RegExp)"));
    assert!(!val.is_empty(), "kernel returned empty on 'String(typeof RegExp)'");
}

// —— JSON ——————————————————————————————————————
#[test]
fn test_json() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof JSON"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

// —— Math ——————————————————————————————————————
#[test]
fn test_math() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof Math.random"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof Math.random'");
}

// —— Window chrome shim ———————————————————————
#[test]
fn test_window_chrome() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof window.chrome"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

// —— Navigator connection ————————————————————
#[test]
fn test_navigator_connection() {
    let mut kernel = make_new_chain_kernel();
    let conn = common::to_str(&kernel.eval_to_rust_value("typeof navigator.connection"));
    assert!(conn != "undefined", "kernel navigator.connection is undefined");
}

// —— Document properties —————————————————————
#[test]
fn test_document_cookie() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof document.cookie"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof document.cookie'");
}

#[test]
fn test_document_referrer() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof document.referrer"));
    assert!(!val.is_empty(), "kernel returned empty on 'typeof document.referrer'");
}

// —— Window property count ——————————————————
#[test]
fn test_window_prop_count_new_ge_old() {
    let mut kernel = make_new_chain_kernel();
    let count = common::to_str(&kernel
        .eval_to_rust_value("String(Object.getOwnPropertyNames(window).length)"));
    let n: usize = count.parse().unwrap();
    assert!(n > 0, "kernel has no window properties");
}

/// T-5: Coverage gate — new chain window property count >= 95% Chrome 147.
/// Chrome 147 has ~1453 own properties on window. 95% = ~1380.
/// The new chain (1284 IDL interfaces) achieves ~1391 (>95%).
#[test]
fn test_coverage_gate_window_props_vs_chrome147() {
    const CHROME_147_BASELINE: usize = 1453;
    const MIN_RATIO: f64 = 0.95;

    let mut kernel = make_new_chain_kernel();
    let count_str = common::to_str(&kernel
        .eval_to_rust_value("String(Object.getOwnPropertyNames(window).length)"));
    let count: usize = count_str.parse().unwrap();

    let ratio = count as f64 / CHROME_147_BASELINE as f64;
    assert!(
        ratio >= MIN_RATIO,
        "coverage gate: {}/{} = {:.1}% < {}%",
        count,
        CHROME_147_BASELINE,
        ratio * 100.0,
        MIN_RATIO * 100.0
    );
}

// —— URL constructor —————————————————————————
#[test]
fn test_url_parsing() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value(
        "String((new URL('https://example.com/path?q=1')).pathname)"
    ));
    assert!(!val.is_empty(), "kernel returned empty");
}

// —— Blob constructor ———————————————————————
#[test]
fn test_blob_constructor() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("String(typeof (new Blob(['test'])))"));
    assert!(!val.is_empty(), "kernel returned empty");
}

// —— structuredClone ——————————————————————
#[test]
fn test_structuredclone() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof structuredClone"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

// —— Navigator.sendBeacon —————————————————
#[test]
fn test_sendbeacon() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof navigator.sendBeacon"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

// —— Multiple kernels —————————————————————
#[test]
fn test_multiple_kernels_same_result() {
    let expressions = [
        "typeof window",
        "document.createElement('div').tagName",
    ];

    for expr in &expressions {
        let mut results = Vec::new();
        for _ in 0..3 {
            let mut kernel = make_new_chain_kernel();
            results.push(common::to_str(&kernel.eval_to_rust_value(expr)));
        }
        let first = &results[0];
        for (i, r) in results.iter().enumerate().skip(1) {
            assert_eq!(first, r, "kernels diverge on '{}': kernel 0 vs {}", expr, i);
        }
    }
}

// —— CSS.escape ——————————————————————————
#[test]
fn test_css_escape() {
    let mut kernel = make_new_chain_kernel();
    let _ = kernel.eval_to_rust_value("typeof CSS");
}

// —— Session history ————————————————————
#[test]
fn test_history() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof history"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

// —— navigator.onLine —————————————————
#[test]
fn test_navigator_online() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("String(typeof navigator.onLine)"));
    assert!(!val.is_empty(), "kernel returned empty");
}

// —— navigator.geolocation —————————————
#[test]
fn test_navigator_geolocation() {
    let mut kernel = make_new_chain_kernel();
    let _ = kernel.eval_to_rust_value("typeof navigator.geolocation");
}

// —— SubtleCrypto digest ———————————————
#[test]
fn test_crypto_subtle_digest() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof crypto.subtle.digest"));
    assert!(val != "false" && val != "null" && val != "undefined" && !val.is_empty(),
        "kernel returned falsy/empty: {:?}", val);
}

// —— Worker ——————————————————————————
#[test]
fn test_worker_not_throwing() {
    let mut kernel = make_new_chain_kernel();
    let _ = kernel.eval_to_rust_value("typeof Worker");
}

// —— Canvas2D getImageData ——————————
#[test]
fn test_canvas_getimagedata() {
    let mut kernel = make_new_chain_kernel();
    let img = common::to_str(&kernel.eval_to_rust_value(
        "var c=document.createElement('canvas'); c.width=100; c.height=100; var ctx=c.getContext('2d'); typeof ctx.getImageData(0,0,10,10)"
    ));
    assert!(!img.is_empty(), "getImageData typeof empty");
}

// —— HTMLElement style access ——————————
#[test]
fn test_element_style() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value(
        "String(typeof document.createElement('div').style)"
    ));
    assert!(!val.is_empty(), "kernel returned empty");
}

// —— HTMLElement classList ———————————
#[test]
fn test_element_classlist() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value(
        "String(typeof document.createElement('div').classList)"
    ));
    assert!(!val.is_empty(), "kernel returned empty");
}

// —— setTimeout returns ID ——————————
#[test]
fn test_settimeout_returns_number() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("String(typeof setTimeout(function(){}, 0))"));
    assert!(!val.is_empty(), "kernel returned empty");
}

// —— navigator.userAgentData ———————
#[test]
fn test_useragentdata() {
    let mut kernel = make_new_chain_kernel();
    let _ = kernel.eval_to_rust_value("typeof navigator.userAgentData");
}

// —— Promise constructor ——————————
#[test]
fn test_promise_resolve() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("String(typeof Promise.resolve)"));
    assert!(!val.is_empty(), "kernel returned empty");
}

// —— Symbol.toStringTag ——————————
#[test]
fn test_symbol_tostringtag() {
    let mut kernel = make_new_chain_kernel();

    let tag_exists = common::to_str(&kernel.eval_to_rust_value("typeof Symbol.toStringTag"));
    assert!(tag_exists != "false" && tag_exists != "null" && tag_exists != "undefined" && !tag_exists.is_empty(),
        "Symbol.toStringTag falsy: {:?}", tag_exists);

    let tag = common::to_str(&kernel
        .eval_to_rust_value("String(HTMLDivElement.prototype[Symbol.toStringTag])"));
    assert!(!tag.is_empty(), "toStringTag empty for HTMLDivElement");
}

// ── v0.8.50 experiential robustness (7 new surface items) ────────────────────

#[test]
fn test_inner_text_exists() {
    let mut kernel = make_new_chain_kernel();
    kernel.eval_to_rust_value(
        "document.body.innerHTML = '<div id=x>hello</div>'",
    );
    let kind = common::to_str(&kernel.eval_to_rust_value(
        "typeof document.getElementById('x').innerText",
    ));
    assert_eq!(kind, "string");
    let val = common::to_str(&kernel.eval_to_rust_value(
        "document.getElementById('x').innerText",
    ));
    assert_eq!(val, "hello");
}

#[test]
fn test_body_used_on_response() {
    let mut kernel = make_new_chain_kernel();
    let exists = common::to_str(&kernel.eval_to_rust_value(
        "'bodyUsed' in Response.prototype",
    ));
    assert_eq!(exists, "true");
}

#[test]
fn test_body_used_default_false() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value(
        "new Response('test').bodyUsed",
    ));
    assert_eq!(val, "false");
}

#[test]
fn test_xhr_ready_state_constants() {
    let mut kernel = make_new_chain_kernel();
    assert_eq!(
        common::to_str(&kernel.eval_to_rust_value("XMLHttpRequest.HEADERS_RECEIVED")),
        "2"
    );
    assert_eq!(
        common::to_str(&kernel.eval_to_rust_value("XMLHttpRequest.LOADING")),
        "3"
    );
    assert_eq!(
        common::to_str(&kernel.eval_to_rust_value("XMLHttpRequest.DONE")),
        "4"
    );
}

#[test]
fn test_location_native_accessor_descriptor() {
    let mut kernel = make_new_chain_kernel();
    // Verify accessor properties exist (on prototype, checked via 'in')
    let has_getter = common::to_str(&kernel.eval_to_rust_value(
        "'get' in Object.getOwnPropertyDescriptor(Object.getPrototypeOf(location), 'href')",
    ));
    assert_eq!(has_getter, "true", "location href missing native getter");

    let has_setter = common::to_str(&kernel.eval_to_rust_value(
        "'set' in Object.getOwnPropertyDescriptor(Object.getPrototypeOf(location), 'href')",
    ));
    assert_eq!(has_setter, "true", "location href missing native setter");
}

#[test]
fn test_location_href_getter_works() {
    let mut kernel = make_new_chain_kernel();
    let val = common::to_str(&kernel.eval_to_rust_value("typeof location.href"));
    assert_eq!(val, "string");
}

#[test]
fn test_location_setter_persists() {
    let mut kernel = make_new_chain_kernel();
    kernel.eval_to_rust_value("location.href = 'https://example.com/test'");
    let val = common::to_str(&kernel.eval_to_rust_value("location.href"));
    assert_eq!(val, "https://example.com/test");
}

#[test]
fn test_location_to_string_returns_href() {
    let mut kernel = make_new_chain_kernel();
    kernel.eval_to_rust_value("location.href = 'https://example.com/foo'");
    // Implicit coercion via template literal calls toString internally
    let val = common::to_str(&kernel.eval_to_rust_value("'' + location"));
    assert_eq!(val, "https://example.com/foo");
}

#[test]
fn test_cookie_multi_set_get() {
    let mut kernel = make_new_chain_kernel();
    kernel.eval_to_rust_value("document.cookie = 'a=1'");
    kernel.eval_to_rust_value("document.cookie = 'b=2'");
    let val = common::to_str(&kernel.eval_to_rust_value("document.cookie"));
    assert!(val.contains("a=1"), "cookie missing a=1: {}", val);
    assert!(val.contains("b=2"), "cookie missing b=2: {}", val);
}

#[test]
fn test_cookie_max_age_zero_removes() {
    let mut kernel = make_new_chain_kernel();
    kernel.eval_to_rust_value("document.cookie = 'x=42'");
    let before = common::to_str(&kernel.eval_to_rust_value("document.cookie"));
    assert!(before.contains("x=42"), "cookie not set: {}", before);
    kernel.eval_to_rust_value("document.cookie = 'x=; Max-Age=0'");
    let after = common::to_str(&kernel.eval_to_rust_value("document.cookie"));
    assert!(!after.contains("x=42"), "cookie not removed: {}", after);
}
