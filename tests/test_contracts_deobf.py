"""Contract tests — deobfuscation families (parametrized)."""

import pytest
from experimental_contract_helpers import (
    assert_diagnostic,
    assert_fields,
    assert_no_strong_evidence,
    load_fixture,
)

DEOBF_CONTRACTS = [
    ("deobf-registry", ["schema_version", "entries", "selection_report"], "DEOBF_REGISTRY_POLICY_REJECTED"),
    ("deobf-validation", ["schema_version", "source_id", "pass_id", "level", "policy_status"], "DEOBF_VALIDATION_MARKER_ONLY"),
    ("deobf-sandbox", ["schema_version", "opt_in_level", "source_mutated"], "DEOBF_SANDBOX_VALIDATION_MISSING"),
    ("deobf-string-array", ["schema_version", "arrays", "status", "source_rewritten"], "DEOBF_ROTATION_IIFE_NOT_FOUND"),
]


@pytest.mark.parametrize("family,fields,code", DEOBF_CONTRACTS)
def test_deobf_contract_schema(family, fields, code):
    report = load_fixture(family)
    assert_fields(report, fields)


@pytest.mark.parametrize("family,fields,code", DEOBF_CONTRACTS)
def test_deobf_contract_no_strong_evidence(family, fields, code):
    report = load_fixture(family)
    assert_no_strong_evidence(report.get("evidence", []))


@pytest.mark.parametrize("family,fields,code", DEOBF_CONTRACTS)
def test_deobf_contract_diagnostics(family, fields, code):
    if code is None:
        return
    report = load_fixture(family)
    assert_diagnostic(report, code)
