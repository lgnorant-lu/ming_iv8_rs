"""Targeted runtime tests for Environment Toolchain types.

Verifies that EnvironmentToolchainReport preserves all fixture fields
through typed round-trip and that policy/mutation guards survive
conversion.
"""

from __future__ import annotations

import pytest
from experimental_contract_helpers import load_fixture
from iv8_rs.environment_toolchain import (
    ToolchainPatchEntry,
    toolchain_report_from_dict,
    toolchain_report_to_dict,
)


def test_environment_toolchain_typed_roundtrip():
    data = load_fixture("environment-toolchain")
    report = toolchain_report_from_dict(data)
    roundtrip = toolchain_report_to_dict(report)

    assert roundtrip == data


def test_environment_toolchain_policy_guards_survive_typing():
    data = load_fixture("environment-toolchain")
    report = toolchain_report_from_dict(data)

    rejected_policies = {p.policy for p in report.rejected_patches}
    assert "analysis_only" in rejected_policies
    assert "unsafe_hook" in rejected_policies


def test_environment_toolchain_patch_entry_roundtrip():
    entry = ToolchainPatchEntry(
        patch_id="test.patch.v0",
        target="navigator.webdriver",
        kind="value",
        policy="runtime_safe",
    )
    d = entry.to_dict()
    restored = ToolchainPatchEntry.from_dict(d)
    assert restored.patch_id == entry.patch_id
    assert restored.target == entry.target
    assert restored.reason is None


def test_environment_toolchain_report_evidence_and_diagnostics():
    data = load_fixture("environment-toolchain")
    report = toolchain_report_from_dict(data)

    assert len(report.evidence) == len(data["evidence"])
    assert len(report.diagnostics) == len(data["diagnostics"])
    assert report.writes is not None and report.writes == []


def test_environment_toolchain_writes_absent_roundtrip():
    data = load_fixture("environment-toolchain").copy()
    data.pop("writes", None)
    report = toolchain_report_from_dict(data)
    roundtrip = toolchain_report_to_dict(report)

    assert "writes" not in roundtrip
    assert report.writes is None


def test_runtime_string_list_preview_scalar_and_list():
    from iv8_rs.environment_toolchain_runtime import _string_list_preview

    assert _string_list_preview("abc") == ["abc"]
    assert _string_list_preview(["a", 1, True]) == ["a", "1", "True"]


def test_runtime_coverage_snapshot_helpers():
    from iv8_rs.environment_toolchain_runtime import (
        _coverage_snapshot,
        _coverage_snapshot_dict,
    )

    coverage = {"present": 2, "missing": 1, "mismatch": 3}
    snapshot = _coverage_snapshot(coverage)

    assert snapshot.present == 2
    assert snapshot.missing == 1
    assert snapshot.mismatch == 3
    assert _coverage_snapshot_dict(coverage) == coverage


def test_runtime_iteration_stop_reason_paths():
    from types import SimpleNamespace

    from iv8_rs.environment_toolchain_runtime import _iteration_stop_reason

    before = SimpleNamespace(gaps=["a", "b"])
    after_clear = SimpleNamespace(gaps=[])
    after_same = SimpleNamespace(gaps=["a", "b"])
    after_less = SimpleNamespace(gaps=["a"])

    assert _iteration_stop_reason(
        {"improved": 0, "regressed": 1}, before_run=before,
        after_run=after_same, stop_on_regression=True,
    ) == "regression_detected"
    assert _iteration_stop_reason(
        {"improved": 1, "regressed": 0}, before_run=before,
        after_run=after_clear, stop_on_regression=True,
    ) == "completed"
    assert _iteration_stop_reason(
        {"improved": 0, "regressed": 0}, before_run=before,
        after_run=after_same, stop_on_regression=False,
    ) == "no_progress"
    assert _iteration_stop_reason(
        {"improved": 1, "regressed": 0}, before_run=before,
        after_run=after_less, stop_on_regression=False,
    ) == "budget_exhausted"


def test_runtime_coherence_primitive_helpers():
    from iv8_rs.environment_toolchain_runtime import (
        _is_non_empty_string,
        _is_non_negative_number,
        _is_positive_number,
        _user_agent_has_mobile_token,
    )

    assert _is_positive_number(1)
    assert not _is_positive_number(0)
    assert not _is_positive_number(True)
    assert _is_non_negative_number(0)
    assert not _is_non_negative_number(-1)
    assert _is_non_empty_string("x")
    assert not _is_non_empty_string("")
    assert _user_agent_has_mobile_token("Mozilla/5.0 Mobile Safari")
    assert not _user_agent_has_mobile_token("Desktop Safari")


def test_runtime_platform_family_classifiers():
    from iv8_rs.environment_toolchain_runtime import (
        _platform_family_from_platform,
        _platform_family_from_ua_data_platform,
        _platform_family_from_user_agent,
    )

    assert _platform_family_from_user_agent("Windows NT 10.0") == "windows"
    assert _platform_family_from_user_agent("Macintosh; Intel Mac OS X") == "macos"
    assert _platform_family_from_user_agent("Android 14") == "android"
    assert _platform_family_from_user_agent("iPhone OS") == "ios"
    assert _platform_family_from_user_agent("X11 Linux") == "linux"
    assert _platform_family_from_user_agent("Unknown") is None

    assert _platform_family_from_platform("Win32") == "windows"
    assert _platform_family_from_platform("Darwin") == "macos"
    assert _platform_family_from_platform("Linux x86_64") == "linux"
    assert _platform_family_from_platform("Android") == "android"
    assert _platform_family_from_platform("iPad") == "ios"
    assert _platform_family_from_platform("Unknown") is None

    assert _platform_family_from_ua_data_platform("Windows") == "windows"
    assert _platform_family_from_ua_data_platform("macOS") == "macos"
    assert _platform_family_from_ua_data_platform("Linux") == "linux"
    assert _platform_family_from_ua_data_platform("Android") == "android"
    assert _platform_family_from_ua_data_platform("iOS") == "ios"
    assert _platform_family_from_ua_data_platform("Unknown") is None


def test_runtime_target_family_classifier():
    from iv8_rs.environment_toolchain_runtime import _classify_target_family

    assert _classify_target_family("performance.now") == "timing"
    assert _classify_target_family("navigator.connection.rtt") == "network_info"
    assert _classify_target_family("screen.width") == "screen"
    assert _classify_target_family("localStorage") == "window"
    assert _classify_target_family("unknown.target") is None


def test_runtime_coverage_delta_and_observation_coverage():
    from types import SimpleNamespace

    from iv8_rs.environment_toolchain_runtime import (
        _coverage_delta,
        _coverage_from_observations,
    )

    before = SimpleNamespace(observations=[
        SimpleNamespace(probe_id="a", passed=False),
        SimpleNamespace(probe_id="b", passed=True),
    ], gaps=["a"])
    after = SimpleNamespace(observations=[
        SimpleNamespace(probe_id="a", passed=True),
        SimpleNamespace(probe_id="b", passed=False),
    ], gaps=["b"])
    assert _coverage_delta(before, after) == {"improved": 1, "regressed": 1, "unresolved": 1}

    observations = [
        SimpleNamespace(passed=True, gap_class=""),
        SimpleNamespace(passed=False, gap_class="missing_api"),
        SimpleNamespace(passed=False, gap_class="descriptor_mismatch"),
    ]
    assert _coverage_from_observations(observations) == {"present": 1, "missing": 1, "mismatch": 1}


