//! JSContext Python class — the main entry point for iv8-rs.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::sync::Arc;

use iv8_core::{EmbeddedV8Kernel, EvalOpts, IV8Error, KernelConfig, RustValue};
use parking_lot::Mutex;

use crate::error;
use crate::expose;

/// Extension trait to add Python-specific methods to EmbeddedV8Kernel.
trait KernelPyExt {
    fn expose_py_fn(&mut self, name: &str, callable: pyo3::Py<pyo3::PyAny>);
}

impl KernelPyExt for EmbeddedV8Kernel {
    fn expose_py_fn(&mut self, name: &str, callable: pyo3::Py<pyo3::PyAny>) {
        self.with_global_scope(|scope, global| {
            expose::expose_py_function(scope, global, name, callable);
        });
    }
}

/// The main iv8-rs context. Each instance owns a V8 Isolate.
///
/// Must be used from the thread that created it.
#[pyclass(frozen)]
pub struct JSContext {
    inner: Arc<JSContextInner>,
}

struct JSContextInner {
    kernel: Mutex<EmbeddedV8Kernel>,
    creator_thread: std::thread::ThreadId,
}

// SAFETY: EmbeddedV8Kernel contains V8 Isolate which is !Send + !Sync.
// We enforce single-thread access via creator_thread check in every public method.
// The Mutex is only for satisfying PyO3's #[pyclass(frozen)] requirement
// (which needs the containing type to be Send+Sync), not for actual cross-thread access.
// No V8 API is ever called from a thread other than the creator thread.
unsafe impl Send for JSContextInner {}
unsafe impl Sync for JSContextInner {}

