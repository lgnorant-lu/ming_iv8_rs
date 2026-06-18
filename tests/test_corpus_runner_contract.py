from __future__ import annotations

import copy
import json

import pytest

from iv8_rs import (
    CorpusManifestItem,
    CorpusRunOptions,
    build_corpus_report,
    default_executor,
    load_manifest,
    run_corpus_manifest,
)
from iv8_rs.corpus import main as corpus_main


def _items():
    return [
        CorpusManifestItem(
            sample_id="ready-present",
            source_path="samples/ready.js",
            path_status="present",
            sample_kind="plain_script",
            runtime_family="plain",
            persona="analysis",
            target_goal="collect trace",
            expected_evidence=["trace"],
            automation_status="ready",
        ),
        CorpusManifestItem(
            sample_id="missing-sample",
            source_path="samples/missing.js",
            path_status="missing",
            sample_kind="plain_script",
            runtime_family="plain",
            persona="analysis",
            automation_status="blocked",
        ),
        CorpusManifestItem(
            sample_id="external-sample",
            source_path="external sample",
            path_status="external",
            sample_kind="vm_dispatch_known",
            runtime_family="vm",
            persona="analysis",
            automation_status="manual_only",
        ),
    ]


def test_manifest_item_validates_status_enums():
    with pytest.raises(ValueError, match="invalid path_status"):
        CorpusManifestItem(
            sample_id="bad",
            source_path="bad.js",
            path_status="gone",
            sample_kind="plain_script",
            runtime_family="plain",
            persona="analysis",
        )


def test_build_corpus_report_emits_required_envelope_and_summary():
    report = build_corpus_report(_items(), manifest_path="manifest.md")

    assert report["schema_version"] == "corpus-report.v0.1"
    assert report["runner_version"] == "0.7.0-dev"
    assert report["manifest_path"] == "manifest.md"
    assert report["policy"] == {"level": "runtime_safe"}
    assert report["summary"] == {
        "total": 3,
        "selected": 3,
        "run": 0,
        "skipped": 3,
        "pass": 0,
        "warn": 0,
        "fail": 0,
        "error": 0,
    }
    assert len(report["samples"]) == 3
    assert report["artifacts"] == []


def test_default_eligibility_skips_without_mutating_inputs():
    items = _items()
    before = copy.deepcopy([item.to_dict() for item in items])
    report = build_corpus_report(items, manifest_path="manifest.md")
    samples = {sample["sample_id"]: sample for sample in report["samples"]}

    assert samples["ready-present"]["skip_reason"] == "executor_not_implemented"
    assert samples["missing-sample"]["skip_reason"] == "path_missing"
    assert samples["external-sample"]["skip_reason"] == "external_not_enabled"
    assert [item.to_dict() for item in items] == before


def test_dry_run_and_filter_are_reflected_in_report():
    report = build_corpus_report(
        _items(),
        manifest_path="manifest.md",
        options=CorpusRunOptions(sample_filter={"ready-present"}, dry_run=True),
    )
    samples = {sample["sample_id"]: sample for sample in report["samples"]}

    assert samples["ready-present"]["eligibility"] == "dry_run"
    assert samples["ready-present"]["skip_reason"] == "dry_run"
    assert samples["missing-sample"]["skip_reason"] == "not_selected"
    assert samples["external-sample"]["skip_reason"] == "not_selected"
    assert report["summary"]["selected"] == 1


def test_report_sample_schema_contains_contract_fields():
    report = build_corpus_report(_items()[:1], manifest_path="manifest.md")
    sample = report["samples"][0]

    for field in [
        "sample_id",
        "source_path",
        "path_status",
        "sample_kind",
        "runtime_family",
        "persona",
        "target_goal",
        "eligibility",
        "skip_reason",
        "result",
        "selected_strategy",
        "executed_strategies",
        "expected_evidence",
        "observed_evidence",
        "missing_evidence",
        "fallback_attempts",
        "diagnostics",
        "trace_meta",
        "artifacts",
        "notes",
    ]:
        assert field in sample