def test_runtime_gap_diagnostics_codes():
    from iv8_rs.environment_toolchain_models import EnvironmentGap
    from iv8_rs.environment_toolchain_runtime import _gap_diagnostics

    gaps = [
        EnvironmentGap(probe_id="p1", target="screen.width", gap_class="missing_api",
                       category="surface", expected=1, actual=None),
        EnvironmentGap(probe_id="p2", target="navigator.webdriver",
                       gap_class="descriptor_mismatch", category="descriptor",
                       expected=False, actual=True, error="bad descriptor"),
    ]
    diagnostics = _gap_diagnostics(gaps)
    codes = {item["code"] for item in diagnostics}

    assert "ENV_TOOLCHAIN_GAP_OBSERVED" in codes
    assert "ENV_TOOLCHAIN_DESCRIPTOR_MISMATCH" in codes


def test_runtime_probe_eval_source_wraps_body():
    from iv8_rs.environment_toolchain_runtime import _probe_eval_source

    source = _probe_eval_source("return 42;")
    assert source.startswith("(function(){")
    assert "return 42;" in source
    assert source.endswith("})()")


def test_runtime_language_coherence_group_paths():
    from iv8_rs.environment_toolchain_runtime import _language_coherence_group

    sources = {"navigator.language": "test", "navigator.languages": "test"}
    assert _language_coherence_group({}, {}).status == "unknown"
    assert _language_coherence_group(
        {"navigator.language": "en-US", "navigator.languages": ["en-US", "en"]},
        sources,
    ).status == "consistent"
    mismatch = _language_coherence_group(
        {"navigator.language": "fr-FR", "navigator.languages": ["en-US", "en"]},
        sources,
    )
    assert mismatch.status == "inconsistent"
    assert "does not match" in mismatch.reason


def test_runtime_screen_window_coherence_group_paths():
    from iv8_rs.environment_toolchain_runtime import _screen_window_coherence_group


    def values(**overrides):
        data = {
            "screen.width": 1920,
            "screen.height": 1080,
            "screen.availWidth": 1920,
            "screen.availHeight": 1040,
            "window.innerWidth": 1280,
            "window.innerHeight": 720,
            "window.devicePixelRatio": 1,
        }
        data.update(overrides)
        return data

    assert _screen_window_coherence_group({}, {}).status == "unknown"
    assert _screen_window_coherence_group(values(), {}).status == "consistent"
    assert _screen_window_coherence_group(
        values(**{"screen.availWidth": 3000}), {}
    ).status == "inconsistent"
    assert _screen_window_coherence_group(
        values(**{"window.innerWidth": 3000}), {}
    ).status == "inconsistent"


def test_runtime_ua_platform_coherence_group_paths():
    from iv8_rs.environment_toolchain_runtime import _ua_platform_coherence_group

    assert _ua_platform_coherence_group({}, {}).status == "unknown"
    assert _ua_platform_coherence_group(
        {"navigator.userAgent": "Windows NT 10.0", "navigator.platform": "Win32"}, {}
    ).status == "consistent"
    assert _ua_platform_coherence_group(
        {"navigator.userAgent": "Windows NT 10.0", "navigator.platform": "MacIntel"}, {}
    ).status == "inconsistent"
    assert _ua_platform_coherence_group(
        {
            "navigator.userAgent": "Windows NT 10.0",
            "navigator.platform": "Win32",
            "navigator.userAgentData.platform": 123,
        },
        {},
    ).status == "unknown"
    assert _ua_platform_coherence_group(
        {
            "navigator.userAgent": "Windows NT 10.0",
            "navigator.platform": "Win32",
            "navigator.userAgentData.platform": "macOS",
        },
        {},
    ).status == "inconsistent"
    assert _ua_platform_coherence_group(
        {
            "navigator.userAgent": "Windows NT 10.0",
            "navigator.platform": "Win32",
            "navigator.userAgentData.mobile": True,
        },
        {},
    ).status == "inconsistent"
    assert _ua_platform_coherence_group(
        {
            "navigator.userAgent": "Mozilla/5.0 Mobile Safari",
            "navigator.platform": "iPhone",
            "navigator.userAgentData.mobile": False,
        },
        {},
    ).status == "inconsistent"


def test_runtime_network_info_coherence_group_paths():
    from iv8_rs.environment_toolchain_runtime import _network_info_coherence_group

    assert _network_info_coherence_group({}, {}).status == "unknown"
    assert _network_info_coherence_group(
        {"navigator.connection.effectiveType": "4g", "navigator.connection.downlink": 10,
         "navigator.connection.rtt": 50, "navigator.connection.saveData": False},
        {},
    ).status == "consistent"
    assert _network_info_coherence_group(
        {"navigator.connection.effectiveType": ""}, {}
    ).status == "unknown"
    assert _network_info_coherence_group(
        {"navigator.connection.downlink": -1}, {}
    ).status == "inconsistent"
    assert _network_info_coherence_group(
        {"navigator.connection.rtt": -1}, {}
    ).status == "inconsistent"
    assert _network_info_coherence_group(
        {"navigator.connection.saveData": "false"}, {}
    ).status == "unknown"


def test_runtime_timezone_locale_coherence_group_paths():
    from iv8_rs.environment_toolchain_runtime import _timezone_locale_coherence_group

    assert _timezone_locale_coherence_group({}, {}).status == "unknown"
    assert _timezone_locale_coherence_group(
        {"config.timezone": "", "navigator.language": "en-US", "navigator.languages": ["en-US"]},
        {},
    ).status == "unknown"
    assert _timezone_locale_coherence_group(
        {"timezone": "", "navigator.language": "en-US", "navigator.languages": ["en-US"]},
        {},
    ).status == "unknown"
    assert _timezone_locale_coherence_group(
        {"timezone": "UTC", "navigator.language": "fr-FR", "navigator.languages": ["en-US"]},
        {},
    ).status == "inconsistent"
    assert _timezone_locale_coherence_group(
        {"config.timezone": "UTC", "timezone": "Europe/Paris",
         "navigator.language": "en-US", "navigator.languages": ["en-US"]},
        {},
    ).status == "inconsistent"
    assert _timezone_locale_coherence_group(
        {"config.timezone": "UTC", "timezone": "UTC",
         "navigator.language": "en-US", "navigator.languages": ["en-US", "en"]},
        {},
    ).status == "consistent"


