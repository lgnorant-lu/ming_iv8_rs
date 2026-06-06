//! Debugger class — Python-facing runtime analysis assistant.
//!
//! Provides lightweight instrumentation for JS reverse engineering:
//! - API call tracing via hookNative
//! - Property watching (read/write interception)
//! - Environment snapshot
//! - Call log capture and summary

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

use crate::context::JSContext;

/// Helper: call ctx.eval(source) from Rust via PyO3.
fn ctx_eval(ctx: &Py<JSContext>, source: &str, py: Python<'_>) -> PyResult<PyObject> {
    ctx.call_method1(py, "eval", (source,))
}

/// Debugger: runtime analysis assistant for a JSContext.
///
/// Usage:
///     dbg = Debugger(ctx)
///     dbg.trace_api('Math.random')
///     ctx.eval('Math.random(); Math.random();')
///     log = dbg.get_call_log()
///     # [{'api': 'Math.random', 'args': '[]', 'result': '0.42', 'timestamp': 0.0}]
#[pyclass]
pub struct Debugger {
    ctx: Py<JSContext>,
    traced_apis: Vec<String>,
    call_log_var: String,
}

#[pymethods]
impl Debugger {
    /// Create a Debugger attached to a JSContext.
    #[new]
    fn new(ctx: Py<JSContext>, py: Python<'_>) -> PyResult<Self> {
        let call_log_var = "__iv8_dbg_log__".to_string();
        let script = format!("globalThis.{} = [];", call_log_var);
        ctx_eval(&ctx, &script, py)?;
        Ok(Self {
            ctx,
            traced_apis: Vec::new(),
            call_log_var,
        })
    }

    /// Trace all calls to a JS API path.
    ///
    /// Installs a hookNative interceptor that records every call.
    ///
    /// Args:
    ///     api_path: Dot-path like 'Math.random', 'document.getElementById'
    fn trace_api(&mut self, api_path: &str, py: Python<'_>) -> PyResult<()> {
        let log_var = self.call_log_var.clone();
        let script = format!(
            r#"
(function() {{
    var _log = globalThis.{log_var};
    __iv8__.hookNative('{api}', function(orig, args) {{
        var t = typeof performance !== 'undefined' ? performance.now() : 0;
        var result;
        var error = null;
        try {{
            result = orig.apply(this, args);
        }} catch(e) {{
            error = String(e);
        }}
        var entry = {{
            api: '{api}',
            args: JSON.stringify(args, function(k, v) {{
                if (typeof v === 'function') return '[Function]';
                if (typeof Node !== 'undefined' && v instanceof Node) return '[Node:' + (v.tagName || v.nodeName) + ']';
                return v;
            }}),
            result: error !== null ? ('[Error] ' + error) : JSON.stringify(result, function(k, v) {{
                if (typeof v === 'function') return '[Function]';
                return v;
            }}),
            timestamp: t,
        }};
        _log.push(entry);
        if (error !== null) throw new Error(error);
        return result;
    }});
}})();
"#,
            log_var = log_var,
            api = api_path,
        );
        ctx_eval(&self.ctx, &script, py)?;
        self.traced_apis.push(api_path.to_string());
        Ok(())
    }

    /// Trace multiple APIs at once.
    fn trace_apis(&mut self, api_paths: Vec<String>, py: Python<'_>) -> PyResult<()> {
        for path in api_paths {
            self.trace_api(&path, py)?;
        }
        Ok(())
    }

    /// Get the call log as a list of dicts.
    ///
    /// Each entry has: api, args, result, timestamp
    fn get_call_log(&self, py: Python<'_>) -> PyResult<PyObject> {
        let script = format!("JSON.stringify(globalThis.{})", self.call_log_var);
        let json_str: String = ctx_eval(&self.ctx, &script, py)?.extract(py)?;
        let entries: Vec<serde_json::Value> = serde_json::from_str(&json_str).unwrap_or_default();

        let list = PyList::empty(py);
        for entry in entries {
            let dict = PyDict::new(py);
            if let Some(obj) = entry.as_object() {
                for (k, v) in obj {
                    let val: PyObject = json_val_to_py(py, v)?;
                    dict.set_item(k, val)?;
                }
            }
            list.append(dict)?;
        }
        Ok(list.into_any().into())
    }

    /// Clear the call log.
    fn clear_call_log(&self, py: Python<'_>) -> PyResult<()> {
        // Use splice(0) to clear in-place, preserving references held by hooks
        let script = format!("globalThis.{}.splice(0);", self.call_log_var);
        ctx_eval(&self.ctx, &script, py)?;
        Ok(())
    }

