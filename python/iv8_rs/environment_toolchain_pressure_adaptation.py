"""Diagnostic-only pressure-to-adaptation attempt helpers.

Constants and validators for pressure-aware controlled adaptation
attempts.  This module is model-first and diagnostic-only: it defines
status vocabularies, eligibility checks, and candidate query helpers,
but does not execute candidates, create JS contexts, or apply values.
"""

from __future__ import annotations

from typing import Any

from iv8_rs.environment_toolchain_bridge_contract import (
    _PACKAGE_SPECIFIC_NAMES,
    _TARGET_FLOW_TERMS,
    ALLOWED_EVIDENCE_CEILINGS,
)
from iv8_rs.environment_toolchain_models import EnvironmentGap

ATTEMPT_STATUSES = frozenset({
    "not_requested",
    "not_eligible",
    "eligible_not_run",
    "attempted",
    "improved",
    "unchanged",
    "regressed",
    "blocked",
})

EVALUATOR_STATUSES = frozenset({
    "diagnostic_only",
    "improvement_observed",
    "no_change_observed",
    "regression_observed",
    "blocked_by_boundary",
    "blocked_by_policy",
    "blocked_by_target_flow",
})

ELIGIBLE_ROUTES = frozenset({
    "candidate_pack_review",
})

ELIGIBLE_PROMOTION_LEVELS = frozenset({
    "candidate_pack",
})


def validate_attempt_status(status: str) -> str:
    if status not in ATTEMPT_STATUSES:
        raise ValueError(f"unknown attempt status: {status}")
    return status


def validate_evaluator_status(status: str) -> str:
    if status not in EVALUATOR_STATUSES:
        raise ValueError(f"unknown evaluator status: {status}")
    return status


def pressure_plan_item_attempt_eligibility(item: dict) -> str:
    """Return the attempt eligibility status for a pressure plan item.

    Returns one of:
      - ``eligible_not_run`` when the item passes all checks.
      - ``not_eligible`` when the route or promotion level is ineligible.
      - ``blocked`` when the item is otherwise blocked.
    """
    route = str(item.get("route", ""))
    if route not in ELIGIBLE_ROUTES:
        return "not_eligible"

    review_status = str(item.get("review_status", ""))
    if review_status == "blocked":
        return "blocked"

    blocked_reasons = item.get("blocked_reasons", [])
    if blocked_reasons:
        return "blocked"

    promotion_level = str(item.get("promotion_level", ""))
    if promotion_level not in ELIGIBLE_PROMOTION_LEVELS:
        return "not_eligible"

    evidence_ceiling = str(item.get("evidence_ceiling", ""))
    if evidence_ceiling not in ALLOWED_EVIDENCE_CEILINGS:
        return "blocked"

    serialized = repr(item).lower()
    if any(term in serialized for term in _TARGET_FLOW_TERMS):
        return "blocked"
    if any(name in serialized for name in _PACKAGE_SPECIFIC_NAMES):
        return "blocked"

    return "eligible_not_run"


def reject_non_eligible(status: str) -> str:
    if status not in {"eligible_not_run", "attempted"}:
        raise ValueError(
            f"attempt is not eligible to run: status is {status}"
        )
    return status


_PRESSURE_KIND_TARGETS: dict[str, list[str]] = {
    "web_api_surface": [
        "Request",
        "Response",
        "Headers",
        "fetch",
        "XMLHttpRequest",
        "navigator",
    ],
    "network_surface": [
        "Request",
        "Response",
        "Headers",
        "fetch",
        "XMLHttpRequest",
    ],
    "dom_surface": [
        "document",
        "document.body",
    ],
}

ELIGIBLE_PRESSURE_KINDS: frozenset[str] = frozenset(_PRESSURE_KIND_TARGETS)


def pressure_plan_item_candidate_query(
    plan_item: dict[str, Any],
    *,
    candidate_pack: Any | None = "chrome_generic",
) -> dict[str, Any]:
    """Query candidates for an eligible pressure plan item without applying them."""
    from iv8_rs.environment_toolchain_candidate_mapping import map_gaps_to_candidates

    status = pressure_plan_item_attempt_eligibility(plan_item)
    reject_non_eligible(status)

    gaps = _pressure_plan_item_to_gaps(plan_item)
    candidates = map_gaps_to_candidates(gaps, candidate_pack=candidate_pack)

    candidate_dicts: list[dict[str, Any]] = []
    for candidate in candidates:
        try:
            candidate_dicts.append(
                candidate.to_dict() if hasattr(candidate, "to_dict") else dict(candidate)
            )
        except Exception:
            candidate_dicts.append({"patch_id": str(getattr(candidate, "patch_id", "unknown"))})

    return {
        "plan_item_id": plan_item["plan_item_id"],
        "pressure_kind": plan_item["pressure_kind"],
        "attempt_status": status,
        "synthetic_gap_count": len(gaps),
        "candidates": candidate_dicts,
        "candidate_count": len(candidate_dicts),
        "apply_authorized": False,
        "writes": [],
    }


def _pressure_plan_item_to_gaps(plan_item: dict[str, Any]) -> list[EnvironmentGap]:
    pressure_kind = str(plan_item.get("pressure_kind", ""))
    if pressure_kind not in ELIGIBLE_PRESSURE_KINDS:
        return []

    targets = _PRESSURE_KIND_TARGETS.get(pressure_kind, [])
    gaps: list[EnvironmentGap] = []
    for i, target in enumerate(targets):
        gaps.append(
            EnvironmentGap(
                probe_id=f"pressure.{plan_item.get('plan_item_id', 'unknown')}.{i}",
                target=target,
                gap_class="value_mismatch",
                category="value",
                expected=None,
                actual=None,
            )
        )
    return gaps
