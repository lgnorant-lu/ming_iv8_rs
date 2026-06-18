from __future__ import annotations

import pytest
from iv8_rs.environment_toolchain import toolchain_report_to_dict
from iv8_rs.environment_toolchain_runtime import run_environment_toolchain


def diagnostics(data: dict, code: str) -> list[dict]:
    return [diagnostic for diagnostic in data["diagnostics"] if diagnostic["code"] == code]


def test_scaffold_gaps_emit_review_only_summary_and_items():
    report = run_environment_toolchain("", profile=None, scaffold_gaps=True)
    data = toolchain_report_to_dict(report)

    summary = diagnostics(data, "ENV_TOOLCHAIN_SCAFFOLD_GAP_SUMMARY")[0]["details"]
    items = diagnostics(data, "ENV_TOOLCHAIN_SCAFFOLD_GAP_ITEM")

    assert summary["enabled"] is True
    assert summary["apply_authorized"] is False
    assert summary["writes"] == []
    assert summary["evidence_ceiling"] == "diagnostic_only"
    assert summary["gap_count"] == len(items)
    assert summary["high_priority_count"] > 0
    assert data["applied_patches"] == []
    assert data["writes"] == []
    assert all(item["details"]["apply_authorized"] is False for item in items)


def test_scaffold_gaps_cover_all_gap_classes():
    report = run_environment_toolchain("", profile=None, scaffold_gaps=True)
    data = toolchain_report_to_dict(report)
    summary = diagnostics(data, "ENV_TOOLCHAIN_SCAFFOLD_GAP_SUMMARY")[0]["details"]

    assert set(summary["gap_class_counts"]) == {
        "substrate_gap",
        "probe_gap",
        "candidate_gap",
        "policy_gap",
        "evidence_gap",
        "rollback_gap",
        "negative_gate_gap",
    }


def test_scaffold_gaps_do_not_generate_candidate_or_apply_entries():
    report = run_environment_toolchain(
        "",
        profile=None,
        candidate_pack=None,
        scaffold_gaps=True,
    )
    data = toolchain_report_to_dict(report)

    assert diagnostics(data, "ENV_TOOLCHAIN_SCAFFOLD_GAP_SUMMARY")
    assert data["applied_patches"] == []
    assert data["rejected_patches"] == []
    assert data["profile_suggestions"] == []


def test_scaffold_gaps_can_coexist_with_other_review_diagnostics():
    report = run_environment_toolchain(
        "",
        profile=None,
        scaffold_gaps=True,
        substrate_coverage=True,
        dry_run_planning=True,
        rollback_diagnostics=True,
    )
    data = toolchain_report_to_dict(report)

    assert diagnostics(data, "ENV_TOOLCHAIN_SCAFFOLD_GAP_SUMMARY")
    assert diagnostics(data, "ENV_TOOLCHAIN_SUBSTRATE_COVERAGE_SUMMARY")
    assert diagnostics(data, "ENV_TOOLCHAIN_DRY_RUN_PLAN_SUMMARY")
    assert diagnostics(data, "ENV_TOOLCHAIN_ROLLBACK_SUMMARY")
    assert data["applied_patches"] == []
    assert data["writes"] == []


def test_scaffold_gaps_cannot_be_combined_with_runtime_safe_apply():
    with pytest.raises(ValueError, match="scaffold_gaps cannot be combined"):
        run_environment_toolchain(
            "",
            profile=None,
            scaffold_gaps=True,
            apply_runtime_safe=True,
        )


def test_scaffold_gaps_cannot_be_combined_with_iterative_apply():
    with pytest.raises(ValueError, match="scaffold_gaps cannot be combined"):
        run_environment_toolchain(
            "",
            profile=None,
            scaffold_gaps=True,
            adapt_runtime_safe=True,
        )


def test_scaffold_gaps_do_not_promote_evidence_strength():
    report = run_environment_toolchain("", profile=None, scaffold_gaps=True)
    data = toolchain_report_to_dict(report)

    assert {item["strength"] for item in data["evidence"]} <= {"diagnostic_only", "weak"}
    assert not any(item["kind"] == "environment_patch_applied" for item in data["evidence"])
