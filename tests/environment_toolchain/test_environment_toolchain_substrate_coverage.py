from __future__ import annotations

import pytest
from iv8_rs.environment_toolchain import toolchain_report_to_dict
from iv8_rs.environment_toolchain_runtime import run_environment_toolchain


def diagnostics(data: dict, code: str) -> list[dict]:
    return [diagnostic for diagnostic in data["diagnostics"] if diagnostic["code"] == code]


def test_substrate_coverage_emits_review_only_summary_and_items():
    report = run_environment_toolchain("", profile=None, substrate_coverage=True)
    data = toolchain_report_to_dict(report)

    summary = diagnostics(data, "ENV_TOOLCHAIN_SUBSTRATE_COVERAGE_SUMMARY")[0]["details"]
    items = diagnostics(data, "ENV_TOOLCHAIN_SUBSTRATE_COVERAGE_ITEM")

    assert summary["enabled"] is True
    assert summary["apply_authorized"] is False
    assert summary["writes"] == []
    assert summary["evidence_ceiling"] == "diagnostic_only"
    assert summary["item_count"] == len(items)
    assert summary["item_count"] > 0
    assert data["applied_patches"] == []
    assert data["writes"] == []
    assert all(item["details"]["evidence_ceiling"] == "diagnostic_only" for item in items)


def test_substrate_coverage_flags_review_gated_surfaces():
    report = run_environment_toolchain("", profile=None, substrate_coverage=True)
    data = toolchain_report_to_dict(report)
    items = {
        item["details"]["surface_id"]: item["details"]
        for item in diagnostics(data, "ENV_TOOLCHAIN_SUBSTRATE_COVERAGE_ITEM")
    }

    assert items["timezone_intl"]["review_status"] == "requires_review"
    assert items["navigator_connection"]["review_status"] == "requires_review"
    assert "native_hardening_without_review" in items["timezone_intl"]["blocked_actions"]
    assert "runtime_apply" in items["navigator_connection"]["blocked_actions"]


def test_substrate_coverage_can_coexist_with_planning_and_rollback_diagnostics():
    report = run_environment_toolchain(
        "",
        profile=None,
        substrate_coverage=True,
        dry_run_planning=True,
        rollback_diagnostics=True,
    )
    data = toolchain_report_to_dict(report)

    assert diagnostics(data, "ENV_TOOLCHAIN_SUBSTRATE_COVERAGE_SUMMARY")
    assert diagnostics(data, "ENV_TOOLCHAIN_DRY_RUN_PLAN_SUMMARY")
    assert diagnostics(data, "ENV_TOOLCHAIN_ROLLBACK_SUMMARY")
    assert data["applied_patches"] == []
    assert data["writes"] == []


def test_substrate_coverage_cannot_be_combined_with_runtime_safe_apply():
    with pytest.raises(ValueError, match="substrate_coverage cannot be combined"):
        run_environment_toolchain(
            "",
            profile=None,
            substrate_coverage=True,
            apply_runtime_safe=True,
        )


def test_substrate_coverage_cannot_be_combined_with_iterative_apply():
    with pytest.raises(ValueError, match="substrate_coverage cannot be combined"):
        run_environment_toolchain(
            "",
            profile=None,
            substrate_coverage=True,
            adapt_runtime_safe=True,
        )


def test_substrate_coverage_does_not_promote_evidence_strength():
    report = run_environment_toolchain("", profile=None, substrate_coverage=True)
    data = toolchain_report_to_dict(report)

    assert {item["strength"] for item in data["evidence"]} <= {"diagnostic_only", "weak"}
    assert not any(item["kind"] == "environment_patch_applied" for item in data["evidence"])
