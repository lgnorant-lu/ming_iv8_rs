"""IV8 IR Node report skeleton.

Typed dataclasses for the iv8-ir-node.v0.1 report schema.
Analysis-only; does not rewrite source.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from iv8_rs.experimental_report import ExperimentalDiagnosticRecord, ExperimentalEvidenceRecord

__all__ = [
    "IRNode",
    "ConfidenceSummary",
    "IRNodeReport",
    "ir_node_report_from_dict",
    "ir_node_report_to_dict",
]


@dataclass
class IRNode:
    id: int
    kind: str
    source: str
    confidence: str

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> IRNode:
        return cls(
            id=data["id"],
            kind=data["kind"],
            source=data["source"],
            confidence=data["confidence"],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "id": self.id,
            "kind": self.kind,
            "source": self.source,
            "confidence": self.confidence,
        }


@dataclass
class ConfidenceSummary:
    diagnostic_only: int = 0
    weak: int = 0
    strong: int = 0

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ConfidenceSummary:
        return cls(
            diagnostic_only=data.get("diagnostic_only", 0),
            weak=data.get("weak", 0),
            strong=data.get("strong", 0),
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "diagnostic_only": self.diagnostic_only,
            "weak": self.weak,
            "strong": self.strong,
        }


@dataclass
class IRNodeReport:
    schema_version: str
    ir_kind: str
    node_count: int
    edge_count: int
    confidence_summary: ConfidenceSummary
    unsupported_features: list[str]
    nodes: list[IRNode]
    evidence: list[ExperimentalEvidenceRecord]
    diagnostics: list[ExperimentalDiagnosticRecord]

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> IRNodeReport:
        return cls(
            schema_version=data["schema_version"],
            ir_kind=data["ir_kind"],
            node_count=data["node_count"],
            edge_count=data["edge_count"],
            confidence_summary=ConfidenceSummary.from_dict(data["confidence_summary"]),
            unsupported_features=list(data.get("unsupported_features", [])),
            nodes=[IRNode.from_dict(n) for n in data.get("nodes", [])],
            evidence=[ExperimentalEvidenceRecord.from_dict(e) for e in data.get("evidence", [])],
            diagnostics=[
                ExperimentalDiagnosticRecord.from_dict(d) for d in data.get("diagnostics", [])
            ],
        )

    def to_dict(self) -> dict[str, Any]:
        return {
            "schema_version": self.schema_version,
            "ir_kind": self.ir_kind,
            "node_count": self.node_count,
            "edge_count": self.edge_count,
            "confidence_summary": self.confidence_summary.to_dict(),
            "unsupported_features": self.unsupported_features,
            "nodes": [n.to_dict() for n in self.nodes],
            "evidence": [e.to_dict() for e in self.evidence],
            "diagnostics": [d_.to_dict() for d_ in self.diagnostics],
        }


def ir_node_report_from_dict(data: dict[str, Any]) -> IRNodeReport:
    return IRNodeReport.from_dict(data)


def ir_node_report_to_dict(report: IRNodeReport) -> dict[str, Any]:
    return report.to_dict()
