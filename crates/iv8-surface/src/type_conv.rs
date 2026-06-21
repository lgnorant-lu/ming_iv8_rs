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
        "long"
        | "short"
        | "byte"
        | "octet"
        | "unsigned long"
        | "unsigned short"
        | "long long"
        | "unsigned long long"
        | "float"
        | "double"
        | "unrestricted float"
        | "unrestricted double" => v8::Number::new(scope, 0.0).into(),
        "DOMString" | "USVString" | "ByteString" => v8::String::new(scope, "")
            .map(|s| s.into())
            .unwrap_or_else(|| v8::undefined(scope).into()),
        "undefined" | "void" => v8::undefined(scope).into(),
        // Typed array / buffer-source types: return empty typed arrays instead
        // of null. The generated IDL surface uses these for getters/methods
        // that return typed arrays; a null return would break any call chain
        // that expects a real typed-array instance (e.g. TextEncoder.encode
        // feeding into crypto.subtle.digest).
        "ArrayBuffer" => {
            let store = v8::ArrayBuffer::new_backing_store_from_vec(vec![]);
            v8::ArrayBuffer::with_backing_store(scope, &store.into()).into()
        }
        "Int8Array" | "Uint8Array" | "Uint8ClampedArray" | "Int16Array" | "Uint16Array"
        | "Int32Array" | "Uint32Array" | "Float32Array" | "Float64Array" | "BigInt64Array"
        | "BigUint64Array" | "DataView" => {
            // Construct an empty typed array via the global constructor.
            // new <Type>(0) gives a zero-length instance of the right class.
            let global = scope.get_current_context().global(scope);
            if let Some(name) = v8::String::new(scope, type_name) {
                if let Some(ctor) = global.get(scope, name.into()) {
                    if ctor.is_function() {
                        let ctor_fn: v8::Local<v8::Function> =
                            unsafe { v8::Local::cast_unchecked(ctor) };
                        if let Some(result) =
                            ctor_fn.new_instance(scope, &[v8::Integer::new(scope, 0).into()])
                        {
                            return result.into();
                        }
                    }
                }
            }
            v8::null(scope).into()
        }
        _ => v8::null(scope).into(),
    }
}

/// Create a V8 string from a Rust &str.
pub fn v8_str<'s>(scope: &v8::PinScope<'s, '_>, s: &str) -> Local<'s, Value> {
    v8::String::new(scope, s)
        .map(|v| v.into())
        .unwrap_or_else(|| v8::undefined(scope).into())
}

/// Create a Float32Array from a slice of f64 values.
pub fn make_float32_array<'s>(scope: &v8::PinScope<'s, '_>, values: &[f64]) -> Local<'s, Value> {
    let arr = v8::Array::new(scope, values.len() as i32);
    for (i, &v) in values.iter().enumerate() {
        arr.set_index(scope, i as u32, v8::Number::new(scope, v).into());
    }
    arr.into()
}

/// Create an Int32Array from a slice of i32 values.
pub fn make_int32_array<'s>(scope: &v8::PinScope<'s, '_>, values: &[i32]) -> Local<'s, Value> {
    let arr = v8::Array::new(scope, values.len() as i32);
    for (i, &v) in values.iter().enumerate() {
        arr.set_index(scope, i as u32, v8::Integer::new(scope, v).into());
    }
    arr.into()
}
