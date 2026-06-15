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


# -- v0.8.42 repair harness gates -----------------------------------------


def test_build_repair_brief_produces_diagnostic_only_brief():
    from tools.diagnostic_bridge import (
        REPAIR_BRIEF_SCHEMA,
        build_candidate_ledger,
        build_repair_brief,
        project_tickets_from_knowledge_index,
    )

    tickets = project_tickets_from_knowledge_index(_SAMPLE_KNOWLEDGE_INDEX)
    candidate = build_candidate_ledger(tickets)[0]
    matching_ticket = next(
        t for t in tickets if t["ticket_id"] == candidate["ticket_id"]
    )
    brief = build_repair_brief(candidate, ticket=matching_ticket)
    assert brief["schema_version"] == REPAIR_BRIEF_SCHEMA
    assert brief["brief_id"].startswith("sha256:")
    assert brief["ticket_id"] == candidate["ticket_id"]
    assert brief["source_vector"] != ""
    assert brief["l3_owner_module"] != ""
    assert brief["writes"] == []
    assert brief["evidence_ceiling"] == "diagnostic_only"


def test_repair_brief_id_is_deterministic_and_inputs_not_mutated():
    from tools.diagnostic_bridge import (
        build_candidate_ledger,
        build_repair_brief,
        project_tickets_from_knowledge_index,
    )

    tickets = project_tickets_from_knowledge_index(_SAMPLE_KNOWLEDGE_INDEX)
    candidate = build_candidate_ledger(tickets)[0]
    matching_ticket = next(
        t for t in tickets if t["ticket_id"] == candidate["ticket_id"]
    )
    before_candidate = copy.deepcopy(candidate)
    before_ticket = copy.deepcopy(matching_ticket)
    first = build_repair_brief(candidate, ticket=matching_ticket)
    second = build_repair_brief(candidate, ticket=matching_ticket)
    assert first["brief_id"] == second["brief_id"]
    assert candidate == before_candidate
    assert matching_ticket == before_ticket


def test_repair_brief_blocks_on_ticket_candidate_mismatch():
    from tools.diagnostic_bridge import (
        build_candidate_ledger,
        build_repair_brief,
        project_tickets_from_knowledge_index,
    )

    tickets = project_tickets_from_knowledge_index(_SAMPLE_KNOWLEDGE_INDEX)
    candidate = build_candidate_ledger(tickets)[0]
    other_ticket = next(
        t for t in tickets if t["ticket_id"] != candidate["ticket_id"]
    )
    brief = build_repair_brief(candidate, ticket=other_ticket)
    assert brief["readiness"] == "blocked"
    assert brief["blocked_reason"] == "ticket_candidate_mismatch"
    assert brief["writes"] == []
    assert brief["evidence_ceiling"] == "diagnostic_only"


def test_build_evidence_bundle_manifest_references_only(tmp_path, monkeypatch):
    from tools.diagnostic_bridge import (
        EVIDENCE_BUNDLE_SCHEMA,
        build_evidence_bundle_manifest,
        build_repair_brief,
    )

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    sources = {"source_reports": ["snapshot:base"], "delta_contract_ref": "delta:1"}
    before = copy.deepcopy(sources)
    monkeypatch.chdir(tmp_path)
    pre = set(tmp_path.iterdir())
    manifest = build_evidence_bundle_manifest(brief, sources)
    post = set(tmp_path.iterdir())
    assert manifest["schema_version"] == EVIDENCE_BUNDLE_SCHEMA
    assert manifest["brief_id"] == brief["brief_id"]
    assert manifest["source_reports"] == ["snapshot:base"]
    assert manifest["missing_refs"] == []
    assert manifest["writes"] == []
    assert manifest["evidence_ceiling"] == "diagnostic_only"
    assert sources == before
    assert pre == post


def test_evidence_bundle_manifest_records_missing_refs():
    from tools.diagnostic_bridge import (
        build_evidence_bundle_manifest,
        build_repair_brief,
    )

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    manifest = build_evidence_bundle_manifest(brief, {})
    assert "source_reports" in manifest["missing_refs"]
    assert "delta_contract_ref" in manifest["missing_refs"]