def test_runtime_profile_coherence_groups_returns_all_groups(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime

    monkeypatch.setattr(runtime, "_coherence_value_source", lambda env: ({}, {}))

    groups = runtime._profile_coherence_groups({})

    assert [group.group_id for group in groups] == [
        "language",
        "screen_window",
        "ua_platform",
        "network_info",
        "timezone_locale",
    ]
    assert all(group.status == "unknown" for group in groups)


def test_runtime_resolve_local_overlay_dict_paths(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime

    values, provenance, rejected = runtime._resolve_local_overlay({"screen.width": 1440})
    assert values == {"screen.width": 1440}
    assert provenance.code == "ENV_TOOLCHAIN_LOCAL_OVERLAY_PROVENANCE"
    assert rejected is None

    values, provenance, rejected = runtime._resolve_local_overlay({"dogepy.custom": 1})
    assert values is None
    assert provenance is None
    assert rejected.code == "ENV_TOOLCHAIN_LOCAL_OVERLAY_REJECTED"

    class BlockedDecision:
        decision = "blocked"
        blocked_terms = ["target_flow"]

    monkeypatch.setattr(runtime, "validate_bypass_boundary", lambda _: BlockedDecision())
    values, provenance, rejected = runtime._resolve_local_overlay({"screen.width": 1440})
    assert values is None
    assert provenance is None
    assert rejected.details["reason"] == "local overlay blocked by boundary validation"


def test_runtime_resolve_local_overlay_path_paths(tmp_path):
    import json

    from iv8_rs.environment_toolchain_runtime import _resolve_local_overlay

    valid_path = tmp_path / "overlay.json"
    valid_path.write_text(json.dumps({"navigator.language": "en-US"}), encoding="utf-8")
    values, provenance, rejected = _resolve_local_overlay(valid_path)
    assert values == {"navigator.language": "en-US"}
    assert provenance.details["origin"] == "custom_path"
    assert provenance.details["redacted_ref"] == "overlay.json"
    assert rejected is None

    invalid_json = tmp_path / "bad.json"
    invalid_json.write_text("{bad", encoding="utf-8")
    values, provenance, rejected = _resolve_local_overlay(invalid_json)
    assert values is None
    assert provenance is None
    assert rejected.code == "ENV_TOOLCHAIN_LOCAL_OVERLAY_REJECTED"

    non_object = tmp_path / "list.json"
    non_object.write_text(json.dumps([1, 2]), encoding="utf-8")
    values, provenance, rejected = _resolve_local_overlay(non_object)
    assert rejected.details["reason"] == "local overlay JSON must contain an object"

    with pytest.raises(ValueError, match="local_overlay must be a dict"):
        _resolve_local_overlay("not-a-pathlike")


def test_runtime_map_gaps_to_family_pressures_and_pressure_report():
    from iv8_rs.environment_toolchain_models import EnvironmentGap
    from iv8_rs.environment_toolchain_runtime import (
        _build_toolchain_pressure_report,
        _classify_pressure_category,
        _map_gaps_to_family_pressures,
    )

    gaps = [
        EnvironmentGap(probe_id="p1", target="screen.width", gap_class="missing_api",
                       category="surface", expected=1, actual=None),
        EnvironmentGap(probe_id="p2", target="screen.height", gap_class="missing_api",
                       category="surface", expected=1, actual=None),
        EnvironmentGap(probe_id="p3", target="unknown.target", gap_class="unknown_gap",
                       category="surface", expected=1, actual=None),
    ]

    assert _classify_pressure_category(gaps[0]) == "missing_api"
    assert _classify_pressure_category(gaps[2]) is None
    pressures = _map_gaps_to_family_pressures(gaps)
    assert len(pressures) == 1
    assert pressures[0].pressure_id == "missing_api__screen"
    assert pressures[0].target_family == "screen"
    assert pressures[0].gap_classes == ["missing_api"]

    report = _build_toolchain_pressure_report("var x = 1;", message="ReferenceError: fetch is not defined")
    assert report.sample_id == "toolchain.inline"
    assert report.pressure.pressure_kind == "network_surface"


def test_runtime_run_single_probe_success_and_error():
    from iv8_rs.environment_toolchain_asset_models import ProbeDefinition
    from iv8_rs.environment_toolchain_runtime import _run_single_probe

    probe = ProbeDefinition(
        probe_id="p1", target="navigator.webdriver", category="presence",
        js="return false;", expected=False, gap_class="missing_api",
    )

    class PassingCtx:
        def eval(self, source):
            assert "return false;" in source
            return False

    observation = _run_single_probe(PassingCtx(), probe)
    assert observation.passed is True
    assert observation.actual is False

    class FailingCtx:
        def eval(self, source):
            raise RuntimeError("boom")

    observation = _run_single_probe(FailingCtx(), probe)
    assert observation.passed is False
    assert observation.error == "boom"


def test_runtime_provenance_diagnostic_and_record():
    from iv8_rs.environment_toolchain_models import AssetProvenance
    from iv8_rs.environment_toolchain_runtime import _provenance_diagnostic, _provenance_record

    provenance = AssetProvenance(
        asset_type="probe pack", pack_id="pack.m1", origin="builtin",
        version=3, redacted_ref="probe.json",
    )
    diagnostic = _provenance_diagnostic(provenance)

    assert diagnostic["code"] == "ENV_TOOLCHAIN_PROBE_PACK_BUILTIN"
    assert diagnostic["stage"] == "environment.asset"
    assert diagnostic["details"]["version"] == 3
    assert diagnostic["details"]["redacted_ref"] == "probe.json"

    record = _provenance_record(provenance)
    assert record.code == diagnostic["code"]
    assert record.severity == "info"


# ── _coherence_value_source ──────────────────────────────────────────────────


def test_runtime_coherence_value_source_defaults_and_environment(monkeypatch):
    import iv8_rs
    import iv8_rs.environment_toolchain_runtime as runtime

    class MockJSContext:
        @staticmethod
        def get_defaults():
            return {"navigator.language": "en-US", "screen.width": 1920}

    monkeypatch.setattr(iv8_rs, "JSContext", MockJSContext)

    values, sources = runtime._coherence_value_source({"screen.width": 1440})
    assert values["navigator.language"] == "en-US"
    assert values["screen.width"] == 1440
    assert sources["navigator.language"] == "profile_default"
    assert sources["screen.width"] == "environment"


def test_runtime_coherence_value_source_get_defaults_error(monkeypatch):
    import iv8_rs
    import iv8_rs.environment_toolchain_runtime as runtime

    class BrokenJSContext:
        @staticmethod
        def get_defaults():
            raise RuntimeError("v8 unavailable")

    monkeypatch.setattr(iv8_rs, "JSContext", BrokenJSContext)

    values, sources = runtime._coherence_value_source({"navigator.language": "fr"})
    assert values == {"navigator.language": "fr"}
    assert sources == {"navigator.language": "environment"}


def test_runtime_coherence_value_source_none_environment(monkeypatch):
    import iv8_rs
    import iv8_rs.environment_toolchain_runtime as runtime

    class MockJSContext:
        @staticmethod
        def get_defaults():
            return {"navigator.language": "en-US"}

    monkeypatch.setattr(iv8_rs, "JSContext", MockJSContext)

    values, sources = runtime._coherence_value_source(None)
    assert values == {"navigator.language": "en-US"}
    assert sources == {"navigator.language": "profile_default"}


# ── _profile_suggestions_from_candidates ─────────────────────────────────────


def test_runtime_profile_suggestions_allowed(monkeypatch):
    from iv8_rs.environment_toolchain_asset_models import ToolchainCandidate
    from iv8_rs.environment_toolchain_runtime import _profile_suggestions_from_candidates

    class AllowedDecision:
        decision = "allowed"
        reason = "ok"

    monkeypatch.setattr(
        "iv8_rs.environment_toolchain_runtime.validate_bypass_boundary",
        lambda _: AllowedDecision(),
    )

    candidates = [
        ToolchainCandidate(
            patch_id="p1", target="navigator.language", target_family="navigator",
            kind="value", policy="runtime_safe", source="test", value_preview="en-US",
        ),
        ToolchainCandidate(
            patch_id="p2", target="screen.width", target_family="screen",
            kind="value", policy="runtime_safe", source="test", value_preview=1920,
        ),
    ]
    suggestions = _profile_suggestions_from_candidates(candidates)
    assert len(suggestions) == 2
    targets = {s.target for s in suggestions}
    assert targets == {"navigator.language", "screen.width"}


def test_runtime_profile_suggestions_blocked_and_dedup(monkeypatch):
    from iv8_rs.environment_toolchain_asset_models import ToolchainCandidate
    from iv8_rs.environment_toolchain_runtime import _profile_suggestions_from_candidates

    call_count = 0

    def mock_validate(payload):
        nonlocal call_count
        call_count += 1
        if payload["target"] == "navigator.language":
            class Blocked:
                decision = "blocked"
                blocked_terms = ["test"]
            return Blocked()
        class Allowed:
            decision = "allowed"
            reason = "ok"
        return Allowed()

    monkeypatch.setattr(
        "iv8_rs.environment_toolchain_runtime.validate_bypass_boundary",
        mock_validate,
    )

    candidates = [
        ToolchainCandidate(
            patch_id="p1", target="navigator.language", target_family="navigator",
            kind="value", policy="runtime_safe", source="test", value_preview="en-US",
        ),
        ToolchainCandidate(
            patch_id="p2", target="navigator.language", target_family="navigator",
            kind="value", policy="runtime_safe", source="test", value_preview="fr-FR",
        ),
        ToolchainCandidate(
            patch_id="p3", target="screen.width", target_family="screen",
            kind="value", policy="runtime_safe", source="test", value_preview=1920,
        ),
    ]
    suggestions = _profile_suggestions_from_candidates(candidates)
    assert len(suggestions) == 1
    assert suggestions[0].target == "screen.width"
    assert suggestions[0].value_preview == ["1920"]


def test_runtime_profile_suggestions_empty():
    from iv8_rs.environment_toolchain_runtime import _profile_suggestions_from_candidates
    assert _profile_suggestions_from_candidates([]) == []


# ── _resolve_local_overlay edge cases ────────────────────────────────────────


def test_runtime_resolve_local_overlay_none():
    from iv8_rs.environment_toolchain_runtime import _resolve_local_overlay
    values, provenance, rejected = _resolve_local_overlay(None)
    assert values is None
    assert provenance is None
    assert rejected is None


def test_runtime_resolve_local_overlay_path_non_generic_keys(tmp_path):
    import json

    from iv8_rs.environment_toolchain_runtime import _resolve_local_overlay

    path = tmp_path / "bad_keys.json"
    path.write_text(json.dumps({"custom.key": "val"}), encoding="utf-8")
    values, provenance, rejected = _resolve_local_overlay(path)
    assert values is None
    assert provenance is None
    assert rejected.code == "ENV_TOOLCHAIN_LOCAL_OVERLAY_REJECTED"
    assert "non-generic keys" in rejected.details["reason"]


def test_runtime_resolve_local_overlay_path_blocked_boundary(tmp_path, monkeypatch):
    import json

    import iv8_rs.environment_toolchain_runtime as runtime

    class BlockedDecision:
        decision = "blocked"
        blocked_terms = ["target_flow"]

    monkeypatch.setattr(runtime, "validate_bypass_boundary", lambda _: BlockedDecision())

    path = tmp_path / "blocked.json"
    path.write_text(json.dumps({"screen.width": 1920}), encoding="utf-8")
    values, provenance, rejected = runtime._resolve_local_overlay(path)
    assert values is None
    assert provenance is None
    assert rejected.code == "ENV_TOOLCHAIN_LOCAL_OVERLAY_REJECTED"
    assert "blocked by boundary" in rejected.details["reason"]


# ── run_probe_pack ───────────────────────────────────────────────────────────


def test_runtime_run_probe_pack_empty_source(monkeypatch):
    import iv8_rs
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import ProbeDefinition, ProbePack
    from iv8_rs.environment_toolchain_models import AssetProvenance

    probe_pack = ProbePack(
        probe_pack="test.m1", version=1, description="test",
        evidence_ceiling="diagnostic_only",
        probes=[ProbeDefinition(
            probe_id="a", target="navigator.webdriver", category="presence",
            js="return false;", expected=False, gap_class="missing_api",
        )],
    )
    monkeypatch.setattr(
        runtime, "_resolve_probe_pack",
        lambda _: (probe_pack, AssetProvenance("probe pack", "test.m1", "test")),
    )

    class MockJSContext:
        def __init__(self, **kw):
            self.kw = kw
        def eval(self, source):
            pass
        def close(self):
            pass

    monkeypatch.setattr(iv8_rs, "JSContext", MockJSContext)

    result = runtime.run_probe_pack("", probe_pack="test.m1")
    assert result.probe_pack == "test.m1"
    assert len(result.observations) == 1
    assert result.observations[0].passed is False
    assert len(result.gaps) == 1


def test_runtime_run_probe_pack_with_probes(monkeypatch):
    import iv8_rs
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import ProbeDefinition, ProbePack
    from iv8_rs.environment_toolchain_models import AssetProvenance

    probe_a = ProbeDefinition(
        probe_id="a", target="navigator.webdriver", category="presence",
        js="return true;", expected=True, gap_class="missing_api",
    )
    probe_b = ProbeDefinition(
        probe_id="b", target="screen.width", category="value",
        js="return 0;", expected=1920, gap_class="value_mismatch",
    )
    probe_pack = ProbePack(
        probe_pack="test.m1", version=1, description="test",
        evidence_ceiling="diagnostic_only",
        probes=[probe_a, probe_b],
    )
    monkeypatch.setattr(
        runtime, "_resolve_probe_pack",
        lambda _: (probe_pack, AssetProvenance("probe pack", "test.m1", "test")),
    )

    eval_results = iter([True, 0])

    class MockJSContext:
        def __init__(self, **kw):
            self.kw = kw
        def eval(self, source):
            return next(eval_results)
        def close(self):
            pass

    monkeypatch.setattr(iv8_rs, "JSContext", MockJSContext)

    result = runtime.run_probe_pack("", probe_pack="test.m1")
    assert result.probe_pack == "test.m1"
    assert len(result.observations) == 2
    assert result.observations[0].passed is True
    assert result.observations[1].passed is False
    assert len(result.gaps) == 1


def test_runtime_run_probe_pack_entry_expr_failure(monkeypatch):
    import iv8_rs
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import ProbeDefinition, ProbePack
    from iv8_rs.environment_toolchain_models import AssetProvenance

    probe_pack = ProbePack(
        probe_pack="test.m1", version=1, description="test",
        evidence_ceiling="diagnostic_only",
        probes=[ProbeDefinition(
            probe_id="a", target="navigator.webdriver", category="presence",
            js="return false;", expected=False, gap_class="missing_api",
        )],
    )
    monkeypatch.setattr(
        runtime, "_resolve_probe_pack",
        lambda _: (probe_pack, AssetProvenance("probe pack", "test.m1", "test")),
    )

    class MockJSContext:
        def __init__(self, **kw):
            self.kw = kw
            self.eval_count = 0
        def eval(self, source):
            self.eval_count += 1
            if self.eval_count == 2:
                raise RuntimeError("entry_expr failed")
        def close(self):
            pass

    monkeypatch.setattr(iv8_rs, "JSContext", MockJSContext)

    result = runtime.run_probe_pack("var x = 1;", probe_pack="test.m1", entry_expr="bad();")
    codes = {d["code"] for d in result.diagnostics}
    assert "ENV_TOOLCHAIN_ENTRY_EXPR_FAILED" in codes


# ── run_environment_toolchain ────────────────────────────────────────────────


def test_runtime_run_environment_toolchain_validation():
    from iv8_rs.environment_toolchain_runtime import run_environment_toolchain

    with pytest.raises(ValueError, match="max_iterations must be non-negative"):
        run_environment_toolchain("", max_iterations=-1)
    with pytest.raises(ValueError, match="dry_run_planning cannot be combined"):
        run_environment_toolchain("", dry_run_planning=True, apply_runtime_safe=True)
    with pytest.raises(ValueError, match="rollback_diagnostics cannot be combined"):
        run_environment_toolchain("", rollback_diagnostics=True, apply_runtime_safe=True)
    with pytest.raises(ValueError, match="substrate_coverage cannot be combined"):
        run_environment_toolchain("", substrate_coverage=True, apply_runtime_safe=True)
    with pytest.raises(ValueError, match="scaffold_gaps cannot be combined"):
        run_environment_toolchain("", scaffold_gaps=True, apply_runtime_safe=True)
    with pytest.raises(ValueError, match="pressure_harness cannot be combined"):
        run_environment_toolchain("", pressure_harness=True, adapt_runtime_safe=True)


def test_runtime_run_environment_toolchain_basic_report_only(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack, ToolchainCandidate
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        EnvironmentGap,
        ProbeRun,
    )

    candidate = ToolchainCandidate(
        patch_id="p1", target="navigator.webdriver", target_family="navigator",
        kind="value", policy="runtime_safe", source="test", value_preview=False,
    )
    candidate_pack = CandidatePack(
        candidate_pack="test", version=1, description="test",
        candidates=[candidate],
    )
    monkeypatch.setattr(
        runtime, "_resolve_candidate_pack",
        lambda _: (candidate_pack, AssetProvenance("candidate pack", "test", "test")),
    )

    probe_run = ProbeRun(
        probe_pack="fp.m1",
        observations=[],
        gaps=[EnvironmentGap(
            probe_id="a", target="navigator.webdriver", gap_class="missing_api",
            category="presence", expected=False, actual=None,
        )],
        coverage={"present": 0, "missing": 1, "mismatch": 0},
    )
    monkeypatch.setattr(runtime, "run_probe_pack", lambda *a, **kw: probe_run)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [candidate])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))

    report = runtime.run_environment_toolchain("var x=1;")
    assert report.probe_pack == "fp.m1"
    assert len(report.rejected_patches) == 1
    assert report.rejected_patches[0].patch_id == "p1"
    assert report.applied_patches == []


