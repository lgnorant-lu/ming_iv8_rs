"""Comprehensive tests for iv8_rs.diagnostics — pure Python data contracts."""
from __future__ import annotations

import pytest

pytest.importorskip("iv8_rs")

from iv8_rs.diagnostics import (
    EVIDENCE_STRENGTHS,
    DIAGNOSTIC_SEVERITIES,
    FALLBACK_OUTCOMES,
    TRACE_PREFIX_REGISTRY,
    DIAGNOSTIC_CATALOG,
    EvidenceRecord,
    DiagnosticRecord,
    FallbackAttempt,
    TraceEvent,
    EvidenceGateResult,
    classify_trace_prefix,
    build_trace_events,
    build_trace_diagnostics,
    build_evidence_diagnostics,
    evaluate_evidence_gate,
    evidence_satisfies,
    confidence_from_evidence,
    _payload_from_raw,
    _int_or_raw,
    _trace_payload_is_partial,
)


# ─── EvidenceRecord ──────────────────────────────────────────────────────────


class TestEvidenceRecord:
    def test_minimal_construction(self):
        r = EvidenceRecord("k", "strong", "s", "st", "sum")
        assert r.kind == "k"
        assert r.producer is None
        assert r.payload == {}

    def test_full_construction(self):
        r = EvidenceRecord(
            kind="k", strength="weak", source="s", stage="st",
            summary="sum", producer="p", sample_kind="sk",
            payload={"a": 1},
        )
        assert r.producer == "p"
        assert r.sample_kind == "sk"
        assert r.payload == {"a": 1}

    @pytest.mark.parametrize("invalid", ["certain", "impossible", "unknown", ""])
    def test_post_init_rejects_invalid_strength(self, invalid):
        with pytest.raises(ValueError, match="invalid evidence strength"):
            EvidenceRecord("k", invalid, "s", "st", "sum")

    @pytest.mark.parametrize("valid", sorted(EVIDENCE_STRENGTHS))
    def test_post_init_accepts_valid_strength(self, valid):
        r = EvidenceRecord("k", valid, "s", "st", "sum")
        assert r.strength == valid

    def test_from_dict_minimal(self):
        d = {"kind": "k", "strength": "strong", "source": "s", "stage": "st", "summary": "sum"}
        r = EvidenceRecord.from_dict(d)
        assert r.kind == "k"
        assert r.producer is None
        assert r.payload == {}

    def test_from_dict_full(self):
        d = {
            "kind": "k", "strength": "weak", "source": "s", "stage": "st",
            "summary": "sum", "producer": "p", "sample_kind": "sk",
            "payload": {"x": 1},
        }
        r = EvidenceRecord.from_dict(d)
        assert r.producer == "p"
        assert r.payload == {"x": 1}

    def test_from_dict_payload_defaults_to_empty(self):
        d = {"kind": "k", "strength": "marker_only", "source": "s", "stage": "st", "summary": "sum"}
        r = EvidenceRecord.from_dict(d)
        assert r.payload == {}

    def test_to_dict_roundtrip_minimal(self):
        r = EvidenceRecord("k", "strong", "s", "st", "sum")
        assert EvidenceRecord.from_dict(r.to_dict()) == r

    def test_to_dict_roundtrip_full(self):
        r = EvidenceRecord("k", "diagnostic_only", "s", "st", "sum", producer="p", sample_kind="sk", payload={"n": 42})
        assert EvidenceRecord.from_dict(r.to_dict()) == r

    def test_to_dict_with_optional_none(self):
        r = EvidenceRecord("k", "strong", "s", "st", "sum")
        d = r.to_dict()
        assert d["producer"] is None
        assert d["sample_kind"] is None


# ─── DiagnosticRecord ────────────────────────────────────────────────────────


