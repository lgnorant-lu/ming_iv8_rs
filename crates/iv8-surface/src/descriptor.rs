//! Descriptor configuration helpers for FunctionTemplate registration.

use v8::FunctionTemplate;
use v8::Local;
use v8::Object;

/// Register an interface constructor on the global object with DONT_ENUM.
///
/// Installed as non-enumerable to match browser behavior where
/// constructors like `window.AudioContext` are not enumerable.
pub fn register_global_constructor(
    scope: &v8::PinScope<'_, '_>,
    global: Local<Object>,
    name: &str,
    func: Local<FunctionTemplate>,
) {
    if let Some(ctor) = func.get_function(scope) {
        let key = v8::String::new(scope, name).unwrap();
        global.define_own_property(
            scope,
            key.into(),
            ctor.into(),
            v8::PropertyAttribute::DONT_ENUM,
        );
    }
}