#[pymethods]
impl JSContext {
    /// Create a new JSContext.
    #[new]
    #[pyo3(signature = (
        environment = None,
        config = None,
        time_mode = "logical",
        js_api = "__iv8__",
        strict_compat = true,
    ))]
    fn new(
        environment: Option<&Bound<'_, PyDict>>,
        config: Option<&Bound<'_, PyDict>>,
        time_mode: &str,
        js_api: &str,
        strict_compat: bool,
    ) -> PyResult<Self> {
        // Parse config dict
        let mut timezone: Option<String> = None;
        let mut locale: Option<String> = None;
        if let Some(cfg) = config {
            if let Ok(tz) = cfg.get_item("timezone") {
                if let Some(tz_val) = tz {
                    timezone = tz_val.extract::<String>().ok();
                }
            }
            if let Ok(lc) = cfg.get_item("locale") {
                if let Some(lc_val) = lc {
                    locale = lc_val.extract::<String>().ok();
                }
            }
        }

        let mut env_overrides = environment.map(|d| pydict_to_json_map(d)).transpose()?
            .unwrap_or_default();

        // Apply config → environment mappings
        if let Some(ref tz) = timezone {
            env_overrides.entry("timezone".to_string())
                .or_insert_with(|| serde_json::Value::String(tz.clone()));
        }
        if let Some(ref lc) = locale {
            env_overrides.entry("navigator.language".to_string())
                .or_insert_with(|| serde_json::Value::String(lc.clone()));
        }

        let env_overrides = if env_overrides.is_empty() { None } else { Some(env_overrides) };

        let tm = match time_mode {
            "logical" => iv8_core::state::TimeMode::Logical,
            "system" => iv8_core::state::TimeMode::System,
            other => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "invalid time_mode: '{}', expected 'logical' or 'system'",
                    other
                )));
            }
        };

        let kernel_config = KernelConfig {
            strict_compat,
            time_mode: tm,
            js_api_name: js_api.to_string(),
            environment_overrides: env_overrides,
        };

        let kernel = EmbeddedV8Kernel::new(kernel_config).map_err(error::iv8_error_to_pyerr)?;

        Ok(Self {
            inner: Arc::new(JSContextInner {
                kernel: Mutex::new(kernel),
                creator_thread: std::thread::current().id(),
            }),
        })
    }

    /// Evaluate JavaScript source code and return the result as a Python object.
    #[pyo3(signature = (source, /, name=None, line=-1, col=-1, to_py=false, devtools=true))]
    fn eval(
        &self,
        py: Python<'_>,
        source: &str,
        name: Option<&str>,
        line: i32,
        col: i32,
        to_py: bool,
        devtools: bool,
    ) -> PyResult<PyObject> {
        self.assert_thread()?;
        let _ = to_py; // Always deep-converts (our default behavior = iv8's to_py=True)
        let _ = devtools; // DevTools not yet implemented (M4)

        let opts = EvalOpts {
            source_url: name.map(|s| s.to_string()),
            line_offset: if line >= 0 { line } else { 0 },
            column_offset: if col >= 0 { col } else { 0 },
        };

        // GIL release strategy: release for source >= 256 bytes
        let rust_value = if source.len() >= 256 {
            let source_owned = source.to_string();
            py.allow_threads(|| {
                let mut kernel = self.inner.kernel.lock();
                let global = kernel.eval(&source_owned, opts)?;
                Ok::<_, IV8Error>(kernel.global_to_rust_value(&global))
            })
            .map_err(error::iv8_error_to_pyerr)?
        } else {
            let mut kernel = self.inner.kernel.lock();
            let global = kernel.eval(source, opts).map_err(error::iv8_error_to_pyerr)?;
            kernel.global_to_rust_value(&global)
        };

        rust_value_to_py(py, &rust_value)
    }

    /// Return the 393 default environment entries as a dict.
    #[classmethod]
    fn get_defaults(_cls: &Bound<'_, pyo3::types::PyType>, py: Python<'_>) -> PyResult<PyObject> {
        let env = iv8_core::EnvironmentMap::defaults();
        let dict = PyDict::new(py);
        for (key, value) in env.iter() {
            dict.set_item(key, json_value_to_py(py, value)?)?;
        }
        Ok(dict.into())
    }

    /// Close the context and release V8 resources.
    fn close(&self) -> PyResult<()> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        kernel.dispose();
        Ok(())
    }

    /// Context manager support: __enter__ returns self.
    fn __enter__(slf: pyo3::Py<Self>) -> pyo3::Py<Self> {
        slf
    }

    /// Context manager support: __exit__ closes the context.
    fn __exit__(
        &self,
        _exc_type: Option<&pyo3::Bound<'_, pyo3::PyAny>>,
        _exc_val: Option<&pyo3::Bound<'_, pyo3::PyAny>>,
        _exc_tb: Option<&pyo3::Bound<'_, pyo3::PyAny>>,
    ) -> PyResult<bool> {
        self.close()?;
        Ok(false) // don't suppress exceptions
    }

    /// Expose a Python callable as a global JS function, OR store data at __iv8__.data.name.
    ///
    /// Two modes:
    /// 1. callable: expose(name, callable) — registers a JS function
    /// 2. data: expose(data_dict, name) — stores data at __iv8__.data.name
    ///
    /// The data mode is used by iv8 examples:
    ///   ctx.expose({"html": "...", "resources": {...}}, "s1")
    ///   ctx.eval("__iv8__.page.load(__iv8__.data.s1)")
    #[pyo3(signature = (name_or_data, callable_or_name=None))]
    fn expose(&self, name_or_data: PyObject, callable_or_name: Option<PyObject>, py: Python<'_>) -> PyResult<()> {
        self.assert_thread()?;

        // Detect mode: expose(name, callable) vs expose(data, name)
        let (name, callable_opt, data_opt) = if let Ok(name_str) = name_or_data.extract::<String>(py) {
            // Mode 1: expose(name, callable)
            let callable = callable_or_name.ok_or_else(|| {
                pyo3::exceptions::PyTypeError::new_err("expose(name, callable): callable required")
            })?;
            (name_str, Some(callable), None)
        } else if let Some(ref name_val) = callable_or_name {
            // Mode 2: expose(data, name)
            let name_str = name_val.extract::<String>(py)?;
            (name_str, None, Some(name_or_data))
        } else {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "expose: use expose(name, callable) or expose(data_dict, name)"
            ));
        };

        if let Some(callable) = callable_opt {
            // Mode 1: register as JS function
            if !callable.bind(py).is_callable() {
                return Err(pyo3::exceptions::PyTypeError::new_err(format!(
                    "expose: '{}' is not callable", name
                )));
            }
            let mut kernel = self.inner.kernel.lock();
            kernel.expose_py_fn(&name, callable);
        } else if let Some(data) = data_opt {
            // Mode 2: store data at __iv8__.data.name
            // Convert Python object to JSON string, then eval to set it
            let json_str = py.import("json")?.call_method1("dumps", (data,))?.extract::<String>()?;
            let js_api = {
                let kernel = self.inner.kernel.lock();
                let state = iv8_core::state::RuntimeState::get(kernel.isolate_ref());
                state.js_api_name.clone()
            };
            let script = format!(
                r#"(function() {{
                    if (!{api}.data) {api}.data = {{}};
                    {api}.data[{name_json}] = {json};
                }})()"#,
                api = js_api,
                name_json = serde_json::to_string(&name).unwrap_or_else(|_| format!("\"{}\"", name)),
                json = json_str,
            );
            let mut kernel = self.inner.kernel.lock();
            kernel.eval(&script, iv8_core::EvalOpts::default())
                .map_err(crate::error::iv8_error_to_pyerr)?;
        }

        Ok(())
    }

    /// Add a resource to the offline bundle.
    ///
    /// When JS later calls fetch(url) or XHR, the registered response is returned.
    ///
    /// Args:
    ///     url: The URL to register.
    ///     body: Response body (str or bytes).
    ///     status: HTTP status code (default 200).
    ///     headers: Optional response headers dict.
    #[pyo3(signature = (url, body, status=200, headers=None))]
    fn add_resource(
        &self,
        url: &str,
        body: &Bound<'_, pyo3::types::PyAny>,
        status: u16,
        headers: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<()> {
        self.assert_thread()?;

        // Convert body to bytes
        let body_bytes: Vec<u8> = if let Ok(s) = body.extract::<String>() {
            s.into_bytes()
        } else if let Ok(b) = body.extract::<Vec<u8>>() {
            b
        } else {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "add_resource: body must be str or bytes",
            ));
        };

        // Convert headers
        let headers_map = headers.map(|h| {
            let mut map = std::collections::HashMap::new();
            for (k, v) in h.iter() {
                if let (Ok(key), Ok(val)) = (k.extract::<String>(), v.extract::<String>()) {
                    map.insert(key, val);
                }
            }
            map
        });

        let kernel = self.inner.kernel.lock();
        kernel.add_resource(url, body_bytes, status, headers_map);
        Ok(())
    }

    /// Check if the context has been disposed.
    fn is_disposed(&self) -> bool {
        let kernel = self.inner.kernel.lock();
        let state = iv8_core::state::RuntimeState::get(kernel.isolate_ref());
        state.is_disposed()
    }

    /// Expose all callable members of a Python module to JS global scope.
    ///
    /// If the module has `__all__`, only those names are exposed.
    /// Otherwise, all public callables (not starting with '_') are exposed.
    ///
    /// Args:
    ///     module: A Python module object.
    #[pyo3(signature = (module))]
    fn expose_module(&self, module: &Bound<'_, pyo3::types::PyModule>, py: Python<'_>) -> PyResult<()> {
        self.assert_thread()?;

        // Get names to expose
        let names: Vec<String> = if let Ok(all) = module.getattr("__all__") {
            all.extract::<Vec<String>>()?
        } else {
            // Fallback: all public names
            module
                .dir()?
                .iter()
                .filter_map(|item| {
                    let name: String = item.extract().ok()?;
                    if name.starts_with('_') {
                        None
                    } else {
                        Some(name)
                    }
                })
                .collect()
        };

        for name in &names {
            if let Ok(attr) = module.getattr(name.as_str()) {
                if attr.is_callable() {
                    let callable: PyObject = attr.into_pyobject(py)?.into_any().unbind();
                    let mut kernel = self.inner.kernel.lock();
                    kernel.expose_py_fn(name, callable);
                }
            }
        }

        Ok(())
    }

    /// Load an HTML page: parse DOM, execute inline scripts, fire DOMContentLoaded.
    ///
    /// This is the primary way to set up a browser environment for JS execution.
    ///
    /// Args:
    ///     html: The HTML source to parse.
    ///     base_url: Optional base URL for resolving relative URLs.
    #[pyo3(signature = (html, base_url=None))]
    fn page_load(&self, html: &str, base_url: Option<&str>) -> PyResult<()> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        kernel.page_load(html, base_url);
        Ok(())
    }

    /// Start the V8 Inspector (CDP WebSocket server).
    ///
    /// Returns self for chaining: `ctx = JSContext().with_devtools(port=9229)`
    ///
    /// Args:
    ///     port: WebSocket port (default 9229)
    ///     watch_apis: List of API paths to auto-breakpoint on access
    ///     enable_console: Whether to enable DevTools console (default True)
    #[pyo3(signature = (port=9229, watch_apis=None, enable_console=true))]
    fn with_devtools(
        slf: pyo3::Py<Self>,
        port: u16,
        watch_apis: Option<Vec<String>>,
        enable_console: bool,
        py: Python<'_>,
    ) -> PyResult<pyo3::Py<Self>> {
        {
            let ctx = slf.bind(py);
            let self_ref = ctx.borrow();
            self_ref.assert_thread()?;
            let mut kernel = self_ref.inner.kernel.lock();
            let devtools_url = kernel.start_inspector(
                port,
                watch_apis.unwrap_or_default(),
                enable_console,
            );
            println!("DevTools URL: {}", devtools_url);
            // Wait for connection (up to 30s)
            kernel.wait_for_devtools(30000);
        }
        Ok(slf)
    }

    /// Get the DevTools URL for the current inspector session.
    fn get_devtools_url(&self) -> PyResult<Option<String>> {
        self.assert_thread()?;
        let kernel = self.inner.kernel.lock();
        let state = iv8_core::state::RuntimeState::get(kernel.isolate_ref());
        let session = state.inspector_session.borrow();
        Ok(session.as_ref().map(|s| s.devtools_url.clone()))
    }

    /// Process pending CDP messages (call periodically when debugging).
    fn process_inspector_messages(&self) -> PyResult<()> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        kernel.process_inspector_messages();
        Ok(())
    }

    /// Set a Python network handler for fetch/XHR fallback.
    ///
    /// The handler is called when a URL is not in the ResourceBundle.
    /// It receives (url: str, method: str) and should return:
    ///   - (status: int, body: str|bytes) to provide a response
    ///   - None to reject with NetworkError
    ///
    /// Example:
    ///     def handler(url, method):
    ///         if 'api.example.com' in url:
    ///             return (200, '{"ok": true}')
    ///         return None
    ///     ctx.set_network_handler(handler)
    #[pyo3(signature = (handler))]
    fn set_network_handler(&self, handler: PyObject, py: Python<'_>) -> PyResult<()> {
        self.assert_thread()?;

        if !handler.bind(py).is_callable() {
            return Err(pyo3::exceptions::PyTypeError::new_err(
                "set_network_handler: handler must be callable",
            ));
        }

        let handler_clone = handler.clone_ref(py);
        let rust_handler: iv8_core::state::NetworkHandler = Box::new(move |url: &str, method: &str| {
            Python::with_gil(|py| {
                let result = handler_clone.call1(py, (url, method)).ok()?;
                if result.is_none(py) {
                    return None;
                }
                // Expect (status, body) tuple
                let tuple = result.downcast_bound::<pyo3::types::PyTuple>(py).ok()?;
                if tuple.len() < 2 {
                    return None;
                }
                let status: u16 = tuple.get_item(0).ok()?.extract().ok()?;
                let body_item = tuple.get_item(1).ok()?;
                let body: Vec<u8> = if let Ok(s) = body_item.extract::<String>() {
                    s.into_bytes()
                } else if let Ok(b) = body_item.extract::<Vec<u8>>() {
                    b
                } else {
                    return None;
                };
                Some((status, body))
            })
        });

        let kernel = self.inner.kernel.lock();
        kernel.set_network_handler(rust_handler);
        Ok(())
    }

    /// Clear the network handler (revert to offline-only mode).
    fn clear_network_handler(&self) -> PyResult<()> {
        self.assert_thread()?;
        let kernel = self.inner.kernel.lock();
        kernel.clear_network_handler();
        Ok(())
    }

    /// Evaluate JavaScript and await the result if it's a Promise.
    ///
    /// If the result is a Promise, runs the event loop until it settles
    /// (up to max_ticks iterations). Returns the resolved value.
    ///
    /// Args:
    ///     source: JavaScript source code.
    ///     max_ticks: Maximum event loop ticks to wait (default 1000).
    #[pyo3(signature = (source, max_ticks=1000))]
    fn eval_promise(&self, py: Python<'_>, source: &str, max_ticks: u32) -> PyResult<PyObject> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        let global = kernel.eval_await(source, max_ticks).map_err(error::iv8_error_to_pyerr)?;
        let rust_value = kernel.global_to_rust_value(&global);
        rust_value_to_py(py, &rust_value)
    }

    /// Get all console messages captured since context creation.
    ///
    /// Returns a list of dicts with 'level' and 'text' keys.
    /// Levels: 'log', 'info', 'warn', 'error', 'debug', 'trace', 'assert'
    fn get_console_messages(&self, py: Python<'_>) -> PyResult<PyObject> {        self.assert_thread()?;
        let kernel = self.inner.kernel.lock();
        let state = iv8_core::state::RuntimeState::get(kernel.isolate_ref());
        let messages = state.console_messages.borrow();
        let list = pyo3::types::PyList::empty(py);
        for msg in messages.iter() {
            let dict = PyDict::new(py);
            dict.set_item("level", &msg.level)?;
            dict.set_item("text", &msg.text)?;
            list.append(dict)?;
        }
        Ok(list.into_any().into())
    }

    /// Clear all captured console messages.
    fn clear_console_messages(&self) -> PyResult<()> {
        self.assert_thread()?;
        let kernel = self.inner.kernel.lock();
        let state = iv8_core::state::RuntimeState::get(kernel.isolate_ref());
        state.console_messages.borrow_mut().clear();
        Ok(())
    }
}