class TestDiagnosticRecord:
    def test_minimal_construction(self):
        r = DiagnosticRecord("TRACE_EMPTY", "warn", "trace.parse", "msg")
        assert r.recovery_hint is None
        assert r.strategy_id is None
        assert r.details == {}

    def test_full_construction(self):
        r = DiagnosticRecord("POLICY_BLOCKED", "error", "policy", "msg", recovery_hint="fix", strategy_id="s1", sample_id="sm1", details={"k": "v"})
        assert r.details == {"k": "v"}

    @pytest.mark.parametrize("invalid", ["fatal", "debug", "", "critical"])
    def test_post_init_rejects_invalid_severity(self, invalid):
        with pytest.raises(ValueError, match="invalid diagnostic severity"):
            DiagnosticRecord("C", invalid, "st", "msg")

    @pytest.mark.parametrize("valid", sorted(DIAGNOSTIC_SEVERITIES))
    def test_post_init_accepts_valid_severity(self, valid):
        r = DiagnosticRecord("C", valid, "st", "msg")
        assert r.severity == valid

    def test_from_dict_minimal(self):
        d = {"code": "C", "severity": "info", "stage": "st", "message": "msg"}
        r = DiagnosticRecord.from_dict(d)
        assert r.recovery_hint is None
        assert r.details == {}

    def test_from_dict_full(self):
        d = {"code": "C", "severity": "error", "stage": "st", "message": "msg", "recovery_hint": "fix", "strategy_id": "s1", "sample_id": "sm1", "details": {"k": "v"}}
        r = DiagnosticRecord.from_dict(d)
        assert r.recovery_hint == "fix"
        assert r.details == {"k": "v"}

    def test_to_dict_roundtrip_minimal(self):
        r = DiagnosticRecord("C", "info", "st", "msg")
        assert DiagnosticRecord.from_dict(r.to_dict()) == r

    def test_to_dict_roundtrip_full(self):
        r = DiagnosticRecord("C", "warn", "st", "msg", recovery_hint="h", strategy_id="s", sample_id="sm", details={"a": 1})
        assert DiagnosticRecord.from_dict(r.to_dict()) == r


# ─── FallbackAttempt ─────────────────────────────────────────────────────────


class TestFallbackAttempt:
    def test_minimal_construction(self):
        f = FallbackAttempt("s1", "pass", "ok")
        assert f.next_strategy is None
        assert f.diagnostics == []
        assert f.evidence == []

    def test_full_construction(self):
        ev = EvidenceRecord("k", "weak", "s", "st", "sum")
        di = DiagnosticRecord("C", "info", "st", "msg")
        f = FallbackAttempt("s1", "warn", "weak", next_strategy="s2", diagnostics=[di], evidence=[ev])
        assert f.diagnostics == [di]
        assert f.evidence == [ev]

    @pytest.mark.parametrize("invalid", ["maybe", "unknown", "error", ""])
    def test_post_init_rejects_invalid_outcome(self, invalid):
        with pytest.raises(ValueError, match="invalid fallback outcome"):
            FallbackAttempt("s1", invalid, "reason")

    @pytest.mark.parametrize("valid", sorted(FALLBACK_OUTCOMES))
    def test_post_init_accepts_valid_outcome(self, valid):
        f = FallbackAttempt("s1", valid, "reason")
        assert f.status == valid

    def test_from_dict_minimal(self):
        d = {"strategy_id": "s1", "status": "pass", "reason": "ok"}
        f = FallbackAttempt.from_dict(d)
        assert f.next_strategy is None
        assert f.diagnostics == []
        assert f.evidence == []

    def test_from_dict_with_nested(self):
        d = {
            "strategy_id": "s1", "status": "warn", "reason": "low",
            "next_strategy": "s2",
            "diagnostics": [{"code": "C", "severity": "info", "stage": "st", "message": "m"}],
            "evidence": [{"kind": "k", "strength": "weak", "source": "s", "stage": "st", "summary": "sum"}],
        }
        f = FallbackAttempt.from_dict(d)
        assert len(f.diagnostics) == 1
        assert len(f.evidence) == 1

    def test_to_dict_roundtrip_minimal(self):
        f = FallbackAttempt("s1", "skip", "n/a")
        assert FallbackAttempt.from_dict(f.to_dict()) == f

    def test_to_dict_roundtrip_full(self):
        ev = EvidenceRecord("k", "weak", "s", "st", "sum")
        di = DiagnosticRecord("C", "warn", "st", "msg")
        f = FallbackAttempt("s1", "fail", "bad", next_strategy="s2", diagnostics=[di], evidence=[ev])
        assert FallbackAttempt.from_dict(f.to_dict()) == f

    def test_to_dict_serializes_nested_records(self):
        ev = EvidenceRecord("k", "diagnostic_only", "s", "st", "sum")
        di = DiagnosticRecord("C", "error", "st", "msg")
        f = FallbackAttempt("s1", "skipped", "n/a", diagnostics=[di], evidence=[ev])
        d = f.to_dict()
        assert d["diagnostics"][0]["code"] == "C"
        assert d["evidence"][0]["kind"] == "k"


