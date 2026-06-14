"""v0.8.33 Slice 4 -- MAPE-K feedback loop tests."""
from __future__ import annotations

import json
import sys
from pathlib import Path

import pytest

sys.path.insert(0, str(Path(__file__).parent.parent))
from tools.feedback_loop import (
    FeedbackState,
    KnowledgeBase,
    analyze,
    execute,
    monitor,
    plan,
    run_mapek_cycle,
)


_SAMPLE_PROBES: list[dict] = [
    {"probe_id": "idl.exists.Window", "category": "presence", "expected": True, "actual": False},
    {"probe_id": "idl.exists.Navigator", "category": "presence", "expected": True, "actual": True},
    {"probe_id": "idl.exists.Screen", "category": "presence", "expected": True, "actual": True},
    {"probe_id": "idl.attr.Navigator.userAgent", "category": "value", "expected": True, "actual": True},
    {"probe_id": "idl.attr.Screen.width", "category": "value", "expected": True, "actual": True},
    {"probe_id": "idl.exists.NotFound", "category": "presence", "expected": True, "actual": "ERR:JSError"},
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
    anl = analyze(mon, state)
    assert len(state.observations) == len(_SAMPLE_PROBES)
    assert len(state.analysis_groups) > 0


def test_plan_recommendations_target_fail_gaps():
    mon = monitor(_SAMPLE_PROBES)
    anl = analyze(mon)
    pln = plan(anl, max_items=5)
    recs = pln["recommendations"]
    assert len(recs) > 0
    targets = {r["target"] for r in recs}
    assert "idl.exists.NotFound" in targets


def test_report_json_serializable():
    cycle = run_mapek_cycle(_SAMPLE_PROBES)
    data = json.dumps(cycle, sort_keys=True, default=str)
    assert len(data) > 100
