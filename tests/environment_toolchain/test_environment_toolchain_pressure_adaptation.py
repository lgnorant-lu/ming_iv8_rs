from __future__ import annotations

import pytest
from iv8_rs.environment_toolchain_pressure_adaptation import (
    ATTEMPT_STATUSES,
    ELIGIBLE_PRESSURE_KINDS,
    ELIGIBLE_PROMOTION_LEVELS,
    ELIGIBLE_ROUTES,
    EVALUATOR_STATUSES,
    execute_pressure_adaptation_attempt,
    pressure_plan_item_attempt_eligibility,
    pressure_plan_item_candidate_query,
    reject_non_eligible,
    validate_attempt_status,
    validate_evaluator_status,
)


def _eligible_item(**overrides) -> dict:
    base = {
        "plan_item_id": "pressure.sample_001",
        "source_kind": "pressure_signal",
        "source_id": "sample_001",
        "input_kind": "direct_js",
        "execution_mode": "browser_like_global",
        "failure_kind": "missing_global_symbol",
        "pressure_kind": "web_api_surface",
        "probe_role": "shape_surface",
        "bridge_level": "B0",
        "promotion_level": "candidate_pack",
        "route": "candidate_pack_review",
        "route_owner": "probe_pack_review",
        "bridge_capability": "browser_like_window",
        "observation_status": "method_reference_only",
        "required_review": "candidate_pack_review",
        "review_status": "review_only",
        "blocked_reasons": [],
        "evidence_ceiling": "diagnostic_only",
        "apply_authorized": False,
        "writes": [],
    }
    base.update(overrides)
    return base


def test_attempt_statuses_are_stable():
    assert ATTEMPT_STATUSES == {
        "not_requested",
        "not_eligible",
        "eligible_not_run",
        "attempted",
        "improved",
        "unchanged",
        "regressed",
        "blocked",
    }


def test_evaluator_statuses_are_stable():
    assert EVALUATOR_STATUSES == {
        "diagnostic_only",
        "improvement_observed",
        "no_change_observed",
        "regression_observed",
        "blocked_by_boundary",
        "blocked_by_policy",
        "blocked_by_target_flow",
    }


def test_evaluator_statuses_do_not_contain_pass_or_strong():
    for status in EVALUATOR_STATUSES:
        assert status not in {"pass", "strong_evidence"}


def test_attempt_statuses_do_not_contain_pass_or_strong():
    for status in ATTEMPT_STATUSES:
        assert status not in {"pass", "strong_evidence"}


def test_validate_attempt_status_accepts_valid():
    assert validate_attempt_status("eligible_not_run") == "eligible_not_run"
    assert validate_attempt_status("blocked") == "blocked"


def test_validate_attempt_status_rejects_unknown():
    with pytest.raises(ValueError, match="unknown attempt status"):
        validate_attempt_status("pass")


def test_validate_evaluator_status_accepts_valid():
    assert validate_evaluator_status("diagnostic_only") == "diagnostic_only"
    assert validate_evaluator_status("improvement_observed") == "improvement_observed"


def test_validate_evaluator_status_rejects_unknown():
    with pytest.raises(ValueError, match="unknown evaluator status"):
        validate_evaluator_status("pass")


def test_eligible_item_returns_eligible_not_run():
    assert pressure_plan_item_attempt_eligibility(_eligible_item()) == "eligible_not_run"


def test_blocked_review_status_is_blocked():
    item = _eligible_item(review_status="blocked")
    assert pressure_plan_item_attempt_eligibility(item) == "blocked"


def test_non_empty_blocked_reasons_is_blocked():
    item = _eligible_item(blocked_reasons=["boundary_blocked"])
    assert pressure_plan_item_attempt_eligibility(item) == "blocked"


def test_non_candidate_pack_review_route_is_not_eligible():
    item = _eligible_item(route="observe_only")
    assert pressure_plan_item_attempt_eligibility(item) == "not_eligible"


