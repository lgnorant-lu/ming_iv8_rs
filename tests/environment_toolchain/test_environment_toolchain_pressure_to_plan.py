from __future__ import annotations

import pytest
from iv8_rs.environment_pressure import build_pressure_report
from iv8_rs.environment_toolchain import toolchain_report_to_dict
from iv8_rs.environment_toolchain_bridge_contract import (
    BRIDGE_LEVELS,
    OBSERVATION_STATUSES,
)
from iv8_rs.environment_toolchain_pressure_planning import (
    pressure_plan_summary,
    pressure_report_to_plan_item,
)
from iv8_rs.environment_toolchain_probe_taxonomy import BRIDGE_CAPABILITIES as TAXONOMY_CAPABILITIES
from iv8_rs.environment_toolchain_runtime import run_environment_toolchain


def diagnostics(data: dict, code: str) -> list[dict]:
    return [diagnostic for diagnostic in data["diagnostics"] if diagnostic["code"] == code]


def pressure_plan(source: str, message: str | None = None) -> dict:
    report = build_pressure_report("sample_001", source, message=message)
    return pressure_report_to_plan_item(report)


def test_pressure_plan_summary_is_review_only():
    report = run_environment_toolchain(
        "new Request('/x')",
        probe_pack="fingerprint.m1",
        profile=None,
        dry_run_planning=True,
        pressure_harness=True,
    )
    data = toolchain_report_to_dict(report)
    summary = diagnostics(data, "ENV_TOOLCHAIN_PRESSURE_PLAN_SUMMARY")[0]["details"]

    assert summary["review_status"] == "review_only"
    assert summary["evidence_ceiling"] == "diagnostic_only"
    assert summary["apply_authorized"] is False
    assert summary["writes"] == []


def test_pressure_plan_item_is_not_apply_authorization():
    report = run_environment_toolchain(
        "new Request('/x')",
        probe_pack="fingerprint.m1",
        profile=None,
        dry_run_planning=True,
        pressure_harness=True,
    )
    data = toolchain_report_to_dict(report)
    item = diagnostics(data, "ENV_TOOLCHAIN_PRESSURE_PLAN_ITEM")[0]["details"]

    assert item["apply_authorized"] is False
    assert item["writes"] == []
    assert data["applied_patches"] == []


def test_pressure_plan_emits_no_strong_or_pass_evidence():
    report = run_environment_toolchain(
        "new Request('/x')",
        probe_pack="fingerprint.m1",
        profile=None,
        dry_run_planning=True,
        pressure_harness=True,
    )
    data = toolchain_report_to_dict(report)

    assert {item["strength"] for item in data["evidence"]} <= {"diagnostic_only", "weak"}
    assert not any("PASS" in diagnostic["code"] for diagnostic in data["diagnostics"])


def test_candidate_pack_pressure_routes_to_review_not_apply():
    item = pressure_plan("new Request('/x')", "ReferenceError: Request is not defined")

    assert item["promotion_level"] == "candidate_pack"
    assert item["route"] == "candidate_pack_review"
    assert item["probe_role"] == "shape_surface"
    assert item["route_owner"] == "probe_pack_review"
    assert item["required_review"] == "candidate_pack_review"
    assert item["apply_authorized"] is False


def test_fixture_pressure_routes_to_local_prelude_or_probe_review():
    item = pressure_plan("$_ts.init()", "ReferenceError: $_ts is not defined")

    assert item["promotion_level"] == "fixture_only"
    assert item["route"] == "local_prelude_required"
    assert item["probe_role"] == "bridge_readiness"
    assert item["route_owner"] == "local_prelude_review"
    assert item["bridge_capability"] == "prelude_bootstrap"
    assert item["writes"] == []


def test_bridge_route_uses_capability_vocabulary_not_package_names():
    item = pressure_plan("document.body", "ReferenceError: document is not defined")
    serialized = repr(item).lower()

    assert item["bridge_capability"] == "dom_fixture_runtime"
    assert all(term not in serialized for term in ("jsdom", "sdenv", "happy-dom", "linkedom"))


