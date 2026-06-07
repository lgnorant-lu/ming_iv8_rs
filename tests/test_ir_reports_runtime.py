"""Targeted runtime tests for IV8 IR Node report."""

from __future__ import annotations

from experimental_contract_helpers import load_fixture
from iv8_rs.ir_reports import ir_node_report_from_dict, ir_node_report_to_dict


def test_ir_node_typed_roundtrip():
    data = load_fixture("ir-node")
    report = ir_node_report_from_dict(data)
    roundtrip = ir_node_report_to_dict(report)

    assert roundtrip == data


def test_ir_node_analysis_only():
    data = load_fixture("ir-node")
    report = ir_node_report_from_dict(data)

    assert report.ir_kind == "structured_ast_ir"
    assert report.confidence_summary.strong == 0


def test_ir_node_mba_candidate_marked():
    data = load_fixture("ir-node")
    report = ir_node_report_from_dict(data)

    node_kinds = {n.kind for n in report.nodes}
    assert "MbaCandidate" in node_kinds
    assert "eval" in report.unsupported_features


def test_ir_node_no_source_rewrite():
    data = load_fixture("ir-node")
    report = ir_node_report_from_dict(data)

    for n in report.nodes:
        assert n.source == "fixture"
        assert n.confidence in ("strong", "medium", "weak", "diagnostic_only")
