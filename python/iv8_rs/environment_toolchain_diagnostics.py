"""Diagnostic builders for Environment Toolchain reports.

This module contains behavior-preserving diagnostic record builders extracted
from `environment_toolchain_runtime.py`. It must not import the runtime module.
"""

from __future__ import annotations

from typing import Any

from iv8_rs.environment_toolchain_models import FamilyPressure, ProfileCoherenceGroup
from iv8_rs.environment_toolchain_static import (
    _ALLOWED_PRESSURE_CATEGORIES,
    _ALLOWED_TARGET_FAMILIES,
    _SCAFFOLD_GAP_ITEMS,
    _SUBSTRATE_COVERAGE_ITEMS,
)
from iv8_rs.experimental_report import ExperimentalDiagnosticRecord


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
