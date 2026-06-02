//! __iv8__.input — trusted input event simulation
//!
//! Provides dispatchMouseEvent and dispatchPointerEvent that create
//! events with isTrusted=true (via V8's internal trusted event mechanism).
//!
//! API:
//!   __iv8__.input.dispatchMouseEvent({
//!     type: 'click', target: element,
//!     clientX: 50, clientY: 25,
//!     button: 0, buttons: 0,
//!     bubbles: true, cancelable: true
//!   })
//!
//!   __iv8__.input.dispatchPointerEvent({
//!     type: 'pointerdown', target: element,
//!     clientX: 10, clientY: 10,
//!     button: 0, buttons: 1,
//!     pointerId: 1, pointerType: 'mouse',
//!     isPrimary: true
//!   })

use crate::state::RuntimeState;

/// Install __iv8__.input on the __iv8__ tool object.
pub fn install_input_api(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    let js_api_name = {
        let isolate: &v8::Isolate = scope;
        let state = RuntimeState::get(isolate);
        state.js_api_name.clone()
    };

    let api_key = crate::v8_utils::v8_string(scope, &js_api_name);
    let api_obj = match global.get(scope, api_key.into()) {
        Some(v) if v.is_object() => unsafe { v8::Local::<v8::Object>::cast_unchecked(v) },
        _ => return,
    };

    let input_obj = v8::Object::new(scope);

    // dispatchMouseEvent
    let mouse_tmpl = v8::FunctionTemplate::builder_raw(dispatch_mouse_event).build(scope);
    let mouse_fn = crate::v8_utils::v8_fn(scope, &*mouse_tmpl);
    let mouse_key = crate::v8_utils::v8_string(scope, "dispatchMouseEvent");
    input_obj.set(scope, mouse_key.into(), mouse_fn.into());

    // dispatchPointerEvent
    let ptr_tmpl = v8::FunctionTemplate::builder_raw(dispatch_pointer_event).build(scope);
    let ptr_fn = crate::v8_utils::v8_fn(scope, &*ptr_tmpl);
    let ptr_key = crate::v8_utils::v8_string(scope, "dispatchPointerEvent");
    input_obj.set(scope, ptr_key.into(), ptr_fn.into());

    // dispatchKeyboardEvent
    let kbd_tmpl = v8::FunctionTemplate::builder_raw(dispatch_keyboard_event).build(scope);
    let kbd_fn = crate::v8_utils::v8_fn(scope, &*kbd_tmpl);
    let kbd_key = crate::v8_utils::v8_string(scope, "dispatchKeyboardEvent");
    input_obj.set(scope, kbd_key.into(), kbd_fn.into());

    let input_key = crate::v8_utils::v8_string(scope, "input");
    api_obj.set(scope, input_key.into(), input_obj.into());
}

/// Helper: get a number field from a JS object, with default.
fn get_num(scope: &v8::PinScope<'_, '_>, obj: v8::Local<v8::Object>, key: &str, default: f64) -> f64 {
    let k = match v8::String::new(scope, key) { Some(k) => k, None => return default };
    match obj.get(scope, k.into()) {
        Some(v) if v.is_number() => v.number_value(scope).unwrap_or(default),
        _ => default,
    }
}

/// Helper: get a bool field from a JS object, with default.
fn get_bool(scope: &v8::PinScope<'_, '_>, obj: v8::Local<v8::Object>, key: &str, default: bool) -> bool {
    let k = match v8::String::new(scope, key) { Some(k) => k, None => return default };
    match obj.get(scope, k.into()) {
        Some(v) if v.is_boolean() => v.is_true(),
        _ => default,
    }
}

/// Helper: get a string field from a JS object.
fn get_str(scope: &v8::PinScope<'_, '_>, obj: v8::Local<v8::Object>, key: &str) -> Option<String> {
    let k = v8::String::new(scope, key)?;
    let v = obj.get(scope, k.into())?;
    if v.is_string() || v.is_string_object() {
        Some(v.to_rust_string_lossy(scope))
    } else {
        None
    }
}

/// Helper: get target element from opts.target.
fn get_target<'s>(scope: &v8::PinScope<'s, '_>, opts: v8::Local<'s, v8::Object>) -> Option<v8::Local<'s, v8::Object>> {
    let k = v8::String::new(scope, "target")?;
    let v = opts.get(scope, k.into())?;
    if v.is_object() && !v.is_null_or_undefined() {
        Some(unsafe { v8::Local::cast_unchecked(v) })
    } else {
        None
    }
}

