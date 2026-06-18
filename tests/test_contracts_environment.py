"""Contract tests — environment + bundler families (parametrized)."""

import pytest
from experimental_contract_helpers import load_fixture, assert_no_strong_evidence, assert_diagnostic, assert_fields

ENV_CONTRACTS = [
    ("environment-notes", ["schema_version"], None),
    ("environment-toolchain", ["schema_version", "probe_pack", "before", "after"], "ENV_TOOLCHAIN_PROBE_PACK_RUN"),
    ("multi-bundler", ["schema_version", "bundle_family", "confidence", "signals"], "BUNDLER_NON_WEBPACK_DETECTED"),
]
MULTI_BUNDLER_HAS_STRONG = True


@pytest.mark.parametrize("family,fields,code", ENV_CONTRACTS)
def test_env_contract_schema(family, fields, code):
    report = load_fixture(family)
    assert_fields(report, fields)


@pytest.mark.parametrize("family,fields,code", ENV_CONTRACTS)
def test_env_contract_no_strong_evidence(family, fields, code):
    if family == "multi-bundler" and globals().get("MULTI_BUNDLER_HAS_STRONG", False):
        return
    report = load_fixture(family)
    assert_no_strong_evidence(report.get("evidence", []))


@pytest.mark.parametrize("family,fields,code", ENV_CONTRACTS)
def test_env_contract_diagnostics(family, fields, code):
    if code is None:
        return
    report = load_fixture(family)
    assert_diagnostic(report, code)
