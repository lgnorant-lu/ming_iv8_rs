"""Tests for v0.6 Environment Plane workflow helpers."""

from __future__ import annotations


def test_build_environment_patch_from_missing_targets():
    from iv8_rs import build_environment_patch

    patch = build_environment_patch({"missing": ["navigator.webdriver", "screen.width"]})

    assert patch.policy == "runtime_safe"
    assert patch.values["navigator.webdriver"] is False
    assert patch.values["screen.width"] == 1920
    assert patch.to_dict()["source"] == "probe_missing_defaults"


def test_build_environment_patch_accepts_explicit_defaults():
    from iv8_rs import build_environment_patch

    patch = build_environment_patch(
        {"missing": ["navigator.deviceMemory"]},
        defaults={"navigator.deviceMemory": 16},
    )

    assert patch.values == {"navigator.deviceMemory": 16}


def test_run_environment_plane_returns_workflow_report():
    from iv8_rs import EnvironmentPlaneReport, run_environment_plane

    report = run_environment_plane("var x = 1 + 1;", profile=None)

    assert isinstance(report, EnvironmentPlaneReport)
    assert report.workflow == ["probe", "patch", "rerun"]
    assert report.policy == "runtime_safe"
    assert isinstance(report.before, dict)
    assert isinstance(report.after, dict)
    assert isinstance(report.patch.values, dict)
    assert "coverage" in report.before
    assert "coverage" in report.after
    assert report.schema_version == "environment-plane.v0.1"
    as_dict = report.to_dict()
    assert "patch_candidates" in as_dict
    assert "applied_patches" in as_dict
    assert "rejected_patches" in as_dict
    assert "coverage" in as_dict
    assert "evidence" in as_dict
    assert "diagnostics" in as_dict


def test_run_environment_plane_policy_report_rejects_explicit_conflict(monkeypatch):
    import iv8_rs.probe
    from iv8_rs import run_environment_plane

    def fake_probe_environment(*args, environment=None, **kwargs):
        return {
            "reads": {},
            "calls": {},
            "writes": {},
            "missing": ["navigator.language"],
            "errors": [],
            "issues": [],
            "coverage": {},
            "vm_info": None,
            "trace_stats": None,
        }

    monkeypatch.setattr(iv8_rs.probe, "probe_environment", fake_probe_environment)

    source = "navigator.language;"
    report = run_environment_plane(
        source,
        profile=None,
        environment={"navigator.language": "fr-FR"},
        patch_defaults={"navigator.language": "en-US"},
    )

    assert "navigator.language" not in report.patch.values
    assert any(
        item["target"] == "navigator.language"
        and item["decision"] == "rejected"
        for item in report.rejected_patches
    )


def test_default_value_for_target_covers_known_targets():
    from iv8_rs.environment import _default_value_for_target

    assert _default_value_for_target("navigator.languages") == ["en-US", "en"]
    assert _default_value_for_target("navigator.platform") == "Win32"
    assert _default_value_for_target("screen.height") == 1080
    assert _default_value_for_target("navigator.hardwareConcurrency") == 8
    assert _default_value_for_target("unknown.target") == {}


def test_policy_checked_patch_applies_non_conflicting_defaults():
    from iv8_rs.environment import _build_policy_checked_patch

    patch, candidates, decisions = _build_policy_checked_patch(
        {"missing": ["navigator.webdriver", "screen.width"]},
        policy="runtime_safe",
        defaults={"screen.width": 1440},
        environment=None,
    )

    assert patch.values == {"navigator.webdriver": False, "screen.width": 1440}
    assert len(candidates) == 2
    assert all(decision.decision == "applied" for decision in decisions)
    assert any("generated conservative placeholder" in note for note in patch.notes)


def test_environment_evidence_covers_all_outcomes():
    from iv8_rs.environment import _build_environment_evidence

    evidence = _build_environment_evidence(
        applied_patches=[{"target": "screen.width", "patch_id": "screen.width.default"}],
        improved_targets=["screen.width"],
        before_missing=["screen.width"],
        candidates=[{"target": "screen.width", "patch_id": "screen.width.default"}],
        rejected_patches=[{"target": "navigator.language", "patch_id": "navigator.language.default"}],
        after_missing=["navigator.language", "screen.height"],
    )

    kinds = {item["kind"] for item in evidence}
    assert "environment_gap_observed" in kinds
    assert "environment_patch_candidate" in kinds
    assert "environment_patch_applied" in kinds
    assert "environment_patch_rejected" in kinds
    assert "environment_coverage_improved" in kinds
    assert "environment_coverage_regressed" in kinds


def test_environment_diagnostics_covers_decision_codes_and_guards():
    from iv8_rs.environment import _build_environment_diagnostics
    from iv8_rs.environment_policy import PatchPolicyDecision

    decisions = [
        PatchPolicyDecision(
            patch_id="applied", target="screen.width", requested_policy="runtime_safe",
            effective_policy="runtime_safe", persona="runtime", decision="applied",
            reason="ok",
        ),
        PatchPolicyDecision(
            patch_id="conflict", target="navigator.language", requested_policy="runtime_safe",
            effective_policy="runtime_safe", persona="runtime", decision="rejected",
            reason="candidate conflicts with explicit environment/profile values",
        ),
        PatchPolicyDecision(
            patch_id="unsafe", target="Function.prototype.call", requested_policy="unsafe_hook",
            effective_policy="unsafe_hook", persona="runtime", decision="blocked",
            reason="unsafe hook requires explicit opt-in",
        ),
    ]

    diagnostics = _build_environment_diagnostics(
        decisions,
        improved_targets=["screen.width"],
        before_missing=["screen.width"],
        after_missing=["navigator.language", "screen.height"],
        unsafe_attempted=True,
        profile_write_attempted=True,
    )
    codes = {item["code"] for item in diagnostics}

    assert "ENVIRONMENT_PATCH_APPLIED" in codes
    assert "ENVIRONMENT_PATCH_CONFLICT" in codes
    assert "ENVIRONMENT_PATCH_UNSAFE" in codes
    assert "ENVIRONMENT_RERUN_IMPROVED" in codes
    assert "ENVIRONMENT_PATCH_UNSAFE" in codes
    assert "ENVIRONMENT_PROFILE_WRITE_BLOCKED" in codes


def test_environment_diagnostics_no_change_and_regression_paths():
    from iv8_rs.environment import _build_environment_diagnostics

    no_change = _build_environment_diagnostics(
        decisions=[],
        improved_targets=[],
        before_missing=["navigator.webdriver"],
        after_missing=["navigator.webdriver"],
    )
    assert any(item["code"] == "ENVIRONMENT_RERUN_NO_CHANGE" for item in no_change)

    regressed = _build_environment_diagnostics(
        decisions=[],
        improved_targets=[],
        before_missing=["navigator.webdriver"],
        after_missing=["navigator.webdriver", "screen.width"],
    )
    assert any(item["code"] == "ENVIRONMENT_RERUN_REGRESSED" for item in regressed)


def test_safe_len_handles_none_unsized_and_sized_values():
    from iv8_rs.environment import _safe_len

    assert _safe_len(None) == 0
    assert _safe_len([1, 2, 3]) == 3
    assert _safe_len(object()) == 0
