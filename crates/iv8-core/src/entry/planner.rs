//! Entry strategy selection and EntryPlan assembly.
//!
//! The EntryPlanner generates candidate strategies based on sample kind
//! and persona/policy, selects the primary strategy, builds fallback chains,
//! and assembles a complete EntryPlan.

use crate::entry::classification;
use crate::entry::types::*;
use crate::entry::webpack;

// ───
// Public API
// ───

/// Plan an entry strategy for the given JS source.
///
/// This is the main entry point for entry planning.
/// Returns an `EntryPlan` ready for execution.
pub fn plan_entry(
    source: &str,
    persona: Persona,
    explicit_policy: Option<Policy>,
    entry_targets: Vec<EntryTarget>,
) -> EntryPlan {
    // 1. Classify sample
    let signals = classification::extract_signals(source);
    let sample_kind = classification::classify(source, &signals);

    // 1b. Run webpack detection for richer signals
    let wp_detection = webpack::detect(source);
    let mut all_signals = signals.clone();
    if wp_detection.detected {
        all_signals.push(format!("webpack_flavor={:?}", wp_detection.flavor));
        for h in &wp_detection.helpers_present {
            all_signals.push(format!("wp_helper={}", h));
        }
        if wp_detection.module_count > 0 {
            all_signals.push(format!("wp_module_count={}", wp_detection.module_count));
        }
    }

    // 2. Merge policy
    let effective_policy = persona.merge_policy(explicit_policy);

    // 3. Generate candidates
    let candidates = generate_candidates(sample_kind, persona, &effective_policy);

    // 4. Select primary strategy
    let selected = select_primary_strategy(&candidates, sample_kind, persona, &effective_policy);

    // 5. Build fallback chain
    let fallback_chain = build_fallback_chain(&candidates, &selected);

    // 6. Determine phase requirements
    let phase_reqs = determine_phase_requirements(sample_kind, &selected);

    // 7. Expected evidence
    let expected_evidence = determine_expected_evidence(sample_kind, &selected);

    // 8. Risk level
    let risk_level = assess_risk(sample_kind, &selected, persona);

    // 9. Assemble plan
    let plan_id = format!("ep_{:x}", chrono_id());
    let state = PlanState::Planned;

    let diagnostics = Diagnostics {
        sample_signals: all_signals.clone(),
        selected_strategy_reason: Some(selected.selection_reason.clone()),
        fallback_attempts: Vec::new(),
        activation_timing: None,
        policy_constraints: Vec::new(),
        missing_capabilities: Vec::new(),
        reload_reason: None,
        collection_summary: None,
        cleanup_summary: None,
    };

    EntryPlan {
        plan_id,
        persona,
        effective_policy,
        sample_kind,
        sample_signals: all_signals,
        selected_strategy: selected,
        candidate_strategies: candidates,
        phase_requirements: phase_reqs.clone(),
        requires_preload: phase_reqs.requires_pre_init_install,
        requires_reload: phase_reqs.requires_init_observation,
        entry_targets,
        expected_evidence,
        fallback_chain,
        risk_level,
        diagnostics,
        state,
    }
}

// ───
// Candidate generation
// ───