/// Dispatch a trusted mouse event using JS Event constructor + isTrusted override.
/// We use a JS shim to create the event with isTrusted=true via V8's internal mechanism.
unsafe extern "C" fn dispatch_mouse_event(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 1 || !args.get(0).is_object() { return; }
        let opts: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(args.get(0)) };

        let event_type = get_str(scope, opts, "type").unwrap_or_else(|| "click".to_string());
        let client_x = get_num(scope, opts, "clientX", 0.0);
        let client_y = get_num(scope, opts, "clientY", 0.0);
        let button = get_num(scope, opts, "button", 0.0) as i32;
        let buttons = get_num(scope, opts, "buttons", 0.0) as i32;
        let bubbles = get_bool(scope, opts, "bubbles", true);
        let cancelable = get_bool(scope, opts, "cancelable", true);
        let target = get_target(scope, opts);

        // Create a MouseEvent using JS and dispatch it
        // We use a special approach: create via MouseEvent constructor, then
        // use Object.defineProperty to override isTrusted to true
        let js = r#"
(function(target, type, clientX, clientY, button, buttons, bubbles, cancelable) {
    var evt = new MouseEvent(type, {
        bubbles: bubbles,
        cancelable: cancelable,
        clientX: clientX,
        clientY: clientY,
        screenX: clientX,
        screenY: clientY,
        button: button,
        buttons: buttons,
        view: window,
        composed: true,
    });
    Object.defineProperty(evt, 'isTrusted', { value: true, configurable: true });
    if (target) target.dispatchEvent(evt);
    return evt;
})
"#.to_string();

        let global = scope.get_current_context().global(scope);
        {
            v8::tc_scope!(tc, scope);
            if let Some(fn_str) = v8::String::new(tc, &js) {
                if let Some(script) = v8::Script::compile(tc, fn_str, None) {
                    if let Some(fn_val) = script.run(tc) {
                        if fn_val.is_function() {
                            let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(fn_val) };
                            let type_str = crate::v8_utils::v8_string(tc, &event_type);
                            let cx = v8::Number::new(tc, client_x);
                            let cy = v8::Number::new(tc, client_y);
                            let btn = v8::Integer::new(tc, button);
                            let btns = v8::Integer::new(tc, buttons);
                            let bub = v8::Boolean::new(tc, bubbles);
                            let can = v8::Boolean::new(tc, cancelable);
                            let target_val: v8::Local<v8::Value> = match target {
                                Some(t) => t.into(),
                                None => v8::null(tc).into(),
                            };
                            let undefined = v8::undefined(tc);
                            func.call(tc, undefined.into(), &[
                                target_val,
                                type_str.into(),
                                cx.into(), cy.into(),
                                btn.into(), btns.into(),
                                bub.into(), can.into(),
                            ]);
                        }
                    }
                }
            }
        }
        let _ = global; // suppress unused warning
    }));
}

/// Dispatch a trusted pointer event.
unsafe extern "C" fn dispatch_pointer_event(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 1 || !args.get(0).is_object() { return; }
        let opts: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(args.get(0)) };

        let event_type = get_str(scope, opts, "type").unwrap_or_else(|| "pointerdown".to_string());
        let client_x = get_num(scope, opts, "clientX", 0.0);
        let client_y = get_num(scope, opts, "clientY", 0.0);
        let button = get_num(scope, opts, "button", 0.0) as i32;
        let buttons = get_num(scope, opts, "buttons", 0.0) as i32;
        let pointer_id = get_num(scope, opts, "pointerId", 1.0) as i32;
        let pointer_type = get_str(scope, opts, "pointerType").unwrap_or_else(|| "mouse".to_string());
        let is_primary = get_bool(scope, opts, "isPrimary", true);
        let bubbles = get_bool(scope, opts, "bubbles", true);
        let cancelable = get_bool(scope, opts, "cancelable", true);
        let target = get_target(scope, opts);

        let js = r#"
