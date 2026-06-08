"""Asset models for Environment Toolchain probe and candidate packs."""

from __future__ import annotations

from dataclasses import asdict, dataclass, field
from typing import Any

from iv8_rs.environment_toolchain_boundary import validate_bypass_boundary
from iv8_rs.environment_toolchain_static import (
    _ALLOWED_EVIDENCE_CEILINGS,
    _ALLOWED_PROBE_CATEGORIES,
    _CANDIDATE_DEPENDENCY_KINDS,
    _CANDIDATE_METADATA_FIELDS,
    _CANDIDATE_PLANNING_STATUSES,
    _ROLLBACK_ALLOWED_SCOPES,
    _ROLLBACK_BLOCKED_SCOPES,
)


@dataclass(frozen=True, slots=True)
class ToolchainCandidate:
    patch_id: str
    target: str
    target_family: str
    kind: str
    policy: str
    source: str
    value_preview: Any
    requires: list[str] = field(default_factory=list)
    risk_reasons: list[str] = field(default_factory=list)
    reversible: bool = True
    validation: dict[str, Any] = field(default_factory=dict)
    metadata: dict[str, Any] = field(default_factory=dict)

    def __post_init__(self) -> None:
        if self.policy != "runtime_safe":
            raise ValueError("slice 3 registry only exposes runtime_safe candidates")
        if not self.reversible:
            raise ValueError("runtime_safe candidates must be reversible")
        _validate_candidate_metadata(self.metadata)

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ToolchainCandidate:
        metadata = dict(data.get("metadata", {}))
        for field_name in _CANDIDATE_METADATA_FIELDS:
            if field_name in data:
                metadata[field_name] = data[field_name]
        return cls(
            patch_id=data["patch_id"],
            target=data["target"],
            target_family=data["target_family"],
            kind=data["kind"],
            policy=data["policy"],
            source=data.get("source", "builtin_registry"),
            value_preview=data.get("value_preview"),
            requires=list(data.get("requires", [])),
            risk_reasons=list(data.get("risk_reasons", [])),
            reversible=bool(data.get("reversible", True)),
            validation=dict(data.get("validation", {})),
            metadata=metadata,
        )

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(frozen=True, slots=True)
class CandidatePack:
    candidate_pack: str
    version: int
    description: str
    candidates: list[ToolchainCandidate]

    def __post_init__(self) -> None:
        if not self.candidate_pack:
            raise ValueError("candidate_pack must not be empty")
        if self.version < 1:
            raise ValueError("candidate pack version must be positive")
        patch_ids = [candidate.patch_id for candidate in self.candidates]
        duplicates = sorted({patch_id for patch_id in patch_ids if patch_ids.count(patch_id) > 1})
        if duplicates:
            raise ValueError(f"duplicate candidate patch ids: {duplicates}")

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> CandidatePack:
        return cls(
            candidate_pack=data["candidate_pack"],
            version=int(data["version"]),
            description=data["description"],
            candidates=[
                ToolchainCandidate.from_dict(candidate)
                for candidate in data.get("candidates", [])
            ],
        )

    def to_dict(self) -> dict[str, Any]:
        data = asdict(self)
        data["candidates"] = [candidate.to_dict() for candidate in self.candidates]
        return data


