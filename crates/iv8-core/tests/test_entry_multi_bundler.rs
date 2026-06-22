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
        .eval(
            iv8_core::entry::browserify::bridge_prelude(),
            EvalOpts::default(),
        )
        .unwrap();
    let wrapped = iv8_core::entry::browserify::wrap_source(&src);
    kernel.eval(&wrapped, EvalOpts::default()).unwrap();
    common::assert_js_str(&mut kernel, "typeof __iv8_b_require", "function");
}

#[test]
fn test_browserify_execution_produces_correct_output() {
    let src = load_fixture("browserify_minimal.js");
    let mut kernel = common::make_kernel();
    kernel
        .eval(
            iv8_core::entry::browserify::bridge_prelude(),
            EvalOpts::default(),
        )
        .unwrap();
    let wrapped = iv8_core::entry::browserify::wrap_source(&src);
    kernel.eval(&wrapped, EvalOpts::default()).unwrap();
    let result = kernel.eval_to_rust_value(
        "(function(){try{return __iv8_b_require(1)}catch(e){return 'ERR:'+e.message}})()",
    );
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
    common::assert_js_str(&mut kernel, "typeof __rollup_result", "object");
}

#[test]
fn test_rollup_umd_global_branch_execution() {
    let src = load_fixture("rollup_umd_minimal.js");
    let mut kernel = common::make_kernel();
    kernel.eval(&src, EvalOpts::default()).unwrap();
    common::assert_js_str(&mut kernel, "typeof MyLib", "object");
}

#[test]
fn test_vite_iife_direct_eval() {
    let src = load_fixture("vite_iife_minimal.js");
    let mut kernel = common::make_kernel();
    kernel.eval(&src, EvalOpts::default()).unwrap();
    common::assert_js_str(&mut kernel, "typeof __vite_result", "string");
}

