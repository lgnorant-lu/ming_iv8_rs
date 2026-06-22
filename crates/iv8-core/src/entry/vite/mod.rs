//! Vite IIFE and ESM bridge for multi-bundler entry plane.
//!
//! Vite IIFE output is a self-executing bundle (Vite configured with
//! build.rollupOptions.output.format = 'iife'). Direct eval suffices.
//! ESM module mode (G5-G8) is implemented here as of v0.8.68 M5.

use crate::entry::diagnostics as diag;
use crate::kernel::embedded_v8::EmbeddedV8Kernel;
use serde_json::json;

pub struct ViteDetection {
    pub detected: bool,
    pub has_iife_wrapper: bool,
    pub has_preload_helpers: bool,
    pub is_esm: bool,
}

pub fn detect(source: &str) -> ViteDetection {
    let has_vite = source.contains("__vitePreload")
        || source.contains("__VITE_IS_MODERN__")
        || source.contains("__vite__mapDeps");
    let has_iife = source.trim_start().starts_with("(function(")
        || source.trim_start().starts_with("!function(")
        || source.trim_start().starts_with("(()=>");
    let is_esm = detect_esm(source);

    ViteDetection {
        detected: (has_vite || is_esm) && !source.contains("__webpack_require__"),
        has_iife_wrapper: has_iife,
        has_preload_helpers: source.contains("__vitePreload") || source.contains("__vite__mapDeps"),
        is_esm,
    }
}

pub fn detect_esm(source: &str) -> bool {
    let has_import = source.contains("import {")
        || source.contains("import \"")
        || source.contains("import *")
        || source.contains("import {");
    let has_export = source.contains("export {")
        || source.contains("export default")
        || source.contains("export const")
        || source.contains("export function")
        || source.contains("export class");
    has_import || has_export
}

pub fn bridge_prelude() -> &'static str {
    // Vite IIFE is self-executing. No special prelude needed.
    // ESM mode prelude is served separately via esm_prelude().
    ""
}

pub fn esm_prelude() -> &'static str {
    r#"
(function(){
    // G6: import.meta shim
    globalThis.__iv8_import_meta = { url: 'file:///inline.js', resolve: function(s) { return s; } };

    // G7: dynamic import hook — M5 bounded: returns rejected Promise in inline ESM mode
    globalThis.__iv8_dynamic_import = function(specifier) {
        return Promise.reject(new Error('dynamic import not available in inline ESM mode'));
    };

    // G5: synthetic module registry for inline multi-module resolution
    globalThis.__iv8_synthetic_modules = {};
})();
"#
}

pub fn collect_evidence(
    _kernel: &mut EmbeddedV8Kernel,
    is_esm: bool,
) -> (
    serde_json::Value,
    Vec<diag::EvidenceRecord>,
    Vec<diag::DiagnosticRecord>,
) {
    let exec_model = if is_esm { "eval_module" } else { "direct_eval" };
    let evidence = vec![diag::EvidenceRecord::new(
        "vite_evaluated",
        diag::EvidenceStrength::Strong,
        "vite",
        "vite.execute",
        &format!("Vite bundle evaluated via {}", exec_model),
    )
    .with_producer("vite_bridge.main")];

    let mut graph = json!({
        "kind": "vite_bundle",
        "execution_model": exec_model,
        "esm_support": if is_esm { "m5_minimal" } else { "iife_only" },
        "evidence_count": evidence.len(),
    });

    if is_esm {
        if let serde_json::Value::Object(ref mut map) = graph {
            map.insert("g5_multi_module".into(), json!("inline_shim"));
            map.insert("g6_import_meta".into(), json!("file_url_shim"));
            map.insert("g7_dynamic_import".into(), json!("rejected_promise"));
            map.insert("g8_top_level_await".into(), json!("microtask_drain"));
        }
    }

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
        assert!(!det.is_esm);
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
    fn test_detect_esm_import_export() {
        let src = "import { x } from './dep.js'; export const y = 1;";
        let det = detect(src);
        assert!(det.detected);
        assert!(det.is_esm);
    }

    #[test]
    fn test_detect_esm_export_only() {
        let src = "export default function hello() { return 42; }";
        let det = detect(src);
        assert!(det.detected);
        assert!(det.is_esm);
    }

    #[test]
    fn test_detect_esm_not_detected_plain() {
        let src = "var x = 1;";
        let det = detect(src);
        assert!(!det.is_esm);
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

    #[test]
    fn test_esm_prelude_not_empty() {
        assert!(!esm_prelude().is_empty());
    }
}
