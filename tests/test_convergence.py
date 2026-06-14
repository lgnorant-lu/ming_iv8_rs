"""v0.8.34 convergence event and snapshot tests."""
from __future__ import annotations

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))

from tools.convergence import (  # noqa: E402
    build_convergence_snapshot,
    build_knowledge_index,
    diff_convergence_snapshots,
    make_convergence_event,
    normalize_events_from_report,
    stable_subject_key,
)


def test_event_id_is_deterministic_and_no_write():
    event_a = make_convergence_event(
        source={"source_id": "a", "report_kind": "browser_surface", "report_schema": "s"},
        subject={"category": "value", "target": "window.x", "probe_id": "a"},
        status="fail",
        expected=True,
        actual=False,
        gap_class="value_mismatch",
    )
    event_b = make_convergence_event(
        source={"report_schema": "s", "report_kind": "browser_surface", "source_id": "a"},
        subject={"probe_id": "a", "target": "window.x", "category": "value"},
        status="fail",
        expected=True,
        actual=False,
        gap_class="value_mismatch",
    )
    assert event_a == event_b
    assert event_a["event_id"].startswith("sha256:")
    assert event_a["writes"] == []
    assert event_a["evidence_ceiling"] == "diagnostic_only"


def test_event_filters_blocked_target_flow_keys():
    event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "p"},
        subject={"probe_id": "p", "target": "x", "category": "value"},
        status="fail",
        expected={"safe": 1, "token": "secret"},
        actual={"safe": 2, "cookie": "secret"},
    )
    assert event["expected"] == {"safe": 1}
    assert event["actual"] == {"safe": 2}


def test_browser_surface_report_normalizes_to_events():
    report = {
        "schema_version": "iv8-browser-surface-report.v0.1",
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
        "writes": [],
    }
    events = normalize_events_from_report(report)
    assert len(events) == 2
    assert [event["status"] for event in events] == ["matched", "mismatched"]
    assert {event["source"]["report_kind"] for event in events} == {"browser_surface"}


def test_bcr_report_normalizes_structural_events():
    report = {
        "schema_version": "iv8-bcr-dispatch-report.v0.1",
        "behavior_slots": 15,
        "active_dispatch_sites": 15,
        "active_direct_install_calls": 0,
        "evidence_ceiling": "diagnostic_only",
        "writes": [],
    }
    events = normalize_events_from_report(report)
    assert len(events) == 2
    assert all(event["status"] == "matched" for event in events)
    assert all(event["writes"] == [] for event in events)


def test_bcr_error_report_normalizes_to_errored_events():
    report = {
        "schema_version": "iv8-bcr-dispatch-report.v0.1",
        "result": "error",
        "error": "missing source",
        "evidence_ceiling": "diagnostic_only",
        "writes": [],
    }
    events = normalize_events_from_report(report)
    assert len(events) == 2
    assert all(event["status"] == "errored" for event in events)
    assert all(event["gap_class"] == "runtime_error" for event in events)


def test_feedback_monitor_preserves_source_evidence_ceiling():
    report = {
        "schema_version": "iv8-feedback-monitor.v0.1",
        "observations": [
            {
                "probe_id": "navigator.userAgent",
                "category": "value",
                "status": "fail",
                "expected": "A",
                "actual": "B",
                "gap_class": "value_mismatch",
                "evidence_ceiling": "v8_surface",
            }
        ],
        "writes": [],
        "evidence_ceiling": "diagnostic_only",
    }
    event = normalize_events_from_report(report)[0]
    assert event["source_evidence_ceiling"] == "v8_surface"
    assert event["evidence_ceiling"] == "diagnostic_only"


def test_snapshot_is_deterministic_and_summarizes_statuses():
    events = [
        make_convergence_event(
            source={"report_schema": "s", "report_kind": "k", "source_id": "b"},
            subject={"probe_id": "b", "target": "b", "category": "value"},
            status="pass",
        ),
        make_convergence_event(
            source={"report_schema": "s", "report_kind": "k", "source_id": "a"},
            subject={"probe_id": "a", "target": "a", "category": "value"},
            status="fail",
        ),
    ]
    snapshot_a = build_convergence_snapshot(events)
    snapshot_b = build_convergence_snapshot(reversed(events))
    assert snapshot_a == snapshot_b
    assert snapshot_a["summary"]["total_events"] == 2
    assert snapshot_a["summary"]["matched"] == 1
    assert snapshot_a["summary"]["mismatched"] == 1
    assert snapshot_a["writes"] == []


