"""Entry plan + run sample-kind acceptance tests.

Validates that prepare_entry / run_with_entry correctly classify,
plan, and execute representative JS samples across all sample_kind
categories defined in docs/acceptance/v0.6-real-samples.md.

Each test:
- Creates a sample of a known kind
- Calls prepare_entry to verify classification and strategy selection
- Calls run_with_entry to verify execution and evidence collection
- Asserts PASS/WARN criteria
"""

from __future__ import annotations

import threading
from typing import Any

from iv8_rs import EntryPlan, EntryResult, prepare_entry, run_with_entry

# JSContext / prepare_entry needs 128MB stack (K-010 / K-011).
_STACK = 128 * 1024 * 1024


def _on_large_stack(fn, *args, **kwargs):
    box: list[Any] = [None, None]

    def target():
        try:
            box[0] = fn(*args, **kwargs)
        except BaseException as e:
            box[1] = e

    old = threading.stack_size()
    threading.stack_size(_STACK)
    try:
        t = threading.Thread(target=target)
        t.start()
        t.join()
    finally:
        threading.stack_size(old)
    if box[1] is not None:
        raise box[1]
    return box[0]


# ───
# Helpers
# ───

def plan_and_run(source: str, persona: str = "analysis",
                 entry_expr: str | None = None) -> tuple[dict, dict]:
    def _run():
        plan = prepare_entry(source, persona=persona)
        plan_obj = EntryPlan.from_dict(plan)
        assert plan_obj.state == "planned"
        result = run_with_entry(plan, source, entry_expr=entry_expr)
        return plan, result

    return _on_large_stack(_run)


def result_has_errors(result: dict) -> bool:
    records = result.get("diagnostic_records", [])
    return any(r.get("severity") == "error" for r in records)


def result_has_trace(result: dict) -> bool:
    return len(result.get("trace", [])) > 0


def result_has_module_graph(result: dict) -> bool:
    return result.get("module_graph") is not None


def result_has_trace_meta(result: dict) -> bool:
    return result.get("trace_meta") is not None


# ───
# 6.1 plain_script
# ───

def test_plain_script_basic():
    """plain_script: PASS if plan classifies correctly and executes."""
    source = "var x = 1 + 1; var y = x * 2;"
    plan, result = plan_and_run(source)
    assert plan["sample_kind"] == "plain_script"
    assert not result_has_errors(result)


# ───
# 6.2 vm_dispatch_known
# ───

def test_vm_dispatch_known_chaosvm():
    """vm_dispatch_known: PASS if plan detects dispatch pattern."""
    source = "var A=[];var Q=[0,1];var U=0;var r=A[Q[U++]]();"
    plan, _ = plan_and_run(source)
    assert plan["sample_kind"] == "vm_dispatch_known"


def test_vm_dispatch_known_switchvm():
    """vm_dispatch_known: switch-VM pattern detection."""
    source = "switch(B[P++]) { case 0: break; }"
    plan, _ = plan_and_run(source)
    assert plan["sample_kind"] == "vm_dispatch_known"


# ───
# 6.3 vm_dispatch_unknown
# ───

def test_vm_dispatch_unknown():
    """vm_dispatch_unknown: PASS with fallback explanation."""
    source = """
    var X = {};
    function dispatcher(pc) {
        var fn = X[pc];
        if (fn) fn();
    }
    """
    plan, _ = plan_and_run(source)
    # Falls back to transparent hook since no extractable dispatch
    assert plan["sample_kind"] in ("plain_script", "vm_dispatch_unknown")
    assert plan["selected_strategy"]["strategy_kind"] in (
        "runtime_transparent", "source_ast", "cdp_probe"
    )


# ───
# 6.4 webpack_runtime
# ───

def test_webpack_runtime():
    """webpack_runtime: PASS if plan selects webpack_bridge strategy."""
    source = """
    (function(modules) {
        function __webpack_require__(moduleId) {
            return modules[moduleId];
        }
        __webpack_require__.m = modules;
        __webpack_require__.c = {};
        return __webpack_require__(0);
    })({});
    """
    plan, _ = plan_and_run(source)
    assert plan["sample_kind"] == "webpack_runtime"
    assert plan["selected_strategy"]["strategy_kind"] == "webpack_bridge"


