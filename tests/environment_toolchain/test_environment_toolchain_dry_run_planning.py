from __future__ import annotations

import pytest
from iv8_rs.environment_toolchain import toolchain_report_to_dict
from iv8_rs.environment_toolchain_runtime import (
    ProbeDefinition,
    ProbePack,
    run_environment_toolchain,
)


def diagnostics(data: dict, code: str) -> list[dict]:
    return [diagnostic for diagnostic in data["diagnostics"] if diagnostic["code"] == code]


def languages_pack() -> ProbePack:
    return ProbePack(
        probe_pack="test.dry.run.planning",
        version=1,
        description="dry-run planning synthetic pack",
        evidence_ceiling="diagnostic_only",
        probes=[
            ProbeDefinition(
                probe_id="navigator.languages.force_gap",
                target="navigator.languages",
                category="value",
                js="return false;",
                expected=True,
                gap_class="value_mismatch",
            )
        ],
    )


def test_dry_run_planning_emits_review_only_summary_and_item():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        profile=None,
        dry_run_planning=True,
    )
    data = toolchain_report_to_dict(report)

    summary = diagnostics(data, "ENV_TOOLCHAIN_DRY_RUN_PLAN_SUMMARY")[0]["details"]
    items = diagnostics(data, "ENV_TOOLCHAIN_DRY_RUN_PLAN_ITEM")

    assert summary["enabled"] is True
    assert summary["apply_authorized"] is False
    assert summary["writes"] == []
    assert summary["evidence_ceiling"] == "diagnostic_only"
    assert summary["candidate_count"] == 1
    assert summary["eligible_for_review_count"] == 1
    assert summary["blocked_candidate_count"] == 0
    assert items[0]["details"]["planning_status"] == "eligible_for_review"
    assert items[0]["details"]["apply_authorized"] is False
    assert items[0]["details"]["blocked_reasons"] == []
    assert data["applied_patches"] == []
    assert data["writes"] == []


def test_dry_run_planning_blocks_explicit_environment_conflict():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        profile=None,
        environment={"navigator.languages": ["fr-FR", "fr"]},
        dry_run_planning=True,
    )
    data = toolchain_report_to_dict(report)

    summary = diagnostics(data, "ENV_TOOLCHAIN_DRY_RUN_PLAN_SUMMARY")[0]["details"]
    item = diagnostics(data, "ENV_TOOLCHAIN_DRY_RUN_PLAN_ITEM")[0]["details"]

    assert summary["review_status"] == "blocked"
    assert summary["eligible_for_review_count"] == 0
    assert summary["blocked_candidate_count"] == 1
    assert item["planning_status"] == "blocked_by_conflict"
    assert item["blocked_reasons"] == ["explicit_environment_precedence"]
    assert item["apply_authorized"] is False
    assert data["applied_patches"] == []
    assert data["rejected_patches"] == []


def test_dry_run_planning_disabled_candidate_pack_has_summary_only():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        candidate_pack=None,
        profile=None,
        dry_run_planning=True,
    )
    data = toolchain_report_to_dict(report)

    summary = diagnostics(data, "ENV_TOOLCHAIN_DRY_RUN_PLAN_SUMMARY")[0]["details"]

    assert summary["candidate_count"] == 0
    assert summary["input_signal_counts"]["candidate_pack_enabled"] is False
    assert diagnostics(data, "ENV_TOOLCHAIN_DRY_RUN_PLAN_ITEM") == []
    assert data["applied_patches"] == []
    assert data["writes"] == []


def test_dry_run_planning_does_not_promote_evidence_strength():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        profile=None,
        dry_run_planning=True,
    )
    data = toolchain_report_to_dict(report)

    assert {item["strength"] for item in data["evidence"]} <= {"diagnostic_only", "weak"}
    assert not any(
        item["kind"] in {"environment_patch_applied", "environment_coverage_improved"}
        for item in data["evidence"]
    )


def test_dry_run_planning_cannot_be_combined_with_runtime_safe_apply():
    with pytest.raises(ValueError, match="dry_run_planning cannot be combined"):
        run_environment_toolchain(
            "",
            probe_pack=languages_pack(),
            profile=None,
            dry_run_planning=True,
            apply_runtime_safe=True,
        )


def test_dry_run_planning_cannot_be_combined_with_iterative_apply():
    with pytest.raises(ValueError, match="dry_run_planning cannot be combined"):
        run_environment_toolchain(
            "",
            probe_pack=languages_pack(),
            profile=None,
            dry_run_planning=True,
            adapt_runtime_safe=True,
        )
