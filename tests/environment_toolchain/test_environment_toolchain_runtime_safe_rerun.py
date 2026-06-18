from __future__ import annotations

from iv8_rs.environment_toolchain import toolchain_report_to_dict
from iv8_rs.environment_toolchain_runtime import (
    ProbeDefinition,
    ProbePack,
    run_environment_toolchain,
)


def make_pack(*probes: ProbeDefinition) -> ProbePack:
    return ProbePack(
        probe_pack="test.runtime.safe",
        version=1,
        description="runtime-safe rerun synthetic pack",
        evidence_ceiling="diagnostic_only",
        probes=list(probes),
    )


def languages_pack() -> ProbePack:
    return make_pack(ProbeDefinition(
        probe_id="navigator.languages.chrome_shape",
        target="navigator.languages",
        category="value",
        js="return navigator.languages[0] === 'en-US' && navigator.languages[1] === 'en';",
        expected=True,
        gap_class="value_mismatch",
    ))


def test_runtime_safe_apply_moves_candidate_to_applied_patches():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        profile=None,
        apply_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)

    assert [patch["patch_id"] for patch in data["applied_patches"]] == [
        "navigator.languages.default.v0"
    ]
    assert data["rejected_patches"] == []
    assert data["coverage_delta"]["improved"] == 1
    assert data["coverage_delta"]["unresolved"] == 0


def test_runtime_safe_apply_uses_fresh_after_context():
    report = run_environment_toolchain(
        "globalThis.beforeOnly = true;",
        probe_pack=languages_pack(),
        profile=None,
        apply_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)

    assert data["before"] == {"present": 0, "missing": 0, "mismatch": 1}
    assert data["after"] == {"present": 1, "missing": 0, "mismatch": 0}


def test_runtime_safe_apply_emits_only_weak_or_diagnostic_evidence():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        profile=None,
        apply_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)
    strengths = {item["strength"] for item in data["evidence"]}

    assert "strong" not in strengths
    assert strengths <= {"diagnostic_only", "weak"}
    assert {"environment_patch_applied", "environment_coverage_improved"}.issubset(
        {item["kind"] for item in data["evidence"]}
    )


def test_runtime_safe_apply_respects_explicit_environment_conflict():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        profile=None,
        environment={"navigator.languages": ["fr-FR", "fr"]},
        apply_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)

    assert data["applied_patches"] == []
    assert data["coverage_delta"]["improved"] == 0


def test_runtime_safe_apply_uses_custom_candidate_pack():
    candidate_pack = {
        "candidate_pack": "custom.languages",
        "version": 1,
        "description": "custom runtime-safe language values",
        "candidates": [
            {
                "patch_id": "navigator.languages.custom.v0",
                "target": "navigator.languages",
                "target_family": "environment_value",
                "kind": "value",
                "policy": "runtime_safe",
                "source": "custom_pack",
                "value_preview": ["en-US", "en"],
                "requires": [],
                "risk_reasons": [],
                "reversible": True,
                "validation": {
                    "probe_pack": "custom.languages",
                    "expected_delta": ["navigator.languages"],
                    "gap_classes": ["value_mismatch"],
                },
            }
        ],
    }
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        candidate_pack=candidate_pack,
        profile=None,
        apply_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)

    assert [patch["patch_id"] for patch in data["applied_patches"]] == [
        "navigator.languages.custom.v0"
    ]
    assert data["coverage_delta"]["improved"] == 1


def test_runtime_safe_apply_can_disable_candidate_pack():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        candidate_pack=None,
        profile=None,
        apply_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)

    assert data["applied_patches"] == []
    assert data["coverage_delta"]["improved"] == 0
