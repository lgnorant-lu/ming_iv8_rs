from __future__ import annotations

from pathlib import Path

import iv8_rs
import pytest
from iv8_rs.environment_pressure import (
    ENVIRONMENT_PRESSURE_SCHEMA_VERSION,
    EnvironmentPressureReport,
    PressureManifestItem,
    PressureSample,
    PressureSignal,
    PromotionDecision,
    build_pressure_report,
    classify_failure_kind,
    classify_input_kind,
    default_execution_mode,
    environment_pressure_batch_to_toolchain_diagnostics,
    pressure_batch_diagnostics,
    pressure_from_failure,
    pressure_report_from_dict,
    pressure_report_to_dict,
    promotion_for_pressure,
    run_environment_pressure_manifest,
    run_environment_pressure_samples,
)
from iv8_rs.environment_toolchain import toolchain_report_from_dict, toolchain_report_to_dict
from iv8_rs.environment_toolchain_runtime import run_environment_toolchain
from iv8_rs.experimental_report import (
    EXPERIMENTAL_SCHEMA_VERSIONS,
    ExperimentalDiagnosticRecord,
    ExperimentalEvidenceRecord,
    experimental_report_roundtrip,
)


def sample_pressure_report() -> dict:
    return {
        "schema_version": ENVIRONMENT_PRESSURE_SCHEMA_VERSION,
        "sample_id": "sample_001",
        "input_kind": "direct_js",
        "execution_mode": "browser_like_global",
        "status": "failed",
        "failure_kind": "missing_global_symbol",
        "pressure": {
            "pressure_kind": "web_api_surface",
            "symbol": "Request",
            "access_pattern": "constructor_reference",
            "behavior_depth": "presence_only",
        },
        "promotion": {
            "level": "candidate_pack",
            "reason": "single sample with low behavior depth",
            "evidence_ceiling": "diagnostic_only",
            "review_status": "review_only",
        },
        "evidence": [
            {
                "kind": "environment_pressure_observed",
                "strength": "diagnostic_only",
            }
        ],
        "diagnostics": [
            {
                "code": "ENV_PRESSURE_OBSERVED",
                "severity": "info",
                "details": {"pressure_kind": "web_api_surface"},
            }
        ],
        "writes": [],
    }


def test_environment_pressure_typed_roundtrip():
    data = sample_pressure_report()
    report = pressure_report_from_dict(data)
    roundtrip = pressure_report_to_dict(report)

    assert roundtrip == data


def test_environment_pressure_dataclass_construction_is_no_write():
    report = EnvironmentPressureReport(
        schema_version=ENVIRONMENT_PRESSURE_SCHEMA_VERSION,
        sample_id="sample_002",
        input_kind="html_document",
        execution_mode="script_tag_bootstrap",
        status="skipped",
        failure_kind="missing_page_bootstrap",
        pressure=PressureSignal(
            pressure_kind="page_lifecycle",
            details={"input_kind": "html_document"},
        ),
        promotion=PromotionDecision(
            level="fixture_only",
            reason="HTML input requires explicit page harness, not substrate promotion",
        ),
        evidence=[ExperimentalEvidenceRecord("environment_pressure_classified", "diagnostic_only")],
        diagnostics=[
            ExperimentalDiagnosticRecord(
                "ENV_PRESSURE_FIXTURE_BOUNDARY",
                "info",
                {"promotion_level": "fixture_only"},
            )
        ],
    )

    data = report.to_dict()
    assert data["writes"] == []
    assert data["promotion"]["level"] == "fixture_only"
    assert data["promotion"]["evidence_ceiling"] == "diagnostic_only"
    assert data["promotion"]["review_status"] == "review_only"


@pytest.mark.parametrize(
    ("field", "value"),
    [
        ("input_kind", "site_specific_page"),
        ("execution_mode", "full_chrome_replay"),
        ("status", "passed_by_patch"),
        ("failure_kind", "cookie_chain_required"),
    ],
)
def test_environment_pressure_rejects_unknown_taxonomy_values(field: str, value: str):
    data = sample_pressure_report()
    data[field] = value

    with pytest.raises(ValueError, match=f"invalid {field if field != 'status' else 'status'}"):
        pressure_report_from_dict(data)


def test_environment_pressure_rejects_unknown_pressure_kind():
    data = sample_pressure_report()
    data["pressure"]["pressure_kind"] = "site_bypass_flow"

    with pytest.raises(ValueError, match="invalid pressure_kind"):
        pressure_report_from_dict(data)


