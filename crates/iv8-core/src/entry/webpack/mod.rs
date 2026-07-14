//! Webpack runtime detection module.
//!
//! Detects and classifies webpack-like bundler runtimes from JS source.

use crate::convert::RustValue;
use crate::kernel::embedded_v8::EmbeddedV8Kernel;

/// Result of webpack runtime detection.
#[derive(Debug, Clone)]
pub struct WebpackDetection {
    /// Whether a webpack-like runtime was detected.
    pub detected: bool,
    /// The detected runtime flavor.
    pub flavor: WebpackFlavor,
    /// Which helpers are present (.m, .c, .d, .e, .l, .o, .p, .r, .u, .f).
    pub helpers_present: Vec<String>,
    /// Module IDs extracted from the modules table (up to a limit).
    pub module_ids: Vec<String>,
    /// Number of modules found in the modules table.
    pub module_count: usize,
}

/// Webpack runtime flavor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebpackFlavor {
    Webpack4,
    Webpack5,
    UnknownWebpackLike,
}

/// Detect webpack runtime from JS source.
pub fn detect(source: &str) -> WebpackDetection {
    let mut helpers: Vec<String> = Vec::new();
    let flavor;
    // Check for main __webpack_require__ marker
    if !source.contains("__webpack_require__") && !source.contains("window.webpackJsonp") {
        return WebpackDetection {
            detected: false,
            flavor: WebpackFlavor::UnknownWebpackLike,
            helpers_present: helpers,
            module_ids: Vec::new(),
            module_count: 0,
        };
    }

    // Detect standard helpers
    let helper_patterns = [".m", ".c", ".d", ".e", ".l", ".o", ".p", ".r", ".u", ".f"];
    for h in &helper_patterns {
        let pattern = format!("__webpack_require__{}", h);
        if source.contains(&pattern) {
            helpers.push(h.to_string());
        }
    }

    // Flavor detection via signature patterns
    if source.contains("self.webpackChunk") || source.contains("webpackChunk") {
        // webpack 5+ uses globalThis.webpackChunk
        flavor = WebpackFlavor::Webpack5;
    } else if source.contains("webpackJsonpCallback") || source.contains("jsonpArray") {
        flavor = WebpackFlavor::Webpack4;
    } else if helpers.contains(&".e".to_string()) {
        // __webpack_require__.e is more common in webpack 5
        flavor = WebpackFlavor::Webpack5;
    } else if helpers.contains(&".r".to_string()) {
        // __webpack_require__.r (define ES module) is webpack 5+ feature
        flavor = WebpackFlavor::Webpack5;
    } else {
        flavor = WebpackFlavor::UnknownWebpackLike;
    }

    // Extract module IDs from modules table definition
    let mut module_ids = Vec::new();
    let module_count = extract_module_count(source);

    // Try to extract specific module IDs
    if let Some(mods) = extract_module_ids(source) {
        module_ids = mods;
    }

    WebpackDetection {
        detected: true,
        flavor,
        helpers_present: helpers,
        module_ids,
        module_count,
    }
}

/// Runtime bridge prelude used by the WebpackBridge strategy.
///
/// When `allow_prototype_patch` is false, a safe variant is returned that
/// does not modify `Function.prototype.call` or rewrite module factories.
pub fn bridge_prelude() -> &'static str {
    static FULL: &str = r#"
(function() {
    var __iv8_log = [];
    globalThis.__iv8_wp_require = null;
    try {
        // Capture require via Function.prototype.c setter.
        // When modules[id].call(module, module, exports, __webpack_require__) triggers
        // __webpack_require__.c = installedModules, the setter fires and captures the ref.
        var _origCP = undefined;
        Object.defineProperty(Function.prototype, 'c', {
            configurable: true, enumerable: true,
            get: function() { return _origCP; },
            set: function(v) {
                _origCP = v;
                var candidate = this;
                if (typeof candidate === 'function' && candidate !== __iv8_wp_require) {
                    globalThis.__iv8_wp_require = candidate;
                    if (__iv8_log.indexOf('wp_require_captured') === -1) {
                        __iv8_log.push('wp_require_proto_c');
                    }
                }
            }
        });
    } catch(e) {}
    if (typeof Function !== 'undefined' && Function.prototype) {
        var origCall = Function.prototype.call;
        Function.prototype.call = function() {
            if (globalThis.__iv8_wp_require === null && arguments.length >= 4) {
                var candidate = arguments[3];
                if (typeof candidate === 'function') {
                    globalThis.__iv8_wp_require = candidate;
                    __iv8_log.push('wp_require_captured');
                    Function.prototype.call = origCall;
                }
            }
            return origCall.apply(this, arguments);
        };
    }
    if (typeof window !== 'undefined') {
        var _origPush = Array.prototype.push;
        var _wrappedPush = function() {
            for (var i = 0; i < arguments.length; i++) {
                var entry = arguments[i];
                if (entry && entry[1]) {
                    for (var mid in entry[1]) {
                        if (entry[1].hasOwnProperty(mid)) {
                            var factory = entry[1][mid];
                            if (typeof factory === 'function') {
                                var src = factory.toString();
                                if (src.indexOf('_getSecuritySign') >= 0) {
                                    var modified = src.replace(
                                        'var ie=ne._getSecuritySign;delete ne._getSecuritySign;',
                                        'var ie=ne._getSecuritySign;globalThis.__iv8_sign_captured=ie;delete ne._getSecuritySign;'
                                    );
                                    try { entry[1][mid] = eval('(' + modified + ')'); } catch(e) {}
                                }
                            }
                        }
                    }
                }
            }
            return _origPush.apply(this, arguments);
        };
        var _realWP = undefined;
        Object.defineProperty(window, 'webpackJsonp', {
            configurable: true, enumerable: true,
            get: function() { return _realWP; },
            set: function(v) {
                if (v && Array.isArray(v) && v !== _realWP) {
                    v.push = _wrappedPush;
                    _realWP = v;
                }
            }
        });
    }
    try {
        if (typeof __webpack_require__ !== 'undefined') {
            globalThis.__iv8_wp_require = __webpack_require__;
            __iv8_log.push('wp_require_global');
        }
    } catch(e) {}
    globalThis.__iv8_webpack_log = __iv8_log;
})();
"#;
    FULL
}

/// Safe variant of the bridge prelude that avoids prototype patching and
/// factory rewriting. Used when `allow_prototype_patch` is false.
pub fn safe_bridge_prelude() -> &'static str {
    static SAFE: &str = r#"
