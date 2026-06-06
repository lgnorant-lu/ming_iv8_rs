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
FALLBACK_OUTCOMES = {"pass", "warn", "fail", "skipped"}


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

    attempt_id: str
    strategy_id: str
    stage: str
    outcome: str
    reason: str
    evidence: List[EvidenceRecord] = field(default_factory=list)
    diagnostics: List[DiagnosticRecord] = field(default_factory=list)

    def __post_init__(self) -> None:
        if self.outcome not in FALLBACK_OUTCOMES:
            raise ValueError(f"invalid fallback outcome: {self.outcome}")

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "FallbackAttempt":
        return cls(
            attempt_id=data["attempt_id"],
            strategy_id=data["strategy_id"],
            stage=data["stage"],
            outcome=data["outcome"],
            reason=data["reason"],
            evidence=[EvidenceRecord.from_dict(item) for item in data.get("evidence", [])],
            diagnostics=[DiagnosticRecord.from_dict(item) for item in data.get("diagnostics", [])],
        )

    def to_dict(self) -> Dict[str, Any]:
        data = asdict(self)
        data["evidence"] = [item.to_dict() for item in self.evidence]
        data["diagnostics"] = [item.to_dict() for item in self.diagnostics]
        return data


def classify_trace_prefix(raw: str) -> Optional[Dict[str, str]]:
    """Return registry metadata for a raw trace prefix, or None."""
    for prefix, meta in TRACE_PREFIX_REGISTRY.items():
        if raw.startswith(prefix):
            return {"prefix": prefix, **meta}
    return None


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
    if not seen:
        diagnostics.append(DiagnosticRecord(
            code="TRACE_EMPTY",
            severity="warn",
            stage="trace.parse",
            message="trace is empty",
        ))
    return diagnostics


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
