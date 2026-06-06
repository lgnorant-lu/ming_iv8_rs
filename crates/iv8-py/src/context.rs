//! JSContext Python class — the main entry point for iv8-rs.

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;
use std::mem::ManuallyDrop;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use iv8_core::{EmbeddedV8Kernel, EvalOpts, IV8Error, KernelConfig, RustValue};
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};

use crate::error;
use crate::expose;

/// Extension trait to add Python-specific methods to EmbeddedV8Kernel.
trait KernelPyExt {
    fn expose_py_fn(
        &mut self,
        name: &str,
        callable: pyo3::Py<pyo3::PyAny>,
    ) -> expose::ExposedPyFnHandle;
}

impl KernelPyExt for EmbeddedV8Kernel {
    fn expose_py_fn(
        &mut self,
        name: &str,
        callable: pyo3::Py<pyo3::PyAny>,
    ) -> expose::ExposedPyFnHandle {
        self.with_global_scope(|scope, global| {
            expose::expose_py_function(scope, global, name, callable)
        })
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
    kernel: ManuallyDrop<KernelCell>,
    creator_thread: std::thread::ThreadId,
    disposed: AtomicBool,
    exposed_callbacks: Mutex<Vec<expose::ExposedPyFnHandle>>,
}

struct KernelCell {
    inner: Mutex<Option<EmbeddedV8Kernel>>,
}

impl KernelCell {
    fn new(kernel: EmbeddedV8Kernel) -> Self {
        Self {
            inner: Mutex::new(Some(kernel)),
        }
    }

    #[allow(clippy::panic)]
    fn lock(&self) -> MappedMutexGuard<'_, EmbeddedV8Kernel> {
        MutexGuard::map(self.inner.lock(), |kernel| match kernel.as_mut() {
            Some(kernel) => kernel,
            // Public methods check disposed state before taking the kernel.
            // Reaching this branch means an internal lifecycle invariant broke.
            None => panic!("JSContext kernel already closed"),
        })
    }

    fn close(&self) {
        if let Some(mut kernel) = self.inner.lock().take() {
            kernel.dispose();
        }
    }
}

// SAFETY: EmbeddedV8Kernel contains V8 Isolate which is !Send + !Sync.
// We enforce single-thread access via creator_thread check in every public method.
// The Mutex is only for satisfying PyO3's #[pyclass(frozen)] requirement
// (which needs the containing type to be Send+Sync), not for actual cross-thread access.
// Public methods check creator_thread before touching V8. Drop is handled
// explicitly below so the V8 isolate is never destroyed on a foreign thread.
unsafe impl Send for JSContextInner {}
unsafe impl Sync for JSContextInner {}

impl Drop for JSContextInner {
    fn drop(&mut self) {
        if std::thread::current().id() == self.creator_thread {
            // SAFETY: the last Python reference is being dropped on the owner
            // thread, so it is safe to destroy the V8 isolate here.
            unsafe {
                ManuallyDrop::drop(&mut self.kernel);
            }
            self.free_exposed_callbacks();
        } else {
            // Dropping V8 from a non-owner thread is not safe. Leaking the
            // context is preferable to process UB/crash; public cross-thread use
            // is already rejected by assert_thread().
            tracing::error!("JSContext dropped from non-creator thread; leaking V8 isolate");
        }
    }
}

