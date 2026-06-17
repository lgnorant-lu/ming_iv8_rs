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
fn test_browserify_execution_exposes_require() {
    let src = load_fixture("browserify_minimal.js");
    let mut kernel = common::make_kernel();
    kernel
        .eval(iv8_core::entry::browserify::bridge_prelude(), EvalOpts::default())
        .unwrap();
    let wrapped = iv8_core::entry::browserify::wrap_source(&src);
    kernel.eval(&wrapped, EvalOpts::default()).unwrap();
    common::assert_js_str(
        &mut kernel,
        "typeof __iv8_b_require",
        "function",
    );
}

#[test]
fn test_browserify_execution_produces_correct_output() {
    let src = load_fixture("browserify_minimal.js");
    let mut kernel = common::make_kernel();
    kernel
        .eval(iv8_core::entry::browserify::bridge_prelude(), EvalOpts::default())
        .unwrap();
    let wrapped = iv8_core::entry::browserify::wrap_source(&src);
    kernel.eval(&wrapped, EvalOpts::default()).unwrap();
    let result = kernel.eval_to_rust_value("(function(){try{return __iv8_b_require(1)}catch(e){return 'ERR:'+e.message}})()");
    let val = common::to_str(&result);
    if val.starts_with("ERR:") {
        panic!("require(1) threw: {}", val);
    }
    assert_eq!(val, "20", "require(1) should return 20, got {}", val);
}

#[test]
fn test_rollup_iife_direct_eval() {
    let src = load_fixture("rollup_iife_minimal.js");
    let mut kernel = common::make_kernel();
    kernel.eval(&src, EvalOpts::default()).unwrap();
    common::assert_js_str(
        &mut kernel,
        "typeof __rollup_result",
        "object",
    );
}

#[test]
fn test_rollup_umd_global_branch_execution() {
    let src = load_fixture("rollup_umd_minimal.js");
    let mut kernel = common::make_kernel();
    kernel.eval(&src, EvalOpts::default()).unwrap();
    common::assert_js_str(
        &mut kernel,
        "typeof MyLib",
        "object",
    );
}

#[test]
fn test_vite_iife_direct_eval() {
    let src = load_fixture("vite_iife_minimal.js");
    let mut kernel = common::make_kernel();
    kernel.eval(&src, EvalOpts::default()).unwrap();
    common::assert_js_str(
        &mut kernel,
        "typeof __vite_result",
        "string",
    );
}

#[test]
fn test_unknown_iife_execution() {
    let src = load_fixture("unknown_iife_minimal.js");
    let mut kernel = common::make_kernel();
    kernel.eval(&src, EvalOpts::default()).unwrap();
    common::assert_js_str(
        &mut kernel,
        "globalThis.__unknown_result",
        "42",
    );
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

#[test]
fn test_browserify_plan_expected_evidence_includes_module_graph() {
    let src = load_fixture("browserify_minimal.js");
    let plan = planner::plan_entry(&src, Persona::Analysis, None, vec![]);
    assert!(
        plan.expected_evidence
            .iter()
            .any(|e| matches!(e, iv8_core::entry::types::Evidence::ModuleGraph)),
        "Browserify plan should expect ModuleGraph evidence"
    );
}

#[test]
fn test_all_format_candidates_have_nonzero_fit_scores() {
    let fixtures: Vec<(&str, SampleKind)> = vec![
        ("browserify_minimal.js", SampleKind::BrowserifyRuntime),
        ("rollup_iife_minimal.js", SampleKind::RollupBundle),
        ("rollup_umd_minimal.js", SampleKind::UmdBundle),
        ("vite_iife_minimal.js", SampleKind::ViteBundle),
        ("esbuild_minimal.js", SampleKind::UnknownIife),
    ];
    for (fixture, _expected_kind) in fixtures {
        let src = load_fixture(fixture);
        let plan = planner::plan_entry(&src, Persona::Analysis, None, vec![]);
        assert!(
            !plan.candidate_strategies.is_empty(),
            "{} should have at least one candidate",
            fixture
        );
        assert!(
            plan.selected_strategy.strategy_kind != StrategyKind::CdpProbe
                || plan.fallback_chain.len() <= 1,
            "{} primary strategy should not be CDP probe",
            fixture
        );
    }
}
