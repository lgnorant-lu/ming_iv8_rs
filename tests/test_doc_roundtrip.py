from __future__ import annotations

import pytest

pytest.importorskip("iv8_rs")

from iv8_rs import (
    experimental_report_from_dict,
    experimental_report_to_dict,
    experimental_report_roundtrip,
    ir_node_report_from_dict,
    ir_node_report_to_dict,
    pressure_report_from_dict,
    pressure_report_to_dict,
    registry_report_from_dict,
    registry_report_to_dict,
    string_array_report_from_dict,
    string_array_report_to_dict,
    toolchain_report_from_dict,
    toolchain_report_to_dict,
    validation_report_from_dict,
    validation_report_to_dict,
    vm_analysis_report_from_dict,
    vm_analysis_report_to_dict,
    vm_handler_table_from_dict,
    vm_handler_table_to_dict,
)
from iv8_rs import (
    BytecodeCandidate,
    ConfidenceSummary,
    CoverageDelta,
    CoverageSnapshot,
    DeobfRegistryReport,
    EnvironmentPressureReport,
    EnvironmentToolchainReport,
    ExperimentalDiagnosticRecord,
    ExperimentalEvidenceRecord,
    ExperimentalReport,
    HandlerEntry,
    IRNode,
    IRNodeReport,
    PressureSignal,
    PromotionDecision,
    SelectionReport,
    StringArrayReport,
    TraceSummary,
    ValidationReport,
    VMAnalysisReport,
    VMHandlerTable,
    OpcodeHint,
    StateModel,
)

pytestmark = pytest.mark.slow