def test_build_validation_plan_does_not_execute(tmp_path, monkeypatch):
    from tools.diagnostic_bridge import (
        VALIDATION_PLAN_SCHEMA,
        build_repair_brief,
        build_validation_plan,
    )

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    monkeypatch.chdir(tmp_path)
    pre = set(tmp_path.iterdir())
    plan = build_validation_plan(
        brief,
        commands=["uv run pytest tests/test_diagnostic_bridge.py -q"],
    )
    post = set(tmp_path.iterdir())
    assert plan["schema_version"] == VALIDATION_PLAN_SCHEMA
    assert plan["brief_id"] == brief["brief_id"]
    assert plan["commands"] == ["uv run pytest tests/test_diagnostic_bridge.py -q"]
    assert plan["writes"] == []
    assert plan["evidence_ceiling"] == "diagnostic_only"
    assert pre == post


def test_classify_repair_readiness_ready_with_complete_bundle_and_plan():
    from tools.diagnostic_bridge import (
        build_evidence_bundle_manifest,
        build_repair_brief,
        build_validation_plan,
        classify_repair_readiness,
    )

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    manifest = build_evidence_bundle_manifest(
        brief,
        {"source_reports": ["snapshot:base"], "delta_contract_ref": "delta:1"},
    )
    plan = build_validation_plan(brief)
    readiness = classify_repair_readiness(brief, manifest, plan)
    assert readiness["readiness"] == "ready"
    assert readiness["readiness_reasons"] == []
    assert readiness["writes"] == []
    assert readiness["evidence_ceiling"] == "diagnostic_only"


def test_classify_repair_readiness_incomplete_with_missing_evidence():
    from tools.diagnostic_bridge import (
        build_evidence_bundle_manifest,
        build_repair_brief,
        build_validation_plan,
        classify_repair_readiness,
    )

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    manifest = build_evidence_bundle_manifest(brief, {})
    plan = build_validation_plan(brief)
    readiness = classify_repair_readiness(brief, manifest, plan)
    assert readiness["readiness"] == "incomplete"
    assert "missing_evidence_refs" in readiness["readiness_reasons"]
    assert readiness["evidence_ceiling"] == "diagnostic_only"


def test_classify_repair_readiness_incomplete_with_missing_owner():
    from tools.diagnostic_bridge import (
        build_evidence_bundle_manifest,
        build_repair_brief,
        build_validation_plan,
        classify_repair_readiness,
    )

    brief = build_repair_brief(
        {"ticket_id": "t-001", "source_vector": "unmapped", "l3_owner_module": "unknown"},
    )
    manifest = build_evidence_bundle_manifest(
        brief,
        {"source_reports": ["snapshot:base"], "delta_contract_ref": "delta:1"},
    )
    plan = build_validation_plan(brief)
    readiness = classify_repair_readiness(brief, manifest, plan)
    assert readiness["readiness"] == "incomplete"
    assert "missing_owner" in readiness["readiness_reasons"]
    assert readiness["evidence_ceiling"] == "diagnostic_only"


def test_classify_repair_readiness_incomplete_without_validation_plan():
    from tools.diagnostic_bridge import (
        build_evidence_bundle_manifest,
        build_repair_brief,
        classify_repair_readiness,
    )

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    manifest = build_evidence_bundle_manifest(
        brief,
        {"source_reports": ["snapshot:base"], "delta_contract_ref": "delta:1"},
    )
    readiness = classify_repair_readiness(brief, manifest, None)
    assert readiness["readiness"] == "incomplete"
    assert "missing_validation_plan" in readiness["readiness_reasons"]
    assert readiness["evidence_ceiling"] == "diagnostic_only"


def test_classify_repair_readiness_blocked_with_reason():
    from tools.diagnostic_bridge import (
        build_evidence_bundle_manifest,
        build_repair_brief,
        build_validation_plan,
        classify_repair_readiness,
    )

    brief = build_repair_brief(
        {"ticket_id": "t-001", "source_vector": "V001"},
    )
    brief["blocked_reason"] = "manual_block"
    brief["readiness"] = "blocked"
    manifest = build_evidence_bundle_manifest(
        brief,
        {"source_reports": ["snapshot:base"], "delta_contract_ref": "delta:1"},
    )
    plan = build_validation_plan(brief)
    readiness = classify_repair_readiness(brief, manifest, plan)
    assert readiness["readiness"] == "blocked"
    assert "manual_block" in readiness["readiness_reasons"]
    assert readiness["evidence_ceiling"] == "diagnostic_only"


