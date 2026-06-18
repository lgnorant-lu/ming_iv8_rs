from __future__ import annotations

import json
from importlib import resources
from pathlib import Path

import pytest
from iv8_rs.environment_toolchain import toolchain_report_to_dict
from iv8_rs.environment_toolchain_candidate_mapping import (
    map_gaps_to_candidates as candidate_mapping_map_gaps_to_candidates,
)
from iv8_rs.environment_toolchain_runtime import (
    CandidatePack,
    EnvironmentGap,
    ProbeDefinition,
    ProbePack,
    ToolchainCandidate,
    available_candidate_targets,
    load_candidate_pack,
    map_gaps_to_candidates,
    run_environment_toolchain,
)

ROOT = Path(__file__).resolve().parents[2]
CANDIDATE_FIXTURE_DIR = (
    ROOT / "tests" / "fixtures" / "environment_toolchain" / "candidate_packs"
)


def gap(target: str, gap_class: str) -> EnvironmentGap:
    return EnvironmentGap(
        probe_id=f"{target}.probe",
        target=target,
        gap_class=gap_class,
        category="presence",
        expected=True,
        actual=False,
    )


def test_candidate_mapping_module_direct_import_matches_runtime_reexport():
    gaps = [gap("navigator.languages", "missing_api")]

    assert [
        candidate.to_dict() for candidate in candidate_mapping_map_gaps_to_candidates(gaps)
    ] == [candidate.to_dict() for candidate in map_gaps_to_candidates(gaps)]


def test_candidate_mapping_module_does_not_import_runtime():
    import iv8_rs.environment_toolchain_candidate_mapping as candidate_mapping_module

    module_names = {
        value.__name__
        for value in vars(candidate_mapping_module).values()
        if getattr(value, "__name__", None)
    }

    assert "iv8_rs.environment_toolchain_runtime" not in module_names


def custom_candidate_pack_data(**overrides) -> dict:
    data = {
        "candidate_pack": "custom.chrome.values",
        "version": 1,
        "description": "minimal custom runtime-safe candidate pack",
        "candidates": [
            {
                "patch_id": "navigator.language.custom.v0",
                "target": "navigator.language",
                "target_family": "environment_value",
                "kind": "value",
                "policy": "runtime_safe",
                "source": "custom_pack",
                "value_preview": "en-US",
                "requires": [],
                "risk_reasons": [],
                "reversible": True,
                "validation": {
                    "probe_pack": "custom.minimal",
                    "expected_delta": ["navigator.language"],
                    "gap_classes": ["missing_api", "value_mismatch"],
                },
            }
        ],
    }
    data.update(overrides)
    return data


def load_candidate_fixture(name: str) -> dict:
    with (CANDIDATE_FIXTURE_DIR / name).open("r", encoding="utf-8") as fh:
        return json.load(fh)


def test_candidate_registry_exposes_generic_runtime_safe_targets():
    assert available_candidate_targets() == [
        "document.readyState",
        "navigator.deviceMemory",
        "navigator.hardwareConcurrency",
        "navigator.language",
        "navigator.languages",
        "navigator.platform",
        "navigator.webdriver",
        "screen.height",
        "screen.width",
    ]


def test_load_candidate_pack_loads_builtin_pack():
    pack = load_candidate_pack("chrome_generic")

    assert isinstance(pack, CandidatePack)
    assert pack.candidate_pack == "chrome_generic"
    assert pack.candidates


def test_custom_candidate_pack_loads_from_dict():
    pack = load_candidate_pack(custom_candidate_pack_data())

    assert pack.candidate_pack == "custom.chrome.values"
    assert [candidate.patch_id for candidate in pack.candidates] == [
        "navigator.language.custom.v0"
    ]
    assert pack.candidates[0].metadata == {}


def test_custom_candidate_pack_accepts_optional_metadata_dict():
    data = custom_candidate_pack_data()
    data["candidates"][0]["metadata"] = {
        "coherence_group": "language",
        "substrate_family": "navigator",
        "dependency_kind": ["probe_gap", "candidate_pack_enabled"],
        "expected_probe_delta": ["navigator.language"],
        "evidence_ceiling": "diagnostic_only",
        "planning_status": "not_planned",
        "rollback_scope": "context_only",
        "rollback_hint": {"capture_before": ["navigator.language"]},
        "boundary_checked": True,
        "blocked_reasons": [],
    }

    pack = load_candidate_pack(data)

    assert pack.candidates[0].metadata["coherence_group"] == "language"
    assert pack.candidates[0].metadata["planning_status"] == "not_planned"
    assert map_gaps_to_candidates(
        [gap("navigator.language", "missing_api")],
        candidate_pack=pack,
    )[0].patch_id == "navigator.language.custom.v0"