@dataclass(frozen=True, slots=True)
class ProbeDefinition:
    probe_id: str
    target: str
    category: str
    js: str
    expected: Any
    gap_class: str
    side_effects: list[str] = field(default_factory=list)
    cleanup: str = "none"
    evidence_ceiling: str = "diagnostic_only"

    def __post_init__(self) -> None:
        if not self.probe_id:
            raise ValueError("probe_id must not be empty")
        if not self.target:
            raise ValueError("target must not be empty")
        if self.category not in _ALLOWED_PROBE_CATEGORIES:
            raise ValueError(f"invalid probe category: {self.category}")
        if self.evidence_ceiling not in _ALLOWED_EVIDENCE_CEILINGS:
            raise ValueError(f"invalid evidence ceiling: {self.evidence_ceiling}")
        if self.evidence_ceiling == "weak":
            raise ValueError("probe definitions cannot claim weak evidence before runner review")
        if self.side_effects:
            raise ValueError("probe side effects are not supported before runner review")
        if self.cleanup != "none":
            raise ValueError("probe cleanup must be none before runner review")

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ProbeDefinition:
        return cls(
            probe_id=data["probe_id"],
            target=data["target"],
            category=data["category"],
            js=data["js"],
            expected=data["expected"],
            gap_class=data["gap_class"],
            side_effects=list(data.get("side_effects", [])),
            cleanup=data.get("cleanup", "none"),
            evidence_ceiling=data.get("evidence_ceiling", "diagnostic_only"),
        )

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(frozen=True, slots=True)
class ProbePack:
    probe_pack: str
    version: int
    description: str
    evidence_ceiling: str
    probes: list[ProbeDefinition]

    def __post_init__(self) -> None:
        if not self.probe_pack:
            raise ValueError("probe_pack must not be empty")
        if self.version < 1:
            raise ValueError("probe pack version must be positive")
        if self.evidence_ceiling != "diagnostic_only":
            raise ValueError("probe packs must be diagnostic_only before runner review")
        if not self.probes:
            raise ValueError("probe pack must contain at least one probe")
        probe_ids = [probe.probe_id for probe in self.probes]
        duplicates = sorted({probe_id for probe_id in probe_ids if probe_ids.count(probe_id) > 1})
        if duplicates:
            raise ValueError(f"duplicate probe ids: {duplicates}")

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ProbePack:
        return cls(
            probe_pack=data["probe_pack"],
            version=int(data["version"]),
            description=data["description"],
            evidence_ceiling=data["evidence_ceiling"],
            probes=[ProbeDefinition.from_dict(probe) for probe in data.get("probes", [])],
        )

    def to_dict(self) -> dict[str, Any]:
        data = asdict(self)
        data["probes"] = [probe.to_dict() for probe in self.probes]
        return data


def _validate_candidate_metadata(metadata: dict[str, Any]) -> None:
    if not metadata:
        return
    unknown_fields = sorted(set(metadata) - _CANDIDATE_METADATA_FIELDS)
    if unknown_fields:
        raise ValueError(f"unknown candidate metadata field: {unknown_fields}")
    decision = validate_bypass_boundary(metadata)
    if decision.decision == "blocked":
        raise ValueError("candidate metadata failed boundary validation")
    evidence_ceiling = metadata.get("evidence_ceiling")
    if evidence_ceiling is not None and evidence_ceiling not in _ALLOWED_EVIDENCE_CEILINGS:
        raise ValueError(f"invalid candidate metadata evidence ceiling: {evidence_ceiling}")
    planning_status = metadata.get("planning_status")
    if planning_status is not None and planning_status not in _CANDIDATE_PLANNING_STATUSES:
        raise ValueError(f"invalid candidate metadata planning status: {planning_status}")
    dependency_kinds = metadata.get("dependency_kind", [])
    if isinstance(dependency_kinds, str):
        dependency_kinds = [dependency_kinds]
    invalid_dependency_kinds = sorted(
        kind for kind in dependency_kinds if kind not in _CANDIDATE_DEPENDENCY_KINDS
    )
    if invalid_dependency_kinds:
        raise ValueError(f"invalid candidate metadata dependency kind: {invalid_dependency_kinds}")
    rollback_scope = metadata.get("rollback_scope")
    if rollback_scope is not None and rollback_scope not in (
        _ROLLBACK_ALLOWED_SCOPES | _ROLLBACK_BLOCKED_SCOPES
    ):
        raise ValueError(f"invalid candidate metadata rollback scope: {rollback_scope}")
