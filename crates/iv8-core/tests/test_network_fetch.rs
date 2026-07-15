#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// Integration tests for fetch() JS binding (Task 46).

use iv8_core::{EmbeddedV8Kernel, EvalOpts, KernelConfig, RustValue};
#[test]
fn fetch_exists() {
    let mut kernel = common::make_kernel();
    assert_eq!(
        kernel.eval_to_rust_value("typeof fetch"),
        RustValue::String("function".into())
    );
}

#[test]
fn fetch_returns_promise() {
    let mut kernel = common::make_kernel();
    let result = kernel.eval_to_rust_value("fetch('http://x.com') instanceof Promise");
    assert_eq!(result, RustValue::Bool(true));
}

#[test]
fn fetch_resolves_with_resource() {
    let mut kernel = common::make_kernel();
    kernel.add_resource(
        "https://api.example.com/data",
        b"hello world".to_vec(),
        200,
        None,
    );

    // Use .then() to capture the resolved value
    kernel
        .eval(
            r#"
        globalThis.fetchResult = null;
        fetch('https://api.example.com/data').then(function(response) {
            globalThis.fetchResult = response.status;
        });
    "#,
            EvalOpts::default(),
        )
        .unwrap();

    // Drain microtasks to resolve the promise
    kernel.drain_microtasks();

    let result = kernel.eval_to_rust_value("globalThis.fetchResult");
    assert_eq!(result, RustValue::Int(200));
}

#[test]
fn fetch_response_ok() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://x.com/ok", b"".to_vec(), 200, None);

    kernel
        .eval(
            r#"
        globalThis.isOk = null;
        fetch('https://x.com/ok').then(function(r) { globalThis.isOk = r.ok; });
    "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();

    assert_eq!(
        kernel.eval_to_rust_value("globalThis.isOk"),
        RustValue::Bool(true)
    );
}

#[test]
fn fetch_response_text() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://x.com/text", b"hello".to_vec(), 200, None);

    kernel
        .eval(
            r#"
        globalThis.body = null;
        fetch('https://x.com/text')
            .then(function(r) { return r.text(); })
            .then(function(t) { globalThis.body = t; });
    "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();

    assert_eq!(
        kernel.eval_to_rust_value("globalThis.body"),
        RustValue::String("hello".into())
    );
}

#[test]
fn fetch_response_json() {
    let mut kernel = common::make_kernel();
    kernel.add_resource(
        "https://x.com/json",
        br#"{"key":"value"}"#.to_vec(),
        200,
        None,
    );

    kernel
        .eval(
            r#"
        globalThis.parsed = null;
        fetch('https://x.com/json')
            .then(function(r) { return r.json(); })
            .then(function(j) { globalThis.parsed = j.key; });
    "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();

    assert_eq!(
        kernel.eval_to_rust_value("globalThis.parsed"),
        RustValue::String("value".into())
    );
}

#[test]
fn response_constructor_text_and_json_body() {
    // new Response(body) must wire __body__ so text()/json() resolve (not only fetch()).
    let mut kernel = common::make_kernel();
    kernel
        .eval(
            r#"
        globalThis.t = null;
        globalThis.k = null;
        new Response('hello').text().then(function(v) { globalThis.t = v; });
        new Response(JSON.stringify({a:1})).json().then(function(j) { globalThis.k = j.a; });
    "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.t"),
        RustValue::String("hello".into())
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.k"),
        RustValue::Int(1)
    );
}

#[test]
fn sequential_second_kernel_constructs() {
    // Serialize init_kernel so second isolate after first still constructs (hang guard).
    let mut k1 = common::make_kernel();
    assert_eq!(
        k1.eval_to_rust_value("1+1"),
        RustValue::Int(2)
    );
    let mut k2 = common::make_kernel();
    assert_eq!(
        k2.eval_to_rust_value("1+1"),
        RustValue::Int(2)
    );
    assert_eq!(
        k2.eval_to_rust_value(
            "(function(){try{document.body.appendChild(document.createElement('div'));return 'ok'}catch(e){return String(e)}})()"
        ),
        RustValue::String("ok".into())
    );
}

