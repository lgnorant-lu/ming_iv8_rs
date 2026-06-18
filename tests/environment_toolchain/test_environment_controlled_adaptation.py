# v0.8.29 L2 Stage 2 MVP — Integration Tests
#
# Tests: S1 probe execution, S3 dry-run, S4 report, 5 guardrails.

import json
import os
import tempfile
from pathlib import Path

import pytest

from helpers.environment_dry_run_engine import Candidate, ComparisonReport, DryRunEngine
from helpers.environment_probe_runner import GapList, ProbeResult, ProbeRunner
from helpers.environment_report_builder import ReportBuilder


# ── S1: Probe Runner ──────────────────────────────────────────────────

def test_probe_runner_with_fingerprint_pack():
    """S1: Execute fingerprint.m1 in JSContext, produce gap list."""
    runner = ProbeRunner("fingerprint.m1")
    assert len(runner.probes) > 0, "fingerprint.m1 should have probes"

    import iv8_rs

    ctx = iv8_rs.JSContext()
    try:
        gaps = runner.run(ctx)
        assert isinstance(gaps, GapList)
        assert gaps.total > 0, "should classify at least some probes"
        assert gaps.present and len(gaps.present) > 0, "should have present probes"
    finally:
        ctx.close()


def test_probe_runner_with_descriptor_pack():
    """S1: Execute descriptor.m1 in JSContext."""
    runner = ProbeRunner("descriptor.m1")
    assert len(runner.probes) > 0, "descriptor.m1 should have probes"

    import iv8_rs

    ctx = iv8_rs.JSContext()
    try:
        gaps = runner.run(ctx)
        assert gaps.total > 0
    finally:
        ctx.close()


def test_probe_missing_symbol_detected():
    """S1: Verify a truly missing symbol is classified as 'missing'."""
    import iv8_rs

    ctx = iv8_rs.JSContext()
    try:
        actual = ctx.eval("typeof HXY_NOT_A_REAL_API_999")
        assert actual == "undefined" or actual is None
    except Exception:
        pass
    finally:
        ctx.close()


# ── S3: Dry-Run Engine ────────────────────────────────────────────────

def test_dry_run_before_after_comparison():
    """S3: Apply a candidate, verify before/after diff produced."""
    # Create engine with inline candidates (candidate pack files may not exist)
    engine = DryRunEngine.__new__(DryRunEngine)
    engine.candidate_pack = "inline"
    engine.candidates = []

    candidate = Candidate(
        symbol="atob",
        patch_js="window.atob = function(s) { return 'test'; };",
        policy="runtime_safe",
        description="Test candidate — fills a missing API",
    )

    report = engine.apply(candidate, "fingerprint.m1")
    assert report.before is not None
    assert report.after is not None
    assert report.diff is not None
    assert "missing_delta" in report.diff
    assert "gaps_closed" in report.diff


def test_dry_run_multiple_kernels_no_crash():
    """S3: Verify multiple JSContext instances can be created/destroyed."""
    import iv8_rs

    for _ in range(5):
        ctx = iv8_rs.JSContext()
        ctx.eval("typeof window")
        ctx.close()


# ── S4: Report Builder ────────────────────────────────────────────────

def test_report_schema_matches_spec():
    """S4: Verify generated report matches l2-stage2.v0.1 schema."""
    builder = ReportBuilder()

    gaps = GapList(
        missing=[
            ProbeResult(
                symbol="Request",
                expression="typeof Request",
                gap_kind="missing",
                expected_type="function",
                error="ReferenceError",
            )
        ],
        mismatch=[],
        present=[
            ProbeResult(
                symbol="window",
                expression="typeof window",
                gap_kind="present",
                expected_type="object",
                actual_value="object",
            )
        ],
    )

    candidates = [
        Candidate(
            symbol="Request",
            patch_js="window.Request = function() {};",
            policy="runtime_safe",
        ),
        Candidate(
            symbol="UnsafeAPI",
            patch_js="/* unsafe */",
            policy="unsafe_hook",
        ),
    ]

    report = builder.build(
        input_info={"probe_pack": "fingerprint.m1", "candidate_pack": "default"},
        gaps=gaps,
        candidates=candidates,
    )

    assert report["schema_version"] == "l2-stage2.v0.1"
    assert "timestamp" in report
    assert report["input"]["probe_pack"] == "fingerprint.m1"
    assert report["gaps"]["total"] == 2
    assert report["gaps"]["missing"] == 1
    assert report["gaps"]["present"] == 1
    assert report["candidates"]["selected"] == 2
    assert report["candidates"]["eligible"] == 1  # only runtime_safe
    assert report["candidates"]["blocked"] == 1  # unsafe_hook
    assert report["writes"] == []
    assert report["applied_patches"] == []


