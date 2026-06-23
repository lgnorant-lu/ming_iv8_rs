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
        tracing::info!(target: "iv8::console", "[console.log] {}", msg);
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
        tracing::info!(target: "iv8::console", "[console.info] {}", msg);
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
        tracing::warn!(target: "iv8::console", "[console.warn] {}", msg);
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
        tracing::error!(target: "iv8::console", "[console.error] {}", msg);
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
        tracing::debug!(target: "iv8::console", "[console.debug] {}", msg);
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
        tracing::trace!(target: "iv8::console", "[console.trace] {}", msg);
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
        tracing::info!(target: "iv8::console", "[console.dir] {}", msg);
    }));
}

unsafe extern "C" fn console_table_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let msg = format_args(scope, &args);
        tracing::info!(target: "iv8::console", "[console.table] {}", msg);
    }));
}

unsafe extern "C" fn console_group_cb(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let msg = format_args(scope, &args);
        tracing::info!(target: "iv8::console", "[console.group] {}", msg);
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
        tracing::debug!(target: "iv8::console", "[console.time] {}", label);
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
        tracing::debug!(target: "iv8::console", "[console.timeEnd] {}", label);
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
        tracing::debug!(target: "iv8::console", "[console.count] {}", label);
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
        tracing::error!(target: "iv8::console", "[console.assert] {}", msg);
        let isolate: &v8::Isolate = &*scope;
        store_message(RuntimeState::get(isolate), "assert", &msg);
    }));
}

unsafe extern "C" fn console_clear_cb(_info: *const v8::FunctionCallbackInfo) {}

/// A single console message.
#[derive(Debug, Clone)]
pub struct ConsoleMessage {
    pub level: String,
    pub text: String,
}
