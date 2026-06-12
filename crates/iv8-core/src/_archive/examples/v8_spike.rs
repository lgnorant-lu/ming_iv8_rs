// v8_spike — V8 Prototype Spike for IV8 v0.8.18+ Blocker Verification
//
// Tests:
//   B-1: Window prototype chain construction (global.__proto__ = Window.prototype)
//   B-1-Extra: Context::new with global_template approach
//   B-2: globalThis self-reference protocol (window === self === frames)
//   B-3: MarkAsUndetectable / SetCallAsFunctionHandler FFI feasibility
//   C-2: Dual context execution model
//
// Targeting: rusty_v8 v147.4.0 API (ContextScope RAII, no ctx.enter/exit)

#![allow(unused, clippy::expect_used, clippy::unwrap_used)]

// Link the C++ wrapper providing MarkAsUndetectable and SetCallAsFunctionHandler.
// The library is compiled by iv8-core's build.rs into OUT_DIR.
#[link(name = "iv8_v8_extra", kind = "static")]
extern "C" {}

use std::ffi::c_void;

// ============================================================================
// Helpers
// ============================================================================

fn v8_str<'s>(scope: &v8::PinScope<'s, '_, ()>, s: &str) -> v8::Local<'s, v8::String> {
    v8::String::new(scope, s).unwrap()
}

/// Compile & run JS in the current context (context is implicit from scope).
fn eval_js<'s>(
    scope: &mut v8::PinScope<'s, '_>,
    code: &str,
) -> Option<v8::Local<'s, v8::Value>> {
    let src = v8_str(scope, code);
    let script = v8::Script::compile(&*scope, src, None)?;
    script.run(&*scope)
}

/// Same as eval_js but returns the result as a Rust String.
fn eval_str(
    scope: &mut v8::PinScope<'_, '_>,
    code: &str,
) -> String {
    eval_js(scope, code)
        .and_then(|v| v.to_string(&*scope))
        .map(|s| s.to_rust_string_lossy(&*scope))
        .unwrap_or_else(|| "<EVAL_ERROR>".into())
}

unsafe extern "C" fn noop_cb(_: *const v8::FunctionCallbackInfo) {}

unsafe extern "C" fn global_self_cb(info: *const v8::FunctionCallbackInfo) {
    let info_ref = &*info;
    v8::callback_scope!(unsafe scope, info_ref);
    let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
    let ctx = scope.get_current_context();
    let g = ctx.global(&*scope);
    rv.set(g.into());
}

fn make_tmpl<'s>(
    scope: &v8::PinScope<'s, '_, ()>,
    name: &str,
) -> v8::Local<'s, v8::FunctionTemplate> {
    let t = v8::FunctionTemplate::builder_raw(noop_cb).build(scope);
    t.set_class_name(v8_str(scope, name));
    t
}

fn report(label: &str, pass: bool, detail: &str) {
    let tag = if pass { "PASS" } else { "FAIL" };
    println!("  [{}] {} -- {}", tag, label, detail);
}

/// Extract the raw V8 pointer from a Local<T> for FFI calls.
/// Local<T> is repr(transparent) over NonNull<T> + PhantomData.
unsafe fn local_to_raw<T>(local: v8::Local<'_, T>) -> *const c_void {
    let nn: std::ptr::NonNull<T> = std::mem::transmute(local);
    nn.as_ptr() as *const c_void
}

// ============================================================================
// B-1: Window Prototype Chain via Object::set_prototype
// ============================================================================