def test_default_substrate_pressure_still_requires_review():
    report = build_pressure_report(
        "sample_001",
        "descriptor mismatch for navigator",
        message="descriptor mismatch for navigator",
        sample_count=3,
    )
    report.promotion.level = "default_substrate"
    item = pressure_report_to_plan_item(report)

    assert item["promotion_level"] == "default_substrate"
    assert item["route"] == "native_substrate_review"
    assert item["review_status"] == "blocked"
    assert "separate_promotion_review_required" in item["blocked_reasons"]


def test_pressure_harness_rejects_iterative_adaptation_until_dedicated_review():
    with pytest.raises(ValueError, match="pressure_harness cannot be combined"):
        run_environment_toolchain(
            "var x = 1;",
            probe_pack="fingerprint.m1",
            profile=None,
            adapt_runtime_safe=True,
            pressure_harness=True,
        )


def test_pressure_plan_rejects_target_flow_payload():
    report = build_pressure_report(
        "sample_001",
        "var x = 1;",
        message="ReferenceError: Request is not defined",
    )
    report.pressure.details["endpoint"] = "https://example.invalid/sign"
    item = pressure_report_to_plan_item(report)

    assert item["route"] == "blocked_target_flow"
    assert item["route_owner"] == "blocked_target_flow"
    assert "endpoint" in item["blocked_reasons"] or "signature" in item["blocked_reasons"]
    assert item["apply_authorized"] is False


def test_pressure_plan_does_not_read_source_ref():
    report = build_pressure_report(
        "source-ref-only.js",
        None,
        message="ReferenceError: Request is not defined",
    )
    item = pressure_report_to_plan_item(report)

    assert item["source_id"] == "source-ref-only.js"
    assert item["writes"] == []


def test_family_pressure_cannot_generate_pressure_plan_candidates():
    item = pressure_plan("descriptor mismatch", "descriptor mismatch for navigator")

    assert item["route"] in {"observe_only", "native_substrate_review"}
    assert item["probe_role"] in {"baseline_surface", "descriptor_surface", "bridge_readiness"}
    assert item["route_owner"] in {"observe_only", "native_substrate_review", "probe_pack_review"}
    assert "candidate_id" not in item
    assert item["apply_authorized"] is False


def test_profile_route_does_not_write_profile_data():
    report = build_pressure_report(
        "sample_001",
        "var x = 1;",
        message="ReferenceError: navigator is not defined",
    )
    report.promotion.level = "profile_candidate"
    item = pressure_report_to_plan_item(report)

    assert item["route"] == "profile_pack_review"
    assert item["probe_role"] == "shape_surface"
    assert item["route_owner"] == "probe_pack_review"
    assert item["required_review"] == "profile_pack_review"
    assert item["writes"] == []


def test_pressure_plan_item_includes_review_only_taxonomy_fields():
    report = run_environment_toolchain(
        "new HXY_NOT_A_REAL_API()",
        probe_pack="fingerprint.m1",
        profile=None,
        dry_run_planning=True,
        pressure_harness=True,
    )
    data = toolchain_report_to_dict(report)
    item = diagnostics(data, "ENV_TOOLCHAIN_PRESSURE_PLAN_ITEM")[0]["details"]

    assert item["probe_role"] == "shape_surface"
    assert item["route_owner"] == "probe_pack_review"
    assert item["apply_authorized"] is False
    assert item["writes"] == []


def test_plan_item_includes_bridge_contract_context():
    item = pressure_plan("new Request('/x')", "ReferenceError: Request is not defined")

    assert "bridge_level" in item
    assert item["bridge_level"] in BRIDGE_LEVELS
    assert "observation_status" in item
    assert item["observation_status"] in OBSERVATION_STATUSES


def test_plan_item_bridge_level_defaults_to_b0():
    item = pressure_plan("new Request('/x')", "ReferenceError: Request is not defined")

    assert item["bridge_level"] == "B0"


def test_plan_item_review_only_observation_is_method_reference():
    item = pressure_plan("new Request('/x')", "ReferenceError: Request is not defined")

    assert item["review_status"] == "review_only"
    assert item["observation_status"] == "method_reference_only"


