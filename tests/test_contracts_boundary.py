"""Contract tests — boundary families: WASM, framework, interpreter, IR, CFF, anti-debug (parametrized)."""

import pytest
from experimental_contract_helpers import load_fixture, assert_no_strong_evidence, assert_diagnostic, assert_fields

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
