from __future__ import annotations

import pytest
from iv8_rs.environment_toolchain import toolchain_report_to_dict
from iv8_rs.environment_toolchain_runtime import (
    CandidatePack,
    ProbeDefinition,
    ProbePack,
    ToolchainCandidate,
    run_environment_toolchain,
)


def make_pack(*probes: ProbeDefinition) -> ProbePack:
    return ProbePack(
        probe_pack="test.iterative.adaptation",
        version=1,
        description="iterative adaptation synthetic pack",
        evidence_ceiling="diagnostic_only",
        probes=list(probes),
    )


def candidate(
    patch_id: str,
    target: str,
    value,
    *,
    gap_class: str = "value_mismatch",
) -> ToolchainCandidate:
    return ToolchainCandidate(
        patch_id=patch_id,
        target=target,
        target_family="environment_value",
        kind="value",
        policy="runtime_safe",
        source="test_pack",
        value_preview=value,
        validation={"gap_classes": [gap_class]},
    )


def candidate_pack(*candidates: ToolchainCandidate) -> CandidatePack:
    return CandidatePack(
        candidate_pack="test.iterative.candidates",
        version=1,
        description="iterative adaptation candidate pack",
        candidates=list(candidates),
    )


def summary(data: dict) -> dict:
    return next(
        diagnostic["details"]
        for diagnostic in data["diagnostics"]
        if diagnostic["code"] == "ENV_TOOLCHAIN_ADAPTATION_SUMMARY"
    )


def iterations(data: dict) -> list[dict]:
    return [
        diagnostic["details"]
        for diagnostic in data["diagnostics"]
        if diagnostic["code"] == "ENV_TOOLCHAIN_ADAPTATION_ITERATION"
    ]


def test_iterative_adaptation_improves_one_probe_and_stops_completed():
    pack = make_pack(ProbeDefinition(
        probe_id="navigator.languages.chrome_shape",
        target="navigator.languages",
        category="value",
        js="return navigator.languages[0] === 'en-US' && navigator.languages[1] === 'en';",
        expected=True,
        gap_class="value_mismatch",
    ))
    candidates = candidate_pack(candidate(
        "navigator.languages.default.v0",
        "navigator.languages",
        ["en-US", "en"],
    ))

    report = run_environment_toolchain(
        "",
        probe_pack=pack,
        candidate_pack=candidates,
        profile=None,
        adapt_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)

    assert data["coverage_delta"] == {"improved": 1, "regressed": 0, "unresolved": 0}
    assert [patch["patch_id"] for patch in data["applied_patches"]] == [
        "navigator.languages.default.v0"
    ]
    assert data["writes"] == []
    assert summary(data)["stop_reason"] == "completed"
    assert iterations(data)[0]["delta"] == {"improved": 1, "regressed": 0, "unresolved": 0}


def test_iterative_adaptation_budget_exhausted_after_one_of_two_gaps():
    pack = make_pack(
        ProbeDefinition(
            probe_id="navigator.languages.chrome_shape",
            target="navigator.languages",
            category="value",
            js="return navigator.languages[0] === 'en-US' && navigator.languages[1] === 'en';",
            expected=True,
            gap_class="value_mismatch",
        ),
        ProbeDefinition(
            probe_id="screen.width.desktop",
            target="screen.width",
            category="value",
            js="return screen.width === 1440;",
            expected=True,
            gap_class="value_mismatch",
        ),
    )
    candidates = candidate_pack(
        candidate("navigator.languages.default.v0", "navigator.languages", ["en-US", "en"]),
        candidate("screen.width.default.v0", "screen.width", 1440),
    )

    report = run_environment_toolchain(
        "",
        probe_pack=pack,
        candidate_pack=candidates,
        profile=None,
        adapt_runtime_safe=True,
        max_iterations=1,
    )
    data = toolchain_report_to_dict(report)

    assert summary(data)["stop_reason"] == "budget_exhausted"
    assert summary(data)["applied_patch_ids"] == ["navigator.languages.default.v0"]
    assert data["coverage_delta"] == {"improved": 1, "regressed": 0, "unresolved": 1}


