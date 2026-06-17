mod common;

use iv8_core::entry::classification;
use iv8_core::entry::planner;
use iv8_core::entry::types::Persona;
use iv8_core::entry::types::PlanState;
use iv8_core::entry::types::SampleKind;
use iv8_core::entry::types::StrategyKind;
use iv8_core::kernel::EvalOpts;

const FIXTURES: &str = "tests/fixtures/multi_bundler";

fn load_fixture(name: &str) -> String {
    let path = format!("{}/{}", FIXTURES, name);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("failed to read fixture {}: {}", path, e))
}

#[test]
fn test_classify_browserify_fixture() {
    let src = load_fixture("browserify_minimal.js");
    let kind = classification::classify(&src, &[]);
    assert_eq!(kind, SampleKind::BrowserifyRuntime);
}

#[test]
fn test_classify_rollup_iife_fixture() {
    let src = load_fixture("rollup_iife_minimal.js");
    let kind = classification::classify(&src, &[]);
    assert_eq!(kind, SampleKind::RollupBundle);
}

#[test]
fn test_classify_rollup_umd_fixture() {
    let src = load_fixture("rollup_umd_minimal.js");
    let kind = classification::classify(&src, &[]);
    assert_eq!(kind, SampleKind::UmdBundle);
}

#[test]
fn test_classify_vite_fixture() {
    let src = load_fixture("vite_iife_minimal.js");
    let kind = classification::classify(&src, &[]);
    assert_eq!(kind, SampleKind::ViteBundle);
}

#[test]
fn test_classify_esbuild_fixture() {
    let src = load_fixture("esbuild_minimal.js");
    let kind = classification::classify(&src, &[]);
    assert_eq!(kind, SampleKind::UnknownIife);
}

#[test]
fn test_classify_unknown_iife_fixture() {
    let src = load_fixture("unknown_iife_minimal.js");
    let kind = classification::classify(&src, &[]);
    assert_eq!(kind, SampleKind::UnknownIife);
}

#[test]
fn test_plan_browserify_routes_to_browserify_bridge() {
    let src = load_fixture("browserify_minimal.js");
    let plan = planner::plan_entry(&src, Persona::Analysis, None, vec![]);
    assert_eq!(plan.sample_kind, SampleKind::BrowserifyRuntime);
    assert_eq!(
        plan.selected_strategy.strategy_kind,
        StrategyKind::BrowserifyBridge
    );
}

#[test]
fn test_plan_rollup_iife_routes_to_rollup_bridge() {
    let src = load_fixture("rollup_iife_minimal.js");
    let plan = planner::plan_entry(&src, Persona::Analysis, None, vec![]);
    assert_eq!(plan.sample_kind, SampleKind::RollupBundle);
    assert_eq!(
        plan.selected_strategy.strategy_kind,
        StrategyKind::RollupBridge
    );
}

#[test]
fn test_plan_rollup_umd_routes_to_umd_bridge() {
    let src = load_fixture("rollup_umd_minimal.js");
    let plan = planner::plan_entry(&src, Persona::Analysis, None, vec![]);
    assert_eq!(plan.sample_kind, SampleKind::UmdBundle);
    assert_eq!(
        plan.selected_strategy.strategy_kind,
        StrategyKind::UmdBridge
    );
}

#[test]
fn test_plan_vite_routes_to_vite_bridge() {
    let src = load_fixture("vite_iife_minimal.js");
    let plan = planner::plan_entry(&src, Persona::Analysis, None, vec![]);
    assert_eq!(plan.sample_kind, SampleKind::ViteBundle);
    assert_eq!(
        plan.selected_strategy.strategy_kind,
        StrategyKind::ViteBridge
    );
}

#[test]
fn test_plan_unknown_iife_routes_to_runtime_transparent() {
    let src = load_fixture("esbuild_minimal.js");
    let plan = planner::plan_entry(&src, Persona::Analysis, None, vec![]);
    assert_eq!(plan.sample_kind, SampleKind::UnknownIife);
    assert_eq!(
        plan.selected_strategy.strategy_kind,
        StrategyKind::RuntimeTransparent
    );
}

