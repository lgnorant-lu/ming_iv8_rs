//! Structured evidence, diagnostic, and fallback records shared across all
//! Entry / Environment / Corpus planes.  Aligned with
//! `python/iv8_rs/diagnostics.py`.
//!
//! All types are `Serialize + Deserialize` so they can be consumed by Python
//! callers and embedded in corpus report fragments.

use serde::{Deserialize, Serialize};

// ───
// Evidence
// ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceStrength {
    Strong,
    Weak,
    MarkerOnly,
    DiagnosticOnly,
}

impl EvidenceStrength {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Strong => "strong",
            Self::Weak => "weak",
            Self::MarkerOnly => "marker_only",
            Self::DiagnosticOnly => "diagnostic_only",
        }
    }

    pub fn can_satisfy_pass(&self) -> bool {
        matches!(self, Self::Strong)
    }
}

/// Normalized evidence envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRecord {
    pub kind: String,
    pub strength: EvidenceStrength,
    pub source: String,
    pub stage: String,
    pub summary: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub producer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sample_kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}

impl EvidenceRecord {
    pub fn new(
        kind: &str,
        strength: EvidenceStrength,
        source: &str,
        stage: &str,
        summary: &str,
    ) -> Self {
        Self {
            kind: kind.to_string(),
            strength,
            source: source.to_string(),
            stage: stage.to_string(),
            summary: summary.to_string(),
            producer: None,
            sample_kind: None,
            payload: None,
        }
    }

    pub fn with_producer(mut self, producer: &str) -> Self {
        self.producer = Some(producer.to_string());
        self
    }

    pub fn with_payload(mut self, payload: serde_json::Value) -> Self {
        self.payload = Some(payload);
        self
    }
}

// ───
// Diagnostics
// ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    Error,
    Warn,
    Info,
}

impl DiagnosticSeverity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warn => "warn",
            Self::Info => "info",
        }
    }
}

/// Normalized diagnostic envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticRecord {
    pub code: String,
    pub severity: DiagnosticSeverity,
    pub stage: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<serde_json::Value>,
}

impl DiagnosticRecord {
    pub fn new(code: &str, severity: DiagnosticSeverity, stage: &str, message: &str) -> Self {
        Self {
            code: code.to_string(),
            severity,
            stage: stage.to_string(),
            message: message.to_string(),
            strategy_id: None,
            recovery_hint: None,
            payload: None,
        }
    }
}

// ───
// Fallback Attempts
// ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FallbackStatus {
    Pass,
    Warn,
    Fail,
    Skip,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackAttempt {
    pub strategy_id: String,
    pub status: FallbackStatus,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_strategy: Option<String>,
    pub diagnostics: Vec<DiagnosticRecord>,
    pub evidence: Vec<EvidenceRecord>,
}

// ───
// Diagnostic code constants
// ───

/// Common codes (trace / evidence / diagnostics spec §9)
pub mod codes {
    // Trace
    pub const TRACE_EMPTY: &str = "TRACE_EMPTY";
    pub const TRACE_PREFIX_UNKNOWN: &str = "TRACE_PREFIX_UNKNOWN";
    pub const TRACE_PARSE_PARTIAL: &str = "TRACE_PARSE_PARTIAL";

    // Evidence
    pub const EVIDENCE_EXPECTED_MISSING: &str = "EVIDENCE_EXPECTED_MISSING";
    pub const EVIDENCE_MARKER_ONLY: &str = "EVIDENCE_MARKER_ONLY";
    pub const CONFIDENCE_DOWNGRADED: &str = "CONFIDENCE_DOWNGRADED";

    // Policy
    pub const POLICY_BLOCKED_ACTION: &str = "POLICY_BLOCKED_ACTION";

    // Fallback
    pub const FALLBACK_USED: &str = "FALLBACK_USED";
    pub const FALLBACK_EXHAUSTED: &str = "FALLBACK_EXHAUSTED";

    // SourceRegex
    pub const SOURCE_REGEX_PASS_THROUGH: &str = "SOURCE_REGEX_PASS_THROUGH";

    // CdpProbe
    pub const CDP_PROBE_NOT_IMPLEMENTED: &str = "CDP_PROBE_NOT_IMPLEMENTED";

    // SwitchVM
    pub const SWITCHVM_MARKER_ONLY: &str = "SWITCHVM_MARKER_ONLY";