def test_iterative_adaptation_candidate_pack_none_stops_no_candidate():
    pack = make_pack(ProbeDefinition(
        probe_id="navigator.languages.chrome_shape",
        target="navigator.languages",
        category="value",
        js="return navigator.languages[0] === 'en-US' && navigator.languages[1] === 'en';",
        expected=True,
        gap_class="value_mismatch",
    ))

    report = run_environment_toolchain(
        "",
        probe_pack=pack,
        candidate_pack=None,
        profile=None,
        adapt_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)

    assert data["applied_patches"] == []
    assert summary(data)["stop_reason"] == "no_candidate"
    assert any(
        diagnostic["code"] == "ENV_TOOLCHAIN_CANDIDATE_PACK_DISABLED"
        for diagnostic in data["diagnostics"]
    )


def test_iterative_adaptation_no_progress_stops():
    pack = make_pack(ProbeDefinition(
        probe_id="navigator.languages.chrome_shape",
        target="navigator.languages",
        category="value",
        js="return navigator.languages[0] === 'en-US' && navigator.languages[1] === 'en';",
        expected=True,
        gap_class="value_mismatch",
    ))
    candidates = candidate_pack(candidate(
        "navigator.languages.wrong.v0",
        "navigator.languages",
        ["fr-FR", "fr"],
    ))

    report = run_environment_toolchain(
        "",
        probe_pack=pack,
        candidate_pack=candidates,
        profile=None,
        adapt_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)

    assert summary(data)["stop_reason"] == "no_progress"
    assert data["coverage_delta"] == {"improved": 0, "regressed": 0, "unresolved": 1}


def test_iterative_adaptation_regression_stops():
    pack = make_pack(
        ProbeDefinition(
            probe_id="screen.width.1440",
            target="screen.width",
            category="value",
            js="return screen.width === 1440;",
            expected=True,
            gap_class="value_mismatch",
        ),
        ProbeDefinition(
            probe_id="screen.width.default",
            target="screen.width",
            category="value",
            js="return screen.width !== 1440;",
            expected=True,
            gap_class="value_mismatch",
        ),
    )
    candidates = candidate_pack(candidate(
        "screen.width.regress.v0",
        "screen.width",
        1440,
    ))

    report = run_environment_toolchain(
        "",
        probe_pack=pack,
        candidate_pack=candidates,
        profile=None,
        adapt_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)

    assert summary(data)["stop_reason"] == "regression_detected"
    assert data["coverage_delta"]["regressed"] == 1
    assert any(
        diagnostic["code"] == "ENV_TOOLCHAIN_COVERAGE_REGRESSED"
        for diagnostic in data["diagnostics"]
    )


def test_iterative_adaptation_rejects_negative_max_iterations():
    with pytest.raises(ValueError, match="max_iterations must be non-negative"):
        run_environment_toolchain("", profile=None, adapt_runtime_safe=True, max_iterations=-1)


def test_iterative_adaptation_emits_no_strong_evidence():
    pack = make_pack(ProbeDefinition(
        probe_id="navigator.languages.chrome_shape",
        target="navigator.languages",
        category="value",
        js="return navigator.languages[0] === 'en-US' && navigator.languages[1] === 'en';",
        expected=True,
        gap_class="value_mismatch",
    ))
    candidates = candidate_pack(candidate(
        "navigator.languages.default.v0",
        "navigator.languages",
        ["en-US", "en"],
    ))

    report = run_environment_toolchain(
        "",
        probe_pack=pack,
        candidate_pack=candidates,
        profile=None,
        adapt_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)

    assert {item["strength"] for item in data["evidence"]} <= {"diagnostic_only", "weak"}
