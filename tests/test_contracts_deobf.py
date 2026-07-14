"""Contract tests — deobfuscation families (parametrized + semantic checks)."""

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


def test_deobf_registry_rejects_unsafe_by_default():
    report = load_fixture("deobf-registry")
    selected = report["selection_report"]["selected"]
    rejected = {item["pass_id"] for item in report["selection_report"]["rejected"]}
    assert "string_array.locator.v0" in selected
    assert "dead_code.cleanup.v0" in rejected


def test_deobf_validation_is_shape_only_accepted():
    report = load_fixture("deobf-validation")
    assert report["level"] == "shape_validated"
    assert report["policy_status"] == "accepted"


def test_deobf_sandbox_does_not_mutate_source():
    report = load_fixture("deobf-sandbox")
    assert report["source_mutated"] is False
    assert report["opt_in_level"] == "sandbox_plan"


def test_deobf_string_array_located_only_no_rewrite():
    report = load_fixture("deobf-string-array")
    assert report["status"] == "located_only"
    assert report["source_rewritten"] is False
    assert report["arrays"][0]["kind"] == "string_array_candidate"
