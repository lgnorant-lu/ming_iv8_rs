//! V8 Value → Rust value conversion (strict_compat mode).
//!
//! Implements the D-3 type conversion matrix:
//! - Number → i64 or f64
//! - String → String
//! - Boolean → bool
//! - null / undefined → None (represented as RustValue::Null)
//! - Array → Vec<RustValue>
//! - Object → HashMap<String, RustValue>
//! - TypedArray / ArrayBuffer → Vec<u8> (raw bytes)
//! - Other (Date/Map/Set/Promise/Function/RegExp) → RustValue::JsObject(toString)

use std::collections::HashMap;

/// Intermediate Rust representation of a JS value.
/// This is what gets converted to Python objects in iv8-py.
#[derive(Debug, Clone, PartialEq)]
pub enum RustValue {
    /// null or undefined
    Null,
    /// boolean
    Bool(bool),
    /// integer (fits in i64)
    Int(i64),
    /// floating point (NaN, Infinity, or non-integer)
    Float(f64),
    /// string
    String(String),
    /// array (recursive)
    Array(Vec<RustValue>),
    /// plain object (recursive)
    Object(HashMap<String, RustValue>),
    /// raw bytes (TypedArray / ArrayBuffer)
    Bytes(Vec<u8>),
    /// opaque JS object (toString representation) — Date/Map/Set/Promise/Function/etc.
    /// In strict_compat mode, complex objects degrade to their toString.
    JsObject(String),
}

/// Convert a V8 Local<Value> to RustValue.
/// `depth` tracks recursion to handle circular references (max 10 levels).
///
/// In v8 147, all V8 API methods accept `&PinScope<'s, '_>` which is what
/// the `scope!` / `scope_with_context!` macros produce. We use a macro
/// to avoid complex generic bounds.
#[macro_export]
macro_rules! v8_to_rust {
    ($scope:expr, $value:expr) => {
        $crate::convert::v8_to_rust_impl($scope, $value, 0)
    };
    ($scope:expr, $value:expr, $depth:expr) => {
        $crate::convert::v8_to_rust_impl($scope, $value, $depth)
    };
}

/// Implementation — called via `v8_to_rust!` macro.
pub fn v8_to_rust_impl(
    scope: &v8::PinScope<'_, '_>,
    value: v8::Local<v8::Value>,
    depth: u32,
) -> RustValue {
    v8_to_rust_with_seen(scope, value, depth, &mut std::collections::HashSet::new())
}

