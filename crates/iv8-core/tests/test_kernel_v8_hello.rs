#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;


// Task 2: Verify v8 crate integration — create isolate, eval "1+1", get 2.

#[test]
fn eval_one_plus_one() {
    iv8_core::v8_init::ensure_v8_initialized();

    let mut isolate = v8::Isolate::new(v8::CreateParams::default());

    v8::scope!(handle_scope, &mut isolate);
    let context = v8::Context::new(handle_scope, Default::default());
    v8::scope_with_context!(scope, handle_scope, context);

    let source = v8::String::new(scope, "1 + 1").expect("failed to create source string");
    let script = v8::Script::compile(scope, source, None).expect("failed to compile");
    let result = script.run(scope).expect("failed to run");

    let result_i32 = result.int32_value(scope).expect("result is not i32");
    assert_eq!(result_i32, 2, "1 + 1 should equal 2");
}
