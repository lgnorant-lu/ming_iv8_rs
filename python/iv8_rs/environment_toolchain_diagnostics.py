"""Diagnostic builders for Environment Toolchain reports.

This module contains behavior-preserving diagnostic record builders extracted
from `environment_toolchain_runtime.py`. It must not import the runtime module.
"""

from __future__ import annotations

from typing import Any

from iv8_rs.environment_toolchain_models import (
    AdaptationIteration,
    EnvironmentGap,
    FamilyPressure,
    ProfileCoherenceGroup,
)
from iv8_rs.environment_toolchain_static import (
    _ADAPTATION_STOP_REASONS,
    _ALLOWED_PRESSURE_CATEGORIES,
    _ALLOWED_TARGET_FAMILIES,
    _DRY_RUN_ALLOWED_STATUSES,
    _ROLLBACK_ALLOWED_SCOPES,
    _ROLLBACK_BLOCKED_SCOPES,
    _SCAFFOLD_GAP_ITEMS,
    _SUBSTRATE_COVERAGE_ITEMS,
)
from iv8_rs.experimental_report import ExperimentalDiagnosticRecord


def _adaptation_records(
    *,
    enabled: bool,
    max_iterations: int,
    iterations: list[AdaptationIteration],
    stop_reason: str,
    applied_candidates: list[Any],
):
    if stop_reason not in _ADAPTATION_STOP_REASONS:
        raise ValueError(f"invalid adaptation stop reason: {stop_reason}")
    records = [
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_ADAPTATION_SUMMARY",
            "info",
            {
                "enabled": enabled,
                "mode": "iterative_runtime_safe" if enabled else "report_only",
                "max_iterations": max_iterations,
                "iterations": len(iterations),
                "stop_reason": stop_reason,
                "applied_patch_ids": [candidate.patch_id for candidate in applied_candidates],
            },
        )
    ]
    records.extend(
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_ADAPTATION_ITERATION",
            "info",
            iteration.to_details(),
        )
        for iteration in iterations
    )
    return records


def _dry_run_planning_records(
    gaps: list[EnvironmentGap],
    *,
    candidate_pack_object: Any | None,
    environment: dict[str, Any] | None,
    candidate_registry: Any,
    classify_target_family: Any,
    validate_boundary: Any,
):
    explicit_environment = environment or {}
    items: list[dict[str, Any]] = []
    if candidate_pack_object is not None:
        registry = candidate_registry(candidate_pack_object)
        seen_patch_ids: set[str] = set()
        for gap in gaps:
            for candidate in registry.get(gap.target, []):
                if candidate.patch_id in seen_patch_ids:
                    continue
                gap_classes = set(candidate.validation.get("gap_classes", []))
                if gap_classes and gap.gap_class not in gap_classes:
                    continue
                seen_patch_ids.add(candidate.patch_id)
                items.append(_dry_run_plan_item(
                    candidate,
                    gap,
                    explicit_environment,
                    classify_target_family=classify_target_family,
                    validate_boundary=validate_boundary,
                ))

    blocked_count = sum(1 for item in items if item["planning_status"] != "eligible_for_review")
    eligible_count = len(items) - blocked_count
    summary = {
        "enabled": True,
        "apply_authorized": False,
        "writes": [],
        "review_status": "blocked" if blocked_count else "review_only",
        "evidence_ceiling": "diagnostic_only",
        "candidate_count": len(items),
        "eligible_for_review_count": eligible_count,
        "blocked_candidate_count": blocked_count,
        "required_review_count": len(items),
        "rollback_required_count": 0,
        "input_signal_counts": {
            "probe_gap_count": len(gaps),
            "candidate_pack_enabled": candidate_pack_object is not None,
            "explicit_environment_count": len(explicit_environment),
        },
        "blocked_actions": [
            "runtime_apply",
            "profile_write",
            "manifest_write",
            "baseline_write",
            "sample_write",
            "source_write",
            "pass_promotion",
        ],
    }
    records = [
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_DRY_RUN_PLAN_SUMMARY",
            "info" if blocked_count == 0 else "warn",
            summary,
        )
    ]
    records.extend(
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_DRY_RUN_PLAN_ITEM",
            "info" if item["planning_status"] == "eligible_for_review" else "warn",
            item,
        )
        for item in items
    )
    return records