def test_environment_pressure_rejects_non_diagnostic_promotion():
    data = sample_pressure_report()
    data["promotion"]["evidence_ceiling"] = "strong"

    with pytest.raises(ValueError, match="diagnostic_only"):
        pressure_report_from_dict(data)


def test_environment_pressure_rejects_writes():
    data = sample_pressure_report()
    data["writes"] = [{"path": "profiles/default.json"}]

    with pytest.raises(ValueError, match="no-write"):
        pressure_report_from_dict(data)


def test_environment_pressure_schema_is_registered_for_experimental_envelope():
    data = sample_pressure_report()

    assert ENVIRONMENT_PRESSURE_SCHEMA_VERSION in EXPERIMENTAL_SCHEMA_VERSIONS
    assert (
        experimental_report_roundtrip(data)["schema_version"]
        == ENVIRONMENT_PRESSURE_SCHEMA_VERSION
    )


def test_source_ref_only_manifest_item_remains_skipped_diagnostic_only():
    item = PressureManifestItem(
        sample_id="missing-source-ref",
        source_ref="does/not/exist/sample.js",
    )
    sample = item.to_sample()
    report = build_pressure_report(
        sample.sample_id,
        sample.source,
        message=sample.message,
        status=sample.status,
        sample_count=sample.sample_count,
    )
    data = report.to_dict()

    assert sample.source is None
    assert sample.status == "skipped"
    assert data["status"] == "skipped"
    assert data["input_kind"] == "empty_or_invalid"
    assert data["promotion"]["evidence_ceiling"] == "diagnostic_only"
    assert data["promotion"]["review_status"] == "review_only"
    assert data["writes"] == []


def test_source_ref_redaction_uses_basename_without_path_resolution():
    item = PressureManifestItem(
        sample_id="windows-source-ref",
        source_ref=r"C:\private\samples\source-ref-only.js",
    )

    assert item.redacted_source_ref() == "source-ref-only.js"


def test_environment_pressure_public_exports_are_current():
    assert iv8_rs.ENVIRONMENT_PRESSURE_SCHEMA_VERSION == ENVIRONMENT_PRESSURE_SCHEMA_VERSION
    assert callable(iv8_rs.classify_input_kind)
    assert callable(iv8_rs.build_pressure_report)
    assert callable(iv8_rs.classify_failure_kind)
    assert callable(iv8_rs.pressure_from_failure)
    assert callable(iv8_rs.pressure_batch_diagnostics)
    assert callable(iv8_rs.environment_pressure_batch_to_toolchain_diagnostics)
    assert callable(iv8_rs.promotion_for_pressure)
    assert callable(iv8_rs.run_environment_pressure_manifest)
    assert callable(iv8_rs.run_environment_pressure_samples)
    assert callable(iv8_rs.pressure_report_from_dict)
    assert callable(iv8_rs.pressure_report_to_dict)

    stub_text = Path(iv8_rs.__file__).with_name("__init__.pyi").read_text(encoding="utf-8")
    assert "EnvironmentPressureReport" in stub_text
    assert "build_pressure_report" in stub_text
    assert "classify_input_kind" in stub_text
    assert "pressure_batch_diagnostics" in stub_text
    assert "environment_pressure_batch_to_toolchain_diagnostics" in stub_text
    assert "run_environment_pressure_manifest" in stub_text
    assert "run_environment_pressure_samples" in stub_text
    assert "pressure_report_from_dict" in stub_text


@pytest.mark.parametrize(
    ("source", "expected"),
    [
        ("", "empty_or_invalid"),
        ("   ", "empty_or_invalid"),
        ("<html><body></body></html>", "html_document"),
        ("<script>window.x = 1</script>", "script_tag_page"),
        ('{"url":"https://example.invalid"}', "json_payload"),
        ('{"entries": []}', "network_trace"),
        ("var x = {a: 1};", "direct_js"),
        (b"var y = 1;", "direct_js"),
    ],
)
def test_classify_input_kind_is_shape_only(source: str | bytes, expected: str):
    assert classify_input_kind(source) == expected


@pytest.mark.parametrize(
    ("input_kind", "expected"),
    [
        ("direct_js", "browser_like_global"),
        ("script_tag_page", "script_tag_bootstrap"),
        ("html_document", "page_dom_fixture"),
        ("page_snapshot", "page_dom_fixture"),
        ("network_trace", "network_stubbed"),
        ("json_payload", "bare_v8"),
    ],
)
def test_default_execution_mode_preserves_harness_boundary(input_kind: str, expected: str):
    assert default_execution_mode(input_kind) == expected


