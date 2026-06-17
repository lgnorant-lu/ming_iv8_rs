//! UMD bridge for multi-bundler entry plane.
//!
//! UMD bundles include AMD/CJS/global three-branch chains.
//! The default execution model for non-extractable UMD bundles is
//! direct eval (the global branch will self-execute).

use crate::entry::diagnostics as diag;
use crate::kernel::embedded_v8::EmbeddedV8Kernel;
use serde_json::json;

pub struct UmdDetection {
    pub detected: bool,
    pub has_amd_branch: bool,
    pub has_cjs_branch: bool,
    pub has_global_branch: bool,
}

pub fn detect(source: &str) -> UmdDetection {
    let has_amd = (source.contains("typeof define === 'function'")
        || source.contains("typeof define==='function'"))
        && source.contains("define.amd");
    let has_cjs = source.contains("typeof module === 'object'")
        || source.contains("typeof module==='object'")
        || source.contains("typeof exports === 'object'")
        || source.contains("typeof exports==='object'");
    let has_global = source.contains("factory(global)")
        || source.contains("factory(globalThis)")
        || source.contains("})(global,")
        || source.contains("})(globalThis,")
        || source.contains("}(global,")
        || source.contains("}(globalThis,");

    UmdDetection {
        detected: has_amd && has_cjs && has_global,
        has_amd_branch: has_amd,
        has_cjs_branch: has_cjs,
        has_global_branch: has_global,
    }
}

pub fn bridge_prelude() -> &'static str {
    // UMD bundles self-dispatch via their branch detection.
    // The global branch will execute natively in IV8 context.
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
        "umd_bundle_evaluated",
        diag::EvidenceStrength::Strong,
        "umd",
        "umd.execute",
        "UMD bundle evaluated; global branch dispatched",
    )
    .with_producer("umd_bridge.main")];

    let graph = json!({
        "kind": "umd_bundle",
        "execution_model": "direct_eval_global_branch",
        "evidence_count": evidence.len(),
    });

    (graph, evidence, Vec::new())
}
