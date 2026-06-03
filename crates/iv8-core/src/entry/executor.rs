//! Entry plan execution harness.
//!
//! Takes an `EntryPlan` produced by the planner, executes the JS source
//! within the V8 engine according to the selected strategy, and returns
//! an `EntryResult` with collected evidence.

use crate::entry::types::*;
use crate::entry::hooks;
use crate::kernel::embedded_v8::EmbeddedV8Kernel;
use crate::kernel::{EvalOpts, KernelConfig};

/// Execute a prepared entry plan and collect results.
///
/// This is the main execution entry point.
/// `chunks` are evaluated in order before the main `source`.
/// If `entry_expr` is provided, it is evaluated after the main source.
pub fn run_entry(
    plan: &EntryPlan,
    source: &str,
    chunks: &[String],
    entry_expr: Option<&str>,
) -> Result<EntryResult, String> {
    let mut kernel = EmbeddedV8Kernel::new(KernelConfig::default())
        .map_err(|e| format!("kernel init failed: {}", e))?;

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

    // Phase: prepared — apply strategy-specific setup
    result.final_state = PlanState::Prepared;
    if let Err(e) = apply_strategy_setup(&mut kernel, plan) {
        result.errors.push(ErrorEntry {
            code: "ACT_STRATEGY_SETUP_FAILED".into(),
            stage: "prepared".into(),
            message: format!("strategy setup failed: {}", e),
            strategy_id: Some(plan.selected_strategy.strategy_id.clone()),
            recoverable: true,
        });
        result.final_state = PlanState::Prepared;
        return Ok(result);
    }

    // Phase: armed — evaluate chunks first (in order), then main source
    result.final_state = PlanState::Armed;

    // Evaluate pre-requisite chunks
    for (i, chunk) in chunks.iter().enumerate() {
        if let Err(e) = kernel.eval(chunk, EvalOpts::default()) {
            result.warnings.push(ErrorEntry {
                code: "ACT_CHUNK_EVAL_FAILED".into(),
                stage: "armed".into(),
                message: format!("chunk[{}] eval failed: {}", i, e),
                strategy_id: Some(plan.selected_strategy.strategy_id.clone()),
                recoverable: true,
            });
        }
    }

    // Apply AST transform if strategy requires it
    let eval_source = match plan.selected_strategy.strategy_kind {
        StrategyKind::SourceAst => {
            let (transformed, diag) = crate::entry::ast::instrument(source);
            if let Some(d) = diag {
                result.warnings.push(ErrorEntry {
                    code: "ACT_AST_TRANSFORM_WARNING".into(),
                    stage: "armed".into(),
                    message: d,
                    strategy_id: Some(plan.selected_strategy.strategy_id.clone()),
                    recoverable: true,
                });
            }
            transformed
        }
        _ => source.to_string(),
    };

    match kernel.eval(&eval_source, EvalOpts::default()) {
        Ok(_) => {
            result.executed_strategies.push(ExecutedStrategy {
                strategy_id: plan.selected_strategy.strategy_id.clone(),
                phase_entered: PlanState::Armed,
                outcome: Outcome::Success,
            });
        }
        Err(e) => {
            result.errors.push(ErrorEntry {
                code: "ACT_SOURCE_EVAL_FAILED".into(),
                stage: "armed".into(),
                message: format!("source eval failed: {}", e),
                strategy_id: Some(plan.selected_strategy.strategy_id.clone()),
                recoverable: false,
            });
            result.final_state = PlanState::Armed;
            return Ok(result);
        }
    }

    // Phase: invoked — evaluate entry expression
    result.final_state = PlanState::Invoked;
    if let Some(expr) = entry_expr {
        match kernel.eval(expr, EvalOpts::default()) {
            Ok(_) => {}
            Err(e) => {
                result.errors.push(ErrorEntry {
                    code: "ACT_ENTRY_EXPR_FAILED".into(),
                    stage: "invoked".into(),
                    message: format!("entry expression failed: {}", e),
                    strategy_id: Some(plan.selected_strategy.strategy_id.clone()),
                    recoverable: false,
                });
                result.final_state = PlanState::Invoked;
                return Ok(result);
            }
        }
    }

    // Phase: collected — gather trace and evidence
    result.final_state = PlanState::Collected;
    collect_evidence(&mut kernel, &mut result, plan);

    // Phase: finalized — cleanup
    result.final_state = PlanState::Finalized;
    Ok(result)
}