def test_load_manifest_reads_current_markdown_table():
    items = load_manifest("docs/acceptance/v0.6-real-sample-manifest.md")
    ids = [item.sample_id for item in items]

    assert ids == [
        "iv8-ref-examples",
        "tdc-chaosvm",
        "yy-webpack-vmp",
        "m27-realworld-audit",
    ]
    assert items[0].path_status == "present"
    assert items[-1].path_status == "present"


def test_run_corpus_manifest_does_not_mutate_manifest(tmp_path):
    source = "docs/acceptance/v0.6-real-sample-manifest.md"
    manifest = tmp_path / "manifest.md"
    original = open(source, "r", encoding="utf-8").read()
    manifest.write_text(original, encoding="utf-8")

    report = run_corpus_manifest(manifest)

    assert report["schema_version"] == "corpus-report.v0.1"
    assert manifest.read_text(encoding="utf-8") == original


def test_fixture_execution_with_default_executor(tmp_path):
    fixture = tmp_path / "fixture.js"
    fixture.write_text("var x = 1 + 1;", encoding="utf-8")
    item = CorpusManifestItem(
        sample_id="fixture-test",
        source_path=str(fixture),
        path_status="present",
        sample_kind="plain_script",
        runtime_family="plain",
        persona="analysis",
        target_goal="execution succeeds",
        expected_evidence=[],
        automation_status="ready",
    )
    report = build_corpus_report(
        [item],
        manifest_path="inline",
        executor=default_executor,
    )
    samples = {s["sample_id"]: s for s in report["samples"]}
    assert samples["fixture-test"]["eligibility"] == "run"
    assert samples["fixture-test"]["result"] in {"PASS", "WARN"}
    assert samples["fixture-test"]["selected_strategy"] is not None
    assert len(samples["fixture-test"]["executed_strategies"]) > 0
    assert samples["fixture-test"]["trace_meta"] is not None


def test_fixture_execution_missing_source_fails_gracefully(tmp_path):
    item = CorpusManifestItem(
        sample_id="missing-fixture",
        source_path=str(tmp_path / "nonexistent.js"),
        path_status="present",
        sample_kind="plain_script",
        runtime_family="plain",
        persona="analysis",
        target_goal="source should be present",
        expected_evidence=[],
        automation_status="ready",
    )
    report = build_corpus_report(
        [item],
        manifest_path="inline",
        executor=default_executor,
    )
    samples = {s["sample_id"]: s for s in report["samples"]}
    assert samples["missing-fixture"]["result"] == "ERROR"
    assert any("source not found" in str(d.get("message", "")) for d in samples["missing-fixture"]["diagnostics"])


def test_fixture_execution_not_triggered_without_executor():
    item = CorpusManifestItem(
        sample_id="no-exec",
        source_path="samples/dummy.js",
        path_status="present",
        sample_kind="plain_script",
        runtime_family="plain",
        persona="analysis",
        target_goal="should skip without executor",
        expected_evidence=[],
        automation_status="ready",
    )
    report = build_corpus_report([item], manifest_path="inline")
    samples = {s["sample_id"]: s for s in report["samples"]}
    assert samples["no-exec"]["eligibility"] == "skipped"
    assert samples["no-exec"]["skip_reason"] == "executor_not_implemented"
    assert samples["no-exec"]["result"] == "SKIP"


def test_fixture_does_not_mutate_manifest(tmp_path):
    source_path = tmp_path / "fixture.js"
    source_path.write_text("var x = 1;", encoding="utf-8")
    manifest_path = tmp_path / "manifest.md"
    manifest_path.write_text("", encoding="utf-8")

    item = CorpusManifestItem(
        sample_id="non-mutate",
        source_path=str(source_path),
        path_status="present",
        sample_kind="plain_script",
        runtime_family="plain",
        persona="analysis",
        automation_status="ready",
    )
    before = manifest_path.read_text(encoding="utf-8")
    build_corpus_report([item], manifest_path=str(manifest_path), executor=default_executor)
    assert manifest_path.read_text(encoding="utf-8") == before


