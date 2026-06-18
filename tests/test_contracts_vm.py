"""Contract tests — VM analysis families (parametrized)."""

import pytest
from experimental_contract_helpers import load_fixture, assert_no_strong_evidence, assert_diagnostic, assert_fields

VM_CONTRACTS = [
    ("vm-analysis", ["schema_version", "vm_family", "dispatch_variant", "handler_table"], "VM_ANALYSIS_OPCODE_HINT_ONLY"),
    ("vm-handler", ["schema_version", "kind", "handler_count", "shape_score"], "VM_HANDLER_EXTRACTION_WEAK"),
    ("vm-bytecode", ["schema_version", "candidates", "artifact_policy"], "VM_BYTECODE_CURSOR_NOT_LINKED"),
    ("vm-trace", ["schema_version", "events", "completeness"], "VM_TRACE_EXTENDED_HINT_ONLY"),
]


@pytest.mark.parametrize("family,fields,code", VM_CONTRACTS)
def test_vm_contract_schema(family, fields, code):
    report = load_fixture(family)
    assert_fields(report, fields)


@pytest.mark.parametrize("family,fields,code", VM_CONTRACTS)
def test_vm_contract_no_strong_evidence(family, fields, code):
    report = load_fixture(family)
    assert_no_strong_evidence(report.get("evidence", []))


@pytest.mark.parametrize("family,fields,code", VM_CONTRACTS)
def test_vm_contract_diagnostics(family, fields, code):
    report = load_fixture(family)
    assert_diagnostic(report, code)
