"""Probe/Witness cross-source correlation tests."""

from __future__ import annotations

import copy
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))

from tools.cross_reference import (  # noqa: E402
    CROSS_SOURCE_MAP_SCHEMA,
    generate_cross_source_report,
    get_cross_source_map,
)


_PROBE_RESULTS = [
    {
        "probe_id": "idl.exists.Window",
        "target": "Window",
        "category": "presence",
        "expected": True,
        "actual": True,
    },
    {
        "probe_id": "idl.exists.Navigator",
        "target": "Navigator",
        "category": "presence",
        "expected": True,
        "actual": False,
    },
    {
        "probe_id": "idl.attr.Navigator.userAgent",
        "target": "navigator.userAgent",
        "category": "value",
        "expected": True,
        "actual": True,
    },
    {
        "probe_id": "idl.attr.Screen.width",
        "target": "screen.width",
        "category": "value",
        "expected": True,
        "actual": True,
    },
]

_BROWSER_SURFACE_REPORT = {
    "schema_version": "iv8-browser-surface-report.v0.1",
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
            "id": "typeof_navigator",
            "expr": "typeof navigator",
            "expected": "object",
            "actual": "object",
            "result": "pass",
        },
        {
            "id": "navigator_userAgent_string",
            "expr": "typeof navigator.userAgent === 'string'",
            "expected": True,
            "actual": True,
            "result": "pass",
        },
    ],
}


def test_cross_source_map_is_static_versioned_and_verified():
    mapping = get_cross_source_map()
    assert mapping["schema_version"] == CROSS_SOURCE_MAP_SCHEMA
    entries = mapping["entries"]
    assert len(entries) >= 9
    by_surface = {entry["surface"]: entry for entry in entries}
    assert by_surface["window.existence"]["probe_id"] == "idl.exists.Window"
    assert by_surface["navigator.userAgent.shape"]["witness_id"] == (
        "navigator_userAgent_string"
    )
    assert "typeof_WebSocket" not in {entry["witness_id"] for entry in entries}
    assert get_cross_source_map() == mapping


def test_cross_source_report_classifies_consistent_and_divergent():
    report = generate_cross_source_report(
        probe_results=_PROBE_RESULTS,
        witness_reports=[_BROWSER_SURFACE_REPORT],
    )
    assert report["schema_version"] == "iv8-cross-source-correlation.v0.1"
    assert report["map_schema_version"] == CROSS_SOURCE_MAP_SCHEMA
    assert report["writes"] == []
    assert report["evidence_ceiling"] == "diagnostic_only"

    by_surface = {item["surface"]: item for item in report["correlations"]}
    assert by_surface["window.existence"]["classification"] == "consistent"
    assert by_surface["navigator.existence"]["classification"] == "divergent"
    assert by_surface["navigator.userAgent.shape"]["classification"] == "consistent"


def test_cross_source_report_classifies_one_sided():
    report = generate_cross_source_report(
        probe_results=[_PROBE_RESULTS[0]],
        witness_reports=[_BROWSER_SURFACE_REPORT],
    )
    by_surface = {item["surface"]: item for item in report["correlations"]}
    assert by_surface["screen.width.shape"]["classification"] == "one_sided"
    assert by_surface["screen.width.shape"]["probe_status"] == "missing"
    assert by_surface["screen.width.shape"]["witness_status"] == "missing"


def test_cross_source_report_is_report_only_and_does_not_mutate_inputs():
    probe_results = copy.deepcopy(_PROBE_RESULTS)
    witness_reports = [copy.deepcopy(_BROWSER_SURFACE_REPORT)]
    before_probes = copy.deepcopy(probe_results)
    before_witness = copy.deepcopy(witness_reports)

    report = generate_cross_source_report(probe_results, witness_reports)

    assert probe_results == before_probes
    assert witness_reports == before_witness
    assert report["writes"] == []
    assert report["evidence_ceiling"] == "diagnostic_only"
    assert all(item["writes"] == [] for item in report["correlations"])
    assert all(
        item["evidence_ceiling"] == "diagnostic_only"
        for item in report["correlations"]
    )


def test_cross_source_report_is_deterministic():
    first = generate_cross_source_report(_PROBE_RESULTS, [_BROWSER_SURFACE_REPORT])
    second = generate_cross_source_report(_PROBE_RESULTS, [_BROWSER_SURFACE_REPORT])
    assert first == second
