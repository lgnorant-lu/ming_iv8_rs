//! Rollup IIFE bridge for multi-bundler entry plane.
//!
//! Rollup IIFE output is a self-executing single-scope bundle.
//! No special prelude needed — direct eval suffices.

use crate::entry::diagnostics as diag;
use crate::kernel::embedded_v8::EmbeddedV8Kernel;
use serde_json::json;

pub struct RollupDetection {
    pub detected: bool,
    pub is_iife: bool,
    pub has_pure_annotations: bool,
}

pub fn detect(source: &str) -> RollupDetection {
    let has_pure = source.contains("/*#__PURE__*/");
    let has_interop = source.contains("_interopNamespace")
        || source.contains("_interopRequireDefault")
        || source.contains("_interopRequireWildcard");
    let is_iife = source.trim_start().starts_with("(function(")
        || source.trim_start().starts_with("!function(");

    RollupDetection {
        detected: (has_pure || has_interop) && !source.contains("__webpack_require__"),
        is_iife,
        has_pure_annotations: has_pure,
    }
}

pub fn bridge_prelude() -> &'static str {
    // Rollup IIFE is self-executing. No special prelude needed.
    // The executor evals the source directly.
    ""
}

pub fn collect_evidence(
    _kernel: &mut EmbeddedV8Kernel,
) -> (
    serde_json::Value,
    Vec<diag::EvidenceRecord>,
    Vec<diag::DiagnosticRecord>,
) {
    let evidence = vec![diag::EvidenceRecord::new(
        "rollup_iife_evaluated",
        diag::EvidenceStrength::Strong,
        "rollup",
        "rollup.execute",
        "Rollup IIFE bundle evaluated via direct eval",
    )
    .with_producer("rollup_bridge.main")];

    let graph = json!({
        "kind": "rollup_iife_bundle",
        "execution_model": "direct_eval",
        "evidence_count": evidence.len(),
    });

    (graph, evidence, Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_iife_with_pure_annotation() {
        let src = "var a=/*#__PURE__*/function(){return 1}();";
        let det = detect(src);
        assert!(det.detected);
        assert!(det.has_pure_annotations);
    }

    #[test]
    fn test_detect_iife_with_interop() {
        let src = "function _interopNamespace(e){return e;}";
        let det = detect(src);
        assert!(det.detected);
    }

    #[test]
    fn test_detect_not_detected_when_webpack() {
        let src = "function _interopNamespace(e){}; __webpack_require__(1);";
        let det = detect(src);
        assert!(!det.detected);
    }

    #[test]
    fn test_detect_not_detected_plain_script() {
        let src = "var x = 1;";
        let det = detect(src);
        assert!(!det.detected);
    }

    #[test]
    fn test_bridge_prelude_is_empty() {
        assert_eq!(bridge_prelude(), "");
    }
}