// ───
// Strategy setup
// ───

fn apply_strategy_setup(
    kernel: &mut EmbeddedV8Kernel,
    plan: &EntryPlan,
) -> Result<(), String> {
    match plan.selected_strategy.strategy_kind {
        StrategyKind::SourceAst | StrategyKind::SourceRegex => {
            // For source-level strategies, the transform is applied before eval
            // (currently handled by the caller passing modified source).
            // No runtime setup needed.
            Ok(())
        }
        StrategyKind::WebpackBridge => {
            // Webpack bridge hooks are installed as JS prelude.
            // Observes module require, cache access, chunk ensure, and chunk load.
            let prelude = r#"
(function() {
    var __iv8_log = [];
    var wp = globalThis.__webpack_require__;
    if (typeof wp !== 'undefined') {
        // Hook chunk ensure (.e)
        if (typeof wp.e === 'function') {
            var origE = wp.e;
            wp.e = function(chunkId) {
                __iv8_log.push('chunk_ensure,' + chunkId);
                return origE.apply(this, arguments);
            };
        }
        // Hook chunk load (.l) — captures script URL
        if (typeof wp.l === 'function') {
            var origL = wp.l;
            wp.l = function(url, done, key, chunkId) {
                __iv8_log.push('chunk_load,' + (url || '') + ',' + (chunkId || ''));
                return origL.apply(this, arguments);
            };
        }
        // Hook module cache access (.c) — not a function, watch via defineProperty
        // Main require function
        var origRequire = wp;
        var __iv8_orig_require = wp;
        globalThis.__webpack_require__ = function(moduleId) {
            __iv8_log.push('require,' + moduleId);
            var result = __iv8_orig_require.apply(this, arguments);
            __iv8_log.push('require_result,' + moduleId + ',' + typeof result);
            return result;
        };
        // Copy back helpers to the new wrapper
        Object.keys(origRequire).forEach(function(k) {
            globalThis.__webpack_require__[k] = origRequire[k];
        });
    }
    globalThis.__iv8_webpack_log = __iv8_log;
})();
"#;
            kernel
                .eval(prelude, EvalOpts::default())
                .map_err(|e| format!("webpack bridge prelude: {}", e))?;
            Ok(())
        }
        StrategyKind::RuntimeTransparent => {
            let hook_js = crate::entry::hooks::transparent::prelude();
            kernel
                .eval(&hook_js, EvalOpts::default())
                .map_err(|e| format!("transparent hook: {}", e))?;
            Ok(())
        }
        StrategyKind::RuntimeAggressive => {
            let hook_js = crate::entry::hooks::aggressive::prelude();
            kernel
                .eval(&hook_js, EvalOpts::default())
                .map_err(|e| format!("aggressive hook: {}", e))?;
            Ok(())
        }
        StrategyKind::CdpProbe => {
            // CDP probe — minimal setup, relies on devtools connection
            // For now, just mark the probe as available
            Ok(())
        }
        StrategyKind::Dispatch => {
            // Dispatch hook setup — instrument the dispatcher
            // This would use instrument_source-like logic
            // For MVP, we note that dispatch is handled at source level
            Ok(())
        }
    }
}

// ───
// Evidence collection
// ───