def test_snapshot_delta_classifies_lifecycle():
    old_event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "same"},
        subject={"probe_id": "same", "target": "same", "category": "value"},
        status="fail",
        expected=True,
        actual=False,
    )
    changed_event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "same"},
        subject={"probe_id": "same", "target": "same", "category": "value"},
        status="pass",
        expected=True,
        actual=True,
    )
    new_event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "new"},
        subject={"probe_id": "new", "target": "new", "category": "value"},
        status="fail",
    )
    resolved_event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "old"},
        subject={"probe_id": "old", "target": "old", "category": "value"},
        status="fail",
    )
    base = build_convergence_snapshot([old_event, resolved_event])
    current = build_convergence_snapshot([changed_event, new_event])
    delta = diff_convergence_snapshots(base, current)
    assert delta["summary"] == {
        "new": 1,
        "resolved": 1,
        "persisting": 0,
        "changed_status": 1,
        "changed_severity": 0,
    }
    assert delta["writes"] == []


def test_knowledge_index_is_derived_and_read_only():
    event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "gap"},
        subject={"probe_id": "gap", "target": "gap", "category": "value"},
        status="fail",
        expected=True,
        actual=False,
    )
    snapshot = build_convergence_snapshot([event])
    empty = build_convergence_snapshot([])
    delta = diff_convergence_snapshots(empty, snapshot)
    index = build_knowledge_index(snapshot, delta)
    assert index["schema_version"] == "iv8-feedback-knowledge-index.v0.1"
    assert index["writes"] == []
    assert index["evidence_ceiling"] == "diagnostic_only"
    assert index["summary"]["known_gaps"] == 1
    assert index["known_gaps"][0]["lifecycle"] == "new"


def test_knowledge_index_skips_expected_divergence():
    event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "expected"},
        subject={"probe_id": "expected", "target": "expected", "category": "value"},
        status="expected_divergence",
        expected="browser_only",
        actual="v8_only",
    )
    snapshot = build_convergence_snapshot([event])
    index = build_knowledge_index(snapshot)
    assert index["summary"]["known_gaps"] == 0
    assert index["known_gaps"] == []


def test_knowledge_index_counts_resolved_gap_from_changed_status():
    failing = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "same"},
        subject={"probe_id": "same", "target": "same", "category": "value"},
        status="fail",
    )
    passing = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "same"},
        subject={"probe_id": "same", "target": "same", "category": "value"},
        status="pass",
    )
    base = build_convergence_snapshot([failing])
    current = build_convergence_snapshot([passing])
    delta = diff_convergence_snapshots(base, current)
    index = build_knowledge_index(current, delta)
    assert index["summary"]["known_gaps"] == 0
    assert index["summary"]["changed"] == 1


def test_reports_are_json_serializable():
    event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "p"},
        subject={"probe_id": "p", "target": "p", "category": "value"},
        status="expected_divergence",
        expected="browser_only",
        actual="v8_only",
    )
    snapshot = build_convergence_snapshot([event])
    encoded = json.dumps({"event": event, "snapshot": snapshot}, sort_keys=True)
    assert "iv8-convergence-event.v0.1" in encoded
    assert "iv8-convergence-snapshot.v0.1" in encoded


def test_stable_subject_key_ignores_expected_actual_changes():
    event_a = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "p"},
        subject={"probe_id": "p", "target": "p", "category": "value"},
        status="fail",
        expected=1,
        actual=2,
    )
    event_b = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "p"},
        subject={"probe_id": "p", "target": "p", "category": "value"},
        status="fail",
        expected=1,
        actual=3,
    )
    assert event_a["event_id"] != event_b["event_id"]
    assert stable_subject_key(event_a) == stable_subject_key(event_b)