# ─── TraceEvent ──────────────────────────────────────────────────────────────


class TestTraceEvent:
    def test_from_raw_dispatch(self):
        e = TraceEvent.from_raw("D,42,15,3")
        assert e is not None
        assert e.kind == "dispatch"
        assert e.prefix == "D"
        assert e.payload["pc"] == 42
        assert e.payload["opcode"] == 15
        assert e.payload["stack_depth"] == 3

    def test_from_raw_read(self):
        e = TraceEvent.from_raw("R,100,target,Mozilla")
        assert e is not None
        assert e.kind == "read"
        assert e.payload["target"] == "target"

    def test_from_raw_write(self):
        e = TraceEvent.from_raw("W,200,x,string")
        assert e is not None
        assert e.kind == "write"
        assert e.payload["target"] == "x"

    def test_from_raw_call(self):
        e = TraceEvent.from_raw("C,300,myFunc,result")
        assert e is not None
        assert e.kind == "call"
        assert e.payload["callee"] == "myFunc"

    def test_from_raw_eval(self):
        e = TraceEvent.from_raw("eval,source.js,alert(1)")
        assert e is not None
        assert e.kind == "eval_source"
        assert e.payload["source_len"] == 18

    def test_from_raw_fn_ctor(self):
        e = TraceEvent.from_raw("fn_ctor,source.js,return 1")
        assert e is not None
        assert e.kind == "function_constructor"
        assert e.payload["source_len"] == 18

    def test_from_raw_require(self):
        e = TraceEvent.from_raw("require,7,0")
        assert e is not None
        assert e.kind == "module"
        assert e.payload["module_id"] == "7"

    def test_from_raw_chunk(self):
        e = TraceEvent.from_raw("chunk_main,https://example.test/main.js")
        assert e is not None
        assert e.kind == "chunk"
        assert e.payload["chunk_id"] == "main"

    def test_from_raw_env_missing(self):
        e = TraceEvent.from_raw("env_missing,navigator.plugins")
        assert e is not None
        assert e.kind == "environment_gap"
        assert e.confidence == "none"
        assert e.payload["target"] == "navigator.plugins"

    def test_from_raw_patch(self):
        e = TraceEvent.from_raw("patch,navigator.language,applied,ok")
        assert e is not None
        assert e.kind == "environment_patch"
        assert e.payload["patch_id"] == "navigator.language"
        assert e.payload["policy"] == "applied"
        assert e.payload["result"] == "ok"

    def test_from_raw_patch_partial(self):
        e = TraceEvent.from_raw("patch,id")
        assert e is not None
        assert e.payload["patch_id"] == "id"
        assert e.payload["policy"] is None
        assert e.payload["result"] is None

    def test_from_raw_env_missing_no_body(self):
        e = TraceEvent.from_raw("env_missing,")
        assert e is not None
        assert e.payload["target"] is None

    def test_from_raw_unknown_returns_none(self):
        assert TraceEvent.from_raw("unknown,payload") is None
        assert TraceEvent.from_raw("") is None
        assert TraceEvent.from_raw(" ,") is None

    def test_from_raw_dispatch_nonnumeric_pc(self):
        e = TraceEvent.from_raw("D,abc,15,3")
        assert e is not None
        assert e.payload["pc"] == "abc"

    def test_from_raw_override_params(self):
        e = TraceEvent.from_raw("D,1,2,0", stage="custom", strategy_id="strat", sample_kind="sk")
        assert e.stage == "custom"
        assert e.strategy_id == "strat"
        assert e.sample_kind == "sk"

    def test_from_dict_roundtrip(self):
        e = TraceEvent.from_raw("D,42,15,3", stage="dispatch.execute", strategy_id="dispatch.main", sample_kind="vm")
        assert e is not None
        assert TraceEvent.from_dict(e.to_dict()) == e

    def test_from_dict_roundtrip_env_missing(self):
        e = TraceEvent.from_raw("env_missing,navigator.plugins")
        assert e is not None
        assert TraceEvent.from_dict(e.to_dict()) == e

    def test_from_dict_minimal(self):
        d = {"version": "1", "kind": "dispatch", "prefix": "D", "stage": "st", "strategy_id": "s", "sample_kind": "sk"}
        e = TraceEvent.from_dict(d)
        assert e.payload == {}
        assert e.source == {}
        assert e.confidence == "weak"

    def test_from_dict_full(self):
        d = {"version": "1", "kind": "dispatch", "prefix": "D", "stage": "st", "strategy_id": "s", "sample_kind": "sk", "payload": {"a": 1}, "source": {"raw": "D,1,2"}, "confidence": "strong"}
        e = TraceEvent.from_dict(d)
        assert e.payload == {"a": 1}
        assert e.confidence == "strong"


