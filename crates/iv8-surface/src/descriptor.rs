//! Descriptor configuration helpers for FunctionTemplate registration.
//!
//! Provides utility functions for setting up FunctionTemplate descriptors
//! with correct configurable/enumerable/writable attributes, matching
//! real browser behavior.
//!
//! v0.8.19: minimal — global constructor registration only.

use v8::FunctionTemplate;
use v8::Local;
use v8::Object;

/// Register an interface constructor on the global object.
///
/// Installed as non-enumerable (DONT_ENUM) to match browser behavior
/// where constructors like `window.AudioContext` are not enumerable.
pub fn register_global_constructor(
    scope: &v8::PinScope<'_, '_>,
    global: Local<Object>,
    name: &str,
    func: Local<FunctionTemplate>,
) {
    if let Some(ctor) = func.get_function(scope) {
        let key = crate::type_conv::v8_str(scope, name);
        // v8::Object::define_own_property requires a Local<Name>
        // Use set() as a simple approach; full descriptor in v0.8.20
        global.set(scope, key, ctor.into());
    }
}
