"""Targeted runtime tests for String Array locator skeleton."""

from __future__ import annotations

from experimental_contract_helpers import load_fixture
from iv8_rs.string_array_reports import string_array_report_from_dict, string_array_report_to_dict


def test_string_array_typed_roundtrip():
    data = load_fixture("deobf-string-array")
    report = string_array_report_from_dict(data)
    roundtrip = string_array_report_to_dict(report)

    assert roundtrip == data


def test_string_array_report_is_located_only():
    data = load_fixture("deobf-string-array")
    report = string_array_report_from_dict(data)

    assert report.status == "located_only"
    assert report.source_rewritten is False


def test_string_array_candidate_fields_preserved():
    data = load_fixture("deobf-string-array")
    report = string_array_report_from_dict(data)

    candidate = report.arrays[0]
    assert candidate.kind == "string_array_candidate"
    assert candidate.binding_id.startswith("scope:")
    assert candidate.element_count >= 10
    assert candidate.string_ratio >= 0.8
    assert candidate.confidence in ("strong", "medium", "weak", "diagnostic_only")


def test_string_array_decoder_no_execution():
    data = load_fixture("deobf-string-array")
    report = string_array_report_from_dict(data)

    for d in report.decoders:
        assert d.param_count >= 0
        assert d.confidence in ("strong", "medium", "weak", "diagnostic_only")


def test_string_array_evidence_marker_only():
    data = load_fixture("deobf-string-array")
    report = string_array_report_from_dict(data)

    for e in report.evidence:
        assert e.strength != "strong"
