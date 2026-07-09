"""Dataclass wrappers for v0.6/v0.7 Entry Plane result objects.

These provide typed access to the dict payloads returned by
prepare_entry() / run_with_entry(). The canonical schema lives
in the Rust types; these wrappers exist for Python ergonomics.
"""

from __future__ import annotations

from dataclasses import dataclass, field

from iv8_rs.diagnostics import DiagnosticRecord, EvidenceRecord, FallbackAttempt


@dataclass
class SelectedStrategy:
    strategy_id: str
    strategy_kind: str
    selection_reason: str

    @classmethod
    def from_dict(cls, d: dict) -> SelectedStrategy:
        return cls(
            strategy_id=d["strategy_id"],
            strategy_kind=d["strategy_kind"],
            selection_reason=d["selection_reason"],
        )


@dataclass
class ExecutedStrategy:
    strategy_id: str
    phase_entered: str
    outcome: str
    diagnostics: list[DiagnosticRecord] = field(default_factory=list)

    @classmethod
    def from_dict(cls, d: dict) -> ExecutedStrategy:
        return cls(
            strategy_id=d["strategy_id"],
            phase_entered=d["phase_entered"],
            outcome=d["outcome"],
            diagnostics=[DiagnosticRecord.from_dict(e) for e in d.get("diagnostics", [])],
        )


@dataclass
class ProbeResult:
    can_swc_parse: bool = False
    has_dispatch_pattern: bool = False
    has_webpack_runtime: bool = False
    has_closure_capture: bool = False
    has_eval_heavy: bool = False
    is_low_obfuscation: bool = False

    @classmethod
    def from_dict(cls, d: dict) -> ProbeResult:
        return cls(
            can_swc_parse=d.get("can_swc_parse", False),
            has_dispatch_pattern=d.get("has_dispatch_pattern", False),
            has_webpack_runtime=d.get("has_webpack_runtime", False),
            has_closure_capture=d.get("has_closure_capture", False),
            has_eval_heavy=d.get("has_eval_heavy", False),
            is_low_obfuscation=d.get("is_low_obfuscation", False),
        )


@dataclass
class EventMeta:
    source_kind: str
    strategy_id: str
    phase: str
    confidence: float = 0.0
    module_id: int | None = None
    chunk_id: str | None = None

    @classmethod
    def from_dict(cls, d: dict) -> EventMeta:
        return cls(
            source_kind=d["source_kind"],
            strategy_id=d["strategy_id"],
            phase=d["phase"],
            confidence=d.get("confidence", 0.0),
            module_id=d.get("module_id"),
            chunk_id=d.get("chunk_id"),
        )


@dataclass
class Diagnostics:
    sample_signals: list[str]
    selected_strategy_reason: str | None
    fallback_attempts: list[FallbackAttempt]
    policy_constraints: list[str]
    missing_capabilities: list[str]
    diagnostic_records: list[DiagnosticRecord] = field(default_factory=list)
    observed_evidence: list[EvidenceRecord] = field(default_factory=list)
    activation_timing: str | None = None
    reload_reason: str | None = None
    collection_summary: str | None = None
    cleanup_summary: str | None = None

    @classmethod
    def from_dict(cls, d: dict) -> Diagnostics:
        # Accept pre-constructed Diagnostics object from Rust (not a dict)
        if not isinstance(d, dict):
            return d
        return cls(
            sample_signals=d.get("sample_signals", []),
            selected_strategy_reason=d.get("selected_strategy_reason"),
            fallback_attempts=[FallbackAttempt.from_dict(f) for f in d.get("fallback_attempts", [])],
            policy_constraints=d.get("policy_constraints", []),
            missing_capabilities=d.get("missing_capabilities", []),
            diagnostic_records=[DiagnosticRecord.from_dict(r) for r in d.get("diagnostic_records", [])],
            observed_evidence=[EvidenceRecord.from_dict(e) for e in d.get("observed_evidence", [])],
            activation_timing=d.get("activation_timing"),
            reload_reason=d.get("reload_reason"),
            collection_summary=d.get("collection_summary"),
            cleanup_summary=d.get("cleanup_summary"),
        )