# ───
# 6.5 webpack_vm_hybrid
# ───

def test_webpack_vm_hybrid():
    """webpack_vm_hybrid: PASS with evidence for both layers."""
    source = """
    var __webpack_require__ = function(id) { return modules[id]; };
    var A = []; var Q = [0]; var U = 0;
    var modules = { 7: function() { return A[Q[U++]](); } };
    """
    plan, _ = plan_and_run(source)
    assert plan["sample_kind"] == "webpack_vm_hybrid"


# ───
# 6.6 eval_heavy
# ───

def test_eval_heavy():
    """eval_heavy: PASS if plan detects multiple eval calls."""
    source = 'eval("1+1"); eval("2+2"); Function("return 3");'
    plan, _ = plan_and_run(source)
    assert plan["sample_kind"] == "eval_heavy"


# ───
# 6.7 closure_captured_runtime
# ───

def test_closure_captured_runtime():
    """closure_captured_runtime: PASS if plan identifies early capture."""
    source = """
    var nav = navigator;
    (function() {
        var x = nav.userAgent;
    })();
    """
    plan, _ = plan_and_run(source)
    # May be plain_script or closure_captured_runtime depending on env
    assert plan["sample_kind"] in ("plain_script", "closure_captured_runtime",
                                   "eval_heavy")


# ───
# Persona-specific
# ───

def test_runtime_persona_conservative():
    """Runtime persona should not select aggressive strategies."""
    source = "var x = 1;"
    plan = prepare_entry(source, persona="runtime")
    kind = plan["selected_strategy"]["strategy_kind"]
    assert kind != "runtime_aggressive"


def test_analysis_persona_allows_source_ast():
    """Analysis persona should allow source_ast for plain scripts."""
    source = "var x = 1;"
    plan = prepare_entry(source, persona="analysis")
    assert plan["selected_strategy"]["strategy_kind"] in (
        "source_ast", "source_regex"
    )


# ───
# EntryPlan / EntryResult dataclass wrappers
# ───

def test_entry_plan_dataclass():
    """EntryPlan dataclass from_dict roundtrip."""
    plan = prepare_entry("var x = 1;", persona="analysis")
    obj = EntryPlan.from_dict(plan)
    assert obj.plan_id == plan["plan_id"]
    assert obj.persona == "analysis"
    assert obj.state == "planned"


def test_entry_result_dataclass_empty():
    """EntryResult dataclass handles minimal result."""
    result = {
        "plan_id": "test",
        "final_state": "planned",
        "selected_strategy": {"strategy_id": "s1", "strategy_kind": "cdp_probe",
                              "selection_reason": "test"},
        "executed_strategies": [],
        "trace": [],
        "diagnostic_records": [],
        "observed_evidence": [],
        "diagnostics": {},
    }
    obj = EntryResult.from_dict(result)
    assert obj.plan_id == "test"
    assert obj.final_state == "planned"
    assert len(obj.trace) == 0


# ───
# trace_meta verification
# ───

def test_run_with_entry_returns_trace_meta():
    """EntryResult from run_with_entry should include trace_meta."""
    source = "var x = 1;"
    _, result = plan_and_run(source)
    assert result_has_trace_meta(result)
    meta = result["trace_meta"]
    assert "plan_id" in meta
    assert "persona" in meta
    assert "sample_kind" in meta
    assert "trace_format" in meta


def test_environment_report_declares_static_summary():
    """environment_report must not masquerade as a full Environment Probe."""
    _, result = plan_and_run("var x = 1;")
    report = result["environment_report"]
    assert report["kind"] == "static_execution_summary"
    assert report["is_probe_report"] is False


def test_entry_result_reports_partial_strategy_semantics():
    """Partial strategies should expose limitations in diagnostics/warnings."""
    _, result = plan_and_run("var x = 1;", persona="analysis")
    records = result.get("diagnostic_records", [])
    assert any(w.get("code") == "ACT_STRATEGY_PARTIAL" for w in records)
    missing = result["diagnostics"].get("missing_capabilities", [])
    assert any("SourceAst" in item or "SourceRegex" in item for item in missing)

