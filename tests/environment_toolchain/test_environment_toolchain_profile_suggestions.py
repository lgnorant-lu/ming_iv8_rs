from __future__ import annotations

from iv8_rs.environment_toolchain import toolchain_report_to_dict
from iv8_rs.environment_toolchain_runtime import (
    ProbeDefinition,
    ProbePack,
    run_environment_toolchain,
)


def make_pack(*probes: ProbeDefinition) -> ProbePack:
    return ProbePack(
        probe_pack="test.profile.suggestion",
        version=1,
        description="profile suggestion synthetic pack",
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


def test_report_only_runner_emits_review_only_profile_suggestion():
    report = run_environment_toolchain("", probe_pack=languages_pack(), profile=None)
    data = toolchain_report_to_dict(report)

    assert data["profile_suggestions"] == [
        {"target": "navigator.languages", "value_preview": ["en-US", "en"]}
    ]
    assert data["writes"] == []
    assert any(
        diagnostic["code"] == "ENV_TOOLCHAIN_PROFILE_SUGGESTION_REVIEW"
        for diagnostic in data["diagnostics"]
    )


def test_runtime_safe_apply_keeps_profile_suggestion_review_only():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        profile=None,
        apply_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)

    assert data["profile_suggestions"] == [
        {"target": "navigator.languages", "value_preview": ["en-US", "en"]}
    ]
    assert data["writes"] == []
    assert data["applied_patches"]


def test_explicit_environment_conflict_suppresses_profile_suggestion():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        profile=None,
        environment={"navigator.languages": ["fr-FR", "fr"]},
        apply_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)

    assert data["profile_suggestions"] == []
    assert data["writes"] == []