@pytest.mark.parametrize(
    ("message", "input_kind", "expected"),
    [
        ("SyntaxError: Unexpected token '<'", "direct_js", "parse_error"),
        ("ReferenceError: Request is not defined", "direct_js", "missing_global_symbol"),
        ("ReferenceError: $_ts is not defined", "direct_js", "missing_prelude_state"),
        ("ReferenceError: document is not defined", "direct_js", "missing_dom_fixture"),
        ("TypeError: Foo is not a constructor", "direct_js", "missing_constructor_surface"),
        ("TypeError: x.y is not a function", "direct_js", "missing_method_surface"),
        ("descriptor mismatch for navigator", "direct_js", "missing_descriptor_shape"),
        (
            "runtime initialization failed: isolate unavailable",
            "direct_js",
            "runtime_internal_error",
        ),
        ("execution budget timeout", "direct_js", "timeout_or_loop"),
        ("not executable", "json_payload", "input_format_error"),
    ],
)
def test_classify_failure_kind_maps_common_pressure_shapes(
    message: str,
    input_kind: str,
    expected: str,
):
    assert classify_failure_kind(message, input_kind=input_kind) == expected


def test_pressure_from_failure_maps_request_to_network_surface_candidate():
    failure_kind = classify_failure_kind("ReferenceError: Request is not defined")
    pressure = pressure_from_failure(
        failure_kind,
        message="ReferenceError: Request is not defined",
        input_kind="direct_js",
    )
    promotion = promotion_for_pressure(pressure, failure_kind=failure_kind)

    assert pressure.pressure_kind == "network_surface"
    assert pressure.symbol == "Request"
    assert pressure.behavior_depth == "presence_only"
    assert promotion.level == "candidate_pack"
    assert promotion.evidence_ceiling == "diagnostic_only"


def test_pressure_from_failure_keeps_prelude_and_page_context_fixture_only():
    prelude_pressure = pressure_from_failure(
        "missing_prelude_state",
        message="ReferenceError: $_ts is not defined",
        input_kind="direct_js",
    )
    page_pressure = pressure_from_failure(
        "missing_page_bootstrap",
        message="document.currentScript is required",
        input_kind="script_tag_page",
    )

    assert prelude_pressure.pressure_kind == "prelude_contract"
    assert promotion_for_pressure(
        prelude_pressure,
        failure_kind="missing_prelude_state",
    ).level == "fixture_only"
    assert page_pressure.pressure_kind == "page_lifecycle"
    assert promotion_for_pressure(
        page_pressure,
        failure_kind="missing_page_bootstrap",
    ).level == "fixture_only"


def test_promotion_for_pressure_requires_repeated_non_presence_for_generic_substrate():
    pressure = PressureSignal(
        "web_api_surface",
        symbol="Request",
        access_pattern="constructor_call",
        behavior_depth="construct_only",
    )

    assert (
        promotion_for_pressure(
            pressure,
            failure_kind="missing_constructor_surface",
            sample_count=3,
        ).level
        == "generic_substrate_candidate"
    )
    assert (
        promotion_for_pressure(
            pressure,
            failure_kind="missing_constructor_surface",
            sample_count=1,
        ).level
        == "candidate_pack"
    )


def test_runtime_stability_pressure_is_observe_only():
    pressure = pressure_from_failure(
        "runtime_internal_error",
        message="runtime initialization failed",
        input_kind="direct_js",
    )
    promotion = promotion_for_pressure(pressure, failure_kind="runtime_internal_error")

    assert pressure.pressure_kind == "runtime_stability"
    assert promotion.level == "observe_only"


def test_build_pressure_report_success_is_diagnostic_observe_only():
    report = build_pressure_report("sample_success", "var x = 1;")
    data = report.to_dict()

    assert data["schema_version"] == ENVIRONMENT_PRESSURE_SCHEMA_VERSION
    assert data["input_kind"] == "direct_js"
    assert data["execution_mode"] == "browser_like_global"
    assert data["status"] == "success"
    assert data["failure_kind"] == "success"
    assert data["pressure"]["pressure_kind"] == "analysis_observability"
    assert data["promotion"]["level"] == "observe_only"
    assert data["writes"] == []
    assert all(item["strength"] == "diagnostic_only" for item in data["evidence"])


