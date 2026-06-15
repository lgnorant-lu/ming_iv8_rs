"""v0.8.33 Slice 4 -- MAPE-K feedback loop tests."""
from __future__ import annotations

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))
from tools.cross_reference import generate_cross_source_report  # noqa: E402
from tools.feedback_loop import (  # noqa: E402
    FeedbackState,
    KnowledgeBase,
    analyze,
    execute,
    monitor,
    plan,
    run_mapek_cycle,
    run_mapek_cycle_with_snapshot,
)

_SAMPLE_PROBES: list[dict] = [
    {"probe_id": "idl.exists.Window", "category": "presence", "expected": True, "actual": False},
    {"probe_id": "idl.exists.Navigator", "category": "presence", "expected": True, "actual": True},
    {"probe_id": "idl.exists.Screen", "category": "presence", "expected": True, "actual": True},
    {
        "probe_id": "idl.attr.Navigator.userAgent",
        "category": "value",
        "expected": True,
        "actual": True,
    },
    {"probe_id": "idl.attr.Screen.width", "category": "value", "expected": True, "actual": True},
    {
        "probe_id": "idl.exists.NotFound",
        "category": "presence",
        "expected": True,
        "actual": "ERR:JSError",
    },
]


def test_monitor_phase_report_only():
    report = monitor(_SAMPLE_PROBES)
    assert report["writes"] == []
    assert report["evidence_ceiling"] == "diagnostic_only"
    assert report["total_observations"] == len(_SAMPLE_PROBES)
    assert len(report["observations"]) == len(_SAMPLE_PROBES)


def test_analyze_phase_report_only():
    mon = monitor(_SAMPLE_PROBES)
    anl = analyze(mon)
    assert anl["writes"] == []
    assert anl["evidence_ceiling"] == "diagnostic_only"
    groups = {g["group"]: g["count"] for g in anl["groups"]}
    assert groups.get("pass", 0) > 0
    assert groups.get("fail", 0) > 0


def test_plan_phase_report_only():
    mon = monitor(_SAMPLE_PROBES)
    anl = analyze(mon)
    pln = plan(anl)
    assert pln["writes"] == []
    assert pln["evidence_ceiling"] == "diagnostic_only"
    assert "recommendations" in pln


def test_execute_phase_dry_run_only():
    mon = monitor(_SAMPLE_PROBES)
    anl = analyze(mon)
    pln = plan(anl)
    exe = execute(pln)
    assert exe["writes"] == []
    assert exe["execution_mode"] == "dry_run"
    assert "no mutations performed" in exe["dry_run_summary"]


def test_full_mapek_cycle():
    cycle = run_mapek_cycle(_SAMPLE_PROBES)
    assert cycle["writes"] == []
    assert cycle["evidence_ceiling"] == "diagnostic_only"
    assert "phases" in cycle
    for phase in ("monitor", "analyze", "plan", "execute", "knowledge"):
        assert phase in cycle["phases"]
        assert cycle["phases"][phase]["writes"] == []


def test_knowledge_base_read_only():
    kb = KnowledgeBase({"probe": {"ceiling": "diagnostic_only"}})
    assert kb.get_schema("probe") == {"ceiling": "diagnostic_only"}
    assert kb.get_schema("nonexistent") is None
    assert "probe" in kb.list_schemas()
    snap = kb.snapshot()
    assert snap["writes"] == []


def test_feedback_state_composable():
    state = FeedbackState()
    mon = monitor(_SAMPLE_PROBES, state)
    analyze(mon, state)
    assert len(state.observations) == len(_SAMPLE_PROBES)
    assert len(state.analysis_groups) > 0


def test_plan_recommendations_target_fail_gaps():
    mon = monitor(_SAMPLE_PROBES)
    anl = analyze(mon)
    pln = plan(anl, max_items_per_group=5)
    recs = pln["recommendations"]
    assert len(recs) > 0
    targets = {r["target"] for r in recs}
    assert "idl.exists.NotFound" in targets


def test_report_json_serializable():
    cycle = run_mapek_cycle(_SAMPLE_PROBES)
    data = json.dumps(cycle, sort_keys=True)
    assert len(data) > 100


def test_monitor_with_empty_input():
    report = monitor([])
    assert report["total_observations"] == 0
    assert report["observations"] == []
    assert report["writes"] == []


def test_plan_with_max_items_zero():
    mon = monitor(_SAMPLE_PROBES)
    anl = analyze(mon)
    pln = plan(anl, max_items_per_group=0)
    assert pln["total_recommendations"] == 0


