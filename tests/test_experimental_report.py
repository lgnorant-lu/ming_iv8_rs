from __future__ import annotations

import pytest

pytest.importorskip("iv8_rs")

from iv8_rs.experimental_report import (
    EXPERIMENTAL_SCHEMA_VERSIONS,
    ExperimentalDiagnosticRecord,
    ExperimentalEvidenceRecord,
    ExperimentalReport,
    experimental_report_from_dict,
    experimental_report_roundtrip,
    experimental_report_to_dict,
)


class TestExperimentalEvidenceRecord:
    def test_construct(self):
        r = ExperimentalEvidenceRecord(kind="deobfuscation", strength="strong")
        assert r.kind == "deobfuscation"
        assert r.strength == "strong"

    def test_to_dict(self):
        r = ExperimentalEvidenceRecord(kind="deobfuscation", strength="moderate")
        assert r.to_dict() == {"kind": "deobfuscation", "strength": "moderate"}

    def test_from_dict(self):
        r = ExperimentalEvidenceRecord.from_dict({"kind": "x", "strength": "weak"})
        assert r.kind == "x"
        assert r.strength == "weak"

    def test_roundtrip(self):
        d = {"kind": "analysis", "strength": "strong"}
        r = ExperimentalEvidenceRecord.from_dict(d)
        assert r.to_dict() == d


class TestExperimentalDiagnosticRecord:
    def test_construct_with_details(self):
        d = ExperimentalDiagnosticRecord(
            code="E001", severity="error", details={"line": 42}
        )
        assert d.code == "E001"
        assert d.severity == "error"
        assert d.details == {"line": 42}

    def test_construct_without_details(self):
        d = ExperimentalDiagnosticRecord(code="W002", severity="warning")
        assert d.code == "W002"
        assert d.severity == "warning"
        assert d.details is None

    def test_construct_with_none_details(self):
        d = ExperimentalDiagnosticRecord(code="I003", severity="info", details=None)
        assert d.details is None

    def test_to_dict_with_details(self):
        d = ExperimentalDiagnosticRecord(code="E001", severity="error", details={"x": 1})
        assert d.to_dict() == {"code": "E001", "severity": "error", "details": {"x": 1}}

    def test_to_dict_without_details(self):
        d = ExperimentalDiagnosticRecord(code="W002", severity="warning")
        assert d.to_dict() == {"code": "W002", "severity": "warning"}

    def test_to_dict_with_none_details(self):
        d = ExperimentalDiagnosticRecord(code="I003", severity="info", details=None)
        assert d.to_dict() == {"code": "I003", "severity": "info"}

    def test_from_dict_with_details(self):
        d = ExperimentalDiagnosticRecord.from_dict(
            {"code": "E001", "severity": "error", "details": {"x": 1}}
        )
        assert d.code == "E001"
        assert d.details == {"x": 1}

    def test_from_dict_without_details(self):
        d = ExperimentalDiagnosticRecord.from_dict({"code": "W002", "severity": "warning"})
        assert d.details is None

    def test_from_dict_with_none_details(self):
        d = ExperimentalDiagnosticRecord.from_dict(
            {"code": "I003", "severity": "info", "details": None}
        )
        assert d.details is None

    def test_roundtrip_with_details(self):
        data = {"code": "C001", "severity": "critical", "details": {"file": "a.js"}}
        assert ExperimentalDiagnosticRecord.from_dict(data).to_dict() == data

    def test_roundtrip_without_details(self):
        data = {"code": "W999", "severity": "warning"}
        assert ExperimentalDiagnosticRecord.from_dict(data).to_dict() == data


