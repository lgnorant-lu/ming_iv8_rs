"""Low-risk data models for Environment Toolchain runtime.

This module contains foundational dataclasses extracted from
`environment_toolchain_runtime.py`. It intentionally excludes probe/candidate
pack parsing models and runner orchestration.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass, field
from typing import Any

from iv8_rs.environment_toolchain_static import (
    _ALLOWED_PRESSURE_CATEGORIES,
    _ALLOWED_TARGET_FAMILIES,
)


@dataclass(frozen=True, slots=True)
class BoundaryDecision:
    decision: str
    reason: str
    redactions: list[str] = field(default_factory=list)
    blocked_terms: list[str] = field(default_factory=list)

    def __post_init__(self) -> None:
        if self.decision not in {"allowed", "blocked"}:
            raise ValueError(f"invalid boundary decision: {self.decision}")

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(frozen=True, slots=True)
class EnvironmentGap:
    probe_id: str
    target: str
    gap_class: str
    category: str
    expected: Any
    actual: Any
    error: str | None = None

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(frozen=True, slots=True)
class ProbeObservation:
    probe_id: str
    target: str
    category: str
    expected: Any
    actual: Any
    passed: bool
    gap_class: str
    evidence_ceiling: str = "diagnostic_only"
    error: str | None = None

    @classmethod
    def from_probe(
        cls,
        probe: Any,
        *,
        actual: Any,
        passed: bool,
        error: str | None = None,
    ) -> ProbeObservation:
        return cls(
            probe_id=probe.probe_id,
            target=probe.target,
            category=probe.category,
            expected=probe.expected,
            actual=actual,
            passed=passed,
            gap_class=probe.gap_class,
            evidence_ceiling=probe.evidence_ceiling,
            error=error,
        )

    def to_gap(self) -> EnvironmentGap | None:
        if self.passed:
            return None
        return EnvironmentGap(
            probe_id=self.probe_id,
            target=self.target,
            gap_class=self.gap_class,
            category=self.category,
            expected=self.expected,
            actual=self.actual,
            error=self.error,
        )

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(frozen=True, slots=True)
class ProbeRun:
    probe_pack: str
    observations: list[ProbeObservation]
    gaps: list[EnvironmentGap]
    coverage: dict[str, int]
    diagnostics: list[dict[str, Any]] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "probe_pack": self.probe_pack,
            "observations": [observation.to_dict() for observation in self.observations],
            "gaps": [gap.to_dict() for gap in self.gaps],
            "coverage": dict(self.coverage),
            "diagnostics": [dict(diagnostic) for diagnostic in self.diagnostics],
        }


@dataclass(frozen=True, slots=True)
class AdaptationIteration:
    index: int
    before: dict[str, int]
    after: dict[str, int]
    delta: dict[str, int]
    matched_patch_ids: list[str]
    applied_patch_ids: list[str]
    rejected: list[dict[str, Any]] = field(default_factory=list)
    stop_reason: str | None = None

    def to_details(self) -> dict[str, Any]:
        details: dict[str, Any] = {
            "index": self.index,
            "before": dict(self.before),
            "after": dict(self.after),
            "delta": dict(self.delta),
            "matched_patch_ids": list(self.matched_patch_ids),
            "applied_patch_ids": list(self.applied_patch_ids),
            "rejected": [dict(item) for item in self.rejected],
        }
        if self.stop_reason is not None:
            details["stop_reason"] = self.stop_reason
        return details


@dataclass(frozen=True, slots=True)
class ProfileCoherenceGroup:
    group_id: str
    status: str
    fields: dict[str, Any]
    sources: dict[str, str]
    reason: str
    review_status: str = "review_only"
    evidence_ceiling: str = "diagnostic_only"

    def __post_init__(self) -> None:
        if self.status not in {"consistent", "inconsistent", "unknown"}:
            raise ValueError(f"invalid profile coherence status: {self.status}")

    def to_details(self) -> dict[str, Any]:
        return {
            "group_id": self.group_id,
            "status": self.status,
            "fields": dict(self.fields),
            "sources": dict(self.sources),
            "reason": self.reason,
            "review_status": self.review_status,
            "evidence_ceiling": self.evidence_ceiling,
        }


@dataclass(frozen=True, slots=True)
class FamilyPressure:
    pressure_id: str
    category: str
    target_family: str
    gap_classes: list[str]
    review_status: str = "review_only"
    evidence_ceiling: str = "diagnostic_only"

    def __post_init__(self) -> None:
        if self.category not in _ALLOWED_PRESSURE_CATEGORIES:
            raise ValueError(f"invalid pressure category: {self.category}")
        if self.target_family not in _ALLOWED_TARGET_FAMILIES:
            raise ValueError(f"invalid target family: {self.target_family}")

    def to_details(self) -> dict[str, Any]:
        return {
            "pressure_id": self.pressure_id,
            "category": self.category,
            "target_family": self.target_family,
            "gap_classes": list(self.gap_classes),
            "review_status": self.review_status,
            "evidence_ceiling": self.evidence_ceiling,
        }


@dataclass(frozen=True, slots=True)
class AssetProvenance:
    asset_type: str
    pack_id: str
    origin: str
    version: int | None = None
    redacted_ref: str | None = None

    @property
    def diagnostic_code(self) -> str:
        asset = self.asset_type.upper().replace(" ", "_")
        origin = self.origin.upper()
        return f"ENV_TOOLCHAIN_{asset}_{origin}"
