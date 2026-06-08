"""Candidate mapping helpers for Environment Toolchain reports."""

from __future__ import annotations

import os
from typing import Any

from iv8_rs.environment_toolchain_asset_loading import _candidate_registry, load_candidate_pack
from iv8_rs.environment_toolchain_asset_models import CandidatePack, ToolchainCandidate
from iv8_rs.environment_toolchain_boundary import validate_bypass_boundary
from iv8_rs.environment_toolchain_models import EnvironmentGap


def map_gaps_to_candidates(
    gaps: list[EnvironmentGap],
    *,
    environment: dict[str, Any] | None = None,
    candidate_pack: str | CandidatePack | dict[str, Any] | os.PathLike[str] | None = (
        "chrome_generic"
    ),
) -> list[ToolchainCandidate]:
    """Map generic gaps to reviewed runtime-safe candidates without applying them."""
    if candidate_pack is None:
        return []
    pack = (
        candidate_pack
        if isinstance(candidate_pack, CandidatePack)
        else load_candidate_pack(candidate_pack)
    )
    explicit_environment = environment or {}
    candidates: list[ToolchainCandidate] = []
    seen_patch_ids: set[str] = set()
    for gap in gaps:
        if gap.target in explicit_environment:
            continue
        for candidate in _candidate_registry(pack).get(gap.target, []):
            gap_classes = set(candidate.validation.get("gap_classes", []))
            if gap_classes and gap.gap_class not in gap_classes:
                continue
            if validate_bypass_boundary(candidate).decision == "blocked":
                continue
            if candidate.patch_id not in seen_patch_ids:
                candidates.append(candidate)
                seen_patch_ids.add(candidate.patch_id)
    return candidates
