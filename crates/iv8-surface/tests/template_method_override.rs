//! v0.8.29 T-1: FunctionTemplate post-creation method injection.
//!
//! FINDING: ObjectTemplate.set() does NOT override existing prototype methods.
//! First set wins; subsequent sets with same key silently ignored.
//! This means v0.8.30+ template-level BCR injection requires codegen mods.

use std::sync::Once;
use v8::*;

static V8_INIT: Once = Once::new();

fn init_v8() {
    V8_INIT.call_once(|| {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });
}

unsafe extern "C" fn stub_constructor(_info: *const FunctionCallbackInfo) {}
unsafe extern "C" fn returns_true(info: *const FunctionCallbackInfo) {
    let info_ref = unsafe { &*info };
    let mut rv = ReturnValue::from_function_callback_info(info_ref);
    rv.set_bool(true);
}
unsafe extern "C" fn returns_false(info: *const FunctionCallbackInfo) {
    let info_ref = unsafe { &*info };
    let mut rv = ReturnValue::from_function_callback_info(info_ref);
    rv.set_bool(false);
}
unsafe extern "C" fn returns_42(info: *const FunctionCallbackInfo) {
    let info_ref = unsafe { &*info };
    callback_scope!(unsafe scope, info_ref);
    let mut rv = ReturnValue::from_function_callback_info(info_ref);
    rv.set(Integer::new(scope, 42).into());
}

#[test]
fn test_prototype_set_does_not_override() {
    init_v8();
    let mut isolate = Isolate::new(CreateParams::default());
    v8::scope!(outer, &mut isolate);

    let ctx = Context::new(outer, Default::default());
    let _g = Global::new(outer, ctx);
    v8::scope_with_context!(scope, outer, ctx);

    let tmpl = FunctionTemplate::builder_raw(stub_constructor).build(scope);
    let proto = tmpl.prototype_template(scope);

    // Step 1: Install method returning false
    {
        let name = String::new(scope, "ok").unwrap();
        let m = FunctionTemplate::builder_raw(returns_false).build(scope);
        proto.set(name.into(), m.into());
    }

    let func = tmpl.get_function(scope).unwrap();
    let key = String::new(scope, "ok").unwrap();

    // Create instance1 BEFORE override
    let i1 = func.new_instance(scope, &[]).unwrap();
    let v1 = get_bool_from_method(scope, i1, &key);
    assert!(!v1, "initial: expected false");

    // Step 2: Attempt override on prototype (this is silently ignored)
    {
        let name = String::new(scope, "ok").unwrap();
        let m = FunctionTemplate::builder_raw(returns_true).build(scope);
        proto.set(name.into(), m.into());
    }

    // Create instance2 AFTER the (attempted) override
    let i2 = func.new_instance(scope, &[]).unwrap();
    let v2 = get_bool_from_method(scope, i2, &key);
    // FINDING: override does NOT take effect — still returns false
    assert!(!v2, "override should be silently ignored: expected false, got true");
}

#[test]
fn test_number_return_from_prototype_method() {
    init_v8();
    let mut isolate = Isolate::new(CreateParams::default());
    v8::scope!(outer, &mut isolate);

    let ctx = Context::new(outer, Default::default());
    let _g = Global::new(outer, ctx);
    v8::scope_with_context!(scope, outer, ctx);

    let tmpl = FunctionTemplate::builder_raw(stub_constructor).build(scope);
    let proto = tmpl.prototype_template(scope);

    {
        let name = String::new(scope, "value").unwrap();
        let m = FunctionTemplate::builder_raw(returns_42).build(scope);
        proto.set(name.into(), m.into());
    }

    let func = tmpl.get_function(scope).unwrap();
    let instance = func.new_instance(scope, &[]).unwrap();
    let key = String::new(scope, "value").unwrap();

    let v: Local<Value> = instance.get(scope, key.into()).unwrap();
    assert!(v.is_function());
    let f: Local<Function> = v.try_into().unwrap();
    let r = f.call(scope, instance.into(), &[]).unwrap();
    assert!(r.is_number());
    assert_eq!(r.integer_value(scope).unwrap(), 42);
}

fn get_bool_from_method<'s>(
    scope: &v8::PinScope<'s, '_>,
    obj: Local<'s, Object>,
    key: &Local<'s, String>,
) -> bool {
    let v: Local<Value> = obj.get(scope, (*key).into()).unwrap();
    let f: Local<Function> = v.try_into().unwrap();
    let r = f.call(scope, obj.into(), &[]).unwrap();
    r.boolean_value(scope)
}
