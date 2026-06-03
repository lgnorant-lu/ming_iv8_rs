"""Dataclass wrappers for v0.6 Entry Plane result objects.

These provide typed access to the dict payloads returned by
prepare_entry() / run_with_entry(). The canonical schema lives
in the Rust types; these wrappers exist for Python ergonomics.
"""

from __future__ import annotations
from dataclasses import dataclass, field
from typing import List, Optional, Dict, Any


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

    @classmethod
    def from_dict(cls, d: dict) -> ExecutedStrategy:
        return cls(
            strategy_id=d["strategy_id"],
            phase_entered=d["phase_entered"],
            outcome=d["outcome"],
        )


@dataclass
class Diagnostics:
    sample_signals: List[str]
    selected_strategy_reason: Optional[str]
    fallback_attempts: List[str]
    policy_constraints: List[str]
    missing_capabilities: List[str]

    @classmethod
    def from_dict(cls, d: dict) -> Diagnostics:
        return cls(
            sample_signals=d.get("sample_signals", []),
            selected_strategy_reason=d.get("selected_strategy_reason"),
            fallback_attempts=d.get("fallback_attempts", []),
            policy_constraints=d.get("policy_constraints", []),
            missing_capabilities=d.get("missing_capabilities", []),
        )


@dataclass
class EntryPlan:
    plan_id: str
    persona: str
    sample_kind: str
    selected_strategy: SelectedStrategy
    state: str
    diagnostics: Diagnostics

    @classmethod
    def from_dict(cls, d: dict) -> EntryPlan:
        return cls(
            plan_id=d["plan_id"],
            persona=d["persona"],
            sample_kind=d["sample_kind"],
            selected_strategy=SelectedStrategy.from_dict(d["selected_strategy"]),
            state=d["state"],
            diagnostics=Diagnostics.from_dict(d.get("diagnostics", {})),
        )


@dataclass
class EntryResult:
    plan_id: str
    final_state: str
    selected_strategy: SelectedStrategy
    executed_strategies: List[ExecutedStrategy]
    trace: List[str]
    errors: List[dict]
    warnings: List[dict]
    diagnostics: Diagnostics

    @classmethod
    def from_dict(cls, d: dict) -> EntryResult:
        return cls(
            plan_id=d["plan_id"],
            final_state=d["final_state"],
            selected_strategy=SelectedStrategy.from_dict(d["selected_strategy"]),
            executed_strategies=[ExecutedStrategy.from_dict(e) for e in d.get("executed_strategies", [])],
            trace=d.get("trace", []),
            errors=d.get("errors", []),
            warnings=d.get("warnings", []),
            diagnostics=Diagnostics.from_dict(d.get("diagnostics", {})),
        )
