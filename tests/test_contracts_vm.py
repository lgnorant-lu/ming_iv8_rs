"""Contract tests — VM analysis families (parametrized)."""

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

# vm-trace fixture intentionally has strong evidence — skip no_strong_evidence check
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
