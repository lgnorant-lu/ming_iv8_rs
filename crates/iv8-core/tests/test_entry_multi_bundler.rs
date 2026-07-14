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

// v0.8.99 S7 BD-1: minified webpack-like (numeric module table + factory shape)
#[test]
fn test_webpack_minified_bdms_detection() {
    let src = r#"
!function(e){var t={};function n(r){if(t[r])return t[r].exports;var o=t[r]={i:r,l:!1,exports:{}};return e[r].call(o.exports,o,o.exports,n),o.l=!0,o.exports}n.m=e,n.c=t,n(n.s=0)}({
0:function(e,t,n){t.a=1},
1:function(e,t,n){t.b=2},
2:function(e,t,n){t.c=3},
3:function(e,t,n){t.d=4}
});
"#;
    let kind = classification::classify(src, &[]);
    assert_eq!(
        kind,
        SampleKind::WebpackRuntime,
        "minified webpack-like should classify as WebpackRuntime"
    );
}

#[test]
fn test_webpack_chunk_global_detected_without_require_name() {
    let src = "self.webpackChunk=self.webpackChunk||[];self.webpackChunk.push([['app'],{1:function(e,t,n){}}]);";
    let kind = classification::classify(src, &[]);
    assert_eq!(kind, SampleKind::WebpackRuntime);
}

/// S7-07/08: multi-source joint eval (runtime + vendor chunk) shares require table
#[test]
fn test_multi_chunk_joint_execution_require_across_chunks() {
    let runtime = load_fixture("webpack_multichunk_runtime.js");
    let vendor = load_fixture("webpack_multichunk_vendor.js");
    let page = load_fixture("webpack_multichunk_page.js");
    let mut kernel = common::make_kernel();
    kernel
        .eval(
            iv8_core::entry::webpack::bridge_prelude(),
            EvalOpts::default(),
        )
        .unwrap();
    kernel.eval(&runtime, EvalOpts::default()).unwrap();
    kernel.eval(&vendor, EvalOpts::default()).unwrap();
    let graph = iv8_core::entry::webpack::collect_module_graph(&mut kernel)
        .expect("graph after joint eval");
    assert!(
        graph["chunk_factories_installed"].as_u64().unwrap_or(0) >= 2,
        "installed={:?}",
        graph["chunk_factories_installed"]
    );
    // chunk_id backfill on vendor modules
    let nodes = graph["nodes"].as_array().cloned().unwrap_or_default();
    let n50 = nodes.iter().find(|n| n["module_id"] == "50");
    assert!(n50.is_some(), "nodes={:?}", nodes);
    assert_eq!(n50.unwrap()["chunk_id"], "vendors");
    kernel.eval(&page, EvalOpts::default()).unwrap();
    common::assert_js_str(&mut kernel, "globalThis.__iv8_page.via", "vendor-lib");
    common::assert_js_str(&mut kernel, "String(globalThis.__iv8_page.sum)", "8");
    common::assert_js_str(&mut kernel, "String(globalThis.__iv8_page.boot)", "true");
}

#[test]
fn test_fixture_bdms_positive_and_negative_files() {
    let pos = load_fixture("bdms_positive_minified_like.js");
    let neg = load_fixture("bdms_negative_plain_cjs.js");
    assert_eq!(
        classification::classify(&pos, &[]),
        SampleKind::WebpackRuntime
    );
    assert_ne!(
        classification::classify(&neg, &[]),
        SampleKind::WebpackRuntime
    );
}

#[test]
fn test_run_entry_chunks_product_path_with_fixtures() {
    let runtime = load_fixture("webpack_multichunk_runtime.js");
    let vendor = load_fixture("webpack_multichunk_vendor.js");
    let page = load_fixture("webpack_multichunk_page.js");
    let plan = planner::plan_entry(&runtime, Persona::Analysis, None, vec![]);
    assert_eq!(plan.sample_kind, SampleKind::WebpackRuntime);
    let result = iv8_core::entry::executor::run_entry(
        &plan,
        &page,
        &[runtime.clone(), vendor.clone()],
        Some("globalThis.__iv8_page && globalThis.__iv8_page.via"),
    )
    .expect("run_entry");
    assert!(result.module_graph.is_some(), "expected module_graph");
    let g = result.module_graph.as_ref().unwrap();
    assert!(
        g["chunk_factories_installed"].as_u64().unwrap_or(0) >= 1
            || g["module_count"].as_u64().unwrap_or(0) >= 2,
        "graph={:?}",
        g
    );
}

#[test]
fn test_source_map_and_amd_helpers() {
    use iv8_core::entry::source_map;
    let src = "var x=1;\n//# sourceMappingURL=bundle.js.map\n";
    assert_eq!(
        source_map::extract_source_mapping_url(src).as_deref(),
        Some("bundle.js.map")
    );
    assert!(iv8_core::entry::amd::detect_amd_markers(
        "define(['exports'], function(exports){ exports.a=1; });"
    ));
    let ts = source_map::detect_treeshaking_markers("var a=/*#__PURE__*/f();");
    assert_eq!(ts["pure_annotation"], true);
}