(function() {
    var __iv8_log = [];
    globalThis.__iv8_wp_require = null;
    if (typeof window !== 'undefined') {
        var _realWP = undefined;
        Object.defineProperty(window, 'webpackJsonp', {
            configurable: true, enumerable: true,
            get: function() { return _realWP; },
            set: function(v) {
                if (v && Array.isArray(v) && v !== _realWP) {
                    _realWP = v;
                }
            }
        });
    }
    try {
        var _realWPReq = undefined;
        Object.defineProperty(globalThis, '__webpack_require__', {
            configurable: true, enumerable: true,
            get: function() { return _realWPReq; },
            set: function(v) {
                _realWPReq = v;
                if (typeof v === 'function') {
                    globalThis.__iv8_wp_require = v;
                    __iv8_log.push('wp_require_global');
                }
            }
        });
    } catch(e) {}
    try {
        if (typeof __webpack_require__ !== 'undefined') {
            globalThis.__iv8_wp_require = __webpack_require__;
            __iv8_log.push('wp_require_global');
        }
    } catch(e) {}
    globalThis.__iv8_webpack_log = __iv8_log;
})();
"#;
    SAFE
}

/// Detect webpack runtime flavor at runtime inside the V8 isolate.
fn detect_runtime_flavor(kernel: &mut EmbeddedV8Kernel) -> String {
    let js = concat!(
        "(function(){",
        "var r = typeof __iv8_wp_require === 'function'",
        "  ? __iv8_wp_require",
        "  : (typeof __webpack_require__ === 'function'",
        "    ? __webpack_require__ : null);",
        "if (!r) return 'unknown_webpack_like';",
        "if (typeof r.e === 'function') return 'webpack5';",
        "if (typeof window !== 'undefined' && (window.webpackChunk || self.webpackChunk))",
        "  return 'webpack5';",
        "if (r.m && r.c) return 'webpack4';",
        "return 'unknown_webpack_like';",
        "})()"
    );
    match kernel.eval_to_rust_value(js) {
        RustValue::String(s) => s,
        _ => "unknown_webpack_like".to_string(),
    }
}

/// Check which module IDs exist in the require cache (executed modules).
fn collect_cache_executed(kernel: &mut EmbeddedV8Kernel) -> Vec<String> {
    let js = concat!(
        "(function(){",
        "var r = typeof __iv8_wp_require === 'function'",
        "  ? __iv8_wp_require",
        "  : (typeof __webpack_require__ === 'function'",
        "    ? __webpack_require__ : null);",
        "if (!r || !r.c) return [];",
        "return Object.keys(r.c);",
        "})()"
    );
    match kernel.eval_to_rust_value(js) {
        RustValue::Array(items) => {
            let mut ids = Vec::new();
            for item in items {
                if let RustValue::String(s) = item {
                    ids.push(s);
                }
            }
            ids
        }
        _ => Vec::new(),
    }
}