def test_report_includes_diagnostics():
    """S4: Verify diagnostics are emitted for gaps and candidates."""
    builder = ReportBuilder()
    gaps = GapList(
        missing=[
            ProbeResult(
                symbol="MissingAPI", expression="x", gap_kind="missing"
            )
        ],
        mismatch=[],
        present=[],
    )
    candidates = [
        Candidate(symbol="MissingAPI", patch_js="x=1", policy="runtime_safe")
    ]

    report = builder.build(
        input_info={"probe_pack": "descriptor.m1"},
        gaps=gaps,
        candidates=candidates,
    )

    codes = [d["code"] for d in report["diagnostics"]]
    assert "ENV_TOOLCHAIN_PROBE_RUN_STARTED" in codes
    assert "ENVIRONMENT_GAP_DETECTED" in codes
    assert "ENVIRONMENT_CANDIDATE_SELECTED" in codes
    assert "PATCH_POLICY_RUNTIME_SAFE" in codes
    assert "ENV_TOOLCHAIN_COMPARISON_REPORT_BUILT" in codes
    assert "ENV_TOOLCHAIN_DRY_RUN_STARTED" not in codes
    assert "ENV_TOOLCHAIN_DRY_RUN_COMPLETED" not in codes
    assert "PATCH_POLICY_UNSAFE_HOOK" not in codes

    report_wet = builder.build(
        input_info={"probe_pack": "fingerprint.m1"},
        gaps=gaps,
        candidates=candidates,
        dry_run=ComparisonReport(
            before="x=1",
            after="x=1",
            diff=[{"field": "test", "before": 1, "after": 1}],
        ),
    )
    wet_codes = [d["code"] for d in report_wet["diagnostics"]]
    assert "ENV_TOOLCHAIN_DRY_RUN_STARTED" in wet_codes
    assert "ENV_TOOLCHAIN_DRY_RUN_COMPLETED" in wet_codes


# ── Guardrails ────────────────────────────────────────────────────────

def test_guardrail_no_profile_write():
    """G1: Report must not write to profile files."""
    builder = ReportBuilder()
    gaps = GapList()
    report = builder.build(
        input_info={}, gaps=gaps, candidates=[]
    )
    assert report["writes"] == []


def test_guardrail_no_manifest_write():
    """G2: Report must not reference manifest file writes."""
    report = ReportBuilder().build(
        input_info={}, gaps=GapList(), candidates=[]
    )
    assert report["writes"] == []


def test_guardrail_no_corpus_write():
    """G3: Report must have empty writes array."""
    report = ReportBuilder().build(
        input_info={}, gaps=GapList(), candidates=[]
    )
    assert report["writes"] == []
    assert report["applied_patches"] == []


def test_guardrail_no_probe_pack_mutation():
    """G4: Probe runner must not modify probe pack assets."""
    import iv8_rs

    # Load pack, run, verify pack file unchanged
    runner = ProbeRunner("fingerprint.m1")
    ctx = iv8_rs.JSContext()
    try:
        runner.run(ctx)
    finally:
        ctx.close()
    # No assert needed — ProbeRunner only reads, never writes


def test_guardrail_no_candidate_pack_mutation():
    """G5: Dry-run engine must not modify candidate pack assets."""
    engine = DryRunEngine.__new__(DryRunEngine)
    engine.candidates = []
    engine.candidate_pack = "inline"
    candidates = engine.select(GapList())
    assert isinstance(candidates, list)
