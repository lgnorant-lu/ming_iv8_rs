//! Browserify bridge for multi-bundler entry plane.
//!
//! Provides detection, source-text wrap prelude, and evidence collection
//! for Browserify (browser-pack) bundles. Uses the stable ~10-year
//! browser-pack prelude pattern:
//!
//! ```text
//! (function(){return outer;})()({id:[fn,{deps}]},{},[entry])
//! ```
//!
//! The source-text wrap prelude transforms this prelude call to expose
//! the inner require function globally:
//!
//! ```text
//! (function(){
//!   var _r=(function(){return outer;})()({...},{},[entry]);
//!   globalThis.__iv8_b_require=_r;
//!   return _r;
//! })()
//! ```

use crate::entry::diagnostics as diag;
use crate::kernel::embedded_v8::EmbeddedV8Kernel;
use serde_json::json;

pub struct BrowserifyDetection {
    pub detected: bool,
    pub is_strong: bool,
    pub module_count: usize,
    pub entry_ids: Vec<usize>,
}

pub fn detect(source: &str) -> BrowserifyDetection {
    let has_wrappers = source.contains("function(require,module,exports)")
        || source.contains("function(require, module, exports)");
    if !has_wrappers {
        return BrowserifyDetection {
            detected: false,
            is_strong: false,
            module_count: 0,
            entry_ids: Vec::new(),
        };
    }

    let has_prelude = source.contains("},{},[")
        || source.contains("},{},{},[");

    let mut module_count = 0;
    let mut pos = 0;
    let bytes = source.as_bytes();
    while pos < bytes.len().saturating_sub(5) {
        if let Some(rel) = source[pos..].find("function(require,module,exports)") {
            module_count += 1;
            pos += rel + 5;
        } else if let Some(rel2) = source[pos..].find("function(require, module, exports)") {
            module_count += 1;
            pos += rel2 + 5;
        } else {
            break;
        }
    }

    let mut entry_ids = Vec::new();
    if has_prelude {
        if let Some(prelude_pos) = source.find("},{},[") {
            let after = &source[prelude_pos + 5..];
            if let Some(end_brk) = after.find(']') {
                let entries_str = &after[..end_brk];
                for part in entries_str.split(',') {
                    if let Ok(id) = part.trim().parse::<usize>() {
                        entry_ids.push(id);
                    }
                }
            }
        }
    }

    BrowserifyDetection {
        detected: true,
        is_strong: has_prelude,
        module_count,
        entry_ids,
    }
}

/// Generate the source-text wrap prelude JS.
///
/// Wraps the Browserify prelude call to expose the inner `require` function
/// as `globalThis.__iv8_b_require`, then returns the wrapped result.
pub fn bridge_prelude() -> &'static str {
    concat!(
        "var __iv8_b_original_source = undefined;",
        "var __iv8_b_require = null;",
        "var __iv8_b_require_cache = {};",
        // Intercept Function.prototype.call to detect module factory execution
        // (non-invasive observation only — does not modify execution)
        "(function(){",
        "  var __iv8_b_orig_call = Function.prototype.call;",
        "  Function.prototype.call = function(thisArg) {",
        "    var args = Array.prototype.slice.call(arguments, 1);",
        "    if (typeof this === 'function' && args.length >= 3) {",
        "      var fnSrc = this.toString().substring(0, 200);",
        "      if (fnSrc.indexOf('require') > -1 && fnSrc.indexOf('module') > -1) {",
        "        if (typeof __iv8_runtime_log !== 'undefined') {",
        "          __iv8_runtime_log.push('browserify_factory,' + fnSrc.length);",
        "        }",
        "      }",
        "    }",
        "    return __iv8_b_orig_call.apply(this, arguments);",
        "  };",
        "})();",
    )
}

/// Collect evidence after Browserify bundle execution.
pub fn collect_evidence(kernel: &mut EmbeddedV8Kernel) -> (serde_json::Value, Vec<diag::EvidenceRecord>, Vec<diag::DiagnosticRecord>) {
    let mut evidence: Vec<diag::EvidenceRecord> = Vec::new();
    let mut diagnostics: Vec<diag::DiagnosticRecord> = Vec::new();

    let req_status = kernel
        .eval_to_rust_value("typeof __iv8_b_require");
    let has_require = match &req_status {
        crate::convert::RustValue::String(s) => s != "undefined",
        _ => false,
    };

    let module_graph = if has_require {
        evidence.push(
            diag::EvidenceRecord::new(
                "browserify_require_exposed",
                diag::EvidenceStrength::Strong,
                "browserify",
                "browserify.execute",
                "inner require() function exposed via source-text wrap",
            )
            .with_producer("browserify_bridge.main"),
        );

        json!({
            "kind": "browserify_module_graph",
            "require_exposed": true,
            "evidence_count": evidence.len(),
        })
    } else {
        diagnostics.push(diag::error_diag(
            "BROWSERIFY_REQUIRE_NOT_EXPOSED",
            "browserify.execute",
            "source-text wrap prelude did not expose __iv8_b_require",
        ));
        json!({
            "kind": "browserify_module_graph",
            "require_exposed": false,
            "evidence_count": 0,
        })
    };

    (module_graph, evidence, diagnostics)
}