def _dry_run_plan_item(
    candidate: Any,
    gap: EnvironmentGap,
    explicit_environment: dict[str, Any],
    *,
    classify_target_family: Any,
    validate_boundary: Any,
) -> dict[str, Any]:
    blocked_reasons: list[str] = []
    planning_status = "eligible_for_review"
    if candidate.target in explicit_environment:
        planning_status = "blocked_by_conflict"
        blocked_reasons.append("explicit_environment_precedence")
    elif candidate.policy != "runtime_safe":
        planning_status = "blocked_by_policy"
        blocked_reasons.append("non_runtime_safe_policy")
    else:
        decision = validate_boundary(candidate)
        if decision.decision == "blocked":
            planning_status = "blocked_by_boundary"
            blocked_reasons.extend(decision.blocked_terms)

    if planning_status not in _DRY_RUN_ALLOWED_STATUSES:
        raise ValueError(f"invalid dry-run planning status: {planning_status}")
    target_family = classify_target_family(candidate.target) or candidate.target_family
    return {
        "plan_item_id": candidate.patch_id,
        "candidate_id": candidate.patch_id,
        "target": candidate.target,
        "target_family": target_family,
        "coherence_group": target_family,
        "policy": candidate.policy,
        "planning_status": planning_status,
        "blocked_reasons": blocked_reasons,
        "required_reviews": ["environment_toolchain_review"],
        "rollback_required": False,
        "rollback_scope": candidate.metadata.get("rollback_scope", "context_only"),
        "evidence_ceiling": "diagnostic_only",
        "apply_authorized": False,
        "expected_probe_delta": list(
            candidate.metadata.get(
                "expected_probe_delta",
                candidate.validation.get("expected_delta", []),
            )
        ),
        "source_gap": {
            "probe_id": gap.probe_id,
            "gap_class": gap.gap_class,
            "category": gap.category,
        },
    }


def _rollback_diagnostic_records(
    gaps: list[EnvironmentGap],
    *,
    candidate_pack_object: Any | None,
    candidate_registry: Any,
    classify_target_family: Any,
    validate_boundary: Any,
):
    records_data: list[dict[str, Any]] = []
    if candidate_pack_object is not None:
        registry = candidate_registry(candidate_pack_object)
        seen_patch_ids: set[str] = set()
        for gap in gaps:
            for candidate in registry.get(gap.target, []):
                if candidate.patch_id in seen_patch_ids:
                    continue
                gap_classes = set(candidate.validation.get("gap_classes", []))
                if gap_classes and gap.gap_class not in gap_classes:
                    continue
                seen_patch_ids.add(candidate.patch_id)
                records_data.append(_rollback_record_details(
                    candidate,
                    gap,
                    classify_target_family=classify_target_family,
                    validate_boundary=validate_boundary,
                ))

    blocked_count = sum(1 for item in records_data if item["review_status"] == "blocked")
    summary = {
        "enabled": True,
        "writes": [],
        "review_status": "blocked" if blocked_count else "review_only",
        "evidence_ceiling": "diagnostic_only",
        "record_count": len(records_data),
        "blocked_record_count": blocked_count,
        "allowed_record_count": len(records_data) - blocked_count,
        "input_signal_counts": {
            "probe_gap_count": len(gaps),
            "candidate_pack_enabled": candidate_pack_object is not None,
        },
        "blocked_scopes": sorted(_ROLLBACK_BLOCKED_SCOPES),
        "allowed_scopes": sorted(_ROLLBACK_ALLOWED_SCOPES),
        "blocked_actions": [
            "rollback_file_write",
            "profile_write",
            "manifest_write",
            "baseline_write",
            "sample_write",
            "source_write",
            "native_substrate_change",
            "apply_authorization",
            "pass_promotion",
        ],
    }
    records = [
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_ROLLBACK_SUMMARY",
            "info" if blocked_count == 0 else "warn",
            summary,
        )
    ]
    records.extend(
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_ROLLBACK_RECORD",
            "info" if item["review_status"] != "blocked" else "warn",
            item,
        )
        for item in records_data
    )
    return records