/// Internal implementation with circular reference tracking.
fn v8_to_rust_with_seen(
    scope: &v8::PinScope<'_, '_>,
    value: v8::Local<v8::Value>,
    depth: u32,
    seen: &mut std::collections::HashSet<i32>,
) -> RustValue {

    // Circular reference protection
    if depth > 10 {
        return RustValue::JsObject("[object Object]".to_string());
    }

    if value.is_null_or_undefined() {
        return RustValue::Null;
    }

    if value.is_boolean() {
        return RustValue::Bool(value.is_true());
    }

    if value.is_int32() {
        return RustValue::Int(value.int32_value(scope).unwrap_or(0) as i64);
    }

    if value.is_uint32() {
        return RustValue::Int(value.uint32_value(scope).unwrap_or(0) as i64);
    }

    if value.is_number() {
        let n = value.number_value(scope).unwrap_or(f64::NAN);
        // Check if it's a "safe integer" that fits in i64
        if n.fract() == 0.0 && n.is_finite() && n.abs() < (i64::MAX as f64) {
            return RustValue::Int(n as i64);
        }
        return RustValue::Float(n);
    }

    if value.is_string() {
        return RustValue::String(value.to_rust_string_lossy(scope));
    }

    // Symbol → string representation
    if value.is_symbol() {
        let desc = value.to_detail_string(scope)
            .map(|s| s.to_rust_string_lossy(scope))
            .unwrap_or_else(|| "Symbol()".to_string());
        return RustValue::String(desc);
    }

    // TypedArray → bytes (raw memcpy, not typed interpretation — matches iv8 behavior)
    if value.is_typed_array() {
        let ta: v8::Local<v8::TypedArray> = unsafe { v8::Local::cast_unchecked(value) };
        let len = ta.byte_length();
        let mut buf = vec![0u8; len];
        if len > 0 {
            let copied = ta.copy_contents(&mut buf);
            buf.truncate(copied);
        }
        return RustValue::Bytes(buf);
    }

    // ArrayBuffer → bytes
    if value.is_array_buffer() {
        let ab: v8::Local<v8::ArrayBuffer> = unsafe { v8::Local::cast_unchecked(value) };
        let len = ab.byte_length();
        let mut buf = vec![0u8; len];
        if len > 0 {
            let store = ab.get_backing_store();
            if let Some(data_ptr) = store.data() {
                let slice = unsafe {
                    std::slice::from_raw_parts(data_ptr.as_ptr() as *const u8, len)
                };
                buf.copy_from_slice(slice);
            }
        }
        return RustValue::Bytes(buf);
    }

    // BigInt → Null in strict_compat mode (iv8 bug: "cannot convert value, type not handled: V8 bigint")
    if value.is_big_int() {
        tracing::error!("[ERROR] cannot convert value, type not handled: V8 bigint");
        return RustValue::Null;
    }

    // Array → recursive
    if value.is_array() {
        let arr: v8::Local<v8::Array> = unsafe { v8::Local::cast_unchecked(value) };
        let len = arr.length();
        let mut result = Vec::with_capacity(len as usize);
        for i in 0..len {
            if let Some(elem) = arr.get_index(scope, i) {
                result.push(v8_to_rust_with_seen(scope, elem, depth + 1, seen));
            } else {
                result.push(RustValue::Null);
            }
        }
        return RustValue::Array(result);
    }

    // Plain Object → recursive (only if not a special type)
    if value.is_object() {
        // Check for special object types that degrade to toString in strict_compat
        if value.is_date()
            || value.is_map()
            || value.is_set()
            || value.is_promise()
            || value.is_function()
            || value.is_reg_exp()
            || value.is_native_error()
        {
            // iv8 0.1.2 strict_compat behavior:
            // - Function → "function name() { ... }" (truncated source)
            // - Error → "[object Error]" (Object.prototype.toString)
            // - Promise → "[object Promise]"
            // - Map/Set → "[object Map]" / "[object Set]"
            // - Date → "[object Date]"
            // - RegExp → "/pattern/flags"
            let s = if value.is_function() {
                // iv8 returns truncated function source: "function name() { ... }"
                // We use toString() which gives the full source, then truncate body
                let full = value.to_detail_string(scope)
                    .map(|s| s.to_rust_string_lossy(scope))
                    .unwrap_or_else(|| "function() {}".to_string());
                // Truncate body to match iv8 format: "function name() { ... }"
                truncate_function_body(&full)
            } else if value.is_native_error() || value.is_map() || value.is_set() || value.is_promise() || value.is_date() {
                // Use Object.prototype.toString for these types
                // This gives "[object Error]", "[object Map]", "[object Set]", "[object Promise]", "[object Date]"
                call_object_to_string(scope, value)
            } else {
                value.to_detail_string(scope)
                    .map(|s| s.to_rust_string_lossy(scope))
                    .unwrap_or_else(|| "[object Object]".to_string())
            };
            return RustValue::JsObject(s);
        }

        // Plain object → dict
        let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(value) };

        // Circular reference detection via identity hash
        let identity = obj.get_identity_hash().get();
        if seen.contains(&identity) {
            return RustValue::JsObject("[object Object]".to_string());
        }
        seen.insert(identity);

        let keys = obj.get_own_property_names(scope, Default::default());
        if let Some(keys) = keys {
            let len = keys.length();
            let mut map = HashMap::with_capacity(len as usize);
            for i in 0..len {
                if let Some(key) = keys.get_index(scope, i) {
                    let key_str = key.to_rust_string_lossy(scope);
                    let val = obj.get(scope, key).unwrap_or_else(|| v8::undefined(scope).into());
                    map.insert(key_str, v8_to_rust_with_seen(scope, val, depth + 1, seen));
                }
            }
            seen.remove(&identity);
            return RustValue::Object(map);
        }
    }

    // Fallback
    RustValue::JsObject(
        value.to_detail_string(scope)
            .map(|s| s.to_rust_string_lossy(scope))
            .unwrap_or_else(|| "[unknown]".to_string()),
    )
}