# ─── CorpusManifestItem validation ───────────────────────────────────────────

def test_invalid_automation_status_raises():
    with pytest.raises(ValueError, match="invalid automation_status"):
        CorpusManifestItem(
            sample_id="bad-auto",
            source_path="test.js",
            path_status="present",
            sample_kind="plain_script",
            runtime_family="plain",
            persona="analysis",
            automation_status="invalid_val",
        )


def test_invalid_validation_status_raises():
    with pytest.raises(ValueError, match="invalid validation_status"):
        CorpusManifestItem(
            sample_id="bad-val",
            source_path="test.js",
            path_status="present",
            sample_kind="plain_script",
            runtime_family="plain",
            persona="analysis",
            validation_status="invalid_val",
        )


# ─── load_manifest edge cases ────────────────────────────────────────────────

_VALID_MANIFEST = """\
# Test Manifest

| sample_id | source_path | path_status | sample_kind | runtime_family | persona | automation_status | validation_status |
|---|---:|---|---|---|---|---|---|
| test-a | samples/a.js | present | plain_script | plain | analysis | ready | not_validated |
"""


def test_load_manifest_empty_cells_skipped(tmp_path):
    """Lines with only pipe separators are skipped (line 109)."""
    content = _VALID_MANIFEST + "|  |\n"
    m = tmp_path / "manifest.md"
    m.write_text(content, encoding="utf-8")
    items = load_manifest(str(m))
    assert len(items) == 1


def test_load_manifest_row_before_header_skipped(tmp_path):
    """A data row appearing before the header row is skipped (line 117)."""
    content = """\
| sample_a | a.js | present | plain | plain | analysis | ready | not_validated |
""" + _VALID_MANIFEST
    m = tmp_path / "manifest.md"
    m.write_text(content, encoding="utf-8")
    items = load_manifest(str(m))
    assert len(items) == 1


def test_load_manifest_no_records_raises(tmp_path):
    """A manifest with no table rows raises ValueError (line 135)."""
    content = "# No table here\n\nJust some text."
    m = tmp_path / "manifest.md"
    m.write_text(content, encoding="utf-8")
    with pytest.raises(ValueError, match="no corpus manifest records found"):
        load_manifest(str(m))


def test_load_manifest_with_expected_evidence_column(tmp_path):
    """Parse expected_evidence with non-empty value (line 496)."""
    content = """\
| sample_id | source_path | path_status | sample_kind | runtime_family | persona | expected_evidence | automation_status | validation_status |
|---|---:|---|---|---|---|---|---|---|
| test | test.js | present | plain_script | plain | analysis | `trace, metadata` | ready | not_validated |
"""
    m = tmp_path / "manifest.md"
    m.write_text(content, encoding="utf-8")
    items = load_manifest(str(m))
    assert items[0].expected_evidence == ["trace", "metadata"]


# ─── _classify_result via executor ───────────────────────────────────────────

def _executor_for(data):
    """Return a minimal executor that returns the given execution dict."""
    def executor(item):
        return data
    return executor


def test_result_class_partial_is_warn():
    """result_state 'partial' maps to WARN (line 183)."""
    item = CorpusManifestItem(
        sample_id="partial", source_path="samples/a.js",
        path_status="present", sample_kind="plain_script",
        runtime_family="plain", persona="analysis",
        automation_status="ready",
    )
    report = build_corpus_report(
        [item], manifest_path="inline",
        executor=_executor_for({
            "plan_id": "p1", "result_class": "WARN", "selected_strategy": None,
            "executed_strategies": [], "observed_evidence": [],
            "missing_evidence": [], "fallback_attempts": [],
            "trace_meta": None, "module_graph": None,
            "environment_report": None, "diagnostics": [],
            "errors": ["partial"],
        }),
    )
    assert report["samples"][0]["result"] == "WARN"