def test_runtime_run_environment_toolchain_apply_runtime_safe(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack, ToolchainCandidate
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        EnvironmentGap,
        ProbeObservation,
        ProbeRun,
    )

    candidate = ToolchainCandidate(
        patch_id="p1", target="navigator.webdriver", target_family="navigator",
        kind="value", policy="runtime_safe", source="test", value_preview=False,
    )
    candidate_pack = CandidatePack(
        candidate_pack="test", version=1, description="test",
        candidates=[candidate],
    )
    monkeypatch.setattr(
        runtime, "_resolve_candidate_pack",
        lambda _: (candidate_pack, AssetProvenance("candidate pack", "test", "test")),
    )

    before_gaps = [EnvironmentGap(
        probe_id="a", target="navigator.webdriver", gap_class="missing_api",
        category="presence", expected=False, actual=None,
    )]
    before_run = ProbeRun(
        probe_pack="fp.m1",
        observations=[ProbeObservation(
            probe_id="a", target="navigator.webdriver", category="presence",
            expected=False, actual=None, passed=False, gap_class="missing_api",
        )],
        gaps=before_gaps,
        coverage={"present": 0, "missing": 1, "mismatch": 0},
    )
    after_run = ProbeRun(
        probe_pack="fp.m1",
        observations=[ProbeObservation(
            probe_id="a", target="navigator.webdriver", category="presence",
            expected=False, actual=False, passed=True, gap_class="missing_api",
        )],
        gaps=[],
        coverage={"present": 1, "missing": 0, "mismatch": 0},
    )

    call_count = 0

    def mock_probe_pack(*a, **kw):
        nonlocal call_count
        call_count += 1
        return after_run if call_count == 2 else before_run

    monkeypatch.setattr(runtime, "run_probe_pack", mock_probe_pack)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [candidate])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))

    report = runtime.run_environment_toolchain("var x=1;", apply_runtime_safe=True)
    assert call_count == 2
    assert len(report.applied_patches) == 1
    assert report.applied_patches[0].patch_id == "p1"
    assert report.coverage_delta.improved > 0


