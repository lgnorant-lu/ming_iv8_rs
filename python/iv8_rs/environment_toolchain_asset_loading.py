"""Asset loading helpers for Environment Toolchain probe and candidate packs."""

from __future__ import annotations

import json
import os
from importlib import resources
from typing import Any

from iv8_rs.environment_toolchain_asset_models import CandidatePack, ProbePack, ToolchainCandidate
from iv8_rs.environment_toolchain_boundary import validate_bypass_boundary
from iv8_rs.environment_toolchain_models import AssetProvenance
from iv8_rs.environment_toolchain_static import _CANDIDATE_PACK_FILES, _PROBE_PACK_FILES


def available_probe_packs() -> list[str]:
    return sorted(_PROBE_PACK_FILES)


def available_candidate_targets() -> list[str]:
    return sorted(_candidate_registry(load_candidate_pack("chrome_generic")))


def load_probe_pack(probe_pack: str | dict[str, Any] | os.PathLike[str]) -> ProbePack:
    return _resolve_probe_pack(probe_pack)[0]


def load_candidate_pack(candidate_pack: str | dict[str, Any] | os.PathLike[str]) -> CandidatePack:
    return _resolve_candidate_pack(candidate_pack)[0]


def _resolve_probe_pack(
    probe_pack: str | ProbePack | dict[str, Any] | os.PathLike[str],
) -> tuple[ProbePack, AssetProvenance]:
    if isinstance(probe_pack, ProbePack):
        return probe_pack, AssetProvenance(
            asset_type="probe_pack",
            pack_id=probe_pack.probe_pack,
            origin="object",
            version=probe_pack.version,
        )
    if isinstance(probe_pack, dict):
        pack = _load_custom_probe_pack_from_dict(probe_pack)
        return pack, AssetProvenance(
            asset_type="probe_pack",
            pack_id=pack.probe_pack,
            origin="custom_dict",
            version=pack.version,
        )
    if isinstance(probe_pack, os.PathLike):
        pack = _load_custom_probe_pack_from_path(probe_pack)
        return pack, AssetProvenance(
            asset_type="probe_pack",
            pack_id=pack.probe_pack,
            origin="custom_path",
            version=pack.version,
            redacted_ref=os.path.basename(os.fspath(probe_pack)),
        )

    asset_name = _PROBE_PACK_FILES.get(probe_pack)
    if asset_name is None:
        if _looks_like_json_path(probe_pack):
            pack = _load_custom_probe_pack_from_path(probe_pack)
            return pack, AssetProvenance(
                asset_type="probe_pack",
                pack_id=pack.probe_pack,
                origin="custom_path",
                version=pack.version,
                redacted_ref=os.path.basename(probe_pack),
            )
        available = ", ".join(available_probe_packs())
        raise ValueError(f"unknown probe pack: {probe_pack}; available: {available}")
    data = _load_json_asset("probe_packs", asset_name)
    _ensure_boundary_allowed(data)
    pack = ProbePack.from_dict(data)
    return pack, AssetProvenance(
        asset_type="probe_pack",
        pack_id=pack.probe_pack,
        origin="builtin",
        version=pack.version,
    )


def _resolve_candidate_pack(
    candidate_pack: str | CandidatePack | dict[str, Any] | os.PathLike[str] | None,
) -> tuple[CandidatePack | None, AssetProvenance]:
    if candidate_pack is None:
        return None, AssetProvenance(
            asset_type="candidate_pack",
            pack_id="disabled",
            origin="disabled",
        )
    if isinstance(candidate_pack, CandidatePack):
        return candidate_pack, AssetProvenance(
            asset_type="candidate_pack",
            pack_id=candidate_pack.candidate_pack,
            origin="object",
            version=candidate_pack.version,
        )
    if isinstance(candidate_pack, dict):
        pack = _load_custom_candidate_pack_from_dict(candidate_pack)
        return pack, AssetProvenance(
            asset_type="candidate_pack",
            pack_id=pack.candidate_pack,
            origin="custom_dict",
            version=pack.version,
        )
    if isinstance(candidate_pack, os.PathLike):
        pack = _load_custom_candidate_pack_from_path(candidate_pack)
        return pack, AssetProvenance(
            asset_type="candidate_pack",
            pack_id=pack.candidate_pack,
            origin="custom_path",
            version=pack.version,
            redacted_ref=os.path.basename(os.fspath(candidate_pack)),
        )

    asset_name = _CANDIDATE_PACK_FILES.get(candidate_pack)
    if asset_name is None:
        if _looks_like_json_path(candidate_pack):
            pack = _load_custom_candidate_pack_from_path(candidate_pack)
            return pack, AssetProvenance(
                asset_type="candidate_pack",
                pack_id=pack.candidate_pack,
                origin="custom_path",
                version=pack.version,
                redacted_ref=os.path.basename(candidate_pack),
            )
        available = ", ".join(sorted(_CANDIDATE_PACK_FILES))
        raise ValueError(f"unknown candidate pack: {candidate_pack}; available: {available}")
    data = _load_json_asset("candidates", asset_name)
    _ensure_boundary_allowed(data, asset_type="candidate pack")
    pack = CandidatePack.from_dict(data)
    return pack, AssetProvenance(
        asset_type="candidate_pack",
        pack_id=pack.candidate_pack,
        origin="builtin",
        version=pack.version,
    )


