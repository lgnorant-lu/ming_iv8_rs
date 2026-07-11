//! V8 platform initialization (process-global, once only).

use std::sync::Once;

static V8_INIT: Once = Once::new();

/// Minimum stack size (bytes) for V8 template creation.
///
/// V8 FunctionTemplate creation (1287 interfaces, 9223 members after mixin
/// merge) recurses deeply in C++ stack. The value is virtual memory with
/// lazy physical commit — actual RSS is far smaller.
const MIN_STACK_SIZE: &str = "1342177128";

/// Initialize V8 platform and engine. Safe to call multiple times —
/// only the first call has effect.
///
/// ## Stack size (K-010)
///
/// Three layers ensure sufficient stack for V8 template creation:
///
/// 1. **`.cargo/config.toml` `[env]`** — sets `RUST_MIN_STACK` for all
///    cargo commands (build, test, run). The Rust test harness reads this
///    when spawning test threads.
/// 2. **This function** — sets `RUST_MIN_STACK` if unset, as a safety net
///    for embedders who don't use cargo. Affects threads spawned after
///    this call (not the calling thread).
/// 3. **Python `__init__.py`** — calls `threading.stack_size(128MB)` at
///    module import time, before any JSContext creation.
///
/// ## V8 internal stack limit
///
/// V8's `--stack-size` flag controls the central-stack window (separate
/// from the OS thread stack). Set to 8MB (8192 KB) as a conservative
/// balance — 128MB caused a non-unwinding panic in WPT test runner
/// (PyEvent_IsSet thread crash). The OS thread stack (via RUST_MIN_STACK)
/// provides the actual recursion depth capacity.
///
/// ## Platform
///
/// Uses `new_default_platform(0, false)` (multi-threaded). V8's background
/// GC/compilation threads are enabled. This is required for Worker isolate
/// creation — without it, V8's shared ReadOnlyHeap GC crashes with
/// `IsOnCentralStack` when Worker isolate creates FunctionTemplates that
/// trigger GC.
///
/// See: docs/roadmap/v0.8/analysis/worker-execution-environment-design.md §9.8
pub fn ensure_v8_initialized() {
    V8_INIT.call_once(|| {
        if std::env::var("RUST_MIN_STACK").is_err() {
            std::env::set_var("RUST_MIN_STACK", MIN_STACK_SIZE);
        }

        v8::V8::set_flags_from_string("--stack-size=8192");

        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_v8_initialized_is_idempotent() {
        ensure_v8_initialized();
        ensure_v8_initialized();
        ensure_v8_initialized();
    }

    #[test]
    fn test_rust_min_stack_is_set_after_init() {
        ensure_v8_initialized();
        let val = std::env::var("RUST_MIN_STACK").expect("RUST_MIN_STACK must be set");
        assert!(
            val.parse::<usize>().unwrap() >= 134_217_728,
            "RUST_MIN_STACK must be >= 128MB, got {}",
            val
        );
    }
}