def test_classify_repair_readiness_deferred_with_reason():
    from tools.diagnostic_bridge import (
        build_evidence_bundle_manifest,
        build_repair_brief,
        build_validation_plan,
        classify_repair_readiness,
    )

    brief = build_repair_brief(
        {"ticket_id": "t-001", "source_vector": "V001", "deferred_reason": "awaiting upstream BCR"},
    )
    manifest = build_evidence_bundle_manifest(
        brief,
        {"source_reports": ["snapshot:base"], "delta_contract_ref": "delta:1"},
    )
    plan = build_validation_plan(brief)
    readiness = classify_repair_readiness(brief, manifest, plan)
    assert readiness["readiness"] == "deferred"
    assert "awaiting upstream BCR" in readiness["readiness_reasons"]
    assert readiness["evidence_ceiling"] == "diagnostic_only"


# -- v0.8.43 L3 P0 runtime batch gates ----------------------------------


def test_select_runtime_briefs_filters_non_ready():
    from tools.diagnostic_bridge import build_repair_brief, select_runtime_briefs

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    assert brief["readiness"] != "ready"
    result = select_runtime_briefs([brief])
    assert len(result) == 0


def test_select_runtime_briefs_filters_wrong_owner():
    from tools.diagnostic_bridge import build_repair_brief, select_runtime_briefs

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V015"})
    brief["readiness"] = "ready"
    brief["gap_class"] = "value_mismatch"
    brief["risk_level"] = "low"
    assert brief["l3_owner_module"] != "iv8-core/native_env.rs"
    result = select_runtime_briefs([brief])
    assert len(result) == 0


def test_select_runtime_briefs_filters_wrong_gap_class():
    from tools.diagnostic_bridge import build_repair_brief, select_runtime_briefs

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    brief["readiness"] = "ready"
    brief["gap_class"] = "structural_mismatch"
    brief["risk_level"] = "low"
    brief["l3_owner_module"] = "iv8-core/native_env.rs"
    result = select_runtime_briefs([brief])
    assert len(result) == 0


def test_select_runtime_briefs_filters_high_risk():
    from tools.diagnostic_bridge import build_repair_brief, select_runtime_briefs

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    brief["readiness"] = "ready"
    brief["gap_class"] = "value_mismatch"
    brief["risk_level"] = "high"
    brief["l3_owner_module"] = "iv8-core/native_env.rs"
    result = select_runtime_briefs([brief])
    assert len(result) == 0


def test_select_runtime_briefs_selects_valid_navigator_brief():
    from tools.diagnostic_bridge import (
        build_evidence_bundle_manifest,
        build_repair_brief,
        build_validation_plan,
        classify_repair_readiness,
        select_runtime_briefs,
    )

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    brief["l3_owner_module"] = "iv8-core/native_env.rs"
    brief["gap_class"] = "value_mismatch"
    brief["risk_level"] = "low"
    manifest = build_evidence_bundle_manifest(
        brief,
        {"source_reports": ["snapshot:base"], "delta_contract_ref": "delta:1"},
    )
    plan = build_validation_plan(brief)
    readiness = classify_repair_readiness(brief, manifest, plan)
    assert readiness["readiness"] == "ready"
    brief["readiness"] = readiness["readiness"]
    result = select_runtime_briefs([brief])
    assert len(result) == 1
    assert result[0]["ticket_id"] == "t-001"


def test_select_runtime_briefs_sorts_low_before_medium():
    from tools.diagnostic_bridge import build_repair_brief, select_runtime_briefs

    b1 = build_repair_brief({"ticket_id": "t-a", "source_vector": "V001"})
    b1["readiness"] = "ready"
    b1["gap_class"] = "value_mismatch"
    b1["risk_level"] = "medium"
    b1["l3_owner_module"] = "iv8-core/native_env.rs"

    b2 = build_repair_brief({"ticket_id": "t-b", "source_vector": "V046"})
    b2["readiness"] = "ready"
    b2["gap_class"] = "value_mismatch"
    b2["risk_level"] = "low"
    b2["l3_owner_module"] = "iv8-core/native_env.rs"

    result = select_runtime_briefs([b1, b2])
    assert len(result) == 2
    assert result[0]["risk_level"] == "low"
    assert result[1]["risk_level"] == "medium"