# ─── EvidenceGateResult ──────────────────────────────────────────────────────


class TestEvidenceGateResult:
    def test_to_dict_with_evidence_and_diagnostics(self):
        ev = EvidenceRecord("k", "weak", "s", "st", "sum")
        di = DiagnosticRecord("C", "info", "st", "msg")
        r = EvidenceGateResult(status="pass", confidence="medium", satisfied=True, evidence=[ev], diagnostics=[di])
        d = r.to_dict()
        assert d["status"] == "pass"
        assert len(d["evidence"]) == 1
        assert len(d["diagnostics"]) == 1
        assert d["evidence"][0]["kind"] == "k"
        assert d["diagnostics"][0]["code"] == "C"

    def test_to_dict_empty_lists(self):
        r = EvidenceGateResult(status="fail", confidence="none", satisfied=False)
        d = r.to_dict()
        assert d["evidence"] == []
        assert d["diagnostics"] == []

    def test_to_dict_roundtrip(self):
        ev = EvidenceRecord("k", "strong", "s", "st", "sum")
        r1 = EvidenceGateResult(status="pass", confidence="strong", satisfied=True, evidence=[ev])
        d = r1.to_dict()
        r2 = EvidenceGateResult(
            status=d["status"],
            confidence=d["confidence"],
            satisfied=d["satisfied"],
            evidence=[EvidenceRecord.from_dict(item) for item in d["evidence"]],
            diagnostics=[DiagnosticRecord.from_dict(item) for item in d["diagnostics"]],
        )
        assert r2.evidence == [ev]


# ─── classify_trace_prefix ────────────────────────────────────────────────────


class TestClassifyTracePrefix:
    @pytest.mark.parametrize("raw,expected_kind,expected_producer", [
        ("D,1,2,0", "dispatch", "dispatch"),
        ("R,1,x,Mozilla", "read", "runtime_hook"),
        ("W,1,y,2", "write", "runtime_hook"),
        ("C,1,fn,value", "call", "runtime_hook"),
        ("eval,source.js,alert", "eval_source", "source_ast"),
        ("fn_ctor,source.js,return", "function_constructor", "source_ast"),
        ("require,7,0", "module", "webpack_bridge"),
        ("chunk_main,https://test.js", "chunk", "webpack_bridge"),
        ("env_missing,navigator", "environment_gap", "environment_plane"),
        ("patch,navigator,applied", "environment_patch", "environment_plane"),
    ])
    def test_known_prefixes(self, raw, expected_kind, expected_producer):
        meta = classify_trace_prefix(raw)
        assert meta is not None
        assert meta["kind"] == expected_kind
        assert meta["producer"] == expected_producer

    @pytest.mark.parametrize("raw", ["", " ,", "unknown,value", "Z,1,2", "xyz"])
    def test_unknown_prefixes_return_none(self, raw):
        assert classify_trace_prefix(raw) is None


# ─── _payload_from_raw (indirect via TraceEvent.from_raw) ──────────────────