def test_custom_candidate_pack_accepts_top_level_optional_metadata_fields():
    data = custom_candidate_pack_data()
    data["candidates"][0]["coherence_group"] = "language"
    data["candidates"][0]["rollback_scope"] = "ephemeral_report"

    pack = load_candidate_pack(data)

    assert pack.candidates[0].metadata == {
        "coherence_group": "language",
        "rollback_scope": "ephemeral_report",
    }


def test_custom_candidate_pack_rejects_applied_metadata_status():
    data = custom_candidate_pack_data()
    data["candidates"][0]["metadata"] = {"planning_status": "applied"}

    with pytest.raises(ValueError, match="planning status"):
        load_candidate_pack(data)


def test_custom_candidate_pack_rejects_strong_metadata_evidence():
    data = custom_candidate_pack_data()
    data["candidates"][0]["metadata"] = {"evidence_ceiling": "strong"}

    with pytest.raises(ValueError, match="evidence ceiling"):
        load_candidate_pack(data)


def test_custom_candidate_pack_rejects_target_flow_metadata():
    data = custom_candidate_pack_data()
    data["candidates"][0]["metadata"] = {
        "rollback_hint": {"note": "copy token from endpoint"},
    }

    with pytest.raises(ValueError, match="boundary validation"):
        load_candidate_pack(data)


def test_custom_candidate_pack_rejects_request_sequence_dependency_kind():
    data = custom_candidate_pack_data()
    data["candidates"][0]["metadata"] = {"dependency_kind": ["request_sequence"]}

    with pytest.raises(ValueError, match="dependency kind"):
        load_candidate_pack(data)


def test_custom_candidate_pack_rejects_unknown_metadata_field():
    data = custom_candidate_pack_data()
    data["candidates"][0]["metadata"] = {"policy": "unsafe_hook"}

    with pytest.raises(ValueError, match="unknown candidate metadata field"):
        load_candidate_pack(data)


def test_candidate_metadata_does_not_change_default_applied_patches():
    data = custom_candidate_pack_data()
    data["candidates"][0]["metadata"] = {
        "planning_status": "eligible_for_review",
        "rollback_scope": "context_only",
        "boundary_checked": True,
    }

    report = run_environment_toolchain(
        "",
        probe_pack=ProbePack(
            probe_pack="test.metadata.no.apply",
            version=1,
            description="metadata no-apply pack",
            evidence_ceiling="diagnostic_only",
            probes=[
                ProbeDefinition(
                    probe_id="navigator.language.force_gap",
                    target="navigator.language",
                    category="value",
                    js="return false;",
                    expected=True,
                    gap_class="value_mismatch",
                )
            ],
        ),
        candidate_pack=data,
        profile=None,
    )
    report_data = toolchain_report_to_dict(report)

    assert report_data["applied_patches"] == []
    assert [patch["patch_id"] for patch in report_data["rejected_patches"]] == [
        "navigator.language.custom.v0"
    ]


def test_custom_candidate_pack_loads_from_path(tmp_path: Path):
    path = tmp_path / "custom-candidate-pack.json"
    path.write_text(json.dumps(custom_candidate_pack_data()), encoding="utf-8")

    pack = load_candidate_pack(path)

    assert pack.candidate_pack == "custom.chrome.values"


def test_custom_candidate_pack_rejects_builtin_override():
    data = custom_candidate_pack_data(candidate_pack="chrome_generic")

    with pytest.raises(ValueError, match="cannot override built-in"):
        load_candidate_pack(data)


def test_custom_candidate_pack_rejects_boundary_payload_before_mapping():
    data = custom_candidate_pack_data(description="copy token from endpoint before rerun")

    with pytest.raises(ValueError, match="boundary validation"):
        load_candidate_pack(data)


def test_custom_candidate_pack_rejects_analysis_only_policy():
    data = custom_candidate_pack_data()
    data["candidates"][0]["policy"] = "analysis_only"

    with pytest.raises(ValueError, match="runtime_safe"):
        load_candidate_pack(data)


def test_custom_candidate_pack_rejects_unsafe_hook_policy():
    data = custom_candidate_pack_data()
    data["candidates"][0]["policy"] = "unsafe_hook"

    with pytest.raises(ValueError, match="runtime_safe"):
        load_candidate_pack(data)


def test_custom_candidate_pack_path_rejects_malformed_json(tmp_path: Path):
    path = tmp_path / "bad-candidate-pack.json"
    path.write_text("{not json", encoding="utf-8")

    with pytest.raises(ValueError, match="invalid candidate pack JSON"):
        load_candidate_pack(path)


def test_custom_candidate_pack_fixture_loads_from_path():
    pack = load_candidate_pack(CANDIDATE_FIXTURE_DIR / "custom.valid.json")

    assert pack.candidate_pack == "custom.fixture.candidate"
    assert pack.candidates[0].patch_id == "navigator.language.fixture.v0"