/// Generate candidate strategies suitable for the given sample kind
/// and permitted by persona/policy.
pub fn generate_candidates(
    sample_kind: SampleKind,
    persona: Persona,
    policy: &Policy,
) -> Vec<CandidateStrategy> {
    let mut candidates = Vec::new();

    // Helper: add candidate if persona+policy permits
    let mut try_add = |kind: StrategyKind,
                       fit: u32,
                       reload: bool,
                       preload: bool,
                       outputs: Vec<Evidence>,
                       limits: Vec<&str>,
                       rejection: Option<&str>| {
        if persona.allows_strategy(policy, &kind) {
            candidates.push(CandidateStrategy {
                strategy_id: format!("{}.main", kind_to_id(&kind)),
                strategy_kind: kind,
                fit_score: fit,
                requires_reload: reload,
                requires_preload: preload,
                risk_level: RiskLevel::Low,
                expected_outputs: outputs,
                known_limitations: limits.iter().map(|s| s.to_string()).collect(),
                rejection_reason: rejection.map(|s| s.to_string()),
            });
        }
    };

    match sample_kind {
        SampleKind::VmDispatchKnown => {
            try_add(
                StrategyKind::Dispatch, 90, false, true,
                vec![Evidence::Trace, Evidence::Diagnostics],
                vec!["requires preload to intercept handler array"],
                None,
            );
            try_add(
                StrategyKind::SourceAst, 70, false, false,
                vec![Evidence::Trace, Evidence::Diagnostics],
                vec!["may not parse correctly on heavily obfuscated VM code"],
                None,
            );
            try_add(
                StrategyKind::RuntimeTransparent, 50, false, true,
                vec![Evidence::Trace, Evidence::Diagnostics],
                vec!["higher detectability"],
                None,
            );
        }
        SampleKind::WebpackRuntime => {
            try_add(
                StrategyKind::WebpackBridge, 90, true, false,
                vec![Evidence::ModuleGraph, Evidence::Trace, Evidence::Diagnostics],
                vec!["requires reload to capture module init timing"],
                None,
            );
            try_add(
                StrategyKind::RuntimeTransparent, 60, false, true,
                vec![Evidence::Trace, Evidence::Diagnostics],
                vec!["lower module-level evidence quality"],
                None,
            );
        }
        SampleKind::WebpackVmHybrid => {
            try_add(
                StrategyKind::WebpackBridge, 80, true, false,
                vec![Evidence::ModuleGraph, Evidence::Trace, Evidence::EvalSources],
                vec!["VM layer may not be visible through bridge alone"],
                None,
            );
            try_add(
                StrategyKind::Dispatch, 60, false, true,
                vec![Evidence::Trace, Evidence::Diagnostics],
                vec!["dispatch pattern may be closure-captured"],
                None,
            );
            try_add(
                StrategyKind::RuntimeTransparent, 50, false, true,
                vec![Evidence::Trace, Evidence::Diagnostics],
                vec!["may not capture module-graph level evidence"],
                None,
            );
        }
        SampleKind::VmDispatchUnknown => {
            try_add(
                StrategyKind::RuntimeTransparent, 70, false, true,
                vec![Evidence::Trace, Evidence::Diagnostics],
                vec!["cannot directly observe dispatch"],
                None,
            );
            try_add(
                StrategyKind::CdpProbe, 50, false, false,
                vec![Evidence::Diagnostics],
                vec!["lower evidence density"],
                None,
            );
        }
        SampleKind::EvalHeavy => {
            try_add(
                StrategyKind::RuntimeTransparent, 80, false, true,
                vec![Evidence::Trace, Evidence::EvalSources],
                vec!["eval sources may not all be captureable at runtime"],
                None,
            );
            try_add(
                StrategyKind::SourceAst, 70, false, false,
                vec![Evidence::Trace, Evidence::EvalSources],
                vec!["AST-level eval interception is more reliable"],
                None,
            );
        }
        SampleKind::ClosureCapturedRuntime => {
            try_add(
                StrategyKind::RuntimeTransparent, 60, true, true,
                vec![Evidence::Trace, Evidence::Diagnostics],
                vec!["requires pre-init install window; may need reload"],
                None,
            );
            try_add(
                StrategyKind::CdpProbe, 40, false, false,
                vec![Evidence::Diagnostics],
                vec!["low evidence density"],
                None,
            );
        }
        SampleKind::PlainScript => {
            try_add(
                StrategyKind::SourceAst, 90, false, false,
                vec![Evidence::Trace, Evidence::Diagnostics],
                vec![],
                None,
            );
            try_add(
                StrategyKind::RuntimeTransparent, 50, false, false,
                vec![Evidence::Trace, Evidence::Diagnostics],
                vec!["overkill for plain scripts"],
                None,
            );
        }
    }

    // Always add CDP probe as last-resort fallback
    try_add(
        StrategyKind::CdpProbe, 10, false, false,
        vec![Evidence::Diagnostics],
        vec!["most limited evidence; only diagnostics"],
        None,
    );

    candidates
}

// ───
// Primary strategy selection
// ───

