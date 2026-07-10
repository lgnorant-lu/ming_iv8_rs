//! V8 platform initialization (process-global, once only).

use std::sync::Once;

static V8_INIT: Once = Once::new();

/// Initialize V8 platform and engine. Safe to call multiple times ��
/// only the first call has effect.
///
/// Uses `new_single_threaded_default_platform` with `--single-threaded` V8 flag.
/// This disables V8's background GC/compilation thread pool, forcing all GC
/// to run on the isolate's foreground thread. This is required for Worker
/// isolate creation on Worker threads — without it, V8's shared ReadOnlyHeap
/// GC crashes with `IsOnCentralStack` when Worker isolate creates
/// FunctionTemplates that trigger GC.
///
/// Deno uses the same approach for `--single-threaded` mode (PR #29066).
/// See: docs/roadmap/v0.8/analysis/worker-execution-environment-design.md §9.8
///
/// ## V8 stack-size (v0.8.88+)
///
/// V8's default central-stack range is ~1MB (--stack-size=984). When IV8
/// runs in a 128MB-stack Python thread (threading.stack_size(128*1024*1024)),
/// the install_all function creating 1287 FunctionTemplates can use >1MB
/// of C++ stack. If V8's GC triggers during this, the conservative stack
/// scanner asserts `CHECK(isolate_->IsOnCentralStack())` because the live
/// SP is outside V8's 1MB central-stack window.
///
/// Fix: set --stack-size to 128MB so V8's central-stack range matches the
/// actual thread stack. This is the safe approach recommended by ClearScript
/// (#726) — unlike SetStackLimit() which poisons the isolate.
pub fn ensure_v8_initialized() {
    V8_INIT.call_once(|| {
        // Set V8 stack size to match Python's 128MB thread stack.
        // Must be called BEFORE V8::initialize().
        // Set V8 stack size to match Python's 128MB thread stack.
        // Must be called BEFORE V8::initialize().
        // Note: Previous attempt with --stack-size=131072 caused non-unwinding panic
        // in WPT test runner (PyEvent_IsSet thread crash). Using --stack-size=8192
        // (8MB) as a conservative balance. The True fix is to store worker_mode in
        // EmbeddedV8Kernel and skip Window-only init in worker_mode (D-135).
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
}
