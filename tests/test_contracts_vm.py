"""Contract tests — VM analysis families (parametrized + semantic checks)."""

import pytest
from experimental_contract_helpers import (
    assert_diagnostic,
    assert_fields,
    assert_no_strong_evidence,
    load_fixture,
)

VM_CONTRACTS = [
    ("vm-analysis", ["schema_version", "vm_family", "dispatch_variant", "handler_table"], "VM_BYTECODE_CANDIDATE_UNVALIDATED"),
    ("vm-handler", ["schema_version", "kind", "handler_count", "shape_score"], "VM_BYTECODE_CANDIDATE_UNVALIDATED"),
    ("vm-bytecode", ["schema_version", "candidates", "cursor_linkage", "raw_export_allowed"], "VM_BYTECODE_CURSOR_NOT_LINKED"),
    ("vm-trace", ["schema_version", "events", "completeness"], "VM_TRACE_STATE_SNAPSHOT_WEAK"),
]

VM_NO_STRONG_SKIP = {"vm-trace"}


@pytest.mark.parametrize("family,fields,code", VM_CONTRACTS)
def test_vm_contract_schema(family, fields, code):
    report = load_fixture(family)
    assert_fields(report, fields)


@pytest.mark.parametrize("family,fields,code", VM_CONTRACTS)
def test_vm_contract_no_strong_evidence(family, fields, code):
    if family in VM_NO_STRONG_SKIP:
        return
    report = load_fixture(family)
    assert_no_strong_evidence(report.get("evidence", []))


@pytest.mark.parametrize("family,fields,code", VM_CONTRACTS)
def test_vm_contract_diagnostics(family, fields, code):
    if code is None:
        return
    report = load_fixture(family)
    assert_diagnostic(report, code)


def test_vm_analysis_handler_table_unvalidated():
    report = load_fixture("vm-analysis")
    table = report["handler_table"]
    assert report["dispatch_variant"] == "switch_vm_numeric_pc"
    assert table["runtime_validated"] is False
    assert table["kind"] == "switch"


def test_vm_handler_not_runtime_validated():
    report = load_fixture("vm-handler")
    assert report["runtime_validated"] is False
    assert report["handler_count"] > 0
    assert report["shape_score"] >= 1


def test_vm_bytecode_hash_preview_only():
    report = load_fixture("vm-bytecode")
    candidate = report["candidates"][0]
    assert report["cursor_linkage"] == "shape_only"
    assert report["raw_export_allowed"] is False
    assert candidate["artifact_policy"] == "hash_preview_only"
    assert candidate["validation"] == "candidate_unvalidated"


def test_vm_trace_extended_event_schema_and_hint_labels():
    report = load_fixture("vm-trace")
    assert_fields(report, ["schema_version", "trace_id", "events", "completeness", "evidence"])
    assert report["completeness"] == "case_level"
    assert all(label.endswith("_hint") for label in report["semantic_labels"])
    event = report["events"][0]
    assert_fields(event, ["event", "step", "handler_id", "opcode", "bytecode_cursor", "confidence"])
    assert event["event"] == "handler_enter"