class TestExperimentalReport:
    def test_construct_minimal(self):
        r = ExperimentalReport(
            schema_version="deobf-pass-registry.v0.1",
            evidence=[],
            diagnostics=[],
        )
        assert r.schema_version == "deobf-pass-registry.v0.1"
        assert r.evidence == []
        assert r.diagnostics == []
        assert r.writes is None
        assert r.extra_fields == {}

    def test_construct_with_writes(self):
        r = ExperimentalReport(
            schema_version="deobf-pass-registry.v0.1",
            evidence=[],
            diagnostics=[],
            writes=["file1.js"],
        )
        assert r.writes == ["file1.js"]

    def test_construct_with_empty_writes(self):
        r = ExperimentalReport(
            schema_version="deobf-pass-registry.v0.1",
            evidence=[],
            diagnostics=[],
            writes=[],
        )
        assert r.writes == []

    def test_construct_with_extra_fields(self):
        r = ExperimentalReport(
            schema_version="deobf-pass-registry.v0.1",
            evidence=[],
            diagnostics=[],
            extra_fields={"track": "vm", "custom": 42},
        )
        assert r.extra_fields == {"track": "vm", "custom": 42}

    def test_to_dict_minimal(self):
        r = ExperimentalReport(
            schema_version="deobf-pass-registry.v0.1",
            evidence=[],
            diagnostics=[],
        )
        assert r.to_dict() == {
            "schema_version": "deobf-pass-registry.v0.1",
            "evidence": [],
            "diagnostics": [],
        }

    def test_to_dict_with_writes(self):
        r = ExperimentalReport(
            schema_version="deobf-pass-registry.v0.1",
            evidence=[],
            diagnostics=[],
            writes=["out.js"],
        )
        d = r.to_dict()
        assert d["writes"] == ["out.js"]

    def test_to_dict_with_extra_fields(self):
        r = ExperimentalReport(
            schema_version="deobf-pass-registry.v0.1",
            evidence=[],
            diagnostics=[],
            extra_fields={"custom_flag": True},
        )
        d = r.to_dict()
        assert d["custom_flag"] is True
        assert d["schema_version"] == "deobf-pass-registry.v0.1"

    def test_to_dict_preserves_evidence_diagnostics(self):
        ev = ExperimentalEvidenceRecord("x", "strong")
        diag = ExperimentalDiagnosticRecord("E1", "error")
        r = ExperimentalReport(
            schema_version="deobf-pass-registry.v0.1",
            evidence=[ev],
            diagnostics=[diag],
        )
        d = r.to_dict()
        assert d["evidence"] == [{"kind": "x", "strength": "strong"}]
        assert d["diagnostics"] == [{"code": "E1", "severity": "error"}]

    def test_from_dict_minimal(self):
        d = {
            "schema_version": "vm-analysis.v0.1",
            "evidence": [],
            "diagnostics": [],
        }
        r = ExperimentalReport.from_dict(d)
        assert r.schema_version == "vm-analysis.v0.1"
        assert r.evidence == []
        assert r.diagnostics == []
        assert r.writes is None
        assert r.extra_fields == {}

    def test_from_dict_with_all_fields(self):
        d = {
            "schema_version": "vm-analysis.v0.1",
            "evidence": [{"kind": "static", "strength": "strong"}],
            "diagnostics": [{"code": "D1", "severity": "info", "details": {"ctx": "a"}}],
            "writes": ["out.bin"],
            "track": "vm-mem",
        }
        r = ExperimentalReport.from_dict(d)
        assert r.schema_version == "vm-analysis.v0.1"
        assert len(r.evidence) == 1
        assert r.evidence[0].kind == "static"
        assert len(r.diagnostics) == 1
        assert r.diagnostics[0].code == "D1"
        assert r.writes == ["out.bin"]
        assert r.extra_fields == {"track": "vm-mem"}

    def test_from_dict_with_empty_writes(self):
        d = {
            "schema_version": "vm-analysis.v0.1",
            "evidence": [],
            "diagnostics": [],
            "writes": [],
        }
        r = ExperimentalReport.from_dict(d)
        assert r.writes == []

    def test_roundtrip_full(self):
        data = {
            "schema_version": "vm-handler-table.v0.1",
            "evidence": [
                {"kind": "dynamic", "strength": "moderate"},
                {"kind": "static", "strength": "weak"},
            ],
            "diagnostics": [
                {"code": "X1", "severity": "error", "details": {"handler": 0x1234}},
                {"code": "X2", "severity": "warning"},
            ],
            "writes": ["dump.bin"],
            "extra_analysis": True,
            "confidence": 0.95,
        }
        assert experimental_report_roundtrip(data) == data

    def test_roundtrip_minimal(self):
        data = {
            "schema_version": "deobf-validation.v0.1",
            "evidence": [],
            "diagnostics": [],
        }
        assert experimental_report_roundtrip(data) == data

    def test_roundtrip_writes_none(self):
        data = {
            "schema_version": "deobf-validation.v0.1",
            "evidence": [],
            "diagnostics": [],
        }
        result = experimental_report_roundtrip(data)
        assert "writes" not in result

    def test_module_functions(self):
        data = {
            "schema_version": "deobf-validation.v0.1",
            "evidence": [],
            "diagnostics": [],
        }
        report = experimental_report_from_dict(data)
        assert isinstance(report, ExperimentalReport)
        assert experimental_report_to_dict(report) == data

    def test_schema_versions_set(self):
        assert isinstance(EXPERIMENTAL_SCHEMA_VERSIONS, frozenset)
        assert "vm-analysis.v0.1" in EXPERIMENTAL_SCHEMA_VERSIONS
        assert len(EXPERIMENTAL_SCHEMA_VERSIONS) == 8
