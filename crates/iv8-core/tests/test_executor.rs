//! Entry executor integration tests.
//!
//! Tests run_entry with plans from the planner to cover executor.rs logic.

mod common;

use iv8_core::entry::{executor, planner, types::*};

fn make_plan(source: &str) -> EntryPlan {
    let persona = Persona::Runtime;
    let policy = Policy {
        hook_level: HookLevel::None,
        source_rewrite: SourceRewrite::Disabled,
        preload_requirement: PreloadRequirement::BestEffort,
        allow_reload: false,
        trace_level: TraceLevel::Off,
        diagnostics_level: DiagnosticsLevel::Off,
        trace_sources: None,
        descriptor_preservation: DescriptorPreservation::BestEffort,
        preserve_native_tostring: true,
        forbid_proxy_on_sensitive_surfaces: true,
        allow_prototype_patch: false,
        allow_function_intrinsic_patch: false,
        cleanup_mode: CleanupMode::None,
    };
    planner::plan_entry(source, persona, Some(policy), vec![])
}

#[test]
fn test_run_entry_simple_eval() {
    let plan = make_plan("1 + 1");
    let result = executor::run_entry(&plan, "1 + 1", &[], None);
    assert!(result.is_ok(), "run_entry should succeed for simple eval");
    let result = result.unwrap();
    assert_eq!(result.final_state, PlanState::Finalized);
}

#[test]
fn test_run_entry_with_chunks() {
    let plan = make_plan("add(2, 3)");
    let chunks = vec!["function add(a, b) { return a + b; }".to_string()];
    let result = executor::run_entry(&plan, "add(2, 3)", &chunks, None);
    assert!(result.is_ok());
}

#[test]
fn test_run_entry_with_entry_expr() {
    let plan = make_plan("var x = 42");
    let result = executor::run_entry(&plan, "var x = 42", &[], Some("x"));
    assert!(result.is_ok());
}

#[test]
fn test_run_entry_syntax_error() {
    let plan = make_plan("{{{");
    let result = executor::run_entry(&plan, "{{{", &[], None);
    match result {
        Ok(r) => assert_ne!(r.final_state, PlanState::Finalized, "syntax error should not finalize"),
        Err(_) => {}
    }
}

#[test]
fn test_run_entry_empty_source() {
    let plan = make_plan("");
    let result = executor::run_entry(&plan, "", &[], None);
    assert!(result.is_ok());
}

#[test]
fn test_run_entry_records_executed_strategy() {
    let plan = make_plan("1");
    let result = executor::run_entry(&plan, "1", &[], None).unwrap();
    assert!(!result.executed_strategies.is_empty(), "should record executed strategy");
}
