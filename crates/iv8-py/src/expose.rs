//! expose: Python callable → V8 global function bridge.
//!
//! When `ctx.expose(name, callable)` is called from Python:
//! 1. Store `Py<PyAny>` (the callable) in a V8 External
//! 2. Create a FunctionTemplate with our trampoline as callback
//! 3. Install the function on the V8 global object
//!
//! When JS calls the exposed function:
//! 1. Trampoline extracts `Py<PyAny>` from External
//! 2. Converts V8 args → RustValue (no GIL needed)
//! 3. `Python::with_gil` → convert RustValue to PyObject → call callable
//! 4. Convert Python return → V8 value
//! 5. If Python raises → throw JS exception

use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyList, PyTuple};
use std::ffi::c_void;

use iv8_core::RustValue;

/// Data stored in V8 External for each exposed Python function.
struct ExposedPyFnData {
    callable: Py<PyAny>,
}

/// Opaque handle for context-owned exposed Python callback metadata.
pub type ExposedPyFnHandle = *mut c_void;

/// Free metadata previously returned by `expose_py_function`.
///
/// # Safety
/// The handle must come from `expose_py_function` and must be freed at most once,
/// after the V8 function using its External data can no longer be invoked.
pub unsafe fn free_exposed_py_function(handle: ExposedPyFnHandle) {
    if !handle.is_null() {
        drop(unsafe { Box::from_raw(handle as *mut ExposedPyFnData) });
    }
}

/// Register a Python callable as a named function on the V8 global object.
/// Must be called with the isolate entered and within a proper scope.
///
/// The returned handle is owned by JSContext and freed after the V8 kernel is
/// closed. V8 External does not call back to free this Rust allocation.
pub fn expose_py_function(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
    name: &str,
    callable: Py<PyAny>,
) -> ExposedPyFnHandle {
    let data = Box::new(ExposedPyFnData { callable });
    let data_ptr = Box::into_raw(data) as *mut c_void;
    let external = v8::External::new(scope, data_ptr);

    let tmpl = v8::FunctionTemplate::builder_raw(py_fn_trampoline)
        .data(external.into())
        .build(scope);

    let func = iv8_core::v8_utils::v8_fn(scope, &tmpl);
    let name_str = iv8_core::v8_utils::v8_string(scope, name);
    func.set_name(name_str);
    global.set(scope, name_str.into(), func.into());
    data_ptr
}

/// The raw extern "C" trampoline that V8 calls for exposed Python functions.
unsafe extern "C" fn py_fn_trampoline(info: *const v8::FunctionCallbackInfo) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };

        v8::callback_scope!(unsafe scope, info_ref);

        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        // Extract the ExposedPyFnData from External
        let data = args.data();
        if !data.is_external() {
            return;
        }
        let external: v8::Local<v8::External> = unsafe { v8::Local::cast_unchecked(data) };
        let fn_data = unsafe { &*(external.value() as *const ExposedPyFnData) };

        // Collect V8 arguments → RustValue (no GIL needed)
        let argc = args.length();
        let mut rust_args: Vec<RustValue> = Vec::with_capacity(argc as usize);
        for i in 0..argc {
            let arg = args.get(i);
            rust_args.push(iv8_core::v8_to_rust_impl(scope, arg, 0));
        }

        // Acquire GIL, call Python, convert result
        let py_result = Python::with_gil(|py| -> Result<RustValue, String> {
            // Convert RustValue args to Python objects
            let py_args = rust_args
                .iter()
                .map(|rv| rust_value_to_py(py, rv))
                .collect::<PyResult<Vec<PyObject>>>()
                .map_err(|e| format!("arg conversion error: {}", e))?;

            let tuple =
                PyTuple::new(py, &py_args).map_err(|e| format!("tuple creation error: {}", e))?;

            // Call the Python callable
            let result = fn_data
                .callable
                .call(py, tuple, None)
                .map_err(|e| e.to_string())?;

            // Convert Python return value to RustValue
            py_to_rust_value(py, result.bind(py))
                .map_err(|e| format!("return conversion error: {}", e))
        });

        match py_result {
            Ok(rust_val) => {
                // RustValue::Null → don't set rv → JS gets undefined
                // (Python None = JS undefined, matching iv8 behavior)
                if !matches!(rust_val, RustValue::Null) {
                    if let Some(v8_val) = rust_value_to_v8(scope, &rust_val) {
                        rv.set(v8_val);
                    }
                }
            }
            Err(err_msg) => {
                if let Some(msg) = v8::String::new(scope, &err_msg) {
                    let exc = v8::Exception::error(scope, msg);
                    scope.throw_exception(exc);
                }
            }
        }
    }));

    if result.is_err() {
        tracing::error!("panic in exposed Python function callback");
    }
}

