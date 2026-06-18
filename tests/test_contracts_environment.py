"""Contract tests — environment + bundler families (parametrized)."""

import pytest
from experimental_contract_helpers import load_fixture, assert_no_strong_evidence, assert_diagnostic, assert_fields

ENV_CONTRACTS = [
    ("environment-notes", ["schema_version", "notes", "review_status"], "ENV_SAMPLE_NOTE_UNREVIEWED"),
    ("environment-toolchain", ["schema_version", "probe_packs", "evidence_ceiling"], "ENV_TOOLCHAIN_DIAGNOSTIC_ONLY"),
    ("multi-bundler", ["schema_version", "formats", "detected"], "MULTI_BUNDLER_DETECTION_ONLY"),
]


@pytest.mark.parametrize("family,fields,code", ENV_CONTRACTS)
def test_env_contract_schema(family, fields, code):
    report = load_fixture(family)
    assert_fields(report, fields)


@pytest.mark.parametrize("family,fields,code", ENV_CONTRACTS)
def test_env_contract_no_strong_evidence(family, fields, code):
    report = load_fixture(family)
    assert_no_strong_evidence(report.get("evidence", []))


@pytest.mark.parametrize("family,fields,code", ENV_CONTRACTS)
def test_env_contract_diagnostics(family, fields, code):
    report = load_fixture(family)
    assert_diagnostic(report, code)
