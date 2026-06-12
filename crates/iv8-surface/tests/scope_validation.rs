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

#[test]
fn test_500_templates_5_batches_with_inherit() {
    ensure_v8();
    // Use heap_limits matching the default init chain (EmbeddedV8Kernel::new)
    let mut isolate = v8::Isolate::new(
        v8::CreateParams::default()
            .heap_limits(512 * 1024 * 1024, 4usize * 1024 * 1024 * 1024),
    );
    v8::scope!(outer_scope, &mut isolate);

    let mut reg: HashMap<String, v8::Global<v8::FunctionTemplate>> = HashMap::new();
    const BATCH: usize = 100;
    const TOTAL: usize = 500;

    // Create inherit chain root in batch 1 — tested across batch boundary
    for b in 0..(TOTAL / BATCH) {
        let start = b * BATCH;
        {
            v8::scope!(let inner, outer_scope);
            for i in 0..BATCH {
                let idx = start + i;
                let name = format!("ScopeTest{}", idx);
                let tmpl = v8::FunctionTemplate::builder_raw(empty_constructor).build(inner);
                let key = v8::String::new(inner, &name).unwrap();
                tmpl.set_class_name(key);
                reg.insert(name, v8::Global::new(inner, tmpl));
            }
        }
    }

    assert_eq!(reg.len(), TOTAL, "500 templates survive 5 scope breaks");
}

#[test]
fn test_inherit_chain_across_batches() {
    ensure_v8();
    let mut isolate = v8::Isolate::new(
        v8::CreateParams::default()
            .heap_limits(512 * 1024 * 1024, 4usize * 1024 * 1024 * 1024),
    );
    v8::scope!(outer_scope, &mut isolate);

    let mut reg: HashMap<String, v8::Global<v8::FunctionTemplate>> = HashMap::new();

    // Batch 1: EventTarget (root)
    {
        v8::scope!(let inner, outer_scope);
        let et = v8::FunctionTemplate::builder_raw(empty_constructor).build(inner);
        et.set_class_name(v8::String::new(inner, "EventTarget").unwrap());
        reg.insert("EventTarget".into(), v8::Global::new(inner, et));
    }

    // Batch 2: Node inherits EventTarget (cross-batch parent lookup)
    {
        v8::scope!(let inner, outer_scope);
        let parent = reg.get("EventTarget")
            .map(|g| v8::Local::new(inner, g))
            .expect("parent should survive batch boundary");
        let node = v8::FunctionTemplate::builder_raw(empty_constructor).build(inner);
        node.inherit(parent);
        node.set_class_name(v8::String::new(inner, "Node").unwrap());
        reg.insert("Node".into(), v8::Global::new(inner, node));
    }

    // Batch 3: Element inherits Node
    {
        v8::scope!(let inner, outer_scope);
        let parent = reg.get("Node")
            .map(|g| v8::Local::new(inner, g))
            .expect("Node should survive batch boundary");
        let elem = v8::FunctionTemplate::builder_raw(empty_constructor).build(inner);
        elem.inherit(parent);
        elem.set_class_name(v8::String::new(inner, "Element").unwrap());
        reg.insert("Element".into(), v8::Global::new(inner, elem));
    }

    // Batch 4: HTMLElement inherits Element
    {
        v8::scope!(let inner, outer_scope);
        let parent = reg.get("Element")
            .map(|g| v8::Local::new(inner, g))
            .expect("Element should survive batch boundary");
        let html = v8::FunctionTemplate::builder_raw(empty_constructor).build(inner);
        html.inherit(parent);
        html.set_class_name(v8::String::new(inner, "HTMLElement").unwrap());
        reg.insert("HTMLElement".into(), v8::Global::new(inner, html));
    }

    // Batch 5: HTMLDivElement inherits HTMLElement
    {
        v8::scope!(let inner, outer_scope);
        let parent = reg.get("HTMLElement")
            .map(|g| v8::Local::new(inner, g))
            .expect("HTMLElement should survive batch boundary");
        let div = v8::FunctionTemplate::builder_raw(empty_constructor).build(inner);
        div.inherit(parent);
        div.set_class_name(v8::String::new(inner, "HTMLDivElement").unwrap());
        reg.insert("HTMLDivElement".into(), v8::Global::new(inner, div));
    }

    assert_eq!(reg.len(), 5, "all 5 templates survive across 5 batch boundaries");
    assert!(reg.contains_key("EventTarget"));
    assert!(reg.contains_key("HTMLDivElement"));
}
