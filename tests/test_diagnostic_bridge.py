"""v0.8.40 Diagnostic-to-Substrate Bridge tests."""
from __future__ import annotations

import copy
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))

from tools.convergence import (  # noqa: E402
    build_convergence_snapshot,
    make_convergence_event,
)
from tools.diagnostic_bridge import (  # noqa: E402
    OWNER_ROUTING_SCHEMA,
    OWNER_ROUTING_TABLE,
    REPAIR_TICKET_SCHEMA,
    RepairTicket,
    build_ticket_id,
    derive_source_vector,
    project_tickets_from_knowledge_index,
    route_ticket_to_owner,
)

_SAMPLE_KNOWLEDGE_INDEX = {
    "schema_version": "iv8-feedback-knowledge-index.v0.1",
    "source_snapshot_id": "sha256:abc",
    "source_delta_schema": "",
    "known_gaps": [
        {
            "gap_id": "sha256:gap-001",
            "subject": {
                "probe_id": "idl.attr.Navigator.userAgent",
                "target": "navigator.userAgent",
                "category": "value",
            },
            "gap_class": "value_mismatch",
            "lifecycle": "new",
            "severity": "medium",
            "source_event_ids": ["sha256:evt-001"],
        },
        {
            "gap_id": "sha256:gap-002",
            "subject": {
                "probe_id": "idl.attr.Screen.width",
                "target": "screen.width",
                "category": "value",
            },
            "gap_class": "value_mismatch",
            "lifecycle": "persisting",
            "severity": "medium",
            "source_event_ids": ["sha256:evt-002"],
        },
        {
            "gap_id": "sha256:gap-003",
            "subject": {
                "probe_id": "idl.attr.Performance.timeOrigin",
                "target": "performance.timeOrigin",
                "category": "value",
            },
            "gap_class": "value_mismatch",
            "lifecycle": "new",
            "severity": "high",
            "source_event_ids": ["sha256:evt-003"],
        },
    ],
    "evidence_ceiling": "diagnostic_only",
    "writes": [],
    "summary": {"known_gaps": 3, "new": 2, "persisting": 1, "resolved": 0, "changed": 0},
}


def test_repair_ticket_has_all_required_fields():
    ticket = RepairTicket(
        ticket_id="t-001",
        source_vector="V001",
        gap_class="value_mismatch",
        evidence_refs=["sha256:evt-001"],
        l3_owner_module="iv8-core/native_env.rs",
        risk_level="medium",
    )
    assert ticket.ticket_id == "t-001"
    assert ticket.source_vector == "V001"
    assert ticket.gap_class == "value_mismatch"
    assert ticket.l3_owner_module == "iv8-core/native_env.rs"
    assert ticket.blocked_reason is None
    assert ticket.estimated_impact == "unknown"


def test_ticket_id_is_deterministic():
    first = build_ticket_id("sha256:gap-001", "V001")
    second = build_ticket_id("sha256:gap-001", "V001")
    assert first == second
    assert first.startswith("sha256:")


def test_project_tickets_from_knowledge_index():
    tickets = project_tickets_from_knowledge_index(_SAMPLE_KNOWLEDGE_INDEX)
    assert len(tickets) == 3
    for ticket in tickets:
        assert ticket["ticket_id"].startswith("sha256:")
        assert ticket["gap_class"] in {"value_mismatch", "none"}
        assert ticket["risk_level"] in {"low", "medium", "high"}
        assert ticket["l3_owner_module"] != ""
        assert ticket["writes"] == []
        assert ticket["evidence_ceiling"] == "diagnostic_only"


def test_knowledge_index_not_mutated():
    before = copy.deepcopy(_SAMPLE_KNOWLEDGE_INDEX)
    project_tickets_from_knowledge_index(_SAMPLE_KNOWLEDGE_INDEX)
    assert _SAMPLE_KNOWLEDGE_INDEX == before


def test_owner_routing_table_is_static():
    assert "V001" in OWNER_ROUTING_TABLE
    assert "V015" in OWNER_ROUTING_TABLE
    assert OWNER_ROUTING_TABLE["V001"] == "iv8-core/native_env.rs"


def test_route_ticket_is_deterministic():
    first = route_ticket_to_owner({"source_vector": "V015"})
    second = route_ticket_to_owner({"source_vector": "V015"})
    assert first == second


def test_derive_source_vector():
    assert derive_source_vector("idl.attr.Navigator.userAgent") == "V001"
    assert derive_source_vector("idl.attr.Screen.width") == "V015"
    assert derive_source_vector("unknown.probe.id") == "unmapped"


