"""Review-only pressure-to-plan helpers for Environment Toolchain reports."""

from __future__ import annotations

from typing import Any

from iv8_rs.environment_toolchain_boundary import validate_bypass_boundary
from iv8_rs.environment_toolchain_bridge_contract import (
    _PACKAGE_SPECIFIC_NAMES,
    _TARGET_FLOW_TERMS,
    BRIDGE_LEVELS,
    check_target_flow_terms,
)
from iv8_rs.environment_toolchain_probe_taxonomy import pressure_kind_probe_route

BRIDGE_CAPABILITIES = frozenset({
    "dom_fixture_runtime",
    "browser_like_window",
    "script_tag_bootstrap",
    "prelude_bootstrap",
    "network_shape_stub",
    "descriptor_probe_surface",
    "event_loop_semantics_probe",
    "external_environment_sidecar",
    "native_substrate_candidate",
})

ROUTES = frozenset({
    "candidate_pack_review",
    "profile_pack_review",
    "local_prelude_required",
    "bridge_capability_review",
    "native_substrate_review",
    "probe_pack_review",
    "blocked_target_flow",
    "observe_only",
})

_PROMOTION_ROUTES = {
    "observe_only": ("observe_only", None, "pressure_observation_review"),
    "fixture_only": ("local_prelude_required", "prelude_bootstrap", "local_fixture_review"),
    "profile_candidate": ("profile_pack_review", None, "profile_pack_review"),
    "candidate_pack": ("candidate_pack_review", None, "candidate_pack_review"),
    "generic_substrate_candidate": (
        "native_substrate_review",
        "native_substrate_candidate",
        "native_substrate_review",
    ),
    "default_substrate": (
        "native_substrate_review",
        "native_substrate_candidate",
        "default_substrate_promotion_review",
    ),
}

_PRESSURE_CAPABILITIES = {
    "input_normalization": "script_tag_bootstrap",
    "web_api_surface": "browser_like_window",
    "dom_surface": "dom_fixture_runtime",
    "descriptor_shape": "descriptor_probe_surface",
    "page_lifecycle": "script_tag_bootstrap",
    "prelude_contract": "prelude_bootstrap",
    "network_surface": "network_shape_stub",
    "timing_surface": "event_loop_semantics_probe",
    "event_loop_semantics": "event_loop_semantics_probe",
    "runtime_stability": "native_substrate_candidate",
    "analysis_observability": "external_environment_sidecar",
}


def pressure_report_to_plan_item(pressure_report: Any) -> dict[str, Any]:
    """Map an EnvironmentPressureReport to one review-only plan item."""
    data = (
        pressure_report.to_dict()
        if hasattr(pressure_report, "to_dict")
        else dict(pressure_report)
    )
    pressure = dict(data.get("pressure", {}))
    promotion = dict(data.get("promotion", {}))
    promotion_level = str(promotion.get("level", "observe_only"))
    route, bridge_capability, required_review = _PROMOTION_ROUTES.get(
        promotion_level,
        ("observe_only", None, "pressure_observation_review"),
    )
    pressure_kind = str(pressure.get("pressure_kind", "analysis_observability"))
    taxonomy_route = _taxonomy_route_for_pressure(pressure_kind)
    bridge_capability = _bridge_capability_for_pressure(pressure_kind, bridge_capability)
    blocked_reasons = _blocked_reasons(data, route, bridge_capability)
    route_owner = taxonomy_route["route_owner"]
    if blocked_reasons:
        route = "blocked_target_flow"
        route_owner = "blocked_target_flow"
        required_review = "boundary_review"
    review_status = (
        "blocked"
        if blocked_reasons or promotion_level == "default_substrate"
        else "review_only"
    )
    if (
        promotion_level == "default_substrate"
        and "separate_promotion_review_required" not in blocked_reasons
    ):
        blocked_reasons.append("separate_promotion_review_required")
        review_status = "blocked"
    target_flow_in_blocked = bool(set(blocked_reasons) & _TARGET_FLOW_TERMS)
    if review_status == "blocked" and target_flow_in_blocked:
        observation_status = "redaction_blocked"
    elif review_status == "blocked":
        observation_status = "boundary_blocked"
    else:
        observation_status = "method_reference_only"
    bridge_level = "B0"
    return {
        "plan_item_id": f"pressure.{data.get('sample_id', 'unknown')}",
        "source_kind": "pressure_signal",
        "source_id": str(data.get("sample_id", "unknown")),
        "input_kind": data.get("input_kind"),
        "execution_mode": data.get("execution_mode"),
        "failure_kind": data.get("failure_kind"),
        "pressure_kind": pressure_kind,
        "probe_role": taxonomy_route["probe_role"],
        "bridge_level": bridge_level,
        "promotion_level": promotion_level,
        "route": route,
        "route_owner": route_owner,
        "bridge_capability": bridge_capability,
        "observation_status": observation_status,
        "required_review": required_review,
        "review_status": review_status,
        "blocked_reasons": blocked_reasons,
        "evidence_ceiling": "diagnostic_only",
        "apply_authorized": False,
        "writes": [],
    }