/// Convert RustValue to a V8 Local<Value>.
fn rust_value_to_v8<'s>(
    scope: &v8::PinScope<'s, '_>,
    value: &RustValue,
) -> Option<v8::Local<'s, v8::Value>> {
    match value {
        RustValue::Null => Some(v8::null(scope).into()),
        RustValue::Bool(b) => Some(v8::Boolean::new(scope, *b).into()),
        RustValue::Int(i) => {
            if *i >= i32::MIN as i64 && *i <= i32::MAX as i64 {
                Some(v8::Integer::new(scope, *i as i32).into())
            } else {
                Some(v8::Number::new(scope, *i as f64).into())
            }
        }
        RustValue::Float(f) => Some(v8::Number::new(scope, *f).into()),
        RustValue::String(s) => v8::String::new(scope, s).map(|s| s.into()),
        RustValue::Bytes(b) => {
            let store = v8::ArrayBuffer::new_backing_store_from_vec(b.clone());
            let ab = v8::ArrayBuffer::with_backing_store(scope, &store.into());
            v8::Uint8Array::new(scope, ab, 0, b.len()).map(|arr| arr.into())
        }
        RustValue::Array(arr) => {
            let v8_arr = v8::Array::new(scope, arr.len() as i32);
            for (i, item) in arr.iter().enumerate() {
                if let Some(v) = rust_value_to_v8(scope, item) {
                    v8_arr.set_index(scope, i as u32, v);
                }
            }
            Some(v8_arr.into())
        }
        RustValue::Object(map) => {
            let obj = v8::Object::new(scope);
            for (k, v) in map {
                if let (Some(key), Some(val)) =
                    (v8::String::new(scope, k), rust_value_to_v8(scope, v))
                {
                    obj.set(scope, key.into(), val);
                }
            }
            Some(obj.into())
        }
        RustValue::JsObject(s) => v8::String::new(scope, s).map(|s| s.into()),
        RustValue::BigInt { negative, words } => {
            // Reconstruct V8 BigInt from sign + words.
            // BigInt::new_from_words requires a non-empty word slice; for zero
            // we synthesize a single-word zero.
            let words_slice: &[u64] = if words.is_empty() { &[0u64] } else { words };
            v8::BigInt::new_from_words(scope, *negative, words_slice).map(|b| b.into())
        }
        RustValue::DateTime(ms) => {
            // Build a JS Date from milliseconds since epoch.
            v8::Date::new(scope, *ms).map(|d| d.into())
        }
        RustValue::Map(entries) => {
            let m = v8::Map::new(scope);
            for (k, v) in entries {
                if let (Some(key), Some(val)) =
                    (rust_value_to_v8(scope, k), rust_value_to_v8(scope, v))
                {
                    m.set(scope, key, val);
                }
            }
            Some(m.into())
        }
        RustValue::Set(values) => {
            let s = v8::Set::new(scope);
            for v in values {
                if let Some(val) = rust_value_to_v8(scope, v) {
                    s.add(scope, val);
                }
            }
            Some(s.into())
        }
        RustValue::TypedArray { kind, elements } => {
            // Round-trip Python list -> JS TypedArray. Encode element list back
            // to bytes, then construct the matching TypedArray subtype.
            let bytes = encode_typed_array(*kind, elements);
            let store = v8::ArrayBuffer::new_backing_store_from_vec(bytes.clone());
            let ab = v8::ArrayBuffer::with_backing_store(scope, &store.into());
            let count = bytes.len() / kind.element_size().max(1);
            match kind {
                iv8_core::convert::TypedArrayKind::Uint8 => {
                    v8::Uint8Array::new(scope, ab, 0, count).map(|a| a.into())
                }
                iv8_core::convert::TypedArrayKind::Uint8Clamped => {
                    v8::Uint8ClampedArray::new(scope, ab, 0, count).map(|a| a.into())
                }
                iv8_core::convert::TypedArrayKind::Int8 => {
                    v8::Int8Array::new(scope, ab, 0, count).map(|a| a.into())
                }
                iv8_core::convert::TypedArrayKind::Uint16 => {
                    v8::Uint16Array::new(scope, ab, 0, count).map(|a| a.into())
                }
                iv8_core::convert::TypedArrayKind::Int16 => {
                    v8::Int16Array::new(scope, ab, 0, count).map(|a| a.into())
                }
                iv8_core::convert::TypedArrayKind::Uint32 => {
                    v8::Uint32Array::new(scope, ab, 0, count).map(|a| a.into())
                }
                iv8_core::convert::TypedArrayKind::Int32 => {
                    v8::Int32Array::new(scope, ab, 0, count).map(|a| a.into())
                }
                iv8_core::convert::TypedArrayKind::Float32 => {
                    v8::Float32Array::new(scope, ab, 0, count).map(|a| a.into())
                }
                iv8_core::convert::TypedArrayKind::Float64 => {
                    v8::Float64Array::new(scope, ab, 0, count).map(|a| a.into())
                }
                iv8_core::convert::TypedArrayKind::BigInt64 => {
                    v8::BigInt64Array::new(scope, ab, 0, count).map(|a| a.into())
                }
                iv8_core::convert::TypedArrayKind::BigUint64 => {
                    v8::BigUint64Array::new(scope, ab, 0, count).map(|a| a.into())
                }
            }
        }
    }
}