    // Environment
    pub const ENVIRONMENT_GAP_OBSERVED: &str = "ENVIRONMENT_GAP_OBSERVED";
    pub const ENVIRONMENT_PATCH_REJECTED: &str = "ENVIRONMENT_PATCH_REJECTED";
    pub const ENVIRONMENT_PATCH_UNSAFE: &str = "ENVIRONMENT_PATCH_UNSAFE";
    pub const ENVIRONMENT_PATCH_APPLIED: &str = "ENVIRONMENT_PATCH_APPLIED";
    pub const ENVIRONMENT_RERUN_IMPROVED: &str = "ENVIRONMENT_RERUN_IMPROVED";
    pub const ENVIRONMENT_RERUN_NO_CHANGE: &str = "ENVIRONMENT_RERUN_NO_CHANGE";
    pub const ENVIRONMENT_RERUN_REGRESSED: &str = "ENVIRONMENT_RERUN_REGRESSED";
    pub const ENVIRONMENT_PROFILE_WRITE_BLOCKED: &str = "ENVIRONMENT_PROFILE_WRITE_BLOCKED";

    // Webpack (webpack-bridge-solidification.md §12)
    pub mod webpack {
        pub const RUNTIME_NOT_FOUND: &str = "WEBPACK_RUNTIME_NOT_FOUND";
        pub const RUNTIME_FLAVOR_UNKNOWN: &str = "WEBPACK_RUNTIME_FLAVOR_UNKNOWN";
        pub const REQUIRE_CAPTURE_FAILED: &str = "WEBPACK_REQUIRE_CAPTURE_FAILED";
        pub const REQUIRE_CAPTURE_LATE: &str = "WEBPACK_REQUIRE_CAPTURE_LATE";
        pub const MODULE_TABLE_EMPTY: &str = "WEBPACK_MODULE_TABLE_EMPTY";
        pub const MODULE_CACHE_EMPTY: &str = "WEBPACK_MODULE_CACHE_EMPTY";
        pub const CHUNK_UNSUPPORTED: &str = "WEBPACK_CHUNK_UNSUPPORTED";
        pub const CHUNK_EVENT_WEAK: &str = "WEBPACK_CHUNK_EVENT_WEAK";
        pub const EVIDENCE_WEAK: &str = "WEBPACK_EVIDENCE_WEAK";
        pub const POLICY_BLOCKED: &str = "WEBPACK_POLICY_BLOCKED";
    }

    // Dispatch (dispatch-generalization.md §13)
    pub mod dispatch {
        pub const CANDIDATE_DETECTED: &str = "DISPATCH_CANDIDATE_DETECTED";
        pub const CANDIDATE_REJECTED: &str = "DISPATCH_CANDIDATE_REJECTED";
        pub const STATIC_WEAK: &str = "DISPATCH_STATIC_WEAK";
        pub const TRAP_OVERBROAD: &str = "DISPATCH_TRAP_OVERBROAD";
        pub const TRACE_EMPTY: &str = "DISPATCH_TRACE_EMPTY";
        pub const MULTI_ARG_OBSERVED: &str = "DISPATCH_MULTI_ARG_OBSERVED";
        pub const SWITCH_OBSERVED: &str = "DISPATCH_SWITCH_OBSERVED";
        pub const RUNTIME_VALIDATION_FAILED: &str = "DISPATCH_RUNTIME_VALIDATION_FAILED";
        pub const CLOSURE_CAPTURED: &str = "DISPATCH_CLOSURE_CAPTURED";
        pub const SOURCE_REGEX_FALLBACK: &str = "DISPATCH_SOURCE_REGEX_FALLBACK";
        pub const SWITCHVM_MARKER_ONLY: &str = "SWITCHVM_MARKER_ONLY";
    }

    // SourceAst (source-ast-pipeline.md §12)
    pub mod source_ast {
        pub const PARSE_FAILED: &str = "SOURCE_AST_PARSE_FAILED";
        pub const CANDIDATE_EMPTY: &str = "SOURCE_AST_CANDIDATE_EMPTY";
        pub const JOINPOINT_UNSUPPORTED: &str = "SOURCE_AST_JOINPOINT_UNSUPPORTED";
        pub const TRANSFORM_FAILED: &str = "SOURCE_AST_TRANSFORM_FAILED";
        pub const EMIT_FAILED: &str = "SOURCE_AST_EMIT_FAILED";
        pub const POLICY_BLOCKED: &str = "SOURCE_AST_POLICY_BLOCKED";
        pub const RUNTIME_VALIDATION_FAILED: &str = "SOURCE_AST_RUNTIME_VALIDATION_FAILED";
        pub const REGEX_CAPTURED: &str = "SOURCE_REGEX_CAPTURED";
        pub const REGEX_PASS_THROUGH: &str = "SOURCE_REGEX_PASS_THROUGH";
    }

