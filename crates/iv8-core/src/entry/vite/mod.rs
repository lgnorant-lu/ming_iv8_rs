//! Vite IIFE bridge for multi-bundler entry plane.
//!
//! Vite IIFE output is a self-executing bundle (Vite configured with
//! build.rollupOptions.output.format = 'iife'). Direct eval suffices.
//! Full ESM module mode is deferred to v0.8.53.

use crate::entry::diagnostics as diag;
use crate::kernel::embedded_v8::EmbeddedV8Kernel;
use serde_json::json;

pub struct ViteDetection {
    pub detected: bool,
    pub has_iife_wrapper: bool,
    pub has_preload_helpers: bool,
}

pub fn detect(source: &str) -> ViteDetection {
    let has_vite = source.contains("__vitePreload")
        || source.contains("__VITE_IS_MODERN__")
        || source.contains("__vite__mapDeps");
    let has_iife = source.trim_start().starts_with("(function(")
        || source.trim_start().starts_with("!function(")
        || source.trim_start().starts_with("(()=>");

    ViteDetection {
        detected: has_vite && !source.contains("__webpack_require__"),
        has_iife_wrapper: has_iife,
        has_preload_helpers: source.contains("__vitePreload") || source.contains("__vite__mapDeps"),
    }
}

pub fn bridge_prelude() -> &'static str {
    // Vite IIFE is self-executing. No special prelude needed.
    // ESM module mode deferred to v0.8.53.
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
        "vite_iife_evaluated",
        diag::EvidenceStrength::Strong,
        "vite",
        "vite.execute",
        "Vite IIFE bundle evaluated via direct eval",
    )
    .with_producer("vite_bridge.main")];

    let graph = json!({
        "kind": "vite_iife_bundle",
        "execution_model": "direct_eval",
        "esm_support": "deferred_to_v0.8.53",
        "evidence_count": evidence.len(),
    });

    (graph, evidence, Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_with_vite_preload() {
        let src = "const __vitePreload=function(u,d,a){return Promise.resolve()};";
        let det = detect(src);
        assert!(det.detected);
        assert!(det.has_preload_helpers);
    }

    #[test]
    fn test_detect_with_vite_modern() {
        let src = "const __VITE_IS_MODERN__=true;";
        let det = detect(src);
        assert!(det.detected);
    }

    #[test]
    fn test_detect_with_map_deps() {
        let src = "const __vite__mapDeps=[];";
        let det = detect(src);
        assert!(det.detected);
        assert!(det.has_preload_helpers);
    }

    #[test]
    fn test_detect_not_detected_when_webpack() {
        let src = "const __vitePreload=function(){}; __webpack_require__(1);";
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
