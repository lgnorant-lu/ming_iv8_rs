from __future__ import annotations

import pytest

from iv8_rs import (
    DiagnosticRecord,
    EvidenceRecord,
    FallbackAttempt,
    build_trace_diagnostics,
    classify_trace_prefix,
    confidence_from_evidence,
    evidence_satisfies,
)


def test_trace_prefix_registry_covers_shared_prefixes():
    cases = {
        "D,1,2,0": "dispatch",
        "R,1,navigator.userAgent,Mozilla": "read",
        "W,1,x,2": "write",
        "C,1,fn,value": "call",
        "eval,source.js,alert(1)": "eval_source",
        "fn_ctor,source.js,return 1": "function_constructor",
        "require,7,0": "module",
        "chunk_main,https://example.test/main.js": "chunk",
        "env_missing,navigator.plugins": "environment_gap",
        "patch,navigator.language,applied": "environment_patch",
    }
    for raw, expected_kind in cases.items():
        meta = classify_trace_prefix(raw)
        assert meta is not None
        assert meta["kind"] == expected_kind


def test_trace_diagnostics_report_empty_and_unknown_prefixes():
    empty = build_trace_diagnostics([])
    assert [item.code for item in empty] == ["TRACE_EMPTY"]

    diagnostics = build_trace_diagnostics(["D,1,2,0", "", "mystery,value"])
    assert [item.code for item in diagnostics] == [
        "TRACE_PARSE_PARTIAL",
        "TRACE_PREFIX_UNKNOWN",
    ]
    assert diagnostics[1].details["raw"] == "mystery,value"


def test_evidence_record_validates_strength_and_roundtrips():
    record = EvidenceRecord(
        kind="dispatch_trace_observed",
        strength="strong",
        source="trace",
        stage="dispatch.validate",
        summary="dispatch trace produced opcode sequence",
    )
    assert EvidenceRecord.from_dict(record.to_dict()) == record

    with pytest.raises(ValueError, match="invalid evidence strength"):
        EvidenceRecord(
            kind="trace",
            strength="certain",
            source="trace",
            stage="trace.parse",
            summary="bad strength",
        )


def test_diagnostic_record_validates_severity_and_roundtrips():
    record = DiagnosticRecord(
        code="EVIDENCE_MARKER_ONLY",
        severity="warn",
        stage="evidence.validate",
        message="marker-only evidence cannot satisfy pass",
        recovery_hint="collect runtime evidence",
    )
    assert DiagnosticRecord.from_dict(record.to_dict()) == record

    with pytest.raises(ValueError, match="invalid diagnostic severity"):
        DiagnosticRecord(
            code="BAD",
            severity="fatal",
            stage="test",
            message="bad severity",
        )


def test_marker_only_evidence_does_not_satisfy_expected_evidence():
    observed = [
        EvidenceRecord(
            kind="webpack_runtime_detected",
            strength="marker_only",
            source="webpack_bridge",
            stage="webpack.probe",
            summary="webpack marker detected",
        )
    ]
    assert not evidence_satisfies(["webpack_runtime_detected"], observed)

    observed.append(EvidenceRecord(
        kind="webpack_runtime_detected",
        strength="weak",
        source="webpack_bridge",
        stage="webpack.validate",
        summary="runtime marker validated by weak module clue",
    ))
    assert evidence_satisfies(["webpack_runtime_detected"], observed)


def test_confidence_from_evidence_uses_shared_labels():
    assert confidence_from_evidence([]) == "none"
    assert confidence_from_evidence([
        EvidenceRecord("marker", "marker_only", "source", "stage", "summary")
    ]) == "weak"
    assert confidence_from_evidence([
        EvidenceRecord("trace", "weak", "source", "stage", "summary")
    ]) == "medium"
    assert confidence_from_evidence([
        EvidenceRecord("trace", "strong", "source", "stage", "summary")
    ]) == "strong"


def test_fallback_attempt_roundtrips_with_evidence_and_diagnostics():
    attempt = FallbackAttempt(
        attempt_id="fallback[1]",
        strategy_id="source_regex.main",
        stage="fallback.execute",
        outcome="warn",
        reason="weak evidence only",
        evidence=[EvidenceRecord("source_regex_candidate", "weak", "source_regex", "probe", "candidate found")],
        diagnostics=[DiagnosticRecord("FALLBACK_USED", "warn", "fallback.execute", "fallback attempted")],
    )
    assert FallbackAttempt.from_dict(attempt.to_dict()) == attempt

    with pytest.raises(ValueError, match="invalid fallback outcome"):
        FallbackAttempt(
            attempt_id="fallback[2]",
            strategy_id="bad",
            stage="fallback.execute",
            outcome="maybe",
            reason="bad outcome",
        )
