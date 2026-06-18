"""Contract tests — deobfuscation families (parametrized)."""

import pytest
from experimental_contract_helpers import load_fixture, assert_no_strong_evidence, assert_diagnostic, assert_fields

DEOBF_CONTRACTS = [
    ("deobf-registry", ["schema_version", "entries", "selection_report"], "DEOBF_REGISTRY_EVIDENCE_OVERCLAIM"),
    ("deobf-validation", ["schema_version", "source_id", "pass_id", "level", "policy_status"], "DEOBF_VALIDATION_POLICY_FAILED"),
    ("deobf-sandbox", ["schema_version", "opt_in_level", "source_mutated"], "DEOBF_SANDBOX_HOST_API_BLOCKED"),
    ("deobf-string-array", ["schema_version", "arrays", "status", "source_rewritten"], "DEOBF_STRING_ARRAY_CANDIDATE_WEAK"),
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
    report = load_fixture(family)
    assert_diagnostic(report, code)
