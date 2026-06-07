"""Targeted runtime tests for Deobf Registry and Validation types."""

from __future__ import annotations

from experimental_contract_helpers import load_fixture
from iv8_rs.deobf_reports import (
    registry_report_from_dict,
    registry_report_to_dict,
    validation_report_from_dict,
    validation_report_to_dict,
)

# Registry


def test_deobf_registry_typed_roundtrip():
    data = load_fixture("deobf-registry")
    report = registry_report_from_dict(data)
    roundtrip = registry_report_to_dict(report)

    assert roundtrip == data


def test_deobf_registry_entry_fields_are_preserved():
    data = load_fixture("deobf-registry")
    report = registry_report_from_dict(data)

    for entry in report.entries:
        assert " " not in entry.pass_id
        assert entry.quality_gate == "tests/test_deobf_pass_registry_contract.py"


def test_deobf_registry_unsafe_entry_negative():
    data = load_fixture("deobf-registry")
    report = registry_report_from_dict(data)
    entries = {e.pass_id: e for e in report.entries}

    unsafe = entries["dead_code.cleanup.v0"]
    assert unsafe.level == "unsafe_rewrite"
    assert unsafe.enabled_by_default is False
    assert unsafe.validation_required is True

    rejected_ids = {item["pass_id"] for item in report.selection_report.rejected}
    assert rejected_ids == {"dead_code.cleanup.v0"}


def test_deobf_registry_policy_ceiling_preserved():
    data = load_fixture("deobf-registry")
    report = registry_report_from_dict(data)

    for entry in report.entries:
        assert entry.evidence_ceiling in ("diagnostic_only", "weak", "marker_only", "strong")
        assert entry.policy_required in ("runtime_safe", "analysis_only", "unsafe_hook")


# Validation


def test_deobf_validation_typed_roundtrip():
    data = load_fixture("deobf-validation")
    report = validation_report_from_dict(data)
    roundtrip = validation_report_to_dict(report)

    assert roundtrip == data


def test_deobf_validation_marker_only_ceiling():
    data = load_fixture("deobf-validation")
    report = validation_report_from_dict(data)

    assert report.level == "shape_validated"
    assert report.policy_status == "accepted"

    for e in report.evidence:
        assert e.strength != "strong"


def test_deobf_validation_policy_failure_overrides_useful_output():
    data = load_fixture("deobf-validation")
    report = validation_report_from_dict(data)

    failures = [case for case in report.negative_cases if case.policy_status == "failed"]
    assert failures
    for case in failures:
        assert case.level == "regressed"
        for e in case.evidence:
            assert e.strength != "strong"
        codes = {d.code for d in case.diagnostics}
        assert "DEOBF_VALIDATION_POLICY_FAILED" in codes
