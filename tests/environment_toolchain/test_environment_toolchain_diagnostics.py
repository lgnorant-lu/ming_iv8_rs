"""Contract and behavioral tests for environment_toolchain_diagnostics."""
import pytest

iv8_rs = pytest.importorskip("iv8_rs")


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

class FakeCandidate:
    def __init__(self, patch_id, target="t1", policy="runtime_safe",
                 target_family="navigator", validation=None, metadata=None):
        self.patch_id = patch_id
        self.target = target
        self.policy = policy
        self.target_family = target_family
        self.validation = validation or {}
        self.metadata = metadata or {}


def _make_boundary(decision="allowed", blocked_terms=None):
    from types import SimpleNamespace
    return SimpleNamespace(decision=decision, blocked_terms=blocked_terms or [])


def _make_registry(mapping):
    class Registry:
        @staticmethod
        def get(target, default=None):
            return mapping.get(target, default or [])
    return Registry()


def _make_pressure_report(data=None, diagnostics=None):
    class FakePressureReport:
        def to_dict(self):
            return data or {"schema_version": "1", "sample_id": "s1",
                            "input_kind": "probe", "execution_mode": "auto",
                            "status": "ok", "failure_kind": None,
                            "pressure": {"pressure_kind": "analysis_observability"},
                            "promotion": {"level": "observe_only"},
                            "writes": []}
    obj = FakePressureReport()
    obj.diagnostics = diagnostics or []
    return obj


# ---------------------------------------------------------------------------
# _adaptation_records
# ---------------------------------------------------------------------------

class TestAdaptationRecords:
    def test_valid_stop_reasons(self):
        from iv8_rs.environment_toolchain_diagnostics import _adaptation_records
        from iv8_rs.environment_toolchain_static import _ADAPTATION_STOP_REASONS

        valid_reason = next(r for r in ["completed", "disabled", "no_progress"]
                          if r in _ADAPTATION_STOP_REASONS)
        records = _adaptation_records(
            enabled=True, max_iterations=5, iterations=[],
            stop_reason=valid_reason,
            applied_candidates=[],
        )
        assert len(records) >= 1
        assert records[0].code == "ENV_TOOLCHAIN_ADAPTATION_SUMMARY"

    def test_invalid_stop_reason_raises(self):
        from iv8_rs.environment_toolchain_diagnostics import _adaptation_records

        with pytest.raises(ValueError, match="invalid adaptation stop reason"):
            _adaptation_records(
                enabled=True, max_iterations=5, iterations=[],
                stop_reason="INVALID_REASON", applied_candidates=[],
            )

    def test_with_iterations_and_applied(self):
        from iv8_rs.environment_toolchain_diagnostics import _adaptation_records
        from iv8_rs.environment_toolchain_models import AdaptationIteration

        it = AdaptationIteration(index=0, before={"a": 1}, after={"a": 2},
                                 delta={"a": 1}, matched_patch_ids=["p1"],
                                 applied_patch_ids=["p1"])
        records = _adaptation_records(
            enabled=False, max_iterations=3, iterations=[it],
            stop_reason="completed",
            applied_candidates=[FakeCandidate(patch_id="p1")],
        )
        assert len(records) == 2
        assert records[1].code == "ENV_TOOLCHAIN_ADAPTATION_ITERATION"


# ---------------------------------------------------------------------------
# _dry_run_planning_records + _dry_run_plan_item
# ---------------------------------------------------------------------------

