"""v0.8.33 Slice 3 -- L3 runtime witness report tests."""
from __future__ import annotations

import json
import sys
from pathlib import Path

import iv8_rs
import pytest

sys.path.insert(0, str(Path(__file__).parent.parent))
from tools.witness_reports import (
    _BROWSER_SURFACE_MATRIX,
    _UNDETECTABLE_CHECKS,
    generate_bcr_dispatch_report,
    generate_browser_surface_report,
    generate_undetectable_report,
)


def test_bcr_dispatch_report_has_expected_shape():
    report = generate_bcr_dispatch_report()
    assert report["schema_version"] == "iv8-bcr-dispatch-report.v0.1"
    assert report["behavior_slots"] == 15
    assert report["writes"] == []
    assert report["evidence_ceiling"] == "diagnostic_only"
    assert report["result"] in ("pass", "review_needed")


def test_bcr_dispatch_report_valid_json():
    json.dumps(generate_bcr_dispatch_report(), sort_keys=True)


def test_bcr_dispatch_report_reads_source():
    """BCR report must perform structural analysis on the actual embedded_v8.rs source."""
    report = generate_bcr_dispatch_report()
    if "error" in report:
        pytest.skip("source file not accessible")
    assert report["active_dispatch_sites"] == 15, (
        f"expected 15 active dispatch sites from source, got {report['active_dispatch_sites']}"
    )
    direct = report["active_direct_install_calls"]
    assert direct >= 0, f"expected non-negative direct count, got {direct}"


def test_browser_surface_report_valid_json():
    report = generate_browser_surface_report()
    assert report["writes"] == []
    assert report["evidence_ceiling"] == "diagnostic_only"
    assert report["total"] > 0
    assert report["total"] == len(report["results"])
    json.dumps(report, sort_keys=True)


def test_browser_surface_report_coverage():
    report = generate_browser_surface_report()
    expected = len(_BROWSER_SURFACE_MATRIX)
    assert report["total"] == expected, (
        f"expected {expected} probes matching _BROWSER_SURFACE_MATRIX, got {report['total']}"
    )
    for entry in report["results"]:
        assert "id" in entry
        assert "expected" in entry
        assert "result" in entry


def test_undetectable_report_valid_json():
    report = generate_undetectable_report()
    assert report["writes"] == []
    assert report["evidence_ceiling"] == "diagnostic_only"
    assert report["total"] == len(report["results"])
    json.dumps(report, sort_keys=True)


def test_undetectable_report_coverage():
    report = generate_undetectable_report()
    expected = len(_UNDETECTABLE_CHECKS)
    assert report["total"] == expected, (
        f"expected {expected} probes matching _UNDETECTABLE_CHECKS, got {report['total']}"
    )


def test_reports_are_deterministic():
    """BCR report must be deterministic for the same source file."""
    first = generate_bcr_dispatch_report()
    second = generate_bcr_dispatch_report()
    # Skip if source file not found
    if "error" in first:
        pytest.skip("source file not accessible")
    assert first == second


_BCR_SYNTHETIC_SOURCE = r"""
pub fn install_browser_surface_with_callbacks(
    &mut self, callbacks: iv8_surface::BehaviorCallbackRegistry,
) -> Result<()> {
    crate::dom::template::install_dom_constructors(
        scope, global, &dom_templates,
    );
    install_behavior_via_bcr(
        scope, global, &callbacks,
        &callbacks.install_event_loop,
        crate::events::binding::install_event_loop_bindings,
    );
    install_behavior_via_bcr(
        scope, global, &callbacks,
        &callbacks.install_timers,
        crate::events::timers::install_timer_globals,
    );
    install_behavior_via_bcr(
        scope, global, &callbacks,
        &callbacks.install_fetch,
        crate::network::fetch::install_fetch,
    );
}
"""

_BCR_FALLBACK_SOURCE = r"""
pub fn install_undetect_shims(&mut self, skip_native_behaviors: bool) {
    if !skip_native_behaviors {
        crate::shims::native_env::install_native_env(scope, global);
        crate::shims::console::install_console(scope, global);
    }
}
"""


def test_bcr_report_with_synthetic_source(tmp_path):
    src = tmp_path / "test_bcr.rs"
    src.write_text(_BCR_SYNTHETIC_SOURCE)
    report = generate_bcr_dispatch_report(src_path=str(src))
    assert report["active_dispatch_sites"] == 3
    assert report["active_direct_install_calls"] == 1
    assert report["result"] == "review_needed"


def test_bcr_report_missing_source_returns_error():
    report = generate_bcr_dispatch_report(src_path="/nonexistent_bcr_source.rs")
    assert report["result"] == "error"
    assert "error" in report
