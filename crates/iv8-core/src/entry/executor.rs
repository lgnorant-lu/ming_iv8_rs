//! Entry plan execution harness.
//!
//! Takes an `EntryPlan` produced by the planner, executes the JS source
//! within the V8 engine according to the selected strategy, and returns
//! an `EntryResult` with collected evidence. Supports fallback chain:
//! if the primary strategy fails, subsequent strategies are tried automatically.

use crate::entry::hooks;
use crate::entry::types::*;
use crate::kernel::embedded_v8::EmbeddedV8Kernel;
use crate::kernel::{EvalOpts, KernelConfig};
use std::collections::HashMap;

/// Execute a prepared entry plan with fallback chain support.
///
/// Tries the primary strategy first. If it fails with a recoverable error,
/// iterates through the fallback chain, creating a fresh kernel for each attempt.
/// `chunks` are evaluated in order before the main `source`.
/// If `entry_expr` is provided, it is evaluated after the main source.
pub fn run_entry(
    plan: &EntryPlan,
    source: &str,
    chunks: &[String],
    entry_expr: Option<&str>,
) -> Result<EntryResult, String> {
    let primary_id = plan.selected_strategy.strategy_id.clone();
    let cand_map: HashMap<&str, &CandidateStrategy> = plan
        .candidate_strategies
        .iter()
        .map(|c| (c.strategy_id.as_str(), c))
        .collect();

    let mut result = EntryResult {
        plan_id: plan.plan_id.clone(),
        final_state: PlanState::Planned,
        selected_strategy: plan.selected_strategy.clone(),
        executed_strategies: Vec::new(),
        trace: Vec::new(),
        trace_meta: None,
        module_graph: None,
        hook_report: None,
        environment_report: None,
        diagnostics: plan.diagnostics.clone(),
        cleanup_state: serde_json::json!({}),
        errors: Vec::new(),
        warnings: Vec::new(),
    };

    // Build ordered list of strategy IDs to try
    let mut try_order: Vec<String> = Vec::new();
    try_order.push(primary_id.clone());
    for fb_id in &plan.fallback_chain {
        if fb_id != &primary_id && !try_order.contains(fb_id) {
            try_order.push(fb_id.clone());
        }
    }

    let mut evidence_satisfied_by_any = false;

    for (attempt_idx, strategy_id) in try_order.iter().enumerate() {
        let strategy_kind = cand_map
            .get(strategy_id.as_str())
            .map(|c| c.strategy_kind)
            .unwrap_or(StrategyKind::CdpProbe);

        record_strategy_semantics(&mut result, strategy_id, strategy_kind);

        let label = if attempt_idx == 0 {
            "primary".to_string()
        } else {
            format!("fallback[{}]", attempt_idx)
        };

        let mut kernel = match EmbeddedV8Kernel::new(KernelConfig::default()) {
            Ok(k) => k,
            Err(e) => {
                result
                    .diagnostics
                    .fallback_attempts
                    .push(format!("{}: kernel init failed: {}", label, e));
                continue;
            }
        };

        // Phase: prepared — apply strategy setup
        if let Err(e) = apply_strategy_prelude(&mut kernel, strategy_kind, source) {
            result
                .diagnostics
                .fallback_attempts
                .push(format!("{}: setup failed: {}", label, e));
            continue;
        }

        // Phase: armed — evaluate chunks then main source
        for (i, chunk) in chunks.iter().enumerate() {
            if let Err(e) = kernel.eval(chunk, EvalOpts::default()) {
                result.warnings.push(ErrorEntry {
                    code: "ACT_CHUNK_EVAL_FAILED".into(),
                    stage: "armed".into(),
                    message: format!("chunk[{}] eval failed: {}", i, e),
                    strategy_id: Some(strategy_id.clone()),
                    recoverable: true,
                });
            }
        }

        // Transform source if strategy requires it
        let eval_source = match strategy_kind {
            StrategyKind::SourceAst => {
                let (transformed, diag) = crate::entry::ast::instrument(source);
                if let Some(d) = diag {
                    result.warnings.push(ErrorEntry {
                        code: "ACT_AST_TRANSFORM_WARNING".into(),
                        stage: "armed".into(),
                        message: d,
                        strategy_id: Some(strategy_id.clone()),
                        recoverable: true,
                    });
                }
                transformed
            }
            StrategyKind::Dispatch => {
                // Dispatch: prepend the runtime dispatch hook prelude
                // (the regex-based dispatch replacement is handled via SourceRegex)
                source.to_string()
            }
            StrategyKind::SourceRegex => source.to_string(),
            _ => source.to_string(),
        };

        let source_ok = match kernel.eval(&eval_source, EvalOpts::default()) {
            Ok(_) => true,
            Err(e) => {
                result.errors.push(ErrorEntry {
                    code: "ACT_SOURCE_EVAL_FAILED".into(),
                    stage: "armed".into(),
                    message: format!("{} source eval: {}", strategy_id, e),
                    strategy_id: Some(strategy_id.clone()),
                    recoverable: true,
                });
                false
            }
        };
        if !source_ok {
            result
                .diagnostics
                .fallback_attempts
                .push(format!("{}: source eval failed", label));
            continue;
        }

        // Phase: invoked — evaluate entry expression
        if let Some(expr) = entry_expr {
            if let Err(e) = kernel.eval(expr, EvalOpts::default()) {
                result.warnings.push(ErrorEntry {
                    code: "ACT_ENTRY_EXPR_FAILED".into(),
                    stage: "invoked".into(),
                    message: format!("{} entry expr: {}", strategy_id, e),
                    strategy_id: Some(strategy_id.clone()),
                    recoverable: true,
                });
            }
        }

        // Phase: collected — gather trace and evidence for this strategy
        collect_strategy_evidence(&mut kernel, &mut result, strategy_id, strategy_kind);

        // Evaluate: does the cumulative evidence meet expected_evidence requirements?
        let evidence_met = evidence_satisfied(&result, &plan.expected_evidence);
        if evidence_met {
            evidence_satisfied_by_any = true;
            result.diagnostics.fallback_attempts.push(format!(
                "{}: evidence satisfied ({})",
                label,
                result.trace.len()
            ));
            break;
        } else {
            result.diagnostics.fallback_attempts.push(format!(
                "{}: evidence insufficient (trace={}), trying next",
                label,
                result.trace.len()
            ));
        }
    }

    if evidence_satisfied_by_any {
        result.final_state = PlanState::Finalized;
    } else if result.executed_strategies.is_empty() {
        result.final_state = PlanState::Armed;
    } else {
        // At least one strategy ran but evidence requirements not met
        result.final_state = PlanState::Collected;
    }

    build_trace_meta(&mut result, plan);
    collect_environment_report(&mut result);
    Ok(result)
}

