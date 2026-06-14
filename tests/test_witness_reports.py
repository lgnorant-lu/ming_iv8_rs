"""v0.8.33 Slice 3 -- L3 runtime witness report tests."""
from __future__ import annotations

import json
import sys
from pathlib import Path

import iv8_rs
import pytest

sys.path.insert(0, str(Path(__file__).parent.parent))
from tools.witness_reports import (
    generate_bcr_dispatch_report,
    generate_browser_surface_report,
    generate_undetectable_report,
)


def test_bcr_dispatch_report_has_expected_shape():
    report = generate_bcr_dispatch_report()
    assert report["schema_version"] == "iv8-bcr-dispatch-report.v0.1"
    assert report["behavior_slots"] == 15
    assert report["active_dispatch_sites"] == 15
    assert report["active_direct_install_calls"] == 0
    assert report["writes"] == []
    assert report["result"] == "pass"


def test_bcr_dispatch_report_valid_json():
    json.dumps(generate_bcr_dispatch_report(), sort_keys=True)


def test_browser_surface_report_valid_json():
    report = generate_browser_surface_report()
    assert report["writes"] == []
    assert report["evidence_ceiling"] == "diagnostic_only"
    assert report["total"] > 0
    assert report["total"] == len(report["results"])
    json.dumps(report, sort_keys=True)


def test_browser_surface_report_coverage():
    report = generate_browser_surface_report()
    assert report["total"] >= 20, f"expected >=20 probes, got {report['total']}"
    for entry in report["results"]:
        assert "id" in entry
        assert "expected" in entry
        assert "result" in entry


def test_browser_surface_passed_count():
    report = generate_browser_surface_report()
    assert report["passed"] + report["failed"] == report["total"]


def test_undetectable_report_valid_json():
    report = generate_undetectable_report()
    assert report["writes"] == []
    assert report["evidence_ceiling"] == "diagnostic_only"
    assert report["total"] == len(report["results"])
    json.dumps(report, sort_keys=True)


def test_undetectable_report_coverage():
    report = generate_undetectable_report()
    assert report["total"] >= 5
    for entry in report["results"]:
        assert "id" in entry
        assert "expected" in entry


def test_reports_are_deterministic():
    first = generate_bcr_dispatch_report()
    second = generate_bcr_dispatch_report()
    assert first == second