class TestDryRunPlanning:

    def test_no_candidate_pack(self):
        from iv8_rs.environment_toolchain_diagnostics import _dry_run_planning_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _dry_run_planning_records(
            [gap], candidate_pack_object=None, environment={"t1": "v1"},
            candidate_registry=lambda _: _make_registry({}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(),
        )
        assert len(records) == 1  # summary only, no items
        assert records[0].code == "ENV_TOOLCHAIN_DRY_RUN_PLAN_SUMMARY"

    def test_with_candidates_eligible(self):
        from iv8_rs.environment_toolchain_diagnostics import _dry_run_planning_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c = FakeCandidate(patch_id="c1", target="t1")
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _dry_run_planning_records(
            [gap], candidate_pack_object={}, environment={},
            candidate_registry=lambda _: _make_registry({"t1": [c]}),
            classify_target_family=lambda t: "custom_family",
            validate_boundary=lambda _: _make_boundary(),
        )
        assert len(records) == 2
        assert records[1].details["planning_status"] == "eligible_for_review"
        assert records[1].details["target_family"] == "custom_family"

    def test_duplicate_patch_id_skipped(self):
        from iv8_rs.environment_toolchain_diagnostics import _dry_run_planning_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c1 = FakeCandidate(patch_id="c1", target="t1")
        c2 = FakeCandidate(patch_id="c1", target="t1")
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _dry_run_planning_records(
            [gap], candidate_pack_object={}, environment={},
            candidate_registry=lambda _: _make_registry({"t1": [c1, c2]}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(),
        )
        assert len(records) == 2  # summary + 1 item (dupe skipped)

    def test_gap_class_filter_skips(self):
        from iv8_rs.environment_toolchain_diagnostics import _dry_run_planning_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c = FakeCandidate(patch_id="c1", target="t1",
                          validation={"gap_classes": ["value_mismatch"]})
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _dry_run_planning_records(
            [gap], candidate_pack_object={}, environment={},
            candidate_registry=lambda _: _make_registry({"t1": [c]}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(),
        )
        assert len(records) == 1  # candidate filtered out

    def test_blocked_by_conflict(self):
        from iv8_rs.environment_toolchain_diagnostics import _dry_run_planning_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c = FakeCandidate(patch_id="c1", target="t1")
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _dry_run_planning_records(
            [gap], candidate_pack_object={}, environment={"t1": "existing"},
            candidate_registry=lambda _: _make_registry({"t1": [c]}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(),
        )
        assert records[1].details["planning_status"] == "blocked_by_conflict"
        assert "explicit_environment_precedence" in records[1].details["blocked_reasons"]

    def test_blocked_by_policy(self):
        from iv8_rs.environment_toolchain_diagnostics import _dry_run_planning_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c = FakeCandidate(patch_id="c1", target="t1", policy="observe_only")
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _dry_run_planning_records(
            [gap], candidate_pack_object={}, environment={},
            candidate_registry=lambda _: _make_registry({"t1": [c]}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(),
        )
        assert records[1].details["planning_status"] == "blocked_by_policy"
        assert "non_runtime_safe_policy" in records[1].details["blocked_reasons"]

    def test_blocked_by_boundary(self):
        from iv8_rs.environment_toolchain_diagnostics import _dry_run_planning_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c = FakeCandidate(patch_id="c1", target="t1")
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _dry_run_planning_records(
            [gap], candidate_pack_object={}, environment={},
            candidate_registry=lambda _: _make_registry({"t1": [c]}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(decision="blocked",
                                                        blocked_terms=["unsafe_hardening"]),
        )
        assert records[1].details["planning_status"] == "blocked_by_boundary"


# ---------------------------------------------------------------------------
# _rollback_diagnostic_records + _rollback_record_details
# ---------------------------------------------------------------------------

class TestRollbackDiagnostics:

    def test_no_candidate_pack(self):
        from iv8_rs.environment_toolchain_diagnostics import _rollback_diagnostic_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _rollback_diagnostic_records(
            [gap], candidate_pack_object=None,
            candidate_registry=lambda _: _make_registry({}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(),
        )
        assert records[0].code == "ENV_TOOLCHAIN_ROLLBACK_SUMMARY"

    def test_with_candidates(self):
        from iv8_rs.environment_toolchain_diagnostics import _rollback_diagnostic_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c = FakeCandidate(patch_id="c1", target="t1",
                          metadata={"rollback_scope": "context_only"})
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _rollback_diagnostic_records(
            [gap], candidate_pack_object={},
            candidate_registry=lambda _: _make_registry({"t1": [c]}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(),
        )
        assert len(records) == 2
        assert records[1].code == "ENV_TOOLCHAIN_ROLLBACK_RECORD"

    def test_with_duplicate_and_gap_class_filter(self):
        from iv8_rs.environment_toolchain_diagnostics import _rollback_diagnostic_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c1 = FakeCandidate(patch_id="c1", target="t1",
                           metadata={"rollback_scope": "context_only"})
        c2 = FakeCandidate(patch_id="c1", target="t1")  # duplicate
        c3 = FakeCandidate(patch_id="c3", target="t1",   # filtered by gap_class
                           validation={"gap_classes": ["value_mismatch"]})
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _rollback_diagnostic_records(
            [gap], candidate_pack_object={},
            candidate_registry=lambda _: _make_registry({"t1": [c1, c2, c3]}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(),
        )
        assert len(records) == 2  # 1 record, dup skipped, gap_class filtered

    def test_blocked_scope(self):
        from iv8_rs.environment_toolchain_diagnostics import _rollback_diagnostic_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c = FakeCandidate(patch_id="c1", target="t1",
                          metadata={"rollback_scope": "profile_file"})
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _rollback_diagnostic_records(
            [gap], candidate_pack_object={},
            candidate_registry=lambda _: _make_registry({"t1": [c]}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(),
        )
        assert records[1].details["review_status"] == "blocked"
        assert records[1].details["restore_strategy"] == "blocked"

    def test_allowed_scope_context_only(self):
        from iv8_rs.environment_toolchain_diagnostics import _rollback_diagnostic_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c = FakeCandidate(patch_id="c1", target="t1",
                          metadata={"rollback_scope": "context_only"})
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _rollback_diagnostic_records(
            [gap], candidate_pack_object={},
            candidate_registry=lambda _: _make_registry({"t1": [c]}),
            classify_target_family=lambda t: "my_family",
            validate_boundary=lambda _: _make_boundary(),
        )
        assert records[1].details["review_status"] == "review_only"
        assert records[1].details["restore_strategy"] == "context_discard"
        assert records[1].details["target_family"] == "my_family"

    def test_allowed_scope_ephemeral(self):
        from iv8_rs.environment_toolchain_diagnostics import _rollback_diagnostic_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c = FakeCandidate(patch_id="c1", target="t1",
                          metadata={"rollback_scope": "ephemeral_report"})
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _rollback_diagnostic_records(
            [gap], candidate_pack_object={},
            candidate_registry=lambda _: _make_registry({"t1": [c]}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(),
        )
        assert records[1].details["restore_strategy"] == "remove_value"

    def test_invalid_scope(self):
        from iv8_rs.environment_toolchain_diagnostics import _rollback_diagnostic_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c = FakeCandidate(patch_id="c1", target="t1",
                          metadata={"rollback_scope": "bogus_scope"})
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _rollback_diagnostic_records(
            [gap], candidate_pack_object={},
            candidate_registry=lambda _: _make_registry({"t1": [c]}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(),
        )
        assert records[1].details["review_status"] == "blocked"
        assert "invalid_rollback_scope" in records[1].details["blocked_reasons"]

    def test_validate_boundary_blocks(self):
        from iv8_rs.environment_toolchain_diagnostics import _rollback_diagnostic_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c = FakeCandidate(patch_id="c1", target="t1",
                          metadata={"rollback_scope": "context_only"})
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _rollback_diagnostic_records(
            [gap], candidate_pack_object={},
            candidate_registry=lambda _: _make_registry({"t1": [c]}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(decision="blocked",
                                                        blocked_terms=["scope_risk"]),
        )
        assert records[1].details["review_status"] == "blocked"
        assert records[1].details["restore_strategy"] == "blocked"
        assert "scope_risk" in records[1].details["blocked_reasons"]

    def test_scope_from_validation_fallback(self):
        from iv8_rs.environment_toolchain_diagnostics import _rollback_diagnostic_records
        from iv8_rs.environment_toolchain_models import EnvironmentGap

        c = FakeCandidate(patch_id="c1", target="t1",
                          validation={"rollback_scope": "context_only"})
        gap = EnvironmentGap(probe_id="p1", target="t1", gap_class="missing",
                             category="missing_api", expected="x", actual="y")
        records = _rollback_diagnostic_records(
            [gap], candidate_pack_object={},
            candidate_registry=lambda _: _make_registry({"t1": [c]}),
            classify_target_family=lambda t: None,
            validate_boundary=lambda _: _make_boundary(),
        )
        assert records[1].details["scope"] == "context_only"
        assert records[1].details["restore_strategy"] == "context_discard"


# ---------------------------------------------------------------------------
# _substrate_coverage_records / _substrate_coverage_item_details
# ---------------------------------------------------------------------------

class TestSubstrateCoverage:
    def test_structure(self):
        from iv8_rs.environment_toolchain_diagnostics import _substrate_coverage_records

        records = _substrate_coverage_records()
        assert len(records) >= 1
        for r in records:
            assert r.code.startswith("ENV_TOOLCHAIN_SUBSTRATE_COVERAGE")


# ---------------------------------------------------------------------------
# _scaffold_gap_records / _scaffold_gap_item_details
# ---------------------------------------------------------------------------

class TestScaffoldGaps:
    def test_structure(self):
        from iv8_rs.environment_toolchain_diagnostics import _scaffold_gap_records

        records = _scaffold_gap_records()
        assert len(records) >= 1
        for r in records:
            assert r.code.startswith("ENV_TOOLCHAIN_SCAFFOLD_GAP")


# ---------------------------------------------------------------------------
# _profile_coherence_records
# ---------------------------------------------------------------------------

class TestProfileCoherence:
    def test_structure(self):
        from iv8_rs.environment_toolchain_diagnostics import _profile_coherence_records
        from iv8_rs.environment_toolchain_models import ProfileCoherenceGroup

        group = ProfileCoherenceGroup(
            group_id="g1", status="consistent",
            fields={"field": "test"}, sources={"src": "probe"},
            reason="all matched",
        )
        records = _profile_coherence_records([group])
        assert len(records) >= 1
        assert any(r.code == "ENV_TOOLCHAIN_PROFILE_COHERENCE_SUMMARY" for r in records)

    def test_inconsistent_group_generates_warn(self):
        from iv8_rs.environment_toolchain_diagnostics import _profile_coherence_records
        from iv8_rs.environment_toolchain_models import ProfileCoherenceGroup

        group = ProfileCoherenceGroup(
            group_id="g2", status="inconsistent",
            fields={"x": "1"}, sources={"src": "probe"},
            reason="mismatch",
        )
        records = _profile_coherence_records([group])
        assert records[1].severity == "warn"

    def test_unknown_status(self):
        from iv8_rs.environment_toolchain_diagnostics import _profile_coherence_records
        from iv8_rs.environment_toolchain_models import ProfileCoherenceGroup

        group = ProfileCoherenceGroup(
            group_id="g3", status="unknown",
            fields={}, sources={}, reason="unknown",
        )
        records = _profile_coherence_records([group])
        assert records[1].severity == "info"

    def test_invalid_status_raises(self):
        from iv8_rs.environment_toolchain_models import ProfileCoherenceGroup

        with pytest.raises(ValueError, match="invalid profile coherence status"):
            ProfileCoherenceGroup(
                group_id="g1", status="INVALID",
                fields={}, sources={}, reason="bad status",
            )


# ---------------------------------------------------------------------------
# _family_pressure_summary_records
# ---------------------------------------------------------------------------

class TestFamilyPressure:
    def test_structure(self):
        from iv8_rs.environment_toolchain_diagnostics import _family_pressure_summary_records
        from iv8_rs.environment_toolchain_models import FamilyPressure

        fp = FamilyPressure(
            pressure_id="fp1",
            category="missing_api",
            target_family="navigator",
            gap_classes=["missing"],
        )
        records = _family_pressure_summary_records([fp])
        assert len(records) >= 1

    def test_invalid_pressure_category_raises(self):
        from iv8_rs.environment_toolchain_models import FamilyPressure

        with pytest.raises(ValueError, match="invalid pressure category"):
            FamilyPressure(
                pressure_id="fp1", category="INVALID",
                target_family="test", gap_classes=[],
            )

    def test_multiple_pressures_fill_counts(self):
        from iv8_rs.environment_toolchain_diagnostics import _family_pressure_summary_records
        from iv8_rs.environment_toolchain_models import FamilyPressure

        pressures = [
            FamilyPressure(pressure_id="fp1", category="missing_api",
                           target_family="navigator", gap_classes=["a"]),
            FamilyPressure(pressure_id="fp2", category="missing_api",
                           target_family="screen", gap_classes=["b"]),
            FamilyPressure(pressure_id="fp3", category="value_mismatch",
                           target_family="navigator", gap_classes=["c"]),
        ]
        records = _family_pressure_summary_records(pressures)
        details = records[0].details
        assert details["pressures"] == 3
        assert details["category_counts"]["missing_api"] == 2
        assert details["family_counts"]["navigator"] == 2
        assert len(details["entries"]) == 3


# ---------------------------------------------------------------------------
# _pressure_harness_records
# ---------------------------------------------------------------------------

class TestPressureHarness:
    def test_simple(self):
        from iv8_rs.environment_toolchain_diagnostics import _pressure_harness_records
        from iv8_rs.experimental_report import ExperimentalDiagnosticRecord

        report = _make_pressure_report(
            data={"schema_version": "0.1", "sample_id": "s1",
                  "input_kind": "probe", "execution_mode": "replay",
                  "status": "failure", "failure_kind": "timeout",
                  "pressure": {"kind": "cpu"}, "promotion": {"level": "B1"},
                  "writes": ["/tmp/test"]},
            diagnostics=[ExperimentalDiagnosticRecord("TEST", "info", {"k": "v"})],
        )
        records = _pressure_harness_records(report)
        assert len(records) == 2
        assert records[0].code == "ENV_TOOLCHAIN_PRESSURE_HARNESS_SUMMARY"
        assert records[0].details["report"]["status"] == "failure"
        assert records[0].details["report"]["failure_kind"] == "timeout"
        assert records[0].details["report"]["pressure"] == {"kind": "cpu"}
        assert records[1].code == "TEST"

    def test_empty_writes(self):
        from iv8_rs.environment_toolchain_diagnostics import _pressure_harness_records

        report = _make_pressure_report()
        records = _pressure_harness_records(report)
        assert records[0].details["report"]["writes"] == []


# ---------------------------------------------------------------------------
# _pressure_plan_records
# ---------------------------------------------------------------------------

class TestPressurePlan:
    def test_plan_records(self):
        from iv8_rs.environment_toolchain_diagnostics import _pressure_plan_records

        report = _make_pressure_report()
        records = _pressure_plan_records(report)
        assert len(records) == 2
        assert records[0].code == "ENV_TOOLCHAIN_PRESSURE_PLAN_SUMMARY"
        assert records[1].code == "ENV_TOOLCHAIN_PRESSURE_PLAN_ITEM"


# ---------------------------------------------------------------------------
# _native_substrate_review_records / _native_substrate_candidate_areas
# ---------------------------------------------------------------------------

class TestNativeSubstrateReview:
    def test_pressure_mismatch_descriptor_prototype(self):
        from iv8_rs.environment_toolchain_diagnostics import _native_substrate_review_records
        from iv8_rs.environment_toolchain_models import ProfileCoherenceGroup, FamilyPressure

        groups = [
            ProfileCoherenceGroup(group_id="ua_platform", status="consistent",
                                  fields={}, sources={}, reason="ok"),
            ProfileCoherenceGroup(group_id="network_info", status="consistent",
                                  fields={}, sources={}, reason="ok"),
            ProfileCoherenceGroup(group_id="timezone_locale", status="consistent",
                                  fields={}, sources={}, reason="ok"),
        ]
        pressures = [
            FamilyPressure(pressure_id="fp1", category="descriptor_mismatch",
                           target_family="navigator", gap_classes=["desc"]),
        ]
        records = _native_substrate_review_records(groups, pressures)
        assert "descriptor_prototype" in records[0].details["candidate_areas"]
        assert records[0].details["review_status"] == "requires_review"

    def test_inconsistent_ua_platform(self):
        from iv8_rs.environment_toolchain_diagnostics import _native_substrate_review_records
        from iv8_rs.environment_toolchain_models import ProfileCoherenceGroup, FamilyPressure

        groups = [
            ProfileCoherenceGroup(group_id="ua_platform", status="inconsistent",
                                  fields={}, sources={}, reason="mismatch"),
            ProfileCoherenceGroup(group_id="network_info", status="consistent",
                                  fields={}, sources={}, reason="ok"),
            ProfileCoherenceGroup(group_id="timezone_locale", status="consistent",
                                  fields={}, sources={}, reason="ok"),
        ]
        pressures = [
            FamilyPressure(pressure_id="fp1", category="missing_api",
                           target_family="navigator", gap_classes=["x"]),
        ]
        records = _native_substrate_review_records(groups, pressures)
        assert "navigator_ua_data" in records[0].details["candidate_areas"]

    def test_inconsistent_network_info(self):
        from iv8_rs.environment_toolchain_diagnostics import _native_substrate_review_records
        from iv8_rs.environment_toolchain_models import ProfileCoherenceGroup, FamilyPressure

        groups = [
            ProfileCoherenceGroup(group_id="ua_platform", status="consistent",
                                  fields={}, sources={}, reason="ok"),
            ProfileCoherenceGroup(group_id="network_info", status="inconsistent",
                                  fields={}, sources={}, reason="mismatch"),
            ProfileCoherenceGroup(group_id="timezone_locale", status="consistent",
                                  fields={}, sources={}, reason="ok"),
        ]
        pressures = [
            FamilyPressure(pressure_id="fp1", category="missing_api",
                           target_family="navigator", gap_classes=["x"]),
        ]
        records = _native_substrate_review_records(groups, pressures)
        assert "navigator_connection" in records[0].details["candidate_areas"]

    def test_inconsistent_timezone_locale(self):
        from iv8_rs.environment_toolchain_diagnostics import _native_substrate_review_records
        from iv8_rs.environment_toolchain_models import ProfileCoherenceGroup, FamilyPressure

        groups = [
            ProfileCoherenceGroup(group_id="ua_platform", status="consistent",
                                  fields={}, sources={}, reason="ok"),
            ProfileCoherenceGroup(group_id="network_info", status="consistent",
                                  fields={}, sources={}, reason="ok"),
            ProfileCoherenceGroup(group_id="timezone_locale", status="inconsistent",
                                  fields={}, sources={}, reason="mismatch"),
        ]
        pressures = [
            FamilyPressure(pressure_id="fp1", category="prototype_mismatch",
                           target_family="navigator", gap_classes=["proto"]),
        ]
        records = _native_substrate_review_records(groups, pressures)
        assert "timezone_intl" in records[0].details["candidate_areas"]

    def test_no_candidate_areas(self):
        from iv8_rs.environment_toolchain_diagnostics import _native_substrate_review_records
        from iv8_rs.environment_toolchain_models import ProfileCoherenceGroup, FamilyPressure

        groups = [
            ProfileCoherenceGroup(group_id="ua_platform", status="consistent",
                                  fields={}, sources={}, reason="ok"),
            ProfileCoherenceGroup(group_id="network_info", status="consistent",
                                  fields={}, sources={}, reason="ok"),
            ProfileCoherenceGroup(group_id="timezone_locale", status="consistent",
                                  fields={}, sources={}, reason="ok"),
        ]
        pressures = [
            FamilyPressure(pressure_id="fp1", category="missing_api",
                           target_family="navigator", gap_classes=["x"]),
        ]
        records = _native_substrate_review_records(groups, pressures)
        assert records[0].details["candidate_areas"] == []
        assert records[0].severity == "info"
        assert records[0].details["review_status"] == "review_only"