// ───
// Strategy setup
// ───

/// Apply the runtime prelude for a given strategy kind.
/// Called once per strategy attempt with a fresh kernel.
/// `source` is the original (untransformed) JS source, needed by some
/// runtime hook strategies (e.g. Dispatch) for pattern detection.
fn apply_strategy_prelude(
    kernel: &mut EmbeddedV8Kernel,
    kind: StrategyKind,
    source: &str,
) -> Result<(), String> {
    match kind {
        StrategyKind::SourceAst | StrategyKind::SourceRegex => {
            // Source-level strategies: transform applied before eval, no runtime setup
            Ok(())
        }
        StrategyKind::Dispatch => {
            // Dispatch runtime hook: inject Proxy on handler array or switch VM marker.
            // This is the RUNTIME-LEVEL dispatch instrumentation (complementary to
            // source-level __iv8_trap transform under SourceAst strategy).
            // Detection was already done during planning; we re-detect here to get
            // the exact handler array / PC variable names needed by the prelude.
            let dispatch_det = crate::entry::dispatch::detect(source);
            if dispatch_det.detected {
                match dispatch_det.flavor {
                    crate::entry::dispatch::DispatchFlavor::HandlerArray => {
                        let ha = dispatch_det.handler_array.as_deref().unwrap_or("handlers");
                        let pc = dispatch_det.pc_var.as_deref().unwrap_or("pc");
                        let idx = dispatch_det.index_array.as_deref().unwrap_or("");
                        let st = dispatch_det.stack_var.as_deref().unwrap_or("S");
                        let prelude =
                            crate::entry::dispatch::handler_array_prelude(ha, pc, idx, st);
                        kernel
                            .eval(&prelude, EvalOpts::default())
                            .map_err(|e| format!("dispatch proxy prelude: {}", e))?;
                    }
                    crate::entry::dispatch::DispatchFlavor::SwitchVM => {
                        let prelude = crate::entry::dispatch::switch_vm_prelude();
                        kernel
                            .eval(&prelude, EvalOpts::default())
                            .map_err(|e| format!("dispatch switch prelude: {}", e))?;
                    }
                    _ => {}
                }
            }
            Ok(())
        }
        StrategyKind::WebpackBridge => {
            kernel
                .eval(crate::entry::webpack::bridge_prelude(), EvalOpts::default())
                .map_err(|e| format!("webpack prelude: {}", e))?;
            Ok(())
        }
        StrategyKind::RuntimeTransparent => {
            let js = hooks::transparent::prelude();
            kernel
                .eval(&js, EvalOpts::default())
                .map_err(|e| format!("transparent hook: {}", e))?;
            Ok(())
        }
        StrategyKind::RuntimeAggressive => {
            let js = hooks::aggressive::prelude();
            kernel
                .eval(&js, EvalOpts::default())
                .map_err(|e| format!("aggressive hook: {}", e))?;
            Ok(())
        }
        StrategyKind::CdpProbe => Ok(()),
    }
}

