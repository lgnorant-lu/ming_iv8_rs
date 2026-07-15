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
    let name_str = crate::v8_utils::v8_string(scope, name);
    tmpl.set_class_name(name_str);
    tmpl.remove_prototype();
    let func = crate::v8_utils::v8_fn(scope, &tmpl);
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
        let mut delay_ms = if args.length() >= 2 {
            args.get(1).number_value(scope).unwrap_or(0.0).max(0.0)
        } else {
            0.0
        };

        let global_fn = v8::Global::new(scope, func);

        let extra_args: Vec<v8::Global<v8::Value>> = (2..args.length())
            .map(|i| v8::Global::new(scope, args.get(i)))
            .collect();

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        // Q082 residual: when document.hidden, apply a simple background min delay
        // (Chrome intensive model is more complex; we use env-tunable 1000ms default).
        delay_ms = apply_hidden_timer_floor(scope, state, delay_ms);
        let id = state
            .event_loop
            .borrow_mut()
            .add_timer(global_fn, delay_ms, TaskKind::Timeout, extra_args);

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
        let mut delay_ms = if args.length() >= 2 {
            args.get(1).number_value(scope).unwrap_or(0.0).max(0.0)
        } else {
            0.0
        };

        let global_fn = v8::Global::new(scope, func);

        let extra_args: Vec<v8::Global<v8::Value>> = (2..args.length())
            .map(|i| v8::Global::new(scope, args.get(i)))
            .collect();

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        delay_ms = apply_hidden_timer_floor(scope, state, delay_ms);
        // min interval from environment, default 1ms
        let min_interval_us: i64 = state
            .environment
            .get_f64("timers.min_interval_ms")
            .map(|ms| (ms * 1000.0) as i64)
            .unwrap_or(1000);
        let period_us = (delay_ms * 1000.0) as i64;
        let id = state.event_loop.borrow_mut().add_timer(
            global_fn,
            delay_ms,
            TaskKind::Interval {
                period_us: period_us.max(min_interval_us),
            },
            extra_args,
        );

        rv.set(v8::Integer::new(scope, id as i32).into());
    }));
}

/// If `document.hidden` is true, enforce a minimum delay for timers.
/// Optional intensive floor: `timers.hidden_intensive_min_ms` (default 60000)
/// when logical time since hide exceeds `timers.hidden_intensive_after_ms`
/// (default 300000 = 5 minutes, Chrome-inspired).
fn apply_hidden_timer_floor(
    scope: &v8::PinScope<'_, '_>,
    state: &RuntimeState,
    delay_ms: f64,
) -> f64 {
    let floor = state
        .environment
        .get_f64("timers.hidden_min_interval_ms")
        .unwrap_or(1000.0);
    if floor <= 0.0 {
        return delay_ms;
    }
    let ctx = scope.get_current_context();
    let global = ctx.global(scope);
    let doc_key = crate::v8_utils::v8_string(scope, "document");
    let Some(doc_val) = global.get(scope, doc_key.into()) else {
        return delay_ms;
    };
    if !doc_val.is_object() {
        return delay_ms;
    }
    let doc: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(doc_val) };
    let hidden_key = crate::v8_utils::v8_string(scope, "hidden");
    let Some(hidden_val) = doc.get(scope, hidden_key.into()) else {
        return delay_ms;
    };
    if !(hidden_val.is_true() || hidden_val.boolean_value(scope)) {
        return delay_ms;
    }
    // Intensive throttle: if page has been hidden past threshold (tracked via
    // globalThis.__iv8HiddenSinceMs set by document.hidden setter), use larger floor.
    let mut effective_floor = floor;
    let since_key = crate::v8_utils::v8_string(scope, "__iv8HiddenSinceMs");
    if let Some(since_val) = global.get(scope, since_key.into()) {
        if let Some(since) = since_val.number_value(scope) {
            let now = state.event_loop.borrow().get_time_ms();
            let after = state
                .environment
                .get_f64("timers.hidden_intensive_after_ms")
                .unwrap_or(300_000.0);
            if after > 0.0 && now - since >= after {
                let intensive = state
                    .environment
                    .get_f64("timers.hidden_intensive_min_ms")
                    .unwrap_or(60_000.0);
                if intensive > effective_floor {
                    effective_floor = intensive;
                }
            }
        }
    }
    delay_ms.max(effective_floor)
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
        let raf_ms = state
            .environment
            .get_f64("timers.raf_interval_ms")
            .unwrap_or(16.67);
        let deadline_ms = state.event_loop.borrow().get_time_ms() + raf_ms;
        let id = state
            .event_loop
            .borrow_mut()
            .add_timer(global_fn, raf_ms, TaskKind::Raf { deadline_ms }, vec![]);

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
