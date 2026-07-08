"""VM Analysis and Handler extraction report skeletons.

Typed dataclasses for the vm-analysis.v0.1 and vm-handler-table.v0.1
report schemas. Confidence-preserving evidence fields only; no decoded
opcode semantics.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from iv8_rs.experimental_report import ExperimentalDiagnosticRecord, ExperimentalEvidenceRecord

__all__ = [
    "HandlerTableSummary",
    "TraceSummary",
    "StateModel",
    "OpcodeHint",
    "VMAnalysisReport",
    "vm_analysis_report_from_dict",
    "vm_analysis_report_to_dict",
    "HandlerEntry",
    "BytecodeCandidate",
    "VMHandlerTable",
    "vm_handler_table_from_dict",
    "vm_handler_table_to_dict",
]

# VM Analysis


@dataclass
class HandlerTableSummary:
    kind: str
    handler_count: int
    ids: list[str]
    source_available: bool
    runtime_validated: bool
    handlers: list[dict[str, Any]]

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> HandlerTableSummary:
        return cls(
            kind=data["kind"],
            handler_count=data["handler_count"],
            ids=list(data["ids"]),
            source_available=data["source_available"],
            runtime_validated=data["runtime_validated"],
            handlers=list(data.get("handlers", [])),
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": self.kind,
            "handler_count": self.handler_count,
            "ids": self.ids,
            "source_available": self.source_available,
            "runtime_validated": self.runtime_validated,
            "handlers": self.handlers,
        }


@dataclass
class TraceSummary:
    opcode_sequence_observed: bool

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> TraceSummary:
        return cls(opcode_sequence_observed=data.get("opcode_sequence_observed", False))

    def to_dict(self) -> dict[str, Any]:
        return {"opcode_sequence_observed": self.opcode_sequence_observed}


@dataclass
class StateModel:
    hints: list[str]

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> StateModel:
        return cls(hints=list(data.get("hints", [])))

    def to_dict(self) -> dict[str, Any]:
        return {"hints": self.hints}


@dataclass
class OpcodeHint:
    label: str
    confidence: str
    evidence: list[str]

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> OpcodeHint:
        return cls(
            label=data["label"],
            confidence=data["confidence"],
            evidence=list(data.get("evidence", [])),
        )

    def to_dict(self) -> dict[str, Any]:
        return {"label": self.label, "confidence": self.confidence, "evidence": self.evidence}


@dataclass
class VMAnalysisReport:
    schema_version: str
    sample_id: str
    vm_family: str
    dispatch_variant: str
    handler_table: HandlerTableSummary | None
    bytecode: Any | None
    trace_summary: TraceSummary
    state_model: StateModel
    opcode_map: dict[str, OpcodeHint]
    evidence: list[ExperimentalEvidenceRecord]
    diagnostics: list[ExperimentalDiagnosticRecord]

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> VMAnalysisReport:
        ht = data.get("handler_table")
        return cls(
            schema_version=data["schema_version"],
            sample_id=data["sample_id"],
            vm_family=data["vm_family"],
            dispatch_variant=data["dispatch_variant"],
            handler_table=HandlerTableSummary.from_dict(ht) if ht else None,
            bytecode=data.get("bytecode"),
            trace_summary=TraceSummary.from_dict(data.get("trace_summary", {})),
            state_model=StateModel.from_dict(data.get("state_model", {})),
            opcode_map={k: OpcodeHint.from_dict(v) for k, v in data.get("opcode_map", {}).items()},
            evidence=[ExperimentalEvidenceRecord.from_dict(e) for e in data.get("evidence", [])],
            diagnostics=[
                ExperimentalDiagnosticRecord.from_dict(d) for d in data.get("diagnostics", [])
            ],
        )

    def to_dict(self) -> dict[str, Any]:
        d: dict[str, Any] = {
            "schema_version": self.schema_version,
            "sample_id": self.sample_id,
            "vm_family": self.vm_family,
            "dispatch_variant": self.dispatch_variant,
            "handler_table": self.handler_table.to_dict() if self.handler_table else None,
            "bytecode": self.bytecode,
            "trace_summary": self.trace_summary.to_dict(),
            "state_model": self.state_model.to_dict(),
            "opcode_map": {k: v.to_dict() for k, v in self.opcode_map.items()},
            "evidence": [e.to_dict() for e in self.evidence],
            "diagnostics": [d_.to_dict() for d_ in self.diagnostics],
        }
        return d


def vm_analysis_report_from_dict(data: dict[str, Any]) -> VMAnalysisReport:
    return VMAnalysisReport.from_dict(data)


def vm_analysis_report_to_dict(report: VMAnalysisReport) -> dict[str, Any]:
    return report.to_dict()


# VM Handler Table


@dataclass
class HandlerEntry:
    handler_id: str
    source_hash: str
    observed_opcodes: list[Any]
    effects: list[Any]
    confidence: str

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> HandlerEntry:
        return cls(
            handler_id=data["handler_id"],
            source_hash=data["source_hash"],
            observed_opcodes=list(data.get("observed_opcodes", [])),
            effects=list(data.get("effects", [])),
            confidence=data["confidence"],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "handler_id": self.handler_id,
            "source_hash": self.source_hash,
            "observed_opcodes": self.observed_opcodes,
            "effects": self.effects,
            "confidence": self.confidence,
        }


@dataclass
class BytecodeCandidate:
    kind: str
    length: int
    numeric_ratio: float
    runtime_validated: bool

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> BytecodeCandidate:
        return cls(
            kind=data["kind"],
            length=data["length"],
            numeric_ratio=data["numeric_ratio"],
            runtime_validated=data["runtime_validated"],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "kind": self.kind,
            "length": self.length,
            "numeric_ratio": self.numeric_ratio,
            "runtime_validated": self.runtime_validated,
        }


@dataclass
class VMHandlerTable:
    schema_version: str
    kind: str
    handler_count: int
    ids: list[str]
    source_available: bool
    runtime_validated: bool
    extraction_quality: str
    shape_score: int
    handlers: list[HandlerEntry]
    bytecode_candidates: list[BytecodeCandidate]
    evidence: list[ExperimentalEvidenceRecord]
    diagnostics: list[ExperimentalDiagnosticRecord]

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> VMHandlerTable:
        return cls(
            schema_version=data["schema_version"],
            kind=data["kind"],
            handler_count=data["handler_count"],
            ids=list(data["ids"]),
            source_available=data["source_available"],
            runtime_validated=data["runtime_validated"],
            extraction_quality=data["extraction_quality"],
            shape_score=data["shape_score"],
            handlers=[HandlerEntry.from_dict(h) for h in data.get("handlers", [])],
            bytecode_candidates=[
                BytecodeCandidate.from_dict(b) for b in data.get("bytecode_candidates", [])
            ],
            evidence=[ExperimentalEvidenceRecord.from_dict(e) for e in data.get("evidence", [])],
            diagnostics=[
                ExperimentalDiagnosticRecord.from_dict(d) for d in data.get("diagnostics", [])
            ],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "schema_version": self.schema_version,
            "kind": self.kind,
            "handler_count": self.handler_count,
            "ids": self.ids,
            "source_available": self.source_available,
            "runtime_validated": self.runtime_validated,
            "extraction_quality": self.extraction_quality,
            "shape_score": self.shape_score,
            "handlers": [h.to_dict() for h in self.handlers],
            "bytecode_candidates": [b.to_dict() for b in self.bytecode_candidates],
            "evidence": [e.to_dict() for e in self.evidence],
            "diagnostics": [d_.to_dict() for d_ in self.diagnostics],
        }


def vm_handler_table_from_dict(data: dict[str, Any]) -> VMHandlerTable:
    return VMHandlerTable.from_dict(data)


def vm_handler_table_to_dict(report: VMHandlerTable) -> dict[str, Any]:
    return report.to_dict()