def test_custom_candidate_pack_blocked_fixture_fails_before_mapping():
    with pytest.raises(ValueError, match="boundary validation"):
        load_candidate_pack(CANDIDATE_FIXTURE_DIR / "custom.blocked.json")


def test_custom_candidate_pack_malformed_fixture_fails_before_mapping():
    with pytest.raises(ValueError, match="runtime_safe"):
        load_candidate_pack(CANDIDATE_FIXTURE_DIR / "custom.malformed.json")


def test_custom_valid_candidate_fixture_contains_no_target_specific_vocabulary():
    fixture_text = (CANDIDATE_FIXTURE_DIR / "custom.valid.json").read_text(
        encoding="utf-8"
    ).lower()
    blocked_terms = ["cookie", "token", "signature", "endpoint", "domain", "nonce"]

    assert not [term for term in blocked_terms if term in fixture_text]


def test_map_gaps_to_candidates_maps_languages_gap():
    candidates = map_gaps_to_candidates([gap("navigator.languages", "missing_api")])

    assert len(candidates) == 1
    candidate = candidates[0]
    assert candidate.patch_id == "navigator.languages.default.v0"
    assert candidate.target == "navigator.languages"
    assert candidate.policy == "runtime_safe"
    assert candidate.target_family == "environment_value"
    assert candidate.value_preview == ["en-US", "en"]
    assert candidate.reversible is True
    assert candidate.validation["probe_pack"] == "fingerprint.m1"


def test_map_gaps_to_candidates_maps_custom_candidate_pack():
    candidates = map_gaps_to_candidates(
        [gap("navigator.language", "missing_api")],
        candidate_pack=custom_candidate_pack_data(),
    )

    assert [candidate.patch_id for candidate in candidates] == ["navigator.language.custom.v0"]


def test_map_gaps_to_candidates_can_disable_candidate_pack():
    candidates = map_gaps_to_candidates(
        [gap("navigator.languages", "missing_api")],
        candidate_pack=None,
    )

    assert candidates == []


def test_map_gaps_to_candidates_preserves_explicit_environment_precedence():
    candidates = map_gaps_to_candidates(
        [gap("navigator.languages", "missing_api")],
        environment={"navigator.languages": ["fr-FR", "fr"]},
    )

    assert candidates == []


def test_map_gaps_to_candidates_ignores_unknown_targets():
    candidates = map_gaps_to_candidates([gap("navigator.plugins", "missing_api")])

    assert candidates == []


def test_map_gaps_to_candidates_respects_gap_class_validation():
    candidates = map_gaps_to_candidates([gap("navigator.webdriver", "missing_api")])

    assert candidates == []


def test_map_gaps_to_candidates_deduplicates_patch_ids():
    candidates = map_gaps_to_candidates([
        gap("screen.width", "missing_api"),
        gap("screen.width", "value_mismatch"),
    ])

    assert [candidate.patch_id for candidate in candidates] == ["screen.width.default.v0"]


def test_toolchain_candidate_rejects_non_runtime_safe_policy():
    with pytest.raises(ValueError, match="runtime_safe"):
        ToolchainCandidate(
            patch_id="eval.capture.v0",
            target="eval",
            target_family="network_observer",
            kind="capture",
            policy="analysis_only",
            source="builtin_registry",
            value_preview=None,
        )


def test_candidate_registry_contains_no_target_specific_vocabulary():
    candidates = map_gaps_to_candidates([
        gap("navigator.languages", "missing_api"),
        gap("navigator.webdriver", "value_mismatch"),
        gap("screen.width", "missing_api"),
    ])
    registry_text = repr([candidate.to_dict() for candidate in candidates]).lower()
    blocked_terms = ["cookie", "token", "signature", "endpoint", "domain", "nonce"]

    assert not [term for term in blocked_terms if term in registry_text]


def test_candidate_asset_contains_no_target_specific_vocabulary():
    asset_text = resources.files(
        "iv8_rs.environment_toolchain_assets.candidates"
    ).joinpath("chrome_generic.json").read_text(encoding="utf-8")
    data = json.loads(asset_text)
    registry_text = repr(data).lower()
    blocked_terms = ["cookie", "token", "signature", "endpoint", "domain", "nonce"]

    assert data["candidate_pack"] == "chrome_generic"
    assert not [term for term in blocked_terms if term in registry_text]


def test_map_gaps_to_candidates_maps_expanded_value_targets():
    candidates = map_gaps_to_candidates([
        gap("navigator.language", "missing_api"),
        gap("navigator.platform", "missing_api"),
        gap("navigator.hardwareConcurrency", "missing_api"),
        gap("navigator.deviceMemory", "missing_api"),
        gap("screen.height", "missing_api"),
        gap("document.readyState", "missing_api"),
    ])

    assert [candidate.target for candidate in candidates] == [
        "navigator.language",
        "navigator.platform",
        "navigator.hardwareConcurrency",
        "navigator.deviceMemory",
        "screen.height",
        "document.readyState",
    ]