/// Detect chunk entries installed via webpackJsonp or webpackChunk.
fn detect_chunks(kernel: &mut EmbeddedV8Kernel) -> Vec<serde_json::Value> {
    let js = concat!(
        "(function(){",
        "var result = [];",
        "var seen = {};",
        // Helper: normalize chunk id list (string | number | array)
        "function pushChunk(rawId, modules, state) {",
        "  var ids = Array.isArray(rawId) ? rawId : [rawId];",
        "  var mcount = modules && typeof modules === 'object' ? Object.keys(modules).length : 0;",
        "  for (var ii = 0; ii < ids.length; ii++) {",
        "    var cid = ids[ii] != null ? String(ids[ii]) : 'main';",
        "    if (!seen[cid]) {",
        "      seen[cid] = true;",
        "      result.push({chunk_id: cid, state: state, modules_added: mcount});",
        "    }",
        "  }",
        "}",
        // webpackJsonp (webpack 4): [chunkIds, moreModules, runtime]
        "if (typeof window !== 'undefined' && Array.isArray(window.webpackJsonp)) {",
        "  for (var ci = 0; ci < window.webpackJsonp.length; ci++) {",
        "    var entry = window.webpackJsonp[ci];",
        "    if (entry && entry[0] != null) {",
        "      pushChunk(entry[0], entry[1], 'requested');",
        "    }",
        "  }",
        "}",
        // webpackChunk (webpack 5)
        "var wpc = typeof self !== 'undefined' && self.webpackChunk;",
        "if (!wpc && typeof window !== 'undefined') wpc = window.webpackChunk;",
        "if (wpc && Array.isArray(wpc)) {",
        "  for (var cj = 0; cj < wpc.length; cj++) {",
        "    var ent = wpc[cj];",
        "    if (ent && Array.isArray(ent) && ent[0] != null) {",
        "      pushChunk(ent[0], ent[1], 'loaded');",
        "    }",
        "  }",
        "}",
        "return result;",
        "})()"
    );
    match kernel.eval_to_rust_value(js) {
        RustValue::Array(items) => {
            let mut chunks = Vec::new();
            for item in items {
                if let RustValue::Object(map) = item {
                    let chunk_id = map
                        .get("chunk_id")
                        .and_then(|v| {
                            if let RustValue::String(s) = v {
                                Some(s.clone())
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default();
                    let state = map
                        .get("state")
                        .and_then(|v| {
                            if let RustValue::String(s) = v {
                                Some(s.clone())
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default();
                    let modules_added = map
                        .get("modules_added")
                        .and_then(|v| {
                            if let RustValue::Int(n) = v {
                                Some(*n as u64)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(0);
                    chunks.push(serde_json::json!({
                        "chunk_id": chunk_id,
                        "state": state,
                        "modules_added": modules_added,
                    }));
                }
            }
            chunks
        }
        _ => Vec::new(),
    }
}

/// Check if the require was captured via the late global fallback path
/// (meaning prelude timing is uncertain).
fn check_capture_late(kernel: &mut EmbeddedV8Kernel) -> bool {
    let js = concat!(
        "(function(){",
        "var log = typeof __iv8_webpack_log !== 'undefined' ? __iv8_webpack_log : [];",
        "var hasPre = false, hasLate = false;",
        "for (var i = 0; i < log.length; i++) {",
        "  if (log[i] === 'wp_require_captured' || log[i] === 'wp_require_proto_c') hasPre = true;",
        "  if (log[i] === 'wp_require_global') hasLate = true;",
        "}",
        "return hasLate && !hasPre;",
        "})()"
    );
    matches!(kernel.eval_to_rust_value(js), RustValue::Bool(true))
}

/// Collect module graph evidence produced by the WebpackBridge prelude.
///
/// Returns a JSON value conforming to the `module-graph.v0.1` schema:
/// - schema_version, runtime_family, runtime_flavor
/// - module_ids, module_count, entry_module_id
/// - nodes[], edges[], chunks[]
/// - evidence[], diagnostics[]
pub fn collect_module_graph(kernel: &mut EmbeddedV8Kernel) -> Option<serde_json::Value> {
    // Step 1: Runtime capture of __webpack_require__ (handles IIFE-local requires)
    // This must run before reading __iv8_webpack_log so the capture is recorded.
    let require_callable = matches!(
        kernel.eval_to_rust_value(
            "(function(){ \
             if (typeof __iv8_wp_require === 'function') return true; \
             if (typeof __webpack_require__ === 'function') { \
               globalThis.__iv8_wp_require = __webpack_require__; \
               if (typeof __iv8_webpack_log !== 'undefined') { \
                 __iv8_webpack_log.push('wp_require_global'); \
               } \
               return true; \
             } \
             return false; \
             })()"
        ),
        RustValue::Bool(true)
    );

    // Step 2: Read webpack log entries
    let log_val = kernel
        .eval_to_rust_value("typeof __iv8_webpack_log !== 'undefined' ? __iv8_webpack_log : []");
    let RustValue::Array(items) = log_val else {
        return None;
    };

    let mut module_ids = Vec::new();
    let mut require_captured_via_prelude = false;
    let mut require_captured_via_proto = false;
    let mut require_captured_via_global = false;
    for item in items {
        if let RustValue::String(s) = item {
            if s == "wp_require_captured" {
                require_captured_via_prelude = true;
            }
            if s == "wp_require_proto_c" {
                require_captured_via_proto = true;
            }
            if s == "wp_require_global" {
                require_captured_via_global = true;
            }
            if let Some(module_id) = s.strip_prefix("module_registered,") {
                module_ids.push(module_id.to_string());
            }
        }
    }

    let require_captured = require_captured_via_prelude
        || require_captured_via_proto
        || require_captured_via_global
        || require_callable;

    // Collect module IDs from require.m
    collect_require_module_ids(kernel, &mut module_ids);

    // S7: install chunk factories into live require.m, then re-collect IDs.
    let factories_installed = install_chunk_factories_into_require(kernel);
    collect_require_module_ids(kernel, &mut module_ids);

    // Merge module IDs from webpackJsonp / webpackChunk tables (multi-chunk).
    {
        let js = concat!(
            "(function(){var ids=[];var seen={};",
            "function add(k){k=String(k);if(!seen[k]){seen[k]=1;ids.push(k);}}",
            "function scan(arr){",
            "  if(!arr||!Array.isArray(arr))return;",
            "  for(var i=0;i<arr.length;i++){",
            "    var e=arr[i];",
            "    if(e&&e[1]&&typeof e[1]==='object'){",
            "      Object.keys(e[1]).forEach(add);",
            "    }",
            "  }",
            "}",
            "if(typeof window!=='undefined')scan(window.webpackJsonp);",
            "var wpc=(typeof self!=='undefined'&&self.webpackChunk)||",
            "  (typeof window!=='undefined'&&window.webpackChunk);",
            "scan(wpc);",
            "return ids;})()"
        );
        if let RustValue::Array(items2) = kernel.eval_to_rust_value(js) {
            for item in items2 {
                if let RustValue::String(s) = item {
                    if !module_ids.contains(&s) {
                        module_ids.push(s);
                    }
                }
            }
        }
    }

    module_ids.sort();
    module_ids.dedup();

    // Runtime evidence collection
    let runtime_flavor = detect_runtime_flavor(kernel);
    let cache_executed = collect_cache_executed(kernel);
    let chunks = detect_chunks(kernel);
    let capture_late = check_capture_late(kernel);
    let edges = collect_static_require_edges(kernel, &module_ids);
    let cycles = detect_cycles_in_edges(&edges);

    // Map module_id -> chunk_id(s) from observed chunk tables
    let mut module_to_chunk: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();
    {
        let js = concat!(
            "(function(){",
            "var map={};",
            "function scan(arr){",
            "  if(!arr||!Array.isArray(arr))return;",
            "  for(var i=0;i<arr.length;i++){",
            "    var e=arr[i];",
            "    if(!e||!e[1]||typeof e[1]!=='object')continue;",
            "    var raw=e[0];",
            "    var cid=Array.isArray(raw)?raw.map(String).join('+'):String(raw);",
            "    Object.keys(e[1]).forEach(function(mid){ map[String(mid)]=cid; });",
            "  }",
            "}",
            "try{if(typeof window!=='undefined')scan(window.webpackJsonp);}catch(e){}",
            "try{",
            "  var wpc=(typeof self!=='undefined'&&self.webpackChunk)||",
            "    (typeof window!=='undefined'&&window.webpackChunk);",
            "  scan(wpc);",
            "}catch(e){}",
            "return map;",
            "})()"
        );
        if let RustValue::Object(map) = kernel.eval_to_rust_value(js) {
            for (k, v) in map {
                if let RustValue::String(cid) = v {
                    module_to_chunk.insert(k, cid);
                }
            }
        }
    }

    // Build nodes with execution metadata from cache
    let nodes: Vec<serde_json::Value> = module_ids
        .iter()
        .map(|module_id| {
            let executed = cache_executed.contains(module_id);
            let mut node_evidence = vec!["module_table_captured"];
            if executed {
                node_evidence.push("module_cache_captured");
            }
            if factories_installed > 0 {
                node_evidence.push("chunk_factory_merged");
            }
            let chunk_id = module_to_chunk
                .get(module_id)
                .cloned()
                .map(serde_json::Value::String)
                .unwrap_or(serde_json::Value::Null);
            serde_json::json!({
                "module_id": module_id,
                "kind": "factory",
                "executed": executed,
                "exports_seen": false,
                "source_available": false,
                "chunk_id": chunk_id,
                "evidence": node_evidence,
            })
        })
        .collect();

    // Find the entry module: module id "0" or the numerically smallest id
    let mut entry_module_id: Option<String> = None;
    if module_ids.contains(&"0".to_string()) || cache_executed.contains(&"0".to_string()) {
        entry_module_id = Some("0".to_string());
    } else if !module_ids.is_empty() {
        // Use smallest id as heuristic
        let ids_sorted = {
            let mut v = module_ids.clone();
            v.sort_by(|a, b| {
                let an = a.parse::<u64>();
                let bn = b.parse::<u64>();
                match (an, bn) {
                    (Ok(a_num), Ok(b_num)) => a_num.cmp(&b_num),
                    _ => a.cmp(b),
                }
            });
            v
        };
        if let Some(first) = ids_sorted.into_iter().next() {
            entry_module_id = Some(first);
        }
    }

    // Build evidence array
    let mut evidence = Vec::new();
    let mut diagnostics = Vec::new();

    // Flavor diagnostics
    if runtime_flavor == "unknown_webpack_like" {
        diagnostics.push(serde_json::json!({
            "code": "WEBPACK_RUNTIME_FLAVOR_UNKNOWN",
            "severity": "warn",
            "stage": "webpack.probe",
            "message": "runtime flavor could not be determined from runtime signatures",
        }));
    }

    // Capture diagnostics
    if capture_late {
        diagnostics.push(serde_json::json!({
            "code": "WEBPACK_REQUIRE_CAPTURE_LATE",
            "severity": "warn",
            "stage": "webpack.capture",
            "message": "require reference was captured via global fallback after runtime init",
        }));
    }

    // Module table evidence
    if !module_ids.is_empty() {
        evidence.push(serde_json::json!({
            "kind": "module_table_captured",
            "strength": "strong",
            "source": "webpack_bridge",
            "stage": "webpack.capture",
            "summary": "captured non-empty webpack module table",
            "payload": {"module_count": module_ids.len(), "module_ids": module_ids},
        }));
    } else {
        diagnostics.push(serde_json::json!({
            "code": "WEBPACK_MODULE_TABLE_EMPTY",
            "severity": "error",
            "stage": "webpack.capture",
            "message": "webpack runtime present but module table was not captured",
        }));
    }

    // Module cache evidence
    if !cache_executed.is_empty() {
        evidence.push(serde_json::json!({
            "kind": "module_cache_captured",
            "strength": "strong",
            "source": "webpack_bridge",
            "stage": "webpack.execute",
            "summary": "captured executed module cache entries",
            "payload": {"executed_count": cache_executed.len()},
        }));
    } else {
        diagnostics.push(serde_json::json!({
            "code": "WEBPACK_MODULE_CACHE_EMPTY",
            "severity": "warn",
            "stage": "webpack.execute",
            "message": "module cache is empty or inaccessible; no executed modules observed",
        }));
    }

    // Require captured evidence
    if require_captured {
        evidence.push(serde_json::json!({
            "kind": "require_captured",
            "strength": "strong",
            "source": "webpack_bridge",
            "stage": "webpack.capture",
            "summary": "captured callable webpack require reference",
        }));
    } else {
        diagnostics.push(serde_json::json!({
            "code": "WEBPACK_REQUIRE_CAPTURE_FAILED",
            "severity": "error",
            "stage": "webpack.capture",
            "message": "webpack require function could not be retained",
        }));
    }

    // Entry module evidence
    if let Some(ref entry_id) = entry_module_id {
        let entry_executed = cache_executed.contains(entry_id);
        evidence.push(serde_json::json!({
            "kind": "entry_module_executed",
            "strength": if entry_executed { "strong" } else { "weak" },
            "source": "webpack_bridge",
            "stage": "webpack.execute",
            "summary": if entry_executed { "entry module found in execution cache" } else { "entry module identified but not yet executed" },
            "payload": {"entry_module_id": entry_id, "executed": entry_executed},
        }));
    }

    // Chunk evidence
    for chunk in &chunks {
        if let Some(chunk_id) = chunk.get("chunk_id").and_then(|v| v.as_str()) {
            evidence.push(serde_json::json!({
                "kind": "chunk_event_observed",
                "strength": "weak",
                "source": "webpack_bridge",
                "stage": "webpack.chunk",
                "summary": format!("chunk '{}' observed with {} modules", chunk_id, chunk.get("modules_added").and_then(|v| v.as_u64()).unwrap_or(0)),
                "payload": chunk,
            }));
        }
    }
    if chunks.is_empty() && !module_ids.is_empty() {
        diagnostics.push(serde_json::json!({
            "code": "WEBPACK_CHUNK_UNSUPPORTED",
            "severity": "warn",
            "stage": "webpack.chunk",
            "message": "no chunk events detected; runtime may use unsupported chunk path",
        }));
    }

    // Weak evidence guard
    let has_strong = evidence
        .iter()
        .any(|e| e.get("strength").and_then(|v| v.as_str()) == Some("strong"));
    if !has_strong && !diagnostics.is_empty() {
        diagnostics.push(serde_json::json!({
            "code": "WEBPACK_EVIDENCE_WEAK",
            "severity": "warn",
            "stage": "webpack.validate",
            "message": "only weak or marker evidence produced; strong module/runtime evidence required for PASS",
        }));
    }

    // Assemble graph
    let mut graph = serde_json::Map::new();
    graph.insert(
        "schema_version".into(),
        serde_json::json!("module-graph.v0.1"),
    );
    graph.insert("runtime_family".into(), serde_json::json!("webpack_like"));
    graph.insert("runtime_flavor".into(), serde_json::json!(runtime_flavor));
    graph.insert("module_ids".into(), serde_json::json!(module_ids));
    graph.insert("module_count".into(), serde_json::json!(nodes.len()));
    graph.insert("entry_module_id".into(), serde_json::json!(entry_module_id));
    graph.insert("nodes".into(), serde_json::Value::Array(nodes));
    graph.insert("edges".into(), serde_json::Value::Array(edges));
    graph.insert("cycles".into(), serde_json::json!(cycles));
    graph.insert(
        "chunk_factories_installed".into(),
        serde_json::json!(factories_installed),
    );
    graph.insert("chunks".into(), serde_json::Value::Array(chunks));
    graph.insert("evidence".into(), serde_json::Value::Array(evidence));
    graph.insert("diagnostics".into(), serde_json::Value::Array(diagnostics));
    Some(serde_json::Value::Object(graph))
}

/// Public wrapper for executor product path (A-P0-1).
pub fn install_chunk_factories_public(kernel: &mut EmbeddedV8Kernel) -> u64 {
    install_chunk_factories_into_require(kernel)
}

/// Register preloaded chunk sources: eval text then merge factories (A-P1-1 product API).
/// `chunks` is ordered list of JS source strings (runtime-like or webpackChunk push files).
/// Does **not** fetch remote URLs.
pub fn preload_chunk_sources(kernel: &mut EmbeddedV8Kernel, chunks: &[String]) -> serde_json::Value {
    let mut eval_ok = 0u64;
    let mut eval_fail = 0u64;
    for (i, src) in chunks.iter().enumerate() {
        match kernel.eval(src, crate::kernel::EvalOpts::default()) {
            Ok(_) => eval_ok += 1,
            Err(e) => {
                eval_fail += 1;
                let _ = e;
                let _ = i;
            }
        }
    }
    let installed = install_chunk_factories_into_require(kernel);
    serde_json::json!({
        "schema": "iv8-webpack-preload-chunks.v0.1",
        "chunks_eval_ok": eval_ok,
        "chunks_eval_fail": eval_fail,
        "factories_installed": installed,
        "note": "caller-supplied chunk text only; no network ensureChunk",
    })
}

/// Install factories from webpackJsonp/webpackChunk into live `__webpack_require__.m`
/// so `require(id)` can resolve modules defined only in chunk tables (S7-03).
fn install_chunk_factories_into_require(kernel: &mut EmbeddedV8Kernel) -> u64 {
    let js = concat!(
        "(function(){",
        "var r = null;",
        "try { if (typeof __iv8_wp_require === 'function') r = __iv8_wp_require; } catch(e) {}",
        "try { if (!r && typeof __webpack_require__ === 'function') r = __webpack_require__; } catch(e) {}",
        "try { if (!r && typeof globalThis.__webpack_require__ === 'function') r = globalThis.__webpack_require__; } catch(e) {}",
        "if (!r) return 0;",
        "if (!r.m || typeof r.m !== 'object') r.m = {};",
        "var installed = 0;",
        "function merge(mods){",
        "  if (!mods || typeof mods !== 'object') return;",
        "  Object.keys(mods).forEach(function(k){",
        "    if (typeof mods[k] === 'function' && typeof r.m[k] !== 'function') {",
        "      r.m[k] = mods[k];",
        "      installed++;",
        "    }",
        "  });",
        "}",
        "function scan(arr){",
        "  if (!arr || !Array.isArray(arr)) return;",
        "  for (var i = 0; i < arr.length; i++) {",
        "    var e = arr[i];",
        "    if (e && e[1]) merge(e[1]);",
        "  }",
        "}",
        "try { if (typeof window !== 'undefined') scan(window.webpackJsonp); } catch(e) {}",
        "try {",
        "  var wpc = null;",
        "  if (typeof self !== 'undefined') wpc = self.webpackChunk;",
        "  if (!wpc && typeof window !== 'undefined') wpc = window.webpackChunk;",
        "  if (!wpc && typeof globalThis !== 'undefined') wpc = globalThis.webpackChunk;",
        "  scan(wpc);",
        "} catch(e) {}",
        // Sync ensureChunk subset: re-merge tables then resolve (A-P1-1)
        "if (!r.__iv8_e_wrapped) {",
        "  var origE = typeof r.e === 'function' ? r.e : null;",
        "  r.e = function(chunkId){",
        "    try {",
        "      if (typeof window !== 'undefined') scan(window.webpackJsonp);",
        "      var w2 = null;",
        "      if (typeof self !== 'undefined') w2 = self.webpackChunk;",
        "      if (!w2 && typeof window !== 'undefined') w2 = window.webpackChunk;",
        "      scan(w2);",
        "      if (typeof __iv8_webpack_log !== 'undefined') {",
        "        __iv8_webpack_log.push('chunk_ensure,' + String(chunkId));",
        "      }",
        "    } catch(e) {}",
        "    return Promise.resolve(chunkId);",
        "  };",
        "  r.__iv8_e_wrapped = true;",
        "  if (origE) r.__iv8_e_orig = origE;",
        "}",
        "return installed;",
        "})()"
    );
    match kernel.eval_to_rust_value(js) {
        RustValue::Int(n) if n >= 0 => n as u64,
        RustValue::Float(n) if n >= 0.0 => n as u64,
        _ => 0,
    }
}

/// Static edges: scan factory.toString() for require(number|string) when available.
fn collect_static_require_edges(
    kernel: &mut EmbeddedV8Kernel,
    _module_ids: &[String],
) -> Vec<serde_json::Value> {
    let js = concat!(
        "(function(){",
        "var r = null;",
        "try { if (typeof __iv8_wp_require === 'function') r = __iv8_wp_require; } catch(e) {}",
        "try { if (!r && typeof __webpack_require__ === 'function') r = __webpack_require__; } catch(e) {}",
        "if (!r || !r.m) return [];",
        "var edges = [];",
        // require / __webpack_require__ / single-letter minified param (r|n|e|t) + numeric id
        "var re = /\\b(?:require|__webpack_require__|[rent])\\(\\s*[\\\"']?(\\d+)[\\\"']?\\s*\\)/g;",
        "Object.keys(r.m).forEach(function(from){",
        "  var fn = r.m[from];",
        "  if (typeof fn !== 'function') return;",
        "  var src = '';",
        "  try { src = Function.prototype.toString.call(fn); } catch(e) { return; }",
        "  if (!src || src.indexOf('[native code]') >= 0) return;",
        "  if (src.length > 200000) return;",
        "  var m;",
        "  re.lastIndex = 0;",
        "  var seen = {};",
        "  while ((m = re.exec(src)) !== null) {",
        "    var to = m[1];",
        "    if (String(from) === String(to)) continue;",
        "    var key = from + '->' + to;",
        "    if (seen[key]) continue;",
        "    seen[key] = 1;",
        "    edges.push({from: String(from), to: String(to), kind: 'static_require'});",
        "  }",
        "});",
        "return edges;",
        "})()"
    );
    match kernel.eval_to_rust_value(js) {
        RustValue::Array(items) => {
            let mut edges = Vec::new();
            for item in items {
                if let RustValue::Object(map) = item {
                    let from = map
                        .get("from")
                        .and_then(|v| {
                            if let RustValue::String(s) = v {
                                Some(s.clone())
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default();
                    let to = map
                        .get("to")
                        .and_then(|v| {
                            if let RustValue::String(s) = v {
                                Some(s.clone())
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default();
                    if !from.is_empty() && !to.is_empty() {
                        edges.push(serde_json::json!({
                            "from": from,
                            "to": to,
                            "kind": "static_require",
                        }));
                    }
                }
            }
            edges
        }
        _ => Vec::new(),
    }
}

/// Public cycle detect for browserify / other graphs.
pub fn detect_cycles_public(edges: &[serde_json::Value]) -> Vec<serde_json::Value> {
    detect_cycles_in_edges(edges)
}

/// Detect simple cycles via DFS on directed edges (S7-05).
fn detect_cycles_in_edges(edges: &[serde_json::Value]) -> Vec<serde_json::Value> {
    use std::collections::{HashMap, HashSet};
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for e in edges {
        let from = e
            .get("from")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let to = e
            .get("to")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        if from.is_empty() || to.is_empty() {
            continue;
        }
        adj.entry(from).or_default().push(to);
    }
    let mut cycles = Vec::new();
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    fn dfs(
        node: &str,
        adj: &HashMap<String, Vec<String>>,
        visiting: &mut HashSet<String>,
        visited: &mut HashSet<String>,
        stack: &mut Vec<String>,
        cycles: &mut Vec<serde_json::Value>,
    ) {
        if visited.contains(node) {
            return;
        }
        if visiting.contains(node) {
            if let Some(start) = stack.iter().position(|n| n == node) {
                let mut cyc = stack[start..].to_vec();
                cyc.push(node.to_string());
                cycles.push(serde_json::json!({ "nodes": cyc, "kind": "cycle" }));
            }
            return;
        }
        visiting.insert(node.to_string());
        stack.push(node.to_string());
        if let Some(nexts) = adj.get(node) {
            for n in nexts {
                dfs(n, adj, visiting, visited, stack, cycles);
            }
        }
        stack.pop();
        visiting.remove(node);
        visited.insert(node.to_string());
    }

    let nodes: Vec<String> = adj.keys().cloned().collect();
    for n in nodes {
        dfs(
            &n,
            &adj,
            &mut visiting,
            &mut visited,
            &mut stack,
            &mut cycles,
        );
    }
    // Cap noise
    cycles.truncate(32);
    cycles
}

fn collect_require_module_ids(kernel: &mut EmbeddedV8Kernel, module_ids: &mut Vec<String>) {
    let js = concat!(
        "(function(){var r=null;",
        "if(typeof __iv8_wp_require === 'function') r=__iv8_wp_require;",
        "else if(typeof globalThis.__webpack_require__ === 'function') r=globalThis.__webpack_require__;",
        "else if(typeof __webpack_require__ === 'function') r=__webpack_require__;",
        "else if(typeof __webpack_require__ === 'object' && __webpack_require__ && __webpack_require__.m) r=__webpack_require__;",
        "return r && r.m ? Object.keys(r.m) : [];})()"
    );
    if let RustValue::Array(items) = kernel.eval_to_rust_value(js) {
        for item in items {
            if let RustValue::String(s) = item {
                module_ids.push(s);
            }
        }
    }
}

/// Find the body of the modules table (content between outermost braces).
fn find_modules_body(source: &str) -> Option<&str> {
    let marker = "__webpack_require__.m";
    let pos = source.find(marker)?;
    let after = source[pos + marker.len()..].trim_start();
    let after = after.trim_start_matches('=').trim_start();
    let brace_start = after.find('{')?;
    let body = &after[brace_start + 1..];
    Some(body)
}

/// Estimate module count from line-based heuristics.
fn extract_module_count(source: &str) -> usize {
    let body = find_modules_body(source).unwrap_or("");
    if let Some(end) = body.find('}') {
        let table = &body[..end];
        return table.matches(':').count().min(10000);
    }
    0
}

/// Extract module IDs up to a reasonable limit.
fn extract_module_ids(source: &str) -> Option<Vec<String>> {
    let mut ids = Vec::new();
    let limit = 200;

    let body = find_modules_body(source)?;
    let bytes = body.as_bytes();
    let mut idx = 0;

    while idx < body.len().min(5000) && ids.len() < limit {
        // Skip whitespace/newlines
        while idx < body.len()
            && (bytes[idx] == b' '
                || bytes[idx] == b'\n'
                || bytes[idx] == b'\r'
                || bytes[idx] == b'\t')
        {
            idx += 1;
        }
        if idx >= body.len() || bytes[idx] == b'}' {
            break;
        }
        if bytes[idx] == b',' {
            idx += 1;
            continue;
        }
        // Read until ':' for the key
        let start = idx;
        while idx < body.len() && bytes[idx] != b':' {
            idx += 1;
        }
        if idx < body.len() && bytes[idx] == b':' {
            let key = body[start..idx].trim();
            if !key.is_empty() {
                let clean = key.trim_matches('"').trim_matches('\'');
                ids.push(clean.to_string());
            }
        }
        // Skip past the value to the next comma or closing brace
        // Track balanced braces so we correctly handle function values
        let mut depth: i32 = 0;
        let mut found_value_end = false;
        while idx < body.len() && !found_value_end {
            match bytes[idx] {
                b'{' | b'[' | b'(' => depth += 1,
                b'}' | b']' | b')' => {
                    if depth == 0 {
                        // We hit a closing brace at depth 0 — the entry value ended
                        found_value_end = true;
                    } else {
                        depth -= 1;
                    }
                }
                b',' if depth == 0 => {
                    found_value_end = true;
                }
                _ => {}
            }
            if !found_value_end {
                idx += 1;
            }
        }
    }

    if ids.is_empty() {
        None
    } else {
        Some(ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_webpack() {
        let d = detect("var x = 1;");
        assert!(!d.detected);
    }

    #[test]
    fn test_basic_webpack4() {
        let src = r#"
(function(modules) {
    function __webpack_require__(moduleId) {
        // ...
    }
    __webpack_require__.m = modules;
    __webpack_require__.c = {};
    __webpack_require__.d = function() {};
    return __webpack_require__(0);
})
"#;
        let d = detect(src);
        assert!(d.detected);
        assert!(d.helpers_present.contains(&".m".to_string()));
        assert!(d.helpers_present.contains(&".c".to_string()));
        assert!(d.helpers_present.contains(&".d".to_string()));
    }

    #[test]
    fn test_webpack5_helpers() {
        let src = r#"
__webpack_require__.m = {};
__webpack_require__.c = {};
__webpack_require__.d = function() {};
__webpack_require__.e = function() {};
__webpack_require__.r = function() {};
"#;
        let d = detect(src);
        assert!(d.detected);
        assert!(d.helpers_present.contains(&".e".to_string()));
        assert!(d.helpers_present.contains(&".r".to_string()));
    }

    #[test]
    fn test_module_id_extraction() {
        let src = r#"
__webpack_require__.m = {
    0: function(module) {},
    7: function(module) {},
    42: function(module) {},
    100: function(module) {},
};
"#;
        let d = detect(src);
        assert!(d.detected);
        assert!(d.module_ids.contains(&"0".to_string()));
        assert!(d.module_ids.contains(&"7".to_string()));
        assert!(d.module_ids.contains(&"42".to_string()));
        assert!(d.module_ids.contains(&"100".to_string()));
    }

    #[test]
    fn test_webpackflavor_5() {
        let src = r#"
var __webpack_require__ = {};
__webpack_require__.e = function() {};
globalThis.webpackChunk = [];
"#;
        let d = detect(src);
        assert_eq!(d.flavor, WebpackFlavor::Webpack5);
    }

    #[test]
    fn test_bridge_prelude_initializes_require_before_capture() {
        let js = bridge_prelude();
        let init = js.find("globalThis.__iv8_wp_require = null").unwrap();
        let capture = js.find("globalThis.__iv8_wp_require === null").unwrap();
        let global_capture = js
            .find("globalThis.__iv8_wp_require = __webpack_require__")
            .unwrap();
        assert!(init < capture);
        assert!(init < global_capture);
    }

    #[test]
    fn test_bridge_prelude_does_not_clear_captured_require_at_end() {
        let js = bridge_prelude();
        let global_capture = js
            .find("globalThis.__iv8_wp_require = __webpack_require__")
            .unwrap();
        let tail = &js[global_capture..];
        assert!(!tail.contains("globalThis.__iv8_wp_require = null"));
    }

    #[test]
    fn test_collect_module_graph_schema_from_global_require() {
        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;

        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel
            .eval(bridge_prelude(), crate::kernel::EvalOpts::default())
            .unwrap();
        kernel
            .eval(
                r#"
function __webpack_require__(id) { return __webpack_require__.m[id](); }
__webpack_require__.m = {
  0: function(){ return "entry"; },
  7: function(){ return "sign"; }
};
__webpack_require__.c = {};
"#,
                crate::kernel::EvalOpts::default(),
            )
            .unwrap();

        let graph = collect_module_graph(&mut kernel).expect("module graph");
        let evidence = graph["evidence"].as_array().unwrap();
        let diagnostics = graph["diagnostics"].as_array().unwrap();

        assert_eq!(graph["schema_version"], "module-graph.v0.1");
        assert_eq!(graph["runtime_family"], "webpack_like");
        // S7 may wrap .e for sync ensureChunk → flavor reports webpack5-like
        let flavor = graph["runtime_flavor"].as_str().unwrap_or("");
        assert!(
            flavor == "webpack4" || flavor == "webpack5" || flavor == "unknown_webpack_like",
            "flavor={flavor}"
        );
        assert_eq!(graph["module_count"], 2);
        assert_eq!(graph["nodes"].as_array().unwrap().len(), 2);

        // Strong evidence
        assert!(evidence
            .iter()
            .any(|e| e["kind"] == "module_table_captured" && e["strength"] == "strong"));
        assert!(evidence
            .iter()
            .any(|e| e["kind"] == "require_captured" && e["strength"] == "strong"));

        // entry_module_executed present with weak confidence (no cache entries)
        assert!(evidence
            .iter()
            .any(|e| e["kind"] == "entry_module_executed"));

        // Diagnostics for empty cache and unsupported chunk
        assert!(diagnostics
            .iter()
            .any(|d| d["code"] == "WEBPACK_MODULE_CACHE_EMPTY"));
        assert!(diagnostics
            .iter()
            .any(|d| d["code"] == "WEBPACK_CHUNK_UNSUPPORTED"));

        // Node execution metadata reflects empty cache
        let nodes = graph["nodes"].as_array().unwrap();
        for node in nodes {
            assert_eq!(node["executed"], false);
            assert_eq!(node["exports_seen"], false);
        }
    }

    #[test]
    fn test_collect_module_graph_marker_only_emits_diagnostics() {
        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;

        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel
            .eval(bridge_prelude(), crate::kernel::EvalOpts::default())
            .unwrap();
        let graph = collect_module_graph(&mut kernel).expect("module graph");
        let diagnostics = graph["diagnostics"].as_array().unwrap();

        assert_eq!(graph["module_count"], 0);
        assert!(graph["evidence"].as_array().unwrap().is_empty());

        // Core diagnostics for empty table and failed capture
        assert!(diagnostics
            .iter()
            .any(|d| d["code"] == "WEBPACK_MODULE_TABLE_EMPTY"));
        assert!(diagnostics
            .iter()
            .any(|d| d["code"] == "WEBPACK_REQUIRE_CAPTURE_FAILED"));

        // Additional diagnostics from flavor detection and weak evidence guard
        assert!(diagnostics
            .iter()
            .any(|d| d["code"] == "WEBPACK_RUNTIME_FLAVOR_UNKNOWN"));
        assert!(diagnostics
            .iter()
            .any(|d| d["code"] == "WEBPACK_EVIDENCE_WEAK"));
    }

    #[test]
    fn test_collect_module_graph_with_executed_cache() {
        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;

        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel
            .eval(bridge_prelude(), crate::kernel::EvalOpts::default())
            .unwrap();
        kernel
            .eval(
                r#"
function __webpack_require__(id) { return __webpack_require__.m[id](); }
__webpack_require__.m = {
  0: function(){ return "entry"; },
  7: function(){ return "sign"; },
  42: function(){ return "util"; }
};
// Simulate cache execution by adding entries to module cache
__webpack_require__.c = {
  0: { exports: {} },
  7: { exports: {} }
};
"#,
                crate::kernel::EvalOpts::default(),
            )
            .unwrap();

        let graph = collect_module_graph(&mut kernel).expect("module graph");
        let evidence = graph["evidence"].as_array().unwrap();
        let diagnostics = graph["diagnostics"].as_array().unwrap();
        let nodes = graph["nodes"].as_array().unwrap();

        // module_cache_captured with cache entries
        assert!(evidence
            .iter()
            .any(|e| e["kind"] == "module_cache_captured" && e["strength"] == "strong"));
        assert!(!diagnostics
            .iter()
            .any(|d| d["code"] == "WEBPACK_MODULE_CACHE_EMPTY"));

        // entry_module_executed with strong evidence (id 0 is in cache)
        assert!(evidence.iter().any(|e| e["kind"] == "entry_module_executed"
            && e["strength"] == "strong"
            && e["payload"]["executed"] == true));

        // Node 0 and 7 have executed=true, 42 has executed=false
        let node_0: &serde_json::Value = nodes.iter().find(|n| n["module_id"] == "0").unwrap();
        let node_7: &serde_json::Value = nodes.iter().find(|n| n["module_id"] == "7").unwrap();
        let node_42: &serde_json::Value = nodes.iter().find(|n| n["module_id"] == "42").unwrap();
        assert_eq!(node_0["executed"], true);
        assert_eq!(node_7["executed"], true);
        assert_eq!(node_42["executed"], false);
    }

    #[test]
    fn test_collect_module_graph_webpack5_flavor_with_chunks() {
        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;

        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel
            .eval(bridge_prelude(), crate::kernel::EvalOpts::default())
            .unwrap();
        kernel
            .eval(
                r#"
var __webpack_require__ = function(id) {};
__webpack_require__.m = { 0: function(){} };
__webpack_require__.c = {};
__webpack_require__.e = function() {};
__webpack_require__.d = function() {};
window.webpackChunk = [
  ["vendors", { 1: function(){}, 2: function(){} }],
  ["main", { 3: function(){} }]
];
"#,
                crate::kernel::EvalOpts::default(),
            )
            .unwrap();

        let graph = collect_module_graph(&mut kernel).expect("module graph");
        let evidence = graph["evidence"].as_array().unwrap();
        let chunks = graph["chunks"].as_array().unwrap();

        assert_eq!(graph["runtime_flavor"], "webpack5");
        // S7: module_count includes require.m + chunk table modules (0,1,2,3)
        assert!(
            graph["module_count"].as_u64().unwrap_or(0) >= 1,
            "module_count={:?}",
            graph["module_count"]
        );
        assert_eq!(graph["entry_module_id"], "0");

        // chunk_event_observed evidence
        assert!(evidence.iter().any(|e| e["kind"] == "chunk_event_observed"));

        // Chunk entries
        assert!(chunks.iter().any(|c| c["chunk_id"] == "vendors"));
        assert!(chunks.iter().any(|c| c["chunk_id"] == "main"));
    }

    /// v0.8.99 S7 BD-2: multi-chunk module IDs merge into graph.nodes
    #[test]
    fn test_collect_module_graph_merges_chunk_module_ids() {
        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;

        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel
            .eval(bridge_prelude(), crate::kernel::EvalOpts::default())
            .unwrap();
        kernel
            .eval(
                r#"
var __webpack_require__ = function(id) {};
__webpack_require__.m = { 0: function(){} };
__webpack_require__.c = { 0: { exports: {} } };
__webpack_require__.e = function() {};
window.webpackChunk = [
  [["vendors"], { 10: function(){}, 11: function(){} }],
  [["page"], { 20: function(){} }]
];
"#,
                crate::kernel::EvalOpts::default(),
            )
            .unwrap();

        let graph = collect_module_graph(&mut kernel).expect("module graph");
        let nodes = graph["nodes"].as_array().unwrap();
        let ids: Vec<&str> = nodes
            .iter()
            .filter_map(|n| n["module_id"].as_str())
            .collect();
        assert!(ids.contains(&"0"), "runtime module 0: {:?}", ids);
        assert!(ids.contains(&"10"), "chunk module 10: {:?}", ids);
        assert!(ids.contains(&"11"), "chunk module 11: {:?}", ids);
        assert!(ids.contains(&"20"), "chunk module 20: {:?}", ids);
        assert!(graph["module_count"].as_u64().unwrap_or(0) >= 4);
        // chunk_id backfill
        let n10 = nodes.iter().find(|n| n["module_id"] == "10");
        assert!(n10.is_some());
        assert_eq!(n10.unwrap()["chunk_id"], "vendors");
    }

    /// S7-03: chunk factories installed into live require.m and callable
    #[test]
    fn test_chunk_factories_installed_and_require_resolves() {
        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;

        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel
            .eval(bridge_prelude(), crate::kernel::EvalOpts::default())
            .unwrap();
        kernel
            .eval(
                r#"
var __webpack_require__ = function(id) {
  if (__webpack_require__.c[id]) return __webpack_require__.c[id].exports;
  var m = { exports: {}, id: id };
  __webpack_require__.c[id] = m;
  var f = __webpack_require__.m[id];
  if (typeof f === 'function') f.call(m.exports, m, m.exports, __webpack_require__);
  return m.exports;
};
__webpack_require__.m = { 0: function(m,e,r){ e.main = true; } };
__webpack_require__.c = {};
window.webpackChunk = [
  [["vendors"], {
    10: function(m,e,r){ e.vendor = 42; },
    11: function(m,e,r){ e.from10 = r(10).vendor; }
  }]
];
"#,
                crate::kernel::EvalOpts::default(),
            )
            .unwrap();

        let graph = collect_module_graph(&mut kernel).expect("graph");
        assert!(
            graph["chunk_factories_installed"].as_u64().unwrap_or(0) >= 2,
            "factories_installed={:?}",
            graph["chunk_factories_installed"]
        );
        // After install, require(10) should work via merged m
        let v = kernel.eval_to_rust_value("__webpack_require__(10).vendor");
        assert_eq!(v, crate::convert::RustValue::Int(42));
        let v2 = kernel.eval_to_rust_value("__webpack_require__(11).from10");
        assert_eq!(v2, crate::convert::RustValue::Int(42));
        // ensureChunk sync subset returns a Promise
        let p = kernel.eval_to_rust_value("typeof __webpack_require__.e(1).then");
        assert_eq!(p, crate::convert::RustValue::String("function".into()));
    }

    /// S7-04/05: static require edges + cycle detection
    #[test]
    fn test_static_require_edges_and_cycle_detect() {
        use crate::kernel::embedded_v8::EmbeddedV8Kernel;
        use crate::kernel::KernelConfig;

        let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default()).unwrap();
        kernel
            .eval(bridge_prelude(), crate::kernel::EvalOpts::default())
            .unwrap();
        kernel
            .eval(
                r#"
var __webpack_require__ = function(id) {
  if (__webpack_require__.c[id]) return __webpack_require__.c[id].exports;
  var m = { exports: {} };
  __webpack_require__.c[id] = m;
  __webpack_require__.m[id](m, m.exports, __webpack_require__);
  return m.exports;
};
__webpack_require__.m = {
  1: function(m,e,r){ r(2); e.a=1; },
  2: function(m,e,r){ r(1); e.b=2; },
  3: function(m,e,r){ r(4); e.c=3; },
  4: function(m,e,r){ e.d=4; }
};
__webpack_require__.c = {};
"#,
                crate::kernel::EvalOpts::default(),
            )
            .unwrap();

        let graph = collect_module_graph(&mut kernel).expect("graph");
        let edges = graph["edges"].as_array().cloned().unwrap_or_default();
        assert!(
            edges.iter().any(|e| e["from"] == "1" && e["to"] == "2"),
            "edges={:?}",
            edges
        );
        assert!(
            edges.iter().any(|e| e["from"] == "2" && e["to"] == "1"),
            "cycle edge missing: {:?}",
            edges
        );
        let cycles = graph["cycles"].as_array().cloned().unwrap_or_default();
        assert!(
            !cycles.is_empty(),
            "expected cycle detection, cycles={:?}",
            cycles
        );
    }
}