def test_runtime_run_environment_toolchain_dry_run_planning(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        EnvironmentGap,
        ProbeRun,
    )

    monkeypatch.setattr(
        runtime, "_resolve_candidate_pack",
        lambda _: (CandidatePack(candidate_pack="test", version=1, description="test", candidates=[]),
                   AssetProvenance("candidate pack", "test", "test")),
    )
    probe_run = ProbeRun(
        probe_pack="fp.m1",
        observations=[],
        gaps=[EnvironmentGap(
            probe_id="a", target="navigator.webdriver", gap_class="missing_api",
            category="presence", expected=False, actual=None,
        )],
        coverage={"present": 0, "missing": 1, "mismatch": 0},
    )
    monkeypatch.setattr(runtime, "run_probe_pack", lambda *a, **kw: probe_run)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))
    monkeypatch.setattr(runtime, "_dry_run_planning_records", lambda *a, **kw: [])
    monkeypatch.setattr(
        runtime, "validate_bypass_boundary",
        lambda _: runtime.BoundaryDecision("allowed", "ok"),
    )

    report = runtime.run_environment_toolchain("var x=1;", dry_run_planning=True)
    codes = [d.code for d in report.diagnostics]
    assert "ENV_TOOLCHAIN_NO_WRITES" in codes


def test_runtime_run_environment_toolchain_additional_diagnostics(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        ProbeRun,
    )

    monkeypatch.setattr(
        runtime, "_resolve_candidate_pack",
        lambda _: (CandidatePack(candidate_pack="test", version=1, description="test", candidates=[]),
                   AssetProvenance("candidate pack", "test", "test")),
    )
    probe_run = ProbeRun(
        probe_pack="fp.m1",
        observations=[],
        gaps=[],
        coverage={"present": 1, "missing": 0, "mismatch": 0},
    )
    monkeypatch.setattr(runtime, "run_probe_pack", lambda *a, **kw: probe_run)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))
    monkeypatch.setattr(runtime, "_dry_run_planning_records", lambda *a, **kw: [])
    monkeypatch.setattr(runtime, "_rollback_diagnostic_records", lambda *a, **kw: [])
    monkeypatch.setattr(runtime, "_substrate_coverage_records", lambda: [])
    monkeypatch.setattr(runtime, "_scaffold_gap_records", lambda: [])
    monkeypatch.setattr(
        runtime, "validate_bypass_boundary",
        lambda _: runtime.BoundaryDecision("allowed", "ok"),
    )

    report = runtime.run_environment_toolchain(
        "var x=1;",
        rollback_diagnostics=True,
        substrate_coverage=True,
        scaffold_gaps=True,
    )
    codes = {d.code for d in report.diagnostics}
    assert "ENV_TOOLCHAIN_NO_WRITES" in codes


def test_runtime_run_environment_toolchain_pressure_harness(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        ProbeRun,
    )

    monkeypatch.setattr(
        runtime, "_resolve_candidate_pack",
        lambda _: (CandidatePack(candidate_pack="test", version=1, description="test", candidates=[]),
                   AssetProvenance("candidate pack", "test", "test")),
    )
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))
    monkeypatch.setattr(runtime, "_pressure_harness_records", lambda _: [])
    monkeypatch.setattr(runtime, "_pressure_plan_records", lambda _: [])

    probe_run = ProbeRun(
        probe_pack="fp.m1", observations=[], gaps=[],
        coverage={"present": 1, "missing": 0, "mismatch": 0},
    )
    call_count = 0

    def mock_probe_pack(*a, **kw):
        nonlocal call_count
        call_count += 1
        if call_count == 1:
            raise RuntimeError("first run failed")
        return probe_run

    monkeypatch.setattr(runtime, "run_probe_pack", mock_probe_pack)

    report = runtime.run_environment_toolchain("var x=1;", pressure_harness=True)
    assert call_count == 2
    codes = {d.code for d in report.diagnostics}
    assert "ENV_TOOLCHAIN_NO_WRITES" in codes


