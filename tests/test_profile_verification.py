"""v0.8.32 Profile Verification Tests.

Integration tests for the convergence checker: profile overrides
-> JSContext -> probe execution -> report verdict.
"""

from __future__ import annotations

from helpers.profile_verification_checker import (
    ConvergenceChecker,
    chrome147_win10_overrides,
)


class TestProfileVerification:
    """Static core profile verification tests."""

    def test_checker_produces_equivalent_verdict(self):
        checker = ConvergenceChecker(chrome147_win10_overrides())
        report = checker.check_static_core()
        assert report.verdict == "equivalent", (
            f"expected equivalent verdict, got {report.verdict}; "
            f"errors: {report.errors}"
        )

    def test_checker_has_no_writes(self):
        checker = ConvergenceChecker(chrome147_win10_overrides())
        report = checker.check_static_core()
        assert len(report.writes) == 0, f"expected zero writes, got {report.writes}"

    def test_checker_detects_material_failure(self):
        expected_overrides = chrome147_win10_overrides()
        runtime_overrides = chrome147_win10_overrides()
        runtime_overrides["navigator.userAgent"] = "evilbot/1.0"
        checker = ConvergenceChecker(
            expected_overrides,
            runtime_environment=runtime_overrides,
        )
        report = checker.check_static_core()
        assert report.verdict == "failed"
        assert report.material_failures > 0
        assert any(
            r.probe_id == "navigator.userAgent" and r.status == "material"
            for r in report.probe_results
        )

    def test_checker_deterministic_rerun(self):
        checker_a = ConvergenceChecker(chrome147_win10_overrides())
        checker_b = ConvergenceChecker(chrome147_win10_overrides())
        report_a = checker_a.check_static_core()
        report_b = checker_b.check_static_core()
        assert report_a.total == report_b.total
        assert report_a.passed == report_b.passed
        assert report_a.verdict == report_b.verdict

    def test_report_to_dict_is_json_serializable(self):
        import json
        checker = ConvergenceChecker(chrome147_win10_overrides())
        report = checker.check_static_core()
        d = report.to_dict()
        serialized = json.dumps(d)
        assert len(serialized) > 0
        assert "equivalent" in serialized

    def test_expected_divergences_present(self):
        checker = ConvergenceChecker(chrome147_win10_overrides())
        report = checker.check_static_core()
        assert report.expected_divergences >= 2, (
            f"expected >=2 divergences (fonts, webrtc), got {report.expected_divergences}"
        )


class TestConvergenceBoundary:
    """Boundary and safety tests."""

    def test_empty_overrides_still_produces_verdict(self):
        checker = ConvergenceChecker({})
        report = checker.check_static_core()
        assert report.verdict in ("equivalent", "partial", "failed", "no_data")

    def test_no_writes_on_empty_overrides(self):
        checker = ConvergenceChecker({})
        report = checker.check_static_core()
        assert len(report.writes) == 0

    def test_report_duration_populated(self):
        checker = ConvergenceChecker(chrome147_win10_overrides())
        report = checker.check_static_core()
        assert report.duration_ms >= 0
