//! Timer V8 bindings: setTimeout, setInterval, clearTimeout, clearInterval,
//! requestAnimationFrame, queueMicrotask.
//!
//! These are installed as global functions (not on __iv8__).

use crate::events::TaskKind;
use crate::state::RuntimeState;

/// Install timer globals on the V8 global object.
pub fn install_timer_globals(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    install_fn(scope, global, "setTimeout", set_timeout);
    install_fn(scope, global, "setInterval", set_interval);
    install_fn(scope, global, "clearTimeout", clear_timer);
    install_fn(scope, global, "clearInterval", clear_timer);
    install_fn(
        scope,
        global,
        "requestAnimationFrame",
        request_animation_frame,
    );
    install_fn(scope, global, "queueMicrotask", queue_microtask);
}

fn install_fn(
    scope: &v8::PinScope<'_, '_>,
    obj: v8::Local<v8::Object>,
    name: &str,
    callback: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
) {
    let tmpl = v8::FunctionTemplate::builder_raw(callback).build(scope);
    let func = crate::v8_utils::v8_fn(scope, &tmpl);
    let name_str = crate::v8_utils::v8_string(scope, name);
    func.set_name(name_str);
    obj.set(scope, name_str.into(), func.into());
}

/// setTimeout(fn, delay=0) → timerId
unsafe extern "C" fn set_timeout(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 || !args.get(0).is_function() {
            rv.set(v8::Integer::new(scope, 0).into());
            return;
        }

        let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(args.get(0)) };
        let delay_ms = if args.length() >= 2 {
            args.get(1).number_value(scope).unwrap_or(0.0).max(0.0)
        } else {
            0.0
        };

        let global_fn = v8::Global::new(scope, func);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let id = state
            .event_loop
            .borrow_mut()
            .add_timer(global_fn, delay_ms, TaskKind::Timeout);

        rv.set(v8::Integer::new(scope, id as i32).into());
    }));
}

/// setInterval(fn, delay=0) → timerId
unsafe extern "C" fn set_interval(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 || !args.get(0).is_function() {
            rv.set(v8::Integer::new(scope, 0).into());
            return;
        }

        let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(args.get(0)) };
        let delay_ms = if args.length() >= 2 {
            args.get(1).number_value(scope).unwrap_or(0.0).max(0.0)
        } else {
            0.0
        };

        let period_us = (delay_ms * 1000.0) as i64;
        let global_fn = v8::Global::new(scope, func);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let id = state.event_loop.borrow_mut().add_timer(
            global_fn,
            delay_ms,
            TaskKind::Interval {
                period_us: period_us.max(1000), // minimum 1ms interval
            },
        );

        rv.set(v8::Integer::new(scope, id as i32).into());
    }));
}

/// clearTimeout(id) / clearInterval(id)
unsafe extern "C" fn clear_timer(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 1 {
            return;
        }

        let id = args.get(0).uint32_value(scope).unwrap_or(0);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        state.event_loop.borrow_mut().remove_timer(id);
    }));
}

/// requestAnimationFrame(fn) → timerId
unsafe extern "C" fn request_animation_frame(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 || !args.get(0).is_function() {
            rv.set(v8::Integer::new(scope, 0).into());
            return;
        }

        let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(args.get(0)) };
        let global_fn = v8::Global::new(scope, func);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        // rAF fires on next frame (~16.67ms from now)
        let id = state
            .event_loop
            .borrow_mut()
            .add_timer(global_fn, 16.67, TaskKind::Raf);

        rv.set(v8::Integer::new(scope, id as i32).into());
    }));
}

/// queueMicrotask(fn) — uses V8's built-in microtask queue
unsafe extern "C" fn queue_microtask(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 1 || !args.get(0).is_function() {
            return;
        }

        let func: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(args.get(0)) };
        scope.enqueue_microtask(func);
    }));
}