/// Truncate function body to match iv8 0.1.2 format: "function name() { ... }"
/// Input: "function foo() { return 42; }" or "() => 42"
/// Output: "function foo() { ... }" or "function() { ... }"
fn truncate_function_body(source: &str) -> String {
    let s = source.trim();
    // Arrow function: "() => 42" → "function() { ... }"
    if s.contains("=>") && !s.starts_with("function") {
        return "function() { ... }".to_string();
    }
    // Named function: "function foo(a, b) { ... }" → "function foo() { ... }"
    // Find the opening brace
    if let Some(brace_pos) = s.find('{') {
        let signature = &s[..brace_pos];
        // Simplify params: "function foo(a, b)" → "function foo()"
        if let Some(paren_open) = signature.find('(') {
            let name_part = &signature[..paren_open];
            return format!("{}() {{ ... }}", name_part.trim());
        }
        return format!("{}{{ ... }}", signature.trim());
    }
    // Fallback
    "function() { ... }".to_string()
}

/// Call Object.prototype.toString on a value to get "[object Type]".
fn call_object_to_string(scope: &v8::PinScope<'_, '_>, value: v8::Local<v8::Value>) -> String {
    // Try to call Object.prototype.toString.call(value)
    let global = scope.get_current_context().global(scope);
    let obj_key = v8::String::new(scope, "Object").expect("key");
    if let Some(obj_ctor) = global.get(scope, obj_key.into()) {
        if obj_ctor.is_function() {
            let obj_ctor: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(obj_ctor) };
            let proto_key = v8::String::new(scope, "prototype").expect("key");
            if let Some(proto) = obj_ctor.get(scope, proto_key.into()) {
                if proto.is_object() {
                    let proto_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(proto) };
                    let ts_key = v8::String::new(scope, "toString").expect("key");
                    if let Some(ts_fn) = proto_obj.get(scope, ts_key.into()) {
                        if ts_fn.is_function() {
                            let ts_fn: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(ts_fn) };
                            if let Some(result) = ts_fn.call(scope, value, &[]) {
                                return result.to_rust_string_lossy(scope);
                            }
                        }
                    }
                }
            }
        }
    }
    // Fallback
    "[object Object]".to_string()
}

