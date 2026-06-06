"""Shared trace, evidence, diagnostics, and fallback contracts.

This module is intentionally lightweight: it gives Python callers and tests a
stable contract layer without changing the Rust Entry Plane result schema.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass, field
from typing import Any, Dict, Iterable, List, Optional


TRACE_PREFIX_REGISTRY: Dict[str, Dict[str, str]] = {
    "D,": {"kind": "dispatch", "producer": "dispatch"},
    "R,": {"kind": "read", "producer": "runtime_hook"},
    "W,": {"kind": "write", "producer": "runtime_hook"},
    "C,": {"kind": "call", "producer": "runtime_hook"},
    "eval,": {"kind": "eval_source", "producer": "source_ast"},
    "fn_ctor,": {"kind": "function_constructor", "producer": "source_ast"},
    "require,": {"kind": "module", "producer": "webpack_bridge"},
    "chunk_": {"kind": "chunk", "producer": "webpack_bridge"},
    "env_missing,": {"kind": "environment_gap", "producer": "environment_plane"},
    "patch,": {"kind": "environment_patch", "producer": "environment_plane"},
}

EVIDENCE_STRENGTHS = {"strong", "weak", "marker_only", "diagnostic_only"}
DIAGNOSTIC_SEVERITIES = {"info", "warn", "error"}
FALLBACK_OUTCOMES = {"pass", "warn", "fail", "skip", "skipped"}

DIAGNOSTIC_CATALOG: Dict[str, Dict[str, str]] = {
    "TRACE_EMPTY": {"severity": "warn", "stage": "trace.parse"},
    "TRACE_PREFIX_UNKNOWN": {"severity": "warn", "stage": "trace.parse"},
    "TRACE_PARSE_PARTIAL": {"severity": "warn", "stage": "trace.parse"},
    "EVIDENCE_EXPECTED_MISSING": {"severity": "error", "stage": "entry.evaluate"},
    "EVIDENCE_MARKER_ONLY": {"severity": "warn", "stage": "entry.evaluate"},
    "CONFIDENCE_DOWNGRADED": {"severity": "warn", "stage": "entry.evaluate"},
    "POLICY_BLOCKED_ACTION": {"severity": "error", "stage": "policy.check"},
    "FALLBACK_USED": {"severity": "info", "stage": "entry.execute"},
    "FALLBACK_EXHAUSTED": {"severity": "error", "stage": "entry.execute"},
    "SOURCE_REGEX_PASS_THROUGH": {"severity": "warn", "stage": "entry.execute"},
    "SWITCHVM_MARKER_ONLY": {"severity": "warn", "stage": "entry.evaluate"},
    "ENVIRONMENT_GAP_OBSERVED": {"severity": "info", "stage": "environment.probe"},
    "ENVIRONMENT_PATCH_REJECTED": {"severity": "warn", "stage": "environment.patch"},
    "ENVIRONMENT_PATCH_UNSAFE": {"severity": "error", "stage": "environment.patch"},
}


@dataclass(slots=True)
class EvidenceRecord:
    """A normalized evidence envelope."""

    kind: str
    strength: str
    source: str
    stage: str
    summary: str
    producer: Optional[str] = None
    sample_kind: Optional[str] = None
    payload: Dict[str, Any] = field(default_factory=dict)

    def __post_init__(self) -> None:
        if self.strength not in EVIDENCE_STRENGTHS:
            raise ValueError(f"invalid evidence strength: {self.strength}")

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "EvidenceRecord":
        return cls(
            kind=data["kind"],
            strength=data["strength"],
            source=data["source"],
            stage=data["stage"],
            summary=data["summary"],
            producer=data.get("producer"),
            sample_kind=data.get("sample_kind"),
            payload=dict(data.get("payload", {})),
        )

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


@dataclass(slots=True)
class TraceEvent:
    """A normalized trace event envelope for raw or strategy-produced traces."""

    version: str
    kind: str
    prefix: str
    stage: str
    strategy_id: str
    sample_kind: str
    payload: Dict[str, Any] = field(default_factory=dict)
    source: Dict[str, Any] = field(default_factory=dict)
    confidence: str = "weak"

    @classmethod
    def from_raw(
        cls,
        raw: str,
        *,
        stage: str = "trace.parse",
        strategy_id: str = "unknown",
        sample_kind: str = "unknown",
    ) -> Optional["TraceEvent"]:
        meta = classify_trace_prefix(raw)
        if meta is None:
            return None
        payload = _payload_from_raw(meta["prefix"], raw)
        confidence = "strong" if meta["prefix"] in {"D,", "eval,", "fn_ctor,", "require,"} else "weak"
        if meta["kind"] in {"environment_gap"}:
            confidence = "none"
        return cls(
            version="1",
            kind=meta["kind"],
            prefix=meta["prefix"].rstrip(","),
            stage=stage,
            strategy_id=strategy_id,
            sample_kind=sample_kind,
            payload=payload,
            source={"raw": raw, "script_id": None, "line": None, "column": None},
            confidence=confidence,
        )

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "TraceEvent":
        return cls(
            version=data["version"],
            kind=data["kind"],
            prefix=data["prefix"],
            stage=data["stage"],
            strategy_id=data["strategy_id"],
            sample_kind=data["sample_kind"],
            payload=dict(data.get("payload", {})),
            source=dict(data.get("source", {})),
            confidence=data.get("confidence", "weak"),
        )

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


@dataclass(slots=True)
class EvidenceGateResult:
    """Unified PASS/WARN/FAIL decision for expected evidence."""

    status: str
    confidence: str
    satisfied: bool
    evidence: List[EvidenceRecord] = field(default_factory=list)
    diagnostics: List[DiagnosticRecord] = field(default_factory=list)

    def to_dict(self) -> Dict[str, Any]:
        data = asdict(self)
        data["evidence"] = [item.to_dict() for item in self.evidence]
        data["diagnostics"] = [item.to_dict() for item in self.diagnostics]
        return data


@dataclass(slots=True)
class DiagnosticRecord:
    """A normalized diagnostic envelope."""

    code: str
    severity: str
    stage: str
    message: str
    recovery_hint: Optional[str] = None
    strategy_id: Optional[str] = None
    sample_id: Optional[str] = None
    details: Dict[str, Any] = field(default_factory=dict)

    def __post_init__(self) -> None:
        if self.severity not in DIAGNOSTIC_SEVERITIES:
            raise ValueError(f"invalid diagnostic severity: {self.severity}")

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "DiagnosticRecord":
        return cls(
            code=data["code"],
            severity=data["severity"],
            stage=data["stage"],
            message=data["message"],
            recovery_hint=data.get("recovery_hint"),
            strategy_id=data.get("strategy_id"),
            sample_id=data.get("sample_id"),
            details=dict(data.get("details", {})),
        )

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


@dataclass(slots=True)
class FallbackAttempt:
    """A structured fallback attempt record."""

    strategy_id: str
    status: str
    reason: str
    next_strategy: Optional[str] = None
    diagnostics: List[DiagnosticRecord] = field(default_factory=list)
    evidence: List[EvidenceRecord] = field(default_factory=list)

    def __post_init__(self) -> None:
        if self.status not in FALLBACK_OUTCOMES:
            raise ValueError(f"invalid fallback outcome: {self.status}")

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "FallbackAttempt":
        return cls(
            strategy_id=data["strategy_id"],
            status=data["status"],
            reason=data["reason"],
            next_strategy=data.get("next_strategy"),
            diagnostics=[DiagnosticRecord.from_dict(item) for item in data.get("diagnostics", [])],
            evidence=[EvidenceRecord.from_dict(item) for item in data.get("evidence", [])],
        )

    def to_dict(self) -> Dict[str, Any]:
        data = asdict(self)
        data["diagnostics"] = [item.to_dict() for item in self.diagnostics]
        data["evidence"] = [item.to_dict() for item in self.evidence]
        return data


def classify_trace_prefix(raw: str) -> Optional[Dict[str, str]]:
    """Return registry metadata for a raw trace prefix, or None."""
    for prefix, meta in TRACE_PREFIX_REGISTRY.items():
        if raw.startswith(prefix):
            return {"prefix": prefix, **meta}
    return None


def build_trace_events(
    raw_trace: Iterable[str],
    *,
    stage: str = "trace.parse",
    strategy_id: str = "unknown",
    sample_kind: str = "unknown",
) -> List[TraceEvent]:
    """Convert known raw trace lines into normalized event envelopes."""
    events: List[TraceEvent] = []
    for raw in raw_trace:
        event = TraceEvent.from_raw(
            raw,
            stage=stage,
            strategy_id=strategy_id,
            sample_kind=sample_kind,
        )
        if event is not None:
            events.append(event)
    return events


def _payload_from_raw(prefix: str, raw: str) -> Dict[str, Any]:
    body = raw[len(prefix):]
    parts = body.split(",") if body else []
    if prefix == "D,":
        return {
            "pc": _int_or_raw(parts[0]) if len(parts) > 0 else None,
            "opcode": _int_or_raw(parts[1]) if len(parts) > 1 else None,
            "stack_depth": _int_or_raw(parts[2]) if len(parts) > 2 else None,
            "raw_fields": parts,
        }
    if prefix in {"R,", "W,"}:
        return {
            "pc": _int_or_raw(parts[0]) if len(parts) > 0 else None,
            "target": parts[1] if len(parts) > 1 else None,
            "value_kind": parts[2] if len(parts) > 2 else None,
            "raw_fields": parts,
        }
    if prefix == "C,":
        return {
            "pc": _int_or_raw(parts[0]) if len(parts) > 0 else None,
            "callee": parts[1] if len(parts) > 1 else None,
            "result_kind": parts[2] if len(parts) > 2 else None,
            "raw_fields": parts,
        }
    if prefix in {"eval,", "fn_ctor,"}:
        source = body
        return {"source_len": len(source), "raw_fields": parts}
    if prefix == "require,":
        return {
            "module_id": parts[0] if len(parts) > 0 else None,
            "parent_id": parts[1] if len(parts) > 1 else None,
            "raw_fields": parts,
        }
    if prefix == "chunk_":
        chunk_parts = raw.split(",", 1)
        return {
            "chunk_id": raw[len(prefix):].split(",", 1)[0] or None,
            "url": chunk_parts[1] if len(chunk_parts) > 1 else None,
            "raw_fields": parts,
        }
    if prefix == "env_missing,":
        return {"target": parts[0] if parts else None, "raw_fields": parts}
    if prefix == "patch,":
        return {
            "patch_id": parts[0] if len(parts) > 0 else None,
            "policy": parts[1] if len(parts) > 1 else None,
            "result": parts[2] if len(parts) > 2 else None,
            "raw_fields": parts,
        }
    return {"raw_fields": parts}


def _int_or_raw(value: str) -> Any:
    try:
        return int(value)
    except (TypeError, ValueError):
        return value


def _trace_payload_is_partial(prefix: str, raw: str) -> bool:
    if not raw.startswith(prefix):
        return False
    body = raw[len(prefix):]
    parts = body.split(",") if body else []
    minimum_fields = {
        "D,": 2,
        "R,": 2,
        "W,": 2,
        "C,": 1,
        "eval,": 1,
        "fn_ctor,": 1,
        "require,": 1,
        "chunk_": 1,
        "env_missing,": 1,
        "patch,": 3,
    }
    required = minimum_fields.get(prefix, 0)
    return len([part for part in parts if part != ""]) < required


def build_trace_diagnostics(raw_trace: Iterable[str]) -> List[DiagnosticRecord]:
    """Diagnose raw trace prefix compatibility without parsing every payload."""
    diagnostics: List[DiagnosticRecord] = []
    seen = False
    for idx, raw in enumerate(raw_trace):
        seen = True
        if not raw:
            diagnostics.append(DiagnosticRecord(
                code="TRACE_PARSE_PARTIAL",
                severity="warn",
                stage="trace.parse",
                message="empty trace line skipped",
                details={"index": idx},
            ))
            continue
        if classify_trace_prefix(raw) is None:
            diagnostics.append(DiagnosticRecord(
                code="TRACE_PREFIX_UNKNOWN",
                severity="warn",
                stage="trace.parse",
                message=f"unknown trace prefix: {raw.split(',', 1)[0]}",
                details={"index": idx, "raw": raw},
            ))
            continue
        meta = classify_trace_prefix(raw)
        if meta is not None and _trace_payload_is_partial(meta["prefix"], raw):
            diagnostics.append(DiagnosticRecord(
                code="TRACE_PARSE_PARTIAL",
                severity="warn",
                stage="trace.parse",
                message="known trace prefix has malformed or incomplete payload",
                details={"index": idx, "raw": raw, "prefix": meta["prefix"]},
            ))
    if not seen:
        diagnostics.append(DiagnosticRecord(
            code="TRACE_EMPTY",
            severity="warn",
            stage="trace.parse",
            message="trace is empty",
        ))
    return diagnostics


def build_evidence_diagnostics(
    expected: Iterable[str],
    observed: Iterable[EvidenceRecord],
    *,
    policy_blocked: bool = False,
) -> List[DiagnosticRecord]:
    """Diagnose missing, marker-only, and policy-blocked evidence gates."""
    observed_items = list(observed)
    by_kind: Dict[str, List[EvidenceRecord]] = {}
    for item in observed_items:
        by_kind.setdefault(item.kind, []).append(item)

    diagnostics: List[DiagnosticRecord] = []
    if policy_blocked:
        diagnostics.append(DiagnosticRecord(
            code="POLICY_BLOCKED_ACTION",
            severity="error",
            stage="policy.check",
            message="policy blocked an action required for this attempt",
            recovery_hint="choose a runtime-safe strategy or request explicit opt-in",
        ))

    for kind in expected:
        candidates = by_kind.get(kind, [])
        if not candidates:
            diagnostics.append(DiagnosticRecord(
                code="EVIDENCE_EXPECTED_MISSING",
                severity="error",
                stage="entry.evaluate",
                message=f"required evidence was not produced: {kind}",
                recovery_hint="collect runtime evidence or run a fallback strategy",
                details={"evidence_kind": kind},
            ))
        elif all(item.strength in {"marker_only", "diagnostic_only"} for item in candidates):
            diagnostics.append(DiagnosticRecord(
                code="EVIDENCE_MARKER_ONLY",
                severity="warn",
                stage="entry.evaluate",
                message=f"required evidence is marker-only: {kind}",
                recovery_hint="collect weak or strong evidence before claiming PASS",
                details={"evidence_kind": kind},
            ))
    return diagnostics


def evaluate_evidence_gate(
    expected: Iterable[str],
    observed: Iterable[EvidenceRecord],
    *,
    policy_blocked: bool = False,
) -> EvidenceGateResult:
    """Evaluate evidence against shared PASS/WARN/FAIL rules."""
    observed_items = list(observed)
    diagnostics = build_evidence_diagnostics(
        expected,
        observed_items,
        policy_blocked=policy_blocked,
    )
    confidence = confidence_from_evidence(observed_items)
    satisfied = evidence_satisfies(expected, observed_items)
    if policy_blocked or any(item.severity == "error" for item in diagnostics):
        status = "fail"
    elif satisfied:
        status = "pass"
    elif observed_items:
        status = "warn"
    else:
        status = "fail"
    return EvidenceGateResult(
        status=status,
        confidence=confidence,
        satisfied=satisfied and not policy_blocked,
        evidence=observed_items,
        diagnostics=diagnostics,
    )


def evidence_satisfies(expected: Iterable[str], observed: Iterable[EvidenceRecord]) -> bool:
    """Return True when all expected evidence kinds have non-marker evidence."""
    by_kind: Dict[str, List[EvidenceRecord]] = {}
    for item in observed:
        by_kind.setdefault(item.kind, []).append(item)

    for kind in expected:
        candidates = by_kind.get(kind, [])
        if not any(item.strength in {"strong", "weak"} for item in candidates):
            return False
    return True


def confidence_from_evidence(observed: Iterable[EvidenceRecord]) -> str:
    """Collapse evidence records to the shared confidence labels."""
    strengths = {item.strength for item in observed}
    if "strong" in strengths:
        return "strong"
    if "weak" in strengths:
        return "medium"
    if "marker_only" in strengths or "diagnostic_only" in strengths:
        return "weak"
    return "none"
