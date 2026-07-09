from __future__ import annotations

from iv8_rs import prepare_entry, run_with_entry


def plan_and_run(source: str, persona: str = "analysis", entry_expr: str | None = None) -> tuple[dict, dict]:
    plan = prepare_entry(source, persona=persona)
    result = run_with_entry(plan, source, entry_expr=entry_expr)
    return plan, result

def test_webpack4_runtime_captured():
    """Minimal webpack4 runtime -> module table and require captured."""
    source = """
    (function(modules) {
        var installedModules = {};
        function __webpack_require__(moduleId) {
            if(installedModules[moduleId]) return installedModules[moduleId].exports;
            var module = installedModules[moduleId] = { i: moduleId, l: false, exports: {} };
            modules[moduleId].call(module.exports, module, module.exports, __webpack_require__);
            module.l = true;
            return module.exports;
        }
        __webpack_require__.m = modules;
        __webpack_require__.c = installedModules;
        // Expose globally so WebpackBridge can capture after init
        globalThis.__webpack_require__ = __webpack_require__;
        return __webpack_require__(0);
    })({
        0: function(module, exports, __webpack_require__) {
            globalThis.entryRun = true;
        },
        7: function(module, exports, __webpack_require__) {
            globalThis.signRun = true;
        }
    });
    """
    plan, result = plan_and_run(source)
    assert plan["sample_kind"] == "webpack_runtime"
    assert plan["selected_strategy"]["strategy_kind"] == "webpack_bridge"

    # Verify module graph schema
    graph = result["module_graph"]
    assert graph is not None
    assert graph["schema_version"] == "module-graph.v0.1"
    assert graph["runtime_family"] == "webpack_like"
    assert graph["runtime_flavor"] in ("webpack4", "webpack5", "unknown_webpack_like")
    assert graph["module_count"] == 2
    assert graph["entry_module_id"] == "0"

    # Assert nodes
    nodes = {node["module_id"]: node for node in graph["nodes"]}
    assert "0" in nodes
    assert "7" in nodes

    # Verify observed evidence
    observed = result["observed_evidence"]
    kinds = [e["kind"] for e in observed]
    assert "module_table_captured" in kinds
    assert "require_captured" in kinds

    # Verify diagnostics - no error diagnostics
    records = result["diagnostic_records"]
    errors = [r for r in records if r["severity"] == "error"]
    assert len(errors) == 0

def test_webpack5_runtime_captured():
    """Minimal webpack5-like runtime -> require captured."""
    source = """
    (function() {
        var __webpack_modules__ = {
            10: function(module, exports, __webpack_require__) {
                globalThis.module10Run = true;
            }
        };
        var __webpack_module_cache__ = {};
        function __webpack_require__(moduleId) {
            var cachedModule = __webpack_module_cache__[moduleId];
            if (cachedModule !== undefined) return cachedModule.exports;
            var module = __webpack_module_cache__[moduleId] = { exports: {} };
            __webpack_modules__[moduleId](module, module.exports, __webpack_require__);
            return module.exports;
        }
        __webpack_require__.m = __webpack_modules__;
        __webpack_require__.c = __webpack_module_cache__;
        __webpack_require__.e = function() {};
        globalThis.__webpack_require__ = __webpack_require__;
        
        // Trigger execution
        __webpack_require__(10);
    })();
    """
    plan, result = plan_and_run(source)
    assert plan["selected_strategy"]["strategy_kind"] == "webpack_bridge"

    observed = result["observed_evidence"]
    kinds = [e["kind"] for e in observed]
    assert "require_captured" in kinds

def test_webpack_chunk_evidence():
    """Webpack chunk push -> chunk event observed."""
    source = """
    (function() {
        window.webpackJsonp = [];
        var modules = {};
        function __webpack_require__(id) {
            return modules[id];
        }
        __webpack_require__.m = modules;
        __webpack_require__.c = {};
        globalThis.__webpack_require__ = __webpack_require__;

        // Push chunk
        window.webpackJsonp.push([
            ["vendors"],
            {
                100: function(module, exports) {
                    globalThis.chunkModuleRun = true;
                }
            }
        ]);
    })();
    """
    plan, result = plan_and_run(source)
    graph = result["module_graph"]
    assert graph is not None
    chunks = graph["chunks"]
    assert len(chunks) > 0
    assert chunks[0]["chunk_id"] == "vendors"

    observed = result["observed_evidence"]
    kinds = [e["kind"] for e in observed]
    assert "chunk_event_observed" in kinds

def test_webpack_marker_only_guard():
    """Webpack marker only (no runtime init) -> WEBPACK_EVIDENCE_WEAK diagnostic, cannot PASS."""
    source = """
    var myFakeMarker = "__webpack_require__";
    var anotherVar = "not a real webpack bundle";
    """
    plan, result = plan_and_run(source)
    # The planner might class it as Webpack because of __webpack_require__ string
    assert plan["selected_strategy"]["strategy_kind"] == "webpack_bridge"

    # Verify observed evidence lacks strong items
    observed = result["observed_evidence"]
    kinds = [e["kind"] for e in observed]
    assert "module_table_captured" not in kinds

    # WEBPACK_EVIDENCE_WEAK / WEBPACK_REQUIRE_CAPTURE_FAILED should be present
    records = result["diagnostic_records"]
    codes = [r["code"] for r in records]
    assert "WEBPACK_REQUIRE_CAPTURE_FAILED" in codes or "WEBPACK_EVIDENCE_WEAK" in codes

def test_webpack_require_capture_late():
    """If require captured too late -> emits WEBPACK_REQUIRE_CAPTURE_LATE."""
    # Source defines __webpack_require__ on globalThis but WITHOUT setting
    # __webpack_require__.c. This means the Function.prototype.c setter does
    # NOT fire. The require is only captured via the runtime fallback in
    # collect_module_graph, which is considered "late".
    source = """
    (function() {
        globalThis.__webpack_require__ = function(id) {};
        globalThis.__webpack_require__.m = {
            0: function() { return "late"; }
        };
    })();
    """
    plan, result = plan_and_run(source)
    mg = result.get("module_graph")
    assert mg is not None, "should have module_graph"
    mg_codes = [d.get("code") for d in mg.get("diagnostics", [])]
    assert "WEBPACK_REQUIRE_CAPTURE_LATE" in mg_codes, (
        f"expected WEBPACK_REQUIRE_CAPTURE_LATE in module_graph diagnostics, got {mg_codes}"
    )

def test_webpack_vm_hybrid_integration():
    """Webpack + VM hybrid -> satisfies both layers."""
    source = """
    (function(modules) {
        function __webpack_require__(moduleId) {
            return modules[moduleId]();
        }
        __webpack_require__.m = modules;
        __webpack_require__.c = {};
        globalThis.__webpack_require__ = __webpack_require__;
        return __webpack_require__(0);
    })({
        0: function() {
            var A = [function() { globalThis.vmRun = true; }];
            var Q = [0];
            var U = 0;
            A[Q[U++]]();
        }
    });
    """
    plan, result = plan_and_run(source)
    assert plan["sample_kind"] == "webpack_vm_hybrid"

    graph = result["module_graph"]
    assert graph is not None
    assert graph["module_count"] == 1

    # Trace contains the dispatch trace
    assert any("D," in t for t in result["trace"])