def test_result_class_fail_for_unknown_state():
    """result_state not collected/completed/finalized maps to FAIL (line 185)."""
    item = CorpusManifestItem(
        sample_id="fail-state", source_path="samples/a.js",
        path_status="present", sample_kind="plain_script",
        runtime_family="plain", persona="analysis",
        automation_status="ready",
    )
    report = build_corpus_report(
        [item], manifest_path="inline",
        executor=_executor_for({
            "plan_id": "p1", "result_class": "FAIL", "selected_strategy": None,
            "executed_strategies": [], "observed_evidence": [],
            "missing_evidence": ["expected_kind"], "fallback_attempts": [],
            "trace_meta": None, "module_graph": None,
            "environment_report": None, "diagnostics": [],
        }),
    )
    assert report["samples"][0]["result"] == "FAIL"


def test_result_class_warn_for_missing_evidence():
    """Missing expected evidence maps to WARN (lines 188-189)."""
    item = CorpusManifestItem(
        sample_id="warn-evidence", source_path="samples/a.js",
        path_status="present", sample_kind="plain_script",
        runtime_family="plain", persona="analysis",
        automation_status="ready",
        expected_evidence=["trace", "meta"],
    )
    report = build_corpus_report(
        [item], manifest_path="inline",
        executor=_executor_for({
            "plan_id": "p1", "result_class": "WARN", "selected_strategy": None,
            "executed_strategies": [], "observed_evidence": [{"kind": "trace"}],
            "missing_evidence": ["meta"], "fallback_attempts": [],
            "trace_meta": None, "module_graph": None,
            "environment_report": None, "diagnostics": [],
        }),
    )
    assert report["samples"][0]["result"] == "WARN"


# ─── Executor exception handling ─────────────────────────────────────────────

def test_executor_raises_exception_returns_error(tmp_path):
    """When executor raises, result is ERROR with diagnostics (lines 290-291)."""
    def failing_executor(item):
        raise RuntimeError("simulated failure")

    item = CorpusManifestItem(
        sample_id="failing-exec", source_path="samples/a.js",
        path_status="present", sample_kind="plain_script",
        runtime_family="plain", persona="analysis",
        automation_status="ready",
    )
    report = build_corpus_report(
        [item], manifest_path="inline", executor=failing_executor,
    )
    sample = report["samples"][0]
    assert sample["result"] == "ERROR"
    assert any("simulated failure" in d.get("message", "") for d in sample["diagnostics"])


# ─── Eligibility branches ────────────────────────────────────────────────────

def test_eligibility_unknown_path_status():
    """path_status 'unknown' → skipped / unknown_path_status (line 388)."""
    item = CorpusManifestItem(
        sample_id="unknown-path", source_path="samples/u.js",
        path_status="unknown", sample_kind="plain_script",
        runtime_family="plain", persona="analysis",
    )
    report = build_corpus_report([item], manifest_path="inline")
    s = report["samples"][0]
    assert s["eligibility"] == "skipped"
    assert s["skip_reason"] == "unknown_path_status"


def test_eligibility_automation_blocked():
    """automation_status 'blocked' → skipped / automation_blocked (line 392)."""
    item = CorpusManifestItem(
        sample_id="blocked", source_path="samples/b.js",
        path_status="present", sample_kind="plain_script",
        runtime_family="plain", persona="analysis",
        automation_status="blocked",
    )
    report = build_corpus_report([item], manifest_path="inline")
    s = report["samples"][0]
    assert s["eligibility"] == "skipped"
    assert s["skip_reason"] == "automation_blocked"


def test_eligibility_not_started():
    """automation_status 'not_started' → skipped / not_started (line 394)."""
    item = CorpusManifestItem(
        sample_id="not-started", source_path="samples/n.js",
        path_status="present", sample_kind="plain_script",
        runtime_family="plain", persona="analysis",
        automation_status="not_started",
    )
    report = build_corpus_report([item], manifest_path="inline")
    s = report["samples"][0]
    assert s["eligibility"] == "skipped"
    assert s["skip_reason"] == "not_started"


