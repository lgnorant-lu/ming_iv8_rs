"""Behavioral tests for environment_toolchain_diagnostics + environment_toolchain_models."""
import pytest

iv8_rs = pytest.importorskip("iv8_rs")


def test_boundary_decision_to_dict():
    from iv8_rs.environment_toolchain_models import BoundaryDecision
    d = BoundaryDecision(decision="blocked", reason="test").to_dict()
    assert d["decision"] == "blocked"


def test_environment_gap_to_dict():
    from iv8_rs.environment_toolchain_models import EnvironmentGap
    gap = EnvironmentGap(target="navigator.missing", gap_class="high",
                         probe_id="p1", category="env",
                         expected="expected_val", actual="actual_val")
    d = gap.to_dict()
    assert d["target"] == "navigator.missing"
    assert d["gap_class"] == "high"


def test_probe_observation_from_probe():
    from iv8_rs.environment_toolchain_models import ProbeObservation
    # Minimal probe object (can be any object with required attrs)
    class FakeProbe:
        probe_id = "navigator.userAgent"
        target = "navigator.userAgent"
        category = "browser"
        expected = "Mozilla"
        gap_class = "exact"
        evidence_ceiling = "diagnostic_only"

    obs = ProbeObservation.from_probe(FakeProbe(), actual="Mozilla", passed=True)
    assert obs.target == "navigator.userAgent"
    assert obs.actual == "Mozilla"
    assert obs.passed


def test_diagnostics_modules_importable():
    import iv8_rs.environment_toolchain_diagnostics
    import iv8_rs.environment_toolchain_models
    assert iv8_rs.environment_toolchain_diagnostics is not None
    assert iv8_rs.environment_toolchain_models is not None