// ───
// Evidence collection (per-strategy)
// ───

/// Collect trace and evidence produced by a single strategy into the result.
fn collect_strategy_evidence(
    kernel: &mut EmbeddedV8Kernel,
    result: &mut EntryResult,
    strategy_id: &str,
    kind: StrategyKind,
) {
    // Pull trace from whichever log is present
    let trace_val = kernel.eval_to_rust_value(concat!(
        "(function(){",
        "if(typeof __iv8_runtime_log!=='undefined')return __iv8_runtime_log;",
        "if(typeof __iv8i_log__!=='undefined')return __iv8i_log__;",
        "return[];",
        "})()"
    ));
    let mut per_strategy_trace: Vec<String> = Vec::new();
    if let crate::convert::RustValue::Array(items) = trace_val {
        for item in items {
            if let crate::convert::RustValue::String(s) = item {
                per_strategy_trace.push(s);
            }
        }
    }

    // Collect webpack module log if applicable
    if matches!(kind, StrategyKind::WebpackBridge) {
        result.module_graph = crate::entry::webpack::collect_module_graph(kernel);
    }

    // Merge per-strategy trace into result
    result.trace.extend(per_strategy_trace.clone());
    result.executed_strategies.push(ExecutedStrategy {
        strategy_id: strategy_id.to_string(),
        phase_entered: PlanState::Collected,
        outcome: Outcome::Success,
        diagnostics: Vec::new(),
    });
}

// ───
// Environment report
// ───

/// Collect environment state snapshot after execution.
fn collect_environment_report(result: &mut EntryResult) {
    result.environment_report = Some(serde_json::json!({
        "kind": "static_execution_summary",
        "is_probe_report": false,
        "note": "This is not an Environment Probe probe/patch/rerun report.",
        "collected_at": "finalized",
        "trace_count": result.trace.len(),
        "strategy_count": result.executed_strategies.len(),
    }));
}

fn record_strategy_semantics(
    result: &mut EntryResult,
    strategy_id: &str,
    strategy_kind: StrategyKind,
) {
    let messages: &[&str] = match strategy_kind {
        StrategyKind::SourceRegex => &[
            "SourceRegex is currently pass-through; no regex transform is applied.",
        ],
        StrategyKind::Dispatch => &[
            "SwitchVM dispatch support is marker/prelude-only and does not perform generic switch-case instrumentation.",
        ],
        StrategyKind::RuntimeAggressive => &[
            "RuntimeAggressive hooks are available but not normally generated by default planner paths.",
        ],
        StrategyKind::SourceAst => &[
            "SourceAst wraps computed member calls; eval/Function source-point capture remains partial.",
        ],
        _ => &[],
    };

    for message in messages {
        let entry = format!("{}: {}", strategy_id, message);
        if !result.diagnostics.missing_capabilities.contains(&entry) {
            result.diagnostics.missing_capabilities.push(entry.clone());
        }
        result.warnings.push(ErrorEntry {
            code: "ACT_STRATEGY_PARTIAL".into(),
            stage: "planned".into(),
            message: message.to_string(),
            strategy_id: Some(strategy_id.to_string()),
            recoverable: true,
        });
    }
}

// ───
// Trace metadata
// ───