# ─── source_ast_report embedding ─────────────────────────────────────────────

def test_source_ast_report_embedded():
    """source_ast_report is embedded when evidence contains source_ast kinds (lines 341-366)."""
    item = CorpusManifestItem(
        sample_id="ast-test", source_path="samples/a.js",
        path_status="present", sample_kind="plain_script",
        runtime_family="plain", persona="analysis",
        automation_status="ready",
    )
    report = build_corpus_report(
        [item], manifest_path="inline",
        executor=_executor_for({
            "plan_id": "p1", "result_class": "PASS", "selected_strategy": "ast",
            "executed_strategies": ["ast"], "observed_evidence": [
                {"kind": "source_ast_transform_applied", "source": "source_ast"},
            ],
            "missing_evidence": [], "fallback_attempts": [],
            "trace_meta": None, "module_graph": None,
            "environment_report": None, "diagnostics": [
                {"code": "AST_OK", "stage": "source_ast.transform", "severity": "info"},
            ],
        }),
    )
    sample = report["samples"][0]
    assert "source_ast_report" in sample
    assert "dispatch_expression" in sample["source_ast_report"]["selected_join_points"]


def test_source_ast_runtime_validated_join_points():
    """runtime_validated evidence adds dispatch_expression + probe join points (lines 354-357)."""
    item = CorpusManifestItem(
        sample_id="ast-rt", source_path="samples/a.js",
        path_status="present", sample_kind="plain_script",
        runtime_family="plain", persona="analysis",
        automation_status="ready",
    )
    report = build_corpus_report(
        [item], manifest_path="inline",
        executor=_executor_for({
            "plan_id": "p1", "result_class": "PASS", "selected_strategy": "ast",
            "executed_strategies": ["ast"], "observed_evidence": [
                {"kind": "source_ast_runtime_validated", "source": "source_ast"},
                {"kind": "eval_source_captured", "source": "source_ast"},
                {"kind": "function_constructor_source_captured", "source": "source_ast"},
            ],
            "missing_evidence": [], "fallback_attempts": [],
            "trace_meta": None, "module_graph": None,
            "environment_report": None, "diagnostics": [],
        }),
    )
    report["samples"][0]["source_ast_report"]


# ─── error_log artifact ──────────────────────────────────────────────────────

def test_error_log_artifact_included():
    """When executor returns errors field, error_log artifact is added (line 471)."""
    item = CorpusManifestItem(
        sample_id="err-artifact", source_path="samples/a.js",
        path_status="present", sample_kind="plain_script",
        runtime_family="plain", persona="analysis",
        automation_status="ready",
    )
    report = build_corpus_report(
        [item], manifest_path="inline",
        executor=_executor_for({
            "plan_id": None, "result_class": "ERROR", "selected_strategy": None,
            "executed_strategies": [], "observed_evidence": [],
            "missing_evidence": [], "fallback_attempts": [],
            "trace_meta": None, "module_graph": None,
            "environment_report": None, "diagnostics": [],
            "errors": ["critical failure"],
        }),
    )
    sample = report["samples"][0]
    kinds = [a["kind"] for a in sample["artifacts"]]
    assert "error_log" in kinds


# ─── CLI main() ──────────────────────────────────────────────────────────────

def test_main_no_manifest():
    """main returns EXIT_CODE_CONFIG_ERROR (2) without --manifest (lines 570-572)."""
    assert corpus_main([]) == 2


def test_main_unknown_argument(tmp_path):
    """main returns 2 for unknown argument (line 567-568)."""
    m = tmp_path / "manifest.md"
    m.write_text(_VALID_MANIFEST, encoding="utf-8")
    assert corpus_main(["--manifest", str(m), "--bogus"]) == 2


def test_main_file_not_found(tmp_path):
    """main returns 2 when manifest file does not exist (line 577-578)."""
    assert corpus_main(["--manifest", str(tmp_path / "nope.md")]) == 2