fn spike_b1(iso: &mut v8::Isolate) {
    println!("\n=== B-1: Window Prototype Chain (manual set_prototype chaining) ===\n");

    v8::scope!(let hs, iso);
    let ctx = v8::Context::new(hs, Default::default());
    let scope = &mut v8::ContextScope::new(hs, ctx);

    // Build chain bottom-up: EventTarget -> WindowProperties -> Window -> global
    // Key insight: FunctionTemplate::inherit() does NOT compose prototype objects.
    // We must manually chain them with Object::set_prototype.

    // 1. EventTarget (base)
    let et = make_tmpl(scope, "EventTarget");
    {
        let p = et.prototype_template(scope);
        let ae = v8::FunctionTemplate::builder_raw(noop_cb).build(scope);
        ae.set_class_name(v8_str(scope, "addEventListener"));
        p.set(v8_str(scope, "addEventListener").into(), ae.into());
        let rm = v8::FunctionTemplate::builder_raw(noop_cb).build(scope);
        rm.set_class_name(v8_str(scope, "removeEventListener"));
        p.set(v8_str(scope, "removeEventListener").into(), rm.into());
        let de = v8::FunctionTemplate::builder_raw(noop_cb).build(scope);
        de.set_class_name(v8_str(scope, "dispatchEvent"));
        p.set(v8_str(scope, "dispatchEvent").into(), de.into());
        p.set(
            v8::Symbol::get_to_string_tag(scope).into(),
            v8_str(scope, "EventTarget").into(),
        );
    }

    // 2. WindowProperties (inherits EventTarget)
    let wp = make_tmpl(scope, "WindowProperties");
    {
        let p = wp.prototype_template(scope);
        p.set(
            v8::Symbol::get_to_string_tag(scope).into(),
            v8_str(scope, "WindowProperties").into(),
        );
        let len_getter = v8::FunctionTemplate::builder_raw(noop_cb).build(scope);
        p.set_accessor_property(
            v8_str(scope, "length").into(),
            Some(len_getter),
            None,
            v8::PropertyAttribute::DONT_DELETE,
        );
    }

    // 3. Window (inherits WindowProperties)
    let wt = make_tmpl(scope, "Window");
    {
        let p = wt.prototype_template(scope);
        let alert = v8::FunctionTemplate::builder_raw(noop_cb).build(scope);
        alert.set_class_name(v8_str(scope, "alert"));
        p.set(v8_str(scope, "alert").into(), alert.into());

        let st = v8::FunctionTemplate::builder_raw(noop_cb).build(scope);
        st.set_class_name(v8_str(scope, "setTimeout"));
        p.set(v8_str(scope, "setTimeout").into(), st.into());

        let fetch = v8::FunctionTemplate::builder_raw(noop_cb).build(scope);
        fetch.set_class_name(v8_str(scope, "fetch"));
        p.set(v8_str(scope, "fetch").into(), fetch.into());

        p.set(
            v8::Symbol::get_to_string_tag(scope).into(),
            v8_str(scope, "Window").into(),
        );
    }

    // 4. Instantiate all three and manually chain prototypes
    let et_proto = et.prototype_template(scope).new_instance(&*scope)
        .expect("EventTarget proto instance");
    let wp_proto = wp.prototype_template(scope).new_instance(&*scope)
        .expect("WindowProperties proto instance");
    let win_proto = wt.prototype_template(scope).new_instance(&*scope)
        .expect("Window proto instance");

    // Chain: Window.prototype -> WindowProperties.prototype -> EventTarget.prototype
    wp_proto.set_prototype(&*scope, et_proto.into());
    win_proto.set_prototype(&*scope, wp_proto.into());

    let global = ctx.global(&*scope);
    let r = global.set_prototype(&*scope, win_proto.into());
    report("set_prototype succeeds", r == Some(true), &format!("{:?}", r));

    // Set up window = globalThis (self-reference)
    let global_val: v8::Local<v8::Value> = global.into();
    global.set(&*scope, v8_str(&*scope, "window").into(), global_val);

    // ---- Verification ----

    let chain = eval_str(scope, r#"
        (function() {
            var c = [], o = window;
            while (o) {
                var n = o.constructor ? o.constructor.name : '(anon)';
                var t = o[Symbol.toStringTag];
                c.push(n + (t ? '['+t+']' : ''));
                o = Object.getPrototypeOf(o);
            }
            return c.join(' -> ');
        })()
    "#);
    report("prototype chain depth", chain.contains("[Window]") && chain.contains("[EventTarget]") && chain.contains("[WindowProperties]"), &chain);

    let props = eval_str(scope, r#"
        JSON.stringify({
            alert: typeof window.alert,
            setTimeout: typeof window.setTimeout,
            fetch: typeof window.fetch,
            addEventListener: typeof window.addEventListener,
        })
    "#);
    report("methods accessible", props.contains("function"), &props);

    let fmt = eval_str(scope,
        "Function.prototype.toString.call(window.alert)");
    report("Function.toString format", fmt.contains("[native code]"), &fmt);

    let loc = eval_str(scope, r#"
        JSON.stringify({
            alertOnWinProto: Object.getPrototypeOf(window).hasOwnProperty('alert'),
            aeOnETProto: (function(){
                var p = Object.getPrototypeOf(Object.getPrototypeOf(Object.getPrototypeOf(window)));
                return p ? p.hasOwnProperty('addEventListener') : false;
            })(),
        })
    "#);
    report("property location", loc.contains("true"), &loc);

    let tag = eval_str(scope, "Object.prototype.toString.call(window)");
    report("toString.call(window)", tag == "[object Window]", &tag);

    let own = eval_str(scope, r#"
        JSON.stringify(Object.getOwnPropertyNames(Object.getPrototypeOf(window)))
    "#);
    report("own props on Window.prototype", own.contains("alert"), &own);
}

// ============================================================================
// B-1-Extra: Context::new with global_template
// ============================================================================

fn spike_b1_extra(iso: &mut v8::Isolate) {
    println!("\n=== B-1-Extra: Context::new with global_template ===\n");

    v8::scope!(let hs, iso);

    let wt = make_tmpl(hs, "Window");
    {
        let p = wt.prototype_template(hs);
        let alert = v8::FunctionTemplate::builder_raw(noop_cb).build(hs);
        alert.set_class_name(v8_str(hs, "alert"));
        p.set(v8_str(hs, "alert").into(), alert.into());
        p.set(
            v8::Symbol::get_to_string_tag(hs).into(),
            v8_str(hs, "Window").into(),
        );
    }

    // Use instance_template as global template
    let gt = wt.instance_template(hs);
    let ctx = v8::Context::new(hs, v8::ContextOptions {
        global_template: Some(gt),
        ..Default::default()
    });
    let scope = &mut v8::ContextScope::new(hs, ctx);

    // Set up window = globalThis (self-reference)
    {
        let g = ctx.global(&*scope);
        let gv: v8::Local<v8::Value> = g.into();
        g.set(&*scope, v8_str(&*scope, "window").into(), gv);
    }

    let chain = eval_str(scope, r#"
        (function() {
            var c = [], o = window;
            while (o) {
                var t = o[Symbol.toStringTag] || o.constructor?.name || '(anon)';
                c.push(t);
                o = Object.getPrototypeOf(o);
            }
            return c.join(' -> ');
        })()
    "#);
    report("prototype chain", !chain.is_empty(), &chain);

    let has_alert = eval_str(scope, "typeof window.alert");
    report("alert accessible", has_alert == "function", &has_alert);

    let ts = eval_str(scope, "Object.prototype.toString.call(window)");
    report("toString", ts == "[object Window]", &ts);

    let desc = eval_str(scope, r#"
        (function() {
            var p = Object.getPrototypeOf(window);
            return JSON.stringify({
                protoNotNull: p !== null && p !== undefined,
                hasAlert: p ? p.hasOwnProperty('alert') : false,
                protoTag: p ? (p[Symbol.toStringTag] || null) : null
            });
        })()
    "#);
    report("prototype structure", desc.contains("true"), &desc);

    // Check: can we still set additional properties on global?
    let set_ok = eval_str(scope, r#"
        (function() {
            window.__test_prop__ = 42;
            return window.__test_prop__;
        })()
    "#);
    report("can set custom props on global", set_ok == "42", &set_ok);
}

// ============================================================================
// B-2: Global Self-Reference Protocol
// ============================================================================

unsafe extern "C" fn replaceable_setter(info: *const v8::FunctionCallbackInfo) {
    let info_ref = &*info;
    v8::callback_scope!(unsafe scope, info_ref);
    // [Replaceable] semantics: delete the accessor and create a data property
    let ctx = scope.get_current_context();
    let global = ctx.global(&*scope);
    let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
    let value = args.get(0);
    let name = v8_str(&*scope, "window");
    // Delete the accessor property, then set as a plain data property
    global.delete(&*scope, name.into());
    global.set(&*scope, name.into(), value);
}

fn spike_b2(iso: &mut v8::Isolate) {
    println!("\n=== B-2: Global Self-Reference Protocol ===\n");

    v8::scope!(let hs, iso);

    let gt = v8::ObjectTemplate::new(hs);

    // window -- accessor with getter + replaceable setter
    let wg = v8::FunctionTemplate::builder_raw(global_self_cb).build(hs);
    let ws = v8::FunctionTemplate::builder_raw(replaceable_setter).build(hs);
    gt.set_accessor_property(v8_str(hs, "window").into(), Some(wg), Some(ws), v8::PropertyAttribute::NONE);

    // self -- accessor, enumerable + configurable
    let sg = v8::FunctionTemplate::builder_raw(global_self_cb).build(hs);
    gt.set_accessor_property(v8_str(hs, "self").into(), Some(sg), None, v8::PropertyAttribute::NONE);

    // frames -- accessor, non-configurable (simulates [LegacyUnforgeable])
    let fg = v8::FunctionTemplate::builder_raw(global_self_cb).build(hs);
    gt.set_accessor_property(v8_str(hs, "frames").into(), Some(fg), None, v8::PropertyAttribute::DONT_DELETE);

    let ctx = v8::Context::new(hs, v8::ContextOptions {
        global_template: Some(gt),
        ..Default::default()
    });
    let scope = &mut v8::ContextScope::new(hs, ctx);

    let eq = eval_str(scope, r#"
        JSON.stringify({
            w_eq_s: window === self,
            s_eq_f: self === frames,
            f_eq_g: frames === globalThis,
            w_eq_g: window === globalThis,
            w_eq_ww: window === window.window,
        })
    "#);
    report("equality chain", eq.contains("true"), &eq);

    let desc = eval_str(scope, r#"
        JSON.stringify({
            w: Object.getOwnPropertyDescriptor(window, 'window'),
            s: Object.getOwnPropertyDescriptor(window, 'self'),
            f: Object.getOwnPropertyDescriptor(window, 'frames'),
        })
    "#);
    report("descriptors exist", desc.contains("enumerable"), &desc);

    let types = eval_str(scope, r#"
        JSON.stringify({
            w: typeof window, s: typeof self,
            f: typeof frames, g: typeof globalThis,
        })
    "#);
    report("typeof all 'object'", types.matches("\"object\"").count() == 4, &types);

    let enum_check = eval_str(scope, r#"
        JSON.stringify({
            w_in: 'window' in window,
            s_in: 'self' in window,
            f_in: 'frames' in window,
            f_enumerable: Object.getOwnPropertyDescriptor(window, 'frames').enumerable,
            w_enumerable: Object.getOwnPropertyDescriptor(window, 'window').enumerable,
        })
    "#);
    report("in/enumerable checks", enum_check.contains("true"), &enum_check);

    // Check: can `window` be overwritten (Replaceable semantics)?
    // Our replaceable_setter deletes the accessor and creates a data property
    let replace = eval_str(scope, r#"
        (function() {
            window = 42;
            var afterAssign = (window === 42);
            // Restore: delete the data property, re-define accessor is too complex for this test
            delete window;
            var restoredType = typeof window;
            return JSON.stringify({afterAssign: afterAssign, restored: restoredType});
        })()
    "#);
    report("Replaceable semantics", replace.contains("\"afterAssign\":true"), &replace);
}

// ============================================================================
// B-3: MarkAsUndetectable + SetCallAsFunctionHandler (FFI probe)
// ============================================================================

// These FFI symbols are provided by our custom C++ wrapper (cxx/iv8_v8_extra.cc)
// compiled via build.rs and linked into the final binary.
extern "C" {
    fn v8__ObjectTemplate__MarkAsUndetectable(this: *const c_void);
    fn v8__ObjectTemplate__SetCallAsFunctionHandler(
        this: *const c_void,
        callback: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
        data: *const c_void,
    );
}

unsafe extern "C" fn callable_handler(info: *const v8::FunctionCallbackInfo) {
    let info_ref = &*info;
    v8::callback_scope!(unsafe scope, info_ref);
    let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);
    rv.set(v8_str(&*scope, "called_from_handler").into());
}

fn spike_b3(iso: &mut v8::Isolate) {
    println!("\n=== B-3: MarkAsUndetectable + SetCallAsFunctionHandler (FFI) ===\n");

    // --- Test 1: JS-only limitation ---
    {
        v8::scope!(let hs, iso);
        let ctx = v8::Context::new(hs, Default::default());
        let scope = &mut v8::ContextScope::new(hs, ctx);

        let sim = eval_str(scope, r#"
            (function() {
                var obj = {};
                Object.defineProperty(obj, '__iv8__', {
                    value: { x: 1 }, enumerable: false, configurable: false, writable: false
                });
                return JSON.stringify({
                    in_check: '__iv8__' in obj,
                    keys_hidden: !Object.keys(obj).includes('__iv8__'),
                    getOwnPropNames_finds: Object.getOwnPropertyNames(obj).includes('__iv8__'),
                });
            })()
        "#);
        report("JS-only hide attempt (in_check=true proves insufficient)", sim.contains("\"in_check\":true"), &sim);
        println!("  -> JS-only cannot hide from 'in' operator. FFI MarkAsUndetectable is required.\n");
    }

    // --- Test 2: SetCallAsFunctionHandler via FFI (standalone) ---
    {
        v8::scope!(let hs, iso);
        let ctx = v8::Context::new(hs, Default::default());
        let scope = &mut v8::ContextScope::new(hs, ctx);

        println!("  Attempting FFI: v8__ObjectTemplate__SetCallAsFunctionHandler...");
        let callable_tmpl = v8::ObjectTemplate::new(&*scope);
        callable_tmpl.set_internal_field_count(1);
        unsafe {
            let raw = local_to_raw(callable_tmpl);
            v8__ObjectTemplate__SetCallAsFunctionHandler(
                raw,
                callable_handler,
                std::ptr::null(),
            );
        }
        println!("  [OK] SetCallAsFunctionHandler FFI call succeeded (no crash)\n");

        let callable_obj = callable_tmpl.new_instance(&*scope).expect("callable instance");
        let g = ctx.global(&*scope);
        g.set(&*scope, v8_str(&*scope, "testCallable").into(), callable_obj.into());

        let typeof_result = eval_str(scope, "typeof testCallable");
        // Note: SetCallAsFunctionHandler makes the object callable, so typeof
        // returns "function" (not "object"). This is V8's intended behavior.
        report("callable typeof is 'function'", typeof_result == "function", &typeof_result);

        let call_result = eval_str(scope, r#"
            (function() {
                try {
                    var r = testCallable();
                    return 'called: ' + r;
                } catch(e) {
                    return 'error: ' + e.message;
                }
            })()
        "#);
        report("callable invocation works", call_result.contains("called"), &call_result);
    }

    // --- Test 3: MarkAsUndetectable + SetCallAsFunctionHandler via FFI ---
    // NOTE: V8 requires SetCallAsFunctionHandler to be set BEFORE MarkAsUndetectable.
    // MarkAsUndetectable alone triggers a V8 assertion failure at new_instance() time.
    {
        v8::scope!(let hs, iso);
        let ctx = v8::Context::new(hs, Default::default());
        let scope = &mut v8::ContextScope::new(hs, ctx);

        println!("\n  Attempting FFI: SetCallAsFunctionHandler + MarkAsUndetectable...");
        let tmpl = v8::ObjectTemplate::new(&*scope);
        tmpl.set_internal_field_count(1);
        tmpl.set(v8_str(&*scope, "__secret__").into(), v8_str(&*scope, "hidden_value").into());

        unsafe {
            let raw = local_to_raw(tmpl);
            // SetCallAsFunctionHandler MUST come before MarkAsUndetectable
            v8__ObjectTemplate__SetCallAsFunctionHandler(
                raw,
                callable_handler,
                std::ptr::null(),
            );
            v8__ObjectTemplate__MarkAsUndetectable(raw);
        }
        println!("  [OK] Both FFI calls succeeded (no crash)\n");

        let inst = tmpl.new_instance(&*scope).expect("undetectable instance");
        let global = ctx.global(&*scope);
        global.set(&*scope, v8_str(&*scope, "__undetect_test__").into(), inst.into());

        let undetect_result = eval_str(scope, r#"
            (function() {
                var obj = globalThis.__undetect_test__;
                return JSON.stringify({
                    typeof_obj: typeof obj,
                    loose_eq_undefined: obj == undefined,
                    strict_eq_undefined: obj === undefined,
                    in_check_secret: '__secret__' in obj,
                    keys_count: Object.keys(obj).length,
                    direct_access_secret: obj.__secret__,
                    getOwnPropNames: Object.getOwnPropertyNames(obj),
                    is_truthy: !!obj,
                });
            })()
        "#);
        report(
            "MarkAsUndetectable: typeof === 'undefined'",
            undetect_result.contains("\"typeof_obj\":\"undefined\""),
            &undetect_result,
        );
        report(
            "MarkAsUndetectable: == undefined",
            undetect_result.contains("\"loose_eq_undefined\":true"),
            &undetect_result,
        );
        report(
            "MarkAsUndetectable: !== undefined (strict)",
            undetect_result.contains("\"strict_eq_undefined\":false"),
            &undetect_result,
        );
        report(
            "MarkAsUndetectable: props still accessible",
            undetect_result.contains("\"direct_access_secret\":\"hidden_value\""),
            &undetect_result,
        );
    }

    // --- Test 4: Combined — document.all emulation (MarkAsUndetectable + SetCallAsFunctionHandler) ---
    {
        v8::scope!(let hs, iso);
        let ctx = v8::Context::new(hs, Default::default());
        let scope = &mut v8::ContextScope::new(hs, ctx);

        println!("\n  Testing combined: MarkAsUndetectable + SetCallAsFunctionHandler (document.all)...");
        let all_tmpl = v8::ObjectTemplate::new(&*scope);
        all_tmpl.set_internal_field_count(1);
        unsafe {
            let raw = local_to_raw(all_tmpl);
            // SetCallAsFunctionHandler MUST come before MarkAsUndetectable
            v8__ObjectTemplate__SetCallAsFunctionHandler(
                raw,
                callable_handler,
                std::ptr::null(),
            );
            v8__ObjectTemplate__MarkAsUndetectable(raw);
        }

        let all_obj = all_tmpl.new_instance(&*scope).expect("all instance");
        let g = ctx.global(&*scope);
        g.set(&*scope, v8_str(&*scope, "testDocAll").into(), all_obj.into());

        let doc_all_result = eval_str(scope, r#"
            (function() {
                var a = testDocAll;
                return JSON.stringify({
                    typeof: typeof a,
                    loose_eq_undefined: a == undefined,
                    strict_eq_undefined: a === undefined,
                    is_truthy: !!a,
                    callable: (function(){ try { var r = a(); return r; } catch(e){ return 'err:'+e.message; } })(),
                });
            })()
        "#);
        report(
            "document.all: typeof === 'undefined'",
            doc_all_result.contains("\"typeof\":\"undefined\""),
            &doc_all_result,
        );
        report(
            "document.all: == undefined",
            doc_all_result.contains("\"loose_eq_undefined\":true"),
            &doc_all_result,
        );
        report(
            "document.all: !== undefined (strict)",
            doc_all_result.contains("\"strict_eq_undefined\":false"),
            &doc_all_result,
        );
        report(
            "document.all: callable as function",
            doc_all_result.contains("\"callable\":\"called_from_handler\""),
            &doc_all_result,
        );
    }
}

// ============================================================================
// C-2: Dual Context Execution Model
// ============================================================================

fn spike_c2(iso: &mut v8::Isolate) {
    println!("\n=== C-2: Dual Context Execution Model ===\n");

    v8::scope!(let hs, iso);

    // Context A -- "old" path (flat global)
    let ctx_a = v8::Context::new(hs, Default::default());

    // Context B -- "new" path (Window.prototype chain)
    let ctx_b = v8::Context::new(hs, Default::default());

    // Configure A
    {
        let scope = &mut v8::ContextScope::new(hs, ctx_a);
        let g = ctx_a.global(&*scope);
        // Set window = globalThis
        let gv: v8::Local<v8::Value> = g.into();
        g.set(&*scope, v8_str(&*scope, "window").into(), gv);
        g.set(&*scope, v8_str(&*scope, "__path__").into(), v8_str(&*scope, "old_flat").into());
        let nav = v8::Object::new(&*scope);
        nav.set(&*scope, v8_str(&*scope, "userAgent").into(), v8_str(&*scope, "old-UA").into());
        g.set(&*scope, v8_str(&*scope, "navigator").into(), nav.into());
    } // ctx_a scope dropped, context exited

    // Configure B
    {
        let scope = &mut v8::ContextScope::new(hs, ctx_b);
        let wt = make_tmpl(scope, "Window");
        {
            let p = wt.prototype_template(scope);
            let f = v8::FunctionTemplate::builder_raw(noop_cb).build(scope);
            f.set_class_name(v8_str(scope, "fetch"));
            p.set(v8_str(scope, "fetch").into(), f.into());
            p.set(v8::Symbol::get_to_string_tag(scope).into(), v8_str(scope, "Window").into());
        }
        let wp = wt.prototype_template(scope).new_instance(&*scope).unwrap();
        let g = ctx_b.global(&*scope);
        g.set_prototype(&*scope, wp.into());
        // Set window = globalThis
        let gv: v8::Local<v8::Value> = g.into();
        g.set(&*scope, v8_str(&*scope, "window").into(), gv);
        g.set(&*scope, v8_str(&*scope, "__path__").into(), v8_str(&*scope, "new_surface").into());
    } // ctx_b scope dropped

    // Test isolation: A
    {
        let scope = &mut v8::ContextScope::new(hs, ctx_a);
        let p = eval_str(scope, "window.__path__");
        report("Context A path", p == "old_flat", &p);
        let f = eval_str(scope, "typeof window.fetch");
        report("Context A no fetch", f == "undefined", &f);
        let ua = eval_str(scope, "navigator ? navigator.userAgent : 'none'");
        report("Context A navigator", ua == "old-UA", &ua);
    }

    // Test isolation: B
    {
        let scope = &mut v8::ContextScope::new(hs, ctx_b);
        let p = eval_str(scope, "window.__path__");
        report("Context B path", p == "new_surface", &p);
        let f = eval_str(scope, "typeof window.fetch");
        report("Context B has fetch", f == "function", &f);
        let nav = eval_str(scope, "typeof navigator");
        report("Context B no old navigator", nav == "undefined", &nav);
        let chain = eval_str(scope, r#"
            (function() {
                var c = [], o = window;
                while (o) {
                    var t = o[Symbol.toStringTag] || o.constructor?.name || '(anon)';
                    c.push(t);
                    o = Object.getPrototypeOf(o);
                }
                return c.join(' -> ');
            })()
        "#);
        report("Context B prototype chain", chain.contains("Window"), &chain);
    }

    // Cross-context rapid switching
    {
        let a = {
            let scope = &mut v8::ContextScope::new(hs, ctx_a);
            eval_str(scope, "typeof window.fetch + '|' + window.__path__")
        };

        let b = {
            let scope = &mut v8::ContextScope::new(hs, ctx_b);
            eval_str(scope, "typeof window.fetch + '|' + window.__path__")
        };

        report("Cross-context A", a == "undefined|old_flat", &a);
        report("Cross-context B", b == "function|new_surface", &b);
    }

    // Rapid switches
    {
        let mut ok = true;
        let mut fail_detail = String::new();
        for i in 0..10 {
            let a = {
                let scope = &mut v8::ContextScope::new(hs, ctx_a);
                eval_str(scope, "window.__path__")
            };
            let b = {
                let scope = &mut v8::ContextScope::new(hs, ctx_b);
                eval_str(scope, "window.__path__")
            };
            if a != "old_flat" || b != "new_surface" {
                ok = false;
                fail_detail = format!("iteration {}: A={}, B={}", i, a, b);
                break;
            }
        }
        report("10 rapid context switches", ok, if ok { "All iterations isolated" } else { &fail_detail });
    }
}

// ============================================================================
// main
// ============================================================================

fn main() {
    println!("=== IV8 v0.8.18+ V8 Prototype Spike ===");
    println!("Testing 4 blocker-level questions against rusty_v8 v147\n");

    let platform = v8::new_default_platform(0, false).make_shared();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();
    println!("V8 initialized.\n");

    {
        let mut iso = v8::Isolate::new(v8::CreateParams::default());
        spike_b1(&mut iso);
    }
    {
        let mut iso = v8::Isolate::new(v8::CreateParams::default());
        spike_b1_extra(&mut iso);
    }
    {
        let mut iso = v8::Isolate::new(v8::CreateParams::default());
        spike_b2(&mut iso);
    }
    {
        let mut iso = v8::Isolate::new(v8::CreateParams::default());
        spike_b3(&mut iso);
    }
    // spike_b3_part2_reference is intentionally not called (FFI symbols missing)
    {
        let mut iso = v8::Isolate::new(v8::CreateParams::default());
        spike_c2(&mut iso);
    }

    println!("\n=== All spikes completed ===");

    unsafe {
        v8::V8::dispose();
        v8::V8::dispose_platform();
    }
}
