"""Behavioral tests for iv8_rs.entry — Entry Plane typed wrappers."""
import pytest

iv8_rs = pytest.importorskip("iv8_rs")


def test_entry_plan_from_dict_minimal():
    from iv8_rs.entry import EntryPlan, SelectedStrategy, Diagnostics
    d = {
        "plan_id": "test-1",
        "persona": "analysis",
        "sample_kind": "plain_script",
        "sample_signals": [],
        "selected_strategy": {"strategy_id": "s1", "strategy_kind": "source_ast", "selection_reason": "test"},
        "candidate_strategies": [],
        "phase_requirements": {},
        "requires_preload": False,
        "requires_reload": False,
        "entry_targets": [],
        "expected_evidence": [],
        "fallback_chain": [],
        "risk_level": "low",
        "diagnostics": Diagnostics.from_dict({"sample_signals": [], "fallback_attempts": [], "diagnostic_records": [], "observed_evidence": []}),
        "state": "planned",
    }
    plan = EntryPlan.from_dict(d)
    assert plan.plan_id == "test-1"
    assert plan.state == "planned"


def test_entry_result_from_dict_minimal():
    from iv8_rs.entry import EntryResult, SelectedStrategy, Diagnostics
    d = {
        "plan_id": "test-1",
        "final_state": "finalized",
        "selected_strategy": {"strategy_id": "s1", "strategy_kind": "source_ast", "selection_reason": "test"},
        "executed_strategies": [],
        "trace": [],
        "diagnostics": Diagnostics.from_dict({"sample_signals": [], "fallback_attempts": [], "diagnostic_records": [], "observed_evidence": []}),
        "cleanup_state": {},
        "diagnostic_records": [],
        "observed_evidence": [],
    }
    result = EntryResult.from_dict(d)
    assert result.plan_id == "test-1"
    assert result.final_state == "finalized"


def test_selected_strategy_from_dict():
    from iv8_rs.entry import SelectedStrategy
    d = {"strategy_id": "dispatch.main", "strategy_kind": "dispatch", "selection_reason": "vm detected"}
    s = SelectedStrategy.from_dict(d)
    assert s.strategy_id == "dispatch.main"
    assert s.strategy_kind == "dispatch"


def test_probe_result_from_dict():
    from iv8_rs.entry import ProbeResult
    d = {"can_swc_parse": True, "has_dispatch_pattern": False, "has_webpack_runtime": True, "has_closure_capture": False, "has_eval_heavy": False, "is_low_obfuscation": True}
    p = ProbeResult.from_dict(d)
    assert p.can_swc_parse
    assert not p.has_dispatch_pattern


def test_entry_types_importable():
    from iv8_rs.entry import (
        EntryPlan, EntryResult, SelectedStrategy, ExecutedStrategy,
        ProbeResult, EventMeta, Diagnostics, TraceMeta,
    )
    assert EntryPlan is not None