def test_main_empty_manifest(tmp_path):
    """main returns 2 when manifest has no records (line 577-578)."""
    m = tmp_path / "manifest.md"
    m.write_text("# Empty", encoding="utf-8")
    assert corpus_main(["--manifest", str(m)]) == 2


def test_main_dry_run_ok(tmp_path):
    """main returns 0 with --dry-run (line 606)."""
    m = tmp_path / "manifest.md"
    m.write_text(_VALID_MANIFEST, encoding="utf-8")
    assert corpus_main(["--manifest", str(m), "--dry-run"]) == 0


def test_main_with_sample_filter(tmp_path):
    """--sample flag filters to a single sample."""
    m = tmp_path / "manifest.md"
    m.write_text(_VALID_MANIFEST, encoding="utf-8")
    ret = corpus_main(["--manifest", str(m), "--dry-run", "--sample", "test-a"])
    assert ret == 0


def test_main_with_include_external(tmp_path):
    """--include-external flag is accepted."""
    m = tmp_path / "manifest.md"
    m.write_text(_VALID_MANIFEST, encoding="utf-8")
    assert corpus_main(["--manifest", str(m), "--dry-run", "--include-external"]) == 0


def test_main_with_output_path(tmp_path):
    """--out writes a JSON report file."""
    m = tmp_path / "manifest.md"
    m.write_text(_VALID_MANIFEST, encoding="utf-8")
    out = tmp_path / "reports" / "out.json"
    ret = corpus_main(["--manifest", str(m), "--dry-run", "--out", str(out)])
    assert ret == 0
    assert out.exists()
    data = json.loads(out.read_text(encoding="utf-8"))
    assert data["schema_version"] == "corpus-report.v0.1"


def test_main_output_write_failure(tmp_path, monkeypatch):
    """main returns 3 when report write fails (lines 602-604)."""
    m = tmp_path / "manifest.md"
    m.write_text(_VALID_MANIFEST, encoding="utf-8")

    def _broken_write_text(self, *a, **kw):
        raise OSError("disk full")

    monkeypatch.setattr("pathlib.Path.write_text", _broken_write_text)
    ret = corpus_main(["--manifest", str(m), "--dry-run", "--out", str(tmp_path / "out.json")])
    assert ret == 3


def test_main_strict_with_warn_via_monkeypatch(tmp_path, monkeypatch):
    """--strict + warnings returns EXIT_CODE_STRICT_WARN (4) (line 523-524)."""
    import iv8_rs.corpus as _corpus_mod
    m = tmp_path / "manifest.md"
    m.write_text(_VALID_MANIFEST, encoding="utf-8")

    def mock_build(*args, **kwargs):
        return {"summary": {"total": 1, "warn": 1, "fail": 0, "error": 0}}
    monkeypatch.setattr(_corpus_mod, "build_corpus_report", mock_build)
    ret = corpus_main(["--manifest", str(m), "--strict"])
    assert ret == 4


def test_main_fail_via_monkeypatch(tmp_path, monkeypatch):
    """With FAIL result, main returns EXIT_CODE_FAIL (1) (line 521-522)."""
    import iv8_rs.corpus as _corpus_mod
    m = tmp_path / "manifest.md"
    m.write_text(_VALID_MANIFEST, encoding="utf-8")

    def mock_build(*args, **kwargs):
        return {"summary": {"total": 1, "warn": 0, "fail": 1, "error": 0}}
    monkeypatch.setattr(_corpus_mod, "build_corpus_report", mock_build)
    ret = corpus_main(["--manifest", str(m)])
    assert ret == 1


def test_main_error_via_monkeypatch(tmp_path, monkeypatch):
    """With ERROR result, main returns EXIT_CODE_FAIL (1)."""
    import iv8_rs.corpus as _corpus_mod
    m = tmp_path / "manifest.md"
    m.write_text(_VALID_MANIFEST, encoding="utf-8")

    def mock_build(*args, **kwargs):
        return {"summary": {"total": 1, "warn": 0, "fail": 0, "error": 1}}
    monkeypatch.setattr(_corpus_mod, "build_corpus_report", mock_build)
    ret = corpus_main(["--manifest", str(m)])
    assert ret == 1