def _rollback_record_details(
    candidate: Any,
    gap: EnvironmentGap,
    *,
    classify_target_family: Any,
    validate_boundary: Any,
) -> dict[str, Any]:
    scope = str(candidate.metadata.get(
        "rollback_scope",
        candidate.validation.get("rollback_scope", "context_only"),
    ))
    blocked_reasons: list[str] = []
    if scope in _ROLLBACK_BLOCKED_SCOPES:
        review_status = "blocked"
        blocked_reasons.append("persistent_scope_blocked")
        restore_strategy = "blocked"
    elif scope in _ROLLBACK_ALLOWED_SCOPES:
        review_status = "review_only"
        restore_strategy = "context_discard" if scope == "context_only" else "remove_value"
    else:
        review_status = "blocked"
        blocked_reasons.append("invalid_rollback_scope")
        restore_strategy = "blocked"

    target_family = classify_target_family(candidate.target) or candidate.target_family
    details = {
        "record_id": f"rollback.{candidate.patch_id}",
        "candidate_id": candidate.patch_id,
        "plan_item_id": candidate.patch_id,
        "target": candidate.target,
        "target_family": target_family,
        "scope": scope,
        "capture_before": list(candidate.metadata.get(
            "expected_probe_delta",
            candidate.validation.get("expected_delta", [candidate.target]),
        )),
        "restore_strategy": restore_strategy,
        "writes": [],
        "redactions": [],
        "review_status": review_status,
        "evidence_ceiling": "diagnostic_only",
        "blocked_reasons": blocked_reasons,
        "source_gap": {
            "probe_id": gap.probe_id,
            "gap_class": gap.gap_class,
            "category": gap.category,
        },
    }
    decision = validate_boundary(details)
    if decision.decision == "blocked":
        details["review_status"] = "blocked"
        details["restore_strategy"] = "blocked"
        details["blocked_reasons"] = sorted({
            *details["blocked_reasons"],
            *decision.blocked_terms,
        })
    return details


def _substrate_coverage_records():
    items = [_substrate_coverage_item_details(item) for item in _SUBSTRATE_COVERAGE_ITEMS]
    review_required_count = sum(
        1 for item in items if item["review_status"] in {"requires_review", "blocked"}
    )
    family_counts: dict[str, int] = {}
    for item in items:
        family_counts[item["surface_family"]] = family_counts.get(item["surface_family"], 0) + 1
    summary = {
        "enabled": True,
        "apply_authorized": False,
        "writes": [],
        "review_status": "requires_review" if review_required_count else "review_only",
        "evidence_ceiling": "diagnostic_only",
        "item_count": len(items),
        "review_required_count": review_required_count,
        "family_counts": family_counts,
        "blocked_actions": [
            "runtime_apply",
            "profile_write",
            "manifest_write",
            "baseline_write",
            "sample_write",
            "source_write",
            "native_hardening_without_review",
            "pass_promotion",
        ],
    }
    records = [
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_SUBSTRATE_COVERAGE_SUMMARY",
            "warn" if review_required_count else "info",
            summary,
        )
    ]
    records.extend(
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_SUBSTRATE_COVERAGE_ITEM",
            "warn" if item["review_status"] in {"requires_review", "blocked"} else "info",
            item,
        )
        for item in items
    )
    return records


def _substrate_coverage_item_details(item: dict[str, str]) -> dict[str, Any]:
    details: dict[str, Any] = {
        "surface_id": item["surface_id"],
        "surface_family": item["surface_family"],
        "owner": item["owner"],
        "coverage": item["coverage"],
        "boundary": item["boundary"],
        "review_status": item["review_status"],
        "gap_class": item["gap_class"],
        "gap": item["gap"],
        "apply_authorized": False,
        "writes": [],
        "evidence_ceiling": "diagnostic_only",
        "blocked_actions": [
            "runtime_apply",
            "profile_write",
            "native_hardening_without_review",
            "pass_promotion",
        ],
    }
    return details


def _scaffold_gap_records():
    items = [_scaffold_gap_item_details(item) for item in _SCAFFOLD_GAP_ITEMS]
    gap_class_counts: dict[str, int] = {}
    for item in items:
        gap_class_counts[item["gap_class"]] = gap_class_counts.get(item["gap_class"], 0) + 1
    high_priority_count = sum(1 for item in items if item["priority"] == "high")
    summary = {
        "enabled": True,
        "apply_authorized": False,
        "writes": [],
        "review_status": "review_only",
        "evidence_ceiling": "diagnostic_only",
        "gap_count": len(items),
        "high_priority_count": high_priority_count,
        "gap_class_counts": gap_class_counts,
        "blocked_actions": [
            "candidate_generation",
            "runtime_apply",
            "profile_write",
            "manifest_write",
            "baseline_write",
            "sample_write",
            "source_write",
            "native_hardening_without_review",
            "pass_promotion",
        ],
    }
    records = [
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_SCAFFOLD_GAP_SUMMARY",
            "info",
            summary,
        )
    ]
    records.extend(
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_SCAFFOLD_GAP_ITEM",
            "warn" if item["priority"] == "high" else "info",
            item,
        )
        for item in items
    )
    return records