#[test]
fn test_browserify_execution_source_text_wrap() {
    let src = load_fixture("browserify_minimal.js");
    let mut kernel = common::make_kernel();
    kernel
        .eval(iv8_core::entry::browserify::bridge_prelude(), EvalOpts::default())
        .unwrap();
    kernel.eval(&src, EvalOpts::default()).unwrap();
    let has_require = kernel.eval_to_rust_value("typeof __iv8_b_require");
    let detected = match &has_require {
        iv8_core::convert::RustValue::String(s) => s != "undefined",
        _ => false,
    };
    assert!(detected, "Browserify require not detected after source-text wrap");
}

#[test]
fn test_rollup_iife_direct_eval() {
    let src = load_fixture("rollup_iife_minimal.js");
    let mut kernel = common::make_kernel();
    kernel.eval(&src, EvalOpts::default()).unwrap();
    let result = kernel.eval_to_rust_value("typeof __rollup_result");
    match &result {
        iv8_core::convert::RustValue::String(s) => {
            assert_ne!(s, "undefined", "Rollup IIFE result should be defined");
        }
        _ => panic!("expected string typeof, got {:?}", result),
    };
}

#[test]
fn test_rollup_umd_global_branch_execution() {
    let src = load_fixture("rollup_umd_minimal.js");
    let mut kernel = common::make_kernel();
    kernel.eval(&src, EvalOpts::default()).unwrap();
    let result = kernel.eval_to_rust_value("typeof MyLib");
    match &result {
        iv8_core::convert::RustValue::String(s) => {
            assert_ne!(s, "undefined", "UMD global branch should expose MyLib");
        }
        _ => panic!("expected string typeof, got {:?}", result),
    };
}

#[test]
fn test_vite_iife_direct_eval() {
    let src = load_fixture("vite_iife_minimal.js");
    let mut kernel = common::make_kernel();
    kernel.eval(&src, EvalOpts::default()).unwrap();
    let result = kernel.eval_to_rust_value("typeof __vite_result");
    match &result {
        iv8_core::convert::RustValue::String(s) => {
            assert_ne!(s, "undefined", "Vite IIFE result should be defined");
        }
        _ => panic!("expected string typeof, got {:?}", result),
    };
}

#[test]
fn test_unknown_iife_execution() {
    let src = load_fixture("unknown_iife_minimal.js");
    let mut kernel = common::make_kernel();
    kernel.eval(&src, EvalOpts::default()).unwrap();
    let result = kernel.eval_to_rust_value("globalThis.__unknown_result");
    match &result {
        iv8_core::convert::RustValue::Int(n) => {
            assert_eq!(*n, 42, "Unknown IIFE should produce 42");
        }
        _ => panic!("expected int 42, got {:?}", result),
    };
}

#[test]
fn test_webpack_still_detected_after_multi_bundler_regression() {
    let src = "__webpack_require__(1); var __webpack_require__ = function(){};";
    let kind = classification::classify(src, &[]);
    assert_eq!(kind, SampleKind::WebpackRuntime);
}

#[test]
fn test_plan_entry_state_is_planned() {
    let src = load_fixture("browserify_minimal.js");
    let plan = planner::plan_entry(&src, Persona::Analysis, None, vec![]);
    assert_eq!(plan.state, PlanState::Planned);
    assert!(!plan.fallback_chain.is_empty());
}

#[test]
fn test_fallback_chain_includes_cdp_probe() {
    let src = load_fixture("esbuild_minimal.js");
    let plan = planner::plan_entry(&src, Persona::Analysis, None, vec![]);
    assert!(
        plan.fallback_chain.iter().any(|s| s.contains("cdp_probe")),
        "fallback chain should include CDP probe as last resort"
    );
}
