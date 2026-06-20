#![allow(
    clippy::expect_used,
    clippy::panic,
    clippy::unwrap_used,
    unused_imports,
    unused_variables
)]
mod common;

// Integration test for v8_extra bindings (MarkAsUndetectable, SetCallAsFunctionHandler).
//
// V8 invariant: an ObjectTemplate marked as undetectable MUST also have a
// CallAsFunctionHandler (V8 asserts this in debug builds and the object
// cannot be instantiated otherwise). The handler can be a no-op.

use iv8_core::v8_extra;

/// Default no-op call handler (returns undefined).
fn noop_call_handler(
    _scope: &mut v8::PinScope,
    _args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
}

#[test]
fn mark_as_undetectable_typeof_returns_undefined() {
    iv8_core::v8_init::ensure_v8_initialized();

    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    v8::scope!(handle_scope, &mut isolate);
    let context = v8::Context::new(handle_scope, Default::default());
    v8::scope_with_context!(scope, handle_scope, context);

    // V8 requires CallAsFunctionHandler when MarkAsUndetectable is set.
    let templ = v8::ObjectTemplate::new(scope);
    v8_extra::mark_as_undetectable(&templ);
    v8_extra::set_call_as_function_handler(&templ, v8::MapFnTo::map_fn_to(noop_call_handler), None);
    let obj = templ
        .new_instance(scope)
        .expect("should create undetectable instance");

    let global = context.global(scope);
    let key = v8::String::new(scope, "foo").unwrap();
    global.set(scope, key.into(), obj.into()).unwrap();

    // typeof foo === 'undefined'
    let source = v8::String::new(scope, "typeof foo").unwrap();
    let result = v8::Script::compile(scope, source, None)
        .unwrap()
        .run(scope)
        .unwrap();
    assert_eq!(result.to_rust_string_lossy(scope), "undefined");

    // foo == null is true (loose equality)
    let source = v8::String::new(scope, "foo == null").unwrap();
    let result = v8::Script::compile(scope, source, None)
        .unwrap()
        .run(scope)
        .unwrap();
    assert!(result.boolean_value(scope));

    // Boolean(foo) === false
    let source = v8::String::new(scope, "Boolean(foo)").unwrap();
    let result = v8::Script::compile(scope, source, None)
        .unwrap()
        .run(scope)
        .unwrap();
    assert!(!result.boolean_value(scope));

    // if (foo) does not enter the branch
    let source = v8::String::new(scope, "let x = 0; if (foo) { x = 1; } x").unwrap();
    let result = v8::Script::compile(scope, source, None)
        .unwrap()
        .run(scope)
        .unwrap();
    assert_eq!(result.int32_value(scope).unwrap(), 0);
}

#[test]
fn set_call_as_function_handler_makes_object_callable() {
    iv8_core::v8_init::ensure_v8_initialized();

    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    v8::scope!(handle_scope, &mut isolate);
    let context = v8::Context::new(handle_scope, Default::default());
    v8::scope_with_context!(scope, handle_scope, context);

    fn callback(
        _scope: &mut v8::PinScope,
        _args: v8::FunctionCallbackArguments,
        mut rv: v8::ReturnValue,
    ) {
        rv.set_int32(42);
    }

    let templ = v8::ObjectTemplate::new(scope);
    v8_extra::set_call_as_function_handler(&templ, v8::MapFnTo::map_fn_to(callback), None);
    let obj = templ.new_instance(scope).unwrap();

    let global = context.global(scope);
    let key = v8::String::new(scope, "callable").unwrap();
    global.set(scope, key.into(), obj.into()).unwrap();

    let source = v8::String::new(scope, "callable()").unwrap();
    let result = v8::Script::compile(scope, source, None)
        .unwrap()
        .run(scope)
        .unwrap();
    assert_eq!(result.int32_value(scope).unwrap(), 42);
}

#[test]
fn document_all_pattern_undetectable_and_callable() {
    // HTMLAllCollection-like: undetectable AND callable.
    iv8_core::v8_init::ensure_v8_initialized();

    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    v8::scope!(handle_scope, &mut isolate);
    let context = v8::Context::new(handle_scope, Default::default());
    v8::scope_with_context!(scope, handle_scope, context);

    fn all_callback(
        scope: &mut v8::PinScope,
        args: v8::FunctionCallbackArguments,
        mut rv: v8::ReturnValue,
    ) {
        if args.length() > 0 {
            let s = args.get(0).to_rust_string_lossy(scope);
            let result = v8::String::new(scope, &format!("got:{s}")).unwrap();
            rv.set(result.into());
        }
    }

    let templ = v8::ObjectTemplate::new(scope);
    v8_extra::mark_as_undetectable(&templ);
    v8_extra::set_call_as_function_handler(&templ, v8::MapFnTo::map_fn_to(all_callback), None);
    let obj = templ.new_instance(scope).unwrap();

    let global = context.global(scope);
    let key = v8::String::new(scope, "all").unwrap();
    global.set(scope, key.into(), obj.into()).unwrap();

    // typeof all === 'undefined'
    let source = v8::String::new(scope, "typeof all").unwrap();
    let result = v8::Script::compile(scope, source, None)
        .unwrap()
        .run(scope)
        .unwrap();
    assert_eq!(result.to_rust_string_lossy(scope), "undefined");

    // all('foo') still works
    let source = v8::String::new(scope, "all('foo')").unwrap();
    let result = v8::Script::compile(scope, source, None)
        .unwrap()
        .run(scope)
        .unwrap();
    assert_eq!(result.to_rust_string_lossy(scope), "got:foo");
}
