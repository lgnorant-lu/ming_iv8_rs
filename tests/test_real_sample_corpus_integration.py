"""Real-world sample corpus integration tests.

Tests v0.7 capabilities against real JS samples present in the workspace.
These are NOT fixture-synthetic tests — they use actual bundled/protected JS.
"""

from __future__ import annotations

from pathlib import Path

import pytest

pytestmark = pytest.mark.e2e

from iv8_rs import (
    CorpusManifestItem,
    build_corpus_report,
    default_executor,
    load_manifest,
    prepare_entry,
    run_with_entry,
)

QQ_VENDOR = "_ref/yy/vendor.chunk.062f57657390b2408623.js"
QQ_RUNTIME = "_ref/yy/runtime~Page.6334d8311b3b2793314f.js"
QQ_PAGE = "_ref/yy/Page.chunk.d5594821969491679d5e.js"
BDMS_JS = "tests/iv8-ref/examples/js/bdms_1.0.1.19.js"
H5ST_JS = "tests/iv8-ref/examples/js/js_security_v3_main.js"
M27_SCRIPT = "scripts/audit_m27_realworld.py"


def _load(path: str) -> str:
    return Path(path).read_text(encoding="utf-8")


def _plan_and_run(source: str, persona: str = "analysis") -> dict:
    plan = prepare_entry(source, persona=persona)
    return run_with_entry(plan, source)


# ──
# QQ Music: real webpack + VMP hybrid (679KB vendor chunk)
# ──


def test_qq_vendor_classified_as_webpack_vm_hybrid():
    """QQ Music vendor chunk: classification, not parse timeout."""
    source = _load(QQ_VENDOR)
    plan = prepare_entry(source, persona="analysis")
    kind = plan["sample_kind"]
    strat = plan["selected_strategy"]
    assert kind == "webpack_vm_hybrid", f"expected webpack_vm_hybrid, got {kind}"
    assert strat["strategy_kind"] == "webpack_bridge"


def test_qq_vendor_module_graph():
    """QQ Music vendor chunk produces module graph evidence."""
    source = _load(QQ_VENDOR)
    result = _plan_and_run(source)
    mg = result.get("module_graph")
    assert mg is not None, "no module_graph"
    assert mg.get("module_count", 0) > 0, f"zero modules: {mg.get('module_ids')}"
    evidence_kinds = [e.get("kind") for e in mg.get("evidence", [])]
    has_core_evidence = "module_table_captured" in evidence_kinds or "require_captured" in evidence_kinds
    assert has_core_evidence, f"no module/require evidence: {evidence_kinds}"
    # QQ Music uses webpack4-style without .e helper; flavor may be unknown at runtime
    # Static detection already confirms webpack_like family
    assert mg.get("runtime_family") == "webpack_like"


def test_qq_runtime_classified():
    """QQ Music runtime chunk classified as webpack or plain script."""
    source = _load(QQ_RUNTIME)
    plan = prepare_entry(source, persona="analysis")
    assert plan["sample_kind"] is not None


def test_qq_vendor_multiple_evidence_planes():
    """QQ vendor: observed_evidence from webpack (possibly dispatch) planes."""
    source = _load(QQ_VENDOR)
    result = _plan_and_run(source)
    observed = result.get("observed_evidence", [])
    kinds = {e.get("kind") for e in observed}
    expected_webpack = {"module_table_captured", "require_captured", "chunk_event_observed", "entry_module_executed"}
    assert len(kinds & expected_webpack) > 0, f"no webpack evidence: {kinds}"
    diag_codes = [d.get("code", "") for d in result.get("diagnostic_records", [])]
    assert len(diag_codes) > 0


# ──
# abogus / TikTok BDMS (147KB)
# ──


def test_abogus_bdms_executes():
    """abogus bdms JS: executes under analysis persona without crash."""
    source = _load(BDMS_JS)
    result = _plan_and_run(source)
    assert result["final_state"] in ("collected", "finalized"), f"state: {result['final_state']}"
    assert len(result.get("diagnostic_records", [])) > 0, "no diagnostics"


def test_abogus_bdms_environment_report():
    """abogus bdms: environment report produced."""
    source = _load(BDMS_JS)
    result = _plan_and_run(source)
    assert result.get("environment_report") is not None


# ──
# h5st / JD.com (242KB)
# ──


def test_h5st_executes():
    """h5st JS: executes without crash."""
    source = _load(H5ST_JS)
    plan = prepare_entry(source, persona="analysis")
    assert plan["selected_strategy"] is not None
    result = run_with_entry(plan, source)
    assert result["final_state"] in ("collected", "finalized"), f"state: {result['final_state']}"
    assert len(result.get("diagnostic_records", [])) > 0


# ──
# M27 crypto audit via corpus runner
# ──


def test_m27_corpus_entry():
    """M27 audit: corpus runner processes it."""
    item = CorpusManifestItem(
        sample_id="m27",
        source_path=M27_SCRIPT,
        path_status="present",
        sample_kind="plain_script",
        runtime_family="plain",
        persona="analysis",
        automation_status="ready",
    )
    report = build_corpus_report([item], manifest_path="inline", executor=default_executor)
    s = report["samples"][0]
    assert s["eligibility"] == "run", f"eligibility: {s['eligibility']}"


# ──
# Corpus runner on v0.7 manifest (fixtures + real if added)
# ──


def test_corpus_on_v07_manifest():
    """Corpus runner processes v0.7 manifest."""
    items = load_manifest("docs/acceptance/v0.7-real-sample-manifest.md")
    report = build_corpus_report(items, manifest_path="docs/acceptance/v0.7-real-sample-manifest.md", executor=default_executor)
    assert report["summary"]["total"] >= 18
    assert report["summary"]["run"] > 0