impl JSContext {
    fn assert_thread(&self) -> PyResult<()> {
        if std::thread::current().id() != self.inner.creator_thread {
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                "JSContext must be used from the thread that created it",
            ));
        }
        Ok(())
    }
}

// --- Helpers ---

/// Convert a Python dict to HashMap<String, serde_json::Value>.
/// Supports both flat format {"navigator.userAgent": "..."} and
/// nested format {"navigator": {"userAgent": "..."}} (auto-flattened).
fn pydict_to_json_map(
    dict: &Bound<'_, PyDict>,
) -> PyResult<HashMap<String, serde_json::Value>> {
    let mut map = HashMap::new();
    for (key, value) in dict.iter() {
        let key_str: String = key.extract()?;
        // If value is a dict, flatten recursively with dot notation
        if let Ok(sub_dict) = value.downcast::<PyDict>() {
            flatten_dict(&key_str, sub_dict, &mut map)?;
        } else {
            let json_val = py_to_json_value(&value)?;
            map.insert(key_str, json_val);
        }
    }
    Ok(map)
}

/// Recursively flatten a nested dict into dot-path keys.
fn flatten_dict(
    prefix: &str,
    dict: &Bound<'_, PyDict>,
    map: &mut HashMap<String, serde_json::Value>,
) -> PyResult<()> {
    for (key, value) in dict.iter() {
        let key_str: String = key.extract()?;
        let full_key = format!("{}.{}", prefix, key_str);
        if let Ok(sub_dict) = value.downcast::<PyDict>() {
            flatten_dict(&full_key, sub_dict, map)?;
        } else {
            let json_val = py_to_json_value(&value)?;
            map.insert(full_key, json_val);
        }
    }
    Ok(())
}

