//! Parcel bridge for multi-bundler entry plane.
//!
//! Parcel bundles use `parcelRequire` + `$parcel$` scope prefix.
//! The bundle is self-executing — direct eval suffices.

use crate::entry::diagnostics as diag;
use crate::kernel::embedded_v8::EmbeddedV8Kernel;
use serde_json::json;

pub struct ParcelDetection {
    pub detected: bool,
    pub has_parcel_require: bool,
    pub has_dollar_parcel: bool,
}

pub fn detect(source: &str) -> ParcelDetection {
    let has_parcel_marker = source.contains("$parcel$");
    let has_parcel_require = source.contains("parcelRequire");

    ParcelDetection {
        detected: has_parcel_marker && has_parcel_require,
        has_parcel_require,
        has_dollar_parcel: has_parcel_marker,
    }
}

pub fn bridge_prelude() -> &'static str {
    // Parcel bundles with parcelRequire are self-executing.
    // No special prelude needed; direct eval suffices.
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
        "parcel_direct_eval",
        diag::EvidenceStrength::Strong,
        "parcel",
        "parcel.execute",
        "Parcel bundle evaluated via direct eval",
    )
    .with_producer("parcel_bridge.main")];

    let graph = json!({
        "kind": "parcel_bundle",
        "execution_model": "direct_eval",
        "evidence_count": evidence.len(),
    });

    (graph, evidence, Vec::new())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_with_markers() {
        let src = "var $parcel$global={};function parcelRequire(id){return{}};";
        let det = detect(src);
        assert!(det.detected);
        assert!(det.has_parcel_require);
        assert!(det.has_dollar_parcel);
    }

    #[test]
    fn test_detect_not_detected_plain_cjs() {
        let src = "var require = function(id) { return {}; }; module.exports = 42;";
        let det = detect(src);
        assert!(!det.detected);
    }

    #[test]
    fn test_detect_parcel_require_only_not_enough() {
        let src = "function parcelRequire(id){return{}};";
        let det = detect(src);
        assert!(!det.detected);
    }

    #[test]
    fn test_detect_dollar_parcel_only_not_enough() {
        let src = "var $parcel$global = {};";
        let det = detect(src);
        assert!(!det.detected);
    }

    #[test]
    fn test_bridge_prelude_is_empty() {
        assert_eq!(bridge_prelude(), "");
    }
}
