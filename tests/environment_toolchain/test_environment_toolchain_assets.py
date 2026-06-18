from __future__ import annotations

import json

import pytest
from iv8_rs.environment_toolchain_asset_loading import (
    _resolve_probe_pack,
)
from iv8_rs.environment_toolchain_asset_loading import (
    available_candidate_targets as asset_available_candidate_targets,
)
from iv8_rs.environment_toolchain_asset_loading import (
    available_probe_packs as asset_available_probe_packs,
)
from iv8_rs.environment_toolchain_asset_loading import (
    load_candidate_pack as asset_load_candidate_pack,
)
from iv8_rs.environment_toolchain_asset_loading import (
    load_probe_pack as asset_load_probe_pack,
)
from iv8_rs.environment_toolchain_asset_models import (
    CandidatePack,
    ProbeDefinition,
    ProbePack,
    ToolchainCandidate,
)
from iv8_rs.environment_toolchain_probe_taxonomy import future_placeholder_probe_packs
from iv8_rs.environment_toolchain_runtime import (
    CandidatePack as RuntimeCandidatePack,
)
from iv8_rs.environment_toolchain_runtime import (
    ProbeDefinition as RuntimeProbeDefinition,
)
from iv8_rs.environment_toolchain_runtime import (
    ProbePack as RuntimeProbePack,
)
from iv8_rs.environment_toolchain_runtime import (
    ToolchainCandidate as RuntimeToolchainCandidate,
)
from iv8_rs.environment_toolchain_runtime import (
    available_candidate_targets,
    available_probe_packs,
    load_candidate_pack,
    load_probe_pack,
)


def candidate_data(**overrides):
    data = {
        "patch_id": "navigator.language.default.v0",
        "target": "navigator.language",
        "target_family": "environment_value",
        "kind": "value",
        "policy": "runtime_safe",
        "source": "builtin_registry",
        "value_preview": "en-US",
        "requires": [],
        "risk_reasons": [],
        "reversible": True,
        "validation": {"probe_pack": "fingerprint.m1"},
    }
    data.update(overrides)
    return data


def probe_data(**overrides):
    data = {
        "probe_id": "navigator.language.present",
        "target": "navigator.language",
        "category": "presence",
        "js": "return typeof navigator.language === 'string';",
        "expected": True,
        "gap_class": "missing_api",
        "side_effects": [],
        "cleanup": "none",
        "evidence_ceiling": "diagnostic_only",
    }
    data.update(overrides)
    return data


def test_asset_models_runtime_reexports_match_asset_module():
    assert RuntimeToolchainCandidate is ToolchainCandidate
    assert RuntimeCandidatePack is CandidatePack
    assert RuntimeProbeDefinition is ProbeDefinition
    assert RuntimeProbePack is ProbePack


def test_candidate_pack_to_dict_shape_is_stable():
    pack = CandidatePack(
        candidate_pack="custom.candidate",
        version=1,
        description="custom candidate pack",
        candidates=[ToolchainCandidate.from_dict(candidate_data())],
    )

    assert pack.to_dict() == {
        "candidate_pack": "custom.candidate",
        "version": 1,
        "description": "custom candidate pack",
        "candidates": [candidate_data(metadata={})],
    }


def test_probe_pack_to_dict_shape_is_stable():
    pack = ProbePack(
        probe_pack="custom.probe",
        version=1,
        description="custom probe pack",
        evidence_ceiling="diagnostic_only",
        probes=[ProbeDefinition.from_dict(probe_data())],
    )

    assert pack.to_dict() == {
        "probe_pack": "custom.probe",
        "version": 1,
        "description": "custom probe pack",
        "evidence_ceiling": "diagnostic_only",
        "probes": [probe_data()],
    }


def test_asset_model_validation_remains_in_asset_module():
    with pytest.raises(ValueError, match="runtime_safe"):
        ToolchainCandidate.from_dict(candidate_data(policy="analysis_only"))

    with pytest.raises(ValueError, match="side effects"):
        ProbeDefinition.from_dict(probe_data(side_effects=["writes_global"]))


def test_asset_model_module_does_not_import_runtime():
    import iv8_rs.environment_toolchain_asset_models as asset_module

    module_names = {
        value.__name__
        for value in vars(asset_module).values()
        if getattr(value, "__name__", None)
    }

    assert "iv8_rs.environment_toolchain_runtime" not in module_names


def test_asset_loading_module_direct_import_matches_runtime_reexport():
    assert asset_available_probe_packs() == available_probe_packs()
    assert asset_available_candidate_targets() == available_candidate_targets()
    assert asset_load_probe_pack("fingerprint.m1").to_dict() == load_probe_pack(
        "fingerprint.m1"
    ).to_dict()
    assert asset_load_candidate_pack("chrome_generic").to_dict() == load_candidate_pack(
        "chrome_generic"
    ).to_dict()


def test_asset_loading_available_probe_packs_excludes_taxonomy_placeholders():
    assert asset_available_probe_packs() == ["descriptor.m1", "fingerprint.m1"]
    assert set(future_placeholder_probe_packs()).isdisjoint(asset_available_probe_packs())


def test_asset_loading_rejects_placeholder_probe_pack_ids():
    for pack_id in future_placeholder_probe_packs():
        with pytest.raises(ValueError, match="unknown probe pack"):
            asset_load_probe_pack(pack_id)


def test_custom_probe_pack_path_provenance_uses_basename_only(tmp_path):
    path = tmp_path / "nested" / "custom-probe-pack.json"
    path.parent.mkdir()
    pack_data = {
        "probe_pack": "custom.path.probe",
        "version": 1,
        "description": "custom path probe pack",
        "evidence_ceiling": "diagnostic_only",
        "probes": [probe_data()],
    }
    path.write_text(json.dumps(pack_data), encoding="utf-8")

    pack, provenance = _resolve_probe_pack(path)

    assert pack.probe_pack == "custom.path.probe"
    assert provenance.origin == "custom_path"
    assert provenance.redacted_ref == "custom-probe-pack.json"
    assert str(path.parent) not in provenance.redacted_ref


def test_asset_loading_module_does_not_import_runtime():
    import iv8_rs.environment_toolchain_asset_loading as loading_module

    module_names = {
        value.__name__
        for value in vars(loading_module).values()
        if getattr(value, "__name__", None)
    }

    assert "iv8_rs.environment_toolchain_runtime" not in module_names
