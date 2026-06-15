"""v0.8.34 convergence event and snapshot tests."""
from __future__ import annotations

import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))

from tools.convergence import (  # noqa: E402
    _V8_HARD_LIMITS,
    _VECTOR_COVERAGE_MAP,
    build_convergence_snapshot,
    build_knowledge_index,
    diff_convergence_snapshots,
    generate_coverage_report,
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


def test_event_redacts_blocked_target_flow_string_values():
    event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "p"},
        subject={"probe_id": "p", "target": "x", "category": "value"},
        status="fail",
        expected="token=secret",
        actual="safe",
    )
    assert event["expected"] == "<redacted:target_flow>"
    assert event["actual"] == "safe"


def test_error_actual_overrides_conflicting_pass_result():
    event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "p"},
        subject={"probe_id": "p", "target": "x", "category": "value"},
        status="pass",
        expected=True,
        actual="ERR:JSError",
    )
    assert event["status"] == "errored"
    assert event["gap_class"] == "runtime_error"


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


def test_snapshot_forces_embedded_events_to_no_write():
    event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "p"},
        subject={"probe_id": "p", "target": "p", "category": "value"},
        status="fail",
    )
    event["writes"] = ["bad"]
    event["evidence_ceiling"] = "accepted_equivalence"
    snapshot = build_convergence_snapshot([event])
    assert snapshot["events"][0]["writes"] == []
    assert snapshot["events"][0]["evidence_ceiling"] == "diagnostic_only"


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


# -- v0.8.35 contract validator tests ----------------------------------------

def test_contract_event_always_diagnostic_only():
    statuses = (
        "matched", "mismatched", "errored", "expected_divergence",
        "missing", "unexpected", "skipped",
    )
    for status in statuses:
        event = make_convergence_event(
            source={"report_schema": "s", "report_kind": "k", "source_id": "p"},
            subject={"probe_id": "p", "target": "p", "category": "value"},
            status=status,
        )
        assert event["writes"] == []
        assert event["evidence_ceiling"] == "diagnostic_only"
        assert event["schema_version"] == "iv8-convergence-event.v0.1"
        assert event["event_id"].startswith("sha256:")


def test_contract_event_with_minimal_fields_is_valid():
    event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "p"},
        subject={"probe_id": "p", "target": "p", "category": "value"},
        status="fail",
    )
    required = {"schema_version", "event_id", "source", "subject", "status",
                "gap_class", "severity", "expected", "actual",
                "source_evidence_ceiling", "evidence_ceiling", "writes"}
    assert set(event.keys()) == required
    assert event["gap_class"] in {
        "none", "missing_api", "value_mismatch", "type_mismatch",
        "descriptor_mismatch", "prototype_chain_mismatch", "runtime_error",
        "expected_divergence", "structural_mismatch", "unsupported_boundary",
    }


def test_contract_snapshot_always_diagnostic_only():
    snapshot = build_convergence_snapshot([])
    assert snapshot["writes"] == []
    assert snapshot["evidence_ceiling"] == "diagnostic_only"
    assert snapshot["schema_version"] == "iv8-convergence-snapshot.v0.1"
    assert snapshot["snapshot_id"].startswith("sha256:")
    assert snapshot["summary"]["total_events"] == 0


def test_contract_snapshot_empty_events_is_still_valid():
    snapshot = build_convergence_snapshot([])
    assert snapshot["events"] == []
    assert snapshot["summary"]["total_events"] == 0
    assert snapshot["source_reports"] == []


def test_contract_delta_always_diagnostic_only():
    empty = build_convergence_snapshot([])
    delta = diff_convergence_snapshots(empty, empty)
    assert delta["writes"] == []
    assert delta["evidence_ceiling"] == "diagnostic_only"
    assert delta["schema_version"] == "iv8-convergence-delta.v0.1"
    for cls in ("new", "resolved", "persisting", "changed_status", "changed_severity"):
        assert delta["summary"][cls] == 0


def test_contract_delta_empty_base_produces_all_new():
    empty = build_convergence_snapshot([])
    event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "p"},
        subject={"probe_id": "p", "target": "p", "category": "value"},
        status="fail",
    )
    current = build_convergence_snapshot([event])
    delta = diff_convergence_snapshots(empty, current)
    assert delta["summary"]["new"] == 1


def test_contract_knowledge_index_always_diagnostic_only():
    snapshot = build_convergence_snapshot([])
    index = build_knowledge_index(snapshot)
    assert index["writes"] == []
    assert index["evidence_ceiling"] == "diagnostic_only"
    assert index["schema_version"] == "iv8-feedback-knowledge-index.v0.1"
    assert index["known_gaps"] == []


