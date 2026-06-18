from __future__ import annotations

from pathlib import Path

import pytest
from iv8_rs.environment_pressure import (
    ENVIRONMENT_PRESSURE_SCHEMA_VERSION,
    environment_pressure_batch_to_toolchain_diagnostics,
    run_environment_pressure_manifest,
)


def diagnostic(data: list[dict], code: str) -> dict:
    return next(record for record in data if record["code"] == code)


def test_synthetic_pressure_manifest_reports_classified_rate_without_reads():
    batch = run_environment_pressure_manifest({
        "samples": [
            {
                "sample_id": "inline-missing-request",
                "source": "new Request('/synthetic')",
                "message": "ReferenceError: Request is not defined",
            },
            {
                "sample_id": "inline-missing-document",
                "source": "document.createElement('div')",
                "message": "ReferenceError: document is not defined",
            },
            {
                "sample_id": "ref-only-placeholder",
                "source_ref": "synthetic/corpus/ref-only-sample.js",
            },
        ]
    })
    data = batch.to_dict()
    summary = data["summary"]
    manifest = summary["manifest"]
    classified_rate = summary["classified_count"] / summary["total"]

    assert data["schema_version"] == ENVIRONMENT_PRESSURE_SCHEMA_VERSION
    assert data["writes"] == []
    assert summary["total"] == 3
    assert summary["classified_count"] == 3
    assert summary["unclassified_count"] == 0
    assert classified_rate == 1
    assert summary["by_pressure_kind"]["network_surface"] == 1
    assert summary["by_pressure_kind"]["dom_surface"] == 1
    assert manifest == {
        "items": 3,
        "inline_source_count": 2,
        "source_ref_only_count": 1,
        "redacted_source_refs": ["ref-only-sample.js"],
        "review_status": "review_only",
        "evidence_ceiling": "diagnostic_only",
    }
    assert "synthetic/corpus" not in repr(data)


def test_synthetic_pressure_manifest_bridge_diagnostics_are_summary_only():
    batch = run_environment_pressure_manifest([
        {
            "sample_id": "inline-missing-request",
            "source": "new Request('/synthetic')",
            "message": "ReferenceError: Request is not defined",
        },
        {
            "sample_id": "ref-only-placeholder",
            "source_ref": "synthetic/corpus/ref-only-sample.js",
        },
    ])
    records = [
        record.to_dict()
        for record in environment_pressure_batch_to_toolchain_diagnostics(batch)
    ]

    assert [record["code"] for record in records] == [
        "ENV_PRESSURE_BATCH_SUMMARY",
        "ENV_PRESSURE_BATCH_CLASSIFICATION_COUNTS",
        "ENV_PRESSURE_BATCH_MANIFEST_SUMMARY",
    ]
    summary = diagnostic(records, "ENV_PRESSURE_BATCH_SUMMARY")["details"]
    counts = diagnostic(records, "ENV_PRESSURE_BATCH_CLASSIFICATION_COUNTS")["details"]
    manifest = diagnostic(records, "ENV_PRESSURE_BATCH_MANIFEST_SUMMARY")["details"]

    assert summary == {
        "schema_version": ENVIRONMENT_PRESSURE_SCHEMA_VERSION,
        "total": 2,
        "classified_count": 2,
        "unclassified_count": 0,
        "review_status": "review_only",
        "evidence_ceiling": "diagnostic_only",
        "writes": [],
    }
    assert counts["by_pressure_kind"]["network_surface"] == 1
    assert counts["by_pressure_kind"]["input_normalization"] == 1
    assert manifest["redacted_source_refs"] == ["ref-only-sample.js"]
    assert manifest["review_status"] == "review_only"
    assert manifest["evidence_ceiling"] == "diagnostic_only"
    assert "synthetic/corpus" not in repr(records)


def test_pressure_manifest_source_ref_does_not_check_path_existence(tmp_path: Path):
    missing_source = tmp_path / "missing" / "source-ref-only.js"

    batch = run_environment_pressure_manifest({
        "samples": [
            {
                "sample_id": "missing-source-ref",
                "source_ref": str(missing_source),
            }
        ]
    })
    data = batch.to_dict()
    report = data["reports"][0]
    manifest = data["summary"]["manifest"]

    assert not missing_source.exists()
    assert report["status"] == "skipped"
    assert report["writes"] == []
    assert manifest["source_ref_only_count"] == 1
    assert manifest["redacted_source_refs"] == ["source-ref-only.js"]
    assert str(missing_source.parent) not in repr(data)


def test_pressure_manifest_does_not_invoke_corpus_runner(monkeypatch: pytest.MonkeyPatch):
    import iv8_rs.corpus as corpus

    def fail_if_called(*_args, **_kwargs):
        raise AssertionError("pressure manifest must not call corpus runner")

    monkeypatch.setattr(corpus, "load_manifest", fail_if_called)
    monkeypatch.setattr(corpus, "build_corpus_report", fail_if_called)

    batch = run_environment_pressure_manifest({
        "samples": [
            {
                "sample_id": "ref-only-placeholder",
                "source_ref": "synthetic/corpus/ref-only-sample.js",
            }
        ]
    })

    assert batch.to_dict()["summary"]["manifest"]["redacted_source_refs"] == [
        "ref-only-sample.js"
    ]
