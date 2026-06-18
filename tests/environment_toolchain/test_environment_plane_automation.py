from __future__ import annotations

from iv8_rs import EnvironmentPlaneReport, run_environment_plane


def test_environment_plane_report_contains_automation_schema():
    report = run_environment_plane("navigator.language; screen.width;", profile=None)

    assert isinstance(report, EnvironmentPlaneReport)
    data = report.to_dict()
    assert data["schema_version"] == "environment-plane.v0.1"
    assert data["workflow"] == ["probe", "patch", "rerun"]
    assert isinstance(data["patch_candidates"], list)
    assert isinstance(data["applied_patches"], list)
    assert isinstance(data["rejected_patches"], list)
    assert set(data["coverage"]) == {
        "probe_coverage_before",
        "probe_coverage_after",
        "coverage_delta",
    }
    assert isinstance(data["evidence"], list)
    assert isinstance(data["diagnostics"], list)


def test_environment_plane_conflicting_environment_value_is_not_overwritten(monkeypatch):
    import iv8_rs.probe

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

    report = run_environment_plane(
        "navigator.language;",
        profile=None,
        environment={"navigator.language": "fr-FR"},
        patch_defaults={"navigator.language": "en-US"},
    )

    assert report.patch.values == {}
    assert report.rejected_patches
    assert report.rejected_patches[0]["diagnostic_code"] == "PATCH_POLICY_CONFLICT"
    assert any(item["code"] == "ENVIRONMENT_PATCH_CONFLICT" for item in report.diagnostics)


def test_environment_plane_applied_patch_is_reported_as_weak_evidence(monkeypatch):
    import iv8_rs.probe

    def fake_probe_environment(*args, environment=None, **kwargs):
        missing = [] if environment and environment.get("navigator.language") else ["navigator.language"]
        return {
            "reads": {},
            "calls": {},
            "writes": {},
            "missing": missing,
            "errors": [],
            "issues": [],
            "coverage": {},
            "vm_info": None,
            "trace_stats": None,
        }

    monkeypatch.setattr(iv8_rs.probe, "probe_environment", fake_probe_environment)

    report = run_environment_plane(
        "navigator.language;",
        profile=None,
        patch_defaults={"navigator.language": "en-US"},
    )

    assert any(item["target"] == "navigator.language" for item in report.applied_patches)
    applied_items = [item for item in report.evidence if item["kind"] == "environment_patch_applied"]
    assert applied_items, "no environment_patch_applied evidence found"
    assert all(item["strength"] == "weak" for item in applied_items)
    assert any(item["kind"] == "environment_gap_observed" for item in report.evidence)
    assert any(item["kind"] == "environment_patch_candidate" for item in report.evidence)


def test_environment_plane_gap_observed_in_evidence(monkeypatch):
    import iv8_rs.probe

    def fake_probe_environment(*args, **kwargs):
        return {
            "reads": {},
            "calls": {},
            "writes": {},
            "missing": ["navigator.plugins"],
            "errors": [],
            "issues": [],
            "coverage": {},
            "vm_info": None,
            "trace_stats": None,
        }

    monkeypatch.setattr(iv8_rs.probe, "probe_environment", fake_probe_environment)
    report = run_environment_plane("navigator.plugins;", profile=None)
    gap_items = [item for item in report.evidence if item["kind"] == "environment_gap_observed"]
    assert gap_items
    assert any("navigator.plugins" in item.get("summary", "") for item in gap_items)
    assert all(item["strength"] == "diagnostic_only" for item in gap_items)


def test_environment_plane_no_improvement_emits_no_change_diagnostic(monkeypatch):
    import iv8_rs.probe

    called = [0]

    def fake_probe_environment(*args, environment=None, **kwargs):
        called[0] += 1
        return {
            "reads": {},
            "calls": {},
            "writes": {},
            "missing": ["navigator.plugins"],
            "errors": [],
            "issues": [],
            "coverage": {},
            "vm_info": None,
            "trace_stats": None,
        }

    monkeypatch.setattr(iv8_rs.probe, "probe_environment", fake_probe_environment)
    report = run_environment_plane("navigator.plugins;", profile=None)
    assert any(item["code"] == "ENVIRONMENT_RERUN_NO_CHANGE" for item in report.diagnostics)
    assert not any(item["code"] == "ENVIRONMENT_RERUN_IMPROVED" for item in report.diagnostics)


def test_environment_plane_regression_detected(monkeypatch):
    import iv8_rs.probe

    call_count = [0]

    def fake_probe_environment(*args, environment=None, **kwargs):
        call_count[0] += 1
        missing = ["navigator.language"] if call_count[0] == 1 else ["navigator.language", "screen.width"]
        return {
            "reads": {},
            "calls": {},
            "writes": {},
            "missing": missing,
            "errors": [],
            "issues": [],
            "coverage": {},
            "vm_info": None,
            "trace_stats": None,
        }

    monkeypatch.setattr(iv8_rs.probe, "probe_environment", fake_probe_environment)
    report = run_environment_plane(
        "navigator.language;",
        profile=None,
        patch_defaults={"navigator.language": "en-US"},
    )
    assert any(item["code"] == "ENVIRONMENT_RERUN_REGRESSED" for item in report.diagnostics)


def test_environment_plane_only_runtime_safe_patches_by_default(monkeypatch):
    import iv8_rs.probe

    def fake_probe_environment(*args, **kwargs):
        return {
            "reads": {}, "calls": {}, "writes": {},
            "missing": ["navigator.webdriver"],
            "errors": [], "issues": [],
            "coverage": {}, "vm_info": None, "trace_stats": None,
        }

    monkeypatch.setattr(iv8_rs.probe, "probe_environment", fake_probe_environment)
    report = run_environment_plane("navigator.webdriver;", profile=None)
    assert report.policy == "runtime_safe"
    assert all(
        p.get("effective_policy") == "runtime_safe" for p in report.applied_patches
    ), "automation must only apply runtime_safe patches by default"


def test_environment_plane_candidate_not_observed_deferred(monkeypatch):
    import iv8_rs.probe

    def fake_probe_environment(*args, **kwargs):
        return {
            "reads": {}, "calls": {}, "writes": {},
            "missing": [],
            "errors": [], "issues": [],
            "coverage": {"total": 0},
            "vm_info": None, "trace_stats": None,
        }

    monkeypatch.setattr(iv8_rs.probe, "probe_environment", fake_probe_environment)
    report = run_environment_plane("var x = 1;", profile=None)
    assert report.patch.values == {}
    candidates = report.patch_candidates
    assert isinstance(candidates, list)


def test_environment_plane_evidence_contains_patch_rejected(monkeypatch):
    import iv8_rs.probe

    def fake_probe_environment(*args, environment=None, **kwargs):
        return {
            "reads": {}, "calls": {}, "writes": {},
            "missing": ["navigator.language"],
            "errors": [], "issues": [],
            "coverage": {}, "vm_info": None, "trace_stats": None,
        }

    monkeypatch.setattr(iv8_rs.probe, "probe_environment", fake_probe_environment)
    report = run_environment_plane(
        "navigator.language;",
        profile=None,
        environment={"navigator.language": "fr-FR"},
    )
    assert any(item["kind"] == "environment_patch_rejected" for item in report.evidence)
