//! Type conversion utilities — IDL types to V8 values and back.
//!
//! Provides helper functions for converting between JavaScript values
//! and Rust types, used by generated getter/setter/method stubs.
//!
//! v0.8.19: minimal — default_value_for_type returns type-appropriate
//! zero/empty values. Full conversion in v0.8.21+.

use v8::Local;
use v8::Value;

/// Return a default V8 value for the given IDL type name.
///
/// Used by generated getter stubs that don't yet have deep behavior.
pub fn default_value_for_type<'s>(
    scope: &v8::PinScope<'s, '_>,
    type_name: &str,
) -> Local<'s, Value> {
    match type_name {
        "boolean" => v8::Boolean::new(scope, false).into(),
        "long" | "short" | "byte" | "octet" | "unsigned long" |
        "unsigned short" | "long long" | "unsigned long long" |
        "float" | "double" | "unrestricted float" | "unrestricted double" =>
            v8::Number::new(scope, 0.0).into(),
        "DOMString" | "USVString" | "ByteString" => {
            v8::String::new(scope, "").map(|s| s.into()).unwrap_or_else(|| v8::undefined(scope).into())
        }
        "undefined" | "void" => v8::undefined(scope).into(),
        _ => v8::null(scope).into(),
    }
}

/// Create a V8 string from a Rust &str.
pub fn v8_str<'s>(scope: &v8::PinScope<'s, '_>, s: &str) -> Local<'s, Value> {
    v8::String::new(scope, s)
        .map(|v| v.into())
        .unwrap_or_else(|| v8::undefined(scope).into())
}
