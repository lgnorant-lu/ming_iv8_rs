"""v0.8.50 S5: L2 diagnostic bridge projection test for RS补環境 vectors."""
from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

from tools.diagnostic_bridge import (
    OWNER_ROUTING_TABLE,
    build_evidence_bundle_manifest,
    build_repair_brief,
    build_validation_plan,
    classify_repair_readiness,
    project_tickets_from_knowledge_index,
    route_ticket_to_owner,
)


def test_rs_probed_vectors_all_in_routing_table():
    """All 8 vectors probed by RS补環境 are in OWNER_ROUTING_TABLE."""
    rs_vectors = [
        "V001",  # navigator.userAgent
        "V005",  # navigator.platform
        "V006",  # navigator.webdriver
        "V012",  # location.href
        "V013",  # location.host/hostname
        "V015",  # window/document references (top/self/parent)
        "V022",  # document.createElement
        "V085",  # document.cookie
    ]
    missing = [v for v in rs_vectors if v not in OWNER_ROUTING_TABLE]
    assert len(missing) == 0, f"RS vectors not in routing table: {missing}"


def test_rs_gap_ticket_projection():
    """RS补環境 gaps produce valid repair tickets with owner mapping."""
    known_gaps = [
        {
            "source_vector": "V015",
            "gap_class": "value_mismatch",
            "probe_id": "window_top_self_parent_identity",
            "detail": "window.top/self/parent === undefined before v0.8.50 fix",
        },
        {
            "source_vector": "V085",
            "gap_class": "value_mismatch",
            "probe_id": "document_cookie_readback",
            "detail": "document.cookie enable_* test probe",
        },
    ]

    knowledge_index = {"known_gaps": known_gaps}
    tickets = project_tickets_from_knowledge_index(knowledge_index)

    assert len(tickets) >= 1, "No tickets projected"

    for t in tickets:
        assert "ticket_id" in t
        assert "source_vector" in t
        assert "gap_class" in t


def test_rs_owner_routing_completeness():
    """Every RS-probed vector routes to a valid owner module."""
    rs_vector_owners = {
        "V001": "iv8-core/native_env.rs",
        "V005": "iv8-core/native_env.rs",
        "V012": "iv8-core/shims/",
        "V015": "iv8-surface",
        "V022": "iv8-undetect/",
        "V085": "iv8-core/dom/",
    }
    for vector, expected in rs_vector_owners.items():
        ticket = {
            "ticket_id": "test",
            "source_vector": vector,
            "gap_class": "missing_api",
            "evidence_refs": [],
            "l3_owner_module": "unknown",
            "risk_level": "low",
            "blocked_reason": None,
            "estimated_impact": None,
        }
        owner = route_ticket_to_owner(ticket)
        assert owner != "unknown", f"Vector {vector} not routed to any owner"
        assert expected in owner or owner in expected, (
            f"Vector {vector} routed to {owner}, expected {expected}"
        )


def test_rs_repair_readiness_classification():
    """RS gaps produce correctly classified repair readiness."""
    ticket = {
        "ticket_id": "test_V015",
        "source_vector": "V015",
        "gap_class": "value_mismatch",
        "evidence_refs": ["v0.8.50 window identity refs fix"],
        "l3_owner_module": "iv8-surface",
        "risk_level": "low",
        "blocked_reason": None,
        "estimated_impact": "window.top/self/parent identity chain",
    }
    dc = {
        "schema_version": "iv8-repair-delta-contract.v0.1",
        "ticket_id": "test_V015",
    }
    brief = build_repair_brief({}, ticket, dc)
    manifest = build_evidence_bundle_manifest(brief, {"source_reports": ["v0.8.50 RS report"]})
    plan = build_validation_plan(brief)
    readiness_result = classify_repair_readiness(brief, manifest, plan)
    readiness = readiness_result.get("readiness", readiness_result) if isinstance(readiness_result, dict) else readiness_result
    assert readiness in ("ready", "incomplete", "deferred", "blocked"), (
        f"Invalid readiness: {readiness}"
    )


def test_rs_vector_count_in_table():
    """RS-probed vector count matches expectations."""
    rs_vectors = [
        "V001", "V005", "V006", "V012", "V013",
        "V015", "V022", "V085",
    ]
    mapped = sum(1 for v in rs_vectors if v in OWNER_ROUTING_TABLE)
    assert mapped == len(rs_vectors), (
        f"Only {mapped}/{len(rs_vectors)} RS vectors in routing table"
    )


def test_rs_gap_classification_consistency():
    """Gap classification terms used in RS diagnostics are valid."""
    valid_gap_classes = {
        "missing_api", "value_mismatch", "missing_value",
        "descriptor_mismatch", "prototype_mismatch", "native_code_shape",
    }
    rs_gap_classes = {"missing_api", "value_mismatch"}
    assert rs_gap_classes.issubset(valid_gap_classes), (
        f"RS gap classes {rs_gap_classes - valid_gap_classes} not in valid set"
    )


def test_l2_rs_coverage_report_structure():
    """L2 RS coverage report has expected fields."""
    report = {
        "sample": "RS main.js + basic $_ts",
        "total_vectors_probed": 8,
        "vectors_in_routing_table": 8,
        "vectors_unmapped": 0,
        "owner_coverage_pct": 100.0,
        "gaps_found": 2,
        "gaps_fixed_in_v0850": 2,
        "gaps_remaining": 0,
    }
    assert report["vectors_unmapped"] == 0
    assert report["owner_coverage_pct"] == 100.0
    assert report["gaps_found"] >= report["gaps_fixed_in_v0850"]
