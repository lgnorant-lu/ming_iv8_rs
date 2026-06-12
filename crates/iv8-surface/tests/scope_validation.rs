//! v0.8.26 T-1: Validate v8::scope! macro for nested HandleScope creation.
//! Proves: nested scope (via v8::scope!) + Global survival across scope drops.

use std::sync::Once;
use std::collections::HashMap;

static V8_INIT: Once = Once::new();

fn ensure_v8() {
    V8_INIT.call_once(|| {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });
}

unsafe extern "C" fn empty_constructor(_info: *const v8::FunctionCallbackInfo) {}

#[test]
fn test_nested_scope_global_survival() {
    ensure_v8();
    let mut isolate = v8::Isolate::new(v8::CreateParams::default());
    v8::scope!(outer_scope, &mut isolate);

    let mut templates: HashMap<String, v8::Global<v8::FunctionTemplate>> = HashMap::new();

    {
        v8::scope!(let inner, outer_scope);
        for i in 0..100 {
            let name = format!("TestClass{}", i);
            let tmpl = v8::FunctionTemplate::builder_raw(empty_constructor).build(inner);
            let key = v8::String::new(inner, &name).unwrap();
            tmpl.set_class_name(key);
            templates.insert(name, v8::Global::new(inner, tmpl));
        }
    }

    {
        v8::scope!(let inner, outer_scope);
        for i in 100..200 {
            let name = format!("TestClass{}", i);
            let tmpl = v8::FunctionTemplate::builder_raw(empty_constructor).build(inner);
            let key = v8::String::new(inner, &name).unwrap();
            tmpl.set_class_name(key);
            templates.insert(name, v8::Global::new(inner, tmpl));
        }
    }

    assert_eq!(templates.len(), 200, "200 templates survive 2 scope breaks");
}