def test_runtime_run_environment_toolchain_overlay_provenance(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack
    from iv8_rs.environment_toolchain_models import AssetProvenance, ProbeRun
    from iv8_rs.experimental_report import ExperimentalDiagnosticRecord

    monkeypatch.setattr(
        runtime, "_resolve_candidate_pack",
        lambda _: (CandidatePack(candidate_pack="test", version=1, description="test", candidates=[]),
                   AssetProvenance("candidate pack", "test", "test")),
    )
    probe_run = ProbeRun(
        probe_pack="fp.m1", observations=[], gaps=[],
        coverage={"present": 1, "missing": 0, "mismatch": 0},
    )
    monkeypatch.setattr(runtime, "run_probe_pack", lambda *a, **kw: probe_run)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))

    overlay_prov = ExperimentalDiagnosticRecord("OVERLAY_PROV", "info", {"k": 1})
    overlay_rej = ExperimentalDiagnosticRecord("OVERLAY_REJ", "warn", {"k": 2})
    monkeypatch.setattr(
        runtime, "_resolve_local_overlay",
        lambda _: ({"screen.width": 1920}, overlay_prov, overlay_rej),
    )

    report = runtime.run_environment_toolchain("var x=1;")
    codes = {d.code for d in report.diagnostics}
    assert "OVERLAY_PROV" in codes
    assert "OVERLAY_REJ" in codes


# ── _run_iterative_environment_toolchain ─────────────────────────────────────


def test_runtime_iterative_environment_toolchain_completes(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack, ToolchainCandidate
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        EnvironmentGap,
        ProbeObservation,
        ProbeRun,
    )

    candidate = ToolchainCandidate(
        patch_id="p1", target="navigator.webdriver", target_family="navigator",
        kind="value", policy="runtime_safe", source="test", value_preview=False,
    )
    candidate_pack = CandidatePack(
        candidate_pack="test", version=1, description="test", candidates=[candidate],
    )
    gap = EnvironmentGap(
        probe_id="a", target="navigator.webdriver", gap_class="missing_api",
        category="presence", expected=False, actual=None,
    )
    before_run = ProbeRun(
        probe_pack="fp.m1",
        observations=[ProbeObservation(
            probe_id="a", target="navigator.webdriver", category="presence",
            expected=False, actual=None, passed=False, gap_class="missing_api",
        )],
        gaps=[gap],
        coverage={"present": 0, "missing": 1, "mismatch": 0},
        diagnostics=[{"code": "INIT", "severity": "info"}],
    )
    after_run = ProbeRun(
        probe_pack="fp.m1",
        observations=[ProbeObservation(
            probe_id="a", target="navigator.webdriver", category="presence",
            expected=False, actual=False, passed=True, gap_class="missing_api",
        )],
        gaps=[],
        coverage={"present": 1, "missing": 0, "mismatch": 0},
        diagnostics=[{"code": "AFTER", "severity": "info"}],
    )

    call_count = [0]

    def mock_probe_pack(*a, **kw):
        call_count[0] += 1
        return after_run if call_count[0] > 1 else before_run

    monkeypatch.setattr(runtime, "run_probe_pack", mock_probe_pack)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [candidate])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))

    report = runtime._run_iterative_environment_toolchain(
        "var x=1;",
        probe_pack="fp.m1",
        profile="default",
        environment=None,
        candidate_pack_object=candidate_pack,
        candidate_provenance=AssetProvenance("candidate pack", "test", "test"),
        max_iterations=5,
        stop_on_regression=True,
        random_seed=42,
        time_freeze=None,
        time_mode="logical",
        entry_expr=None,
    )
    assert report.coverage_delta.improved > 0


def test_runtime_iterative_environment_toolchain_no_gaps(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack
    from iv8_rs.environment_toolchain_models import AssetProvenance, ProbeRun

    probe_run = ProbeRun(
        probe_pack="fp.m1", observations=[], gaps=[],
        coverage={"present": 1, "missing": 0, "mismatch": 0},
        diagnostics=[{"code": "TEST", "severity": "info"}],
    )
    monkeypatch.setattr(runtime, "run_probe_pack", lambda *a, **kw: probe_run)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))

    report = runtime._run_iterative_environment_toolchain(
        "",
        probe_pack="fp.m1", profile=None, environment=None,
        candidate_pack_object=CandidatePack("test", 1, "test", []),
        candidate_provenance=AssetProvenance("candidate pack", "test", "test"),
        max_iterations=0, stop_on_regression=True,
        random_seed=None, time_freeze=None, time_mode="logical",
        entry_expr=None,
    )
    assert report.coverage_delta.improved == 0


def test_runtime_iterative_environment_toolchain_no_candidates(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack
    from iv8_rs.environment_toolchain_models import AssetProvenance, EnvironmentGap, ProbeRun

    probe_run = ProbeRun(
        probe_pack="fp.m1", observations=[], gaps=[EnvironmentGap(
            probe_id="a", target="x", gap_class="missing_api",
            category="presence", expected=1, actual=None,
        )],
        coverage={"present": 0, "missing": 1, "mismatch": 0},
        diagnostics=[{"code": "TEST", "severity": "info"}],
    )
    monkeypatch.setattr(runtime, "run_probe_pack", lambda *a, **kw: probe_run)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))

    report = runtime._run_iterative_environment_toolchain(
        "",
        probe_pack="fp.m1", profile=None, environment=None,
        candidate_pack_object=CandidatePack("test", 1, "test", []),
        candidate_provenance=AssetProvenance("candidate pack", "test", "test"),
        max_iterations=1, stop_on_regression=True,
        random_seed=None, time_freeze=None, time_mode="logical",
        entry_expr=None,
    )
    assert report.coverage_delta.unresolved > 0


def test_runtime_iterative_environment_toolchain_budget_exhausted(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack, ToolchainCandidate
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        EnvironmentGap,
        ProbeRun,
    )

    candidate = ToolchainCandidate(
        patch_id="p1", target="x", target_family="navigator",
        kind="value", policy="runtime_safe", source="test", value_preview=False,
    )
    candidate_pack = CandidatePack(
        candidate_pack="test", version=1, description="test", candidates=[candidate],
    )
    gap = EnvironmentGap(
        probe_id="a", target="x", gap_class="missing_api",
        category="presence", expected=False, actual=None,
    )
    probe_run = ProbeRun(
        probe_pack="fp.m1", observations=[], gaps=[gap],
        coverage={"present": 0, "missing": 1, "mismatch": 0},
        diagnostics=[{"code": "TEST", "severity": "info"}],
    )
    monkeypatch.setattr(runtime, "run_probe_pack", lambda *a, **kw: probe_run)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [candidate])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))

    report = runtime._run_iterative_environment_toolchain(
        "",
        probe_pack="fp.m1", profile=None, environment=None,
        candidate_pack_object=candidate_pack,
        candidate_provenance=AssetProvenance("candidate pack", "test", "test"),
        max_iterations=1, stop_on_regression=False,
        random_seed=None, time_freeze=None, time_mode="logical",
        entry_expr=None,
    )
    assert report.coverage_delta.unresolved > 0


# ── _map_gaps_to_family_pressures — family None skip ─────────────────────────


