//! V8 platform initialization (process-global, once only).

use std::sync::Once;

static V8_INIT: Once = Once::new();

/// Initialize V8 platform and engine. Safe to call multiple times —
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
pub fn ensure_v8_initialized() {
    V8_INIT.call_once(|| {
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
