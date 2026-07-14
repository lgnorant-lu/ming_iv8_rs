//! iv8-py: Python binding for iv8-rs via PyO3.

// too_many_arguments: PyO3 binding functions need many args; suppress.
#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;

mod context;
mod debugger;
mod entry;
mod error;
mod expose;
mod instrumentation;
mod logging;
mod value_convert;

/// The iv8_rs Python module.
#[pymodule]
fn _iv8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<context::JSContext>()?;
    m.add_class::<debugger::Debugger>()?;
    m.add_function(wrap_pyfunction!(logging::enable_logging, m)?)?;
    m.add_function(wrap_pyfunction!(trace_diff, m)?)?;
    m.add_function(wrap_pyfunction!(instrumentation::instrument_source, m)?)?;
    m.add_function(wrap_pyfunction!(entry::prepare_entry, m)?)?;
    m.add_function(wrap_pyfunction!(entry::plan_multi_entry, m)?)?;
    m.add_function(wrap_pyfunction!(entry::run_with_entry, m)?)?;
    error::register(m)?;
    Ok(())
}

/// Compare two trace logs and find the first divergence point.
///
/// Args:
///     trace_a: First trace log (list of strings from get_vm_trace or get_trace_log)
///     trace_b: Second trace log (list of strings)
///
/// Returns:
///     dict with divergence info:
///     - index: position of first difference (-1 if identical)
///     - a: entry from trace_a at divergence point (or None)
///     - b: entry from trace_b at divergence point (or None)
///     - total_a: length of trace_a
///     - total_b: length of trace_b
///     - match_count: number of matching entries before divergence
#[pyfunction]
#[pyo3(signature = (trace_a, trace_b))]
fn trace_diff(trace_a: Vec<String>, trace_b: Vec<String>, py: Python<'_>) -> PyResult<PyObject> {
    use pyo3::types::PyDict;

    let dict = PyDict::new(py);
    dict.set_item("total_a", trace_a.len())?;
    dict.set_item("total_b", trace_b.len())?;

    let min_len = trace_a.len().min(trace_b.len());
    let mut diverge_idx: i64 = -1;

    for i in 0..min_len {
        if trace_a[i] != trace_b[i] {
            diverge_idx = i as i64;
            break;
        }
    }

    if diverge_idx == -1 && trace_a.len() != trace_b.len() {
        // One is longer than the other
        diverge_idx = min_len as i64;
    }

    dict.set_item("index", diverge_idx)?;
    dict.set_item(
        "match_count",
        if diverge_idx >= 0 {
            diverge_idx
        } else {
            min_len as i64
        },
    )?;

    if diverge_idx >= 0 {
        let idx = diverge_idx as usize;
        dict.set_item("a", trace_a.get(idx).map(|s| s.as_str()))?;
        dict.set_item("b", trace_b.get(idx).map(|s| s.as_str()))?;
    } else {
        dict.set_item("a", py.None())?;
        dict.set_item("b", py.None())?;
    }

    Ok(dict.into_any().unbind())
}
