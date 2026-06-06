from __future__ import annotations

import pytest

from iv8_rs import (
    DIAGNOSTIC_CATALOG,
    DiagnosticRecord,
    EvidenceGateResult,
    EvidenceRecord,
    FallbackAttempt,
    TraceEvent,
    build_evidence_diagnostics,
    build_trace_diagnostics,
    build_trace_events,
    classify_trace_prefix,
    confidence_from_evidence,
    evaluate_evidence_gate,
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


def test_initial_diagnostic_catalog_covers_v061_codes():
    expected_codes = {
        "TRACE_EMPTY",
        "TRACE_PREFIX_UNKNOWN",
        "TRACE_PARSE_PARTIAL",
        "EVIDENCE_EXPECTED_MISSING",
        "EVIDENCE_MARKER_ONLY",
        "CONFIDENCE_DOWNGRADED",
        "POLICY_BLOCKED_ACTION",
        "FALLBACK_USED",
        "FALLBACK_EXHAUSTED",
        "SOURCE_REGEX_PASS_THROUGH",
        "SWITCHVM_MARKER_ONLY",
        "ENVIRONMENT_GAP_OBSERVED",
        "ENVIRONMENT_PATCH_REJECTED",
        "ENVIRONMENT_PATCH_UNSAFE",
    }
    assert expected_codes <= set(DIAGNOSTIC_CATALOG)
    for code in expected_codes:
        assert DIAGNOSTIC_CATALOG[code]["severity"] in {"info", "warn", "error"}
        assert DIAGNOSTIC_CATALOG[code]["stage"]


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


def test_evidence_diagnostics_cover_missing_marker_only_and_policy_block():
    observed = [
        EvidenceRecord(
            kind="webpack_runtime_detected",
            strength="marker_only",
            source="webpack_bridge",
            stage="webpack.probe",
            summary="webpack marker detected",
        )
    ]
    diagnostics = build_evidence_diagnostics(
        ["webpack_runtime_detected", "module_table_captured"],
        observed,
        policy_blocked=True,
    )
    codes = [item.code for item in diagnostics]
    assert codes == [
        "POLICY_BLOCKED_ACTION",
        "EVIDENCE_MARKER_ONLY",
        "EVIDENCE_EXPECTED_MISSING",
    ]
    assert diagnostics[0].severity == "error"
    assert diagnostics[1].details["evidence_kind"] == "webpack_runtime_detected"
    assert diagnostics[2].details["evidence_kind"] == "module_table_captured"


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

    skipped = FallbackAttempt(
        attempt_id="fallback[skip]",
        strategy_id="manual_hint",
        stage="fallback.plan",
        outcome="skip",
        reason="not applicable",
    )
    assert FallbackAttempt.from_dict(skipped.to_dict()) == skipped

    with pytest.raises(ValueError, match="invalid fallback outcome"):
        FallbackAttempt(
            attempt_id="fallback[2]",
            strategy_id="bad",
            stage="fallback.execute",
            outcome="maybe",
            reason="bad outcome",
        )


def test_trace_event_from_raw_known_prefixes():
    event = TraceEvent.from_raw("D,42,15,3")
    assert event is not None
    assert event.kind == "dispatch"
    assert event.prefix == "D"
    assert event.payload["pc"] == 42
    assert event.payload["opcode"] == 15
    assert event.payload["stack_depth"] == 3

    event_r = TraceEvent.from_raw("R,100,navigator.userAgent,Mozilla/5.0")
    assert event_r is not None
    assert event_r.kind == "read"
    assert event_r.payload["target"] == "navigator.userAgent"

    event_eval = TraceEvent.from_raw("eval,source.js,alert(1)")
    assert event_eval is not None
    assert event_eval.kind == "eval_source"

    event_require = TraceEvent.from_raw("require,7,0")
    assert event_require is not None
    assert event_require.kind == "module"

    event_chunk = TraceEvent.from_raw("chunk_main,https://example.test/main.js")
    assert event_chunk is not None
    assert event_chunk.kind == "chunk"
    assert event_chunk.payload["chunk_id"] == "main"


def test_trace_event_from_raw_unknown_returns_none():
    assert TraceEvent.from_raw("unknown,payload") is None
    assert TraceEvent.from_raw("") is None


def test_trace_event_roundtrip():
    event = TraceEvent.from_raw("D,42,15,3", stage="dispatch.execute", strategy_id="dispatch.main", sample_kind="vm_dispatch_known")
    assert event is not None
    assert TraceEvent.from_dict(event.to_dict()) == event


def test_build_trace_events():
    raw = ["D,1,2,0", "R,100,screen.width,1920", "mystery,value"]
    events = build_trace_events(raw, strategy_id="test")
    assert len(events) == 2
    assert events[0].kind == "dispatch"
    assert events[1].kind == "read"
    assert all(e.strategy_id == "test" for e in events)


def test_trace_prefix_malformed_payload_detected():
    diagnostics = build_trace_diagnostics(["D,1", "R,0"])
    codes = [item.code for item in diagnostics]
    assert "TRACE_PARSE_PARTIAL" in codes, f"TRACE_PARSE_PARTIAL missing from {codes}"

    empty_prefix = build_trace_diagnostics(["D,"])
    assert any(item.code == "TRACE_PARSE_PARTIAL" for item in empty_prefix)


def test_evaluate_evidence_gate_pass():
    observed = [
        EvidenceRecord(kind="dispatch_trace_observed", strength="strong", source="dispatch", stage="distpatch.validate", summary="rt observed"),
    ]
    result = evaluate_evidence_gate(["dispatch_trace_observed"], observed)
    assert result.status == "pass"
    assert result.satisfied is True
    assert result.confidence == "strong"
    assert isinstance(result, EvidenceGateResult)


def test_evaluate_evidence_gate_marker_only():
    observed = [
        EvidenceRecord(kind="webpack_runtime_detected", strength="marker_only", source="probe", stage="probe", summary="marker"),
    ]
    result = evaluate_evidence_gate(["webpack_runtime_detected"], observed)
    assert result.status == "warn", "marker-only evidence should be WARN, not PASS or FAIL"
    assert result.satisfied is False
    assert result.confidence == "weak"


def test_evaluate_evidence_gate_policy_blocked():
    observed = [
        EvidenceRecord(kind="dispatch_trace_observed", strength="strong", source="dispatch", stage="distpatch.validate", summary="rt observed"),
    ]
    result = evaluate_evidence_gate(["dispatch_trace_observed"], observed, policy_blocked=True)
    assert result.status == "fail"
    assert result.satisfied is False
    assert any(item.code == "POLICY_BLOCKED_ACTION" for item in result.diagnostics)


def test_evaluate_evidence_gate_empty():
    result = evaluate_evidence_gate(["required_evidence"], [])
    assert result.status == "fail"
    assert result.satisfied is False
    assert result.confidence == "none"
    assert any(item.code == "EVIDENCE_EXPECTED_MISSING" for item in result.diagnostics)