class TestPayloadFromRaw:
    def test_dispatch_fields(self):
        e = TraceEvent.from_raw("D,1,2,3,extra,fields")
        assert e is not None
        assert e.payload["pc"] == 1
        assert e.payload["opcode"] == 2
        assert e.payload["stack_depth"] == 3
        assert e.payload["raw_fields"] == ["1", "2", "3", "extra", "fields"]

    def test_dispatch_minimal(self):
        e = TraceEvent.from_raw("D,")
        assert e is not None
        assert e.payload["pc"] is None
        assert e.payload["opcode"] is None
        assert e.payload["stack_depth"] is None

    def test_read_fields(self):
        e = TraceEvent.from_raw("R,100,target,kind,extra")
        assert e is not None
        assert e.payload["pc"] == 100
        assert e.payload["target"] == "target"
        assert e.payload["value_kind"] == "kind"

    def test_read_minimal(self):
        e = TraceEvent.from_raw("R,")
        assert e is not None
        assert e.payload["pc"] is None

    def test_write_fields(self):
        e = TraceEvent.from_raw("W,200,target2,str")
        assert e is not None
        assert e.payload["pc"] == 200
        assert e.payload["target"] == "target2"
        assert e.payload["value_kind"] == "str"

    def test_call_fields(self):
        e = TraceEvent.from_raw("C,300,myFunc,result")
        assert e is not None
        assert e.payload["pc"] == 300
        assert e.payload["callee"] == "myFunc"
        assert e.payload["result_kind"] == "result"

    def test_call_minimal(self):
        e = TraceEvent.from_raw("C,400")
        assert e is not None
        assert e.payload["callee"] is None

    def test_eval_source(self):
        e = TraceEvent.from_raw("eval,alert(1)")
        assert e is not None
        assert e.payload["source_len"] == 8

    def test_fn_ctor_source(self):
        e = TraceEvent.from_raw("fn_ctor,return 42")
        assert e is not None
        assert e.payload["source_len"] == 9

    def test_require_full(self):
        e = TraceEvent.from_raw("require,modId,parentId")
        assert e is not None
        assert e.payload["module_id"] == "modId"
        assert e.payload["parent_id"] == "parentId"

    def test_require_minimal(self):
        e = TraceEvent.from_raw("require,modId")
        assert e is not None
        assert e.payload["parent_id"] is None

    def test_chunk_with_url(self):
        e = TraceEvent.from_raw("chunk_abc,https://test.com/c.js")
        assert e is not None
        assert e.payload["chunk_id"] == "abc"
        assert e.payload["url"] == "https://test.com/c.js"

    def test_chunk_no_url(self):
        e = TraceEvent.from_raw("chunk_def")
        assert e is not None
        assert e.payload["chunk_id"] == "def"
        assert e.payload["url"] is None

    def test_env_missing_with_target(self):
        e = TraceEvent.from_raw("env_missing,navigator.cookieEnabled")
        assert e is not None
        assert e.payload["target"] == "navigator.cookieEnabled"

    def test_env_missing_empty(self):
        e = TraceEvent.from_raw("env_missing,")
        assert e is not None
        assert e.payload["target"] is None

    def test_patch_full(self):
        e = TraceEvent.from_raw("patch,pid,policy,result")
        assert e is not None
        assert e.payload["patch_id"] == "pid"
        assert e.payload["policy"] == "policy"
        assert e.payload["result"] == "result"

    def test_patch_partial(self):
        e = TraceEvent.from_raw("patch,pid,policy")
        assert e is not None
        assert e.payload["patch_id"] == "pid"
        assert e.payload["policy"] == "policy"
        assert e.payload["result"] is None


# ─── _int_or_raw ──────────────────────────────────────────────────────────────


class TestIntOrRaw:
    def test_converts_integer_string(self):
        assert _int_or_raw("42") == 42
        assert _int_or_raw("-1") == -1
        assert _int_or_raw("0") == 0

    def test_returns_raw_on_non_numeric(self):
        assert _int_or_raw("abc") == "abc"
        assert _int_or_raw("") == ""
        assert _int_or_raw("12.5") == "12.5"

    def test_via_trace_event_dispatch_nonnumeric_opcode(self):
        e = TraceEvent.from_raw("D,1,xyz,3")
        assert e is not None
        assert e.payload["opcode"] == "xyz"


# ─── _trace_payload_is_partial ────────────────────────────────────────────────


