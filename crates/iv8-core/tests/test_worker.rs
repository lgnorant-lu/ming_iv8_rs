mod common;

#[test]
fn test_worker_constructor_returns_object() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof new Worker('data:,')", "object");
}

#[test]
fn test_worker_terminate_no_panic() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "(function(){ var w = new Worker('data:,'); w.terminate(); return 'ok'; })()",
        "ok",
    );
}

#[test]
fn test_worker_post_message_no_panic() {
    let mut k = common::make_kernel();
    common::assert_js_str(
        &mut k,
        "(function(){ var w = new Worker('data:,'); w.postMessage({a:1}); return 'ok'; })()",
        "ok",
    );
}

#[test]
fn test_worker_prototype_has_terminate() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof Worker.prototype.terminate", "function");
}

#[test]
fn test_worker_prototype_has_post_message() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof Worker.prototype.postMessage", "function");
}

#[test]
fn test_worker_instance_has_worker_prototype() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value(
        "Object.getPrototypeOf(new Worker('data:,')).constructor.name",
    ));
    assert_eq!(val, "Worker");
}
