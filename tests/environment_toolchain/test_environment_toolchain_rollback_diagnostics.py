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
        probe_pack="test.rollback.diagnostics",
        version=1,
        description="rollback diagnostics synthetic pack",
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


def candidate_pack_with_scope(scope: str) -> dict:
    return {
        "candidate_pack": "custom.rollback.values",
        "version": 1,
        "description": "custom rollback metadata candidate pack",
        "candidates": [
            {
                "patch_id": "navigator.languages.rollback.v0",
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
                    "probe_pack": "test.rollback.diagnostics",
                    "expected_delta": ["navigator.languages"],
                    "gap_classes": ["value_mismatch"],
                    "rollback_scope": scope,
                },
            }
        ],
    }


def test_rollback_diagnostics_emit_review_only_summary_and_record():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        profile=None,
        rollback_diagnostics=True,
    )
    data = toolchain_report_to_dict(report)

    summary = diagnostics(data, "ENV_TOOLCHAIN_ROLLBACK_SUMMARY")[0]["details"]
    record = diagnostics(data, "ENV_TOOLCHAIN_ROLLBACK_RECORD")[0]["details"]

    assert summary["enabled"] is True
    assert summary["writes"] == []
    assert summary["evidence_ceiling"] == "diagnostic_only"
    assert summary["record_count"] == 1
    assert summary["blocked_record_count"] == 0
    assert record["scope"] == "context_only"
    assert record["restore_strategy"] == "context_discard"
    assert record["writes"] == []
    assert record["review_status"] == "review_only"
    assert record["evidence_ceiling"] == "diagnostic_only"
    assert record["capture_before"] == ["navigator.languages"]
    assert data["applied_patches"] == []
    assert data["writes"] == []


def test_rollback_diagnostics_block_persistent_scope_metadata():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        candidate_pack=candidate_pack_with_scope("profile_file"),
        profile=None,
        rollback_diagnostics=True,
    )
    data = toolchain_report_to_dict(report)

    summary = diagnostics(data, "ENV_TOOLCHAIN_ROLLBACK_SUMMARY")[0]["details"]
    record = diagnostics(data, "ENV_TOOLCHAIN_ROLLBACK_RECORD")[0]["details"]

    assert summary["review_status"] == "blocked"
    assert summary["blocked_record_count"] == 1
    assert record["scope"] == "profile_file"
    assert record["review_status"] == "blocked"
    assert record["blocked_reasons"] == ["persistent_scope_blocked"]
    assert record["writes"] == []
    assert data["applied_patches"] == []


def test_rollback_diagnostics_disabled_candidate_pack_has_summary_only():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        candidate_pack=None,
        profile=None,
        rollback_diagnostics=True,
    )
    data = toolchain_report_to_dict(report)

    summary = diagnostics(data, "ENV_TOOLCHAIN_ROLLBACK_SUMMARY")[0]["details"]

    assert summary["record_count"] == 0
    assert summary["input_signal_counts"]["candidate_pack_enabled"] is False
    assert diagnostics(data, "ENV_TOOLCHAIN_ROLLBACK_RECORD") == []
    assert data["applied_patches"] == []
    assert data["writes"] == []


def test_rollback_diagnostics_can_coexist_with_dry_run_planning():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        profile=None,
        dry_run_planning=True,
        rollback_diagnostics=True,
    )
    data = toolchain_report_to_dict(report)

    assert diagnostics(data, "ENV_TOOLCHAIN_DRY_RUN_PLAN_SUMMARY")
    assert diagnostics(data, "ENV_TOOLCHAIN_ROLLBACK_SUMMARY")
    assert data["applied_patches"] == []
    assert data["writes"] == []


def test_rollback_diagnostics_do_not_promote_evidence_strength():
    report = run_environment_toolchain(
        "",
        probe_pack=languages_pack(),
        profile=None,
        rollback_diagnostics=True,
    )
    data = toolchain_report_to_dict(report)

    assert {item["strength"] for item in data["evidence"]} <= {"diagnostic_only", "weak"}
    assert not any(item["kind"] == "environment_patch_applied" for item in data["evidence"])


def test_rollback_diagnostics_cannot_be_combined_with_runtime_safe_apply():
    with pytest.raises(ValueError, match="rollback_diagnostics cannot be combined"):
        run_environment_toolchain(
            "",
            probe_pack=languages_pack(),
            profile=None,
            rollback_diagnostics=True,
            apply_runtime_safe=True,
        )


def test_rollback_diagnostics_cannot_be_combined_with_iterative_apply():
    with pytest.raises(ValueError, match="rollback_diagnostics cannot be combined"):
        run_environment_toolchain(
            "",
            probe_pack=languages_pack(),
            profile=None,
            rollback_diagnostics=True,
            adapt_runtime_safe=True,
        )
