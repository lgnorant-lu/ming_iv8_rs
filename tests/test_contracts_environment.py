"""Contract tests — environment + bundler families (parametrized + semantic checks)."""

import pytest
from experimental_contract_helpers import (
    assert_diagnostic,
    assert_fields,
    assert_no_strong_evidence,
    load_fixture,
)

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


def test_environment_notes_redacted_and_non_mutating():
    note = load_fixture("environment-notes")
    assert note["review_status"] == "redacted"
    assert "secret_value" in note["redactions"]
    assert "site_specific_url" in note["redactions"]
    assert note["writes"] == []


def test_environment_toolchain_records_probe_pack_delta():
    report = load_fixture("environment-toolchain")
    assert report["probe_pack"] == "fingerprint.m1"
    assert report["after"]["present"] >= report["before"]["present"]


def test_multi_bundler_blocks_webpack_overclaim():
    report = load_fixture("multi-bundler")
    table = report["module_table"]
    assert report["bundle_family"] == "browserify"
    assert table["family"] == "browserify"
    assert table["runtime_validated"] is False
    assert_fields(
        table["modules"][0],
        ["id", "wrapper", "static_deps", "dynamic_require_expressions"],
    )