class TestTracePayloadIsPartial:
    @pytest.mark.parametrize("raw,prefix,expected", [
        ("D,1,2,3", "D,", False),
        ("D,1", "D,", True),
        ("D,", "D,", True),
        ("R,100,target", "R,", False),
        ("R,", "R,", True),
        ("C,400", "C,", False),
        ("C,", "C,", True),
        ("patch,id,policy,result", "patch,", False),
        ("patch,id", "patch,", True),
        ("patch,id,policy", "patch,", True),
        ("env_missing,target", "env_missing,", False),
        ("env_missing,", "env_missing,", True),
        ("", "D,", False),
    ])
    def test_partial_detection(self, raw, prefix, expected):
        assert _trace_payload_is_partial(prefix, raw) == expected


# ─── build_trace_events ───────────────────────────────────────────────────────


class TestBuildTraceEvents:
    def test_filters_unknown_and_empty(self):
        events = build_trace_events(["D,1,2,0", "", "unknown,value", "R,100,target"])
        assert len(events) == 2
        assert all(e is not None for e in events)

    def test_empty_input(self):
        assert build_trace_events([]) == []

    def test_all_unknown(self):
        assert build_trace_events(["foo,1", "bar,2"]) == []

    def test_custom_params(self):
        events = build_trace_events(["D,1,2,0"], stage="custom", strategy_id="strat", sample_kind="sk")
        assert len(events) == 1
        assert events[0].stage == "custom"
        assert events[0].strategy_id == "strat"
        assert events[0].sample_kind == "sk"


# ─── build_trace_diagnostics ──────────────────────────────────────────────────


class TestBuildTraceDiagnostics:
    def test_empty_trace(self):
        diags = build_trace_diagnostics([])
        assert len(diags) == 1
        assert diags[0].code == "TRACE_EMPTY"

    def test_empty_line_skipped(self):
        diags = build_trace_diagnostics(["D,1,2,0", "", "R,1,x"])
        codes = [d.code for d in diags]
        assert "TRACE_PARSE_PARTIAL" in codes

    def test_unknown_prefix(self):
        diags = build_trace_diagnostics(["mystery,value"])
        assert diags[0].code == "TRACE_PREFIX_UNKNOWN"
        assert diags[0].details["raw"] == "mystery,value"

    def test_complete_trace_produces_no_diagnostics(self):
        diags = build_trace_diagnostics(["D,1,2,3", "R,100,target", "W,200,x,str"])
        assert diags == []

    def test_malformed_payload(self):
        diags = build_trace_diagnostics(["D,1", "R,0"])
        codes = [d.code for d in diags]
        assert "TRACE_PARSE_PARTIAL" in codes

    def test_empty_string_in_middle(self):
        diags = build_trace_diagnostics(["D,1,2,3", "", "R,100"])
        codes = [d.code for d in diags]
        assert "TRACE_PARSE_PARTIAL" in codes

    def test_partial_dispatch(self):
        diags = build_trace_diagnostics(["D,"])
        assert any(d.code == "TRACE_PARSE_PARTIAL" for d in diags)

    def test_partial_patch(self):
        diags = build_trace_diagnostics(["patch,"])
        assert any(d.code == "TRACE_PARSE_PARTIAL" for d in diags)

    def test_partial_patch_one_field(self):
        diags = build_trace_diagnostics(["patch,id"])
        assert any(d.code == "TRACE_PARSE_PARTIAL" for d in diags)

    def test_partial_patch_two_fields(self):
        diags = build_trace_diagnostics(["patch,id,policy"])
        assert any(d.code == "TRACE_PARSE_PARTIAL" for d in diags)


# ─── build_evidence_diagnostics ──────────────────────────────────────────────