def test_analyze_only_pass_probes():
    all_pass = [
        {"probe_id": "a", "expected": True, "actual": True},
        {"probe_id": "b", "expected": True, "actual": True},
    ]
    mon = monitor(all_pass)
    anl = analyze(mon)
    groups = {g["group"]: g["count"] for g in anl["groups"]}
    assert groups.get("pass", 0) == 2
    assert groups.get("fail", 0) == 0


def test_gap_class_flows_through_pipeline():
    probes = [{"probe_id": "x", "expected": True, "actual": False, "gap_class": "missing_api"}]
    mon = monitor(probes)
    obs = mon["observations"][0]
    assert obs["gap_class"] == "missing_api"
    assert obs["status"] == "fail"


def test_mapek_cycle_with_snapshot_is_report_only():
    cycle = run_mapek_cycle_with_snapshot(_SAMPLE_PROBES)
    assert cycle["writes"] == []
    assert cycle["evidence_ceiling"] == "diagnostic_only"
    assert cycle["phases"]["execute"]["execution_mode"] == "dry_run"
    convergence = cycle["convergence"]
    assert convergence["writes"] == []
    assert convergence["evidence_ceiling"] == "diagnostic_only"
    assert convergence["snapshot"]["writes"] == []
    assert convergence["knowledge_index"]["writes"] == []
    assert len(convergence["events"]) == len(_SAMPLE_PROBES)


def test_mapek_cycle_with_snapshot_delta():
    base_cycle = run_mapek_cycle_with_snapshot(_SAMPLE_PROBES)
    fixed_probes = [dict(item) for item in _SAMPLE_PROBES]
    fixed_probes[0]["actual"] = True
    current_cycle = run_mapek_cycle_with_snapshot(
        fixed_probes,
        base_snapshot=base_cycle["convergence"]["snapshot"],
    )
    convergence = current_cycle["convergence"]
    assert convergence["delta"]["writes"] == []
    assert convergence["delta"]["summary"]["changed_status"] == 1
    assert convergence["knowledge_index"]["writes"] == []


# -- v0.8.35 expanded probe pipeline test ------------------------------------

def test_expanded_probes_mapek_convergence_pipeline():
    expanded_results = _SAMPLE_PROBES + [
        {"probe_id": "idl.attr.Navigator.clipboard", "category": "value",
         "expected": True, "actual": True},
        {"probe_id": "idl.attr.Navigator.geolocation", "category": "value",
         "expected": True, "actual": False,
         "gap_class": "value_mismatch"},
        {"probe_id": "idl.attr.Document.title", "category": "value",
         "expected": True, "actual": True},
        {"probe_id": "idl.descr.Navigator.maxTouchPoints", "category": "descriptor",
         "expected": True, "actual": False,
         "gap_class": "descriptor_mismatch"},
        {"probe_id": "idl.inherits.Window", "category": "presence",
         "expected": True, "actual": True},
    ]
    base = run_mapek_cycle_with_snapshot(_SAMPLE_PROBES)
    current = run_mapek_cycle_with_snapshot(
        expanded_results,
        base_snapshot=base["convergence"]["snapshot"],
    )
    assert current["writes"] == []
    assert current["evidence_ceiling"] == "diagnostic_only"
    conv = current["convergence"]
    assert conv["snapshot"]["writes"] == []
    delta = conv.get("delta")
    assert delta is not None
    assert delta["summary"]["new"] >= 2
    assert delta["writes"] == []
    index = conv["knowledge_index"]
    assert index["writes"] == []
    assert index["evidence_ceiling"] == "diagnostic_only"
    assert index["summary"]["known_gaps"] >= 1


# -- v0.8.36 witness report routing ------------------------------------------


_SAMPLE_BROWSER_SURFACE_REPORT = {
    "schema_version": "iv8-browser-surface-report.v0.1",
    "surface": "BrowserSurface (synthetic)",
    "total": 2,
    "passed": 1,
    "failed": 1,
    "writes": [],
    "evidence_ceiling": "diagnostic_only",
    "results": [
        {
            "id": "typeof_window",
            "expr": "typeof window",
            "expected": "object",
            "actual": "object",
            "result": "pass",
        },
        {
            "id": "typeof_fetch",
            "expr": "typeof fetch",
            "expected": "function",
            "actual": "undefined",
            "result": "fail",
        },
    ],
    "result": "partial",
}


