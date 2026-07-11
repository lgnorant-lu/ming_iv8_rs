//! console.log / console.warn / console.error / console.debug / console.info
//!
//! Routes JS console output to Rust tracing.
//! Also stores the last N messages in RuntimeState.console_messages for
//! Python-side retrieval via Context.get_console_messages().

use crate::state::RuntimeState;

/// Install console object on the global.
pub fn install_console(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    let console_obj = v8::Object::new(scope);

    for (name, cb) in &[
        (
            "log",
            console_log_cb as unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
        ),
        ("info", console_info_cb),
        ("warn", console_warn_cb),
        ("error", console_error_cb),
        ("debug", console_debug_cb),
        ("trace", console_trace_cb),
        ("dir", console_dir_cb),
        ("table", console_table_cb),
        ("group", console_group_cb),
        ("groupCollapsed", console_group_cb),
        ("groupEnd", console_group_end_cb),
        ("time", console_time_cb),
        ("timeEnd", console_time_end_cb),
        ("count", console_count_cb),
        ("countReset", console_count_reset_cb),
        ("assert", console_assert_cb),
        ("clear", console_clear_cb),
        ("dirxml", console_noop_cb),
        ("profile", console_noop_cb),
        ("profileEnd", console_noop_cb),
        ("timeLog", console_noop_cb),
        ("timeStamp", console_noop_cb),
        ("context", console_noop_cb),
        ("createTask", console_noop_cb),
    ] {
        let fn_tmpl = v8::FunctionTemplate::builder_raw(*cb).build(scope);
        let func = crate::v8_utils::v8_fn(scope, &fn_tmpl);
        let key = crate::v8_utils::v8_string(scope, name);
        func.set_name(key);
        console_obj.set(scope, key.into(), func.into());
    }

    let key = crate::v8_utils::v8_string(scope, "console");
    global.define_own_property(
        scope,
        key.into(),
        console_obj.into(),
        v8::PropertyAttribute::DONT_ENUM,
    );

    let memory_obj = v8::Object::new(scope);
    let js_heap_size_limit = crate::v8_utils::v8_string(scope, "jsHeapSizeLimit");
    let total_js_heap_size = crate::v8_utils::v8_string(scope, "totalJSHeapSize");
    let used_js_heap_size = crate::v8_utils::v8_string(scope, "usedJSHeapSize");
    memory_obj.set(scope, js_heap_size_limit.into(), v8::Number::new(scope, 4294705152.0).into());
    memory_obj.set(scope, total_js_heap_size.into(), v8::Number::new(scope, 4294705152.0).into());
    memory_obj.set(scope, used_js_heap_size.into(), v8::Number::new(scope, 4294705152.0).into());
    let memory_key = crate::v8_utils::v8_string(scope, "memory");
    console_obj.set(scope, memory_key.into(), memory_obj.into());
}

/// Format console arguments to a string.
fn format_args(scope: &v8::PinScope<'_, '_>, args: &v8::FunctionCallbackArguments<'_>) -> String {
    let mut parts = Vec::new();
    for i in 0..args.length() {
        let val = args.get(i);
        parts.push(val.to_rust_string_lossy(scope));
    }
    parts.join(" ")
}

/// Store a console message in RuntimeState for Python retrieval.
fn store_message(state: &RuntimeState, level: &str, msg: &str) {
    state.console_messages.borrow_mut().push(ConsoleMessage {
        level: level.to_string(),
        text: msg.to_string(),
    });
}

unsafe extern "C" fn console_log_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let msg = format_args(scope, &args);
        crate::telemetry::console_message("log", "info", &msg);
        let isolate: &v8::Isolate = &*scope;
        store_message(RuntimeState::get(isolate), "log", &msg);
    }));
}

unsafe extern "C" fn console_info_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let msg = format_args(scope, &args);
        crate::telemetry::console_message("info", "info", &msg);
        let isolate: &v8::Isolate = &*scope;
        store_message(RuntimeState::get(isolate), "info", &msg);
    }));
}

unsafe extern "C" fn console_warn_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let msg = format_args(scope, &args);
        crate::telemetry::console_message("warn", "warn", &msg);
        let isolate: &v8::Isolate = &*scope;
        store_message(RuntimeState::get(isolate), "warn", &msg);
    }));
}

unsafe extern "C" fn console_error_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let msg = format_args(scope, &args);
        crate::telemetry::console_message("error", "error", &msg);
        let isolate: &v8::Isolate = &*scope;
        store_message(RuntimeState::get(isolate), "error", &msg);
    }));
}

unsafe extern "C" fn console_debug_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let msg = format_args(scope, &args);
        crate::telemetry::console_message("debug", "debug", &msg);
        let isolate: &v8::Isolate = &*scope;
        store_message(RuntimeState::get(isolate), "debug", &msg);
    }));
}

unsafe extern "C" fn console_trace_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let msg = format_args(scope, &args);
        crate::telemetry::console_message("trace", "trace", &msg);
        let isolate: &v8::Isolate = &*scope;
        store_message(RuntimeState::get(isolate), "trace", &msg);
    }));
}

unsafe extern "C" fn console_dir_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let msg = format_args(scope, &args);
        crate::telemetry::console_message("dir", "info", &msg);
    }));
}

unsafe extern "C" fn console_table_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let msg = format_args(scope, &args);
        crate::telemetry::console_message("table", "info", &msg);
    }));
}

unsafe extern "C" fn console_group_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let msg = format_args(scope, &args);
        crate::telemetry::console_message("group", "info", &msg);
    }));
}

unsafe extern "C" fn console_group_end_cb(_info: *const v8::FunctionCallbackInfo) {}

unsafe extern "C" fn console_time_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let label = if args.length() >= 1 {
            args.get(0).to_rust_string_lossy(scope)
        } else {
            "default".to_string()
        };
        crate::telemetry::console_message("time", "debug", &label);
    }));
}

unsafe extern "C" fn console_time_end_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let label = if args.length() >= 1 {
            args.get(0).to_rust_string_lossy(scope)
        } else {
            "default".to_string()
        };
        crate::telemetry::console_message("timeEnd", "debug", &label);
    }));
}

unsafe extern "C" fn console_count_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let label = if args.length() >= 1 {
            args.get(0).to_rust_string_lossy(scope)
        } else {
            "default".to_string()
        };
        crate::telemetry::console_message("count", "debug", &label);
    }));
}

unsafe extern "C" fn console_count_reset_cb(_info: *const v8::FunctionCallbackInfo) {}

unsafe extern "C" fn console_assert_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        if args.length() >= 1 && args.get(0).is_true() {
            return;
        }
        let msg = if args.length() >= 2 {
            format!(
                "Assertion failed: {}",
                args.get(1).to_rust_string_lossy(scope)
            )
        } else {
            "Assertion failed".to_string()
        };
        crate::telemetry::console_message("assert", "error", &msg);
        let isolate: &v8::Isolate = &*scope;
        store_message(RuntimeState::get(isolate), "assert", &msg);
    }));
}

unsafe extern "C" fn console_clear_cb(_info: *const v8::FunctionCallbackInfo) {}

unsafe extern "C" fn console_noop_cb(_info: *const v8::FunctionCallbackInfo) {}

/// A single console message.
#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    pub level: String,
    pub text: String,
}