def test_plan_item_blocked_by_boundary_is_boundary_blocked():
    report = build_pressure_report(
        "sample_001",
        "descriptor mismatch for navigator",
        message="descriptor mismatch for navigator",
    )
    report.promotion.level = "default_substrate"
    item = pressure_report_to_plan_item(report)

    assert item["review_status"] == "blocked"
    assert item["observation_status"] == "boundary_blocked"


def test_plan_item_target_flow_blocked_is_redaction_blocked():
    report = build_pressure_report(
        "sample_001",
        "var x = 1;",
        message="ReferenceError: Request is not defined",
    )
    report.pressure.details["endpoint"] = "https://example.invalid/sign"
    item = pressure_report_to_plan_item(report)

    assert item["review_status"] == "blocked"
    assert item["observation_status"] == "redaction_blocked"


def test_plan_item_secret_in_payload_is_redaction_blocked():
    report = build_pressure_report(
        "sample_001",
        "var x = 1;",
        message="ReferenceError: Request is not defined",
    )
    report.pressure.details["secret"] = "abc123"
    item = pressure_report_to_plan_item(report)

    assert item["review_status"] == "blocked"
    assert item["observation_status"] == "redaction_blocked"
    assert "secret" in item["blocked_reasons"]


def test_summary_includes_bridge_level_and_observation_counts():
    items = [pressure_plan("new Request('/x')", "ReferenceError: Request is not defined")]
    summary = pressure_plan_summary(items)

    assert "bridge_level_counts" in summary
    assert "observation_status_counts" in summary
    assert summary["bridge_level_counts"].get("B0", 0) >= 1
    assert "method_reference_only" in summary["observation_status_counts"]


def test_bridge_capabilities_match_taxonomy():
    from iv8_rs.environment_toolchain_pressure_planning import BRIDGE_CAPABILITIES as PP_CAPS

    assert PP_CAPS == TAXONOMY_CAPABILITIES


def test_package_specific_detection_is_consistent_with_bridge_contract():
    report = build_pressure_report(
        "sample_001",
        "var x = 1;",
        message="ReferenceError: Request is not defined",
    )
    report.pressure.details["runtime"] = "jsdom"
    item = pressure_report_to_plan_item(report)

    assert item["review_status"] == "blocked"
    assert "jsdom" in item["blocked_reasons"]


def test_playwright_and_cdp_detected_as_package_specific():
    report = build_pressure_report(
        "sample_001",
        "var x = 1;",
        message="ReferenceError: Request is not defined",
    )
    report.pressure.details["runtime"] = "playwright"
    item = pressure_report_to_plan_item(report)

    assert item["review_status"] == "blocked"
    assert "playwright" in item["blocked_reasons"]


def test_cdp_detected_as_package_specific():
    report = build_pressure_report(
        "sample_001",
        "var x = 1;",
        message="ReferenceError: document is not defined",
    )
    report.pressure.details["driver"] = "cdp"
    item = pressure_report_to_plan_item(report)

    assert item["review_status"] == "blocked"
    assert "cdp" in item["blocked_reasons"]


def test_authorization_key_detected_as_target_flow():
    report = build_pressure_report(
        "sample_001",
        "var x = 1;",
        message="ReferenceError: Request is not defined",
    )
    report.pressure.details["authorization"] = "Bearer xyz"
    item = pressure_report_to_plan_item(report)

    assert item["review_status"] == "blocked"
    auth_found = (
        "authorization" in item["blocked_reasons"]
        or "authorization_header" in item["blocked_reasons"]
    )
    assert auth_found
    assert item["observation_status"] == "redaction_blocked"


def test_evidence_ceiling_remains_diagnostic_only():
    item = pressure_plan("new Request('/x')", "ReferenceError: Request is not defined")

    assert item["evidence_ceiling"] == "diagnostic_only"
    assert item["apply_authorized"] is False
    assert item["writes"] == []


def test_plan_item_no_source_ref_reads():
    report = build_pressure_report(
        "sample_001",
        "var x = 1;",
        message="ReferenceError: Request is not defined",
    )
    report.pressure.details["source_ref"] = "/etc/passwd"
    item = pressure_report_to_plan_item(report)

    no_source_ref_read = (
        "source_ref" not in item["blocked_reasons"]
        or item["route"] != "blocked_target_flow"
        or item["writes"] == []
    )
    assert no_source_ref_read