def test_runtime_map_gaps_family_none_skip():
    from iv8_rs.environment_toolchain_models import EnvironmentGap
    from iv8_rs.environment_toolchain_runtime import _map_gaps_to_family_pressures

    gaps = [
        EnvironmentGap(
            probe_id="p1", target="unknown.target", gap_class="missing_api",
            category="surface", expected=1, actual=None,
        ),
        EnvironmentGap(
            probe_id="p2", target="screen.width", gap_class="missing_api",
            category="surface", expected=1, actual=None,
        ),
    ]
    pressures = _map_gaps_to_family_pressures(gaps)
    target_families = {p.target_family for p in pressures}
    assert "screen" in target_families
    assert all(p.pressure_id.count("__") == 1 for p in pressures)


# ── _coverage_delta — probe not in after_run ─────────────────────────────────


def test_runtime_coverage_delta_probe_missing_in_after():
    from types import SimpleNamespace

    from iv8_rs.environment_toolchain_runtime import _coverage_delta

    before = SimpleNamespace(observations=[
        SimpleNamespace(probe_id="a", passed=False),
        SimpleNamespace(probe_id="b", passed=True),
        SimpleNamespace(probe_id="c", passed=False),
    ], gaps=["a"])
    after = SimpleNamespace(observations=[
        SimpleNamespace(probe_id="a", passed=True),
        SimpleNamespace(probe_id="c", passed=False),
    ], gaps=["c"])
    delta = _coverage_delta(before, after)
    assert delta == {"improved": 1, "regressed": 0, "unresolved": 1}


# ── _classify_target_family — exact and prefix matches ───────────────────────


def test_runtime_classify_target_family_exact_match():
    from iv8_rs.environment_toolchain_runtime import _classify_target_family
    assert _classify_target_family("navigator") == "navigator"
    assert _classify_target_family("screen") == "screen"
    assert _classify_target_family("window") == "window"


def test_runtime_classify_target_family_prefix_match():
    from iv8_rs.environment_toolchain_runtime import _classify_target_family
    assert _classify_target_family("screen.width") == "screen"
    assert _classify_target_family("window.innerWidth") == "window"
    assert _classify_target_family("navigator.userAgent") == "navigator"


# ── coherence group edge cases ───────────────────────────────────────────────


def test_runtime_ua_platform_mobile_value_malformed():
    from iv8_rs.environment_toolchain_runtime import _ua_platform_coherence_group

    result = _ua_platform_coherence_group(
        {
            "navigator.userAgent": "Windows NT 10.0",
            "navigator.platform": "Win32",
            "navigator.userAgentData.mobile": "yes",
        },
        {},
    )
    assert result.status == "unknown"
    assert "mobile value is malformed" in result.reason


def test_runtime_network_connection_type_empty():
    from iv8_rs.environment_toolchain_runtime import _network_info_coherence_group

    result = _network_info_coherence_group(
        {"navigator.connection.type": ""}, {}
    )
    assert result.status == "unknown"
    assert "type value is unavailable or malformed" in result.reason


def test_runtime_timezone_locale_both_none_consistent_language():
    from iv8_rs.environment_toolchain_runtime import _timezone_locale_coherence_group

    result = _timezone_locale_coherence_group(
        {
            "navigator.language": "en-US",
            "navigator.languages": ["en-US", "en"],
        },
        {},
    )
    assert result.status == "unknown"
    assert "timezone value is unavailable" in result.reason


def test_runtime_timezone_locale_both_none_inconsistent_language():
    from iv8_rs.environment_toolchain_runtime import _timezone_locale_coherence_group

    result = _timezone_locale_coherence_group(
        {
            "navigator.language": "fr-FR",
            "navigator.languages": ["en-US", "en"],
        },
        {},
    )
    assert result.status == "inconsistent"
    assert "does not match" in result.reason


# ── remaining edge cases ─────────────────────────────────────────────────────


def test_runtime_classify_target_family_fallback_family_prefix():
    from iv8_rs.environment_toolchain_runtime import _classify_target_family
    assert _classify_target_family("network_info.rtt") == "network_info"


def test_runtime_run_environment_toolchain_adapt_runtime_safe(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        ProbeRun,
    )

    monkeypatch.setattr(
        runtime, "_resolve_candidate_pack",
        lambda _: (CandidatePack(candidate_pack="test", version=1, description="test", candidates=[]),
                   AssetProvenance("candidate pack", "test", "test")),
    )
    probe_run = ProbeRun(
        probe_pack="fp.m1", observations=[], gaps=[],
        coverage={"present": 1, "missing": 0, "mismatch": 0},
        diagnostics=[{"code": "TEST", "severity": "info"}],
    )
    monkeypatch.setattr(runtime, "run_probe_pack", lambda *a, **kw: probe_run)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))

    report = runtime.run_environment_toolchain("var x=1;", adapt_runtime_safe=True)
    assert report.probe_pack == "fp.m1"


def test_runtime_run_environment_toolchain_pressure_harness_first_ok(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        ProbeRun,
    )

    monkeypatch.setattr(
        runtime, "_resolve_candidate_pack",
        lambda _: (CandidatePack(candidate_pack="test", version=1, description="test", candidates=[]),
                   AssetProvenance("candidate pack", "test", "test")),
    )
    probe_run = ProbeRun(
        probe_pack="fp.m1", observations=[], gaps=[],
        coverage={"present": 1, "missing": 0, "mismatch": 0},
    )
    monkeypatch.setattr(runtime, "run_probe_pack", lambda *a, **kw: probe_run)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))
    monkeypatch.setattr(runtime, "_pressure_harness_records", lambda _: [])

    report = runtime.run_environment_toolchain("var x=1;", pressure_harness=True)
    assert report.probe_pack == "fp.m1"


def test_runtime_run_environment_toolchain_regressed_and_suggestions(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack, ToolchainCandidate
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        EnvironmentGap,
        ProbeObservation,
        ProbeRun,
    )

    candidate = ToolchainCandidate(
        patch_id="p1", target="navigator.webdriver", target_family="navigator",
        kind="value", policy="runtime_safe", source="test", value_preview=False,
    )
    monkeypatch.setattr(
        runtime, "_resolve_candidate_pack",
        lambda _: (CandidatePack(candidate_pack="test", version=1, description="test",
                                 candidates=[candidate]),
                   AssetProvenance("candidate pack", "test", "test")),
    )
    before_run = ProbeRun(
        probe_pack="fp.m1",
        observations=[ProbeObservation(
            probe_id="a", target="navigator.webdriver", category="presence",
            expected=False, actual=False, passed=True, gap_class="missing_api",
        )],
        gaps=[],
        coverage={"present": 1, "missing": 0, "mismatch": 0},
    )
    after_run = ProbeRun(
        probe_pack="fp.m1",
        observations=[ProbeObservation(
            probe_id="a", target="navigator.webdriver", category="presence",
            expected=False, actual=None, passed=False, gap_class="missing_api",
        )],
        gaps=[EnvironmentGap(
            probe_id="a", target="navigator.webdriver", gap_class="missing_api",
            category="presence", expected=False, actual=None,
        )],
        coverage={"present": 0, "missing": 1, "mismatch": 0},
    )

    call_count = [0]

    def mock_probe_pack(*a, **kw):
        call_count[0] += 1
        return after_run if call_count[0] == 2 else before_run

    monkeypatch.setattr(runtime, "run_probe_pack", mock_probe_pack)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [candidate])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))

    report = runtime.run_environment_toolchain("var x=1;", apply_runtime_safe=True)
    codes = {d.code for d in report.diagnostics}
    assert "ENV_TOOLCHAIN_COVERAGE_REGRESSED" in codes