def test_build_pressure_report_maps_request_failure_without_apply():
    report = build_pressure_report(
        "sample_request",
        "new Request('/x')",
        message="ReferenceError: Request is not defined",
    )
    data = report.to_dict()

    assert data["status"] == "failed"
    assert data["failure_kind"] == "missing_global_symbol"
    assert data["pressure"]["pressure_kind"] == "network_surface"
    assert data["pressure"]["symbol"] == "Request"
    assert data["promotion"]["level"] == "candidate_pack"
    assert data["writes"] == []


@pytest.mark.parametrize(
    ("source", "input_kind", "execution_mode"),
    [
        ('{"entries": []}', "network_trace", "network_stubbed"),
        ('{"a": 1}', "json_payload", "bare_v8"),
        ("", "empty_or_invalid", "bare_v8"),
    ],
)
def test_build_pressure_report_skips_non_executable_inputs(
    source: str,
    input_kind: str,
    execution_mode: str,
):
    report = build_pressure_report("sample_skipped", source)
    data = report.to_dict()

    assert data["input_kind"] == input_kind
    assert data["execution_mode"] == execution_mode
    assert data["status"] == "skipped"
    assert data["failure_kind"] == "input_format_error"
    assert data["pressure"]["pressure_kind"] == "input_normalization"
    assert data["writes"] == []


def test_build_pressure_report_keeps_script_tag_page_fixture_only():
    report = build_pressure_report(
        "sample_script_tag",
        "<script>document.currentScript.dataset.x</script>",
        message="document.currentScript is required",
    )
    data = report.to_dict()

    assert data["input_kind"] == "script_tag_page"
    assert data["execution_mode"] == "script_tag_bootstrap"
    assert data["failure_kind"] == "missing_page_bootstrap"
    assert data["pressure"]["pressure_kind"] == "page_lifecycle"
    assert data["promotion"]["level"] == "fixture_only"


def test_build_pressure_report_can_override_status_but_not_writes():
    report = build_pressure_report(
        "sample_review",
        "var x = 1;",
        message="unsupported semantic",
        status="skipped",
    )
    data = report.to_dict()

    assert data["status"] == "skipped"
    assert data["failure_kind"] == "unsupported_semantics"
    assert data["writes"] == []


def test_run_environment_pressure_samples_summarizes_classification_coverage():
    batch = run_environment_pressure_samples([
        {"sample_id": "s1", "source": "var x = 1;"},
        {
            "sample_id": "s2",
            "source": "new Request('/x')",
            "message": "ReferenceError: Request is not defined",
        },
        {"sample_id": "s3", "source": '{"entries": []}'},
        PressureSample("s4", "var z = 1;", message="mystery failure"),
    ])
    data = batch.to_dict()
    summary = data["summary"]

    assert data["schema_version"] == ENVIRONMENT_PRESSURE_SCHEMA_VERSION
    assert len(data["reports"]) == 4
    assert data["writes"] == []
    assert summary["total"] == 4
    assert summary["classified_count"] == 3
    assert summary["unclassified_count"] == 1
    assert summary["by_input_kind"]["direct_js"] == 3
    assert summary["by_input_kind"]["network_trace"] == 1
    assert summary["by_failure_kind"]["success"] == 1
    assert summary["by_failure_kind"]["missing_global_symbol"] == 1
    assert summary["by_failure_kind"]["input_format_error"] == 1
    assert summary["by_failure_kind"]["unclassified"] == 1
    assert summary["by_pressure_kind"]["network_surface"] == 1
    assert summary["by_pressure_kind"]["input_normalization"] == 1
    assert summary["by_promotion_level"]["candidate_pack"] == 1
    assert summary["writes"] == []


def test_run_environment_pressure_samples_empty_batch_is_no_write():
    batch = run_environment_pressure_samples([])
    data = batch.to_dict()

    assert data["reports"] == []
    assert data["summary"]["total"] == 0
    assert data["summary"]["classified_count"] == 0
    assert data["summary"]["unclassified_count"] == 0
    assert data["writes"] == []


def test_run_environment_pressure_manifest_supports_inline_and_ref_only_items():
    batch = run_environment_pressure_manifest({
        "samples": [
            {"sample_id": "inline", "source": "var x = 1;"},
            {
                "sample_id": "ref-only",
                "source_ref": "D:/private/corpus/sample-a.js",
                "notes": {"family": "unknown"},
            },
            PressureManifestItem(
                "typed",
                source="new Request('/x')",
                message="ReferenceError: Request is not defined",
                source_ref="nested/sample-b.js",
            ),
        ]
    })
    data = batch.to_dict()
    manifest = data["summary"]["manifest"]

    assert data["writes"] == []
    assert data["summary"]["total"] == 3
    assert manifest["items"] == 3
    assert manifest["inline_source_count"] == 2
    assert manifest["source_ref_only_count"] == 1
    assert manifest["redacted_source_refs"] == ["sample-a.js", "sample-b.js"]
    assert "D:/private" not in repr(data)
    assert manifest["review_status"] == "review_only"
    assert manifest["evidence_ceiling"] == "diagnostic_only"

    ref_report = next(report for report in data["reports"] if report["sample_id"] == "ref-only")
    assert ref_report["status"] == "skipped"
    assert ref_report["input_kind"] == "empty_or_invalid"
    assert ref_report["writes"] == []