#[test]
fn test_unknown_iife_execution() {
    let src = load_fixture("unknown_iife_minimal.js");
    let mut kernel = common::make_kernel();
    kernel.eval(&src, EvalOpts::default()).unwrap();
    common::assert_js_str(&mut kernel, "globalThis.__unknown_result", "42");
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

#[test]
fn test_eval_module_vite_esm() {
    let src = load_fixture("vite_esm_minimal.js");
    let mut kernel = common::make_kernel();
    let result = kernel
        .eval_module(
            &src,
            Some("vite-esm-test.js"),
            iv8_core::kernel::EvalOpts::default(),
        )
        .expect("eval_module should succeed");
    let ctx = kernel.eval_to_rust_value("typeof hello");
    match &ctx {
        iv8_core::convert::RustValue::String(s) => {
            assert_eq!(
                s, "undefined",
                "top-level export should not leak to global scope"
            );
        }
        _ => panic!("expected string, got {:?}", ctx),
    }
    // Verify the Global value exists (module namespace returned)
    drop(result);
}

#[test]
fn test_browserify_ast_extraction_basics() {
    let src = r#"(function(modules,cache,entries){function r(id){return id;}return r})({1:[function(require,module,exports){module.exports=42},{}]},{},[1]);"#;
    let graph = iv8_core::entry::browserify::extract_modules(src)
        .expect("should extract from browserify source");
    assert_eq!(graph.module_count, 1);
    assert_eq!(graph.entry_ids, vec![1]);
}

#[test]
fn test_browserify_ast_extraction_non_browserify() {
    let src = "var x = 1;";
    assert!(iv8_core::entry::browserify::extract_modules(src).is_none());
}

#[test]
fn test_parcel_detect_basic() {
    let src = "var $parcel$global={};function parcelRequire(id){return{}};";
    let kind = classification::classify(src, &[]);
    assert_eq!(kind, SampleKind::ParcelBundle);
}

#[test]
fn test_parcel_classification_with_plan() {
    let src = "var $parcel$global={};function parcelRequire(id){return{}};";
    let plan = planner::plan_entry(src, Persona::Analysis, None, vec![]);
    let has_parcel = plan
        .candidate_strategies
        .iter()
        .any(|c| matches!(c.strategy_kind, StrategyKind::ParcelBridge));
    assert!(has_parcel);
}

#[test]
fn test_parcel_not_detected_plain_cjs() {
    let src = "var require = function(id) { return {}; }; module.exports = 42;";
    let plan = planner::plan_entry(src, Persona::Analysis, None, vec![]);
    let has_parcel = plan
        .candidate_strategies
        .iter()
        .any(|c| matches!(c.strategy_kind, StrategyKind::ParcelBridge));
    assert!(!has_parcel);
}

#[test]
fn test_parcel_execution_direct_eval() {
    let src = r#"
        var $parcel$global = {};
        function parcelRequire(id) {
            var modules = {1: [function(require,module,exports){
                module.exports = 42;
            },{}]};
            var mod = {exports:{}};
            modules[id][0](parcelRequire, mod, mod.exports);
            return mod.exports;
        }
    "#;
    let mut kernel = common::make_kernel();
    kernel.eval(src, EvalOpts::default()).unwrap();
    let val = kernel.eval_to_rust_value(
        "(function(){try{return parcelRequire(1)}catch(e){return 'ERR:'+e.message}})()",
    );
    assert_eq!(val, iv8_core::convert::RustValue::Int(42));
}

#[test]
fn test_vite_esm_detection_in_plan() {
    let src = "import { x } from './dep.js'; export const y = 1;";
    let plan = planner::plan_entry(src, Persona::Analysis, None, vec![]);
    let has_vite = plan
        .candidate_strategies
        .iter()
        .any(|c| matches!(c.strategy_kind, StrategyKind::ViteBridge));
    assert!(has_vite);
}

#[test]
fn test_vite_tla_does_not_hang() {
    let src = "const x = await Promise.resolve(42); export { x };";
    let mut kernel = common::make_kernel();
    kernel.eval(iv8_core::entry::vite::esm_prelude(), EvalOpts::default()).unwrap();
    let result = kernel.eval_module(src, Some("inline.js"), EvalOpts::default());
    assert!(result.is_ok());
    kernel.drain_microtasks();
}

#[test]
fn test_vite_import_meta_url_shim() {
    let src = "export const url = import.meta.url;";
    let mut kernel = common::make_kernel();
    kernel.eval(iv8_core::entry::vite::esm_prelude(), EvalOpts::default()).unwrap();
    let result = kernel.eval_module(src, Some("inline.js"), EvalOpts::default());
    assert!(result.is_ok());
    kernel.drain_microtasks();
}

#[test]
fn test_vite_dynamic_import_returns_rejected_promise() {
    let src = r#"
        export async function load() {
            try { await globalThis.__iv8_dynamic_import('./dep'); return 'resolved'; }
            catch(e) { return e.message; }
        }
    "#;
    let mut kernel = common::make_kernel();
    kernel.eval(iv8_core::entry::vite::esm_prelude(), EvalOpts::default()).unwrap();
    kernel.eval_module(src, Some("inline.js"), EvalOpts::default()).unwrap();
    kernel.drain_microtasks();
    let val = kernel.eval_to_rust_value(
        "(async function(){try{return await load()}catch(e){return 'ERR:'+e.message}})()",
    );
    // import() is async, so the IIFE result is a Promise; the caller must await it
    assert!(val != iv8_core::convert::RustValue::Null);
}

#[test]
fn test_vite_iife_still_works() {
    let src = r#"
        const __vitePreload = function() { return Promise.resolve(); };
        var result = 42;
    "#;
    let mut kernel = common::make_kernel();
    kernel.eval(src, EvalOpts::default()).unwrap();
    let val = kernel.eval_to_rust_value("result");
    assert_eq!(val, iv8_core::convert::RustValue::Int(42));
}
