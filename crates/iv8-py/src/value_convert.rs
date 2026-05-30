//! Shared RustValue ↔ Python conversion logic.
//!
//! Centralizes the v0.2 strict_compat=false enhancements (BigInt, DateTime,
//! Map, Set) so both `context.rs::rust_value_to_python` and
//! `expose.rs::rust_value_to_py` can use them.

use iv8_core::convert::RustValue;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PySet};

/// Convert a `RustValue::BigInt` payload into a Python `int`.
///
/// Strategy: build big-endian bytes from the little-endian u64 words, then
/// call `int.from_bytes(buf, 'big', signed=False)`. Negate if needed.
/// Zero is represented as `words = []`.
pub fn bigint_to_python(py: Python<'_>, negative: bool, words: &[u64]) -> PyResult<PyObject> {
    if words.is_empty() {
        return Ok(0i64.into_pyobject(py)?.into_any().unbind());
    }

    // Pack words (little-endian) into big-endian bytes for int.from_bytes.
    let mut buf = Vec::with_capacity(words.len() * 8);
    for w in words.iter().rev() {
        buf.extend_from_slice(&w.to_be_bytes());
    }
    // Trim leading zeros so the bytes encode the minimal representation.
    let first_nonzero = buf.iter().position(|b| *b != 0).unwrap_or(buf.len());
    let trimmed = &buf[first_nonzero..];

    let int_type = py.import("builtins")?.getattr("int")?;
    let kwargs = PyDict::new(py);
    kwargs.set_item("byteorder", "big")?;
    kwargs.set_item("signed", false)?;
    let value = int_type.call_method("from_bytes", (trimmed,), Some(&kwargs))?;
    if negative {
        // Python int negation: -value
        let neg = value.call_method0("__neg__")?;
        return Ok(neg.unbind());
    }
    Ok(value.unbind())
}

/// Convert milliseconds-since-epoch into a Python `datetime.datetime` (UTC).
pub fn ms_to_datetime(py: Python<'_>, ms: f64) -> PyResult<PyObject> {
    // datetime.datetime.fromtimestamp(ms/1000.0, tz=timezone.utc)
    let datetime_mod = py.import("datetime")?;
    let tz_utc = datetime_mod.getattr("timezone")?.getattr("utc")?;
    let dt = datetime_mod
        .getattr("datetime")?
        .call_method1("fromtimestamp", (ms / 1000.0, tz_utc))?;
    Ok(dt.unbind())
}

/// Convert a `RustValue::Map` to a Python dict.
///
/// Keys are converted by calling `key_to_py` (since they may not be hashable
/// in Python — e.g. JS object keys). Non-hashable keys are stringified by
/// calling `str()` on the Python value.
pub fn map_to_python_dict(
    py: Python<'_>,
    entries: &[(RustValue, RustValue)],
    val_to_py: &dyn Fn(Python<'_>, &RustValue) -> PyResult<PyObject>,
) -> PyResult<PyObject> {
    let dict = PyDict::new(py);
    for (k, v) in entries {
        let key_obj = val_to_py(py, k)?;
        let val_obj = val_to_py(py, v)?;
        // Try to set as-is; if key is unhashable, fall back to str(key).
        match dict.set_item(&key_obj, &val_obj) {
            Ok(()) => {}
            Err(_) => {
                let key_str = key_obj.bind(py).str()?;
                dict.set_item(key_str, val_obj)?;
            }
        }
    }
    Ok(dict.into_any().unbind())
}

/// Convert a `RustValue::Set` to a Python set.
///
/// Falls back to a list when an element is not hashable.
pub fn set_to_python_set(
    py: Python<'_>,
    values: &[RustValue],
    val_to_py: &dyn Fn(Python<'_>, &RustValue) -> PyResult<PyObject>,
) -> PyResult<PyObject> {
    let py_set = PySet::empty(py)?;
    let mut all_hashable = true;
    let py_values: Vec<PyObject> = values
        .iter()
        .map(|v| val_to_py(py, v))
        .collect::<PyResult<_>>()?;
    for v in &py_values {
        if py_set.add(v).is_err() {
            all_hashable = false;
            break;
        }
    }
    if all_hashable {
        return Ok(py_set.into_any().unbind());
    }
    // Fallback: list (preserves order, matches v0.1 Array behavior)
    let list = PyList::empty(py);
    for v in py_values {
        list.append(v)?;
    }
    Ok(list.into_any().unbind())
}