/// Encode a list of typed scalar `RustValue`s back into bytes per element kind.
fn encode_typed_array(kind: iv8_core::convert::TypedArrayKind, elements: &[RustValue]) -> Vec<u8> {
    use iv8_core::convert::TypedArrayKind as K;
    let mut buf = Vec::with_capacity(elements.len() * kind.element_size());
    for el in elements {
        match (kind, el) {
            (K::Uint8 | K::Uint8Clamped, RustValue::Int(i)) => {
                buf.push(*i as u8);
            }
            (K::Int8, RustValue::Int(i)) => {
                buf.push(*i as i8 as u8);
            }
            (K::Uint16, RustValue::Int(i)) => {
                buf.extend_from_slice(&(*i as u16).to_le_bytes());
            }
            (K::Int16, RustValue::Int(i)) => {
                buf.extend_from_slice(&(*i as i16).to_le_bytes());
            }
            (K::Uint32, RustValue::Int(i)) => {
                buf.extend_from_slice(&(*i as u32).to_le_bytes());
            }
            (K::Int32, RustValue::Int(i)) => {
                buf.extend_from_slice(&(*i as i32).to_le_bytes());
            }
            (K::Float32, RustValue::Float(f)) => {
                buf.extend_from_slice(&(*f as f32).to_le_bytes());
            }
            (K::Float32, RustValue::Int(i)) => {
                buf.extend_from_slice(&(*i as f32).to_le_bytes());
            }
            (K::Float64, RustValue::Float(f)) => {
                buf.extend_from_slice(&f.to_le_bytes());
            }
            (K::Float64, RustValue::Int(i)) => {
                buf.extend_from_slice(&(*i as f64).to_le_bytes());
            }
            (K::BigInt64, RustValue::Int(i)) => {
                buf.extend_from_slice(&i.to_le_bytes());
            }
            (K::BigInt64, RustValue::BigInt { negative, words }) => {
                let mag = words.first().copied().unwrap_or(0);
                let v: i64 = if *negative { -(mag as i64) } else { mag as i64 };
                buf.extend_from_slice(&v.to_le_bytes());
            }
            (K::BigUint64, RustValue::Int(i)) => {
                buf.extend_from_slice(&(*i as u64).to_le_bytes());
            }
            (K::BigUint64, RustValue::BigInt { words, .. }) => {
                let v = words.first().copied().unwrap_or(0);
                buf.extend_from_slice(&v.to_le_bytes());
            }
            _ => {
                // Type mismatch: fill with zero of element_size.
                buf.resize(buf.len() + kind.element_size(), 0);
            }
        }
    }
    buf
}