impl JSContextInner {
    fn free_exposed_callbacks(&self) {
        let handles: Vec<_> = self.exposed_callbacks.lock().drain(..).collect();
        Python::with_gil(|_| {
            for handle in handles {
                unsafe {
                    expose::free_exposed_py_function(handle);
                }
            }
        });
    }
}

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
        random_seed = None,
        crypto_seed = None,
        time_freeze = None,
    ))]
    fn new(
        environment: Option<&Bound<'_, PyDict>>,
        config: Option<&Bound<'_, PyDict>>,
        time_mode: &str,
        js_api: &str,
        strict_compat: bool,
        random_seed: Option<u64>,
        crypto_seed: Option<u64>,
        time_freeze: Option<f64>,
    ) -> PyResult<Self> {
        // Parse config dict
        let mut timezone: Option<String> = None;
        let mut locale: Option<String> = None;
        if let Some(cfg) = config {
            if let Ok(Some(tz_val)) = cfg.get_item("timezone") {
                timezone = tz_val.extract::<String>().ok();
            }
            if let Ok(Some(lc_val)) = cfg.get_item("locale") {
                locale = lc_val.extract::<String>().ok();
            }
        }

        let mut env_overrides = environment
            .map(|d| pydict_to_json_map(d))
            .transpose()?
            .unwrap_or_default();

        // Apply config → environment mappings
        if let Some(ref tz) = timezone {
            env_overrides
                .entry("timezone".to_string())
                .or_insert_with(|| serde_json::Value::String(tz.clone()));
        }
        if let Some(ref lc) = locale {
            env_overrides
                .entry("navigator.language".to_string())
                .or_insert_with(|| serde_json::Value::String(lc.clone()));
        }

        let env_overrides = if env_overrides.is_empty() {
            None
        } else {
            Some(env_overrides)
        };

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
            random_seed,
            crypto_seed,
            time_freeze,
        };

        let kernel = EmbeddedV8Kernel::new(kernel_config).map_err(error::iv8_error_to_pyerr)?;

        Ok(Self {
            inner: Arc::new(JSContextInner {
                kernel: ManuallyDrop::new(KernelCell::new(kernel)),
                creator_thread: std::thread::current().id(),
                disposed: AtomicBool::new(false),
                exposed_callbacks: Mutex::new(Vec::new()),
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
            let global = kernel
                .eval(source, opts)
                .map_err(error::iv8_error_to_pyerr)?;
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
        self.assert_creator_thread()?;
        if self.inner.disposed.swap(true, Ordering::SeqCst) {
            return Ok(());
        }
        self.inner.kernel.close();
        self.inner.free_exposed_callbacks();
        Ok(())
    }

    /// Context manager support: __enter__ returns self.
    fn __enter__(slf: pyo3::Py<Self>) -> pyo3::Py<Self> {
        slf
    }

    /// Context manager support: __exit__ closes the context.
    #[pyo3(signature = (_exc_type=None, _exc_val=None, _exc_tb=None))]
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
    fn expose(
        &self,
        name_or_data: PyObject,
        callable_or_name: Option<PyObject>,
        py: Python<'_>,
    ) -> PyResult<()> {
        self.assert_thread()?;

        // Detect mode: expose(name, callable) vs expose(data, name)
        let (name, callable_opt, data_opt) = if let Ok(name_str) =
            name_or_data.extract::<String>(py)
        {
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
                "expose: use expose(name, callable) or expose(data_dict, name)",
            ));
        };

        if let Some(callable) = callable_opt {
            // Mode 1: register as JS function
            if !callable.bind(py).is_callable() {
                return Err(pyo3::exceptions::PyTypeError::new_err(format!(
                    "expose: '{}' is not callable",
                    name
                )));
            }
            let mut kernel = self.inner.kernel.lock();
            let handle = kernel.expose_py_fn(&name, callable);
            self.inner.exposed_callbacks.lock().push(handle);
        } else if let Some(data) = data_opt {
            // Mode 2: store data at __iv8__.data.name
            // Convert Python object to JSON string, then eval to set it
            let json_str = py
                .import("json")?
                .call_method1("dumps", (data,))?
                .extract::<String>()?;
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
                name_json =
                    serde_json::to_string(&name).unwrap_or_else(|_| format!("\"{}\"", name)),
                json = json_str,
            );
            let mut kernel = self.inner.kernel.lock();
            kernel
                .eval(&script, iv8_core::EvalOpts::default())
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
        self.inner.disposed.load(Ordering::SeqCst)
    }

    /// Expose all callable members of a Python module to JS global scope.
    ///
    /// If the module has `__all__`, only those names are exposed.
    /// Otherwise, all public callables (not starting with '_') are exposed.
    ///
    /// Args:
    ///     module: A Python module object.
    #[pyo3(signature = (module))]
    fn expose_module(
        &self,
        module: &Bound<'_, pyo3::types::PyModule>,
        py: Python<'_>,
    ) -> PyResult<()> {
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
                    let handle = kernel.expose_py_fn(name, callable);
                    self.inner.exposed_callbacks.lock().push(handle);
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
    ///     wait: Whether to wait for a DevTools client to connect (default True).
    ///           Set to False for programmatic CDP use (cdp_set_breakpoint etc.)
    #[pyo3(signature = (port=9229, watch_apis=None, enable_console=true, wait=true))]
    fn with_devtools(
        slf: pyo3::Py<Self>,
        port: u16,
        watch_apis: Option<Vec<String>>,
        enable_console: bool,
        wait: bool,
        py: Python<'_>,
    ) -> PyResult<pyo3::Py<Self>> {
        {
            let ctx = slf.bind(py);
            let self_ref = ctx.borrow();
            self_ref.assert_thread()?;
            let mut kernel = self_ref.inner.kernel.lock();
            let devtools_url = kernel
                .start_inspector(port, watch_apis.unwrap_or_default(), enable_console)
                .map_err(pyo3::exceptions::PyRuntimeError::new_err)?;
            println!("DevTools URL: {}", devtools_url);
            if wait {
                // Wait for external DevTools client to connect (up to 30s)
                kernel.wait_for_devtools(30000);
            }
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

    // ─── CDP Programmatic API (v0.3 M15) ─────────────────────────────────────

    /// Set a breakpoint by script URL.
    ///
    /// Args:
    ///     url: Script URL (e.g. "tdc.js" or full URL)
    ///     line: Line number (0-based)
    ///     column: Column number (0-based, optional)
    ///     condition: JS expression; breakpoint only fires when true (optional)
    ///
    /// Returns:
    ///     breakpoint_id (str) for later removal
    ///
    /// Requires with_devtools() to have been called first.
    #[pyo3(signature = (url, line, column=None, condition=None))]
    fn cdp_set_breakpoint(
        &self,
        url: &str,
        line: u32,
        column: Option<u32>,
        condition: Option<&str>,
    ) -> PyResult<String> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        kernel
            .cdp_set_breakpoint(url, line, column, condition)
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)
    }

    /// Remove a breakpoint by id.
    fn cdp_remove_breakpoint(&self, breakpoint_id: &str) -> PyResult<()> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        kernel
            .cdp_remove_breakpoint(breakpoint_id)
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)
    }

    /// Evaluate an expression on a call frame while paused at a breakpoint.
    ///
    /// Args:
    ///     call_frame_id: Frame ID from cdp_get_call_frames()
    ///     expression: JS expression to evaluate in that frame's scope
    ///
    /// Returns:
    ///     The evaluation result (Python value)
    fn cdp_evaluate_on_frame(&self, call_frame_id: &str, expression: &str) -> PyResult<PyObject> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        let result = kernel
            .cdp_evaluate_on_frame(call_frame_id, expression)
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)?;
        Python::with_gil(|py| json_value_to_py(py, &result))
    }

    /// Resume execution after a breakpoint pause.
    fn cdp_resume(&self) -> PyResult<()> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        kernel
            .cdp_resume()
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)
    }

    /// Step over (next statement, skip function calls).
    fn cdp_step_over(&self) -> PyResult<()> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        kernel
            .cdp_step_over()
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)
    }

    /// Step into (enter function calls).
    fn cdp_step_into(&self) -> PyResult<()> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        kernel
            .cdp_step_into()
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)
    }

    /// Get call frames from the last breakpoint pause.
    ///
    /// Returns:
    ///     list of frame dicts, or None if not paused.
    ///     Each frame: {functionName, url, lineNumber, columnNumber, callFrameId, ...}
    fn cdp_get_call_frames(&self) -> PyResult<Option<PyObject>> {
        self.assert_thread()?;
        let kernel = self.inner.kernel.lock();
        match kernel.cdp_get_call_frames() {
            Some(frames) => Python::with_gil(|py| {
                let obj = json_value_to_py(py, &frames)?;
                Ok(Some(obj))
            }),
            None => Ok(None),
        }
    }

    /// Get properties of a scope object (enumerate closure/local variables).
    ///
    /// Use while paused at a breakpoint. Get the objectId from call frames'
    /// scopeChain[i].object.objectId.
    ///
    /// Args:
    ///     object_id: The remote object ID (from scope chain)
    ///     own_properties: If True, only own properties (default True)
    ///
    /// Returns:
    ///     dict with 'result' (list of property descriptors) from Runtime.getProperties
    #[pyo3(signature = (object_id, own_properties=true))]
    fn cdp_get_scope_properties(
        &self,
        object_id: &str,
        own_properties: bool,
    ) -> PyResult<PyObject> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        let result = kernel
            .cdp_get_properties(object_id, own_properties)
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)?;
        Python::with_gil(|py| json_value_to_py(py, &result))
    }

    /// Process CDP events (check if execution paused at breakpoint).
    /// Returns True if a Debugger.paused event was received.
    fn cdp_process_events(&self) -> PyResult<bool> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        Ok(kernel.cdp_process_events())
    }

    // ─── Trace Mode (v0.3 M16) ───────────────────────────────────────────────

    /// Set a trace point: a conditional breakpoint that doesn't pause execution
    /// but records data via a side-effect expression.
    ///
    /// Internally sets a CDP breakpoint with condition:
    ///   `(__iv8_trace__.push(<expression>), false)`
    ///
    /// The expression should evaluate to a JSON-serializable value (e.g.
    /// `JSON.stringify({pc:pc, op:H[pc], s:stack.slice(0,3)})`).
    ///
    /// Call `get_trace_log()` after execution to retrieve all recorded entries.
    ///
    /// Args:
    ///     url: Script URL to set trace point in
    ///     line: Line number (0-based)
    ///     column: Column number (0-based, optional)
    ///     expression: JS expression to evaluate and record each time the line is hit
    ///
    /// Returns:
    ///     trace_point_id (str) for later removal via remove_trace_point()
    ///
    /// Example:
    ///     tp = ctx.set_trace_point("tdc.js", 1234, None,
    ///         "JSON.stringify({pc:pc, op:H[pc]})")
    ///     ctx.eval("TDC.getData(true)")
    ///     trace = ctx.get_trace_log()
    #[pyo3(signature = (url, line, column=None, expression="'hit'"))]
    fn set_trace_point(
        &self,
        url: &str,
        line: u32,
        column: Option<u32>,
        expression: &str,
    ) -> PyResult<String> {
        self.assert_thread()?;

        // Ensure __iv8_trace__ array exists
        {
            let mut kernel = self.inner.kernel.lock();
            kernel
                .eval(
                    "if (typeof __iv8_trace__ === 'undefined') { var __iv8_trace__ = []; }",
                    iv8_core::EvalOpts::default(),
                )
                .ok();
        }

        // Build condition: push expression result to trace array, return false (don't pause)
        // Respects __iv8_trace_limit__ if set.
        let condition = format!(
            "(typeof __iv8_trace_limit__ === 'undefined' || __iv8_trace__.length < __iv8_trace_limit__) \
             ? (__iv8_trace__.push({}), false) : false",
            expression
        );

        // Set the breakpoint via CDP
        let mut kernel = self.inner.kernel.lock();
        kernel
            .cdp_set_breakpoint(url, line, column, Some(&condition))
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)
    }

    /// Remove a trace point by id (returned by set_trace_point).
    fn remove_trace_point(&self, trace_point_id: &str) -> PyResult<()> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        kernel
            .cdp_remove_breakpoint(trace_point_id)
            .map_err(pyo3::exceptions::PyRuntimeError::new_err)
    }

    /// Get the trace log: all entries recorded by trace points since last clear.
    ///
    /// Returns:
    ///     list of values (whatever the trace point expressions produced)
    fn get_trace_log(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        let global = kernel
            .eval(
                "typeof __iv8_trace__ !== 'undefined' ? __iv8_trace__ : []",
                iv8_core::EvalOpts::default(),
            )
            .map_err(crate::error::iv8_error_to_pyerr)?;
        let rust_value = kernel.global_to_rust_value(&global);
        rust_value_to_py(py, &rust_value)
    }

    /// Clear the trace log.
    fn clear_trace_log(&self) -> PyResult<()> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        kernel
            .eval("__iv8_trace__ = [];", iv8_core::EvalOpts::default())
            .map_err(crate::error::iv8_error_to_pyerr)?;
        Ok(())
    }

    /// Set a maximum trace log size. When reached, trace points stop recording.
    ///
    /// Args:
    ///     max_entries: Maximum number of entries before auto-stop.
    ///                  Set to 0 to disable limit.
    fn set_trace_limit(&self, max_entries: u32) -> PyResult<()> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        if max_entries == 0 {
            kernel
                .eval(
                    "__iv8_trace_limit__ = Infinity;",
                    iv8_core::EvalOpts::default(),
                )
                .map_err(crate::error::iv8_error_to_pyerr)?;
        } else {
            kernel
                .eval(
                    &format!("__iv8_trace_limit__ = {};", max_entries),
                    iv8_core::EvalOpts::default(),
                )
                .map_err(crate::error::iv8_error_to_pyerr)?;
        }
        Ok(())
    }

    // ─── M19: Deep Trace Enhancement (Layer 4 + 5) ────────────────────────────

    /// Detect ChaosVM/JSVMP variable names from JS source code.
    ///
    /// Searches for common patterns:
    /// - Handler array: `A[Q[U++]]()` or `handlers[pc++]`
    /// - PC variable: incremented in dispatch loop
    /// - Stack variable: push/pop patterns
    ///
    /// Args:
    ///     source: The JS source code to analyze (e.g. tdc.js content)
    ///
    /// Returns:
    ///     dict with detected variable names, or None if no VM pattern found.
    ///     Example: {"handler_array": "A", "pc": "U", "stack": "S", "scope": "Q"}
    #[pyo3(signature = (source))]
    fn detect_chaosvm_vars(&self, source: &str) -> PyResult<Option<PyObject>> {
        // Heuristic patterns for ChaosVM-style dispatch:
        // Pattern 1: A[Q[U++]]() — TDC ChaosVM
        // Pattern 2: handlers[H[pc++]]() — generic
        // Pattern 3: switch(bytecode[ip]) — switch-based VM

        use pyo3::types::PyDict;

        // Search for A[X[Y++]]() pattern (handler_array[index_array[pc++]]())
        let re_handler_dispatch = regex_lite::Regex::new(
            r"([A-Za-z_$][A-Za-z0-9_$]*)\[([A-Za-z_$][A-Za-z0-9_$]*)\[([A-Za-z_$][A-Za-z0-9_$]*)\+\+\]\]",
        );

        if let Ok(re) = re_handler_dispatch {
            if let Some(caps) = re.captures(source) {
                let handler_array = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let index_array = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                let pc_var = caps.get(3).map(|m| m.as_str()).unwrap_or("");

                // Try to find stack variable (look for .push/.pop patterns near the dispatch)
                let stack_re = regex_lite::Regex::new(
                    r"([A-Za-z_$][A-Za-z0-9_$]*)\.push\(|([A-Za-z_$][A-Za-z0-9_$]*)\.pop\(\)",
                );
                let stack_var = if let Ok(sre) = stack_re {
                    sre.captures(source)
                        .and_then(|c| c.get(1).or(c.get(2)))
                        .map(|m| m.as_str().to_string())
                        .unwrap_or_default()
                } else {
                    String::new()
                };

                return Python::with_gil(|py| {
                    let dict = PyDict::new(py);
                    dict.set_item("handler_array", handler_array)?;
                    dict.set_item("index_array", index_array)?;
                    dict.set_item("pc", pc_var)?;
                    dict.set_item("stack", &stack_var)?;
                    Ok(Some(dict.into_any().unbind()))
                });
            }
        }

        Ok(None)
    }

    /// Instrument a ChaosVM/JSVMP handler array for high-performance tracing.
    ///
    /// Wraps the handler array with a Proxy that records every dispatch call.
    /// Much faster than CDP breakpoints (~0.5s for 50000 instructions vs 30s+).
    ///
    /// Args:
    ///     handler_array: Variable name of the handler/function array (e.g. "A")
    ///     pc_var: Variable name of the program counter (e.g. "U")
    ///     stack_var: Variable name of the stack (e.g. "S")
    ///     capture_stack_depth: How many stack top elements to capture (default 3)
    ///     limit: Maximum trace entries (default 100000)
    ///
    /// After calling this, execute JS normally. Then call get_vm_trace() to retrieve.
    #[pyo3(signature = (handler_array, pc_var, stack_var, capture_stack_depth=3, limit=100000))]
    fn instrument_chaosvm(
        &self,
        handler_array: &str,
        pc_var: &str,
        stack_var: &str,
        capture_stack_depth: u32,
        limit: u32,
    ) -> PyResult<()> {
        self.assert_thread()?;
        let js = format!(
            r#"
(function() {{
    var __orig_handlers = {handler};
    var __vm_log = [];
    var __vm_limit = {limit};
    {handler} = new Proxy(__orig_handlers, {{
        get: function(target, prop) {{
            var fn = target[prop];
            if (typeof fn !== 'function') return fn;
            return function() {{
                if (__vm_log.length < __vm_limit) {{
                    var entry = {pc} + ',' + prop;
                    var st = {stack};
                    if (st && st.length > 0) {{
                        var depth = Math.min(st.length, {depth});
                        for (var i = st.length - depth; i < st.length; i++) {{
                            var v = st[i];
                            entry += ',' + (typeof v === 'string' ? v.slice(0,20) : String(v));
                        }}
                    }}
                    __vm_log.push(entry);
                }}
                return fn.apply(this, arguments);
            }};
        }}
    }});
    globalThis.__iv8_vm_log__ = __vm_log;
    globalThis.__iv8_vm_orig_handlers__ = __orig_handlers;
}})();
"#,
            handler = handler_array,
            pc = pc_var,
            stack = stack_var,
            depth = capture_stack_depth,
            limit = limit,
        );

        let mut kernel = self.inner.kernel.lock();
        kernel
            .eval(&js, iv8_core::EvalOpts::default())
            .map_err(crate::error::iv8_error_to_pyerr)?;
        Ok(())
    }

    /// Get the VM trace log (after instrument_chaosvm + execution).
    ///
    /// Returns:
    ///     list of trace entry strings. Each entry: "pc,opcode,stack0,stack1,..."
    ///     Parse with: pc, op, *stack = entry.split(',')
    fn get_vm_trace(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        let global = kernel
            .eval(
                "typeof __iv8_vm_log__ !== 'undefined' ? __iv8_vm_log__ : []",
                iv8_core::EvalOpts::default(),
            )
            .map_err(crate::error::iv8_error_to_pyerr)?;
        let rv = kernel.global_to_rust_value(&global);
        rust_value_to_py(py, &rv)
    }

    /// Clear the VM trace log.
    fn clear_vm_trace(&self) -> PyResult<()> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        kernel
            .eval(
                "if (typeof __iv8_vm_log__ !== 'undefined') __iv8_vm_log__.length = 0;",
                iv8_core::EvalOpts::default(),
            )
            .map_err(crate::error::iv8_error_to_pyerr)?;
        Ok(())
    }

    /// Restore original handler array (undo instrument_chaosvm).
    fn uninstrument_chaosvm(&self, handler_array: &str) -> PyResult<()> {
        self.assert_thread()?;
        let js = format!(
            "if (typeof __iv8_vm_orig_handlers__ !== 'undefined') {{ {} = __iv8_vm_orig_handlers__; }}",
            handler_array
        );
        let mut kernel = self.inner.kernel.lock();
        kernel
            .eval(&js, iv8_core::EvalOpts::default())
            .map_err(crate::error::iv8_error_to_pyerr)?;
        Ok(())
    }

    /// Get the unified trace log (from instrument_source injection).
    ///
    /// Returns entries in format: "TYPE,PC,target,value"
    /// - D,pc,opcode,stack_depth — VM dispatch
    /// - R,pc,obj.prop,value — Environment read
    /// - C,pc,obj.method,result — Function call
    /// - W,pc,obj.prop,value — Property write
    fn get_unified_trace(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        let global = kernel
            .eval(
                "typeof __iv8i_log__ !== 'undefined' ? __iv8i_log__ : []",
                iv8_core::EvalOpts::default(),
            )
            .map_err(crate::error::iv8_error_to_pyerr)?;
        let rv = kernel.global_to_rust_value(&global);
        rust_value_to_py(py, &rv)
    }

    /// Clear the unified trace log.
    fn clear_unified_trace(&self) -> PyResult<()> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        kernel
            .eval(
                "if (typeof __iv8i_log__ !== 'undefined') __iv8i_log__.length = 0;",
                iv8_core::EvalOpts::default(),
            )
            .map_err(crate::error::iv8_error_to_pyerr)?;
        Ok(())
    }

    /// Start recording all property reads/writes/calls on specified global objects.
    ///
    /// This is "Layer 5 approximate" — records all observable interactions between
    /// JS code and the browser environment, without modifying the JS source.
    ///
    /// Args:
    ///     targets: List of global object names to monitor
    ///              (default: navigator, screen, document, location, Math, crypto, performance)
    ///     record_reads: Record property reads (default True)
    ///     record_writes: Record property writes (default True)
    ///     record_calls: Record function calls (default True)
    ///     limit: Maximum entries (default 50000)
    #[pyo3(signature = (targets=None, record_reads=true, record_writes=true, record_calls=true, limit=50000))]
    fn start_recording(
        &self,
        targets: Option<Vec<String>>,
        record_reads: bool,
        record_writes: bool,
        record_calls: bool,
        limit: u32,
    ) -> PyResult<()> {
        self.assert_thread()?;

        let target_list = targets.unwrap_or_else(|| {
            vec![
                "navigator".into(),
                "screen".into(),
                "document".into(),
                "location".into(),
                "Math".into(),
                "crypto".into(),
                "performance".into(),
            ]
        });

        let targets_json = serde_json::to_string(&target_list).unwrap_or("[]".into());

        let js = format!(
            r#"
(function() {{
    var __rec = [];
    var __rec_limit = {limit};
    var __rec_reads = {reads};
    var __rec_writes = {writes};
    var __rec_calls = {calls};
    var __targets = {targets};
    var __originals = {{}};

    __targets.forEach(function(name) {{
        var obj = globalThis[name];
        if (!obj || typeof obj !== 'object') return;
        __originals[name] = obj;

        var proxy = new Proxy(obj, {{
            get: function(target, prop, receiver) {{
                var val = Reflect.get(target, prop, receiver);
                if (__rec.length >= __rec_limit) return val;
                if (typeof prop === 'symbol') return val;
                if (prop === 'then' || prop === 'toJSON') return val;

                if (typeof val === 'function' && __rec_calls) {{
                    return function() {{
                        var result = val.apply(target, arguments);
                        if (__rec.length < __rec_limit) {{
                            __rec.push('C,' + name + '.' + prop + ',' + String(result).slice(0,30));
                        }}
                        return result;
                    }};
                }}

                if (__rec_reads) {{
                    __rec.push('R,' + name + '.' + prop + ',' + String(val).slice(0,30));
                }}
                return val;
            }},
            set: function(target, prop, value, receiver) {{
                if (__rec_writes && __rec.length < __rec_limit) {{
                    __rec.push('W,' + name + '.' + prop + ',' + String(value).slice(0,30));
                }}
                return Reflect.set(target, prop, value, receiver);
            }}
        }});

        try {{
            Object.defineProperty(globalThis, name, {{
                value: proxy, writable: true, configurable: true, enumerable: true
            }});
        }} catch(e) {{
            // non-configurable (e.g. navigator) — skip silently
        }}
    }});

    globalThis.__iv8_recording__ = __rec;
    globalThis.__iv8_rec_originals__ = __originals;
}})();
"#,
            limit = limit,
            reads = record_reads,
            writes = record_writes,
            calls = record_calls,
            targets = targets_json,
        );

        let mut kernel = self.inner.kernel.lock();
        kernel
            .eval(&js, iv8_core::EvalOpts::default())
            .map_err(crate::error::iv8_error_to_pyerr)?;
        Ok(())
    }

    /// Stop recording and return all captured entries.
    ///
    /// Returns:
    ///     list of entry strings. Format: "TYPE,target.prop,value"
    ///     TYPE: R=read, W=write, C=call
    fn stop_recording(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();

        // Restore originals
        kernel.eval(r#"
(function() {
    if (typeof __iv8_rec_originals__ === 'undefined') return;
    for (var name in __iv8_rec_originals__) {
        try {
            Object.defineProperty(globalThis, name, {
                value: __iv8_rec_originals__[name], writable: true, configurable: true, enumerable: true
            });
        } catch(e) {}
    }
})();
"#, iv8_core::EvalOpts::default()).ok();

        // Get recording
        let global = kernel
            .eval(
                "typeof __iv8_recording__ !== 'undefined' ? __iv8_recording__ : []",
                iv8_core::EvalOpts::default(),
            )
            .map_err(crate::error::iv8_error_to_pyerr)?;
        let rv = kernel.global_to_rust_value(&global);
        rust_value_to_py(py, &rv)
    }

    /// Start V8 CPU Profiler (function-level call graph).
    ///
    /// Requires with_devtools(wait=False) to have been called.
    /// After execution, call stop_profiler() to get the profile.
    fn start_profiler(&self) -> PyResult<()> {
        self.assert_thread()?;
        let mut kernel = self.inner.kernel.lock();
        kernel
            .cdp_set_breakpoint("__profiler_dummy__", 0, None, None)
            .ok(); // ensure debugger enabled
                   // Use CDP to start profiler
        let state = iv8_core::state::RuntimeState::get(kernel.isolate_ref());
        let session_guard = state.inspector_session.borrow();
        if let Some(ref session) = *session_guard {
            if let Some(v8_session) = session.session_ref() {
                let cdp = state.cdp_client.borrow();
                if let Some(ref c) = *cdp {
                    c.send_and_wait(v8_session, "Profiler.enable", serde_json::json!({}))
                        .ok();
                    c.send_and_wait(v8_session, "Profiler.start", serde_json::json!({}))
                        .ok();
                }
            }
        }
        Ok(())
    }

    /// Stop V8 CPU Profiler and return the profile data.
    ///
    /// Returns:
    ///     dict with V8 CPU Profile format (nodes, startTime, endTime, samples)
    fn stop_profiler(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.assert_thread()?;
        let kernel = self.inner.kernel.lock();
        let state = iv8_core::state::RuntimeState::get(kernel.isolate_ref());
        let session_guard = state.inspector_session.borrow();
        if let Some(ref session) = *session_guard {
            if let Some(v8_session) = session.session_ref() {
                let cdp = state.cdp_client.borrow();
                if let Some(ref c) = *cdp {
                    if let Ok(result) =
                        c.send_and_wait(v8_session, "Profiler.stop", serde_json::json!({}))
                    {
                        if let Some(profile) = result.get("result").and_then(|r| r.get("profile")) {
                            return json_value_to_py(py, profile);
                        }
                    }
                }
            }
        }
        Ok(py.None())
    }

    /// Start precise code coverage collection.
    ///
    /// Requires with_devtools(wait=False).
    fn start_coverage(&self) -> PyResult<()> {
        self.assert_thread()?;
        let kernel = self.inner.kernel.lock();
        let state = iv8_core::state::RuntimeState::get(kernel.isolate_ref());
        let session_guard = state.inspector_session.borrow();
        if let Some(ref session) = *session_guard {
            if let Some(v8_session) = session.session_ref() {
                let cdp = state.cdp_client.borrow();
                if let Some(ref c) = *cdp {
                    c.send_and_wait(v8_session, "Profiler.enable", serde_json::json!({}))
                        .ok();
                    c.send_and_wait(
                        v8_session,
                        "Profiler.startPreciseCoverage",
                        serde_json::json!({"callCount": true, "detailed": true}),
                    )
                    .ok();
                }
            }
        }
        Ok(())
    }

    /// Stop coverage collection and return results.
    ///
    /// Returns:
    ///     list of script coverage dicts (scriptId, url, functions with ranges and counts)
    fn stop_coverage(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.assert_thread()?;
        let kernel = self.inner.kernel.lock();
        let state = iv8_core::state::RuntimeState::get(kernel.isolate_ref());
        let session_guard = state.inspector_session.borrow();
        if let Some(ref session) = *session_guard {
            if let Some(v8_session) = session.session_ref() {
                let cdp = state.cdp_client.borrow();
                if let Some(ref c) = *cdp {
                    if let Ok(result) = c.send_and_wait(
                        v8_session,
                        "Profiler.takePreciseCoverage",
                        serde_json::json!({}),
                    ) {
                        if let Some(cov) = result.get("result").and_then(|r| r.get("result")) {
                            return json_value_to_py(py, cov);
                        }
                    }
                }
            }
        }
        Ok(py.None())
    }

    // ─── VM-aware Helper (v0.3 M18) ──────────────────────────────────────────

    /// Detect a JSVMP dispatch loop in a loaded script.
    ///
    /// Searches for common patterns:
    /// - `while` loop with `switch` or handler array dispatch
    /// - Characteristic variable names (H, pc, stack, handlers)
    ///
    /// Args:
    ///     script_url: URL of the script to search in (e.g. "tdc.js")
    ///     patterns: Optional list of search strings to look for.
    ///              Defaults to common JSVMP patterns.
    ///
    /// Returns:
    ///     dict with {url, line, column, pattern} if found, None otherwise.
    ///
    /// Note: This uses CDP Debugger.searchInContent which requires with_devtools().
    #[pyo3(signature = (script_url, patterns=None))]
    fn detect_vm_dispatch(
        &self,
        script_url: &str,
        patterns: Option<Vec<String>>,
        py: Python<'_>,
    ) -> PyResult<Option<PyObject>> {
        self.assert_thread()?;

        let search_patterns = patterns.unwrap_or_else(|| {
            vec![
                // ChaosVM / TDC pattern
                "handlers[".to_string(),
                "while(1)".to_string(),
                "while(true)".to_string(),
                // Generic JSVMP patterns
                "switch(H[".to_string(),
                "switch(b[".to_string(),
                "case ".to_string(),
            ]
        });

        // Use eval to search in the script source via CDP
        // First, get the script ID by searching for the URL
        let mut kernel = self.inner.kernel.lock();

        for _pattern in &search_patterns {
            // Use CDP Debugger.searchInContent if available
            let search_js = r#"
(function() {
    // Search in all evaluated scripts for the pattern
    // This is a heuristic — look for the pattern in the global source
    try {
        var scripts = document.querySelectorAll('script');
        // Fallback: search in __iv8_script_sources__ if available
        return null;
    } catch(e) { return null; }
})()
"#
            .to_string();
            // For now, use a simpler approach: search the script source directly
            // The user typically knows the script URL and can find the line manually
            // or use Chrome DevTools. This method provides a programmatic hint.
            let _ = search_js;
        }

        // Simplified implementation: use CDP to search
        let result = kernel.eval(
            &format!(
                r#"
(function() {{
    // Heuristic: search for JSVMP dispatch patterns in loaded scripts
    // Returns the first match position or null
    var src = '';
    try {{
        // Try to find the script source (if it was loaded via page_load)
        var scripts = document.getElementsByTagName('script');
        for (var i = 0; i < scripts.length; i++) {{
            if (scripts[i].src && scripts[i].src.indexOf('{}') >= 0) {{
                // External script — source not directly accessible from DOM
                // User should use CDP searchInContent instead
                return null;
            }}
            if (scripts[i].textContent && scripts[i].textContent.length > 1000) {{
                src = scripts[i].textContent;
                break;
            }}
        }}
    }} catch(e) {{}}

    if (!src) return null;

    // Search for dispatch patterns
    var patterns = {};
    for (var p = 0; p < patterns.length; p++) {{
        var idx = src.indexOf(patterns[p]);
        if (idx >= 0) {{
            // Count line number
            var before = src.substring(0, idx);
            var line = before.split('\\n').length - 1;
            var lastNewline = before.lastIndexOf('\\n');
            var col = idx - lastNewline - 1;
            return JSON.stringify({{
                url: '{}',
                line: line,
                column: col,
                pattern: patterns[p],
                char_offset: idx
            }});
        }}
    }}
    return null;
}})()
"#,
                script_url,
                serde_json::to_string(&search_patterns).unwrap_or_else(|_| "[]".to_string()),
                script_url,
            ),
            iv8_core::EvalOpts::default(),
        );

        match result {
            Ok(global) => {
                let rv = kernel.global_to_rust_value(&global);
                match rv {
                    iv8_core::convert::RustValue::String(s) if !s.is_empty() => {
                        // Parse the JSON result
                        let parsed: serde_json::Value =
                            serde_json::from_str(&s).unwrap_or(serde_json::Value::Null);
                        if parsed.is_null() {
                            Ok(None)
                        } else {
                            let obj = json_value_to_py(py, &parsed)?;
                            Ok(Some(obj))
                        }
                    }
                    _ => Ok(None),
                }
            }
            Err(_) => Ok(None),
        }
    }

    /// Trace a VM dispatch loop: set a trace point and return structured results.
    ///
    /// This is a high-level convenience that combines:
    /// 1. set_trace_point at the dispatch line
    /// 2. Execute JS
    /// 3. get_trace_log and parse results
    ///
    /// Args:
    ///     url: Script URL
    ///     line: Dispatch loop line number
    ///     column: Column (optional, needed for minified single-line scripts)
    ///     vars: List of JS expressions to capture at each step.
    ///           Default: ["pc", "H[pc]"]
    ///     limit: Max trace entries (default 50000)
    ///
    /// Returns:
    ///     trace_point_id (str). After eval, call get_trace_log() to get results.
    ///
    /// Example:
    ///     tp = ctx.trace_vm("tdc.js", 1234, 0, vars=["pc", "H[pc]", "stack[0]"])
    ///     ctx.eval("TDC.getData(true)")
    ///     trace = ctx.get_trace_log()  # list of dicts
    #[pyo3(signature = (url, line, column=None, vars=None, limit=50000))]
    fn trace_vm(
        &self,
        url: &str,
        line: u32,
        column: Option<u32>,
        vars: Option<Vec<String>>,
        limit: u32,
    ) -> PyResult<String> {
        self.assert_thread()?;

        let var_list = vars.unwrap_or_else(|| vec!["pc".to_string(), "H[pc]".to_string()]);

        // Build the trace expression: JSON.stringify({var1: var1, var2: var2, ...})
        let obj_fields: Vec<String> = var_list
            .iter()
            .map(|v| {
                // Use the variable name as key, handle expressions with brackets
                let key = v.replace('[', "_").replace(']', "").replace('.', "_");
                format!("{}:{}", key, v)
            })
            .collect();
        let expression = format!("JSON.stringify({{{}}})", obj_fields.join(","));

        // Set trace limit
        self.set_trace_limit(limit)?;

        // Clear previous trace
        self.clear_trace_log()?;

        // Set the trace point
        self.set_trace_point(url, line, column, &expression)
    }

    /// Set a Python network handler for fetch/XHR fallback.
    ///
    /// The handler runs as the second tier in the three-layer chain:
    ///
    ///   1. ResourceBundle (pre-registered offline responses)
    ///   2. Python handler (this) — always called when a URL is not in the bundle
    ///   3. NetworkError (offline default when handler returns None)
    ///
    /// The handler is invoked regardless of `strict_compat` mode. It receives
    /// `(url: str, method: str)` and should return:
    ///
    ///   - `(status: int, body: str | bytes)` to provide a response
    ///   - `None` to fall through to NetworkError
    ///
    /// Both `fetch()` and synchronous XMLHttpRequest call the handler. For
    /// asynchronous XHR, the handler is invoked when the event loop drains
    /// the timer queue.
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
        let rust_handler: iv8_core::state::NetworkHandler =
            Box::new(move |url: &str, method: &str| {
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
        let global = kernel
            .eval_await(source, max_ticks)
            .map_err(error::iv8_error_to_pyerr)?;
        let rust_value = kernel.global_to_rust_value(&global);
        rust_value_to_py(py, &rust_value)
    }

    /// Get all console messages captured since context creation.
    ///
    /// Returns a list of dicts with 'level' and 'text' keys.
    /// Levels: 'log', 'info', 'warn', 'error', 'debug', 'trace', 'assert'
    fn get_console_messages(&self, py: Python<'_>) -> PyResult<PyObject> {
        self.assert_thread()?;
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
    fn assert_creator_thread(&self) -> PyResult<()> {
        if std::thread::current().id() != self.inner.creator_thread {
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                "JSContext must be used from the thread that created it",
            ));
        }
        Ok(())
    }

    fn assert_thread(&self) -> PyResult<()> {
        self.assert_creator_thread()?;
        if self.inner.disposed.load(Ordering::SeqCst) {
            return Err(pyo3::exceptions::PyRuntimeError::new_err(
                "JSContext is closed",
            ));
        }
        Ok(())
    }
}

// --- Helpers ---

/// Convert a Python dict to HashMap<String, serde_json::Value>.
/// Supports both flat format {"navigator.userAgent": "..."} and
/// nested format {"navigator": {"userAgent": "..."}} (auto-flattened).
fn pydict_to_json_map(dict: &Bound<'_, PyDict>) -> PyResult<HashMap<String, serde_json::Value>> {
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
            // Convert to Python list of typed scalars (int/float/big int).
            // The kind is currently not preserved on the Python side; users
            // who need numpy-typed arrays can construct from the list.
            use pyo3::types::PyList;
            let list = PyList::empty(py);
            for el in elements {
                list.append(rust_value_to_py(py, el)?)?;
            }
            Ok(list.into_any().unbind())
        }
    }
}

/// Convert serde_json::Value to Python object.
fn json_value_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    use pyo3::types::PyList;
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok((*b).into_pyobject(py)?.to_owned().into_any().into()),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.into_any().into())
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_pyobject(py)?.into_any().into())
            } else {
                Ok(py.None())
            }
        }
        serde_json::Value::String(s) => Ok(s.as_str().into_pyobject(py)?.into_any().into()),
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