/// Handle circular reference: convert object but truncate at depth limit.
/// For the circular fixture, iv8 returns {'self': '[object Object]'} at depth 1.
fn convert_circular_object(
    scope: &v8::PinScope<'_, '_>,
    obj: v8::Local<v8::Object>,
    depth: u32,
) -> RustValue {
    let keys = obj.get_own_property_names(scope, Default::default());
    if let Some(keys) = keys {
        let len = keys.length();
        let mut map = std::collections::HashMap::with_capacity(len as usize);
        for i in 0..len {
            if let Some(key) = keys.get_index(scope, i) {
                let key_str = key.to_rust_string_lossy(scope);
                let val = obj.get(scope, key).unwrap_or_else(|| v8::undefined(scope).into());
                map.insert(key_str, v8_to_rust_impl(scope, val, depth + 1));
            }
        }
        RustValue::Object(map)
    } else {
        RustValue::Object(std::collections::HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::embedded_v8::EmbeddedV8Kernel;
    use crate::kernel::{EvalOpts, KernelConfig};

    fn eval_to_rust(source: &str) -> RustValue {
        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel.eval_to_rust_value(source)
    }

    #[test]
    fn convert_integer() {
        assert_eq!(eval_to_rust("42"), RustValue::Int(42));
        assert_eq!(eval_to_rust("0"), RustValue::Int(0));
        assert_eq!(eval_to_rust("-1"), RustValue::Int(-1));
    }

    #[test]
    fn convert_float() {
        assert_eq!(eval_to_rust("3.14"), RustValue::Float(3.14));
        match eval_to_rust("NaN") {
            RustValue::Float(n) => assert!(n.is_nan()),
            other => panic!("expected Float(NaN), got {:?}", other),
        }
        assert_eq!(eval_to_rust("Infinity"), RustValue::Float(f64::INFINITY));
        assert_eq!(eval_to_rust("-Infinity"), RustValue::Float(f64::NEG_INFINITY));
    }

    #[test]
    fn convert_string() {
        assert_eq!(eval_to_rust("'hello'"), RustValue::String("hello".into()));
        assert_eq!(eval_to_rust("''"), RustValue::String("".into()));
    }

    #[test]
    fn convert_boolean() {
        assert_eq!(eval_to_rust("true"), RustValue::Bool(true));
        assert_eq!(eval_to_rust("false"), RustValue::Bool(false));
    }

    #[test]
    fn convert_null_undefined() {
        assert_eq!(eval_to_rust("null"), RustValue::Null);
        assert_eq!(eval_to_rust("undefined"), RustValue::Null);
    }

    #[test]
    fn convert_array() {
        assert_eq!(
            eval_to_rust("[1, 'two', true, null]"),
            RustValue::Array(vec![
                RustValue::Int(1),
                RustValue::String("two".into()),
                RustValue::Bool(true),
                RustValue::Null,
            ])
        );
    }

    #[test]
    fn convert_nested_array() {
        assert_eq!(
            eval_to_rust("[[1, 2], [3]]"),
            RustValue::Array(vec![
                RustValue::Array(vec![RustValue::Int(1), RustValue::Int(2)]),
                RustValue::Array(vec![RustValue::Int(3)]),
            ])
        );
    }

    #[test]
    fn convert_object() {
        let result = eval_to_rust("({a: 1, b: 'two'})");
        match result {
            RustValue::Object(map) => {
                assert_eq!(map.get("a"), Some(&RustValue::Int(1)));
                assert_eq!(map.get("b"), Some(&RustValue::String("two".into())));
            }
            other => panic!("expected Object, got {:?}", other),
        }
    }

    #[test]
    fn convert_nested_object() {
        let result = eval_to_rust("({x: {y: {z: 42}}})");
        match result {
            RustValue::Object(map) => {
                match map.get("x") {
                    Some(RustValue::Object(inner)) => {
                        match inner.get("y") {
                            Some(RustValue::Object(innermost)) => {
                                assert_eq!(innermost.get("z"), Some(&RustValue::Int(42)));
                            }
                            other => panic!("expected nested Object, got {:?}", other),
                        }
                    }
                    other => panic!("expected Object at x, got {:?}", other),
                }
            }
            other => panic!("expected Object, got {:?}", other),
        }
    }

    #[test]
    fn convert_typed_array_to_bytes() {
        let result = eval_to_rust("new Uint8Array([1, 2, 3])");
        assert_eq!(result, RustValue::Bytes(vec![1, 2, 3]));
    }

    #[test]
    fn convert_bigint_returns_null_strict_compat() {
        // iv8 0.1.2 behavior: BigInt → None + error log
        let result = eval_to_rust("1n");
        assert_eq!(result, RustValue::Null);
    }

    #[test]
    fn convert_date_to_js_object_string() {
        let result = eval_to_rust("new Date(0)");
        match result {
            RustValue::JsObject(s) => {
                assert!(s.contains("1970") || s.contains("Date"), "date string: {}", s);
            }
            other => panic!("expected JsObject for Date, got {:?}", other),
        }
    }

    #[test]
    fn convert_function_to_js_object() {
        let result = eval_to_rust("(function foo() {})");
        match result {
            RustValue::JsObject(s) => {
                assert!(s.contains("function") || s.contains("foo"), "fn string: {}", s);
            }
            other => panic!("expected JsObject for Function, got {:?}", other),
        }
    }

    #[test]
    fn convert_circular_reference_degrades() {
        // iv8 behavior: circular ref at depth > 1 degrades to string
        let result = eval_to_rust("var o = {}; o.self = o; o");
        match result {
            RustValue::Object(map) => {
                // The self-reference should degrade at depth limit
                match map.get("self") {
                    Some(RustValue::Object(inner)) => {
                        // At depth 2, inner.self should still be Object...
                        // Eventually at depth 10 it degrades
                        // Just verify we don't infinite loop / stack overflow
                    }
                    Some(RustValue::JsObject(_)) => {
                        // Also acceptable — degraded early
                    }
                    other => panic!("unexpected self value: {:?}", other),
                }
            }
            other => panic!("expected Object, got {:?}", other),
        }
    }

    #[test]
    fn convert_empty_eval() {
        // eval("") returns undefined
        let result = eval_to_rust("");
        assert_eq!(result, RustValue::Null);
    }

    #[test]
    fn convert_symbol() {
        let result = eval_to_rust("Symbol('test')");
        match result {
            RustValue::String(s) => {
                assert!(s.contains("Symbol") && s.contains("test"), "symbol: {}", s);
            }
            other => panic!("expected String for Symbol, got {:?}", other),
        }
    }
}
