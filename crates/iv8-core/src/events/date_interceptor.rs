//! DateInterceptor: override Date.now() and performance.now() to return logical time.
//!
//! In time_mode='logical' (default), these return the EventLoop's virtual clock.
//! In time_mode='system', they return real wall clock time.
//!
//! Implementation:
//! - Date.now → native callback reading EventLoop.get_time_ms() + epoch offset
//! - performance.now → native callback reading EventLoop.get_time_ms() (relative)
//! - new Date() → JS shim wrapping original constructor to use __iv8_now__()

use crate::state::{RuntimeState, TimeMode};

/// Install date/time interceptors.
/// Must be called after EventLoop is set up.
pub fn install_date_interceptor(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    // Install __iv8_now__ as a hidden native function (used by the Date shim)
    let now_tmpl = v8::FunctionTemplate::builder_raw(iv8_now_callback).build(scope);
    let now_fn = crate::v8_utils::v8_fn(scope, &now_tmpl);
    let now_key = crate::v8_utils::v8_string(scope, "__iv8_now__");
    global.define_own_property(
        scope,
        now_key.into(),
        now_fn.into(),
        v8::PropertyAttribute::DONT_ENUM,
    );

    // Override Date.now with native function
    let date_now_tmpl = v8::FunctionTemplate::builder_raw(date_now_callback).build(scope);
    let date_now_fn = crate::v8_utils::v8_fn(scope, &date_now_tmpl);
    let date_key = crate::v8_utils::v8_string(scope, "Date");
    if let Some(date_val) = global.get(scope, date_key.into()) {
        if date_val.is_function() {
            let date_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(date_val) };
            let now_key = crate::v8_utils::v8_string(scope, "now");
            date_obj.set(scope, now_key.into(), date_now_fn.into());
        }
    }

    // Install performance.now
    let perf_key = crate::v8_utils::v8_string(scope, "performance");
    let perf_obj = if let Some(perf_val) = global.get(scope, perf_key.into()) {
        if perf_val.is_object() {
            unsafe { v8::Local::<v8::Object>::cast_unchecked(perf_val) }
        } else {
            let obj = v8::Object::new(scope);
            global.set(scope, perf_key.into(), obj.into());
            obj
        }
    } else {
        let obj = v8::Object::new(scope);
        global.set(scope, perf_key.into(), obj.into());
        obj
    };

    let perf_now_tmpl = v8::FunctionTemplate::builder_raw(performance_now_callback).build(scope);
    let perf_now_fn = crate::v8_utils::v8_fn(scope, &perf_now_tmpl);
    let now_method_key = crate::v8_utils::v8_string(scope, "now");
    perf_obj.set(scope, now_method_key.into(), perf_now_fn.into());

    // Install performance.timeOrigin — context creation logical epoch or real wall-clock.
    // Logical mode: 1704067200000.0 (IV8 epoch 2024-01-01T00:00:00Z).
    // System mode: system time at context creation captured by kernel.
    let origin_key = crate::v8_utils::v8_string(scope, "timeOrigin");
    let origin_val = {
        let isolate: &v8::Isolate = &**scope;
        let state = RuntimeState::get(isolate);
        match state.time_mode {
            TimeMode::Logical => 1704067200000.0,
            TimeMode::System => {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs_f64()
                    * 1000.0
            }
        }
    };
    perf_obj.set(scope, origin_key.into(), v8::Number::new(scope, origin_val).into());

    // Install Date constructor shim (wraps original to use logical time for no-arg calls)
    // This is done via JS to preserve the original Date behavior for Date(timestamp) calls.
}

/// Install the Date constructor shim via JS eval.
/// Must be called after __iv8_now__ is installed.
pub const DATE_SHIM_JS: &str = r#"
(function() {
    const _OriginalDate = Date;
    const _iv8_now = globalThis.__iv8_now__;

    function PatchedDate(...args) {
        if (args.length === 0) {
            // new Date() → use logical time
            return new _OriginalDate(_iv8_now());
        }
        if (new.target) {
            return new _OriginalDate(...args);
        }
        // Date() without new → string representation
        return new _OriginalDate(_iv8_now()).toString();
    }

    // Copy static methods
    PatchedDate.now = _iv8_now;
    PatchedDate.parse = _OriginalDate.parse;
    PatchedDate.UTC = _OriginalDate.UTC;

    // Preserve prototype chain
    PatchedDate.prototype = _OriginalDate.prototype;
    PatchedDate.prototype.constructor = PatchedDate;

    // Replace global Date
    Object.defineProperty(globalThis, 'Date', {
        value: PatchedDate,
        writable: true,
        configurable: true,
        enumerable: false,
    });
})();
"#;

/// __iv8_now__() → current logical time as Unix timestamp (ms since epoch).
/// In logical mode: epoch (1704067200000 = 2024-01-01T00:00:00Z) + eventLoop time.
/// In system mode: real Date.now().
unsafe extern "C" fn iv8_now_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let now_ms = match state.time_mode {
            TimeMode::Logical => {
                // Epoch: 2024-01-01T00:00:00Z = 1704067200000ms
                const EPOCH_MS: f64 = 1704067200000.0;
                let el_time = state.event_loop.borrow().get_time_ms();
                EPOCH_MS + el_time
            }
            TimeMode::System => {
                let duration = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default();
                duration.as_secs_f64() * 1000.0
            }
        };

        rv.set(v8::Number::new(scope, now_ms).into());
    }));
}

/// Date.now() callback — same as __iv8_now__ but installed on Date object.
unsafe extern "C" fn date_now_callback(info: *const v8::FunctionCallbackInfo) {
    // Delegate to the same logic
    iv8_now_callback(info);
}

/// performance.now() → time since context creation (ms, high resolution).
/// In logical mode: eventLoop time directly.
/// In system mode: real performance.now equivalent.
unsafe extern "C" fn performance_now_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let now_ms = match state.time_mode {
            TimeMode::Logical => state.event_loop.borrow().get_time_ms(),
            TimeMode::System => {
                // Approximate: use process uptime
                let duration = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default();
                // Return ms with microsecond precision
                (duration.as_micros() % 1_000_000_000) as f64 / 1000.0
            }
        };

        rv.set(v8::Number::new(scope, now_ms).into());
    }));
}