#[test]
fn cross_thread_second_kernel_waits_then_succeeds_after_drop() {
    // Full kernel live on this thread: other thread blocks in LIVE_FULL_KERNELS wait
    // (not V8 hang). Drop k1 → notify → other thread proceeds.
    let k1 = common::make_kernel();
    let (tx, rx) = std::sync::mpsc::channel();
    let handle = std::thread::Builder::new()
        .stack_size(128 * 1024 * 1024)
        .spawn(move || {
            let t0 = std::time::Instant::now();
            let mut k = EmbeddedV8Kernel::new(KernelConfig::default()).expect("create after wait");
            let elapsed = t0.elapsed();
            let _ = tx.send(elapsed);
            assert_eq!(k.eval_to_rust_value("1+1"), RustValue::Int(2));
        })
        .expect("spawn");
    // Give child time to block on LIVE_FULL_KERNELS wait
    std::thread::sleep(std::time::Duration::from_millis(300));
    drop(k1);
    let elapsed = rx
        .recv_timeout(std::time::Duration::from_secs(15))
        .expect("child result");
    assert!(
        elapsed >= std::time::Duration::from_millis(200),
        "child should have waited for drop, elapsed={elapsed:?}"
    );
    handle.join().expect("join");
}

#[test]
fn fetch_missing_url_resolves_404_not_reject() {
    // Browser fetch: network failure can reject; offline ResourceBundle path
    // resolves with Response(status=404) when URL not registered (fetch.rs).
    let mut kernel = common::make_kernel();

    kernel
        .eval(
            r#"
        globalThis.missingStatus = -1;
        globalThis.missingOk = null;
        globalThis.missingRejected = false;
        fetch('https://unknown.com/missing')
            .then(function(r) {
                globalThis.missingStatus = r.status;
                globalThis.missingOk = r.ok;
            })
            .catch(function() { globalThis.missingRejected = true; });
    "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();

    assert_eq!(
        kernel.eval_to_rust_value("globalThis.missingRejected"),
        RustValue::Bool(false)
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.missingStatus"),
        RustValue::Int(404)
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.missingOk"),
        RustValue::Bool(false)
    );
}

#[test]
fn fetch_404_resolves_not_rejects() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://x.com/404", b"not found".to_vec(), 404, None);

    kernel
        .eval(
            r#"
        globalThis.got404 = false;
        fetch('https://x.com/404').then(function(r) {
            globalThis.got404 = (r.status === 404 && r.ok === false);
        });
    "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();

    assert_eq!(
        kernel.eval_to_rust_value("globalThis.got404"),
        RustValue::Bool(true)
    );
}

#[test]
fn fetch_response_ok_status_no_recursion() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://x.com/regr", b"body".to_vec(), 200, None);
    kernel
        .eval(
            r#"
        globalThis.regrOk = null;
        globalThis.regrStatus = -1;
        fetch('https://x.com/regr').then(function(r) {
            globalThis.regrOk = r.ok;
            globalThis.regrStatus = r.status;
        });
        "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.regrOk"),
        RustValue::Bool(true)
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.regrStatus"),
        RustValue::Int(200)
    );
}

#[test]
fn fetch_response_404_ok_status_no_recursion() {
    let mut kernel = common::make_kernel();
    kernel.add_resource("https://x.com/nope", b"not found".to_vec(), 404, None);
    kernel
        .eval(
            r#"
        globalThis.r4ok = null;
        globalThis.r4st = -1;
        fetch('https://x.com/nope').then(function(r) {
            globalThis.r4ok = r.ok;
            globalThis.r4st = r.status;
        });
        "#,
            EvalOpts::default(),
        )
        .unwrap();
    kernel.drain_microtasks();
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.r4ok"),
        RustValue::Bool(false)
    );
    assert_eq!(
        kernel.eval_to_rust_value("globalThis.r4st"),
        RustValue::Int(404)
    );
}