def test_ticket_schema_versions_are_explicit():
    assert REPAIR_TICKET_SCHEMA == "iv8-repair-ticket.v0.1"
    assert OWNER_ROUTING_SCHEMA == "iv8-owner-routing-table.v0.1"


def test_bridge_does_not_write_files(tmp_path, monkeypatch):
    monkeypatch.chdir(tmp_path)
    pre = set(tmp_path.iterdir())
    project_tickets_from_knowledge_index(_SAMPLE_KNOWLEDGE_INDEX)
    post = set(tmp_path.iterdir())
    assert pre == post


# -- v0.8.41 delta contract and candidate ledger gates ---------------------


def test_build_delta_contract_produces_valid_contract():
    from tools.diagnostic_bridge import (
        build_delta_contract,
        project_tickets_from_knowledge_index,
    )

    tickets = project_tickets_from_knowledge_index(_SAMPLE_KNOWLEDGE_INDEX)
    ticket = tickets[0]
    subj = {
        "probe_id": "idl.attr.Navigator.userAgent",
        "target": "navigator.userAgent",
        "category": "value",
    }
    event_a = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "evt-001"},
        subject=subj,
        status="fail", expected="A", actual="B",
    )
    event_b = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "evt-001"},
        subject=subj,
        status="pass", expected="B", actual="B",
    )
    base = build_convergence_snapshot([event_a])
    current = build_convergence_snapshot([event_b])
    contract = build_delta_contract(ticket, base, current)
    assert contract["ticket_id"] == ticket["ticket_id"]
    assert contract["base_snapshot_id"] == base["snapshot_id"]
    assert contract["current_snapshot_id"] == current["snapshot_id"]
    assert "delta_summary" in contract
    assert contract["writes"] == []
    assert contract["evidence_ceiling"] == "diagnostic_only"


def test_check_gap_resolved_identifies_resolved():
    from tools.diagnostic_bridge import (
        check_gap_resolved,
        project_tickets_from_knowledge_index,
    )

    tickets = project_tickets_from_knowledge_index(_SAMPLE_KNOWLEDGE_INDEX)
    ticket = tickets[0]
    subj = {
        "probe_id": "idl.attr.Navigator.userAgent",
        "target": "navigator.userAgent",
        "category": "value",
    }
    event_a = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "evt-001"},
        subject=subj,
        status="fail",
    )
    event_b = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "evt-001"},
        subject=subj,
        status="pass",
    )
    base = build_convergence_snapshot([event_a])
    current = build_convergence_snapshot([event_b])
    assert check_gap_resolved(ticket, base, current)


def test_delta_contract_snapshots_not_mutated():
    from tools.diagnostic_bridge import (
        build_delta_contract,
        project_tickets_from_knowledge_index,
    )

    tickets = project_tickets_from_knowledge_index(_SAMPLE_KNOWLEDGE_INDEX)
    ticket = tickets[0]
    subj = {
        "probe_id": "idl.attr.Navigator.userAgent",
        "target": "navigator.userAgent",
        "category": "value",
    }
    event = make_convergence_event(
        source={"report_schema": "s", "report_kind": "k", "source_id": "evt-001"},
        subject=subj,
        status="fail",
    )
    base = build_convergence_snapshot([event])
    current = build_convergence_snapshot([event])
    base_before = copy.deepcopy(base)
    current_before = copy.deepcopy(current)
    build_delta_contract(ticket, base, current)
    assert base == base_before
    assert current == current_before


def test_build_candidate_ledger_sorts_by_risk():
    from tools.diagnostic_bridge import (
        build_candidate_ledger,
        project_tickets_from_knowledge_index,
    )

    tickets = project_tickets_from_knowledge_index(_SAMPLE_KNOWLEDGE_INDEX)
    ledger = build_candidate_ledger(tickets)
    assert len(ledger) >= 2
    priorities = [e["priority"] for e in ledger]
    assert priorities == sorted(priorities)


def test_candidate_ledger_lifecycle_starts_open():
    from tools.diagnostic_bridge import (
        build_candidate_ledger,
        project_tickets_from_knowledge_index,
    )

    tickets = project_tickets_from_knowledge_index(_SAMPLE_KNOWLEDGE_INDEX)
    ledger = build_candidate_ledger(tickets)
    for entry in ledger:
        assert entry["lifecycle"] == "open"
        assert entry["writes"] == []
        assert entry["evidence_ceiling"] == "diagnostic_only"
