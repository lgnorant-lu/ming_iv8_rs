//! V8 platform initialization (process-global, once only).

use std::sync::Once;

static V8_INIT: Once = Once::new();

/// Initialize V8 platform and engine. Safe to call multiple times —
/// only the first call has effect.
///
/// Uses `new_unprotected_default_platform` instead of `new_default_platform`
/// because Worker isolates are created on Worker threads (not the thread that
/// called `V8::initialize`). The default platform enforces thread-isolated
/// allocations which require all isolates to be created on the initializing
/// thread. The unprotected platform disables this restriction, allowing
/// multi-thread isolate creation.
/// See: docs/roadmap/v0.8/analysis/worker-execution-environment-design.md §9.8
pub fn ensure_v8_initialized() {
    V8_INIT.call_once(|| {
        let platform = v8::new_unprotected_default_platform(0, false).make_shared();
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