/// Convert a Python object to RustValue (for return value conversion).
fn py_to_rust_value(_py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<RustValue> {
    if obj.is_none() {
        return Ok(RustValue::Null);
    }

    // bool before int (bool is subclass of int in Python)
    if let Ok(b) = obj.extract::<bool>() {
        return Ok(RustValue::Bool(b));
    }

    if let Ok(i) = obj.extract::<i64>() {
        return Ok(RustValue::Int(i));
    }

    if let Ok(f) = obj.extract::<f64>() {
        return Ok(RustValue::Float(f));
    }

    if let Ok(s) = obj.extract::<String>() {
        return Ok(RustValue::String(s));
    }

    // Check if it's bytes type
    if obj.is_instance_of::<PyBytes>() {
        if let Ok(b) = obj.extract::<Vec<u8>>() {
            return Ok(RustValue::Bytes(b));
        }
    }

    if let Ok(list) = obj.downcast::<PyList>() {
        let mut arr = Vec::with_capacity(list.len());
        for item in list.iter() {
            arr.push(py_to_rust_value(_py, &item)?);
        }
        return Ok(RustValue::Array(arr));
    }

    if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = std::collections::HashMap::new();
        for (k, v) in dict.iter() {
            let key: String = k.extract()?;
            map.insert(key, py_to_rust_value(_py, &v)?);
        }
        return Ok(RustValue::Object(map));
    }

    // Fallback: convert to string
    let s = obj.str()?.to_string();
    Ok(RustValue::String(s))
}

/// Convert RustValue to PyObject (for argument conversion to Python).
fn rust_value_to_py(py: Python<'_>, value: &RustValue) -> PyResult<PyObject> {
    match value {
        RustValue::Null => Ok(py.None()),
        RustValue::Bool(b) => Ok((*b).into_pyobject(py)?.to_owned().into_any().into()),
        RustValue::Int(i) => Ok((*i).into_pyobject(py)?.into_any().into()),
        RustValue::Float(f) => Ok((*f).into_pyobject(py)?.into_any().into()),
        RustValue::String(s) => Ok(s.as_str().into_pyobject(py)?.into_any().into()),
        RustValue::Bytes(b) => Ok(b.as_slice().into_pyobject(py)?.into_any().into()),
        RustValue::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(rust_value_to_py(py, item)?)?;
            }
            Ok(list.into_any().into())
        }
        RustValue::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, rust_value_to_py(py, v)?)?;
            }
            Ok(dict.into_any().into())
        }
        RustValue::JsObject(s) => Ok(s.as_str().into_pyobject(py)?.into_any().into()),
        RustValue::BigInt { negative, words } => {
            crate::value_convert::bigint_to_python(py, *negative, words)
        }
        RustValue::DateTime(ms) => crate::value_convert::ms_to_datetime(py, *ms),
        RustValue::Map(entries) => {
            crate::value_convert::map_to_python_dict(py, entries, &rust_value_to_py)
        }
        RustValue::Set(values) => {
            crate::value_convert::set_to_python_set(py, values, &rust_value_to_py)
        }
        RustValue::TypedArray { elements, .. } => {
            let list = PyList::empty(py);
            for el in elements {
                list.append(rust_value_to_py(py, el)?)?;
            }
            Ok(list.into_any().unbind())
        }
    }
}