def test_runtime_run_environment_toolchain_pressure_harness_dry_run(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        ProbeRun,
    )

    monkeypatch.setattr(
        runtime, "_resolve_candidate_pack",
        lambda _: (CandidatePack(candidate_pack="test", version=1, description="test", candidates=[]),
                   AssetProvenance("candidate pack", "test", "test")),
    )
    probe_run = ProbeRun(
        probe_pack="fp.m1", observations=[], gaps=[], coverage={"present": 1, "missing": 0, "mismatch": 0},
    )
    monkeypatch.setattr(runtime, "run_probe_pack", lambda *a, **kw: probe_run)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))
    monkeypatch.setattr(runtime, "_pressure_harness_records", lambda _: [])
    monkeypatch.setattr(runtime, "_pressure_plan_records", lambda _: [])
    monkeypatch.setattr(
        runtime, "validate_bypass_boundary",
        lambda _: runtime.BoundaryDecision("allowed", "ok"),
    )

    report = runtime.run_environment_toolchain(
        "var x=1;", pressure_harness=True, dry_run_planning=True,
    )
    assert report.probe_pack == "fp.m1"


def test_runtime_iterative_environment_toolchain_loop_breaks_no_gaps(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        ProbeRun,
    )

    probe_run = ProbeRun(
        probe_pack="fp.m1", observations=[], gaps=[],
        coverage={"present": 1, "missing": 0, "mismatch": 0},
        diagnostics=[{"code": "TEST", "severity": "info"}],
    )
    monkeypatch.setattr(runtime, "run_probe_pack", lambda *a, **kw: probe_run)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))

    report = runtime._run_iterative_environment_toolchain(
        "",
        probe_pack="fp.m1", profile=None, environment=None,
        candidate_pack_object=CandidatePack("test", 1, "test", []),
        candidate_provenance=AssetProvenance("candidate pack", "test", "test"),
        max_iterations=1, stop_on_regression=True,
        random_seed=None, time_freeze=None, time_mode="logical",
        entry_expr=None,
    )
    assert report.coverage_delta.unresolved == 0


def test_runtime_iterative_environment_toolchain_regression(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack, ToolchainCandidate
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        EnvironmentGap,
        ProbeObservation,
        ProbeRun,
    )

    candidate = ToolchainCandidate(
        patch_id="p1", target="x", target_family="navigator",
        kind="value", policy="runtime_safe", source="test", value_preview=False,
    )
    gap_b = EnvironmentGap(
        probe_id="b", target="y", gap_class="missing_api",
        category="presence", expected=1, actual=None,
    )
    before_run = ProbeRun(
        probe_pack="fp.m1",
        observations=[
            ProbeObservation(
                probe_id="a", target="x", category="presence",
                expected=False, actual=False, passed=True, gap_class="missing_api",
            ),
            ProbeObservation(
                probe_id="b", target="y", category="presence",
                expected=1, actual=None, passed=False, gap_class="missing_api",
            ),
        ],
        gaps=[gap_b],
        coverage={"present": 1, "missing": 1, "mismatch": 0},
        diagnostics=[{"code": "BEFORE", "severity": "info"}],
    )
    after_run = ProbeRun(
        probe_pack="fp.m1",
        observations=[
            ProbeObservation(
                probe_id="a", target="x", category="presence",
                expected=False, actual=None, passed=False, gap_class="missing_api",
            ),
            ProbeObservation(
                probe_id="b", target="y", category="presence",
                expected=1, actual=None, passed=False, gap_class="missing_api",
            ),
        ],
        gaps=[EnvironmentGap(
            probe_id="a", target="x", gap_class="missing_api",
            category="presence", expected=False, actual=None,
        ), gap_b],
        coverage={"present": 0, "missing": 2, "mismatch": 0},
        diagnostics=[{"code": "AFTER", "severity": "info"}],
    )

    call_count = [0]

    def mock_probe_pack(*a, **kw):
        call_count[0] += 1
        return after_run if call_count[0] > 1 else before_run

    monkeypatch.setattr(runtime, "run_probe_pack", mock_probe_pack)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [candidate])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_resolve_local_overlay", lambda _: (None, None, None))
    monkeypatch.setattr(runtime, "_profile_suggestions_from_candidates", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))

    report = runtime._run_iterative_environment_toolchain(
        "",
        probe_pack="fp.m1", profile=None, environment=None,
        candidate_pack_object=CandidatePack("test", 1, "test", [candidate]),
        candidate_provenance=AssetProvenance("candidate pack", "test", "test"),
        max_iterations=1, stop_on_regression=True,
        random_seed=None, time_freeze=None, time_mode="logical",
        entry_expr=None,
    )
    assert report.coverage_delta.regressed > 0


def test_runtime_iterative_environment_toolchain_overlay(monkeypatch):
    import iv8_rs.environment_toolchain_runtime as runtime
    from iv8_rs.environment_toolchain_asset_models import CandidatePack
    from iv8_rs.environment_toolchain_models import (
        AssetProvenance,
        EnvironmentGap,
        ProbeRun,
    )
    from iv8_rs.experimental_report import ExperimentalDiagnosticRecord

    probe_run = ProbeRun(
        probe_pack="fp.m1", observations=[], gaps=[EnvironmentGap(
            probe_id="a", target="x", gap_class="missing_api",
            category="presence", expected=1, actual=None,
        )],
        coverage={"present": 0, "missing": 1, "mismatch": 0},
        diagnostics=[{"code": "TEST", "severity": "info"}],
    )
    monkeypatch.setattr(runtime, "run_probe_pack", lambda *a, **kw: probe_run)
    monkeypatch.setattr(runtime, "map_gaps_to_candidates", lambda *a, **kw: [])
    monkeypatch.setattr(runtime, "_profile_coherence_groups", lambda _: [])
    monkeypatch.setattr(runtime, "_map_gaps_to_family_pressures", lambda _: [])
    monkeypatch.setattr(runtime, "_coherence_value_source", lambda _: ({}, {}))

    overlay_prov = ExperimentalDiagnosticRecord("PROV", "info", {})
    overlay_rej = ExperimentalDiagnosticRecord("REJ", "warn", {})
    monkeypatch.setattr(
        runtime, "_resolve_local_overlay",
        lambda _: ({"x": 1}, overlay_prov, overlay_rej),
    )

    report = runtime._run_iterative_environment_toolchain(
        "",
        probe_pack="fp.m1", profile=None, environment=None,
        candidate_pack_object=CandidatePack("test", 1, "test", []),
        candidate_provenance=AssetProvenance("candidate pack", "test", "test"),
        max_iterations=1, stop_on_regression=True,
        random_seed=None, time_freeze=None, time_mode="logical",
        entry_expr=None,
    )
    codes = {d.code for d in report.diagnostics}
    assert "PROV" in codes
    assert "REJ" in codes


def test_runtime_profile_suggestions_dedup_path(monkeypatch):
    from iv8_rs.environment_toolchain_asset_models import ToolchainCandidate
    from iv8_rs.environment_toolchain_runtime import _profile_suggestions_from_candidates

    class AllowedDecision:
        decision = "allowed"
        reason = "ok"

    monkeypatch.setattr(
        "iv8_rs.environment_toolchain_runtime.validate_bypass_boundary",
        lambda _: AllowedDecision(),
    )

    candidates = [
        ToolchainCandidate(
            patch_id="p1", target="navigator.language", target_family="navigator",
            kind="value", policy="runtime_safe", source="test", value_preview="en-US",
        ),
        ToolchainCandidate(
            patch_id="p2", target="navigator.language", target_family="navigator",
            kind="value", policy="runtime_safe", source="test", value_preview="fr-FR",
        ),
    ]
    suggestions = _profile_suggestions_from_candidates(candidates)
    assert len(suggestions) == 1
    assert suggestions[0].target == "navigator.language"
