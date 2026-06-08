"""Shared experimental report/data model types.

This module defines the common envelope types for experimental
reports. It does NOT add behavior beyond schema validation and serialization.
Per-track schemas are preserved through extra_fields until S-002 onward.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any

__all__ = [
    "ExperimentalEvidenceRecord",
    "ExperimentalDiagnosticRecord",
    "ExperimentalReport",
    "experimental_report_from_dict",
    "experimental_report_to_dict",
    "experimental_report_roundtrip",
    "EXPERIMENTAL_SCHEMA_VERSIONS",
]

EXPERIMENTAL_SCHEMA_VERSIONS: frozenset[str] = frozenset({
    "environment-toolchain.v0.1",
    "environment-pressure.v0.1",
    "deobf-pass-registry.v0.1",
    "deobf-validation.v0.1",
    "deobf-string-array.v0.1",
    "vm-analysis.v0.1",
    "vm-handler-table.v0.1",
    "iv8-ir-node.v0.1",
})


@dataclass
class ExperimentalEvidenceRecord:
    """A single evidence record within an experimental report."""

    kind: str
    strength: str

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ExperimentalEvidenceRecord:
        return cls(kind=data["kind"], strength=data["strength"])

    def to_dict(self) -> dict[str, Any]:
        return {"kind": self.kind, "strength": self.strength}


@dataclass
class ExperimentalDiagnosticRecord:
    """A single diagnostic message within an experimental report."""

    code: str
    severity: str
    details: dict[str, Any] | None = None

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ExperimentalDiagnosticRecord:
        return cls(code=data["code"], severity=data["severity"], details=data.get("details"))

    def to_dict(self) -> dict[str, Any]:
        data = {"code": self.code, "severity": self.severity}
        if self.details is not None:
            data["details"] = self.details
        return data


@dataclass
class ExperimentalReport:
    """Common experimental report envelope.

    schema_version, evidence, and diagnostics are required.
    writes is optional (defaults to empty list).
    All other dict keys are preserved in extra_fields.
    """

    schema_version: str
    evidence: list[ExperimentalEvidenceRecord]
    diagnostics: list[ExperimentalDiagnosticRecord]
    writes: list[Any] | None = None
    extra_fields: dict[str, Any] = field(default_factory=dict)

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ExperimentalReport:
        known = {"schema_version", "evidence", "diagnostics", "writes"}
        extra = {k: v for k, v in data.items() if k not in known}
        writes = data.get("writes")
        return cls(
            schema_version=data["schema_version"],
            evidence=[ExperimentalEvidenceRecord.from_dict(e) for e in data.get("evidence", [])],
            diagnostics=[
                ExperimentalDiagnosticRecord.from_dict(d) for d in data.get("diagnostics", [])
            ],
            writes=writes,  # None if absent, [] if explicitly empty
            extra_fields=extra,
        )

    def to_dict(self) -> dict[str, Any]:
        d: dict[str, Any] = {
            "schema_version": self.schema_version,
            "evidence": [e.to_dict() for e in self.evidence],
            "diagnostics": [d_.to_dict() for d_ in self.diagnostics],
        }
        if self.writes is not None:
            d["writes"] = self.writes
        d.update(self.extra_fields)
        return d


def experimental_report_from_dict(data: dict[str, Any]) -> ExperimentalReport:
    """Convert a dict (e.g. from a JSON fixture) to a typed ExperimentalReport."""
    return ExperimentalReport.from_dict(data)


def experimental_report_to_dict(report: ExperimentalReport) -> dict[str, Any]:
    """Convert an ExperimentalReport back to a plain dict."""
    return report.to_dict()


def experimental_report_roundtrip(data: dict[str, Any]) -> dict[str, Any]:
    """Load a dict through ExperimentalReport types and back."""
    return experimental_report_to_dict(experimental_report_from_dict(data))
