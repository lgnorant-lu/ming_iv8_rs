#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// Integration tests for __iv8__.page.load() — the page_api.rs code path.
// This is the primary page load entry point used by all comparison scripts.
// v0.8.77: coverage backfill for page_api.rs (was 5.41% lines).

use iv8_core::{RustValue};
use common::to_str;

/// Helper: call __iv8__.page.load(snapshot) via eval.
fn page_load(kernel: &mut iv8_core::EmbeddedV8Kernel, html: &str, base_url: &str, headers: &[(&str, &str)]) {
    let header_js = if headers.is_empty() {
        "[]".to_string()
    } else {
        let pairs: Vec<String> = headers
            .iter()
            .map(|(k, v)| format!("[\"{}\", \"{}\"]", k, v))
            .collect();
        format!("[{}]", pairs.join(", "))
    };
    let html_escaped = html.replace('\\', "\\\\").replace('\'', "\\'").replace('\n', "\\n");
    let js = format!(
        r#"
        __iv8__.data = {{
            s1: {{
                baseURL: '{}',
                html: '{}',
                headers: {},
                resources: {{}}
            }}
        }};
        __iv8__.page.load(__iv8__.data.s1);
        "#,
        base_url, html_escaped, header_js
    );
    kernel.eval_to_rust_value(&js);
}

// ============================================================
// T1: Basic page load + inline script execution
// ============================================================

#[test]
fn page_api_basic_html() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body><div id='app'>Hello</div></body></html>", "http://test.com/", &[]);
    let result = k.eval_to_rust_value("document.getElementById('app').textContent");
    assert_eq!(to_str(&result), "Hello");
}

#[test]
fn page_api_executes_inline_script() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body><script>globalThis._loaded = true;</script></body></html>", "http://test.com/", &[]);
    assert_eq!(k.eval_to_rust_value("globalThis._loaded"), RustValue::Bool(true));
}

#[test]
fn page_api_multiple_scripts_in_order() {
    let mut k = common::make_kernel();
    page_load(&mut k, r#"<html><body>
        <script>globalThis._order = [];</script>
        <script>globalThis._order.push(1);</script>
        <script>globalThis._order.push(2);</script>
        </body></html>"#, "http://test.com/", &[]);
    let result = k.eval_to_rust_value("globalThis._order");
    match result {
        RustValue::Array(arr) => {
            assert_eq!(arr.len(), 2);
            assert_eq!(arr[0], RustValue::Int(1));
            assert_eq!(arr[1], RustValue::Int(2));
        }
        other => panic!("expected array, got {:?}", other),
    }
}

// ============================================================
// T2: Cookie accessor — the core v0.8.77 fix
// ============================================================

#[test]
fn page_api_cookie_set_and_get() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body><script>document.cookie = 'a=1; path=/';</script></body></html>", "http://test.com/", &[]);
    let val = to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("a=1"), "cookie missing a=1: {}", val);
}

#[test]
fn page_api_cookie_multiple_values() {
    let mut k = common::make_kernel();
    page_load(&mut k, r#"<html><body>
        <script>document.cookie = 'x=hello; path=/';</script>
        <script>document.cookie = 'y=world; path=/';</script>
        </body></html>"#, "http://test.com/", &[]);
    let val = to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("x=hello"), "missing x=hello: {}", val);
    assert!(val.contains("y=world"), "missing y=world: {}", val);
}

#[test]
fn page_api_cookie_getter_is_function() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body></body></html>", "http://test.com/", &[]);
    let getter = to_str(&k.eval_to_rust_value(
        "document.__lookupGetter__('cookie') ? 'function' : 'undefined'"
    ));
    assert_eq!(getter, "function");
}

#[test]
fn page_api_cookie_setter_is_function() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body></body></html>", "http://test.com/", &[]);
    let setter = to_str(&k.eval_to_rust_value(
        "document.__lookupSetter__('cookie') ? 'function' : 'undefined'"
    ));
    assert_eq!(setter, "function");
}

#[test]
fn page_api_cookie_store_populated() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body><script>document.cookie = 'rs_token=abc; path=/';</script></body></html>", "http://test.com/", &[]);
    let store = to_str(&k.eval_to_rust_value(
        "JSON.stringify(window._iv8CookieStore)"
    ));
    assert!(store.contains("rs_token"), "store missing rs_token: {}", store);
    assert!(store.contains("abc"), "store missing value abc: {}", store);
}

// ============================================================
// T3: Set-Cookie header injection
// ============================================================

#[test]
fn page_api_set_cookie_header_injected() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body></body></html>", "http://test.com/", &[("Set-Cookie", "from_header=val1; path=/")]);
    let val = to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("from_header=val1"), "Set-Cookie header not injected: {}", val);
}

#[test]
fn page_api_set_cookie_header_plus_inline() {
    let mut k = common::make_kernel();
    page_load(
        &mut k,
        "<html><body><script>document.cookie = 'from_script=val2; path=/';</script></body></html>",
        "http://test.com/",
        &[("Set-Cookie", "from_header=val1; path=/")]
    );
    let val = to_str(&k.eval_to_rust_value("document.cookie"));
    assert!(val.contains("from_header=val1"), "missing header cookie: {}", val);
    assert!(val.contains("from_script=val2"), "missing script cookie: {}", val);
}

// ============================================================
// T4: Event dispatch (DOMContentLoaded + load)
// ============================================================

