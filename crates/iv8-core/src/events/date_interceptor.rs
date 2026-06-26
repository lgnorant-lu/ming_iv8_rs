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

    // Install performance.memory — quantized, per-page-stable snapshot.
    // Real Chrome returns per-call-varying unbucketed heap bytes from V8
    // Isolate statistics; exposing those is a bot-tell. We expose fixed
    // values quantized to 100KB (102400-byte) buckets, stable across all
    // calls within a page session.
    install_performance_memory(scope, perf_obj);

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
/// In logical mode: eventLoop time directly, plus a small monotonic sub-ms
/// jitter so consecutive samples in a tight loop do not return identical
/// diffs (a known bot-tell when 10 samples yield the exact same delta).
/// In system mode: real performance.now equivalent with the same jitter.
unsafe extern "C" fn performance_now_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        let base_ms = match state.time_mode {
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

        // Monotonic jitter: ensure each call is strictly greater than the
        // last returned value by a small sub-millisecond increment, so that
        // repeated sampling does not produce identical consecutive diffs.
        // The increment varies per call (deterministic, no RNG) so the diffs
        // themselves are not a constant — a constant delta across N samples
        // is itself a bot-tell. Stays sub-millisecond to avoid disturbing
        // timer scheduling.
        let prev = state.perf_now_last.get();
        let mut now_ms = base_ms;
        if now_ms <= prev {
            // Varying sub-ms step in the ~0.5–1.5 microsecond range, derived
            // from a cheap xorshift over the monotonic last value. This keeps
            // diffs non-identical while preserving strict monotonicity.
            let mut s = prev.to_bits().wrapping_add(0x9E37_79B9_7F4A_7C15);
            s ^= s >> 17;
            s = s.wrapping_mul(0xBF58_476D_1CE4_E5B9);
            s ^= s >> 31;
            // Map to [0.0005, 0.0015) ms (0.5–1.5 us).
            let frac = (s & 0xFFFF) as f64 / 65535.0;
            let step = 0.0005 + frac * 0.0010;
            now_ms = prev + step;
        }
        state.perf_now_last.set(now_ms);

        rv.set(v8::Number::new(scope, now_ms).into());
    }));
}

/// Install `performance.memory` as a non-enumerable accessor returning a
/// per-page-stable, 100KB-quantized heap snapshot.
///
/// The values are computed once (lazily on first access) and cached in
/// `RuntimeState.perf_memory`, so every subsequent read within the same
/// page session returns identical numbers — matching real-browser
/// behavior where `performance.memory` is approximately stable for a
/// given page, while avoiding the per-call-varying unbucketed bytes that
/// fingerprinting scripts flag as a bot-tell.
fn install_performance_memory(scope: &v8::PinScope<'_, '_>, perf_obj: v8::Local<v8::Object>) {
    let isolate: &v8::Isolate = &**scope;
    let state = RuntimeState::get(isolate);

    // Lazily initialize the per-page-stable quantized snapshot.
    let mem = {
        let mut slot = state.perf_memory.borrow_mut();
        if slot.is_none() {
            *slot = Some(crate::state::PerformanceMemory::default_quantized());
        }
        // We only need the copy; drop the borrow before touching V8.
        slot.as_ref().copied().expect("initialized above")
    };

    let memory_obj = v8::Object::new(scope);

    let limit = v8::Number::new(scope, mem.js_heap_size_limit as f64);
    let total = v8::Number::new(scope, mem.total_js_heap_size as f64);
    let used = v8::Number::new(scope, mem.used_js_heap_size as f64);

    let k_limit = crate::v8_utils::v8_string(scope, "jsHeapSizeLimit");
    let k_total = crate::v8_utils::v8_string(scope, "totalJSHeapSize");
    let k_used = crate::v8_utils::v8_string(scope, "usedJSHeapSize");

    // Define as read-only, non-enumerable properties to mirror Chrome.
    let ro = v8::PropertyAttribute::READ_ONLY | v8::PropertyAttribute::DONT_ENUM;
    memory_obj.define_own_property(scope, k_limit.into(), limit.into(), ro);
    let ro = v8::PropertyAttribute::READ_ONLY | v8::PropertyAttribute::DONT_ENUM;
    memory_obj.define_own_property(scope, k_total.into(), total.into(), ro);
    let ro = v8::PropertyAttribute::READ_ONLY | v8::PropertyAttribute::DONT_ENUM;
    memory_obj.define_own_property(scope, k_used.into(), used.into(), ro);

    let k_memory = crate::v8_utils::v8_string(scope, "memory");
    perf_obj.set(scope, k_memory.into(), memory_obj.into());
}
