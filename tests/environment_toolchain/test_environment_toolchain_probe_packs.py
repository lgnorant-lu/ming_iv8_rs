from __future__ import annotations

import json
from importlib import resources
from pathlib import Path

import pytest
from iv8_rs.environment_toolchain_probe_taxonomy import future_placeholder_probe_packs
from iv8_rs.environment_toolchain_runtime import (
    ProbeDefinition,
    ProbePack,
    available_probe_packs,
    load_probe_pack,
    probe_pack_from_dict,
    probe_pack_to_dict,
)

ROOT = Path(__file__).resolve().parents[2]
FIXTURE = (
    ROOT
    / "tests"
    / "fixtures"
    / "environment_toolchain"
    / "probe_packs"
    / "fingerprint.m1.json"
)
PROBE_FIXTURE_DIR = FIXTURE.parent


def load_probe_pack_fixture() -> dict:
    with FIXTURE.open("r", encoding="utf-8") as fh:
        return json.load(fh)


def load_probe_fixture(name: str) -> dict:
    with (PROBE_FIXTURE_DIR / name).open("r", encoding="utf-8") as fh:
        return json.load(fh)


def custom_probe_pack_data(**overrides) -> dict:
    data = {
        "probe_pack": "custom.minimal",
        "version": 1,
        "description": "minimal custom probe pack",
        "evidence_ceiling": "diagnostic_only",
        "probes": [
            {
                "probe_id": "navigator.language.custom.present",
                "target": "navigator.language",
                "category": "presence",
                "js": (
                    "return typeof navigator.language === 'string' "
                    "&& navigator.language.length > 0;"
                ),
                "expected": True,
                "gap_class": "missing_api",
                "side_effects": [],
                "cleanup": "none",
                "evidence_ceiling": "diagnostic_only",
            }
        ],
    }
    data.update(overrides)
    return data


def test_fingerprint_m1_probe_pack_roundtrip_matches_fixture():
    fixture = load_probe_pack_fixture()
    pack = load_probe_pack("fingerprint.m1")

    assert probe_pack_to_dict(pack) == fixture
    assert probe_pack_from_dict(fixture) == pack


def test_fingerprint_m1_asset_matches_contract_fixture():
    asset_text = resources.files(
        "iv8_rs.environment_toolchain_assets.probe_packs"
    ).joinpath("_archive/fingerprint.m1.json").read_text(encoding="utf-8")

    assert json.loads(asset_text) == load_probe_pack_fixture()


def test_available_probe_packs_lists_fingerprint_m1():
    assert available_probe_packs() == ["descriptor.m1", "fingerprint.m1"]


def test_future_placeholder_probe_packs_are_not_loadable_builtins():
    assert set(future_placeholder_probe_packs()).isdisjoint(available_probe_packs())

    for pack_id in future_placeholder_probe_packs():
        with pytest.raises(ValueError, match="unknown probe pack"):
            load_probe_pack(pack_id)


def test_descriptor_m1_asset_matches_contract_fixture():
    asset_text = resources.files(
        "iv8_rs.environment_toolchain_assets.probe_packs"
    ).joinpath("_archive/descriptor.m1.json").read_text(encoding="utf-8")

    assert json.loads(asset_text) == load_probe_fixture("descriptor.m1.json")


def test_descriptor_m1_probe_pack_loads_as_diagnostic_only():
    pack = load_probe_pack("descriptor.m1")

    assert pack.probe_pack == "descriptor.m1"
    assert pack.evidence_ceiling == "diagnostic_only"
    assert len(pack.probes) == 8
    assert {probe.gap_class for probe in pack.probes} == {
        "descriptor_mismatch",
        "prototype_chain_mismatch",
    }
    assert all(probe.evidence_ceiling == "diagnostic_only" for probe in pack.probes)
    assert all(probe.side_effects == [] and probe.cleanup == "none" for probe in pack.probes)


def test_probe_pack_requires_diagnostic_only_evidence():
    data = load_probe_pack_fixture()
    data["evidence_ceiling"] = "weak"

    with pytest.raises(ValueError, match="diagnostic_only"):
        probe_pack_from_dict(data)


