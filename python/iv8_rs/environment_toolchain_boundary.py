"""Boundary validation helpers for Environment Toolchain payloads."""

from __future__ import annotations

from typing import Any

from iv8_rs.environment_toolchain_models import BoundaryDecision
from iv8_rs.environment_toolchain_static import (
    _BLOCKED_BOUNDARY_TERMS,
    _GENERIC_TARGET_PREFIXES,
    _ORDERED_RECIPE_RE,
    _RAW_LOCAL_PATH_RE,
)


def validate_bypass_boundary(payload: Any) -> BoundaryDecision:
    """Block target-specific bypass vocabulary in candidates or suggestion payloads."""
    data = payload.to_dict() if hasattr(payload, "to_dict") else dict(payload)
    blocked: list[str] = []
    for path, value in _walk_payload(data):
        if not isinstance(value, str):
            continue
        if path.endswith(".target") or path == "target" or path == "patch_id":
            if _is_generic_target(value):
                continue
        lowered = value.lower()
        blocked.extend(term for term in _BLOCKED_BOUNDARY_TERMS if term in lowered)
        if _RAW_LOCAL_PATH_RE.search(value):
            blocked.append("raw_path")
        if _ORDERED_RECIPE_RE.search(value):
            blocked.append("ordered_recipe")

    blocked = sorted(set(blocked))
    if blocked:
        return BoundaryDecision(
            decision="blocked",
            reason="payload contains target-specific bypass vocabulary",
            blocked_terms=blocked,
        )
    return BoundaryDecision(decision="allowed", reason="generic environment payload")


def _walk_payload(value: Any, prefix: str = "") -> list[tuple[str, Any]]:
    if isinstance(value, dict):
        items: list[tuple[str, Any]] = []
        for key, child in value.items():
            child_prefix = f"{prefix}.{key}" if prefix else str(key)
            items.extend(_walk_payload(child, child_prefix))
        return items
    if isinstance(value, list):
        items = []
        for idx, child in enumerate(value):
            child_prefix = f"{prefix}[{idx}]"
            items.extend(_walk_payload(child, child_prefix))
        return items
    return [(prefix, value)]


def _is_generic_target(value: str) -> bool:
    return value.startswith(_GENERIC_TARGET_PREFIXES) or value in {"Date", "Math", "crypto"}