    // Corpus (corpus-runner-contract.md §15)
    pub mod corpus {
        pub const MANIFEST_INVALID: &str = "CORPUS_MANIFEST_INVALID";
        pub const SAMPLE_SKIPPED: &str = "CORPUS_SAMPLE_SKIPPED";
        pub const SAMPLE_PATH_MISSING: &str = "CORPUS_SAMPLE_PATH_MISSING";
        pub const EXTERNAL_UNRESOLVED: &str = "CORPUS_EXTERNAL_UNRESOLVED";
        pub const EXPECTED_EVIDENCE_MISSING: &str = "CORPUS_EXPECTED_EVIDENCE_MISSING";
        pub const POLICY_VIOLATION: &str = "CORPUS_POLICY_VIOLATION";
        pub const REPORT_WRITE_FAILED: &str = "CORPUS_REPORT_WRITE_FAILED";
        pub const FIXTURE_ONLY: &str = "CORPUS_FIXTURE_ONLY";
    }

    // Environment policy (environment-patch-policy.md §13)
    pub mod policy {
        pub const APPLIED: &str = "PATCH_POLICY_APPLIED";
        pub const REJECTED: &str = "PATCH_POLICY_REJECTED";
        pub const BLOCKED: &str = "PATCH_POLICY_BLOCKED";
        pub const CONFLICT: &str = "PATCH_POLICY_CONFLICT";
        pub const RECLASSIFIED: &str = "PATCH_POLICY_RECLASSIFIED";
        pub const OPT_IN_MISSING: &str = "PATCH_POLICY_OPT_IN_MISSING";
        pub const PERSONA_MISMATCH: &str = "PATCH_POLICY_PERSONA_MISMATCH";
        pub const MUTATION_BLOCKED: &str = "PATCH_POLICY_MUTATION_BLOCKED";
        pub const REGRESSION: &str = "PATCH_POLICY_REGRESSION";
    }
}

/// Helper to build a common DiagnosticRecord with severity, stage & message.
pub fn diag(
    code: &str,
    severity: DiagnosticSeverity,
    stage: &str,
    message: &str,
) -> DiagnosticRecord {
    DiagnosticRecord::new(code, severity, stage, message)
}

pub fn info_diag(code: &str, stage: &str, message: &str) -> DiagnosticRecord {
    diag(code, DiagnosticSeverity::Info, stage, message)
}

pub fn warn_diag(code: &str, stage: &str, message: &str) -> DiagnosticRecord {
    diag(code, DiagnosticSeverity::Warn, stage, message)
}

pub fn error_diag(code: &str, stage: &str, message: &str) -> DiagnosticRecord {
    diag(code, DiagnosticSeverity::Error, stage, message)
}

/// Verify that all required diagnostic codes exist in the Python catalog.
/// Called from integration tests to keep both sides in sync.
pub fn verify_diagnostic_catalog(catalog_keys: &[String]) -> Vec<String> {
    let rust_codes = rust_diagnostic_codes();
    let mut missing = Vec::new();
    for code in &rust_codes {
        if !catalog_keys.contains(code) {
            missing.push(code.clone());
        }
    }
    missing
}

