"""Targeted runtime tests for VM Analysis and Handler types."""

from __future__ import annotations

from experimental_contract_helpers import load_fixture
from iv8_rs.vm_reports import (
    vm_analysis_report_from_dict,
    vm_analysis_report_to_dict,
    vm_handler_table_from_dict,
    vm_handler_table_to_dict,
)

# VM Analysis


def test_vm_analysis_typed_roundtrip():
    data = load_fixture("vm-analysis")
    report = vm_analysis_report_from_dict(data)
    roundtrip = vm_analysis_report_to_dict(report)

    assert roundtrip == data


def test_vm_analysis_preserves_unknown_confidence():
    data = load_fixture("vm-analysis")
    report = vm_analysis_report_from_dict(data)

    assert report.vm_family == "unknown_js_vm"
    assert report.bytecode is None
    assert report.trace_summary.opcode_sequence_observed is False


def test_vm_analysis_opcode_labels_are_hints():
    data = load_fixture("vm-analysis")
    report = vm_analysis_report_from_dict(data)

    for key, opcode in report.opcode_map.items():
        assert opcode.label.endswith("_hint")
        assert opcode.confidence == "weak"
        assert len(opcode.evidence) > 0


# VM Handler Table


def test_vm_handler_typed_roundtrip():
    data = load_fixture("vm-handler")
    table = vm_handler_table_from_dict(data)
    roundtrip = vm_handler_table_to_dict(table)

    assert roundtrip == data


def test_vm_handler_shape_score_preserved():
    data = load_fixture("vm-handler")
    table = vm_handler_table_from_dict(data)

    assert table.runtime_validated is False
    assert table.shape_score >= 6
    assert len(table.handlers) > 0


def test_vm_handler_candidates_no_decoded_semantics():
    data = load_fixture("vm-handler")
    table = vm_handler_table_from_dict(data)

    for h in table.handlers:
        assert h.confidence in ("strong", "medium", "weak", "diagnostic_only")
        assert h.observed_opcodes == []
        assert h.effects == []

    candidate = table.bytecode_candidates[0]
    assert candidate.runtime_validated is False
    assert candidate.length >= 20
    assert candidate.numeric_ratio >= 0.8