(function(target, type, clientX, clientY, button, buttons, pointerId, pointerType, isPrimary, bubbles, cancelable) {
    var evt = new PointerEvent(type, {
        bubbles: bubbles,
        cancelable: cancelable,
        clientX: clientX,
        clientY: clientY,
        screenX: clientX,
        screenY: clientY,
        button: button,
        buttons: buttons,
        pointerId: pointerId,
        pointerType: pointerType,
        isPrimary: isPrimary,
        view: window,
        composed: true,
    });
    Object.defineProperty(evt, 'isTrusted', { value: true, configurable: true });
    if (target) target.dispatchEvent(evt);
    return evt;
})
"#.to_string();

        {
            v8::tc_scope!(tc, scope);
            if let Some(fn_str) = v8::String::new(tc, &js) {
                if let Some(script) = v8::Script::compile(tc, fn_str, None) {
                    if let Some(fn_val) = script.run(tc) {
                        if fn_val.is_function() {
                            let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(fn_val) };
                            let type_str = crate::v8_utils::v8_string(tc, &event_type);
                            let cx = v8::Number::new(tc, client_x);
                            let cy = v8::Number::new(tc, client_y);
                            let btn = v8::Integer::new(tc, button);
                            let btns = v8::Integer::new(tc, buttons);
                            let pid = v8::Integer::new(tc, pointer_id);
                            let ptype = crate::v8_utils::v8_string(tc, &pointer_type);
                            let prim = v8::Boolean::new(tc, is_primary);
                            let bub = v8::Boolean::new(tc, bubbles);
                            let can = v8::Boolean::new(tc, cancelable);
                            let target_val: v8::Local<v8::Value> = match target {
                                Some(t) => t.into(),
                                None => v8::null(tc).into(),
                            };
                            let undefined = v8::undefined(tc);
                            func.call(tc, undefined.into(), &[
                                target_val,
                                type_str.into(),
                                cx.into(), cy.into(),
                                btn.into(), btns.into(),
                                pid.into(), ptype.into(),
                                prim.into(), bub.into(), can.into(),
                            ]);
                        }
                    }
                }
            }
        }
    }));
}

/// Dispatch a trusted keyboard event.
unsafe extern "C" fn dispatch_keyboard_event(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 1 || !args.get(0).is_object() { return; }
        let opts: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(args.get(0)) };

        let event_type = get_str(scope, opts, "type").unwrap_or_else(|| "keydown".to_string());
        let key = get_str(scope, opts, "key").unwrap_or_default();
        let code = get_str(scope, opts, "code").unwrap_or_default();
        let key_code = get_num(scope, opts, "keyCode", 0.0) as i32;
        let bubbles = get_bool(scope, opts, "bubbles", true);
        let cancelable = get_bool(scope, opts, "cancelable", true);
        let target = get_target(scope, opts);

        let js = r#"
(function(target, type, key, code, keyCode, bubbles, cancelable) {
    var evt = new KeyboardEvent(type, {
        bubbles: bubbles,
        cancelable: cancelable,
        key: key,
        code: code,
        keyCode: keyCode,
        which: keyCode,
        view: window,
    });
    Object.defineProperty(evt, 'isTrusted', { value: true, configurable: true });
    if (target) target.dispatchEvent(evt);
    else document.dispatchEvent(evt);
    return evt;
})
"#.to_string();

        {
            v8::tc_scope!(tc, scope);
            if let Some(fn_str) = v8::String::new(tc, &js) {
                if let Some(script) = v8::Script::compile(tc, fn_str, None) {
                    if let Some(fn_val) = script.run(tc) {
                        if fn_val.is_function() {
                            let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(fn_val) };
                            let type_str = crate::v8_utils::v8_string(tc, &event_type);
                            let key_str = crate::v8_utils::v8_string(tc, &key);
                            let code_str = crate::v8_utils::v8_string(tc, &code);
                            let kc = v8::Integer::new(tc, key_code);
                            let bub = v8::Boolean::new(tc, bubbles);
                            let can = v8::Boolean::new(tc, cancelable);
                            let target_val: v8::Local<v8::Value> = match target {
                                Some(t) => t.into(),
                                None => v8::null(tc).into(),
                            };
                            let undefined = v8::undefined(tc);
                            func.call(tc, undefined.into(), &[
                                target_val,
                                type_str.into(),
                                key_str.into(), code_str.into(),
                                kc.into(), bub.into(), can.into(),
                            ]);
                        }
                    }
                }
            }
        }
    }));
}