    /// Get the list of currently traced APIs.
    fn get_traced_apis(&self) -> Vec<String> {
        self.traced_apis.clone()
    }

    /// Evaluate JS and return both the result and the call log.
    ///
    /// Returns: (result, call_log_entries)
    fn eval_traced(&self, source: &str, py: Python<'_>) -> PyResult<(PyObject, PyObject)> {
        self.clear_call_log(py)?;
        let result = ctx_eval(&self.ctx, source, py)?;
        let log = self.get_call_log(py)?;
        Ok((result, log))
    }

    /// Get a snapshot of the current environment (navigator, screen, etc.).
    fn snapshot(&self, py: Python<'_>) -> PyResult<PyObject> {
        let script = r#"JSON.stringify((function() {
    var s = {};
    s.userAgent = navigator.userAgent;
    s.platform = navigator.platform;
    s.language = navigator.language;
    s.languages = Array.from(navigator.languages || []);
    s.hardwareConcurrency = navigator.hardwareConcurrency;
    s.deviceMemory = navigator.deviceMemory;
    s.cookieEnabled = navigator.cookieEnabled;
    s.onLine = navigator.onLine;
    s.webdriver = navigator.webdriver;
    s.pdfViewerEnabled = navigator.pdfViewerEnabled;
    s.screenWidth = screen.width;
    s.screenHeight = screen.height;
    s.colorDepth = screen.colorDepth;
    s.innerWidth = window.innerWidth;
    s.innerHeight = window.innerHeight;
    s.devicePixelRatio = window.devicePixelRatio;
    s.hasChrome = typeof window.chrome === 'object';
    s.hasChromeRuntime = typeof (window.chrome && window.chrome.runtime) === 'object';
    s.hasCrypto = typeof crypto === 'object';
    s.hasSubtleCrypto = typeof (crypto && crypto.subtle) === 'object';
    s.performanceNow = performance.now();
    s.dateNow = Date.now();
    s.documentURL = document.URL;
    s.documentTitle = document.title;
    s.readyState = document.readyState;
    s.visibilityState = document.visibilityState;
    return s;
})())"#;
        let json_str: String = ctx_eval(&self.ctx, script, py)?.extract(py)?;
        let value: serde_json::Value = serde_json::from_str(&json_str)
            .unwrap_or(serde_json::Value::Object(Default::default()));

        let dict = PyDict::new(py);
        if let Some(obj) = value.as_object() {
            for (k, v) in obj {
                dict.set_item(k, json_val_to_py(py, v)?)?;
            }
        }
        Ok(dict.into_any().into())
    }

    /// Install a watch on a property — logs every read/write.
    ///
    /// Args:
    ///     obj_path: Path to the object (e.g. 'navigator', 'document')
    ///     prop: Property name to watch (e.g. 'userAgent', 'cookie')
    ///     mode: 'read', 'write', or 'both' (default 'both')
    #[pyo3(signature = (obj_path, prop, mode="both"))]
    fn watch_property(
        &mut self,
        obj_path: &str,
        prop: &str,
        mode: &str,
        py: Python<'_>,
    ) -> PyResult<()> {
        let log_var = self.call_log_var.clone();
        let api_path = format!("{}.{}", obj_path, prop);
        let watch_read = mode == "read" || mode == "both";
        let watch_write = mode == "write" || mode == "both";

        let script = format!(
            r#"
(function() {{
    var _log = globalThis.{log_var};
    var _obj = (function() {{
        var parts = '{obj_path}'.split('.');
        var o = globalThis;
        for (var i = 0; i < parts.length; i++) {{ if (o == null) return null; o = o[parts[i]]; }}
        return o;
    }})();
    if (!_obj) return;
    var _desc = Object.getOwnPropertyDescriptor(_obj, '{prop}');
    if (!_desc) return;

    // If property is non-configurable (e.g. environment-injected ReadOnly+DontDelete),
    // we cannot use Object.defineProperty. Fall back to wrapping the parent object
    // with a Proxy on the global path, intercepting get/set for this specific property.
    if (!_desc.configurable) {{
        // Strategy: replace the parent object on its parent with a Proxy.
        // For top-level objects (navigator, screen, document), replace on globalThis.
        var _parentParts = '{obj_path}'.split('.');
        var _parentParent = globalThis;
        for (var i = 0; i < _parentParts.length - 1; i++) {{
            _parentParent = _parentParent[_parentParts[i]];
        }}
        var _lastKey = _parentParts[_parentParts.length - 1];
        var _target = _parentParent[_lastKey];
        if (!_target || typeof _target !== 'object') return;

        var _proxy = new Proxy(_target, {{
            get: function(target, key, receiver) {{
                var val = Reflect.get(target, key, receiver);
                if (key === '{prop}' && {watch_read}) {{
                    _log.push({{ api: '{api_path}', args: '[]', result: JSON.stringify(val), timestamp: performance.now(), mode: 'read' }});
                }}
                return val;
            }},
            set: function(target, key, value, receiver) {{
                if (key === '{prop}' && {watch_write}) {{
                    _log.push({{ api: '{api_path}', args: JSON.stringify([value]), result: 'undefined', timestamp: performance.now(), mode: 'write' }});
                }}
                return Reflect.set(target, key, value, receiver);
            }}
        }});
        // Replace on parent (or globalThis for top-level)
        if (_parentParts.length === 1) {{
            try {{
                Object.defineProperty(globalThis, _lastKey, {{
                    value: _proxy, writable: true, configurable: true, enumerable: true
                }});
            }} catch(e) {{
                // globalThis property is also non-configurable (e.g. navigator).
                // Last resort: install a getter on the prototype chain or just
                // accept we can't intercept. Log a one-time snapshot instead.
                var _val = _target['{prop}'];
                _log.push({{ api: '{api_path}', args: '[]', result: JSON.stringify(_val), timestamp: performance.now(), mode: 'read', note: 'snapshot (non-configurable parent)' }});
            }}
        }} else {{
            _parentParent[_lastKey] = _proxy;
        }}
        return;
    }}

    // Normal path: property is configurable, use Object.defineProperty
    var _origGet = _desc.get;
    var _origSet = _desc.set;
    var _origVal = _desc.value;
    Object.defineProperty(_obj, '{prop}', {{
        get: function() {{
            var val = _origGet ? _origGet.call(this) : _origVal;
            if ({watch_read}) {{
                _log.push({{ api: '{api_path}', args: '[]', result: JSON.stringify(val), timestamp: performance.now(), mode: 'read' }});
            }}
            return val;
        }},
        set: function(v) {{
            if ({watch_write}) {{
                _log.push({{ api: '{api_path}', args: JSON.stringify([v]), result: 'undefined', timestamp: performance.now(), mode: 'write' }});
            }}
            if (_origSet) _origSet.call(this, v); else _origVal = v;
        }},
        configurable: true,
        enumerable: _desc.enumerable !== false,
    }});
}})();
"#,
            log_var = log_var,
            obj_path = obj_path,
            prop = prop,
            api_path = api_path,
            watch_read = if watch_read { "true" } else { "false" },
            watch_write = if watch_write { "true" } else { "false" },
        );

        ctx_eval(&self.ctx, &script, py)?;
        self.traced_apis.push(api_path);
        Ok(())
    }

    /// Get a summary of call counts per API.
    fn get_call_summary(&self, py: Python<'_>) -> PyResult<PyObject> {
        let script = format!(
            r#"JSON.stringify((function() {{
    var log = globalThis.{log_var};
    var counts = {{}};
    for (var i = 0; i < log.length; i++) {{
        var api = log[i].api;
        counts[api] = (counts[api] || 0) + 1;
    }}
    return counts;
}})())"#,
            log_var = self.call_log_var
        );

        let json_str: String = ctx_eval(&self.ctx, &script, py)?.extract(py)?;
        let value: serde_json::Value = serde_json::from_str(&json_str)
            .unwrap_or(serde_json::Value::Object(Default::default()));

        let dict = PyDict::new(py);
        if let Some(obj) = value.as_object() {
            for (k, v) in obj {
                dict.set_item(k, v.as_i64().unwrap_or(0))?;
            }
        }
        Ok(dict.into_any().into())
    }

    /// Schedule a pause on the next JS statement (requires DevTools connected).
    fn schedule_pause(&self, py: Python<'_>) -> PyResult<()> {
        ctx_eval(&self.ctx, "vdebugger", py).ok();
        Ok(())
    }

    fn __repr__(&self) -> String {
        format!("Debugger(traced_apis={:?})", self.traced_apis)
    }
}

// ─── helpers ──────────────────────────────────────────────────────────────────

fn json_val_to_py(py: Python<'_>, v: &serde_json::Value) -> PyResult<PyObject> {
    match v {
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
                list.append(json_val_to_py(py, item)?)?;
            }
            Ok(list.into_any().into())
        }
        serde_json::Value::Object(map) => {
            let dict = PyDict::new(py);
            for (k, val) in map {
                dict.set_item(k, json_val_to_py(py, val)?)?;
            }
            Ok(dict.into_any().into())
        }
    }
}
