"""Tests for probe_environment (M31).

Note: Native getters (navigator.*, screen.*) are implemented as V8
ObjectTemplate accessors, which bypass the recording-mode JS Proxy.
Therefore recording-mode tests assert report structure rather than
specific entry contents. VM-instrumentation-mode tests would require
a real VM source (e.g. instrument_source-detected JS)."""

from __future__ import annotations


def test_probe_basic_report_structure():
    """Verify probe_environment returns the expected top-level keys."""
    from iv8_rs import probe_environment

    report = probe_environment(
        js_source="var x = 1 + 1;",
        profile=None,
        random_seed=42,
    )

    assert isinstance(report, dict)
    assert "reads" in report
    assert "calls" in report
    assert "writes" in report
    assert "missing" in report
    assert "errors" in report
    assert "issues" in report
    assert "coverage" in report
    assert "vm_info" in report
    assert "trace_stats" in report


def test_probe_with_profile():
    """Verify loading a profile does not crash."""
    from iv8_rs import probe_environment

    report = probe_environment(
        js_source="var x = 1 + 1;",
        profile="default",
        random_seed=42,
    )

    assert isinstance(report, dict)


def test_probe_coverage_stats_structure():
    """Verify coverage statistics dict has all expected fields."""
    from iv8_rs import probe_environment

    report = probe_environment(
        js_source="var x = 1 + 1;",
        profile=None,
        random_seed=42,
    )

    cov = report["coverage"]
    assert isinstance(cov, dict)
    assert "total_targets" in cov
    assert "configured" in cov
    assert "missing" in cov
    assert "error_count" in cov
    assert "coverage_pct" in cov
    assert 0.0 <= cov["coverage_pct"] <= 100.0


def test_probe_vm_info_present():
    """Verify vm_info key exists (value depends on VM detection)."""
    from iv8_rs import probe_environment

    report = probe_environment(
        js_source="var x = 1 + 1;",
        profile=None,
        random_seed=42,
    )

    assert "vm_info" in report


def test_probe_empty_source():
    """Verify empty source produces a valid report with empty containers."""
    from iv8_rs import probe_environment

    report = probe_environment(
        js_source="",
        profile=None,
        random_seed=42,
    )

    assert isinstance(report["reads"], dict)
    assert isinstance(report["calls"], dict)
    assert isinstance(report["writes"], dict)
    assert isinstance(report["missing"], list)
    assert isinstance(report["errors"], list)
    assert isinstance(report["issues"], list)


def test_probe_trace_stats_present():
    """Verify trace_stats has expected structure when trace exists."""
    from iv8_rs import probe_environment

    report = probe_environment(
        js_source="var x = 1 + 1;",
        profile=None,
        random_seed=42,
    )

    stats = report["trace_stats"]
    if stats is not None:
        assert "total" in stats
        assert "counts_by_type" in stats


def test_probe_with_entry_expr():
    """Verify entry_expr runs without error after main source."""
    from iv8_rs import probe_environment

    report = probe_environment(
        js_source="var result = 0;",
        entry_expr="1 + 1",
        profile=None,
        random_seed=42,
    )

    assert isinstance(report, dict)


def test_probe_with_environment_override():
    """Verify environment parameter is accepted."""
    from iv8_rs import probe_environment

    report = probe_environment(
        js_source="var x = 1 + 1;",
        environment={"screen.width": 1024},
        profile=None,
        random_seed=42,
    )

    assert isinstance(report, dict)