def test_probe_definition_rejects_weak_evidence_before_runner_review():
    with pytest.raises(ValueError, match="weak evidence"):
        ProbeDefinition(
            probe_id="navigator.language.present",
            target="navigator.language",
            category="presence",
            js="return typeof navigator.language === 'string';",
            expected=True,
            gap_class="missing_api",
            evidence_ceiling="weak",
        )


def test_probe_pack_rejects_duplicate_probe_ids():
    probe = ProbeDefinition(
        probe_id="navigator.language.present",
        target="navigator.language",
        category="presence",
        js="return typeof navigator.language === 'string';",
        expected=True,
        gap_class="missing_api",
    )

    with pytest.raises(ValueError, match="duplicate probe ids"):
        ProbePack(
            probe_pack="test.duplicates",
            version=1,
            description="duplicate probe test",
            evidence_ceiling="diagnostic_only",
            probes=[probe, probe],
        )


def test_unknown_probe_pack_fails_clearly():
    with pytest.raises(ValueError, match="unknown probe pack"):
        load_probe_pack("sample.sign")


def test_custom_probe_pack_loads_from_dict():
    pack = load_probe_pack(custom_probe_pack_data())

    assert pack.probe_pack == "custom.minimal"
    assert [probe.probe_id for probe in pack.probes] == ["navigator.language.custom.present"]


def test_custom_probe_pack_loads_from_path(tmp_path: Path):
    path = tmp_path / "custom-probe-pack.json"
    path.write_text(json.dumps(custom_probe_pack_data()), encoding="utf-8")

    pack = load_probe_pack(path)

    assert pack.probe_pack == "custom.minimal"


def test_custom_probe_pack_rejects_builtin_override():
    data = custom_probe_pack_data(probe_pack="fingerprint.m1")

    with pytest.raises(ValueError, match="cannot override built-in"):
        load_probe_pack(data)


def test_custom_probe_pack_rejects_boundary_payload_before_load():
    data = custom_probe_pack_data(description="copy token from endpoint before rerun")

    with pytest.raises(ValueError, match="boundary validation"):
        load_probe_pack(data)


def test_custom_probe_pack_rejects_unsupported_side_effects():
    data = custom_probe_pack_data()
    data["probes"][0]["side_effects"] = ["writes_global"]

    with pytest.raises(ValueError, match="side effects"):
        load_probe_pack(data)


def test_custom_probe_pack_rejects_unsupported_cleanup():
    data = custom_probe_pack_data()
    data["probes"][0]["cleanup"] = "reset_global"

    with pytest.raises(ValueError, match="cleanup"):
        load_probe_pack(data)


def test_custom_probe_pack_path_rejects_malformed_json(tmp_path: Path):
    path = tmp_path / "bad-probe-pack.json"
    path.write_text("{not json", encoding="utf-8")

    with pytest.raises(ValueError, match="invalid probe pack JSON"):
        load_probe_pack(path)


def test_custom_probe_pack_fixture_loads_from_path():
    pack = load_probe_pack(PROBE_FIXTURE_DIR / "custom.valid.json")

    assert pack.probe_pack == "custom.fixture.probe"
    assert pack.probes[0].probe_id == "navigator.language.fixture.present"


def test_custom_probe_pack_blocked_fixture_fails_before_load():
    with pytest.raises(ValueError, match="boundary validation"):
        load_probe_pack(PROBE_FIXTURE_DIR / "custom.blocked.json")


def test_custom_probe_pack_malformed_fixture_fails_before_load():
    with pytest.raises(ValueError, match="side effects"):
        load_probe_pack(PROBE_FIXTURE_DIR / "custom.malformed.json")


def test_custom_valid_probe_fixture_contains_no_target_specific_vocabulary():
    fixture_text = (PROBE_FIXTURE_DIR / "custom.valid.json").read_text(encoding="utf-8").lower()
    blocked_terms = ["cookie", "token", "signature", "endpoint", "domain", "nonce"]

    assert not [term for term in blocked_terms if term in fixture_text]


def test_probe_pack_fixture_contains_no_target_specific_vocabulary():
    fixture_text = FIXTURE.read_text(encoding="utf-8").lower()
    blocked_terms = ["cookie", "token", "signature", "endpoint", "domain", "nonce"]

    assert not [term for term in blocked_terms if term in fixture_text]