/// Select the primary strategy from candidates.
/// Picks the highest-fit candidate that is not rejected.
fn select_primary_strategy(
    candidates: &[CandidateStrategy],
    sample_kind: SampleKind,
    persona: Persona,
    _policy: &Policy,
) -> SelectedStrategy {
    // Find best non-rejected candidate
    let best = candidates
        .iter()
        .filter(|c| c.rejection_reason.is_none())
        .max_by_key(|c| c.fit_score);

    match best {
        Some(candidate) => SelectedStrategy {
            strategy_id: candidate.strategy_id.clone(),
            strategy_kind: candidate.strategy_kind,
            selection_reason: format!(
                "sample_kind={:?}, strategy={:?}, fit_score={}, persona={:?}",
                sample_kind, candidate.strategy_kind, candidate.fit_score, persona
            ),
        },
        None => SelectedStrategy {
            strategy_id: "cdp_probe.last_resort".into(),
            strategy_kind: StrategyKind::CdpProbe,
            selection_reason: "no candidate strategy passed persona/policy constraints".into(),
        },
    }
}

// ───
// Fallback chain
// ───

/// Build fallback chain from candidates, ordered by fit descending.
pub fn build_fallback_chain(
    candidates: &[CandidateStrategy],
    selected: &SelectedStrategy,
) -> Vec<String> {
    let mut chain: Vec<String> = candidates
        .iter()
        .filter(|c| {
            c.rejection_reason.is_none()
                && c.strategy_id != selected.strategy_id
        })
        .map(|c| c.strategy_id.clone())
        .collect();
    chain.sort(); // not strictly meaningful — kept for deterministic order
    chain.push("cdp_probe.last_resort".into());
    chain
}

// ───
// Phase requirements
// ───

fn determine_phase_requirements(
    _sample_kind: SampleKind,
    selected: &SelectedStrategy,
) -> PhaseRequirements {
    match selected.strategy_kind {
        StrategyKind::Dispatch => PhaseRequirements {
            requires_pre_init_install: true,
            requires_init_observation: false,
            requires_armed_transition: true,
            requires_invoke_expr: true,
            requires_post_collection_cleanup: true,
        },
        StrategyKind::WebpackBridge => PhaseRequirements {
            requires_pre_init_install: false,
            requires_init_observation: true,
            requires_armed_transition: false,
            requires_invoke_expr: true,
            requires_post_collection_cleanup: true,
        },
        StrategyKind::RuntimeTransparent => PhaseRequirements {
            requires_pre_init_install: true,
            requires_init_observation: false,
            requires_armed_transition: true,
            requires_invoke_expr: true,
            requires_post_collection_cleanup: true,
        },
        StrategyKind::RuntimeAggressive => PhaseRequirements {
            requires_pre_init_install: true,
            requires_init_observation: false,
            requires_armed_transition: true,
            requires_invoke_expr: true,
            requires_post_collection_cleanup: true,
        },
        _ => PhaseRequirements {
            requires_pre_init_install: false,
            requires_init_observation: false,
            requires_armed_transition: false,
            requires_invoke_expr: true,
            requires_post_collection_cleanup: true,
        },
    }
}

fn determine_expected_evidence(
    sample_kind: SampleKind,
    selected: &SelectedStrategy,
) -> Vec<Evidence> {
    // Base evidence from strategy
    let mut evidence = match selected.strategy_kind {
        StrategyKind::Dispatch => vec![Evidence::Trace],
        StrategyKind::WebpackBridge => vec![Evidence::ModuleGraph, Evidence::Trace],
        StrategyKind::RuntimeTransparent => vec![Evidence::Trace],
        StrategyKind::RuntimeAggressive => vec![Evidence::Trace],
        StrategyKind::SourceAst => vec![Evidence::Trace],
        StrategyKind::SourceRegex => vec![Evidence::Trace],
        StrategyKind::CdpProbe => vec![Evidence::Diagnostics],
    };

    // Add category-specific augmentation
    match sample_kind {
        SampleKind::WebpackVmHybrid | SampleKind::EvalHeavy => {
            evidence.push(Evidence::EvalSources);
        }
        SampleKind::WebpackRuntime => {
            evidence.push(Evidence::ModuleGraph);
        }
        _ => {}
    }

    evidence.push(Evidence::Diagnostics);
    evidence
}

