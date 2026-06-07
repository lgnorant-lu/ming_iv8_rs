"""Round-trip contract test for shared experimental report types.

Verifies that every experimental contract fixture can be loaded through
typed ExperimentalReport dataclasses and back without lossy fields.
"""

from __future__ import annotations

from experimental_contract_helpers import load_fixture
from iv8_rs.experimental_report import experimental_report_roundtrip

EXPERIMENTAL_FIXTURE_FAMILIES = [
    "environment-toolchain",
    "deobf-registry",
    "deobf-validation",
    "deobf-string-array",
    "vm-analysis",
    "vm-handler",
    "ir-node",
]


def test_all_experimental_fixtures_survive_typed_roundtrip():
    for family in EXPERIMENTAL_FIXTURE_FAMILIES:
        data = load_fixture(family)
        roundtrip = experimental_report_roundtrip(data)

        assert roundtrip == data, f"{family}: fixture changed during typed roundtrip"


def test_experimental_version_set_matches_fixture_families():
    """Each fixture family's schema_version is in EXPERIMENTAL_SCHEMA_VERSIONS."""
    from iv8_rs.experimental_report import EXPERIMENTAL_SCHEMA_VERSIONS

    for family in EXPERIMENTAL_FIXTURE_FAMILIES:
        data = load_fixture(family)
        assert data["schema_version"] in EXPERIMENTAL_SCHEMA_VERSIONS, (
            f"{family}: unknown schema_version {data['schema_version']!r}"
        )
