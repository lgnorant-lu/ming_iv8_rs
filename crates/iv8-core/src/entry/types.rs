//! v0.6 entry plane canonical types.
//!
//! These types define the shared vocabulary across EntryPlanner, WebpackBridge,
//! RuntimeHookPack, AST instrumentation, and Acceptance Harness.
//! All types are serializable for cross-language consumption.

use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEntry {
    pub code: String,
    pub stage: String,
    pub message: String,
    pub strategy_id: Option<String>,
    pub recoverable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostics {
    pub sample_signals: Vec<String>,
    pub selected_strategy_reason: Option<String>,
    pub fallback_attempts: Vec<String>,
    pub activation_timing: Option<String>,
    pub policy_constraints: Vec<String>,
    pub missing_capabilities: Vec<String>,
    pub reload_reason: Option<String>,
    pub collection_summary: Option<String>,
    pub cleanup_summary: Option<String>,
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
    pub confidence: String,
    pub module_id: Option<u32>,
    pub chunk_id: Option<String>,
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
    pub errors: Vec<ErrorEntry>,
    pub warnings: Vec<ErrorEntry>,
}