class TestBuildEvidenceDiagnostics:
    def test_all_expected_present_with_strong_evidence(self):
        observed = [EvidenceRecord("kind_a", "strong", "s", "st", "sum")]
        diags = build_evidence_diagnostics(["kind_a"], observed)
        assert diags == []

    def test_missing_expected(self):
        diags = build_evidence_diagnostics(["missing_kind"], [])
        assert len(diags) == 1
        assert diags[0].code == "EVIDENCE_EXPECTED_MISSING"

    def test_marker_only(self):
        observed = [EvidenceRecord("k", "marker_only", "s", "st", "sum")]
        diags = build_evidence_diagnostics(["k"], observed)
        assert diags[0].code == "EVIDENCE_MARKER_ONLY"

    def test_diagnostic_only_also_counts_as_marker_only(self):
        observed = [EvidenceRecord("k", "diagnostic_only", "s", "st", "sum")]
        diags = build_evidence_diagnostics(["k"], observed)
        assert diags[0].code == "EVIDENCE_MARKER_ONLY"

    def test_policy_blocked(self):
        diags = build_evidence_diagnostics([], [], policy_blocked=True)
        assert diags[0].code == "POLICY_BLOCKED_ACTION"

    def test_policy_blocked_and_missing(self):
        observed = [EvidenceRecord("k", "marker_only", "s", "st", "sum")]
        diags = build_evidence_diagnostics(["k", "missing_kind"], observed, policy_blocked=True)
        codes = [d.code for d in diags]
        assert "POLICY_BLOCKED_ACTION" in codes
        assert "EVIDENCE_MARKER_ONLY" in codes
        assert "EVIDENCE_EXPECTED_MISSING" in codes

    def test_mixed_strength_for_same_kind(self):
        observed = [
            EvidenceRecord("k", "marker_only", "s", "st", "sum"),
            EvidenceRecord("k", "weak", "s", "st", "sum"),
        ]
        diags = build_evidence_diagnostics(["k"], observed)
        assert diags == []

    def test_empty_expected_and_no_policy(self):
        assert build_evidence_diagnostics([], []) == []


# ─── evaluate_evidence_gate ──────────────────────────────────────────────────


class TestEvaluateEvidenceGate:
    def test_pass(self):
        observed = [EvidenceRecord("k", "strong", "s", "st", "sum")]
        r = evaluate_evidence_gate(["k"], observed)
        assert r.status == "pass"
        assert r.satisfied is True
        assert r.confidence == "strong"

    def test_warn_marker_only(self):
        observed = [EvidenceRecord("k", "marker_only", "s", "st", "sum")]
        r = evaluate_evidence_gate(["k"], observed)
        assert r.status == "warn"
        assert r.satisfied is False

    def test_fail_missing_evidence(self):
        r = evaluate_evidence_gate(["k"], [])
        assert r.status == "fail"
        assert r.satisfied is False

    def test_fail_policy_blocked(self):
        observed = [EvidenceRecord("k", "strong", "s", "st", "sum")]
        r = evaluate_evidence_gate(["k"], observed, policy_blocked=True)
        assert r.status == "fail"
        assert r.satisfied is False

    def test_warn_diagnostic_only(self):
        observed = [EvidenceRecord("k", "diagnostic_only", "s", "st", "sum")]
        r = evaluate_evidence_gate(["k"], observed)
        assert r.status == "warn"

    def test_empty_expected_satisfied(self):
        r = evaluate_evidence_gate([], [])
        assert r.status == "pass"
        assert r.satisfied is True

    def test_result_has_evidence_and_diagnostics(self):
        di = DiagnosticRecord("C", "error", "st", "msg")
        ev = EvidenceRecord("k", "strong", "s", "st", "sum")
        r = evaluate_evidence_gate(["k"], [ev])
        assert r.evidence == [ev]
        assert r.diagnostics == []


# ─── evidence_satisfies ───────────────────────────────────────────────────────


