//! V8 platform initialization (process-global, once only).

use std::sync::Once;

static V8_INIT: Once = Once::new();

/// Initialize V8 platform and engine. Safe to call multiple times —
/// only the first call has effect.
pub fn ensure_v8_initialized() {
    V8_INIT.call_once(|| {
        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });
}
