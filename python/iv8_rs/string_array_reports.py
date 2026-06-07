"""String Array deobfuscation locator skeleton.

Typed dataclasses for the deobf-string-array.v0.1 report schema.
Static markers only; no decoder execution.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from iv8_rs.experimental_report import ExperimentalDiagnosticRecord, ExperimentalEvidenceRecord

__all__ = [
    "StringArrayCandidate",
    "RotationIIFE",
    "StringDecoder",
    "ReplacementSite",
    "StringArrayReport",
    "string_array_report_from_dict",
    "string_array_report_to_dict",
]


@dataclass
class StringArrayCandidate:
    kind: str
    binding_id: str
    element_count: int
    string_ratio: float
    confidence: str

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> StringArrayCandidate:
        return cls(
            kind=data["kind"],
            binding_id=data["binding_id"],
            element_count=data["element_count"],
            string_ratio=data["string_ratio"],
            confidence=data["confidence"],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": self.kind,
            "binding_id": self.binding_id,
            "element_count": self.element_count,
            "string_ratio": self.string_ratio,
            "confidence": self.confidence,
        }


@dataclass
class RotationIIFE:
    binding_id: str
    confidence: str

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> RotationIIFE:
        return cls(binding_id=data["binding_id"], confidence=data["confidence"])

    def to_dict(self) -> dict[str, Any]:
        return {"binding_id": self.binding_id, "confidence": self.confidence}


@dataclass
class StringDecoder:
    binding_id: str
    param_count: int
    confidence: str

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> StringDecoder:
        return cls(
            binding_id=data["binding_id"],
            param_count=data["param_count"],
            confidence=data["confidence"],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "binding_id": self.binding_id,
            "param_count": self.param_count,
            "confidence": self.confidence,
        }


@dataclass
class ReplacementSite:
    callee_binding_id: str
    argument_kind: str

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ReplacementSite:
        return cls(callee_binding_id=data["callee_binding_id"], argument_kind=data["argument_kind"])

    def to_dict(self) -> dict[str, Any]:
        return {"callee_binding_id": self.callee_binding_id, "argument_kind": self.argument_kind}


@dataclass
class StringArrayReport:
    schema_version: str
    arrays: list[StringArrayCandidate]
    rotation_iifes: list[RotationIIFE]
    decoders: list[StringDecoder]
    replacement_sites: list[ReplacementSite]
    status: str
    source_rewritten: bool
    evidence: list[ExperimentalEvidenceRecord]
    diagnostics: list[ExperimentalDiagnosticRecord]

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> StringArrayReport:
        return cls(
            schema_version=data["schema_version"],
            arrays=[StringArrayCandidate.from_dict(a) for a in data.get("arrays", [])],
            rotation_iifes=[RotationIIFE.from_dict(r) for r in data.get("rotation_iifes", [])],
            decoders=[StringDecoder.from_dict(d) for d in data.get("decoders", [])],
            replacement_sites=[
                ReplacementSite.from_dict(s) for s in data.get("replacement_sites", [])
            ],
            status=data["status"],
            source_rewritten=data["source_rewritten"],
            evidence=[ExperimentalEvidenceRecord.from_dict(e) for e in data.get("evidence", [])],
            diagnostics=[
                ExperimentalDiagnosticRecord.from_dict(d) for d in data.get("diagnostics", [])
            ],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "schema_version": self.schema_version,
            "arrays": [a.to_dict() for a in self.arrays],
            "rotation_iifes": [r.to_dict() for r in self.rotation_iifes],
            "decoders": [d.to_dict() for d in self.decoders],
            "replacement_sites": [s.to_dict() for s in self.replacement_sites],
            "status": self.status,
            "source_rewritten": self.source_rewritten,
            "evidence": [e.to_dict() for e in self.evidence],
            "diagnostics": [d_.to_dict() for d_ in self.diagnostics],
        }


def string_array_report_from_dict(data: dict[str, Any]) -> StringArrayReport:
    return StringArrayReport.from_dict(data)


def string_array_report_to_dict(report: StringArrayReport) -> dict[str, Any]:
    return report.to_dict()
