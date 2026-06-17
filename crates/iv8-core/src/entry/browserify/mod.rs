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
//! Source-text wrapping transforms the prelude call to expose the inner
//! require function globally:
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

const WRAPPER_PATTERN: &str = "function(require,module,exports)";
const WRAPPER_PATTERN_LEN: usize = 33; // "function(require,module,exports)".len()

pub struct BrowserifyDetection {
    pub detected: bool,
    pub is_strong: bool,
    pub module_count: usize,
    pub entry_ids: Vec<usize>,
}

pub fn detect(source: &str) -> BrowserifyDetection {
    let has_wrappers = source.contains(WRAPPER_PATTERN)
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
    while pos < bytes.len().saturating_sub(WRAPPER_PATTERN_LEN) {
        if let Some(rel) = source[pos..].find(WRAPPER_PATTERN) {
            module_count += 1;
            pos += rel + WRAPPER_PATTERN_LEN;
        } else if let Some(rel2) = source[pos..].find("function(require, module, exports)") {
            module_count += 1;
            pos += rel2 + "function(require, module, exports)".len();
        } else {
            break;
        }
    }

    let mut entry_ids = Vec::new();
    if has_prelude {
        let sep = if source.contains("},{},{},[") {
            "},{},{},["
        } else {
            "},{},["
        };
        if let Some(prelude_pos) = source.find(sep) {
            let after = &source[prelude_pos + sep.len()..];
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

/// Source-text wrap: wraps the browser-pack source to capture the inner
/// `require` function as `globalThis.__iv8_b_require`.
///
/// The browser-pack source itself evaluates to the `newRequire` function.
/// We wrap it in an IIFE that captures this return value.
pub fn wrap_source(source: &str) -> String {
    format!(
        "(function(){{var _r={};globalThis.__iv8_b_require=_r;return _r;}})()",
        source
    )
}

/// Generate the observation prelude JS.
///
/// Sets up the __iv8_b_require_cache for use after source execution.
/// The actual require function is exposed via source-text wrapping (wrap_source).
pub fn bridge_prelude() -> &'static str {
    "var __iv8_b_require_cache = {};"
}

/// Collect evidence after Browserify bundle execution.
pub fn collect_evidence(kernel: &mut EmbeddedV8Kernel) -> (serde_json::Value, Vec<diag::EvidenceRecord>, Vec<diag::DiagnosticRecord>) {
    let mut evidence: Vec<diag::EvidenceRecord> = Vec::new();
    let mut diagnostics: Vec<diag::DiagnosticRecord> = Vec::new();

    let req_val = kernel.eval_to_rust_value("__iv8_b_require");
    let has_require = match &req_val {
        crate::convert::RustValue::Null => false,
        crate::convert::RustValue::String(s) => s != "undefined",
        _ => true,
    };

    if has_require {
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
    } else {
        diagnostics.push(diag::warn_diag(
            "BROWSERIFY_REQUIRE_NOT_EXPOSED",
            "browserify.execute",
            "source-text wrap did not expose __iv8_b_require",
        ));
    }

    let graph = json!({
        "kind": "browserify_module_graph",
        "require_exposed": has_require,
        "evidence_count": evidence.len(),
    });

    (graph, evidence, diagnostics)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_strong_returns_correct_module_count() {
        let src = "(function(){var e={};function r(){return o;}return r})()({1:[function(require,module,exports){module.exports=42},{dep:2}],2:[function(require,module,exports){}]},{},[1])";
        let det = detect(src);
        assert!(det.detected);
        assert!(det.is_strong);
        assert_eq!(det.module_count, 2);
        assert_eq!(det.entry_ids, vec![1]);
    }

    #[test]
    fn test_detect_weak_returns_correctly() {
        let src = "function(require,module,exports){ module.exports = 42; }";
        let det = detect(src);
        assert!(det.detected);
        assert!(!det.is_strong);
        assert_eq!(det.module_count, 1);
    }

    #[test]
    fn test_detect_not_detected_when_no_wrappers() {
        let src = "var x = 1 + 1;";
        let det = detect(src);
        assert!(!det.detected);
        assert!(!det.is_strong);
    }

    #[test]
    fn test_wrap_source_transforms_prelude() {
        let src = "(function(){return function r(id){return id};})()({1:[function(require,module,exports){module.exports=42}]},{},[1])";
        let wrapped = wrap_source(src);
        assert!(wrapped.contains("__iv8_b_require"), "wrapped source should assign __iv8_b_require");
        assert!(wrapped.contains("globalThis.__iv8_b_require=_r"), "wrapped source should expose require");
        assert_ne!(wrapped, src, "wrapped source should differ from original");
    }

    #[test]
    fn test_wrap_source_wraps_any_source() {
        let src = "42";
        let wrapped = wrap_source(src);
        assert!(wrapped.contains("__iv8_b_require"));
        assert!(wrapped.contains("42"));
    }
}
