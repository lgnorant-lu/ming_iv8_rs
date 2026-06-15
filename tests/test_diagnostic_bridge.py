"""v0.8.40 Diagnostic-to-Substrate Bridge tests."""
from __future__ import annotations

import copy
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent.parent))

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