def test_run_environment_pressure_manifest_accepts_list_shape():
    batch = run_environment_pressure_manifest([
        {"sample_id": "s1", "source": "var x = 1;"},
        {"sample_id": "s2", "source": ""},
    ])
    data = batch.to_dict()

    assert data["summary"]["manifest"]["items"] == 2
    assert data["summary"]["manifest"]["inline_source_count"] == 2
    assert data["summary"]["manifest"]["source_ref_only_count"] == 0
    assert data["writes"] == []


def test_pressure_batch_diagnostics_project_counts_without_sources():
    batch = run_environment_pressure_samples([
        {"sample_id": "s1", "source": "var secret = 'do-not-leak';"},
        {
            "sample_id": "s2",
            "source": "new Request('/x')",
            "message": "ReferenceError: Request is not defined",
        },
    ])
    records = pressure_batch_diagnostics(batch)
    data = [record.to_dict() for record in records]

    assert [record["code"] for record in data] == [
        "ENV_PRESSURE_BATCH_SUMMARY",
        "ENV_PRESSURE_BATCH_CLASSIFICATION_COUNTS",
    ]
    assert data[0]["details"]["schema_version"] == ENVIRONMENT_PRESSURE_SCHEMA_VERSION
    assert data[0]["details"]["total"] == 2
    assert data[0]["details"]["classified_count"] == 2
    assert data[0]["details"]["unclassified_count"] == 0
    assert data[0]["details"]["review_status"] == "review_only"
    assert data[0]["details"]["evidence_ceiling"] == "diagnostic_only"
    assert data[0]["details"]["writes"] == []
    assert data[1]["details"]["by_pressure_kind"]["network_surface"] == 1
    assert data[1]["details"]["by_promotion_level"]["candidate_pack"] == 1
    assert "do-not-leak" not in repr(data)


def test_pressure_batch_diagnostics_include_redacted_manifest_summary_only():
    batch = run_environment_pressure_manifest({
        "samples": [
            {
                "sample_id": "ref",
                "source_ref": "C:/private/real-samples/sample.js",
            }
        ]
    })
    records = pressure_batch_diagnostics(batch)
    data = [record.to_dict() for record in records]
    manifest = next(
        record["details"]
        for record in data
        if record["code"] == "ENV_PRESSURE_BATCH_MANIFEST_SUMMARY"
    )

    assert manifest["items"] == 1
    assert manifest["source_ref_only_count"] == 1
    assert manifest["redacted_source_refs"] == ["sample.js"]
    assert manifest["review_status"] == "review_only"
    assert manifest["evidence_ceiling"] == "diagnostic_only"
    assert "C:/private" not in repr(data)


def test_pressure_batch_diagnostics_can_append_to_toolchain_report_roundtrip():
    toolchain_report = run_environment_toolchain("", probe_pack="fingerprint.m1", profile=None)
    batch = run_environment_pressure_manifest([
        {"sample_id": "s1", "source": "var x = 1;"},
        {"sample_id": "s2", "source_ref": "D:/private/sample.js"},
    ])
    bridge_records = environment_pressure_batch_to_toolchain_diagnostics(batch)

    data = toolchain_report_to_dict(toolchain_report)
    data["diagnostics"].extend(record.to_dict() for record in bridge_records)
    roundtrip = toolchain_report_to_dict(toolchain_report_from_dict(data))

    assert roundtrip["writes"] == []
    assert any(
        diagnostic["code"] == "ENV_PRESSURE_BATCH_SUMMARY"
        for diagnostic in roundtrip["diagnostics"]
    )
    assert any(
        diagnostic["code"] == "ENV_PRESSURE_BATCH_MANIFEST_SUMMARY"
        and diagnostic["details"]["redacted_source_refs"] == ["sample.js"]
        for diagnostic in roundtrip["diagnostics"]
    )
    assert "D:/private" not in repr(roundtrip)