def test_contract_knowledge_index_empty_snapshot_is_still_valid():
    snapshot = build_convergence_snapshot([])
    index = build_knowledge_index(snapshot)
    assert index["known_gaps"] == []
    assert index["summary"]["known_gaps"] == 0
    assert index["summary"]["new"] == 0
    assert index["source_snapshot_id"] == snapshot["snapshot_id"]


def test_contract_no_target_flow_in_generated_payloads():
    event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "p"},
        subject={"probe_id": "p", "target": "p", "category": "value"},
        status="fail",
        expected={"token": "bad", "cookie": "bad", "signature": "bad", "nonce": "bad"},
        actual="safe",
    )
    encoded = json.dumps(event, sort_keys=True)
    for term in ("token", "cookie", "signature", "nonce", "authorization", "endpoint", "domain"):
        assert term not in encoded.lower() or "redacted" in encoded.lower()


# -- v0.8.35 coverage gap report tests ---------------------------------------

def test_coverage_report_classifies_covered_vectors():
    probe_ids = [
        "idl.attr.Navigator.userAgent",
        "idl.attr.Navigator.webdriver",
        "idl.descr.Navigator.maxTouchPoints",
    ]
    report = generate_coverage_report(probe_ids)
    assert report["writes"] == []
    assert report["evidence_ceiling"] == "diagnostic_only"
    assert report["summary"]["covered"] >= 2


def test_coverage_report_classifies_hard_limits():
    probe_ids = ["idl.attr.Navigator.userAgent"]
    report = generate_coverage_report(probe_ids)
    assert report["summary"]["hard_limit"] >= 10


def test_coverage_report_is_diagnostic_only():
    probe_ids = ["idl.attr.Navigator.userAgent"]
    report = generate_coverage_report(probe_ids)
    assert report["writes"] == []
    assert report["evidence_ceiling"] == "diagnostic_only"
    assert report["schema_version"] == "iv8-coverage-gap-report.v0.1"


# -- v0.8.36 coverage map data-fill ------------------------------------------


def test_coverage_report_maps_identity_rendering_behavioral_jsvm_protocol():
    probe_ids = [
        "idl.attr.Screen.width",
        "browser_surface.window_chrome_object",
        "browser_surface.crypto_subtle_object",
        "idl.attr.Performance.timeOrigin",
        "idl.inherits.Window",
        "browser_surface.typeof_fetch",
        "browser_surface.typeof_XMLHttpRequest",
        "browser_surface.typeof_WebGLRenderingContext",
    ]
    report = generate_coverage_report(probe_ids)
    covered = set(report["coverage"]["covered"])
    assert {"V015", "V022", "V095", "V103", "V032", "V067", "V085"} <= covered


def test_coverage_report_mapping_does_not_promote_hard_limits():
    probe_ids = [
        "idl.attr.Screen.width",
        "browser_surface.typeof_fetch",
    ]
    report = generate_coverage_report(probe_ids)
    assert "V058" in report["coverage"]["hard_limit"]
    assert "V091" in report["coverage"]["hard_limit"]
    assert "V058" not in report["coverage"]["covered"]
    assert "V091" not in report["coverage"]["covered"]


def test_coverage_report_data_fill_expands_known_vector_universe():
    report = generate_coverage_report([])
    assert report["total_vectors"] >= 45
    assert report["summary"]["hard_limit"] >= 13


# -- v0.8.38 coverage map completion gates -------------------------------


def test_coverage_map_completion_adds_handled_entries():
    entries_from = set(_VECTOR_COVERAGE_MAP)
    assert "screen.availLeft" in entries_from
    assert "screen.availTop" in entries_from
    assert "window.scrollX" in entries_from
    assert "window.scrollY" in entries_from
    assert len(entries_from) > 68


def test_coverage_completion_does_not_promote_true_missing():
    report = generate_coverage_report([])
    assert report["summary"]["covered"] == 0
    assert report["summary"]["not_yet_probed"] > 0
    assert report["summary"]["hard_limit"] >= 13


def test_coverage_completion_preserves_hard_limits():
    report = generate_coverage_report([])
    hard = set(report["coverage"]["hard_limit"])
    for vid in _V8_HARD_LIMITS:
        assert vid in hard, f"hard limit {vid} missing"


def test_coverage_completion_handled_paths_reach_expected_vectors():
    from tools.idl_probe.generate_probe_pack import generate_probe_pack

    pack = generate_probe_pack()
    ids = [p["probe_id"] for p in pack["probes"]]
    report = generate_coverage_report(ids)
    assert report["total_vectors"] >= 45
    assert "V016" in set(report["coverage"]["covered"])
    assert "V021" in set(report["coverage"]["covered"])



def test_coverage_completion_report_is_diagnostic_only():
    report = generate_coverage_report([])
    assert report["writes"] == []
    assert report["evidence_ceiling"] == "diagnostic_only"