def _taxonomy_route_for_pressure(pressure_kind: str) -> dict[str, str]:
    try:
        return pressure_kind_probe_route(pressure_kind)
    except ValueError:
        return pressure_kind_probe_route("analysis_observability")


def pressure_plan_summary(items: list[dict[str, Any]]) -> dict[str, Any]:
    """Build a review-only summary for pressure-backed plan items."""
    blocked_count = sum(1 for item in items if item["review_status"] == "blocked")
    route_counts: dict[str, int] = dict.fromkeys(sorted(ROUTES), 0)
    capability_counts: dict[str, int] = dict.fromkeys(sorted(BRIDGE_CAPABILITIES), 0)
    bridge_level_counts: dict[str, int] = dict.fromkeys(sorted(BRIDGE_LEVELS), 0)
    observation_status_counts: dict[str, int] = {}
    for item in items:
        route_counts[item["route"]] = route_counts.get(item["route"], 0) + 1
        capability = item.get("bridge_capability")
        if capability in capability_counts:
            capability_counts[capability] = capability_counts.get(capability, 0) + 1
        level = item.get("bridge_level", "B0")
        bridge_level_counts[level] = bridge_level_counts.get(level, 0) + 1
        status = item.get("observation_status", "not_configured")
        observation_status_counts[status] = observation_status_counts.get(status, 0) + 1
    return {
        "enabled": True,
        "review_status": "blocked" if blocked_count else "review_only",
        "evidence_ceiling": "diagnostic_only",
        "apply_authorized": False,
        "writes": [],
        "item_count": len(items),
        "blocked_item_count": blocked_count,
        "review_only_item_count": len(items) - blocked_count,
        "route_counts": route_counts,
        "bridge_capability_counts": capability_counts,
        "bridge_level_counts": bridge_level_counts,
        "observation_status_counts": observation_status_counts,
        "blocked_actions": [
            "runtime_apply",
            "profile_write",
            "manifest_write",
            "baseline_write",
            "sample_write",
            "source_write",
            "native_substrate_change",
            "external_package_adapter",
            "pass_promotion",
        ],
    }


def _bridge_capability_for_pressure(
    pressure_kind: str,
    preferred: str | None,
) -> str | None:
    capability = _PRESSURE_CAPABILITIES.get(pressure_kind) or preferred
    if capability not in BRIDGE_CAPABILITIES:
        return None
    return capability


def _blocked_reasons(
    data: dict[str, Any],
    route: str,
    bridge_capability: str | None,
) -> list[str]:
    payload = {
        "pressure": data.get("pressure", {}),
        "promotion": data.get("promotion", {}),
        "route": route,
        "bridge_capability": bridge_capability or "",
    }
    decision = validate_bypass_boundary(payload)
    blocked = list(decision.blocked_terms) if decision.decision == "blocked" else []
    blocked.extend(check_target_flow_terms(payload))
    serialized = repr(payload).lower()
    blocked.extend(name for name in _PACKAGE_SPECIFIC_NAMES if name in serialized)
    return sorted(set(blocked))