@pytest.mark.parametrize(
    ("from_dict_fn", "to_dict_fn", "instance", "expected_type", "check_fields"),
    [
        pytest.param(
            experimental_report_from_dict,
            experimental_report_to_dict,
            ExperimentalReport(
                schema_version="test.v1",
                evidence=[ExperimentalEvidenceRecord(kind="t", strength="diagnostic_only")],
                diagnostics=[ExperimentalDiagnosticRecord(code="T1", severity="info")],
            ),
            ExperimentalReport,
            {"schema_version": "test.v1"},
            id="experimental_report",
        ),
        pytest.param(
            ir_node_report_from_dict,
            ir_node_report_to_dict,
            IRNodeReport(
                schema_version="test.v1",
                ir_kind="cfg",
                node_count=1,
                edge_count=0,
                confidence_summary=ConfidenceSummary(),
                unsupported_features=[],
                nodes=[IRNode(id=0, kind="basic_block", source="a", confidence="strong")],
                evidence=[ExperimentalEvidenceRecord(kind="t", strength="diagnostic_only")],
                diagnostics=[ExperimentalDiagnosticRecord(code="T1", severity="info")],
            ),
            IRNodeReport,
            {"schema_version": "test.v1", "ir_kind": "cfg", "node_count": 1},
            id="ir_node_report",
        ),
        pytest.param(
            pressure_report_from_dict,
            pressure_report_to_dict,
            EnvironmentPressureReport(
                schema_version="environment-pressure.v0.1",
                sample_id="s1",
                input_kind="direct_js",
                execution_mode="bare_v8",
                status="success",
                failure_kind="success",
                pressure=PressureSignal(pressure_kind="input_normalization"),
                promotion=PromotionDecision(
                    level="observe_only",
                    reason="test",
                    evidence_ceiling="diagnostic_only",
                    review_status="review_only",
                ),
                evidence=[ExperimentalEvidenceRecord(kind="t", strength="diagnostic_only")],
                diagnostics=[ExperimentalDiagnosticRecord(code="T1", severity="info")],
                writes=[],
            ),
            EnvironmentPressureReport,
            {"schema_version": "environment-pressure.v0.1", "sample_id": "s1"},
            id="pressure_report",
        ),
        pytest.param(
            registry_report_from_dict,
            registry_report_to_dict,
            DeobfRegistryReport(
                schema_version="test.v1",
                entries=[],
                selection_report=SelectionReport(selected=[], rejected=[]),
                evidence=[ExperimentalEvidenceRecord(kind="t", strength="diagnostic_only")],
                diagnostics=[ExperimentalDiagnosticRecord(code="T1", severity="info")],
            ),
            DeobfRegistryReport,
            {"schema_version": "test.v1"},
            id="registry_report",
        ),
        pytest.param(
            string_array_report_from_dict,
            string_array_report_to_dict,
            StringArrayReport(
                schema_version="test.v1",
                arrays=[],
                rotation_iifes=[],
                decoders=[],
                replacement_sites=[],
                status="ok",
                source_rewritten=False,
                evidence=[ExperimentalEvidenceRecord(kind="t", strength="diagnostic_only")],
                diagnostics=[ExperimentalDiagnosticRecord(code="T1", severity="info")],
            ),
            StringArrayReport,
            {"schema_version": "test.v1", "status": "ok"},
            id="string_array_report",
        ),
        pytest.param(
            toolchain_report_from_dict,
            toolchain_report_to_dict,
            EnvironmentToolchainReport(
                schema_version="test.v1",
                probe_pack="default",
                before=CoverageSnapshot(present=0, missing=0, mismatch=0),
                after=CoverageSnapshot(present=0, missing=0, mismatch=0),
                coverage_delta=CoverageDelta(improved=0, regressed=0, unresolved=0),
                applied_patches=[],
                rejected_patches=[],
                profile_suggestions=[],
                evidence=[ExperimentalEvidenceRecord(kind="t", strength="diagnostic_only")],
                diagnostics=[ExperimentalDiagnosticRecord(code="T1", severity="info")],
            ),
            EnvironmentToolchainReport,
            {"schema_version": "test.v1", "probe_pack": "default"},
            id="toolchain_report",
        ),
        pytest.param(
            validation_report_from_dict,
            validation_report_to_dict,
            ValidationReport(
                schema_version="test.v1",
                source_id="src",
                pass_id="p1",
                input_hash="in",
                output_hash="out",
                level="pass",
                policy_status="accepted",
                checks=[],
                evidence=[ExperimentalEvidenceRecord(kind="t", strength="diagnostic_only")],
                diagnostics=[ExperimentalDiagnosticRecord(code="T1", severity="info")],
            ),
            ValidationReport,
            {"schema_version": "test.v1", "source_id": "src", "pass_id": "p1"},
            id="validation_report",
        ),
        pytest.param(
            vm_analysis_report_from_dict,
            vm_analysis_report_to_dict,
            VMAnalysisReport(
                schema_version="test.v1",
                sample_id="s1",
                vm_family="test_vm",
                dispatch_variant="computed",
                handler_table=None,
                bytecode=None,
                trace_summary=TraceSummary(opcode_sequence_observed=False),
                state_model=StateModel(hints=[]),
                opcode_map={"h1": OpcodeHint(label="hint", confidence="strong", evidence=[])},
                evidence=[ExperimentalEvidenceRecord(kind="t", strength="diagnostic_only")],
                diagnostics=[ExperimentalDiagnosticRecord(code="T1", severity="info")],
            ),
            VMAnalysisReport,
            {"schema_version": "test.v1", "sample_id": "s1", "vm_family": "test_vm"},
            id="vm_analysis_report",
        ),
        pytest.param(
            vm_handler_table_from_dict,
            vm_handler_table_to_dict,
            VMHandlerTable(
                schema_version="test.v1",
                kind="dispatch",
                handler_count=0,
                ids=[],
                source_available=True,
                runtime_validated=False,
                extraction_quality="low",
                shape_score=0,
                handlers=[],
                bytecode_candidates=[],
                evidence=[ExperimentalEvidenceRecord(kind="t", strength="diagnostic_only")],
                diagnostics=[ExperimentalDiagnosticRecord(code="T1", severity="info")],
            ),
            VMHandlerTable,
            {"schema_version": "test.v1", "kind": "dispatch"},
            id="vm_handler_table",
        ),
    ],
)
def test_dto_roundtrip(from_dict_fn, to_dict_fn, instance, expected_type, check_fields):
    d = to_dict_fn(instance)
    obj2 = from_dict_fn(d)
    assert isinstance(obj2, expected_type)
    for field, expected in check_fields.items():
        assert getattr(obj2, field) == expected


def test_experimental_report_roundtrip():
    data = {
        "schema_version": "test.v1",
        "evidence": [{"kind": "t", "strength": "diagnostic_only"}],
        "diagnostics": [{"code": "T1", "severity": "info"}],
    }
    result = experimental_report_roundtrip(data)
    assert result == data