def test_mapek_snapshot_routes_witness_reports_as_convergence_events():
    cycle = run_mapek_cycle_with_snapshot(
        _SAMPLE_PROBES,
        witness_reports=[_SAMPLE_BROWSER_SURFACE_REPORT],
    )
    monitor_report = cycle["phases"]["monitor"]
    assert monitor_report["total_observations"] == len(_SAMPLE_PROBES)
    assert len(monitor_report["observations"]) == len(_SAMPLE_PROBES)

    convergence = cycle["convergence"]
    assert len(convergence["events"]) == len(_SAMPLE_PROBES) + 2
    sources = {
        (src["report_schema"], src["report_kind"])
        for src in convergence["snapshot"]["source_reports"]
    }
    assert ("iv8-browser-surface-report.v0.1", "browser_surface") in sources
    assert ("iv8-feedback-monitor.v0.1", "feedback_monitor") in sources


def test_mapek_snapshot_witness_reports_are_report_only():
    cycle = run_mapek_cycle_with_snapshot(
        _SAMPLE_PROBES,
        witness_reports=[_SAMPLE_BROWSER_SURFACE_REPORT],
    )
    assert cycle["writes"] == []
    assert cycle["evidence_ceiling"] == "diagnostic_only"
    convergence = cycle["convergence"]
    assert convergence["writes"] == []
    assert convergence["evidence_ceiling"] == "diagnostic_only"
    for event in convergence["events"]:
        assert event["writes"] == []
        assert event["evidence_ceiling"] == "diagnostic_only"


def test_mapek_snapshot_preserves_positional_arguments_with_witness_keyword():
    base = run_mapek_cycle_with_snapshot(_SAMPLE_PROBES)
    cycle = run_mapek_cycle_with_snapshot(
        _SAMPLE_PROBES,
        3,
        base["convergence"]["snapshot"],
        witness_reports=[_SAMPLE_BROWSER_SURFACE_REPORT],
    )
    assert cycle["convergence"].get("delta") is not None
    assert cycle["phases"]["plan"]["total_recommendations"] <= 6


# -- v0.8.39 analyze/plan depth gates --------------------------------------


def test_analyze_includes_gap_class_distribution():
    mon = monitor(_SAMPLE_PROBES)
    anl = analyze(mon)
    groups = anl["groups"]
    assert len(groups) > 0
    for group in groups:
        assert "gap_classes" in group
        assert isinstance(group["gap_classes"], dict)


def test_analyze_includes_severity_summary():
    mon = monitor(_SAMPLE_PROBES)
    anl = analyze(mon)
    groups = anl["groups"]
    for group in groups:
        assert "severity" in group
        assert isinstance(group["severity"], dict)


def test_analyze_preserves_backward_compatibility():
    mon = monitor(_SAMPLE_PROBES)
    anl = analyze(mon)
    assert "groups" in anl
    assert "phase" in anl
    assert anl["phase"] == "analyze"
    assert anl["writes"] == []
    assert anl["evidence_ceiling"] == "diagnostic_only"


def test_analyze_accepts_cross_source_report():
    mon = monitor(_SAMPLE_PROBES)
    xref = generate_cross_source_report(_SAMPLE_PROBES, [])
    anl = analyze(mon, cross_source_report=xref)
    assert "cross_source" in anl
    assert anl["cross_source"]["writes"] == []
    assert anl["evidence_ceiling"] == "diagnostic_only"


def test_analyze_cross_source_report_not_mutated():
    mon = monitor(_SAMPLE_PROBES)
    xref = generate_cross_source_report(_SAMPLE_PROBES, [])
    before = json.dumps(xref, sort_keys=True)
    analyze(mon, cross_source_report=xref)
    after = json.dumps(xref, sort_keys=True)
    assert before == after


def test_plan_includes_gap_class_and_severity():
    mon = monitor(_SAMPLE_PROBES)
    anl = analyze(mon)
    pln = plan(anl)
    recs = pln["recommendations"]
    for rec in recs:
        assert "gap_class" in rec
        assert "severity" in rec


def test_plan_action_remains_report_only():
    mon = monitor(_SAMPLE_PROBES)
    anl = analyze(mon)
    pln = plan(anl)
    for rec in pln["recommendations"]:
        assert rec["action"] == "report_only"


def test_plan_accepts_cross_source_classifications():
    mon = monitor(_SAMPLE_PROBES)
    xref = generate_cross_source_report(_SAMPLE_PROBES, [])
    anl = analyze(mon, cross_source_report=xref)
    pln = plan(anl, cross_source_report=xref)
    for rec in pln["recommendations"]:
        assert "cross_classification" in rec


def test_enriched_outputs_are_diagnostic_only():
    mon = monitor(_SAMPLE_PROBES)
    anl = analyze(mon)
    pln = plan(anl)
    assert anl["writes"] == []
    assert anl["evidence_ceiling"] == "diagnostic_only"
    assert pln["writes"] == []
    assert pln["evidence_ceiling"] == "diagnostic_only"