/// Build final trace_meta from all accumulated evidence.
fn build_trace_meta(result: &mut EntryResult, plan: &EntryPlan) {
    let executed_ids: Vec<String> = result
        .executed_strategies
        .iter()
        .map(|e| e.strategy_id.clone())
        .collect();

    let mut trace_sources: Vec<TraceSourceKind> = Vec::new();
    for st in &result.executed_strategies {
        let cand = plan
            .candidate_strategies
            .iter()
            .find(|c| c.strategy_id == st.strategy_id);
        if let Some(c) = cand {
            for sk in derive_trace_sources(&c.strategy_kind) {
                if !trace_sources.contains(&sk) {
                    trace_sources.push(sk);
                }
            }
        }
    }

    let mut events = HashMap::new();
    for (idx, entry) in result.trace.iter().enumerate() {
        let (source_kind, confidence) = classify_trace_entry(entry);
        if let Some(sk) = source_kind {
            events.insert(
                idx,
                EventMeta {
                    source_kind: sk,
                    strategy_id: plan.selected_strategy.strategy_id.clone(),
                    phase: "invoked".to_string(),
                    confidence,
                    module_id: None,
                    chunk_id: None,
                },
            );
        }
    }

    result.trace_meta = Some(TraceMeta {
        trace_format: "raw_drcw_v1".to_string(),
        plan_id: plan.plan_id.clone(),
        persona: plan.persona,
        sample_kind: plan.sample_kind,
        selected_strategy_id: plan.selected_strategy.strategy_id.clone(),
        executed_strategy_ids: executed_ids,
        trace_sources,
        events,
    });
}

/// Check whether the collected evidence satisfies the expected evidence requirements.
/// Returns true if ALL required evidence types have been met.
fn evidence_satisfied(result: &EntryResult, expected: &[Evidence]) -> bool {
    if expected.is_empty() {
        return true;
    }
    for ev in expected {
        let ok = match ev {
            Evidence::Trace => !result.trace.is_empty(),
            Evidence::ModuleGraph => result.module_graph.is_some(),
            Evidence::ChunkEvents => result.trace.iter().any(|t| t.starts_with("chunk_")),
            Evidence::EnvReport => result.environment_report.is_some(),
            Evidence::Diagnostics => {
                !result.diagnostics.sample_signals.is_empty()
                    || result.diagnostics.selected_strategy_reason.is_some()
            }
            Evidence::EvalSources => result
                .trace
                .iter()
                .any(|t| t.starts_with("eval,") || t.starts_with("fn_ctor,")),
        };
        if !ok {
            return false;
        }
    }
    true
}

/// Map strategy kind to trace source categories.
fn derive_trace_sources(kind: &StrategyKind) -> Vec<TraceSourceKind> {
    match kind {
        StrategyKind::Dispatch => vec![TraceSourceKind::Dispatch],
        StrategyKind::WebpackBridge => {
            vec![TraceSourceKind::ModuleBridge, TraceSourceKind::RuntimeProxy]
        }
        StrategyKind::RuntimeTransparent => vec![TraceSourceKind::TransparentHook],
        StrategyKind::RuntimeAggressive => vec![TraceSourceKind::RuntimeProxy],
        StrategyKind::SourceAst => vec![TraceSourceKind::SourceAst],
        StrategyKind::SourceRegex => vec![TraceSourceKind::SourceAst],
        StrategyKind::CdpProbe => vec![TraceSourceKind::Cdp],
    }
}

/// Best-effort classification of a raw trace entry string.
/// Returns (source_kind, confidence: 1.0=deterministic, 0.7=probe, 0.5=hook, 0.0=unknown).
fn classify_trace_entry(entry: &str) -> (Option<TraceSourceKind>, f64) {
    if entry.starts_with("D,") {
        return (Some(TraceSourceKind::Dispatch), 1.0);
    }
    if entry.starts_with("R,") || entry.starts_with("W,") {
        return (Some(TraceSourceKind::TransparentHook), 0.5);
    }
    if entry.starts_with("C,") {
        return (Some(TraceSourceKind::TransparentHook), 0.5);
    }
    if entry.starts_with("eval,") || entry.starts_with("fn_ctor,") {
        return (Some(TraceSourceKind::SourceAst), 0.7);
    }
    if entry.starts_with("aggressive_") {
        return (Some(TraceSourceKind::RuntimeProxy), 0.5);
    }
    if entry.starts_with("require,") || entry.starts_with("chunk_") {
        return (Some(TraceSourceKind::ModuleBridge), 1.0);
    }
    if entry.starts_with("env_read,") {
        return (Some(TraceSourceKind::TransparentHook), 0.3);
    }
    (None, 0.0)
}
