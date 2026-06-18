from __future__ import annotations

import json

from iv8_rs.environment_toolchain import EnvironmentToolchainReport, toolchain_report_to_dict
from iv8_rs.environment_toolchain_runtime import (
    ProbeDefinition,
    ProbePack,
    run_environment_toolchain,
)


def test_root_export_exposes_environment_toolchain_runner():
    import iv8_rs

    assert iv8_rs.run_environment_toolchain is run_environment_toolchain


def make_pack(*probes: ProbeDefinition) -> ProbePack:
    return ProbePack(
        probe_pack="test.report.only",
        version=1,
        description="report-only synthetic pack",
        evidence_ceiling="diagnostic_only",
        probes=list(probes),
    )


def make_custom_pack_dict() -> dict:
    return {
        "probe_pack": "custom.report.only",
        "version": 1,
        "description": "custom report-only pack",
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


def test_report_only_runner_returns_environment_toolchain_report():
    report = run_environment_toolchain("", probe_pack="fingerprint.m1", profile=None)

    assert isinstance(report, EnvironmentToolchainReport)
    data = toolchain_report_to_dict(report)
    assert data["schema_version"] == "environment-toolchain.v0.1"
    assert data["probe_pack"] == "fingerprint.m1"
    assert data["writes"] == []


def test_report_only_runner_accepts_custom_probe_pack_dict():
    report = run_environment_toolchain("", probe_pack=make_custom_pack_dict(), profile=None)
    data = toolchain_report_to_dict(report)

    assert data["probe_pack"] == "custom.report.only"
    assert data["writes"] == []
    assert data["applied_patches"] == []


def test_report_projects_builtin_asset_provenance():
    report = run_environment_toolchain("", probe_pack="fingerprint.m1", profile=None)
    diagnostics = toolchain_report_to_dict(report)["diagnostics"]

    assert {
        "code": "ENV_TOOLCHAIN_PROBE_PACK_BUILTIN",
        "severity": "info",
        "details": {
            "asset_type": "probe_pack",
            "pack_id": "fingerprint.m1",
            "origin": "builtin",
            "version": 1,
        },
    } in diagnostics
    assert {
        "code": "ENV_TOOLCHAIN_CANDIDATE_PACK_BUILTIN",
        "severity": "info",
        "details": {
            "asset_type": "candidate_pack",
            "pack_id": "chrome_generic",
            "origin": "builtin",
            "version": 1,
        },
    } in diagnostics


def test_report_projects_custom_dict_asset_provenance():
    candidate_pack = {
        "candidate_pack": "custom.disabled.values",
        "version": 1,
        "description": "custom empty candidate pack",
        "candidates": [],
    }
    report = run_environment_toolchain(
        "",
        probe_pack=make_custom_pack_dict(),
        candidate_pack=candidate_pack,
        profile=None,
    )
    diagnostics = toolchain_report_to_dict(report)["diagnostics"]

    assert any(
        diagnostic["code"] == "ENV_TOOLCHAIN_PROBE_PACK_CUSTOM_DICT"
        and diagnostic["details"]["pack_id"] == "custom.report.only"
        for diagnostic in diagnostics
    )
    assert any(
        diagnostic["code"] == "ENV_TOOLCHAIN_CANDIDATE_PACK_CUSTOM_DICT"
        and diagnostic["details"]["pack_id"] == "custom.disabled.values"
        for diagnostic in diagnostics
    )


def test_report_projects_disabled_candidate_pack_provenance():
    report = run_environment_toolchain(
        "",
        probe_pack=make_custom_pack_dict(),
        candidate_pack=None,
        profile=None,
    )
    diagnostics = toolchain_report_to_dict(report)["diagnostics"]

    assert any(
        diagnostic["code"] == "ENV_TOOLCHAIN_CANDIDATE_PACK_DISABLED"
        and diagnostic["details"] == {
            "asset_type": "candidate_pack",
            "pack_id": "disabled",
            "origin": "disabled",
        }
        for diagnostic in diagnostics
    )


def test_report_redacts_custom_path_provenance(tmp_path):
    probe_path = tmp_path / "custom-probe-pack.json"
    candidate_path = tmp_path / "custom-candidate-pack.json"
    probe_path.write_text(json.dumps(make_custom_pack_dict()), encoding="utf-8")
    candidate_path.write_text(
        json.dumps({
            "candidate_pack": "custom.path.values",
            "version": 1,
            "description": "custom empty candidate path pack",
            "candidates": [],
        }),
        encoding="utf-8",
    )

    report = run_environment_toolchain(
        "",
        probe_pack=probe_path,
        candidate_pack=candidate_path,
        profile=None,
    )
    data = toolchain_report_to_dict(report)
    report_text = repr(data)

    assert str(tmp_path) not in report_text
    assert any(
        diagnostic["code"] == "ENV_TOOLCHAIN_PROBE_PACK_CUSTOM_PATH"
        and diagnostic["details"]["redacted_ref"] == "custom-probe-pack.json"
        for diagnostic in data["diagnostics"]
    )
    assert any(
        diagnostic["code"] == "ENV_TOOLCHAIN_CANDIDATE_PACK_CUSTOM_PATH"
        and diagnostic["details"]["redacted_ref"] == "custom-candidate-pack.json"
        for diagnostic in data["diagnostics"]
    )
    assert data["writes"] == []


def test_descriptor_pack_report_is_diagnostic_only_and_no_write():
    report = run_environment_toolchain("", probe_pack="descriptor.m1", profile=None)
    data = toolchain_report_to_dict(report)

    assert data["probe_pack"] == "descriptor.m1"
    assert data["applied_patches"] == []
    assert data["writes"] == []
    assert {item["strength"] for item in data["evidence"]} <= {"diagnostic_only", "weak"}
    assert any(
        diagnostic["code"] == "ENV_TOOLCHAIN_PROBE_PACK_BUILTIN"
        and diagnostic["details"]["pack_id"] == "descriptor.m1"
        for diagnostic in data["diagnostics"]
    )


def test_report_only_runner_does_not_apply_candidates_by_default():
    pack = make_pack(ProbeDefinition(
        probe_id="navigator.languages.force_gap",
        target="navigator.languages",
        category="presence",
        js="return false;",
        expected=True,
        gap_class="missing_api",
    ))
    report = run_environment_toolchain(
        "",
        probe_pack=pack,
        profile=None,
    )
    data = toolchain_report_to_dict(report)

    assert data["applied_patches"] == []
    assert data["rejected_patches"]
    assert all("report-only default" in patch["reason"] for patch in data["rejected_patches"])


def test_report_only_runner_after_equals_before_without_apply():
    report = run_environment_toolchain("", probe_pack="fingerprint.m1", profile=None)
    data = toolchain_report_to_dict(report)

    assert data["after"] == data["before"]
    assert data["coverage_delta"]["improved"] == 0
    assert data["coverage_delta"]["regressed"] == 0


def test_report_only_runner_emits_no_strong_evidence():
    report = run_environment_toolchain("", probe_pack="fingerprint.m1", profile=None)
    data = toolchain_report_to_dict(report)

    strengths = {item["strength"] for item in data["evidence"]}

    assert "strong" not in strengths
    assert strengths <= {"diagnostic_only", "weak"}


def test_runtime_safe_apply_without_candidates_is_noop():
    report = run_environment_toolchain(
        "",
        probe_pack="fingerprint.m1",
        profile=None,
        apply_runtime_safe=True,
    )
    data = toolchain_report_to_dict(report)

    assert data["applied_patches"] == []
    assert data["writes"] == []


def test_pressure_harness_is_disabled_by_default():
    report = run_environment_toolchain("var x = 1;", probe_pack="fingerprint.m1", profile=None)
    data = toolchain_report_to_dict(report)

    assert not any(
        diagnostic["code"] == "ENV_TOOLCHAIN_PRESSURE_HARNESS_SUMMARY"
        for diagnostic in data["diagnostics"]
    )
    assert not any(
        item["kind"] == "environment_pressure_report_built" for item in data["evidence"]
    )


def test_pressure_harness_disabled_preserves_entry_failure_behavior():
    import pytest

    with pytest.raises(Exception, match="HXY_NOT_A_REAL_API is not defined"):
        run_environment_toolchain(
            "new HXY_NOT_A_REAL_API()",
            probe_pack="fingerprint.m1",
            profile=None,
        )


def test_pressure_harness_adds_review_only_diagnostics_without_apply():
    report = run_environment_toolchain(
        "new HXY_NOT_A_REAL_API()",
        probe_pack="fingerprint.m1",
        profile=None,
        pressure_harness=True,
    )
    data = toolchain_report_to_dict(report)
    summaries = [
        diagnostic
        for diagnostic in data["diagnostics"]
        if diagnostic["code"] == "ENV_TOOLCHAIN_PRESSURE_HARNESS_SUMMARY"
    ]

    assert len(summaries) == 1
    summary = summaries[0]["details"]
    assert summary["enabled"] is True
    assert summary["review_status"] == "review_only"
    assert summary["evidence_ceiling"] == "diagnostic_only"
    assert summary["report"]["schema_version"] == "environment-pressure.v0.1"
    assert summary["report"]["input_kind"] == "direct_js"
    assert summary["report"]["status"] == "failed"
    assert summary["report"]["failure_kind"] == "missing_global_symbol"
    assert summary["report"]["pressure"]["pressure_kind"] == "web_api_surface"
    assert summary["report"]["pressure"]["symbol"] == "HXY_NOT_A_REAL_API"
    assert summary["report"]["promotion"]["level"] == "candidate_pack"
    assert summary["report"]["writes"] == []
    assert any(
        item["kind"] == "environment_pressure_report_built"
        and item["strength"] == "diagnostic_only"
        for item in data["evidence"]
    )
    assert data["writes"] == []
    assert data["applied_patches"] == []


def test_pressure_harness_rejects_iterative_adaptation_until_dedicated_review():
    import pytest

    with pytest.raises(ValueError, match="pressure_harness cannot be combined"):
        run_environment_toolchain(
            "var x = 1;",
            probe_pack="fingerprint.m1",
            profile=None,
            adapt_runtime_safe=True,
            pressure_harness=True,
        )
