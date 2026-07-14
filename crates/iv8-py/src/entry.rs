//! PyO3 bindings for v0.6 Entry Plane API.
//!
//! Exposes `prepare_entry()` and `run_with_entry()` to Python.

use pyo3::prelude::*;
use pyo3::types::PyDict;

use iv8_core::entry::executor;
use iv8_core::entry::planner;
use iv8_core::entry::types::*;

/// Plan an entry strategy for a JS source.
///
/// Args:
///     source: JavaScript source code.
///     persona: "runtime" or "analysis" (default "analysis").
///     entry_targets: Optional list of target expressions.
///
/// Returns:
///     dict representing the EntryPlan.
#[pyfunction]
#[pyo3(signature = (source, persona="analysis", entry_targets=None))]
pub fn prepare_entry(
    source: &str,
    persona: &str,
    entry_targets: Option<Vec<String>>,
    py: Python<'_>,
) -> PyResult<PyObject> {
    let p = match persona {
        "runtime" => Persona::Runtime,
        "analysis" => Persona::Analysis,
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "persona must be 'runtime' or 'analysis', got '{}'",
                persona
            )))
        }
    };

    let targets = entry_targets
        .unwrap_or_default()
        .into_iter()
        .map(|v| EntryTarget {
            target_kind: EntryTargetKind::Expr,
            target_value: v,
        })
        .collect();

    let plan = planner::plan_entry(source, p, None, targets);
    let json = serde_json::to_value(&plan)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("serialization: {}", e)))?;
    json_to_py(py, &json)
}

/// Plan multiple named sources as a multi-bundle project (S7).
///
/// Args:
///     sources: list of (name, source) pairs, e.g. [("runtime.js", "..."), ("vendor.js", "...")].
///     persona: "runtime" or "analysis" (default "analysis").
///
/// Returns:
///     dict with schema iv8-multi-entry-plan.v0.1 (entries + primary plan).
///
/// Note: for joint webpack multi-chunk execution, pass chunk sources to
/// ``run_with_entry(..., chunks=[...])`` — no remote URL fetch.
#[pyfunction]
#[pyo3(signature = (sources, persona="analysis"))]
pub fn plan_multi_entry(
    sources: Vec<(String, String)>,
    persona: &str,
    py: Python<'_>,
) -> PyResult<PyObject> {
    let p = match persona {
        "runtime" => Persona::Runtime,
        "analysis" => Persona::Analysis,
        _ => {
            return Err(pyo3::exceptions::PyValueError::new_err(format!(
                "persona must be 'runtime' or 'analysis', got '{}'",
                persona
            )))
        }
    };
    let refs: Vec<(&str, &str)> = sources
        .iter()
        .map(|(n, s)| (n.as_str(), s.as_str()))
        .collect();
    let multi = planner::plan_multi_entry(&refs, p, None);
    let json = serde_json::to_value(&multi)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("serialization: {}", e)))?;
    json_to_py(py, &json)
}

/// Execute a prepared entry plan.
///
/// Args:
///     plan: EntryPlan dict (from prepare_entry).
///     source: Original JS source (or transformed source).
///     chunks: Optional list of JS **source strings** to evaluate before
///         ``source`` (e.g. vendor/runtime chunks). Order matters.
///         Caller-supplied text only — does **not** fetch URLs.
///     entry_expr: Optional expression to evaluate after main source.
///
/// Returns:
///     dict representing the EntryResult (includes module_graph when webpack).
///
/// Product path for multi-chunk webpack::
///
///     plan = prepare_entry(runtime_src)
///     result = run_with_entry(plan, page_src, chunks=[vendor_src, runtime_src])
#[pyfunction]
#[pyo3(signature = (plan, source, chunks=None, entry_expr=None))]
pub fn run_with_entry(
    plan: &Bound<'_, PyDict>,
    source: &str,
    chunks: Option<Vec<String>>,
    entry_expr: Option<&str>,
    py: Python<'_>,
) -> PyResult<PyObject> {
    // Convert PyDict to serde_json::Value for deserialization
    let plan_json = py_dict_to_json(plan)?;
    let entry_plan: EntryPlan = serde_json::from_value(plan_json)
        .map_err(|e| pyo3::exceptions::PyTypeError::new_err(format!("invalid EntryPlan: {}", e)))?;

    let chunks_vec = chunks.unwrap_or_default();
    let result = executor::run_entry(&entry_plan, source, &chunks_vec, entry_expr)
        .map_err(pyo3::exceptions::PyRuntimeError::new_err)?;

    let json = serde_json::to_value(&result)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("serialization: {}", e)))?;
    json_to_py(py, &json)
}

// ───
// Conversion helpers
// ───

fn json_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok(b.into_pyobject(py)?.to_owned().into_any().unbind()),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.into_any().unbind())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py)?.into_any().unbind())
            } else {
                Ok(py.None())
            }
        }
        serde_json::Value::String(s) => {
            Ok(s.as_str().into_pyobject(py)?.to_owned().into_any().unbind())
        }
        serde_json::Value::Array(arr) => {
            let list = pyo3::types::PyList::empty(py);
            for item in arr {
                list.append(json_to_py(py, item)?)?;
            }
            Ok(list.into_any().unbind())
        }
        serde_json::Value::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k.as_str(), json_to_py(py, v)?)?;
            }
            Ok(dict.into_any().unbind())
        }
    }
}

fn py_dict_to_json(dict: &Bound<'_, PyDict>) -> PyResult<serde_json::Value> {
    let mut map = serde_json::Map::new();
    for (key, value) in dict.iter() {
        let k: String = key.extract()?;
        let v = py_any_to_json(&value)?;
        map.insert(k, v);
    }
    Ok(serde_json::Value::Object(map))
}

fn py_any_to_json(obj: &Bound<'_, pyo3::types::PyAny>) -> PyResult<serde_json::Value> {
    if obj.is_none() {
        return Ok(serde_json::Value::Null);
    }
    if let Ok(s) = obj.extract::<String>() {
        return Ok(serde_json::Value::String(s));
    }
    if let Ok(b) = obj.extract::<bool>() {
        return Ok(serde_json::Value::Bool(b));
    }
    if let Ok(i) = obj.extract::<i64>() {
        return Ok(serde_json::json!(i));
    }
    if let Ok(f) = obj.extract::<f64>() {
        return Ok(serde_json::json!(f));
    }
    if let Ok(list) = obj.downcast::<pyo3::types::PyList>() {
        let mut arr = Vec::new();
        for item in list.iter() {
            arr.push(py_any_to_json(&item)?);
        }
        return Ok(serde_json::Value::Array(arr));
    }
    if let Ok(d) = obj.downcast::<PyDict>() {
        return py_dict_to_json(d);
    }
    Ok(serde_json::Value::String(obj.str()?.to_string()))
}
