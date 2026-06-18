from __future__ import annotations

import pytest
from iv8_rs.environment_toolchain_bridge_contract import (
    _PACKAGE_SPECIFIC_NAMES,
    ALLOWED_EVIDENCE_CEILINGS,
    BLOCKED_RESULT_STATES,
    BRIDGE_CAPABILITIES,
    BRIDGE_LEVELS,
    OBSERVATION_STATUSES,
    check_target_flow_terms,
    validate_bridge_capability,
    validate_bridge_level,
    validate_contract_writes,
    validate_evidence_ceiling,
    validate_observation_status,
    validate_package_neutral,
)


def test_bridge_levels_are_stable():
    assert BRIDGE_LEVELS == {"B0", "B1", "B2", "B3", "B4"}


def test_observation_statuses_are_diagnostic_only():
    assert "pass" not in OBSERVATION_STATUSES
    assert "strong_evidence" not in OBSERVATION_STATUSES


def test_blocked_result_states_are_not_in_observation_statuses():
    assert BLOCKED_RESULT_STATES.isdisjoint(OBSERVATION_STATUSES)


def test_evidence_ceiling_defaults_to_diagnostic_only():
    assert ALLOWED_EVIDENCE_CEILINGS == {"diagnostic_only", "weak"}
    assert validate_evidence_ceiling("diagnostic_only") == "diagnostic_only"
    assert validate_evidence_ceiling("weak") == "weak"


def test_evidence_ceiling_rejects_non_diagnostic():
    with pytest.raises(ValueError, match="diagnostic_only or weak"):
        validate_evidence_ceiling("strong")
    with pytest.raises(ValueError, match="diagnostic_only or weak"):
        validate_evidence_ceiling("pass")


def test_bridge_capabilities_are_package_neutral():
    serialized = repr(sorted(BRIDGE_CAPABILITIES)).lower()
    for name in _PACKAGE_SPECIFIC_NAMES:
        assert name not in serialized


def test_bridge_capabilities_match_taxonomy_capabilities():
    from iv8_rs.environment_toolchain_probe_taxonomy import (
        BRIDGE_CAPABILITIES as TAXONOMY_CAPABILITIES,
    )

    assert BRIDGE_CAPABILITIES == TAXONOMY_CAPABILITIES


def test_package_names_rejected_as_route_owner():
    for name in _PACKAGE_SPECIFIC_NAMES:
        with pytest.raises(ValueError, match="package-specific name"):
            validate_package_neutral(name)


def test_generic_capability_name_passes_validation():
    assert validate_package_neutral("dom_fixture_runtime") == "dom_fixture_runtime"
    assert validate_package_neutral("network_shape_stub") == "network_shape_stub"


def test_contract_writes_must_be_empty():
    assert validate_contract_writes([]) == []
    with pytest.raises(ValueError, match="must be empty"):
        validate_contract_writes(["profile_write"])
    with pytest.raises(ValueError, match="must be empty"):
        validate_contract_writes([{"path": "manifest.json"}])


def test_bridge_level_validation_rejects_unknown():
    assert validate_bridge_level("B1") == "B1"
    with pytest.raises(ValueError, match="unknown bridge level"):
        validate_bridge_level("B5")
    with pytest.raises(ValueError, match="unknown bridge level"):
        validate_bridge_level("production")


def test_observation_status_validation_rejects_pass():
    assert validate_observation_status("review_only") == "review_only"
    with pytest.raises(ValueError, match="unknown observation status"):
        validate_observation_status("pass")
    with pytest.raises(ValueError, match="unknown observation status"):
        validate_observation_status("strong_evidence")


def test_bridge_capability_validation_rejects_unknown():
    assert validate_bridge_capability("dom_fixture_runtime") == "dom_fixture_runtime"
    with pytest.raises(ValueError, match="unknown bridge capability"):
        validate_bridge_capability("jsdom")
    with pytest.raises(ValueError, match="unknown bridge capability"):
        validate_bridge_capability("playwright")


def test_target_flow_terms_are_detected_in_payload():
    payload = {"endpoint": "https://example.invalid/sign", "cookie": "session=abc"}
    found = check_target_flow_terms(payload)
    assert "endpoint" in found
    assert "cookie" in found
    assert len(found) >= 2


def test_target_flow_terms_are_detected_in_nested_payload():
    payload = {"details": {"entry": {"request_body": '{"sign":"abc"}'}}}
    found = check_target_flow_terms(payload)
    assert "request_body" in found or "request body" in found


def test_target_flow_terms_are_detected_in_string():
    found = check_target_flow_terms("copy token from endpoint before rerun")
    assert "token" in found
    assert "endpoint" in found


def test_clean_payload_returns_empty():
    assert check_target_flow_terms({"probe_role": "baseline_surface"}) == []
    assert check_target_flow_terms("") == []
    assert check_target_flow_terms({}) == []


def test_bridge_contract_does_not_weaken_probe_taxonomy_boundaries():
    from iv8_rs.environment_toolchain_probe_taxonomy import (
        BRIDGE_CAPABILITIES as TAX_CAPS,
    )

    assert BRIDGE_CAPABILITIES == TAX_CAPS


def test_bridge_contract_does_not_weaken_pressure_package_detection():
    from iv8_rs.environment_pressure import build_pressure_report
    from iv8_rs.environment_toolchain_pressure_planning import pressure_report_to_plan_item

    report = build_pressure_report(
        "sample_001",
        "var x = 1;",
        message="ReferenceError: document is not defined",
    )
    report.pressure.details["runtime"] = "playwright"
    item = pressure_report_to_plan_item(report)

    assert item["review_status"] == "blocked"
    assert "playwright" in item["blocked_reasons"]


def test_bridge_contract_does_not_weaken_target_flow_detection():
    from iv8_rs.environment_pressure import build_pressure_report
    from iv8_rs.environment_toolchain_pressure_planning import pressure_report_to_plan_item

    report = build_pressure_report(
        "sample_001",
        "var x = 1;",
        message="ReferenceError: Request is not defined",
    )
    report.pressure.details["domain"] = "example.com"
    item = pressure_report_to_plan_item(report)

    assert item["review_status"] == "blocked"
    assert "domain" in item["blocked_reasons"]


def test_bridge_contract_does_not_introduce_source_ref_reads():
    from iv8_rs.environment_pressure import build_pressure_report
    from iv8_rs.environment_toolchain_pressure_planning import pressure_report_to_plan_item

    report = build_pressure_report(
        "sample_001",
        "var x = 1;",
        message="ReferenceError: Request is not defined",
    )
    report.pressure.details["source_ref"] = "/path/to/sample.js"
    item = pressure_report_to_plan_item(report)

    assert item["apply_authorized"] is False
    assert item["writes"] == []


def test_bridge_contract_does_not_introduce_apply_or_writes():
    from iv8_rs.environment_pressure import build_pressure_report
    from iv8_rs.environment_toolchain_pressure_planning import pressure_report_to_plan_item

    item = pressure_report_to_plan_item(build_pressure_report(
            "new Request('/x')",
            "ReferenceError: Request is not defined",
        )
    )

    assert item["apply_authorized"] is False
    assert item["writes"] == []
    assert item["evidence_ceiling"] == "diagnostic_only"


def test_bridge_contract_observation_is_not_pass_or_strong():
    for status in OBSERVATION_STATUSES:
        assert status not in BLOCKED_RESULT_STATES