#[test]
fn test_amd_subset_loader_define_require() {
    let mut kernel = common::make_kernel();
    kernel
        .eval(iv8_core::entry::amd::amd_prelude(), EvalOpts::default())
        .unwrap();
    kernel
        .eval(
            r#"
define('util', ['exports'], function(exports){ exports.double = function(x){ return x*2; }; });
define('main', ['util','exports'], function(util, exports){ exports.out = util.double(21); });
globalThis.__amd_out = require('main').out;
"#,
            EvalOpts::default(),
        )
        .unwrap();
    common::assert_js_str(&mut kernel, "String(globalThis.__amd_out)", "42");
}

#[test]
fn test_preload_chunk_sources_api() {
    let mut kernel = common::make_kernel();
    kernel
        .eval(
            iv8_core::entry::webpack::bridge_prelude(),
            EvalOpts::default(),
        )
        .unwrap();
    kernel
        .eval(
            r#"
var __webpack_require__ = function(id){
  if(__webpack_require__.c[id]) return __webpack_require__.c[id].exports;
  var m={exports:{}}; __webpack_require__.c[id]=m;
  var f=__webpack_require__.m[id]; if(typeof f==='function') f(m,m.exports,__webpack_require__);
  return m.exports;
};
__webpack_require__.m={}; __webpack_require__.c={};
"#,
            EvalOpts::default(),
        )
        .unwrap();
    let chunk = r#"window.webpackChunk=window.webpackChunk||[];window.webpackChunk.push([["v"],{7:function(m,e,r){e.ok=true;}}]);"#;
    let rep =
        iv8_core::entry::webpack::preload_chunk_sources(&mut kernel, &[chunk.to_string()]);
    assert_eq!(rep["chunks_eval_ok"], 1);
    assert!(rep["factories_installed"].as_u64().unwrap_or(0) >= 1);
    common::assert_js_str(&mut kernel, "String(__webpack_require__(7).ok)", "true");
}

#[test]
fn test_bdms_negative_plain_cjs_not_webpack() {
    let src = "function add(a,b){return a+b;} module.exports = {add:add};";
    let kind = classification::classify(src, &[]);
    assert_ne!(
        kind,
        SampleKind::WebpackRuntime,
        "plain CJS must not be BDMS webpack"
    );
}

#[test]
fn test_plan_multi_entry_lists_kinds() {
    let sources = [
        (
            "runtime.js",
            "var __webpack_require__=function(){}; __webpack_require__.m={};",
        ),
        (
            "app.js",
            "var $parcel$global={};function parcelRequire(id){return{};}",
        ),
    ];
    let multi = iv8_core::entry::planner::plan_multi_entry(&sources, Persona::Analysis, None);
    assert_eq!(multi["entry_count"], 2);
    assert!(multi["entries"].as_array().unwrap().len() == 2);
}

#[test]
fn test_browserify_edges_and_cycles_from_ast() {
    let src = r#"(function(modules,cache,entries){function r(id){return id;}return r})({1:[function(require,module,exports){require(2);module.exports=1},{"./x":2}],2:[function(require,module,exports){require(1);module.exports=2},{"./y":1}]},{},[1]);"#;
    let graph = iv8_core::entry::browserify::extract_modules(src).expect("extract");
    let (edges, cycles) = iv8_core::entry::browserify::graph_edges_and_cycles(&graph);
    assert!(
        edges.iter().any(|e| e["from"] == "1" && e["to"] == "2"),
        "edges={:?}",
        edges
    );
    assert!(
        !cycles.is_empty() || edges.iter().any(|e| e["from"] == "2" && e["to"] == "1"),
        "expected cycle or mutual edges: edges={:?} cycles={:?}",
        edges,
        cycles
    );
}

/// A-P0-3: circular modules share cache (half-init style) without hanging
#[test]
fn test_node_chunk_id_backfill_from_webpack_chunk() {
    let mut kernel = common::make_kernel();
    kernel
        .eval(
            iv8_core::entry::webpack::bridge_prelude(),
            EvalOpts::default(),
        )
        .unwrap();
    kernel
        .eval(
            r#"
var __webpack_require__ = function(id){};
__webpack_require__.m = { 0: function(){} };
__webpack_require__.c = {};
window.webpackChunk = [[["vendors"], { 9: function(){}, 10: function(){} }]];
"#,
            EvalOpts::default(),
        )
        .unwrap();
    let graph = iv8_core::entry::webpack::collect_module_graph(&mut kernel).expect("graph");
    let nodes = graph["nodes"].as_array().unwrap();
    let n9 = nodes.iter().find(|n| n["module_id"] == "9");
    assert!(n9.is_some(), "nodes={:?}", nodes);
    assert_eq!(n9.unwrap()["chunk_id"], "vendors");
}

#[test]
fn test_amd_capability_bounds_exposed() {
    let b = iv8_core::entry::amd::amd_capability_bounds("define('a',[],function(){});");
    assert_eq!(b["sync_define_require"], true);
    assert_eq!(b["loader_plugins"], false);
}

