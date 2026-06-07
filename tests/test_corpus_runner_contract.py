from __future__ import annotations

import copy

import pytest

from iv8_rs import (
    CorpusManifestItem,
    CorpusRunOptions,
    build_corpus_report,
    default_executor,
    load_manifest,
    run_corpus_manifest,
)


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