def _load_custom_probe_pack_from_dict(data: dict[str, Any]) -> ProbePack:
    _ensure_custom_probe_pack_id(data)
    _ensure_boundary_allowed(data)
    return ProbePack.from_dict(data)


def _load_custom_probe_pack_from_path(path: str | os.PathLike[str]) -> ProbePack:
    try:
        with open(path, encoding="utf-8") as fh:  # noqa: PTH123 - accepts os.PathLike without forcing pathlib.
            data = json.load(fh)
    except json.JSONDecodeError as exc:
        raise ValueError(f"invalid probe pack JSON: {exc}") from exc
    except OSError as exc:
        raise ValueError(f"cannot read probe pack path: {exc}") from exc
    if not isinstance(data, dict):
        raise ValueError("probe pack JSON must contain an object")
    return _load_custom_probe_pack_from_dict(data)


def _load_custom_candidate_pack_from_dict(data: dict[str, Any]) -> CandidatePack:
    _ensure_custom_candidate_pack_id(data)
    _ensure_boundary_allowed(data, asset_type="candidate pack")
    return CandidatePack.from_dict(data)


def _load_custom_candidate_pack_from_path(path: str | os.PathLike[str]) -> CandidatePack:
    try:
        with open(path, encoding="utf-8") as fh:  # noqa: PTH123 - accepts os.PathLike without forcing pathlib.
            data = json.load(fh)
    except json.JSONDecodeError as exc:
        raise ValueError(f"invalid candidate pack JSON: {exc}") from exc
    except OSError as exc:
        raise ValueError(f"cannot read candidate pack path: {exc}") from exc
    if not isinstance(data, dict):
        raise ValueError("candidate pack JSON must contain an object")
    return _load_custom_candidate_pack_from_dict(data)


def _looks_like_json_path(value: str) -> bool:
    return (
        value.endswith(".json")
        or os.path.sep in value
        or bool(os.path.altsep and os.path.altsep in value)
    )


def _ensure_custom_probe_pack_id(data: dict[str, Any]) -> None:
    pack_id = data.get("probe_pack")
    if pack_id in _PROBE_PACK_FILES:
        raise ValueError(f"custom probe pack cannot override built-in pack: {pack_id}")


def _ensure_custom_candidate_pack_id(data: dict[str, Any]) -> None:
    pack_id = data.get("candidate_pack")
    if pack_id in _CANDIDATE_PACK_FILES:
        raise ValueError(f"custom candidate pack cannot override built-in pack: {pack_id}")


def _ensure_boundary_allowed(data: dict[str, Any], *, asset_type: str = "probe pack") -> None:
    decision = validate_bypass_boundary(data)
    if decision.decision == "blocked":
        terms = ", ".join(decision.blocked_terms)
        raise ValueError(f"{asset_type} failed boundary validation: {terms}")


def _candidate_registry(pack: CandidatePack) -> dict[str, list[ToolchainCandidate]]:
    registry: dict[str, list[ToolchainCandidate]] = {}
    for candidate in pack.candidates:
        registry.setdefault(candidate.target, []).append(candidate)
    return registry


def _load_json_asset(asset_group: str, asset_name: str) -> dict[str, Any]:
    package = f"iv8_rs.environment_toolchain_assets.{asset_group}"
    text = resources.files(package).joinpath(asset_name).read_text(encoding="utf-8")
    return json.loads(text)


def probe_pack_from_dict(data: dict[str, Any]) -> ProbePack:
    return ProbePack.from_dict(data)


def probe_pack_to_dict(probe_pack: ProbePack) -> dict[str, Any]:
    return probe_pack.to_dict()