def test_validate_runtime_brief_rejects_non_navigator_vector():
    from tools.diagnostic_bridge import build_repair_brief, validate_runtime_brief

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V015"})
    brief["readiness"] = "ready"
    brief["gap_class"] = "value_mismatch"
    brief["l3_owner_module"] = "iv8-core/native_env.rs"
    result = validate_runtime_brief(brief)
    assert result["valid"] is False
    assert any("not_in_runtime_set" in r for r in result["blocked_reasons"])
    assert result["evidence_ceiling"] == "diagnostic_only"


def test_validate_runtime_brief_accepts_valid_navigator_brief():
    from tools.diagnostic_bridge import build_repair_brief, validate_runtime_brief

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    brief["readiness"] = "ready"
    brief["gap_class"] = "value_mismatch"
    brief["l3_owner_module"] = "iv8-core/native_env.rs"
    result = validate_runtime_brief(brief)
    assert result["valid"] is True
    assert result["blocked_reasons"] == []
    assert result["evidence_ceiling"] == "diagnostic_only"


def test_runtime_brief_end_to_end_selection_and_validation():
    from tools.diagnostic_bridge import (
        build_evidence_bundle_manifest,
        build_repair_brief,
        build_validation_plan,
        classify_repair_readiness,
        select_runtime_briefs,
        validate_runtime_brief,
    )

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    brief["l3_owner_module"] = "iv8-core/native_env.rs"
    brief["gap_class"] = "value_mismatch"
    brief["risk_level"] = "low"
    manifest = build_evidence_bundle_manifest(
        brief,
        {"source_reports": ["snapshot:base"], "delta_contract_ref": "delta:1"},
    )
    plan = build_validation_plan(brief)
    readiness = classify_repair_readiness(brief, manifest, plan)
    assert readiness["readiness"] == "ready"
    brief["readiness"] = readiness["readiness"]
    selected = select_runtime_briefs([brief])
    assert len(selected) == 1
    validation = validate_runtime_brief(selected[0])
    assert validation["valid"] is True
    assert validation["writes"] == []


# -- v0.8.44 L3 P1 screen/window runtime batch gates -------------------


def test_select_runtime_briefs_default_still_navigator_only():
    from tools.diagnostic_bridge import build_repair_brief, select_runtime_briefs

    nav = build_repair_brief({"ticket_id": "t-nav", "source_vector": "V001"})
    nav["readiness"] = "ready"
    nav["gap_class"] = "value_mismatch"
    nav["risk_level"] = "low"
    nav["l3_owner_module"] = "iv8-core/native_env.rs"

    screen = build_repair_brief({"ticket_id": "t-screen", "source_vector": "V015"})
    screen["readiness"] = "ready"
    screen["gap_class"] = "value_mismatch"
    screen["risk_level"] = "low"
    screen["l3_owner_module"] = "iv8-surface"

    result = select_runtime_briefs([nav, screen])
    assert len(result) == 1
    assert result[0]["ticket_id"] == "t-nav"


def test_select_runtime_briefs_screen_owner_selects_screen_briefs():
    from tools.diagnostic_bridge import (
        SCREEN_WINDOW_OWNER_PATH,
        build_repair_brief,
        select_runtime_briefs,
    )

    nav = build_repair_brief({"ticket_id": "t-nav", "source_vector": "V001"})
    nav["readiness"] = "ready"
    nav["gap_class"] = "value_mismatch"
    nav["risk_level"] = "low"
    nav["l3_owner_module"] = "iv8-core/native_env.rs"

    screen = build_repair_brief({"ticket_id": "t-screen", "source_vector": "V015"})
    screen["readiness"] = "ready"
    screen["gap_class"] = "value_mismatch"
    screen["risk_level"] = "low"
    screen["l3_owner_module"] = "iv8-surface"

    result = select_runtime_briefs([nav, screen], owner_path=SCREEN_WINDOW_OWNER_PATH)
    assert len(result) == 1
    assert result[0]["ticket_id"] == "t-screen"


