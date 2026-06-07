"""Deobfuscation registry and validation runtime surface.

Typed dataclasses for the deobf-pass-registry.v0.1 and
deobf-validation.v0.1 report schemas.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any

from iv8_rs.experimental_report import ExperimentalDiagnosticRecord, ExperimentalEvidenceRecord

__all__ = [
    "RegistryEntry",
    "SelectionReport",
    "DeobfRegistryReport",
    "registry_report_from_dict",
    "registry_report_to_dict",
    "ValidationCheck",
    "ValidationReport",
    "validation_report_from_dict",
    "validation_report_to_dict",
]

# Deobf Pass Registry


@dataclass
class RegistryEntry:
    pass_id: str
    name: str
    family: str
    level: str
    enabled_by_default: bool
    inputs: list[str]
    outputs: list[str]
    requires: list[str]
    policy_required: str
    evidence_ceiling: str
    validation_required: bool
    quality_gate: str

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> RegistryEntry:
        return cls(
            pass_id=data["pass_id"],
            name=data["name"],
            family=data["family"],
            level=data["level"],
            enabled_by_default=data["enabled_by_default"],
            inputs=list(data["inputs"]),
            outputs=list(data["outputs"]),
            requires=list(data.get("requires", [])),
            policy_required=data["policy_required"],
            evidence_ceiling=data["evidence_ceiling"],
            validation_required=data["validation_required"],
            quality_gate=data["quality_gate"],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "pass_id": self.pass_id,
            "name": self.name,
            "family": self.family,
            "level": self.level,
            "enabled_by_default": self.enabled_by_default,
            "inputs": self.inputs,
            "outputs": self.outputs,
            "requires": self.requires,
            "policy_required": self.policy_required,
            "evidence_ceiling": self.evidence_ceiling,
            "validation_required": self.validation_required,
            "quality_gate": self.quality_gate,
        }


@dataclass
class SelectionReport:
    selected: list[str]
    rejected: list[dict[str, str]]

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> SelectionReport:
        return cls(
            selected=list(data["selected"]),
            rejected=[dict(item) for item in data.get("rejected", [])],
        )

    def to_dict(self) -> dict[str, Any]:
        return {"selected": self.selected, "rejected": self.rejected}


@dataclass
class DeobfRegistryReport:
    schema_version: str
    entries: list[RegistryEntry]
    selection_report: SelectionReport
    evidence: list[ExperimentalEvidenceRecord]
    diagnostics: list[ExperimentalDiagnosticRecord]

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> DeobfRegistryReport:
        return cls(
            schema_version=data["schema_version"],
            entries=[RegistryEntry.from_dict(e) for e in data["entries"]],
            selection_report=SelectionReport.from_dict(data["selection_report"]),
            evidence=[ExperimentalEvidenceRecord.from_dict(e) for e in data.get("evidence", [])],
            diagnostics=[
                ExperimentalDiagnosticRecord.from_dict(d) for d in data.get("diagnostics", [])
            ],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "schema_version": self.schema_version,
            "entries": [e.to_dict() for e in self.entries],
            "selection_report": self.selection_report.to_dict(),
            "evidence": [e.to_dict() for e in self.evidence],
            "diagnostics": [d_.to_dict() for d_ in self.diagnostics],
        }


def registry_report_from_dict(data: dict[str, Any]) -> DeobfRegistryReport:
    return DeobfRegistryReport.from_dict(data)


def registry_report_to_dict(report: DeobfRegistryReport) -> dict[str, Any]:
    return report.to_dict()


# Deobf Validation


@dataclass
class ValidationCheck:
    kind: str
    result: str

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ValidationCheck:
        return cls(kind=data["kind"], result=data["result"])

    def to_dict(self) -> dict[str, Any]:
        return {"kind": self.kind, "result": self.result}


@dataclass
class PolicyFailureCase:
    pass_id: str
    level: str
    policy_status: str
    diagnostics: list[ExperimentalDiagnosticRecord]
    evidence: list[ExperimentalEvidenceRecord]

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> PolicyFailureCase:
        return cls(
            pass_id=data["pass_id"],
            level=data["level"],
            policy_status=data["policy_status"],
            diagnostics=[
                ExperimentalDiagnosticRecord.from_dict(d) for d in data.get("diagnostics", [])
            ],
            evidence=[ExperimentalEvidenceRecord.from_dict(e) for e in data.get("evidence", [])],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "pass_id": self.pass_id,
            "level": self.level,
            "policy_status": self.policy_status,
            "diagnostics": [d.to_dict() for d in self.diagnostics],
            "evidence": [e.to_dict() for e in self.evidence],
        }


@dataclass
class ValidationReport:
    schema_version: str
    source_id: str
    pass_id: str
    input_hash: str
    output_hash: str
    level: str
    policy_status: str
    checks: list[ValidationCheck]
    evidence: list[ExperimentalEvidenceRecord]
    diagnostics: list[ExperimentalDiagnosticRecord]
    negative_cases: list[PolicyFailureCase] = field(default_factory=list)

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ValidationReport:
        return cls(
            schema_version=data["schema_version"],
            source_id=data["source_id"],
            pass_id=data["pass_id"],
            input_hash=data["input_hash"],
            output_hash=data["output_hash"],
            level=data["level"],
            policy_status=data["policy_status"],
            checks=[ValidationCheck.from_dict(c) for c in data.get("checks", [])],
            evidence=[ExperimentalEvidenceRecord.from_dict(e) for e in data.get("evidence", [])],
            diagnostics=[
                ExperimentalDiagnosticRecord.from_dict(d) for d in data.get("diagnostics", [])
            ],
            negative_cases=[PolicyFailureCase.from_dict(n) for n in data.get("negative_cases", [])],
        )

    def to_dict(self) -> dict[str, Any]:
        d: dict[str, Any] = {
            "schema_version": self.schema_version,
            "source_id": self.source_id,
            "pass_id": self.pass_id,
            "input_hash": self.input_hash,
            "output_hash": self.output_hash,
            "level": self.level,
            "policy_status": self.policy_status,
            "checks": [c.to_dict() for c in self.checks],
            "evidence": [e.to_dict() for e in self.evidence],
            "diagnostics": [d_.to_dict() for d_ in self.diagnostics],
        }
        if self.negative_cases:
            d["negative_cases"] = [n.to_dict() for n in self.negative_cases]
        return d


def validation_report_from_dict(data: dict[str, Any]) -> ValidationReport:
    return ValidationReport.from_dict(data)


def validation_report_to_dict(report: ValidationReport) -> dict[str, Any]:
    return report.to_dict()