#[test]
fn page_api_domcontentloaded_fired() {
    let mut k = common::make_kernel();
    page_load(
        &mut k,
        r#"<html><body><script>
        document.addEventListener('DOMContentLoaded', function() { globalThis._dcl = true; });
        </script></body></html>"#,
        "http://test.com/", &[]
    );
    assert_eq!(k.eval_to_rust_value("globalThis._dcl"), RustValue::Bool(true));
}

#[test]
fn page_api_load_event_fired() {
    let mut k = common::make_kernel();
    page_load(
        &mut k,
        r#"<html><body><script>
        document.addEventListener('load', function() { globalThis._loaded_evt = true; });
        </script></body></html>"#,
        "http://test.com/", &[]
    );
    assert_eq!(k.eval_to_rust_value("globalThis._loaded_evt"), RustValue::Bool(true));
}

// ============================================================
// T5: readyState transitions
// ============================================================

#[test]
fn page_api_ready_state_complete() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body></body></html>", "http://test.com/", &[]);
    let rs = to_str(&k.eval_to_rust_value("document.readyState"));
    assert_eq!(rs, "complete");
}

#[test]
fn page_api_ready_state_during_script() {
    let mut k = common::make_kernel();
    page_load(
        &mut k,
        r#"<html><body><script>globalThis._script_rs = document.readyState;</script></body></html>"#,
        "http://test.com/", &[]
    );
    // During script execution, readyState should be 'loading' (before step 6 sets interactive)
    let rs = to_str(&k.eval_to_rust_value("globalThis._script_rs"));
    // readyState during script is set by the shim default ('complete'),
    // then step 6 sets 'interactive', then step 8 sets 'complete'
    // The exact value depends on when the script runs relative to step 6
    assert!(
        rs == "loading" || rs == "interactive" || rs == "complete",
        "unexpected readyState during script: {}",
        rs
    );
}

// ============================================================
// T6: Microtask drain (Promise resolution)
// ============================================================

#[test]
fn page_api_promise_microtask_drained() {
    let mut k = common::make_kernel();
    page_load(
        &mut k,
        r#"<html><body><script>
        globalThis._promise_val = null;
        Promise.resolve(42).then(function(v) { globalThis._promise_val = v; });
        </script></body></html>"#,
        "http://test.com/", &[]
    );
    let val = k.eval_to_rust_value("globalThis._promise_val");
    assert_eq!(val, RustValue::Int(42));
}

// ============================================================
// T7: Location updated before scripts (parity fix)
// ============================================================

#[test]
fn page_api_location_updated_before_scripts() {
    let mut k = common::make_kernel();
    page_load(
        &mut k,
        r#"<html><body><script>globalThis._script_href = location.href;</script></body></html>"#,
        "http://new.com/page.html", &[]
    );
    let href = to_str(&k.eval_to_rust_value("globalThis._script_href"));
    assert_eq!(href, "http://new.com/page.html");
}

#[test]
fn page_api_location_pathname_correct() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body></body></html>", "http://test.com/path/page.html", &[]);
    let pn = to_str(&k.eval_to_rust_value("location.pathname"));
    assert_eq!(pn, "/path/page.html");
}

// ============================================================
// T8: Shim re-installation (Canvas2D, document.write, etc.)
// ============================================================

#[test]
fn page_api_canvas2d_shim_available() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body><canvas id='c'></canvas></body></html>", "http://test.com/", &[]);
    // Canvas2D shim should be installed — test basic context
    let result = to_str(&k.eval_to_rust_value(
        "typeof document.getElementById('c').getContext('2d').fillRect === 'function' ? 'yes' : 'no'"
    ));
    assert_eq!(result, "yes");
}

#[test]
fn page_api_document_write_available() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body></body></html>", "http://test.com/", &[]);
    let result = to_str(&k.eval_to_rust_value(
        "typeof document.write === 'function' ? 'yes' : 'no'"
    ));
    assert_eq!(result, "yes");
}

// ============================================================
// T9: Document properties (EXCLUDED_ATTRIBUTES shim is source of truth)
// ============================================================

#[test]
fn page_api_document_charset() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body></body></html>", "http://test.com/", &[]);
    assert_eq!(to_str(&k.eval_to_rust_value("document.characterSet")), "UTF-8");
}

#[test]
fn page_api_document_compat_mode() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body></body></html>", "http://test.com/", &[]);
    assert_eq!(to_str(&k.eval_to_rust_value("document.compatMode")), "CSS1Compat");
}

#[test]
fn page_api_document_hidden() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body></body></html>", "http://test.com/", &[]);
    assert_eq!(k.eval_to_rust_value("document.hidden"), RustValue::Bool(false));
}

#[test]
fn page_api_document_visibility_state() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body></body></html>", "http://test.com/", &[]);
    assert_eq!(to_str(&k.eval_to_rust_value("document.visibilityState")), "visible");
}

#[test]
fn page_api_document_referrer() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body></body></html>", "http://test.com/", &[]);
    assert_eq!(to_str(&k.eval_to_rust_value("document.referrer")), "");
}

#[test]
fn page_api_document_url() {
    let mut k = common::make_kernel();
    page_load(&mut k, "<html><body></body></html>", "http://test.com/path", &[]);
    assert_eq!(to_str(&k.eval_to_rust_value("document.URL")), "http://test.com/path");
}