def test_native_substrate_review_route_is_not_eligible():
    item = _eligible_item(route="native_substrate_review")
    assert pressure_plan_item_attempt_eligibility(item) == "not_eligible"


def test_profile_pack_review_route_is_not_eligible():
    item = _eligible_item(route="profile_pack_review", promotion_level="profile_candidate")
    assert pressure_plan_item_attempt_eligibility(item) == "not_eligible"


def test_local_prelude_required_route_is_not_eligible():
    item = _eligible_item(route="local_prelude_required", promotion_level="fixture_only")
    assert pressure_plan_item_attempt_eligibility(item) == "not_eligible"


def test_bridge_capability_review_route_is_not_eligible():
    item = _eligible_item(route="bridge_capability_review")
    assert pressure_plan_item_attempt_eligibility(item) == "not_eligible"


def test_probe_pack_review_route_is_not_eligible():
    item = _eligible_item(route="probe_pack_review")
    assert pressure_plan_item_attempt_eligibility(item) == "not_eligible"


def test_blocked_target_flow_route_is_not_eligible():
    item = _eligible_item(route="blocked_target_flow", review_status="blocked",
                          blocked_reasons=["endpoint"])
    assert pressure_plan_item_attempt_eligibility(item) == "not_eligible"


def test_non_candidate_pack_promotion_is_not_eligible():
    item = _eligible_item(promotion_level="observe_only")
    assert pressure_plan_item_attempt_eligibility(item) == "not_eligible"


def test_fixture_only_promotion_is_not_eligible():
    item = _eligible_item(promotion_level="fixture_only", route="local_prelude_required")
    assert pressure_plan_item_attempt_eligibility(item) == "not_eligible"


def test_target_flow_term_in_item_is_blocked():
    item = _eligible_item()
    item["endpoint"] = "https://example.invalid/sign"
    assert pressure_plan_item_attempt_eligibility(item) == "blocked"


def test_package_specific_name_in_item_is_blocked():
    item = _eligible_item()
    item["runtime"] = "playwright"
    assert pressure_plan_item_attempt_eligibility(item) == "blocked"


def test_jsdom_name_in_item_is_blocked():
    item = _eligible_item()
    item["runner"] = "jsdom"
    assert pressure_plan_item_attempt_eligibility(item) == "blocked"


def test_non_diagnostic_evidence_ceiling_is_blocked():
    item = _eligible_item(evidence_ceiling="strong")
    assert pressure_plan_item_attempt_eligibility(item) == "blocked"


def test_weak_evidence_ceiling_is_accepted():
    item = _eligible_item(evidence_ceiling="weak")
    assert pressure_plan_item_attempt_eligibility(item) == "eligible_not_run"


def test_eligibility_preserves_existing_item_keys():
    item = _eligible_item()
    result = pressure_plan_item_attempt_eligibility(item)
    assert result == "eligible_not_run"
    assert item["apply_authorized"] is False
    assert item["writes"] == []
    assert item["evidence_ceiling"] == "diagnostic_only"


def test_eligible_routes_are_stable():
    assert ELIGIBLE_ROUTES == {"candidate_pack_review"}


def test_eligible_promotion_levels_are_stable():
    assert ELIGIBLE_PROMOTION_LEVELS == {"candidate_pack"}


def test_reject_non_eligible_raises_for_not_eligible():
    with pytest.raises(ValueError, match="not eligible"):
        reject_non_eligible("not_eligible")


def test_reject_non_eligible_raises_for_blocked():
    with pytest.raises(ValueError, match="not eligible"):
        reject_non_eligible("blocked")


def test_reject_non_eligible_passes_for_eligible_not_run():
    assert reject_non_eligible("eligible_not_run") == "eligible_not_run"


def test_reject_non_eligible_passes_for_attempted():
    assert reject_non_eligible("attempted") == "attempted"