def test_validate_runtime_brief_screen_owner_vec_non_screen_rejected():
    from tools.diagnostic_bridge import (
        SCREEN_WINDOW_OWNER_PATH,
        build_repair_brief,
        validate_runtime_brief,
    )

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    brief["readiness"] = "ready"
    brief["gap_class"] = "value_mismatch"
    brief["l3_owner_module"] = "iv8-surface"
    result = validate_runtime_brief(brief, owner_path=SCREEN_WINDOW_OWNER_PATH)
    assert result["valid"] is False
    assert any("not_in_runtime_set" in r for r in result["blocked_reasons"])


def test_validate_runtime_brief_screen_owner_accepts_screen_vector():
    from tools.diagnostic_bridge import (
        SCREEN_WINDOW_OWNER_PATH,
        build_repair_brief,
        validate_runtime_brief,
    )

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V015"})
    brief["readiness"] = "ready"
    brief["gap_class"] = "value_mismatch"
    brief["l3_owner_module"] = "iv8-surface"
    result = validate_runtime_brief(brief, owner_path=SCREEN_WINDOW_OWNER_PATH)
    assert result["valid"] is True
    assert result["blocked_reasons"] == []


# -- v0.8.45 L3 P2 UAData low-entropy boundary gates -------------------


def test_select_runtime_briefs_uadata_owner_selects_v014():
    from tools.diagnostic_bridge import (
        NAVIGATOR_UADATA_OWNER_PATH,
        build_repair_brief,
        select_runtime_briefs,
    )

    uad = build_repair_brief({"ticket_id": "t-uad", "source_vector": "V014"})
    uad["readiness"] = "ready"
    uad["gap_class"] = "value_mismatch"
    uad["risk_level"] = "low"
    uad["l3_owner_module"] = "iv8-core/native_env.rs"

    nav = build_repair_brief({"ticket_id": "t-nav", "source_vector": "V001"})
    nav["readiness"] = "ready"
    nav["gap_class"] = "value_mismatch"
    nav["risk_level"] = "low"
    nav["l3_owner_module"] = "iv8-core/native_env.rs"

    result = select_runtime_briefs(
        [uad, nav],
        owner_path=NAVIGATOR_UADATA_OWNER_PATH,
    )
    ids = {b["ticket_id"] for b in result}
    assert "t-uad" in ids
    assert len(result) >= 1


def test_validate_runtime_brief_uadata_owner_accepts_v014():
    from tools.diagnostic_bridge import (
        NAVIGATOR_UADATA_OWNER_PATH,
        build_repair_brief,
        validate_runtime_brief,
    )

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V014"})
    brief["readiness"] = "ready"
    brief["gap_class"] = "value_mismatch"
    brief["l3_owner_module"] = "iv8-core/native_env.rs"
    result = validate_runtime_brief(brief, owner_path=NAVIGATOR_UADATA_OWNER_PATH)
    assert result["valid"] is True
    assert result["blocked_reasons"] == []


def test_validate_runtime_brief_uadata_owner_rejects_non_v014():
    from tools.diagnostic_bridge import (
        NAVIGATOR_UADATA_OWNER_PATH,
        build_repair_brief,
        validate_runtime_brief,
    )

    brief = build_repair_brief({"ticket_id": "t-001", "source_vector": "V001"})
    brief["readiness"] = "ready"
    brief["gap_class"] = "value_mismatch"
    brief["l3_owner_module"] = "iv8-core/native_env.rs"
    result = validate_runtime_brief(brief, owner_path=NAVIGATOR_UADATA_OWNER_PATH)
    assert result["valid"] is False
    assert any("not_in_runtime_set" in r for r in result["blocked_reasons"])


def test_select_runtime_briefs_default_includes_v014_in_navigator_set():
    from tools.diagnostic_bridge import build_repair_brief, select_runtime_briefs

    brief = build_repair_brief({"ticket_id": "t-uad", "source_vector": "V014"})
    brief["readiness"] = "ready"
    brief["gap_class"] = "value_mismatch"
    brief["risk_level"] = "low"
    brief["l3_owner_module"] = "iv8-core/native_env.rs"

    selected = select_runtime_briefs([brief])
    assert len(selected) == 1
    assert selected[0]["source_vector"] == "V014"