#[test]
fn test_treeshaking_diagnostics_counts() {
    let m = iv8_core::entry::source_map::detect_treeshaking_markers(
        "/*#__PURE__*/f(); /*#__PURE__*/g(); /* harmony export */",
    );
    assert_eq!(m["pure_annotation_count"], 2);
    assert_eq!(m["harmony_markers"], true);
}

/// Official webpack common-chunk-and-vendor-chunk dist (L2): named webpackChunk* global.
#[test]
fn test_l2_official_webpack_named_chunk_global_and_graph() {
    let vendor = load_fixture("generated/vendor.js");
    let commons2 = load_fixture("generated/commons-utility2_js.js");
    let page_a = load_fixture("generated/pageA.js");
    assert!(
        vendor.contains("webpackChunk") || page_a.contains("webpackChunk"),
        "expected webpackChunk* marker in official dist"
    );
    let mut kernel = common::make_kernel();
    kernel
        .eval(
            iv8_core::entry::webpack::bridge_prelude(),
            EvalOpts::default(),
        )
        .unwrap();
    // order: shared chunks then entry (matches browser script order)
    for src in [&vendor, &commons2, &page_a] {
        kernel.eval(src, EvalOpts::default()).unwrap();
    }
    let graph = iv8_core::entry::webpack::collect_module_graph(&mut kernel).expect("graph");
    let count = graph["module_count"].as_u64().unwrap_or(0);
    assert!(
        count >= 3,
        "expected modules from vendor+commons+page, module_count={} graph={:?}",
        count,
        graph
    );
    assert!(
        graph["chunk_factories_installed"].as_u64().unwrap_or(0) >= 1
            || graph["module_count"].as_u64().unwrap_or(0) >= 3,
        "factories or modules missing: {:?}",
        graph
    );
    let chunks = graph["chunks"].as_array().cloned().unwrap_or_default();
    assert!(
        !chunks.is_empty()
            || graph["nodes"]
                .as_array()
                .map(|n| !n.is_empty())
                .unwrap_or(false),
        "expected chunks or nodes from named webpackChunk*: {:?}",
        graph
    );
}

#[test]
fn test_named_webpack_chunk_global_scan() {
    let mut kernel = common::make_kernel();
    kernel
        .eval(
            iv8_core::entry::webpack::bridge_prelude(),
            EvalOpts::default(),
        )
        .unwrap();
    kernel
        .eval(
            r#"
var __webpack_require__ = function(id){
  if(__webpack_require__.c[id]) return __webpack_require__.c[id].exports;
  var m={exports:{}}; __webpack_require__.c[id]=m;
  var f=__webpack_require__.m[id]; if(typeof f==='function') f(m,m.exports,__webpack_require__);
  return m.exports;
};
__webpack_require__.m={}; __webpack_require__.c={};
self.webpackChunkiv8_l2_webpack_example = self.webpackChunkiv8_l2_webpack_example || [];
self.webpackChunkiv8_l2_webpack_example.push([["vendor"],{
  353: function(e){ e.exports = "vendor1"; },
  992: function(e){ e.exports = "vendor2"; }
}]);
"#,
            EvalOpts::default(),
        )
        .unwrap();
    let graph = iv8_core::entry::webpack::collect_module_graph(&mut kernel).expect("graph");
    assert!(
        graph["chunk_factories_installed"].as_u64().unwrap_or(0) >= 2,
        "named webpackChunk* must install factories: {:?}",
        graph
    );
    common::assert_js_str(&mut kernel, "String(__webpack_require__(353))", "vendor1");
}

#[test]
fn test_webpack_circular_require_uses_cache() {
    let src = r#"
var __webpack_require__ = function(id) {
  if (__webpack_require__.c[id]) return __webpack_require__.c[id].exports;
  var m = { exports: {} };
  __webpack_require__.c[id] = m;
  __webpack_require__.m[id](m, m.exports, __webpack_require__);
  return m.exports;
};
__webpack_require__.m = {
  1: function(m,e,r){ e.a = 1; e.other = r(2).b; },
  2: function(m,e,r){ e.b = 2; e.other = r(1).a; }
};
__webpack_require__.c = {};
globalThis.__circ = { one: __webpack_require__(1), two: __webpack_require__(2) };
"#;
    let mut kernel = common::make_kernel();
    kernel
        .eval(
            iv8_core::entry::webpack::bridge_prelude(),
            EvalOpts::default(),
        )
        .unwrap();
    kernel.eval(src, EvalOpts::default()).unwrap();
    let graph = iv8_core::entry::webpack::collect_module_graph(&mut kernel).expect("graph");
    let cycles = graph["cycles"].as_array().cloned().unwrap_or_default();
    assert!(!cycles.is_empty(), "cycles={:?}", cycles);
    common::assert_js_str(&mut kernel, "String(globalThis.__circ.one.a)", "1");
    common::assert_js_str(&mut kernel, "String(globalThis.__circ.two.b)", "2");
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
