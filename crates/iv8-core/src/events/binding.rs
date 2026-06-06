//! EventLoop V8 bindings: install __iv8__.eventLoop.{advance,sleep,tick,drain,...}
//!
//! Uses the borrow-release-execute-reborrow pattern to avoid RefCell reentrancy.

use crate::events::event_loop::{run_due_tasks, run_one_due_task};
use crate::state::RuntimeState;

/// Install eventLoop API on the __iv8__ tool object.
pub fn install_event_loop_bindings(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    let js_api_name = {
        let isolate: &v8::Isolate = scope;
        let state = RuntimeState::get(isolate);
        state.js_api_name.clone()
    };

    let api_key = crate::v8_utils::v8_string(scope, &js_api_name);
    let api_obj = global.get(scope, api_key.into());
    let api_obj = match api_obj {
        Some(v) if v.is_object() => unsafe { v8::Local::<v8::Object>::cast_unchecked(v) },
        _ => return,
    };

    let el_obj = v8::Object::new(scope);

    install_method(scope, el_obj, "advance", el_advance);
    install_method(scope, el_obj, "sleep", el_sleep);
    install_method(scope, el_obj, "tick", el_tick);
    install_method(scope, el_obj, "drain", el_drain);
    install_method(scope, el_obj, "drainMicrotasks", el_drain_microtasks);
    install_method(scope, el_obj, "drainTimers", el_drain_timers);
    install_method(scope, el_obj, "getTime", el_get_time);
    install_method(scope, el_obj, "reset", el_reset);
    install_method(
        scope,
        el_obj,
        "setAutoAdvanceStep",
        el_set_auto_advance_step,
    );

    let el_key = crate::v8_utils::v8_string(scope, "eventLoop");
    api_obj.set(scope, el_key.into(), el_obj.into());

    // Install netLog on __iv8__
    let netlog_obj = v8::Object::new(scope);
    let entries_arr = v8::Array::new(scope, 0);
    let entries_key = crate::v8_utils::v8_string(scope, "entries");
    netlog_obj.set(scope, entries_key.into(), entries_arr.into());
    let netlog_key = crate::v8_utils::v8_string(scope, "netLog");
    api_obj.set(scope, netlog_key.into(), netlog_obj.into());
}

fn install_method(
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

/// __iv8__.eventLoop.advance(totalMs, stepMs=16.67)
unsafe extern "C" fn el_advance(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        let total_ms = if args.length() >= 1 {
            args.get(0).number_value(scope).unwrap_or(0.0)
        } else {
            0.0
        };
        let step_ms = if args.length() >= 2 {
            args.get(1).number_value(scope).unwrap_or(16.67)
        } else {
            16.67
        };

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let total_us = (total_ms * 1000.0) as i64;
        let step_us = (step_ms * 1000.0) as i64;
        let end_us = state.event_loop.borrow().get_time_us() + total_us;

        loop {
            let current = state.event_loop.borrow().get_time_us();
            if current >= end_us {
                break;
            }
            let next = (current + step_us).min(end_us);
            state.event_loop.borrow_mut().current_us = next;
            run_due_tasks(scope, state);
        }
    }));
}

/// __iv8__.eventLoop.sleep(ms)
unsafe extern "C" fn el_sleep(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        let ms = if args.length() >= 1 {
            args.get(0).number_value(scope).unwrap_or(0.0)
        } else {
            0.0
        };

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        state.event_loop.borrow_mut().advance_time(ms);
        run_due_tasks(scope, state);
    }));
}

/// __iv8__.eventLoop.tick(ms=0)
unsafe extern "C" fn el_tick(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        let ms = if args.length() >= 1 {
            args.get(0).number_value(scope).unwrap_or(0.0)
        } else {
            0.0
        };

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        state.event_loop.borrow_mut().tick_time(ms);
        run_one_due_task(scope, state);
    }));
}

/// __iv8__.eventLoop.drain()
unsafe extern "C" fn el_drain(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        state.event_loop.borrow_mut().advance_to_last_deadline();
        run_due_tasks(scope, state);
    }));
}

/// __iv8__.eventLoop.drainMicrotasks()
unsafe extern "C" fn el_drain_microtasks(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let _ = scope; // microtasks run at checkpoint (handled by V8 automatically)
    }));
}

/// __iv8__.eventLoop.drainTimers()
unsafe extern "C" fn el_drain_timers(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        run_due_tasks(scope, state);
    }));
}

/// __iv8__.eventLoop.getTime()
unsafe extern "C" fn el_get_time(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let time_ms = state.event_loop.borrow().get_time_ms();
        rv.set(v8::Number::new(scope, time_ms).into());
    }));
}

/// __iv8__.eventLoop.reset()
unsafe extern "C" fn el_reset(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        state.event_loop.borrow_mut().reset();
    }));
}

/// __iv8__.eventLoop.setAutoAdvanceStep(us)
unsafe extern "C" fn el_set_auto_advance_step(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        let us = if args.length() >= 1 {
            args.get(0).number_value(scope).unwrap_or(4000.0) as i64
        } else {
            4000
        };

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        state.event_loop.borrow_mut().set_auto_advance_step_us(us);
    }));
}