fn rust_diagnostic_codes() -> Vec<String> {
    let mut codes = vec![
        // Common
        codes::TRACE_EMPTY,
        codes::TRACE_PREFIX_UNKNOWN,
        codes::TRACE_PARSE_PARTIAL,
        codes::EVIDENCE_EXPECTED_MISSING,
        codes::EVIDENCE_MARKER_ONLY,
        codes::CONFIDENCE_DOWNGRADED,
        codes::POLICY_BLOCKED_ACTION,
        codes::FALLBACK_USED,
        codes::FALLBACK_EXHAUSTED,
        codes::SOURCE_REGEX_PASS_THROUGH,
        codes::CDP_PROBE_NOT_IMPLEMENTED,
        codes::SWITCHVM_MARKER_ONLY,
        codes::ENVIRONMENT_GAP_OBSERVED,
        codes::ENVIRONMENT_PATCH_REJECTED,
        codes::ENVIRONMENT_PATCH_UNSAFE,
        codes::ENVIRONMENT_PATCH_APPLIED,
        codes::ENVIRONMENT_RERUN_IMPROVED,
        codes::ENVIRONMENT_RERUN_NO_CHANGE,
        codes::ENVIRONMENT_RERUN_REGRESSED,
        codes::ENVIRONMENT_PROFILE_WRITE_BLOCKED,
    ];
    // Webpack
    codes.extend_from_slice(&[
        codes::webpack::RUNTIME_NOT_FOUND,
        codes::webpack::RUNTIME_FLAVOR_UNKNOWN,
        codes::webpack::REQUIRE_CAPTURE_FAILED,
        codes::webpack::REQUIRE_CAPTURE_LATE,
        codes::webpack::MODULE_TABLE_EMPTY,
        codes::webpack::MODULE_CACHE_EMPTY,
        codes::webpack::CHUNK_UNSUPPORTED,
        codes::webpack::CHUNK_EVENT_WEAK,
        codes::webpack::EVIDENCE_WEAK,
        codes::webpack::POLICY_BLOCKED,
    ]);
    // Dispatch
    codes.extend_from_slice(&[
        codes::dispatch::CANDIDATE_DETECTED,
        codes::dispatch::CANDIDATE_REJECTED,
        codes::dispatch::STATIC_WEAK,
        codes::dispatch::TRAP_OVERBROAD,
        codes::dispatch::TRACE_EMPTY,
        codes::dispatch::MULTI_ARG_OBSERVED,
        codes::dispatch::SWITCH_OBSERVED,
        codes::dispatch::RUNTIME_VALIDATION_FAILED,
        codes::dispatch::CLOSURE_CAPTURED,
        codes::dispatch::SOURCE_REGEX_FALLBACK,
        codes::dispatch::SWITCHVM_MARKER_ONLY,
    ]);
    // SourceAst
    codes.extend_from_slice(&[
        codes::source_ast::PARSE_FAILED,
        codes::source_ast::CANDIDATE_EMPTY,
        codes::source_ast::JOINPOINT_UNSUPPORTED,
        codes::source_ast::TRANSFORM_FAILED,
        codes::source_ast::EMIT_FAILED,
        codes::source_ast::POLICY_BLOCKED,
        codes::source_ast::RUNTIME_VALIDATION_FAILED,
        codes::source_ast::REGEX_CAPTURED,
        codes::source_ast::REGEX_PASS_THROUGH,
    ]);
    // Corpus
    codes.extend_from_slice(&[
        codes::corpus::MANIFEST_INVALID,
        codes::corpus::SAMPLE_SKIPPED,
        codes::corpus::SAMPLE_PATH_MISSING,
        codes::corpus::EXTERNAL_UNRESOLVED,
        codes::corpus::EXPECTED_EVIDENCE_MISSING,
        codes::corpus::POLICY_VIOLATION,
        codes::corpus::REPORT_WRITE_FAILED,
        codes::corpus::FIXTURE_ONLY,
    ]);
    // Policy
    codes.extend_from_slice(&[
        codes::policy::APPLIED,
        codes::policy::REJECTED,
        codes::policy::BLOCKED,
        codes::policy::CONFLICT,
        codes::policy::RECLASSIFIED,
        codes::policy::OPT_IN_MISSING,
        codes::policy::PERSONA_MISMATCH,
        codes::policy::MUTATION_BLOCKED,
        codes::policy::REGRESSION,
    ]);
    codes.into_iter().map(String::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evidence_strength_as_str_returns_snake_case() {
        assert_eq!(EvidenceStrength::Strong.as_str(), "strong");
        assert_eq!(EvidenceStrength::Weak.as_str(), "weak");
        assert_eq!(EvidenceStrength::MarkerOnly.as_str(), "marker_only");
        assert_eq!(EvidenceStrength::DiagnosticOnly.as_str(), "diagnostic_only");
    }

    #[test]
    fn test_evidence_strength_can_satisfy_pass_only_strong() {
        assert!(EvidenceStrength::Strong.can_satisfy_pass());
        assert!(!EvidenceStrength::Weak.can_satisfy_pass());
        assert!(!EvidenceStrength::MarkerOnly.can_satisfy_pass());
        assert!(!EvidenceStrength::DiagnosticOnly.can_satisfy_pass());
    }

    #[test]
    fn test_evidence_record_new_sets_required_fields() {
        let record = EvidenceRecord::new("TYPE", EvidenceStrength::Weak, "src", "stage", "summary text");
        assert_eq!(record.kind, "TYPE");
        assert_eq!(record.strength, EvidenceStrength::Weak);
        assert_eq!(record.source, "src");
        assert_eq!(record.stage, "stage");
        assert_eq!(record.summary, "summary text");
        assert!(record.producer.is_none());
        assert!(record.sample_kind.is_none());
        assert!(record.payload.is_none());
    }

    #[test]
    fn test_evidence_record_builder_chains_producer_and_payload() {
        let payload = serde_json::json!({"key": "val"});
        let record = EvidenceRecord::new("TYPE", EvidenceStrength::Strong, "s", "t", "m")
            .with_producer("test_producer")
            .with_payload(payload.clone());
        assert_eq!(record.producer.as_deref(), Some("test_producer"));
        assert_eq!(record.payload, Some(payload));
        assert!(record.sample_kind.is_none());
    }

    #[test]
    fn test_diagnostic_severity_as_str_returns_snake_case() {
        assert_eq!(DiagnosticSeverity::Error.as_str(), "error");
        assert_eq!(DiagnosticSeverity::Warn.as_str(), "warn");
        assert_eq!(DiagnosticSeverity::Info.as_str(), "info");
    }

    #[test]
    fn test_diagnostic_record_new_sets_required_fields() {
        let record = DiagnosticRecord::new("CODE", DiagnosticSeverity::Error, "s1", "msg");
        assert_eq!(record.code, "CODE");
        assert_eq!(record.severity, DiagnosticSeverity::Error);
        assert_eq!(record.stage, "s1");
        assert_eq!(record.message, "msg");
        assert!(record.strategy_id.is_none());
        assert!(record.recovery_hint.is_none());
        assert!(record.payload.is_none());
    }

    #[test]
    fn test_diag_helpers_create_correct_severity() {
        let info = info_diag("C1", "s", "m");
        let warn = warn_diag("C2", "s", "m");
        let error = error_diag("C3", "s", "m");
        assert_eq!(info.severity, DiagnosticSeverity::Info);
        assert_eq!(warn.severity, DiagnosticSeverity::Warn);
        assert_eq!(error.severity, DiagnosticSeverity::Error);
        assert_eq!(info.code, "C1");
        assert_eq!(warn.code, "C2");
        assert_eq!(error.code, "C3");
    }

    #[test]
    fn test_fallback_attempt_structure() {
        let diags = vec![DiagnosticRecord::new("C1", DiagnosticSeverity::Info, "s1", "m1")];
        let evidence = vec![EvidenceRecord::new("T", EvidenceStrength::Weak, "s2", "s3", "m2")];
        let attempt = FallbackAttempt {
            strategy_id: "strat_1".into(),
            status: FallbackStatus::Fail,
            reason: "no candidates".into(),
            next_strategy: Some("strat_2".into()),
            diagnostics: diags.clone(),
            evidence: evidence.clone(),
        };
        assert_eq!(attempt.strategy_id, "strat_1");
        assert_eq!(attempt.status, FallbackStatus::Fail);
        assert_eq!(attempt.reason, "no candidates");
        assert_eq!(attempt.next_strategy.as_deref(), Some("strat_2"));
        assert_eq!(attempt.diagnostics.len(), 1);
        assert_eq!(attempt.evidence.len(), 1);
    }

    #[test]
    fn test_verify_catalog_returns_missing_codes() {
        let partial: Vec<String> = vec!["TRACE_EMPTY".into(), "FALLBACK_USED".into()];
        let missing = verify_diagnostic_catalog(&partial);
        assert!(!missing.is_empty(), "should have missing codes");
        assert!(!missing.contains(&"TRACE_EMPTY".to_string()));
        assert!(!missing.contains(&"FALLBACK_USED".to_string()));
        // spot-check a few expected codes are in the missing list
        assert!(missing.contains(&"TRACE_PREFIX_UNKNOWN".to_string()));
        assert!(missing.contains(&"EVIDENCE_EXPECTED_MISSING".to_string()));
    }

    #[test]
    fn test_verify_catalog_empty_when_all_codes_present() {
        let all_codes = rust_diagnostic_codes();
        let missing = verify_diagnostic_catalog(&all_codes);
        assert!(missing.is_empty(), "should have no missing codes when all present");
    }
}