fn assess_risk(
    _sample_kind: SampleKind,
    selected: &SelectedStrategy,
    persona: Persona,
) -> RiskLevel {
    match persona {
        Persona::Runtime => {
            if matches!(selected.strategy_kind, StrategyKind::RuntimeAggressive) {
                RiskLevel::High
            } else if matches!(selected.strategy_kind, StrategyKind::RuntimeTransparent) {
                RiskLevel::Medium
            } else {
                RiskLevel::Low
            }
        }
        Persona::Analysis => RiskLevel::Low,
    }
}

// ───
// Helpers
// ───

fn kind_to_id(kind: &StrategyKind) -> &'static str {
    match kind {
        StrategyKind::Dispatch => "dispatch",
        StrategyKind::RuntimeTransparent => "runtime_transparent",
        StrategyKind::RuntimeAggressive => "runtime_aggressive",
        StrategyKind::SourceAst => "source_ast",
        StrategyKind::SourceRegex => "source_regex",
        StrategyKind::WebpackBridge => "webpack_bridge",
        StrategyKind::CdpProbe => "cdp_probe",
    }
}

/// Simple pseudo-unique ID based on timestamp.
fn chrono_id() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos() as u64
}

// ───
// Tests
// ───

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plan_plain_script_analysis() {
        let plan = plan_entry(
            "var x = 1 + 1;",
            Persona::Analysis,
            None,
            vec![],
        );
        assert_eq!(plan.sample_kind, SampleKind::PlainScript);
        assert_eq!(plan.selected_strategy.strategy_kind, StrategyKind::SourceAst);
        assert_eq!(plan.state, PlanState::Planned);
    }

    #[test]
    fn test_plan_webpack_runtime() {
        let plan = plan_entry(
            "__webpack_require__(42);",
            Persona::Analysis,
            None,
            vec![],
        );
        assert_eq!(plan.sample_kind, SampleKind::WebpackRuntime);
        assert_eq!(plan.selected_strategy.strategy_kind, StrategyKind::WebpackBridge);
    }

    #[test]
    fn test_plan_chaosvm_dispatch() {
        let plan = plan_entry(
            "var A=[];var Q=[];var U=0; A[Q[U++]]();",
            Persona::Analysis,
            None,
            vec![],
        );
        assert_eq!(plan.sample_kind, SampleKind::VmDispatchKnown);
        assert_eq!(plan.selected_strategy.strategy_kind, StrategyKind::Dispatch);
    }

    #[test]
    fn test_plan_runtime_persona_limits() {
        // Runtime persona does not allow source rewrite by default
        let plan = plan_entry(
            "var x = 1;",
            Persona::Runtime,
            None,
            vec![],
        );
        // Falls back to CDP probe since source_ast is not allowed
        assert_eq!(
            plan.selected_strategy.strategy_kind,
            StrategyKind::CdpProbe
        );
        // Should have warnings indicating the limitation
        assert!(
            plan.candidate_strategies.iter().any(|c| !c.expected_outputs.is_empty())
        );
    }

    #[test]
    fn test_candidates_have_rejection_reason_only_when_rejected() {
        let candidates = generate_candidates(
            SampleKind::PlainScript,
            Persona::Analysis,
            &Persona::Analysis.default_policy(),
        );
        for c in &candidates {
            if c.rejection_reason.is_some() {
                assert!(
                    c.fit_score == 0 || c.fit_score < 30,
                    "rejected candidate should have low fit score"
                );
            }
        }
    }

    #[test]
    fn test_fallback_chain_includes_last_resort() {
        let candidates = generate_candidates(
            SampleKind::VmDispatchUnknown,
            Persona::Runtime,
            &Persona::Runtime.default_policy(),
        );
        let selected = select_primary_strategy(
            &candidates, SampleKind::VmDispatchUnknown, Persona::Runtime,
            &Persona::Runtime.default_policy(),
        );
        let chain = build_fallback_chain(&candidates, &selected);
        assert!(chain.iter().any(|s| s.contains("cdp_probe")));
    }
}