def test_candidate_query_returns_result_for_eligible_item():
    item = _eligible_item(pressure_kind="web_api_surface")
    result = pressure_plan_item_candidate_query(item)
    assert result["plan_item_id"] == item["plan_item_id"]
    assert result["attempt_status"] == "eligible_not_run"
    assert result["apply_authorized"] is False
    assert result["writes"] == []


def test_candidate_query_rejects_non_eligible_route():
    item = _eligible_item(route="observe_only")
    with pytest.raises(ValueError, match="not eligible"):
        pressure_plan_item_candidate_query(item)


def test_candidate_query_rejects_blocked_item():
    item = _eligible_item(review_status="blocked")
    with pytest.raises(ValueError, match="not eligible"):
        pressure_plan_item_candidate_query(item)


def test_candidate_query_network_surface_produces_synthetic_gaps():
    item = _eligible_item(pressure_kind="network_surface")
    result = pressure_plan_item_candidate_query(item)
    assert result["attempt_status"] == "eligible_not_run"
    assert result["apply_authorized"] is False
    assert result["synthetic_gap_count"] > 0


def test_candidate_query_dom_surface_produces_synthetic_gaps():
    item = _eligible_item(pressure_kind="dom_surface")
    result = pressure_plan_item_candidate_query(item)
    assert result["attempt_status"] == "eligible_not_run"
    assert result["apply_authorized"] is False
    assert result["synthetic_gap_count"] > 0


def test_candidate_query_ineligible_pressure_kind_produces_empty():
    item = _eligible_item(pressure_kind="runtime_stability")
    result = pressure_plan_item_candidate_query(item)
    assert result["synthetic_gap_count"] == 0
    assert result["candidate_count"] == 0
    assert result["apply_authorized"] is False


def test_eligible_pressure_kinds_are_stable():
    assert ELIGIBLE_PRESSURE_KINDS == frozenset({
        "web_api_surface",
        "network_surface",
        "dom_surface",
    })


def test_candidate_query_result_is_diagnostic_only():
    item = _eligible_item(pressure_kind="web_api_surface")
    result = pressure_plan_item_candidate_query(item)
    assert result["apply_authorized"] is False
    assert result["writes"] == []
    assert "candidates" in result
    assert "synthetic_gap_count" in result


def test_execute_attempt_returns_result():
    item = _eligible_item()
    candidate = {"target": "screen.width", "value_preview": 1440, "patch_id": "test.v0"}
    result = execute_pressure_adaptation_attempt(item, candidate, js_source="")
    assert result["plan_item_id"] == item["plan_item_id"]
    assert result["attempt_status"] in ATTEMPT_STATUSES
    assert result["apply_authorized"] is False
    assert result["writes"] == []
    assert "before" in result
    assert "after" in result
    assert "delta" in result


def test_execute_attempt_rejects_blocked_item():
    item = _eligible_item(review_status="blocked")
    with pytest.raises(ValueError, match="not eligible"):
        execute_pressure_adaptation_attempt(item, {}, js_source="")


def test_execute_attempt_rejects_non_eligible_route():
    item = _eligible_item(route="observe_only")
    with pytest.raises(ValueError, match="not eligible"):
        execute_pressure_adaptation_attempt(item, {}, js_source="")


def test_execute_attempt_result_has_evaluator_status():
    item = _eligible_item()
    candidate = {"target": "screen.width", "value_preview": 1440, "patch_id": "test.v0"}
    result = execute_pressure_adaptation_attempt(item, candidate, js_source="")
    assert result["evaluator_status"] in EVALUATOR_STATUSES
    assert result["evidence_ceiling"] == "diagnostic_only"


def test_execute_attempt_result_applies_candidate_target():
    item = _eligible_item()
    candidate = {"target": "navigator.languages", "value_preview": ["en-US", "en"],
                 "patch_id": "test.v0"}
    result = execute_pressure_adaptation_attempt(item, candidate, js_source="")
    assert result["candidate_target"] == "navigator.languages"
    assert result["candidate_patch_id"] == "test.v0"