@dataclass
class EntryPlan:
    plan_id: str
    persona: str
    sample_kind: str
    selected_strategy: SelectedStrategy
    state: str
    diagnostics: Diagnostics
    sample_signals: list[str] = field(default_factory=list)
    expected_evidence: list[str] = field(default_factory=list)
    fallback_chain: list[str] = field(default_factory=list)
    risk_level: str = "low"
    requires_preload: bool = False
    requires_reload: bool = False

    @classmethod
    def from_dict(cls, d: dict) -> EntryPlan:
        return cls(
            plan_id=d["plan_id"],
            persona=d["persona"],
            sample_kind=d["sample_kind"],
            selected_strategy=SelectedStrategy.from_dict(d["selected_strategy"]),
            state=d["state"],
            diagnostics=Diagnostics.from_dict(d.get("diagnostics", {})),
            sample_signals=d.get("sample_signals", []),
            expected_evidence=d.get("expected_evidence", []),
            fallback_chain=d.get("fallback_chain", []),
            risk_level=d.get("risk_level", "low"),
            requires_preload=d.get("requires_preload", False),
            requires_reload=d.get("requires_reload", False),
        )


@dataclass
class TraceMeta:
    trace_format: str
    plan_id: str
    persona: str
    sample_kind: str
    selected_strategy_id: str
    executed_strategy_ids: list[str]
    trace_sources: list[str]
    events: dict[int, EventMeta] = field(default_factory=dict)

    @classmethod
    def from_dict(cls, d: dict) -> TraceMeta:
        raw_events = d.get("events", {})
        events = {}
        for k, v in raw_events.items():
            try:
                events[int(k)] = EventMeta.from_dict(v)
            except (ValueError, KeyError):
                continue
        return cls(
            trace_format=d["trace_format"],
            plan_id=d["plan_id"],
            persona=d["persona"],
            sample_kind=d["sample_kind"],
            selected_strategy_id=d["selected_strategy_id"],
            executed_strategy_ids=d.get("executed_strategy_ids", []),
            trace_sources=d.get("trace_sources", []),
            events=events,
        )


@dataclass
class EntryResult:
    plan_id: str
    final_state: str
    selected_strategy: SelectedStrategy
    executed_strategies: list[ExecutedStrategy]
    trace: list[str]
    diagnostic_records: list[DiagnosticRecord]
    observed_evidence: list[EvidenceRecord]
    diagnostics: Diagnostics
    trace_meta: TraceMeta | None = None
    module_graph: dict | None = None
    hook_report: dict | None = None
    environment_report: dict | None = None
    cleanup_state: dict | None = None

    @classmethod
    def from_dict(cls, d: dict) -> EntryResult:
        return cls(
            plan_id=d["plan_id"],
            final_state=d["final_state"],
            selected_strategy=SelectedStrategy.from_dict(d["selected_strategy"]),
            executed_strategies=[ExecutedStrategy.from_dict(e) for e in d.get("executed_strategies", [])],
            trace=d.get("trace", []),
            diagnostic_records=[DiagnosticRecord.from_dict(r) for r in d.get("diagnostic_records", [])],
            observed_evidence=[EvidenceRecord.from_dict(e) for e in d.get("observed_evidence", [])],
            diagnostics=Diagnostics.from_dict(d.get("diagnostics", {})),
            trace_meta=TraceMeta.from_dict(d["trace_meta"]) if d.get("trace_meta") else None,
            module_graph=d.get("module_graph"),
            hook_report=d.get("hook_report"),
            environment_report=d.get("environment_report"),
            cleanup_state=d.get("cleanup_state"),
        )
