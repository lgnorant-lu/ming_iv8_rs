//! v0.6/v0.7 entry plane canonical types.
//!
//! These types define the shared vocabulary across EntryPlanner, WebpackBridge,
//! RuntimeHookPack, AST instrumentation, Diagnostics, and Acceptance Harness.
//! All types are serializable for cross-language consumption.

use serde::{Deserialize, Serialize};

use super::diagnostics; // Re-export for convenience

// ───
// Enums
// ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Persona {
    Runtime,
    Analysis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SampleKind {
    PlainScript,
    VmDispatchKnown,
    VmDispatchUnknown,
    WebpackRuntime,
    WebpackVmHybrid,
    EvalHeavy,
    ClosureCapturedRuntime,
    BrowserifyRuntime,
    RollupBundle,
    ViteBundle,
    UmdBundle,
    UnknownIife,
    ParcelBundle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StrategyKind {
    Dispatch,
    RuntimeTransparent,
    RuntimeAggressive,
    SourceAst,
    SourceRegex,
    WebpackBridge,
    CdpProbe,
    BrowserifyBridge,
    RollupBridge,
    UmdBridge,
    ViteBridge,
    ParcelBridge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanState {
    Discovered,
    Planned,
    Prepared,
    Armed,
    Invoked,
    Collected,
    Finalized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Success,
    Partial,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Evidence {
    Trace,
    ModuleGraph,
    ChunkEvents,
    EnvReport,
    Diagnostics,
    EvalSources,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryTargetKind {
    Expr,
    ModuleId,
    ExportName,
    RuntimeMarker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceSourceKind {
    Dispatch,
    RuntimeProxy,
    TransparentHook,
    SourceAst,
    Cdp,
    ModuleBridge,
}

// ───
// Policy sub-enums
// ───

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HookLevel {
    None,
    Transparent,
    Aggressive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceRewrite {
    Disabled,
    Selective,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TraceLevel {
    Off,
    Summary,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticsLevel {
    Off,
    Summary,
    Full,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreloadRequirement {
    BestEffort,
    Required,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DescriptorPreservation {
    BestEffort,
    Strict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CleanupMode {
    None,
    Uninstall,
    Reset,
}

// ───
// Structs
// ───

/// Behaviour policy for an entry session.
/// Controls what hooks, rewrites, traces and diagnostics are allowed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub hook_level: HookLevel,
    pub source_rewrite: SourceRewrite,
    pub preload_requirement: PreloadRequirement,
    pub allow_reload: bool,
    pub trace_level: TraceLevel,
    pub diagnostics_level: DiagnosticsLevel,
    pub trace_sources: Option<Vec<TraceSourceKind>>,
    pub descriptor_preservation: DescriptorPreservation,
    pub preserve_native_tostring: bool,
    pub forbid_proxy_on_sensitive_surfaces: bool,
    pub allow_prototype_patch: bool,
    pub allow_function_intrinsic_patch: bool,
    pub cleanup_mode: CleanupMode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedStrategy {
    pub strategy_id: String,
    pub strategy_kind: StrategyKind,
    pub selection_reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CandidateStrategy {
    pub strategy_id: String,
    pub strategy_kind: StrategyKind,
    pub fit_score: u32,
    pub requires_reload: bool,
    pub requires_preload: bool,
    pub risk_level: RiskLevel,
    pub expected_outputs: Vec<Evidence>,
    pub known_limitations: Vec<String>,
    pub rejection_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryTarget {
    pub target_kind: EntryTargetKind,
    pub target_value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhaseRequirements {
    pub requires_pre_init_install: bool,
    pub requires_init_observation: bool,
    pub requires_armed_transition: bool,
    pub requires_invoke_expr: bool,
    pub requires_post_collection_cleanup: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutedStrategy {
    pub strategy_id: String,
    pub phase_entered: PlanState,
    pub outcome: Outcome,
    pub diagnostics: Vec<diagnostics::DiagnosticRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostics {
    pub sample_signals: Vec<String>,
    pub selected_strategy_reason: Option<String>,
    pub fallback_attempts: Vec<diagnostics::FallbackAttempt>,
    pub activation_timing: Option<String>,
    pub policy_constraints: Vec<String>,
    pub missing_capabilities: Vec<String>,
    pub reload_reason: Option<String>,
    pub collection_summary: Option<String>,
    pub cleanup_summary: Option<String>,
    pub diagnostic_records: Vec<diagnostics::DiagnosticRecord>,
    pub observed_evidence: Vec<diagnostics::EvidenceRecord>,
}

// ───
// Core entry types
// ───

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryPlan {
    pub plan_id: String,
    pub persona: Persona,
    pub effective_policy: Policy,
    pub sample_kind: SampleKind,
    pub sample_signals: Vec<String>,
    pub selected_strategy: SelectedStrategy,
    pub candidate_strategies: Vec<CandidateStrategy>,
    pub phase_requirements: PhaseRequirements,
    pub requires_preload: bool,
    pub requires_reload: bool,
    pub entry_targets: Vec<EntryTarget>,
    pub expected_evidence: Vec<Evidence>,
    pub fallback_chain: Vec<String>,
    pub risk_level: RiskLevel,
    pub diagnostics: Diagnostics,
    pub state: PlanState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventMeta {
    pub source_kind: TraceSourceKind,
    pub strategy_id: String,
    pub phase: String,
    /// Confidence level for this event: 1.0 = source deterministic, 0.7 = probe,
    /// 0.5 = hook probabilistic, 0.0 = unknown / no confidence.
    pub confidence: f64,
    pub module_id: Option<u32>,
    pub chunk_id: Option<String>,
}

/// Results of static viability probing before strategy selection.
/// Each field answers: "is this strategy potentially viable for this source?"
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProbeResult {
    /// SWC successfully parsed the source without error.
    pub can_swc_parse: bool,
    /// A known dispatch pattern (X[Y[Z++]]() or switch(X[Y++])) was found.
    pub has_dispatch_pattern: bool,
    /// Webpack-like runtime (webpackJsonp, __webpack_require__) detected.
    pub has_webpack_runtime: bool,
    /// Early reference capture pattern (references captured before IIFE) detected.
    pub has_closure_capture: bool,
    /// Heavy eval / Function constructor usage detected.
    pub has_eval_heavy: bool,
    /// Source has minimal obfuscation — AST-based transform is low-risk.
    pub is_low_obfuscation: bool,
    /// Browserify prelude pattern detected.
    pub has_browserify_runtime: bool,
    /// Rollup IIFE/UMD bundle detected.
    pub has_rollup_bundle: bool,
    /// Vite IIFE output detected.
    pub has_vite_bundle: bool,
}

impl ProbeResult {
    /// Whether the source can be meaningfully transformed at the AST level.
    pub fn source_rewrite_viable(&self) -> bool {
        self.can_swc_parse && self.is_low_obfuscation
    }

    /// Whether regex-based dispatch hook can be applied.
    pub fn dispatch_regex_viable(&self) -> bool {
        self.has_dispatch_pattern
    }

    /// Whether a runtime probe (webpack bridge) is applicable.
    pub fn webpack_probe_viable(&self) -> bool {
        self.has_webpack_runtime
    }

    /// Whether pre-install hooks are likely to survive closure capture.
    pub fn pre_install_required(&self) -> bool {
        self.has_closure_capture
    }

    /// Whether a browserify bridge probe is applicable.
    pub fn browserify_probe_viable(&self) -> bool {
        self.has_browserify_runtime
    }

    /// Whether a rollup bridge probe is applicable.
    pub fn rollup_probe_viable(&self) -> bool {
        self.has_rollup_bundle
    }

    /// Whether a vite bridge probe is applicable.
    pub fn vite_probe_viable(&self) -> bool {
        self.has_vite_bundle
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceMeta {
    pub trace_format: String,
    pub plan_id: String,
    pub persona: Persona,
    pub sample_kind: SampleKind,
    pub selected_strategy_id: String,
    pub executed_strategy_ids: Vec<String>,
    pub trace_sources: Vec<TraceSourceKind>,
    pub events: std::collections::HashMap<usize, EventMeta>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntryResult {
    pub plan_id: String,
    pub final_state: PlanState,
    pub selected_strategy: SelectedStrategy,
    pub executed_strategies: Vec<ExecutedStrategy>,
    pub trace: Vec<String>,
    pub trace_meta: Option<TraceMeta>,
    pub module_graph: Option<serde_json::Value>,
    pub hook_report: Option<serde_json::Value>,
    pub environment_report: Option<serde_json::Value>,
    pub diagnostics: Diagnostics,
    pub cleanup_state: serde_json::Value,
    /// Flat list of all structured diagnostic records produced during entry.
    pub diagnostic_records: Vec<diagnostics::DiagnosticRecord>,
    /// Structured evidence collected during execution.
    pub observed_evidence: Vec<diagnostics::EvidenceRecord>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe_browserify_viable_when_has_runtime() {
        let mut probe = ProbeResult::default();
        assert!(!probe.browserify_probe_viable());
        probe.has_browserify_runtime = true;
        assert!(probe.browserify_probe_viable());
    }

    #[test]
    fn test_probe_rollup_viable_when_has_bundle() {
        let mut probe = ProbeResult::default();
        assert!(!probe.rollup_probe_viable());
        probe.has_rollup_bundle = true;
        assert!(probe.rollup_probe_viable());
    }

    #[test]
    fn test_probe_vite_viable_when_has_bundle() {
        let mut probe = ProbeResult::default();
        assert!(!probe.vite_probe_viable());
        probe.has_vite_bundle = true;
        assert!(probe.vite_probe_viable());
    }

    #[test]
    fn test_probe_source_rewrite_viable() {
        let mut probe = ProbeResult::default();
        assert!(!probe.source_rewrite_viable());
        probe.can_swc_parse = true;
        probe.is_low_obfuscation = true;
        assert!(probe.source_rewrite_viable());
    }

    #[test]
    fn test_probe_webpack_viable() {
        let mut probe = ProbeResult::default();
        assert!(!probe.webpack_probe_viable());
        probe.has_webpack_runtime = true;
        assert!(probe.webpack_probe_viable());
    }
}