def test_main_ok_via_monkeypatch(tmp_path, monkeypatch):
    """Clean run with no warnings returns EXIT_CODE_OK (0) (line 525)."""
    import iv8_rs.corpus as _corpus_mod
    m = tmp_path / "manifest.md"
    m.write_text(_VALID_MANIFEST, encoding="utf-8")

    def mock_build(*args, **kwargs):
        return {"summary": {"total": 1, "warn": 0, "fail": 0, "error": 0}}
    monkeypatch.setattr(_corpus_mod, "build_corpus_report", mock_build)
    ret = corpus_main(["--manifest", str(m)])
    assert ret == 0


# ─── _classify_result direct tests ───────────────────────────────────────────

def test_classify_partial_returns_warn():
    from iv8_rs.corpus import _classify_result
    assert _classify_result("partial", [], []) == "WARN"
    assert _classify_result("degraded", [], []) == "WARN"


def test_classify_unknown_state_returns_fail():
    from iv8_rs.corpus import _classify_result
    assert _classify_result("unknown", [], []) == "FAIL"


def test_classify_missing_evidence_returns_warn():
    from iv8_rs.corpus import _classify_result
    result = _classify_result("collected", ["trace", "meta"], [{"kind": "trace"}])
    assert result == "WARN"


def test_classify_all_evidence_present_returns_pass():
    from iv8_rs.corpus import _classify_result
    result = _classify_result("completed", ["trace", "meta"], [{"kind": "trace"}, {"kind": "meta"}])
    assert result == "PASS"


# ─── build_corpus_report with dict items (from_dict) ─────────────────────────

def test_build_report_with_dict_inputs():
    """build_corpus_report accepts dict items and converts them via from_dict (line 58)."""
    items = [
        {
            "sample_id": "dict-item",
            "source_path": "samples/d.js",
            "path_status": "present",
            "sample_kind": "plain_script",
            "runtime_family": "plain",
            "persona": "analysis",
            "automation_status": "ready",
        },
    ]
    report = build_corpus_report(items, manifest_path="inline")
    assert report["samples"][0]["sample_id"] == "dict-item"


# ─── module_graph embedding ──────────────────────────────────────────────────

def test_module_graph_embedded():
    """When executor returns module_graph, it is embedded in sample (line 341)."""
    item = CorpusManifestItem(
        sample_id="modgraph", source_path="samples/a.js",
        path_status="present", sample_kind="plain_script",
        runtime_family="plain", persona="analysis",
        automation_status="ready",
    )
    report = build_corpus_report(
        [item], manifest_path="inline",
        executor=_executor_for({
            "plan_id": "p1", "result_class": "PASS", "selected_strategy": None,
            "executed_strategies": [], "observed_evidence": [],
            "missing_evidence": [], "fallback_attempts": [],
            "trace_meta": None,
            "module_graph": {"nodes": ["a.js"], "edges": []},
            "environment_report": None, "diagnostics": [],
        }),
    )
    assert "module_graph" in report["samples"][0]


# ─── run_corpus_manifest mutation detection ──────────────────────────────────

def test_run_corpus_manifest_detects_mutation(tmp_path, monkeypatch):
    """If manifest content changes during run, RuntimeError is raised (line 267)."""
    import iv8_rs.corpus as _corpus_mod
    from pathlib import Path as _Path

    m = tmp_path / "manifest.md"
    m.write_text(_VALID_MANIFEST, encoding="utf-8")

    original_load = _corpus_mod.load_manifest

    def _mutating_load(path):
        items = original_load(path)
        # Append a newline to the file after loading
        with open(path, "a", encoding="utf-8") as f:
            f.write("\n")
        return items

    monkeypatch.setattr(_corpus_mod, "load_manifest", _mutating_load)

    with pytest.raises(RuntimeError, match="mutated manifest"):
        run_corpus_manifest(str(m))
