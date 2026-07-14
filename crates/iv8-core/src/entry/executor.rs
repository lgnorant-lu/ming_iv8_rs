//! Entry plan execution harness.
//!
//! Takes an `EntryPlan` produced by the planner, executes the JS source
//! within the V8 engine according to the selected strategy, and returns
//! an `EntryResult` with collected evidence. Supports fallback chain:
//! if the primary strategy fails, subsequent strategies are tried automatically.

use crate::entry::diagnostics as diag;
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
        diagnostic_records: Vec::new(),
        observed_evidence: Vec::new(),
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

        let _label = if attempt_idx == 0 {
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
                    .push(diag::FallbackAttempt {
                        strategy_id: strategy_id.clone(),
                        status: diag::FallbackStatus::Fail,
                        reason: format!("kernel init failed: {}", e),
                        next_strategy: None,
                        diagnostics: vec![diag::error_diag(
                            diag::codes::FALLBACK_EXHAUSTED,
                            "entry.execute",
                            &format!("kernel init failed: {}", e),
                        )],
                        evidence: vec![],
                    });
                continue;
            }
        };

        // Phase: prepared — apply strategy setup
        if let Err(e) =
            apply_strategy_prelude(&mut kernel, strategy_kind, source, &plan.effective_policy)
        {
            result
                .diagnostics
                .fallback_attempts
                .push(diag::FallbackAttempt {
                    strategy_id: strategy_id.clone(),
                    status: diag::FallbackStatus::Warn,
                    reason: format!("setup failed: {}", e),
                    next_strategy: None,
                    diagnostics: vec![diag::warn_diag(
                        diag::codes::FALLBACK_USED,
                        "entry.execute",
                        &format!("setup failed: {}", e),
                    )],
                    evidence: vec![],
                });
            continue;
        }

        // Phase: armed — evaluate chunks then main source
        for (i, chunk) in chunks.iter().enumerate() {
            if let Err(e) = kernel.eval(chunk, EvalOpts::default()) {
                result.diagnostic_records.push(diag::DiagnosticRecord::new(
                    "ACT_CHUNK_EVAL_FAILED",
                    diag::DiagnosticSeverity::Warn,
                    "armed",
                    &format!("chunk[{}] eval failed: {}", i, e),
                ));
            } else if strategy_kind == StrategyKind::WebpackBridge {
                // A-P0-1: after each chunk, merge factories into live require.m
                let _ = crate::entry::webpack::install_chunk_factories_public(&mut kernel);
            }
        }
        if strategy_kind == StrategyKind::WebpackBridge && !chunks.is_empty() {
            let _ = crate::entry::webpack::install_chunk_factories_public(&mut kernel);
        }

        // Transform source if strategy requires it
        let eval_source = match strategy_kind {
            StrategyKind::SourceAst => {
                let (transformed, report) = crate::entry::ast::instrument_with_report(source);
                result.observed_evidence.extend(report.evidence);
                result.diagnostic_records.extend(report.diagnostics);
                transformed
            }
            StrategyKind::Dispatch => source.to_string(),
            StrategyKind::BrowserifyBridge => crate::entry::browserify::wrap_source(source),
            StrategyKind::SourceRegex => source.to_string(),
            _ => source.to_string(),
        };

        let source_ok = if strategy_kind == StrategyKind::ViteBridge
            && crate::entry::vite::detect(source).is_esm
        {
            match kernel.eval_module(source, Some("inline.js"), EvalOpts::default()) {
                Ok(_) => {
                    kernel.drain_microtasks();
                    true
                }
                Err(e) => {
                    result.diagnostic_records.push(diag::DiagnosticRecord::new(
                        "ACT_SOURCE_EVAL_FAILED",
                        diag::DiagnosticSeverity::Error,
                        "armed",
                        &format!("{} source eval (ESM): {}", strategy_id, e),
                    ));
                    false
                }
            }
        } else {
            match kernel.eval(&eval_source, EvalOpts::default()) {
                Ok(_) => true,
                Err(e) => {
                    result.diagnostic_records.push(diag::DiagnosticRecord::new(
                        "ACT_SOURCE_EVAL_FAILED",
                        diag::DiagnosticSeverity::Error,
                        "armed",
                        &format!("{} source eval: {}", strategy_id, e),
                    ));
                    false
                }
            }
        };
        if !source_ok {
            result
                .diagnostics
                .fallback_attempts
                .push(diag::FallbackAttempt {
                    strategy_id: strategy_id.clone(),
                    status: diag::FallbackStatus::Fail,
                    reason: "source eval failed".to_string(),
                    next_strategy: None,
                    diagnostics: vec![diag::error_diag(
                        diag::codes::FALLBACK_EXHAUSTED,
                        "entry.execute",
                        &format!("{} source eval failed", strategy_id),
                    )],
                    evidence: vec![],
                });
            continue;
        }

        // Phase: invoked — evaluate entry expression
        if let Some(expr) = entry_expr {
            if let Err(e) = kernel.eval(expr, EvalOpts::default()) {
                result.diagnostic_records.push(diag::DiagnosticRecord::new(
                    "ACT_ENTRY_EXPR_FAILED",
                    diag::DiagnosticSeverity::Warn,
                    "invoked",
                    &format!("{} entry expr: {}", strategy_id, e),
                ));
            }
        }

        // Phase: collected — gather trace and evidence for this strategy
        collect_strategy_evidence(&mut kernel, &mut result, strategy_id, strategy_kind, source);

        // Evaluate: does the cumulative evidence meet expected_evidence requirements?
        let evidence_met = evidence_satisfied(&result, &plan.expected_evidence);
        if evidence_met {
            evidence_satisfied_by_any = true;
            result
                .diagnostics
                .fallback_attempts
                .push(diag::FallbackAttempt {
                    strategy_id: strategy_id.clone(),
                    status: diag::FallbackStatus::Pass,
                    reason: format!("evidence satisfied ({})", result.trace.len()),
                    next_strategy: None,
                    diagnostics: vec![],
                    evidence: vec![],
                });
            break;
        } else {
            result
                .diagnostics
                .fallback_attempts
                .push(diag::FallbackAttempt {
                    strategy_id: strategy_id.clone(),
                    status: diag::FallbackStatus::Warn,
                    reason: format!(
                        "evidence insufficient (trace={}), trying next",
                        result.trace.len()
                    ),
                    next_strategy: None,
                    diagnostics: vec![],
                    evidence: vec![],
                });
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

/// Apply the runtime prelude for a given strategy kind, subject to policy.
/// Called once per strategy attempt with a fresh kernel.
fn apply_strategy_prelude(
    kernel: &mut EmbeddedV8Kernel,
    kind: StrategyKind,
    source: &str,
    policy: &Policy,
) -> Result<(), String> {
    match kind {
        StrategyKind::SourceAst | StrategyKind::SourceRegex => Ok(()),
        StrategyKind::Dispatch => {
            // Dispatch Proxy instrumentation modifies get/apply traps.
            // Skip if proxy on sensitive surfaces is forbidden.
            if policy.forbid_proxy_on_sensitive_surfaces {
                return Ok(());
            }
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
                    crate::entry::dispatch::DispatchFlavor::ClosureScoped => {
                        // ClosureScoped dispatch: hook installed after closure capture.
                        // No prelude to evaluate — runtime proxy cannot observe all calls
                        // via the standard handler_array_prelude path. This is a known
                        // limitation; the detection record already warns about it.
                    }
                    _ => {}
                }
            }
            Ok(())
        }
        StrategyKind::WebpackBridge => {
            // Webpack prelude may modify Function.prototype.call and rewrite
            // module factories. Use the safe variant when prototype patch is forbidden.
            if policy.allow_prototype_patch {
                kernel
                    .eval(crate::entry::webpack::bridge_prelude(), EvalOpts::default())
                    .map_err(|e| format!("webpack prelude: {}", e))?;
            } else {
                kernel
                    .eval(
                        crate::entry::webpack::safe_bridge_prelude(),
                        EvalOpts::default(),
                    )
                    .map_err(|e| format!("webpack safe prelude: {}", e))?;
            }
            Ok(())
        }
        StrategyKind::RuntimeTransparent => {
            // Transparent hook reads existing getters without modifying prototypes.
            // It is safe under all policies.
            let js = hooks::transparent::prelude();
            kernel
                .eval(&js, EvalOpts::default())
                .map_err(|e| format!("transparent hook: {}", e))?;
            Ok(())
        }
        StrategyKind::RuntimeAggressive => {
            // Aggressive hook uses Proxy on global objects. Skip if forbidden.
            if policy.forbid_proxy_on_sensitive_surfaces {
                return Ok(());
            }
            if !policy.allow_function_intrinsic_patch {
                // Aggressive mode patches Function.prototype.toString.
                // Skip if intrinsic patch is forbidden.
                return Ok(());
            }
            let js = hooks::aggressive::prelude();
            kernel
                .eval(&js, EvalOpts::default())
                .map_err(|e| format!("aggressive hook: {}", e))?;
            Ok(())
        }
        StrategyKind::CdpProbe => Ok(()),
        StrategyKind::BrowserifyBridge => {
            kernel
                .eval(
                    crate::entry::browserify::bridge_prelude(),
                    EvalOpts::default(),
                )
                .map_err(|e| format!("browserify prelude: {}", e))?;
            Ok(())
        }
        StrategyKind::RollupBridge | StrategyKind::UmdBridge
            | StrategyKind::ParcelBridge => Ok(()),
        StrategyKind::ViteBridge => {
            let det = crate::entry::vite::detect(source);
            if det.is_esm {
                let prelude = crate::entry::vite::esm_prelude();
                kernel
                    .eval(prelude, EvalOpts::default())
                    .map_err(|e| format!("vite esm prelude: {}", e))?;
            }
            Ok(())
        }
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
    source: &str,
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
        if let Some(ref graph) = result.module_graph {
            if let Some(ev_array) = graph.get("evidence").and_then(|v| v.as_array()) {
                for ev in ev_array {
                    if let Ok(record) = serde_json::from_value::<diag::EvidenceRecord>(ev.clone()) {
                        result.observed_evidence.push(record);
                    }
                }
            }
            if let Some(diag_array) = graph.get("diagnostics").and_then(|v| v.as_array()) {
                for d in diag_array {
                    if let Ok(record) = serde_json::from_value::<diag::DiagnosticRecord>(d.clone())
                    {
                        result.diagnostic_records.push(record);
                    }
                }
            }
        }
    }

    if matches!(kind, StrategyKind::BrowserifyBridge) {
        let (graph, evidence, diagnostics) = crate::entry::browserify::collect_evidence(kernel);
        result.module_graph = Some(graph);
        result.observed_evidence.extend(evidence);
        result.diagnostic_records.extend(diagnostics);
    }

    if matches!(kind, StrategyKind::RollupBridge) {
        let (graph, evidence, diagnostics) = crate::entry::rollup::collect_evidence(kernel);
        result.module_graph = Some(graph);
        result.observed_evidence.extend(evidence);
        result.diagnostic_records.extend(diagnostics);
    }

    if matches!(kind, StrategyKind::ViteBridge) {
        let is_esm = crate::entry::vite::detect(source).is_esm;
        let (graph, evidence, diagnostics) = crate::entry::vite::collect_evidence(kernel, is_esm);
        result.module_graph = Some(graph);
        result.observed_evidence.extend(evidence);
        result.diagnostic_records.extend(diagnostics);
    }

    if matches!(kind, StrategyKind::UmdBridge) {
        let (graph, evidence, diagnostics) = crate::entry::umd::collect_evidence(kernel);
        result.module_graph = Some(graph);
        result.observed_evidence.extend(evidence);
        result.diagnostic_records.extend(diagnostics);
    }

    if matches!(kind, StrategyKind::ParcelBridge) {
        let (graph, evidence, diagnostics) = crate::entry::parcel::collect_evidence(kernel);
        result.module_graph = Some(graph);
        result.observed_evidence.extend(evidence);
        result.diagnostic_records.extend(diagnostics);
    }

    if matches!(kind, StrategyKind::Dispatch) {
        let det = crate::entry::dispatch::detect(source);
        result.observed_evidence.extend(det.to_evidence_records());
        result
            .diagnostic_records
            .extend(det.to_diagnostic_records());

        let has_dispatch_trace = per_strategy_trace.iter().any(|t| t.starts_with("D,"));
        if has_dispatch_trace {
            result.observed_evidence.push(
                diag::EvidenceRecord::new(
                    "dispatch_trace_observed",
                    diag::EvidenceStrength::Strong,
                    "dispatch",
                    "dispatch.execute",
                    "dispatch trace observed at runtime",
                )
                .with_producer("dispatch.main"),
            );
        } else {
            result.diagnostic_records.push(diag::error_diag(
                diag::codes::dispatch::TRACE_EMPTY,
                "dispatch.execute",
                "dispatch strategy executed but produced no trace events",
            ));
        }
    }

    if matches!(kind, StrategyKind::SourceAst) {
        let has_candidates = result
            .observed_evidence
            .iter()
            .any(|e| e.kind == "source_ast_candidate_detected");
        let has_ast_trace = per_strategy_trace
            .iter()
            .any(|t| t.starts_with("D,") || t.starts_with("eval,") || t.starts_with("fn_ctor,"));
        if has_ast_trace {
            result.observed_evidence.push(
                diag::EvidenceRecord::new(
                    "source_ast_runtime_validated",
                    diag::EvidenceStrength::Strong,
                    "source_ast",
                    "source_ast.validate",
                    "AST join points validated by runtime trace",
                )
                .with_producer("source_ast.main"),
            );

            let has_eval = per_strategy_trace.iter().any(|t| t.starts_with("eval,"));
            if has_eval {
                result.observed_evidence.push(
                    diag::EvidenceRecord::new(
                        "eval_source_captured",
                        diag::EvidenceStrength::Strong,
                        "source_ast",
                        "source_ast.validate",
                        "eval source captured at runtime",
                    )
                    .with_producer("source_ast.main"),
                );
            }
            let has_fn = per_strategy_trace.iter().any(|t| t.starts_with("fn_ctor,"));
            if has_fn {
                result.observed_evidence.push(
                    diag::EvidenceRecord::new(
                        "function_constructor_source_captured",
                        diag::EvidenceStrength::Strong,
                        "source_ast",
                        "source_ast.validate",
                        "Function constructor source captured at runtime",
                    )
                    .with_producer("source_ast.main"),
                );
            }
        } else if has_candidates {
            result.diagnostic_records.push(diag::warn_diag(
                diag::codes::source_ast::RUNTIME_VALIDATION_FAILED,
                "source_ast.validate",
                "AST strategy executed but produced no instrumented trace events",
            ));
        }
    }

    if matches!(kind, StrategyKind::SourceRegex) {
        result.observed_evidence.push(
            diag::EvidenceRecord::new(
                "source_regex_candidate",
                diag::EvidenceStrength::Weak,
                "source_regex",
                "source_regex.capture",
                "regex candidate matched statically",
            )
            .with_producer("source_regex.main"),
        );
        result.diagnostic_records.push(diag::warn_diag(
            diag::codes::source_ast::REGEX_PASS_THROUGH,
            "source_regex.capture",
            "regex fallback executed as pass-through",
        ));
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
        StrategyKind::BrowserifyBridge => &[
            "BrowserifyBridge uses source-text wrap and Function.prototype.call hook for module factory observation.",
        ],
        StrategyKind::CdpProbe => &[
            "CdpProbe is a placeholder — Chrome DevTools Protocol remote debugging integration is not yet implemented.",
        ],
        _ => &[],
    };

    for message in messages {
        let entry = format!("{}: {}", strategy_id, message);
        if !result.diagnostics.missing_capabilities.contains(&entry) {
            result.diagnostics.missing_capabilities.push(entry.clone());
        }
        result.diagnostic_records.push(diag::DiagnosticRecord::new(
            "ACT_STRATEGY_PARTIAL",
            diag::DiagnosticSeverity::Warn,
            "planned",
            message,
        ));
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
                !result.diagnostics.fallback_attempts.is_empty()
                    || result.diagnostics.activation_timing.is_some()
                    || !result.diagnostics.policy_constraints.is_empty()
                    || result.diagnostics.reload_reason.is_some()
                    || !result.diagnostic_records.is_empty() // runtime diagnostic output
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
        StrategyKind::BrowserifyBridge => {
            vec![TraceSourceKind::ModuleBridge, TraceSourceKind::RuntimeProxy]
        }
        StrategyKind::RollupBridge => vec![TraceSourceKind::TransparentHook],
        StrategyKind::ViteBridge => vec![TraceSourceKind::TransparentHook],
        StrategyKind::UmdBridge => vec![TraceSourceKind::TransparentHook],
        StrategyKind::RuntimeTransparent => vec![TraceSourceKind::TransparentHook],
        StrategyKind::RuntimeAggressive => vec![TraceSourceKind::RuntimeProxy],
        StrategyKind::SourceAst => vec![TraceSourceKind::SourceAst],
        StrategyKind::SourceRegex => vec![TraceSourceKind::SourceAst],
        StrategyKind::CdpProbe => vec![TraceSourceKind::Cdp],
        StrategyKind::ParcelBridge => vec![TraceSourceKind::TransparentHook],
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