fn collect_evidence(
    kernel: &mut EmbeddedV8Kernel,
    result: &mut EntryResult,
    plan: &EntryPlan,
) {
    // Collect trace if available
    let trace_val = kernel.eval_to_rust_value(
        "typeof __iv8_runtime_log !== 'undefined' ? __iv8_runtime_log : []",
    );
    if let crate::convert::RustValue::Array(items) = trace_val {
        for item in items {
            if let crate::convert::RustValue::String(s) = item {
                result.trace.push(s);
            }
        }
    }

    // Collect webpack module log if available
    if matches!(plan.selected_strategy.strategy_kind, StrategyKind::WebpackBridge) {
        let log_val = kernel.eval_to_rust_value(
            "typeof __iv8_webpack_log !== 'undefined' ? __iv8_webpack_log : []",
        );
        if let crate::convert::RustValue::Array(items) = log_val {
            let mut requires = Vec::new();
            let mut chunks = Vec::new();
            for item in items {
                if let crate::convert::RustValue::String(s) = item {
                    if s.starts_with("require,") {
                        requires.push(s);
                    } else if s.starts_with("chunk_") {
                        chunks.push(s);
                    }
                }
            }
            let mut graph = serde_json::Map::new();
            graph.insert("require_calls".into(), serde_json::json!(requires));
            graph.insert("chunk_events".into(), serde_json::json!(chunks));
            graph.insert("require_count".into(), serde_json::json!(requires.len()));
            graph.insert("chunk_count".into(), serde_json::json!(chunks.len()));
            result.module_graph = Some(serde_json::Value::Object(graph));
        }
    }

    // Collect hook report (strategy-specific)
    let trace_entries: Vec<String> = result.trace.iter().map(|s| s.to_string()).collect();
    result.hook_report = match plan.selected_strategy.strategy_kind {
        StrategyKind::RuntimeTransparent => {
            Some(hooks::transparent::collect(&trace_entries))
        }
        StrategyKind::RuntimeAggressive => {
            Some(hooks::aggressive::collect(&trace_entries))
        }
        _ => {
            Some(serde_json::json!({
                "strategy_id": plan.selected_strategy.strategy_id,
                "strategy_kind": format!("{:?}", plan.selected_strategy.strategy_kind),
            }))
        }
    };

    // Assemble trace_meta
    let trace_sources = derive_trace_sources(&plan.selected_strategy.strategy_kind);
    let executed_ids: Vec<String> = result.executed_strategies
        .iter()
        .map(|e| e.strategy_id.clone())
        .collect();

    // Build event-level metadata for trace entries that have recognizable prefixes
    let mut events = std::collections::HashMap::new();
    for (idx, entry) in result.trace.iter().enumerate() {
        let (source_kind, confidence) = classify_trace_entry(entry, &plan.selected_strategy.strategy_kind);
        if let Some(sk) = source_kind {
            events.insert(idx, EventMeta {
                source_kind: sk,
                strategy_id: plan.selected_strategy.strategy_id.clone(),
                phase: "invoked".to_string(),
                confidence,
                module_id: None,
                chunk_id: None,
            });
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

/// Map strategy kind to trace source categories.
fn derive_trace_sources(kind: &StrategyKind) -> Vec<TraceSourceKind> {
    match kind {
        StrategyKind::Dispatch => vec![TraceSourceKind::Dispatch],
        StrategyKind::WebpackBridge => vec![TraceSourceKind::ModuleBridge, TraceSourceKind::RuntimeProxy],
        StrategyKind::RuntimeTransparent => vec![TraceSourceKind::TransparentHook],
        StrategyKind::RuntimeAggressive => vec![TraceSourceKind::RuntimeProxy],
        StrategyKind::SourceAst => vec![TraceSourceKind::SourceAst],
        StrategyKind::SourceRegex => vec![TraceSourceKind::SourceAst],
        StrategyKind::CdpProbe => vec![TraceSourceKind::Cdp],
    }
}

/// Best-effort classification of a raw trace entry string.
/// Returns (source_kind, confidence) or (None, _) if unrecognized.
fn classify_trace_entry(entry: &str, _strategy: &StrategyKind) -> (Option<TraceSourceKind>, String) {
    if entry.starts_with("D,") {
        return (Some(TraceSourceKind::Dispatch), "high".into());
    }
    if entry.starts_with("R,") || entry.starts_with("W,") {
        return (Some(TraceSourceKind::TransparentHook), "medium".into());
    }
    if entry.starts_with("C,") {
        return (Some(TraceSourceKind::TransparentHook), "medium".into());
    }
    if entry.starts_with("eval,") || entry.starts_with("fn_ctor,") {
        return (Some(TraceSourceKind::TransparentHook), "medium".into());
    }
    if entry.starts_with("aggressive_") {
        return (Some(TraceSourceKind::RuntimeProxy), "high".into());
    }
    if entry.starts_with("require,") || entry.starts_with("chunk_") {
        return (Some(TraceSourceKind::ModuleBridge), "high".into());
    }
    if entry.starts_with("env_read,") {
        return (Some(TraceSourceKind::TransparentHook), "low".into());
    }
    (None, "unknown".into())
}
