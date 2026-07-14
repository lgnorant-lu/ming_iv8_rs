"""Contract tests — boundary families: WASM, framework, interpreter, IR, CFF, anti-debug."""

import pytest
from experimental_contract_helpers import (
    assert_diagnostic,
    assert_fields,
    assert_no_strong_evidence,
    load_fixture,
)

BOUNDARY_CONTRACTS = [
    ("wasm-boundary", ["schema_version", "parser_status", "signals"], "WASM_PARSER_DEFERRED"),
    ("framework-boundary", ["schema_version", "markers"], "FRAMEWORK_RECOVERY_DEFERRED"),
    ("interpreter-model", ["schema_version", "model_scope", "transitions"], "INTERPRETER_MODEL_SEMANTIC_HINT_ONLY"),
    ("ir-node", ["schema_version", "ir_kind", "nodes", "confidence_summary"], "IR_MBA_CANDIDATE_MARKED"),
    ("cff", ["schema_version", "detected", "variant", "markers"], "CFF_MARKER_ONLY"),
    ("anti-debug", ["schema_version", "markers"], "ANTI_DEBUG_REWRITE_BLOCKED"),
]


@pytest.mark.parametrize("family,fields,code", BOUNDARY_CONTRACTS)
def test_boundary_contract_schema(family, fields, code):
    report = load_fixture(family)
    assert_fields(report, fields)


@pytest.mark.parametrize("family,fields,code", BOUNDARY_CONTRACTS)
def test_boundary_contract_no_strong_evidence(family, fields, code):
    report = load_fixture(family)
    assert_no_strong_evidence(report.get("evidence", []))


@pytest.mark.parametrize("family,fields,code", BOUNDARY_CONTRACTS)
def test_boundary_contract_diagnostics(family, fields, code):
    if code is None:
        return
    report = load_fixture(family)
    assert_diagnostic(report, code)


def test_wasm_boundary_defers_parser():
    report = load_fixture("wasm-boundary")
    assert report["parser_status"] == "not_attempted"
    assert "wasm_bindgen_glue" in report["signals"]


def test_framework_boundary_records_markers_without_recovery():
    report = load_fixture("framework-boundary")
    assert report["recovery_attempted"] is False
    assert report["source_map_status"] == "reference_missing"


def test_interpreter_model_is_observed_trace_only():
    report = load_fixture("interpreter-model")
    transition = report["transitions"][0]
    assert report["model_scope"] == "observed_trace_only"
    assert transition["effects"][0]["value"].endswith("_hint")
    assert transition["confidence"] == "weak"


def test_cff_marker_stays_marker_only():
    report = load_fixture("cff")
    marker = report["markers"][0]
    assert report["vm_distinction"] == "cff_likely"
    assert marker["action"] == "marker_only"
    assert marker["policy_if_rewrite"] == "unsafe_rewrite"


def test_anti_debug_blocks_rewrite():
    report = load_fixture("anti-debug")
    marker = report["markers"][0]
    assert marker["action"] == "marker_only"
    assert marker["policy_if_rewrite"] == "unsafe_hook"


def test_ir_node_mba_candidate_and_no_strong_summary():
    report = load_fixture("ir-node")
    assert report["ir_kind"] == "structured_ast_ir"
    assert report["confidence_summary"]["strong"] == 0
    assert "MbaCandidate" in {n["kind"] for n in report["nodes"]}
