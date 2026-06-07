"""Environment Toolchain runtime surface.

Typed dataclasses for the environment-toolchain.v0.1 report schema.
Conversion preserves all fixture fields for round-trip compatibility.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from iv8_rs.experimental_report import ExperimentalDiagnosticRecord, ExperimentalEvidenceRecord

__all__ = [
    "CoverageSnapshot",
    "CoverageDelta",
    "ToolchainPatchEntry",
    "ProfileSuggestion",
    "EnvironmentToolchainReport",
    "toolchain_report_from_dict",
    "toolchain_report_to_dict",
]


@dataclass
class CoverageSnapshot:
    present: int
    missing: int
    mismatch: int

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> CoverageSnapshot:
        return cls(present=data["present"], missing=data["missing"], mismatch=data["mismatch"])

    def to_dict(self) -> dict[str, Any]:
        return {"present": self.present, "missing": self.missing, "mismatch": self.mismatch}


@dataclass
class CoverageDelta:
    improved: int
    regressed: int
    unresolved: int

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> CoverageDelta:
        return cls(
            improved=data["improved"],
            regressed=data["regressed"],
            unresolved=data["unresolved"],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "improved": self.improved,
            "regressed": self.regressed,
            "unresolved": self.unresolved,
        }


@dataclass
class ToolchainPatchEntry:
    patch_id: str
    target: str
    kind: str
    policy: str
    reason: str | None = None

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ToolchainPatchEntry:
        return cls(
            patch_id=data["patch_id"],
            target=data["target"],
            kind=data["kind"],
            policy=data["policy"],
            reason=data.get("reason"),
        )

    def to_dict(self) -> dict[str, Any]:
        d: dict[str, Any] = {
            "patch_id": self.patch_id,
            "target": self.target,
            "kind": self.kind,
            "policy": self.policy,
        }
        if self.reason is not None:
            d["reason"] = self.reason
        return d


@dataclass
class ProfileSuggestion:
    target: str
    value_preview: list[str]

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ProfileSuggestion:
        return cls(target=data["target"], value_preview=list(data["value_preview"]))

    def to_dict(self) -> dict[str, Any]:
        return {"target": self.target, "value_preview": self.value_preview}


@dataclass
class EnvironmentToolchainReport:
    schema_version: str
    probe_pack: str
    before: CoverageSnapshot
    after: CoverageSnapshot
    coverage_delta: CoverageDelta
    applied_patches: list[ToolchainPatchEntry]
    rejected_patches: list[ToolchainPatchEntry]
    profile_suggestions: list[ProfileSuggestion]
    evidence: list[ExperimentalEvidenceRecord]
    diagnostics: list[ExperimentalDiagnosticRecord]
    writes: list[Any] | None = None

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> EnvironmentToolchainReport:
        return cls(
            schema_version=data["schema_version"],
            probe_pack=data["probe_pack"],
            before=CoverageSnapshot.from_dict(data["before"]),
            after=CoverageSnapshot.from_dict(data["after"]),
            coverage_delta=CoverageDelta.from_dict(data["coverage_delta"]),
            applied_patches=[
                ToolchainPatchEntry.from_dict(p) for p in data.get("applied_patches", [])
            ],
            rejected_patches=[
                ToolchainPatchEntry.from_dict(p) for p in data.get("rejected_patches", [])
            ],
            profile_suggestions=[
                ProfileSuggestion.from_dict(s) for s in data.get("profile_suggestions", [])
            ],
            evidence=[ExperimentalEvidenceRecord.from_dict(e) for e in data.get("evidence", [])],
            diagnostics=[
                ExperimentalDiagnosticRecord.from_dict(d) for d in data.get("diagnostics", [])
            ],
            writes=data.get("writes"),
        )

    def to_dict(self) -> dict[str, Any]:
        d: dict[str, Any] = {
            "schema_version": self.schema_version,
            "probe_pack": self.probe_pack,
            "before": self.before.to_dict(),
            "after": self.after.to_dict(),
            "coverage_delta": self.coverage_delta.to_dict(),
            "applied_patches": [p.to_dict() for p in self.applied_patches],
            "rejected_patches": [p.to_dict() for p in self.rejected_patches],
            "profile_suggestions": [s.to_dict() for s in self.profile_suggestions],
            "evidence": [e.to_dict() for e in self.evidence],
            "diagnostics": [d_.to_dict() for d_ in self.diagnostics],
        }
        if self.writes is not None:
            d["writes"] = self.writes
        return d


def toolchain_report_from_dict(data: dict[str, Any]) -> EnvironmentToolchainReport:
    return EnvironmentToolchainReport.from_dict(data)


def toolchain_report_to_dict(report: EnvironmentToolchainReport) -> dict[str, Any]:
    return report.to_dict()