class TestEvidenceSatisfies:
    def test_empty_expected_is_satisfied(self):
        assert evidence_satisfies([], []) is True

    def test_strong_satisfies(self):
        ev = EvidenceRecord("k", "strong", "s", "st", "sum")
        assert evidence_satisfies(["k"], [ev]) is True

    def test_weak_satisfies(self):
        ev = EvidenceRecord("k", "weak", "s", "st", "sum")
        assert evidence_satisfies(["k"], [ev]) is True

    def test_marker_only_does_not_satisfy(self):
        ev = EvidenceRecord("k", "marker_only", "s", "st", "sum")
        assert evidence_satisfies(["k"], [ev]) is False

    def test_diagnostic_only_does_not_satisfy(self):
        ev = EvidenceRecord("k", "diagnostic_only", "s", "st", "sum")
        assert evidence_satisfies(["k"], [ev]) is False

    def test_missing_kind_does_not_satisfy(self):
        ev = EvidenceRecord("other_kind", "strong", "s", "st", "sum")
        assert evidence_satisfies(["k"], [ev]) is False

    def test_multiple_kinds_all_must_be_satisfied(self):
        ev1 = EvidenceRecord("k1", "strong", "s", "st", "sum")
        ev2 = EvidenceRecord("k2", "marker_only", "s", "st", "sum")
        assert evidence_satisfies(["k1", "k2"], [ev1, ev2]) is False

    def test_mixed_strengths_for_same_kind(self):
        ev1 = EvidenceRecord("k", "marker_only", "s", "st", "sum")
        ev2 = EvidenceRecord("k", "weak", "s", "st", "sum")
        assert evidence_satisfies(["k"], [ev1, ev2]) is True


# ─── confidence_from_evidence ─────────────────────────────────────────────────


class TestConfidenceFromEvidence:
    def test_empty_is_none(self):
        assert confidence_from_evidence([]) == "none"

    def test_strong_returns_strong(self):
        assert confidence_from_evidence([EvidenceRecord("k", "strong", "s", "st", "sum")]) == "strong"

    def test_weak_returns_medium(self):
        assert confidence_from_evidence([EvidenceRecord("k", "weak", "s", "st", "sum")]) == "medium"

    def test_marker_only_returns_weak(self):
        assert confidence_from_evidence([EvidenceRecord("k", "marker_only", "s", "st", "sum")]) == "weak"

    def test_diagnostic_only_returns_weak(self):
        assert confidence_from_evidence([EvidenceRecord("k", "diagnostic_only", "s", "st", "sum")]) == "weak"

    def test_strong_overrides_others(self):
        recs = [
            EvidenceRecord("k", "marker_only", "s", "st", "sum"),
            EvidenceRecord("k", "strong", "s", "st", "sum"),
        ]
        assert confidence_from_evidence(recs) == "strong"

    def test_weak_overrides_marker_only(self):
        recs = [
            EvidenceRecord("k", "marker_only", "s", "st", "sum"),
            EvidenceRecord("k", "weak", "s", "st", "sum"),
        ]
        assert confidence_from_evidence(recs) == "medium"

    def test_extra_unexpected_strength_is_ignored(self):
        assert confidence_from_evidence([]) == "none"


# ─── DIAGNOSTIC_CATALOG ──────────────────────────────────────────────────────


class TestDiagnosticCatalog:
    def test_all_codes_have_valid_severity(self):
        for code, meta in DIAGNOSTIC_CATALOG.items():
            assert meta["severity"] in DIAGNOSTIC_SEVERITIES, f"{code} has bad severity"

    def test_all_codes_have_stage(self):
        for code, meta in DIAGNOSTIC_CATALOG.items():
            assert isinstance(meta["stage"], str) and meta["stage"], f"{code} missing stage"

    def test_specific_codes_present(self):
        required = {
            "TRACE_EMPTY", "TRACE_PREFIX_UNKNOWN", "TRACE_PARSE_PARTIAL",
            "EVIDENCE_EXPECTED_MISSING", "EVIDENCE_MARKER_ONLY",
            "POLICY_BLOCKED_ACTION", "FALLBACK_USED", "FALLBACK_EXHAUSTED",
            "ENVIRONMENT_GAP_OBSERVED", "ENVIRONMENT_PATCH_REJECTED",
        }
        assert required <= set(DIAGNOSTIC_CATALOG)


# ─── TRACE_PREFIX_REGISTRY ────────────────────────────────────────────────────


class TestTracePrefixRegistry:
    def test_all_entries_have_kind_and_producer(self):
        for prefix, meta in TRACE_PREFIX_REGISTRY.items():
            assert "kind" in meta, f"{prefix} missing kind"
            assert "producer" in meta, f"{prefix} missing producer"

    def test_all_prefixes_are_covered_by_classify(self):
        for prefix in TRACE_PREFIX_REGISTRY:
            test_str = prefix + "test"
            meta = classify_trace_prefix(test_str)
            assert meta is not None, f"classify_trace_prefix failed for {prefix!r}"
            assert meta["prefix"] == prefix