def _scaffold_gap_item_details(item: dict[str, str]) -> dict[str, Any]:
    return {
        "gap_id": item["gap_id"],
        "gap_class": item["gap_class"],
        "surface_family": item["surface_family"],
        "priority": item["priority"],
        "current_evidence": item["current_evidence"],
        "next_artifact": item["next_artifact"],
        "review_gate": item["review_gate"],
        "negative_gate": item["negative_gate"],
        "apply_authorized": False,
        "writes": [],
        "review_status": "review_only",
        "evidence_ceiling": "diagnostic_only",
        "blocked_actions": [
            "candidate_generation",
            "runtime_apply",
            "native_hardening_without_review",
            "pass_promotion",
        ],
    }


def _profile_coherence_records(groups: list[ProfileCoherenceGroup]):
    counts = {"consistent": 0, "inconsistent": 0, "unknown": 0}
    for group in groups:
        counts[group.status] += 1
    records = [
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_PROFILE_COHERENCE_SUMMARY",
            "info",
            {
                "enabled": True,
                "groups": len(groups),
                "consistent": counts["consistent"],
                "inconsistent": counts["inconsistent"],
                "unknown": counts["unknown"],
                "review_status": "review_only",
                "evidence_ceiling": "diagnostic_only",
            },
        )
    ]
    records.extend(
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_PROFILE_COHERENCE_GROUP",
            "warn" if group.status == "inconsistent" else "info",
            group.to_details(),
        )
        for group in groups
    )
    return records


def _family_pressure_summary_records(
    pressures: list[FamilyPressure],
):
    category_counts: dict[str, int] = dict.fromkeys(sorted(_ALLOWED_PRESSURE_CATEGORIES), 0)
    family_counts: dict[str, int] = dict.fromkeys(sorted(_ALLOWED_TARGET_FAMILIES), 0)
    for pressure in pressures:
        category_counts[pressure.category] += 1
        family_counts[pressure.target_family] += 1
    records = [
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_FAMILY_PRESSURE_SUMMARY",
            "info",
            {
                "enabled": True,
                "pressures": len(pressures),
                "category_counts": category_counts,
                "family_counts": family_counts,
                "entries": [pressure.to_details() for pressure in pressures],
                "review_status": "review_only",
                "evidence_ceiling": "diagnostic_only",
            },
        )
    ]
    return records


def _pressure_harness_records(pressure_report: Any):
    data = pressure_report.to_dict()
    summary = {
        "schema_version": data["schema_version"],
        "sample_id": data["sample_id"],
        "input_kind": data["input_kind"],
        "execution_mode": data["execution_mode"],
        "status": data["status"],
        "failure_kind": data["failure_kind"],
        "pressure": data["pressure"],
        "promotion": data["promotion"],
        "writes": data["writes"],
    }
    return [
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_PRESSURE_HARNESS_SUMMARY",
            "info",
            {
                "enabled": True,
                "report": summary,
                "review_status": "review_only",
                "evidence_ceiling": "diagnostic_only",
            },
        ),
        *pressure_report.diagnostics,
    ]


def _native_substrate_review_records(
    groups: list[ProfileCoherenceGroup],
    pressures: list[FamilyPressure],
):
    candidate_areas = _native_substrate_candidate_areas(groups, pressures)
    return [
        ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_NATIVE_SUBSTRATE_REVIEW",
            "info" if not candidate_areas else "warn",
            {
                "enabled": True,
                "candidate_areas": candidate_areas,
                "blocked_actions": [
                    "profile_auto_apply",
                    "local_overlay_runtime_apply",
                    "family_pressure_runtime_apply",
                    "unsafe_hook_default_apply",
                    "target_flow_replay",
                    "rust_native_hardening_without_review",
                ],
                "review_status": "requires_review" if candidate_areas else "review_only",
                "evidence_ceiling": "diagnostic_only",
            },
        )
    ]


def _native_substrate_candidate_areas(
    groups: list[ProfileCoherenceGroup],
    pressures: list[FamilyPressure],
) -> list[str]:
    areas: set[str] = set()
    if any(
        pressure.category in {"descriptor_mismatch", "prototype_mismatch"}
        for pressure in pressures
    ):
        areas.add("descriptor_prototype")
    status_by_group = {group.group_id: group.status for group in groups}
    if status_by_group.get("ua_platform") == "inconsistent":
        areas.add("navigator_ua_data")
    if status_by_group.get("network_info") == "inconsistent":
        areas.add("navigator_connection")
    if status_by_group.get("timezone_locale") == "inconsistent":
        areas.add("timezone_intl")
    return sorted(areas)