/// Convert a Python object to serde_json::Value (basic types only).
fn py_to_json_value(obj: &Bound<'_, PyAny>) -> PyResult<serde_json::Value> {
    if obj.is_none() {
        Ok(serde_json::Value::Null)
    } else if let Ok(b) = obj.extract::<bool>() {
        Ok(serde_json::Value::Bool(b))
    } else if let Ok(i) = obj.extract::<i64>() {
        Ok(serde_json::json!(i))
    } else if let Ok(f) = obj.extract::<f64>() {
        Ok(serde_json::json!(f))
    } else if let Ok(s) = obj.extract::<String>() {
        Ok(serde_json::Value::String(s))
    } else if let Ok(list) = obj.downcast::<pyo3::types::PyList>() {
        let mut arr = Vec::new();
        for item in list.iter() {
            arr.push(py_to_json_value(&item)?);
        }
        Ok(serde_json::Value::Array(arr))
    } else if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (k, v) in dict.iter() {
            let key: String = k.extract()?;
            map.insert(key, py_to_json_value(&v)?);
        }
        Ok(serde_json::Value::Object(map))
    } else {
        Ok(serde_json::Value::String(obj.str()?.to_string()))
    }
}

/// Convert RustValue to Python object.
fn rust_value_to_py(py: Python<'_>, value: &RustValue) -> PyResult<PyObject> {
    use pyo3::types::PyList;
    match value {
        RustValue::Null => Ok(py.None()),
        RustValue::Bool(b) => Ok((*b).into_pyobject(py).expect("bool").to_owned().into_any().into()),
        RustValue::Int(i) => Ok((*i).into_pyobject(py).expect("int").into_any().into()),
        RustValue::Float(f) => Ok((*f).into_pyobject(py).expect("float").into_any().into()),
        RustValue::String(s) => Ok(s.as_str().into_pyobject(py).expect("str").into_any().into()),
        RustValue::Bytes(b) => Ok(b.as_slice().into_pyobject(py).expect("bytes").into_any().into()),
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
        RustValue::JsObject(s) => Ok(s.as_str().into_pyobject(py).expect("str").into_any().into()),
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
    }
}

/// Convert serde_json::Value to Python object.
fn json_value_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    use pyo3::types::PyList;
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok((*b).into_pyobject(py).expect("bool").to_owned().into_any().into()),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py).expect("int").into_any().into())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py).expect("float").into_any().into())
            } else {
                Ok(py.None())
            }
        }
        serde_json::Value::String(s) => Ok(s.as_str().into_pyobject(py).expect("str").into_any().into()),
        serde_json::Value::Array(arr) => {
            let list = PyList::empty(py);
            for item in arr {
                list.append(json_value_to_py(py, item)?)?;
            }
            Ok(list.into_any().into())
        }
        serde_json::Value::Object(map) => {
            let dict = PyDict::new(py);
            for (k, v) in map {
                dict.set_item(k, json_value_to_py(py, v)?)?;
            }
            Ok(dict.into_any().into())
        }
    }
}
