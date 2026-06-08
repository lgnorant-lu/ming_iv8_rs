"""Runtime helpers for Environment Toolchain assets.

This module hosts the bounded Environment Toolchain runtime: probe packs,
candidate packs, validation, provenance, report-only execution, and explicit
runtime-safe reruns. It never writes profiles, manifests, baselines, or corpus
state.
"""

from __future__ import annotations

import json
import os
import re
from dataclasses import asdict, dataclass, field
from importlib import resources
from typing import Any

__all__ = [
    "BoundaryDecision",
    "EnvironmentGap",
    "ProbeDefinition",
    "ProbeObservation",
    "ProbePack",
    "ProbeRun",
    "CandidatePack",
    "ToolchainCandidate",
    "available_candidate_targets",
    "available_probe_packs",
    "load_candidate_pack",
    "load_probe_pack",
    "map_gaps_to_candidates",
    "probe_pack_from_dict",
    "probe_pack_to_dict",
    "run_probe_pack",
    "run_environment_toolchain",
    "validate_bypass_boundary",
]


_ALLOWED_EVIDENCE_CEILINGS = {"diagnostic_only", "weak"}
_ALLOWED_PROBE_CATEGORIES = {"presence", "descriptor", "behavior", "value"}
_GENERIC_TARGET_PREFIXES = (
    "navigator.",
    "screen.",
    "document.",
    "window.",
    "location.",
    "performance.",
    "Math.",
    "Date.",
    "crypto.",
)
_BLOCKED_BOUNDARY_TERMS = (
    "domain",
    "endpoint",
    "cookie",
    "token",
    "signature",
    "nonce",
    "request_body",
    "request body",
    "authorization",
    "secret",
)
_ALLOWED_PRESSURE_CATEGORIES = frozenset({
    "descriptor_mismatch",
    "prototype_mismatch",
    "value_mismatch",
    "missing_api",
    "behavior_mismatch",
})
_ALLOWED_TARGET_FAMILIES = frozenset({
    "navigator",
    "screen",
    "window",
    "document",
    "timing",
    "network_info",
})
_GAP_CLASS_TO_PRESSURE_CATEGORY = {
    "missing_api": "missing_api",
    "value_mismatch": "value_mismatch",
    "descriptor_mismatch": "descriptor_mismatch",
    "behavior_mismatch": "behavior_mismatch",
    "prototype_chain_mismatch": "prototype_mismatch",
}
_ORDERED_RECIPE_RE = re.compile(r"apply\s+.+request\s+.+(?:copy|rerun)", re.IGNORECASE)
_PROBE_PACK_FILES = {
    "descriptor.m1": "descriptor.m1.json",
    "fingerprint.m1": "fingerprint.m1.json",
}
_CANDIDATE_PACK_FILES = {"chrome_generic": "chrome_generic.json"}
_ADAPTATION_STOP_REASONS = {
    "disabled",
    "completed",
    "budget_exhausted",
    "no_candidate",
    "policy_blocked",
    "boundary_blocked",
    "regression_detected",
    "no_progress",
    "entry_failure",
    "asset_error",
}


@dataclass(frozen=True, slots=True)
class BoundaryDecision:
    decision: str
    reason: str
    redactions: list[str] = field(default_factory=list)
    blocked_terms: list[str] = field(default_factory=list)

    def __post_init__(self) -> None:
        if self.decision not in {"allowed", "blocked"}:
            raise ValueError(f"invalid boundary decision: {self.decision}")

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(frozen=True, slots=True)
class EnvironmentGap:
    probe_id: str
    target: str
    gap_class: str
    category: str
    expected: Any
    actual: Any
    error: str | None = None

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(frozen=True, slots=True)
class ProbeObservation:
    probe_id: str
    target: str
    category: str
    expected: Any
    actual: Any
    passed: bool
    gap_class: str
    evidence_ceiling: str = "diagnostic_only"
    error: str | None = None

    @classmethod
    def from_probe(
        cls,
        probe: ProbeDefinition,
        *,
        actual: Any,
        passed: bool,
        error: str | None = None,
    ) -> ProbeObservation:
        return cls(
            probe_id=probe.probe_id,
            target=probe.target,
            category=probe.category,
            expected=probe.expected,
            actual=actual,
            passed=passed,
            gap_class=probe.gap_class,
            evidence_ceiling=probe.evidence_ceiling,
            error=error,
        )

    def to_gap(self) -> EnvironmentGap | None:
        if self.passed:
            return None
        return EnvironmentGap(
            probe_id=self.probe_id,
            target=self.target,
            gap_class=self.gap_class,
            category=self.category,
            expected=self.expected,
            actual=self.actual,
            error=self.error,
        )

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(frozen=True, slots=True)
class ProbeRun:
    probe_pack: str
    observations: list[ProbeObservation]
    gaps: list[EnvironmentGap]
    coverage: dict[str, int]
    diagnostics: list[dict[str, Any]] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        return {
            "probe_pack": self.probe_pack,
            "observations": [observation.to_dict() for observation in self.observations],
            "gaps": [gap.to_dict() for gap in self.gaps],
            "coverage": dict(self.coverage),
            "diagnostics": [dict(diagnostic) for diagnostic in self.diagnostics],
        }


@dataclass(frozen=True, slots=True)
class AdaptationIteration:
    index: int
    before: dict[str, int]
    after: dict[str, int]
    delta: dict[str, int]
    matched_patch_ids: list[str]
    applied_patch_ids: list[str]
    rejected: list[dict[str, Any]] = field(default_factory=list)
    stop_reason: str | None = None

    def to_details(self) -> dict[str, Any]:
        details: dict[str, Any] = {
            "index": self.index,
            "before": dict(self.before),
            "after": dict(self.after),
            "delta": dict(self.delta),
            "matched_patch_ids": list(self.matched_patch_ids),
            "applied_patch_ids": list(self.applied_patch_ids),
            "rejected": [dict(item) for item in self.rejected],
        }
        if self.stop_reason is not None:
            details["stop_reason"] = self.stop_reason
        return details


@dataclass(frozen=True, slots=True)
class ProfileCoherenceGroup:
    group_id: str
    status: str
    fields: dict[str, Any]
    sources: dict[str, str]
    reason: str
    review_status: str = "review_only"
    evidence_ceiling: str = "diagnostic_only"

    def __post_init__(self) -> None:
        if self.status not in {"consistent", "inconsistent", "unknown"}:
            raise ValueError(f"invalid profile coherence status: {self.status}")

    def to_details(self) -> dict[str, Any]:
        return {
            "group_id": self.group_id,
            "status": self.status,
            "fields": dict(self.fields),
            "sources": dict(self.sources),
            "reason": self.reason,
            "review_status": self.review_status,
            "evidence_ceiling": self.evidence_ceiling,
        }


@dataclass(frozen=True, slots=True)
class FamilyPressure:
    pressure_id: str
    category: str
    target_family: str
    gap_classes: list[str]
    review_status: str = "review_only"
    evidence_ceiling: str = "diagnostic_only"

    def __post_init__(self) -> None:
        if self.category not in _ALLOWED_PRESSURE_CATEGORIES:
            raise ValueError(f"invalid pressure category: {self.category}")
        if self.target_family not in _ALLOWED_TARGET_FAMILIES:
            raise ValueError(f"invalid target family: {self.target_family}")

    def to_details(self) -> dict[str, Any]:
        return {
            "pressure_id": self.pressure_id,
            "category": self.category,
            "target_family": self.target_family,
            "gap_classes": list(self.gap_classes),
            "review_status": self.review_status,
            "evidence_ceiling": self.evidence_ceiling,
        }


@dataclass(frozen=True, slots=True)
class AssetProvenance:
    asset_type: str
    pack_id: str
    origin: str
    version: int | None = None
    redacted_ref: str | None = None

    @property
    def diagnostic_code(self) -> str:
        asset = self.asset_type.upper().replace(" ", "_")
        origin = self.origin.upper()
        return f"ENV_TOOLCHAIN_{asset}_{origin}"


@dataclass(frozen=True, slots=True)
class ToolchainCandidate:
    patch_id: str
    target: str
    target_family: str
    kind: str
    policy: str
    source: str
    value_preview: Any
    requires: list[str] = field(default_factory=list)
    risk_reasons: list[str] = field(default_factory=list)
    reversible: bool = True
    validation: dict[str, Any] = field(default_factory=dict)

    def __post_init__(self) -> None:
        if self.policy != "runtime_safe":
            raise ValueError("slice 3 registry only exposes runtime_safe candidates")
        if not self.reversible:
            raise ValueError("runtime_safe candidates must be reversible")

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ToolchainCandidate:
        return cls(
            patch_id=data["patch_id"],
            target=data["target"],
            target_family=data["target_family"],
            kind=data["kind"],
            policy=data["policy"],
            source=data.get("source", "builtin_registry"),
            value_preview=data.get("value_preview"),
            requires=list(data.get("requires", [])),
            risk_reasons=list(data.get("risk_reasons", [])),
            reversible=bool(data.get("reversible", True)),
            validation=dict(data.get("validation", {})),
        )

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(frozen=True, slots=True)
class CandidatePack:
    candidate_pack: str
    version: int
    description: str
    candidates: list[ToolchainCandidate]

    def __post_init__(self) -> None:
        if not self.candidate_pack:
            raise ValueError("candidate_pack must not be empty")
        if self.version < 1:
            raise ValueError("candidate pack version must be positive")
        patch_ids = [candidate.patch_id for candidate in self.candidates]
        duplicates = sorted({patch_id for patch_id in patch_ids if patch_ids.count(patch_id) > 1})
        if duplicates:
            raise ValueError(f"duplicate candidate patch ids: {duplicates}")

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> CandidatePack:
        return cls(
            candidate_pack=data["candidate_pack"],
            version=int(data["version"]),
            description=data["description"],
            candidates=[
                ToolchainCandidate.from_dict(candidate)
                for candidate in data.get("candidates", [])
            ],
        )

    def to_dict(self) -> dict[str, Any]:
        data = asdict(self)
        data["candidates"] = [candidate.to_dict() for candidate in self.candidates]
        return data


@dataclass(frozen=True, slots=True)
class ProbeDefinition:
    probe_id: str
    target: str
    category: str
    js: str
    expected: Any
    gap_class: str
    side_effects: list[str] = field(default_factory=list)
    cleanup: str = "none"
    evidence_ceiling: str = "diagnostic_only"

    def __post_init__(self) -> None:
        if not self.probe_id:
            raise ValueError("probe_id must not be empty")
        if not self.target:
            raise ValueError("target must not be empty")
        if self.category not in _ALLOWED_PROBE_CATEGORIES:
            raise ValueError(f"invalid probe category: {self.category}")
        if self.evidence_ceiling not in _ALLOWED_EVIDENCE_CEILINGS:
            raise ValueError(f"invalid evidence ceiling: {self.evidence_ceiling}")
        if self.evidence_ceiling == "weak":
            raise ValueError("probe definitions cannot claim weak evidence before runner review")
        if self.side_effects:
            raise ValueError("probe side effects are not supported before runner review")
        if self.cleanup != "none":
            raise ValueError("probe cleanup must be none before runner review")

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ProbeDefinition:
        return cls(
            probe_id=data["probe_id"],
            target=data["target"],
            category=data["category"],
            js=data["js"],
            expected=data["expected"],
            gap_class=data["gap_class"],
            side_effects=list(data.get("side_effects", [])),
            cleanup=data.get("cleanup", "none"),
            evidence_ceiling=data.get("evidence_ceiling", "diagnostic_only"),
        )

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(frozen=True, slots=True)
class ProbePack:
    probe_pack: str
    version: int
    description: str
    evidence_ceiling: str
    probes: list[ProbeDefinition]

    def __post_init__(self) -> None:
        if not self.probe_pack:
            raise ValueError("probe_pack must not be empty")
        if self.version < 1:
            raise ValueError("probe pack version must be positive")
        if self.evidence_ceiling != "diagnostic_only":
            raise ValueError("probe packs must be diagnostic_only before runner review")
        if not self.probes:
            raise ValueError("probe pack must contain at least one probe")
        probe_ids = [probe.probe_id for probe in self.probes]
        duplicates = sorted({probe_id for probe_id in probe_ids if probe_ids.count(probe_id) > 1})
        if duplicates:
            raise ValueError(f"duplicate probe ids: {duplicates}")

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ProbePack:
        return cls(
            probe_pack=data["probe_pack"],
            version=int(data["version"]),
            description=data["description"],
            evidence_ceiling=data["evidence_ceiling"],
            probes=[ProbeDefinition.from_dict(probe) for probe in data.get("probes", [])],
        )

    def to_dict(self) -> dict[str, Any]:
        data = asdict(self)
        data["probes"] = [probe.to_dict() for probe in self.probes]
        return data


def available_probe_packs() -> list[str]:
    return sorted(_PROBE_PACK_FILES)


def available_candidate_targets() -> list[str]:
    return sorted(_candidate_registry(load_candidate_pack("chrome_generic")))


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


def validate_bypass_boundary(payload: ToolchainCandidate | dict[str, Any]) -> BoundaryDecision:
    """Block target-specific bypass vocabulary in candidates or suggestion payloads."""
    data = payload.to_dict() if isinstance(payload, ToolchainCandidate) else dict(payload)
    blocked: list[str] = []
    for path, value in _walk_payload(data):
        if not isinstance(value, str):
            continue
        if path.endswith(".target") or path == "target" or path == "patch_id":
            if _is_generic_target(value):
                continue
        lowered = value.lower()
        blocked.extend(term for term in _BLOCKED_BOUNDARY_TERMS if term in lowered)
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


def run_probe_pack(
    js_source: str,
    probe_pack: str | ProbePack | dict[str, Any] | os.PathLike[str] = "fingerprint.m1",
    *,
    profile: str | None = "default",
    environment: dict[str, Any] | None = None,
    random_seed: int | None = 42,
    time_freeze: float | None = None,
    time_mode: str = "logical",
    entry_expr: str | None = None,
) -> ProbeRun:
    """Run a bounded probe pack in a fresh JSContext and classify generic gaps."""
    from iv8_rs import JSContext

    pack, provenance = _resolve_probe_pack(probe_pack)
    observations: list[ProbeObservation] = []
    diagnostics: list[dict[str, Any]] = [
        _provenance_diagnostic(provenance),
        {
            "code": "ENV_TOOLCHAIN_PROBE_PACK_RUN",
            "severity": "info",
            "stage": "environment.probe",
            "message": f"probe pack executed: {pack.probe_pack}",
        }
    ]

    ctx = JSContext(
        profile=profile,
        environment=environment,
        random_seed=random_seed,
        time_freeze=time_freeze,
        time_mode=time_mode,
    )
    try:
        if js_source:
            ctx.eval(js_source)
        if entry_expr:
            try:
                ctx.eval(entry_expr)
            except Exception as exc:  # noqa: BLE001 - diagnostics must preserve probe continuity.
                diagnostics.append(_diagnostic(
                    "ENV_TOOLCHAIN_ENTRY_EXPR_FAILED",
                    "warn",
                    "environment.probe",
                    f"entry_expr failed: {exc}",
                ))

        for probe in pack.probes:
            observations.append(_run_single_probe(ctx, probe))
    finally:
        ctx.close()

    gaps = [gap for observation in observations if (gap := observation.to_gap()) is not None]
    diagnostics.extend(_gap_diagnostics(gaps))
    return ProbeRun(
        probe_pack=pack.probe_pack,
        observations=observations,
        gaps=gaps,
        coverage=_coverage_from_observations(observations),
        diagnostics=diagnostics,
    )


def run_environment_toolchain(
    js_source: str,
    *,
    probe_pack: str | ProbePack | dict[str, Any] | os.PathLike[str] = "fingerprint.m1",
    profile: str | None = "default",
    environment: dict[str, Any] | None = None,
    candidate_pack: str | CandidatePack | dict[str, Any] | os.PathLike[str] | None = (
        "chrome_generic"
    ),
    apply_runtime_safe: bool = False,
    adapt_runtime_safe: bool = False,
    local_overlay: dict[str, Any] | os.PathLike[str] | None = None,
    max_iterations: int = 1,
    stop_on_regression: bool = True,
    random_seed: int | None = 42,
    time_freeze: float | None = None,
    time_mode: str = "logical",
    entry_expr: str | None = None,
):
    """Run the Environment Toolchain flow with optional runtime-safe rerun."""
    if max_iterations < 0:
        raise ValueError("max_iterations must be non-negative")

    from iv8_rs.environment_toolchain import (
        CoverageDelta,
        EnvironmentToolchainReport,
        ToolchainPatchEntry,
    )
    from iv8_rs.experimental_report import ExperimentalDiagnosticRecord, ExperimentalEvidenceRecord

    candidate_pack_object, candidate_provenance = _resolve_candidate_pack(candidate_pack)
    if adapt_runtime_safe:
        return _run_iterative_environment_toolchain(
            js_source,
            probe_pack=probe_pack,
            profile=profile,
            environment=environment,
            candidate_pack_object=candidate_pack_object,
            candidate_provenance=candidate_provenance,
            max_iterations=max_iterations,
            stop_on_regression=stop_on_regression,
            random_seed=random_seed,
            time_freeze=time_freeze,
            time_mode=time_mode,
            entry_expr=entry_expr,
            local_overlay=local_overlay,
        )

    before_run = run_probe_pack(
        js_source,
        probe_pack=probe_pack,
        profile=profile,
        environment=environment,
        random_seed=random_seed,
        time_freeze=time_freeze,
        time_mode=time_mode,
        entry_expr=entry_expr,
    )
    candidates = map_gaps_to_candidates(
        before_run.gaps,
        environment=environment,
        candidate_pack=candidate_pack_object,
    )
    applied_candidates = candidates if apply_runtime_safe else []
    rejected_candidates = [] if apply_runtime_safe else candidates
    applied = [
        ToolchainPatchEntry(
            patch_id=candidate.patch_id,
            target=candidate.target,
            kind=candidate.kind,
            policy=candidate.policy,
            reason="explicit runtime_safe apply",
        )
        for candidate in applied_candidates
    ]
    rejected = [
        ToolchainPatchEntry(
            patch_id=candidate.patch_id,
            target=candidate.target,
            kind=candidate.kind,
            policy=candidate.policy,
            reason="report-only default; explicit apply_runtime_safe required",
        )
        for candidate in rejected_candidates
    ]

    if apply_runtime_safe and applied_candidates:
        after_environment = dict(environment or {})
        for candidate in applied_candidates:
            after_environment[candidate.target] = candidate.value_preview
        after_run = run_probe_pack(
            js_source,
            probe_pack=probe_pack,
            profile=profile,
            environment=after_environment,
            random_seed=random_seed,
            time_freeze=time_freeze,
            time_mode=time_mode,
            entry_expr=entry_expr,
        )
    else:
        after_environment = environment
        after_run = before_run

    delta = _coverage_delta(before_run, after_run)

    evidence = [
        ExperimentalEvidenceRecord("environment_probe_pack_run", "diagnostic_only"),
        *[
            ExperimentalEvidenceRecord("environment_gap_observed", "diagnostic_only")
            for _gap in before_run.gaps
        ],
        *[
            ExperimentalEvidenceRecord("environment_patch_registry_candidate", "diagnostic_only")
            for _candidate in candidates
        ],
        *[
            ExperimentalEvidenceRecord("environment_patch_applied", "weak")
            for _candidate in applied_candidates
        ],
    ]
    if delta["improved"]:
        evidence.append(ExperimentalEvidenceRecord("environment_coverage_improved", "weak"))
    evidence.append(ExperimentalEvidenceRecord(
        "environment_profile_coherence_analyzed",
        "diagnostic_only",
    ))
    evidence.append(ExperimentalEvidenceRecord(
        "environment_family_pressure_analyzed",
        "diagnostic_only",
    ))
    evidence.append(ExperimentalEvidenceRecord(
        "environment_family_pressure_analyzed",
        "diagnostic_only",
    ))
    overlay_values, overlay_prov, overlay_rej = _resolve_local_overlay(local_overlay)
    coherence_env = dict(after_environment or {})
    if overlay_values:
        coherence_env.update(overlay_values)
    profile_suggestions = _profile_suggestions_from_candidates(candidates)
    coherence_groups = _profile_coherence_groups(coherence_env)
    family_pressures = _map_gaps_to_family_pressures(before_run.gaps)
    diagnostics = [
        ExperimentalDiagnosticRecord(item["code"], item["severity"], item.get("details"))
        for item in before_run.diagnostics
    ]
    diagnostics.append(_provenance_record(candidate_provenance))
    if candidates:
        diagnostics.append(ExperimentalDiagnosticRecord("ENV_TOOLCHAIN_PATCH_CANDIDATE", "info"))
        if apply_runtime_safe:
            diagnostics.append(ExperimentalDiagnosticRecord("ENV_TOOLCHAIN_PATCH_APPLIED", "info"))
        else:
            diagnostics.append(ExperimentalDiagnosticRecord("ENV_TOOLCHAIN_PATCH_REJECTED", "warn"))
    if delta["improved"]:
        diagnostics.append(ExperimentalDiagnosticRecord("ENV_TOOLCHAIN_COVERAGE_IMPROVED", "info"))
    if delta["regressed"]:
        diagnostics.append(ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_COVERAGE_REGRESSED",
            "error",
        ))
    if profile_suggestions:
        diagnostics.append(ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_PROFILE_SUGGESTION_REVIEW",
            "info",
        ))
    diagnostics.extend(_profile_coherence_records(coherence_groups))
    diagnostics.extend(_family_pressure_summary_records(family_pressures))
    diagnostics.extend(_native_substrate_review_records(coherence_groups, family_pressures))
    if overlay_prov:
        diagnostics.append(overlay_prov)
    if overlay_rej:
        diagnostics.append(overlay_rej)
    diagnostics.append(ExperimentalDiagnosticRecord("ENV_TOOLCHAIN_NO_WRITES", "info"))

    before_snapshot = _coverage_snapshot(before_run.coverage)
    after_snapshot = _coverage_snapshot(after_run.coverage)
    return EnvironmentToolchainReport(
        schema_version="environment-toolchain.v0.1",
        probe_pack=before_run.probe_pack,
        before=before_snapshot,
        after=after_snapshot,
        coverage_delta=CoverageDelta(
            improved=delta["improved"],
            regressed=delta["regressed"],
            unresolved=delta["unresolved"],
        ),
        applied_patches=applied,
        rejected_patches=rejected,
        profile_suggestions=profile_suggestions,
        evidence=evidence,
        diagnostics=diagnostics,
        writes=[],
    )


def _run_iterative_environment_toolchain(
    js_source: str,
    *,
    probe_pack: str | ProbePack | dict[str, Any] | os.PathLike[str],
    profile: str | None,
    environment: dict[str, Any] | None,
    candidate_pack_object: CandidatePack | None,
    candidate_provenance: AssetProvenance,
    max_iterations: int,
    stop_on_regression: bool,
    random_seed: int | None,
    time_freeze: float | None,
    time_mode: str,
    entry_expr: str | None,
    local_overlay: dict[str, Any] | os.PathLike[str] | None = None,
):
    from iv8_rs.environment_toolchain import (
        CoverageDelta,
        EnvironmentToolchainReport,
        ToolchainPatchEntry,
    )
    from iv8_rs.experimental_report import ExperimentalDiagnosticRecord, ExperimentalEvidenceRecord

    before_run = run_probe_pack(
        js_source,
        probe_pack=probe_pack,
        profile=profile,
        environment=environment,
        random_seed=random_seed,
        time_freeze=time_freeze,
        time_mode=time_mode,
        entry_expr=entry_expr,
    )
    first_run = before_run
    current_run = before_run
    accumulated_environment = dict(environment or {})
    applied_candidates: list[ToolchainCandidate] = []
    rejected_entries: list[dict[str, Any]] = []
    iterations: list[AdaptationIteration] = []
    stop_reason = "disabled" if max_iterations == 0 else "completed"

    for index in range(max_iterations):
        if not current_run.gaps:
            stop_reason = "completed"
            break

        candidates = map_gaps_to_candidates(
            current_run.gaps,
            environment=accumulated_environment,
            candidate_pack=candidate_pack_object,
        )
        if not candidates:
            stop_reason = "no_candidate"
            break

        candidate = candidates[0]
        next_environment = dict(accumulated_environment)
        next_environment[candidate.target] = candidate.value_preview
        after_run = run_probe_pack(
            js_source,
            probe_pack=probe_pack,
            profile=profile,
            environment=next_environment,
            random_seed=random_seed,
            time_freeze=time_freeze,
            time_mode=time_mode,
            entry_expr=entry_expr,
        )
        delta = _coverage_delta(current_run, after_run)
        iteration_stop_reason = _iteration_stop_reason(
            delta,
            before_run=current_run,
            after_run=after_run,
            stop_on_regression=stop_on_regression,
        )
        iterations.append(AdaptationIteration(
            index=index,
            before=_coverage_snapshot_dict(current_run.coverage),
            after=_coverage_snapshot_dict(after_run.coverage),
            delta=delta,
            matched_patch_ids=[candidate.patch_id for candidate in candidates],
            applied_patch_ids=[candidate.patch_id],
            stop_reason=iteration_stop_reason,
        ))
        applied_candidates.append(candidate)
        accumulated_environment = next_environment
        current_run = after_run

        if iteration_stop_reason == "completed":
            stop_reason = "completed"
            break
        if iteration_stop_reason in {"regression_detected", "no_progress"}:
            stop_reason = iteration_stop_reason
            break
    else:
        stop_reason = "completed" if not current_run.gaps else "budget_exhausted"

    final_delta = _coverage_delta(first_run, current_run)
    applied = [
        ToolchainPatchEntry(
            patch_id=candidate.patch_id,
            target=candidate.target,
            kind=candidate.kind,
            policy=candidate.policy,
            reason="explicit iterative runtime_safe apply",
        )
        for candidate in applied_candidates
    ]
    rejected = [
        ToolchainPatchEntry(
            patch_id=entry["patch_id"],
            target=entry["target"],
            kind=entry["kind"],
            policy=entry["policy"],
            reason=entry["reason"],
        )
        for entry in rejected_entries
    ]

    evidence = [
        ExperimentalEvidenceRecord("environment_probe_pack_run", "diagnostic_only"),
        *[
            ExperimentalEvidenceRecord("environment_gap_observed", "diagnostic_only")
            for _gap in first_run.gaps
        ],
        *[
            ExperimentalEvidenceRecord("environment_patch_applied", "weak")
            for _candidate in applied_candidates
        ],
    ]
    if final_delta["improved"]:
        evidence.append(ExperimentalEvidenceRecord("environment_coverage_improved", "weak"))
    evidence.append(ExperimentalEvidenceRecord(
        "environment_profile_coherence_analyzed",
        "diagnostic_only",
    ))

    diagnostics = [
        ExperimentalDiagnosticRecord(item["code"], item["severity"], item.get("details"))
        for item in first_run.diagnostics
    ]
    diagnostics.append(_provenance_record(candidate_provenance))
    diagnostics.extend(_adaptation_records(
        enabled=True,
        max_iterations=max_iterations,
        iterations=iterations,
        stop_reason=stop_reason,
        applied_candidates=applied_candidates,
    ))
    if applied_candidates:
        diagnostics.append(ExperimentalDiagnosticRecord("ENV_TOOLCHAIN_PATCH_APPLIED", "info"))
    if final_delta["improved"]:
        diagnostics.append(ExperimentalDiagnosticRecord("ENV_TOOLCHAIN_COVERAGE_IMPROVED", "info"))
    if final_delta["regressed"]:
        diagnostics.append(ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_COVERAGE_REGRESSED",
            "error",
        ))
    overlay_values, overlay_prov, overlay_rej = _resolve_local_overlay(local_overlay)
    coherence_env = dict(accumulated_environment)
    if overlay_values:
        coherence_env.update(overlay_values)
    coherence_groups = _profile_coherence_groups(coherence_env)
    family_pressures = _map_gaps_to_family_pressures(first_run.gaps)
    diagnostics.extend(_profile_coherence_records(coherence_groups))
    diagnostics.extend(_family_pressure_summary_records(family_pressures))
    diagnostics.extend(_native_substrate_review_records(coherence_groups, family_pressures))
    if overlay_prov:
        diagnostics.append(overlay_prov)
    if overlay_rej:
        diagnostics.append(overlay_rej)
    diagnostics.append(ExperimentalDiagnosticRecord("ENV_TOOLCHAIN_NO_WRITES", "info"))

    before_snapshot = _coverage_snapshot(first_run.coverage)
    after_snapshot = _coverage_snapshot(current_run.coverage)
    return EnvironmentToolchainReport(
        schema_version="environment-toolchain.v0.1",
        probe_pack=first_run.probe_pack,
        before=before_snapshot,
        after=after_snapshot,
        coverage_delta=CoverageDelta(
            improved=final_delta["improved"],
            regressed=final_delta["regressed"],
            unresolved=final_delta["unresolved"],
        ),
        applied_patches=applied,
        rejected_patches=rejected,
        profile_suggestions=_profile_suggestions_from_candidates(applied_candidates),
        evidence=evidence,
        diagnostics=diagnostics,
        writes=[],
    )


def _profile_suggestions_from_candidates(candidates: list[ToolchainCandidate]):
    from iv8_rs.environment_toolchain import ProfileSuggestion

    suggestions: list[ProfileSuggestion] = []
    seen_targets: set[str] = set()
    for candidate in candidates:
        if candidate.target in seen_targets:
            continue
        payload = {
            "target": candidate.target,
            "value_preview": candidate.value_preview,
            "policy": candidate.policy,
        }
        if validate_bypass_boundary(payload).decision == "blocked":
            continue
        suggestions.append(ProfileSuggestion(
            target=candidate.target,
            value_preview=_string_list_preview(candidate.value_preview),
        ))
        seen_targets.add(candidate.target)
    return suggestions


def _string_list_preview(value: Any) -> list[str]:
    if isinstance(value, list):
        return [str(item) for item in value]
    return [str(value)]


def _coverage_snapshot(coverage: dict[str, int]):
    from iv8_rs.environment_toolchain import CoverageSnapshot

    return CoverageSnapshot(
        present=coverage["present"],
        missing=coverage["missing"],
        mismatch=coverage["mismatch"],
    )


def _coverage_snapshot_dict(coverage: dict[str, int]) -> dict[str, int]:
    return {
        "present": coverage["present"],
        "missing": coverage["missing"],
        "mismatch": coverage["mismatch"],
    }


def _iteration_stop_reason(
    delta: dict[str, int],
    *,
    before_run: ProbeRun,
    after_run: ProbeRun,
    stop_on_regression: bool,
) -> str:
    if delta["regressed"] and stop_on_regression:
        return "regression_detected"
    if not after_run.gaps:
        return "completed"
    if delta["improved"] == 0 and len(after_run.gaps) >= len(before_run.gaps):
        return "no_progress"
    return "budget_exhausted"


def _adaptation_records(
    *,
    enabled: bool,
    max_iterations: int,
    iterations: list[AdaptationIteration],
    stop_reason: str,
    applied_candidates: list[ToolchainCandidate],
):
    from iv8_rs.experimental_report import ExperimentalDiagnosticRecord

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


def _resolve_local_overlay(
    overlay: dict[str, Any] | os.PathLike[str] | None,
) -> tuple[dict[str, Any] | None, Any | None, Any | None]:
    """Resolve local overlay input for diagnostic coherence analysis.

    Returns (values_dict_or_None, provenance_record_or_None, rejected_record_or_None).
    The overlay is used only for coherence diagnostics; it never applies runtime
    values, creates patches, or writes persistent state.
    """
    if overlay is None:
        return None, None, None

    from iv8_rs.experimental_report import ExperimentalDiagnosticRecord

    if isinstance(overlay, dict):
        if not _is_all_overlay_key_generic(overlay):
            return None, None, ExperimentalDiagnosticRecord(
                "ENV_TOOLCHAIN_LOCAL_OVERLAY_REJECTED",
                "warn",
                {
                    "reason": "local overlay contains non-generic keys",
                    "non_generic_keys": [
                        key for key in overlay if not _is_generic_target(key)
                    ],
                },
            )
        decision = validate_bypass_boundary(overlay)
        if decision.decision == "blocked":
            return None, None, ExperimentalDiagnosticRecord(
                "ENV_TOOLCHAIN_LOCAL_OVERLAY_REJECTED",
                "warn",
                {
                    "reason": "local overlay blocked by boundary validation",
                    "blocked_terms": list(decision.blocked_terms),
                },
            )
        return overlay, ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_LOCAL_OVERLAY_PROVENANCE",
            "info",
            {
                "asset_type": "local_overlay",
                "origin": "custom_dict",
                "key_count": len(overlay),
            },
        ), None

    if isinstance(overlay, os.PathLike):
        try:
            with open(overlay, encoding="utf-8") as fh:
                data = json.load(fh)
        except (json.JSONDecodeError, OSError) as exc:
            return None, None, ExperimentalDiagnosticRecord(
                "ENV_TOOLCHAIN_LOCAL_OVERLAY_REJECTED",
                "warn",
                {"reason": f"cannot load local overlay path: {exc}"},
            )
        if not isinstance(data, dict):
            return None, None, ExperimentalDiagnosticRecord(
                "ENV_TOOLCHAIN_LOCAL_OVERLAY_REJECTED",
                "warn",
                {"reason": "local overlay JSON must contain an object"},
            )
        if not _is_all_overlay_key_generic(data):
            return None, None, ExperimentalDiagnosticRecord(
                "ENV_TOOLCHAIN_LOCAL_OVERLAY_REJECTED",
                "warn",
                {
                    "reason": "local overlay contains non-generic keys",
                    "non_generic_keys": [
                        key for key in data if not _is_generic_target(key)
                    ],
                },
            )
        decision = validate_bypass_boundary(data)
        if decision.decision == "blocked":
            return None, None, ExperimentalDiagnosticRecord(
                "ENV_TOOLCHAIN_LOCAL_OVERLAY_REJECTED",
                "warn",
                {
                    "reason": "local overlay blocked by boundary validation",
                    "blocked_terms": list(decision.blocked_terms),
                },
            )
        return data, ExperimentalDiagnosticRecord(
            "ENV_TOOLCHAIN_LOCAL_OVERLAY_PROVENANCE",
            "info",
            {
                "asset_type": "local_overlay",
                "origin": "custom_path",
                "key_count": len(data),
                "redacted_ref": os.path.basename(os.fspath(overlay)),
            },
        ), None

    raise ValueError("local_overlay must be a dict or a PathLike path to a JSON file")


def _is_all_overlay_key_generic(data: dict[str, Any]) -> bool:
    """Reject overlay keys that are not generic browser environment targets."""
    return all(_is_generic_target(key) for key in data)


def _profile_coherence_groups(environment: dict[str, Any] | None) -> list[ProfileCoherenceGroup]:
    values, sources = _coherence_value_source(environment)
    return [
        _language_coherence_group(values, sources),
        _screen_window_coherence_group(values, sources),
        _ua_platform_coherence_group(values, sources),
        _network_info_coherence_group(values, sources),
    ]


def _coherence_value_source(
    environment: dict[str, Any] | None,
) -> tuple[dict[str, Any], dict[str, str]]:
    values: dict[str, Any] = {}
    sources: dict[str, str] = {}
    try:
        from iv8_rs import JSContext

        defaults = JSContext.get_defaults()
    except Exception:  # noqa: BLE001 - coherence diagnostics must never break reports.
        defaults = {}
    for key, value in defaults.items():
        values[key] = value
        sources[key] = "profile_default"
    for key, value in (environment or {}).items():
        values[key] = value
        sources[key] = "environment"
    return values, sources


def _language_coherence_group(
    values: dict[str, Any],
    sources: dict[str, str],
) -> ProfileCoherenceGroup:
    fields = _coherence_fields(values, "navigator.language", "navigator.languages")
    field_sources = _coherence_fields(sources, "navigator.language", "navigator.languages")
    language = fields.get("navigator.language")
    languages = fields.get("navigator.languages")
    if not isinstance(language, str) or not isinstance(languages, list) or not languages:
        return ProfileCoherenceGroup(
            group_id="language",
            status="unknown",
            fields=fields,
            sources=field_sources,
            reason="language or languages value is unavailable or malformed",
        )
    first_language = languages[0]
    if isinstance(first_language, str) and first_language == language:
        return ProfileCoherenceGroup(
            group_id="language",
            status="consistent",
            fields=fields,
            sources=field_sources,
            reason="primary language matches first languages entry",
        )
    return ProfileCoherenceGroup(
        group_id="language",
        status="inconsistent",
        fields=fields,
        sources=field_sources,
        reason="primary language does not match first languages entry",
    )


def _screen_window_coherence_group(
    values: dict[str, Any],
    sources: dict[str, str],
) -> ProfileCoherenceGroup:
    keys = (
        "screen.width",
        "screen.height",
        "screen.availWidth",
        "screen.availHeight",
        "window.innerWidth",
        "window.innerHeight",
        "window.devicePixelRatio",
    )
    fields = _coherence_fields(values, *keys)
    field_sources = _coherence_fields(sources, *keys)
    if any(not _is_positive_number(fields.get(key)) for key in keys):
        return ProfileCoherenceGroup(
            group_id="screen_window",
            status="unknown",
            fields=fields,
            sources=field_sources,
            reason="one or more screen/window values are unavailable or malformed",
        )
    width = float(fields["screen.width"])
    height = float(fields["screen.height"])
    avail_width = float(fields["screen.availWidth"])
    avail_height = float(fields["screen.availHeight"])
    inner_width = float(fields["window.innerWidth"])
    inner_height = float(fields["window.innerHeight"])
    if avail_width > width or avail_height > height:
        return ProfileCoherenceGroup(
            group_id="screen_window",
            status="inconsistent",
            fields=fields,
            sources=field_sources,
            reason="available screen dimensions exceed screen dimensions",
        )
    if inner_width > width or inner_height > height:
        return ProfileCoherenceGroup(
            group_id="screen_window",
            status="inconsistent",
            fields=fields,
            sources=field_sources,
            reason="window dimensions exceed screen dimensions",
        )
    return ProfileCoherenceGroup(
        group_id="screen_window",
        status="consistent",
        fields=fields,
        sources=field_sources,
        reason="screen, available screen, and window dimensions are bounded",
    )


def _ua_platform_coherence_group(
    values: dict[str, Any],
    sources: dict[str, str],
) -> ProfileCoherenceGroup:
    keys = (
        "navigator.userAgent",
        "navigator.platform",
        "navigator.userAgentData.platform",
        "navigator.userAgentData.mobile",
    )
    fields = _coherence_fields(values, *keys)
    field_sources = _coherence_fields(sources, *keys)
    user_agent = fields.get("navigator.userAgent")
    platform = fields.get("navigator.platform")
    ua_data_platform = fields.get("navigator.userAgentData.platform")
    ua_data_mobile = fields.get("navigator.userAgentData.mobile")
    if not isinstance(user_agent, str) or not isinstance(platform, str):
        return ProfileCoherenceGroup(
            group_id="ua_platform",
            status="unknown",
            fields=fields,
            sources=field_sources,
            reason="userAgent or platform value is unavailable or malformed",
        )
    if ua_data_platform is not None and not isinstance(ua_data_platform, str):
        return ProfileCoherenceGroup(
            group_id="ua_platform",
            status="unknown",
            fields=fields,
            sources=field_sources,
            reason="userAgentData platform value is malformed",
        )
    if ua_data_mobile is not None and not isinstance(ua_data_mobile, bool):
        return ProfileCoherenceGroup(
            group_id="ua_platform",
            status="unknown",
            fields=fields,
            sources=field_sources,
            reason="userAgentData mobile value is malformed",
        )
    ua_family = _platform_family_from_user_agent(user_agent)
    platform_family = _platform_family_from_platform(platform)
    ua_data_family = (
        _platform_family_from_ua_data_platform(ua_data_platform)
        if isinstance(ua_data_platform, str)
        else None
    )
    if (
        ua_family is not None
        and platform_family is not None
        and ua_family != platform_family
    ):
        return ProfileCoherenceGroup(
            group_id="ua_platform",
            status="inconsistent",
            fields=fields,
            sources=field_sources,
            reason="userAgent platform token contradicts navigator.platform",
        )
    if (
        ua_data_family is not None
        and platform_family is not None
        and ua_data_family != platform_family
    ):
        return ProfileCoherenceGroup(
            group_id="ua_platform",
            status="inconsistent",
            fields=fields,
            sources=field_sources,
            reason="userAgentData platform contradicts navigator.platform",
        )
    ua_is_mobile = _user_agent_has_mobile_token(user_agent)
    if (
        ua_data_mobile is True
        and not ua_is_mobile
        and platform_family in {"windows", "macos", "linux"}
    ):
        return ProfileCoherenceGroup(
            group_id="ua_platform",
            status="inconsistent",
            fields=fields,
            sources=field_sources,
            reason="userAgentData mobile flag contradicts desktop platform tokens",
        )
    if ua_data_mobile is False and ua_is_mobile:
        return ProfileCoherenceGroup(
            group_id="ua_platform",
            status="inconsistent",
            fields=fields,
            sources=field_sources,
            reason="userAgentData mobile flag contradicts mobile userAgent token",
        )
    return ProfileCoherenceGroup(
        group_id="ua_platform",
        status="consistent",
        fields=fields,
        sources=field_sources,
        reason="userAgent, platform, and userAgentData platform fields are bounded",
    )


def _network_info_coherence_group(
    values: dict[str, Any],
    sources: dict[str, str],
) -> ProfileCoherenceGroup:
    keys = (
        "navigator.connection.effectiveType",
        "navigator.connection.downlink",
        "navigator.connection.rtt",
        "navigator.connection.saveData",
        "navigator.connection.type",
    )
    fields = _coherence_fields(values, *keys)
    field_sources = _coherence_fields(sources, *keys)
    if not fields:
        return ProfileCoherenceGroup(
            group_id="network_info",
            status="unknown",
            fields=fields,
            sources=field_sources,
            reason="network information values are unavailable",
        )
    effective_type = fields.get("navigator.connection.effectiveType")
    connection_type = fields.get("navigator.connection.type")
    downlink = fields.get("navigator.connection.downlink")
    rtt = fields.get("navigator.connection.rtt")
    save_data = fields.get("navigator.connection.saveData")
    if effective_type is not None and not _is_non_empty_string(effective_type):
        return ProfileCoherenceGroup(
            group_id="network_info",
            status="unknown",
            fields=fields,
            sources=field_sources,
            reason="network effectiveType value is unavailable or malformed",
        )
    if connection_type is not None and not _is_non_empty_string(connection_type):
        return ProfileCoherenceGroup(
            group_id="network_info",
            status="unknown",
            fields=fields,
            sources=field_sources,
            reason="network type value is unavailable or malformed",
        )
    if downlink is not None and not _is_non_negative_number(downlink):
        return ProfileCoherenceGroup(
            group_id="network_info",
            status="inconsistent",
            fields=fields,
            sources=field_sources,
            reason="network downlink value is negative or malformed",
        )
    if rtt is not None and not _is_non_negative_number(rtt):
        return ProfileCoherenceGroup(
            group_id="network_info",
            status="inconsistent",
            fields=fields,
            sources=field_sources,
            reason="network rtt value is negative or malformed",
        )
    if save_data is not None and not isinstance(save_data, bool):
        return ProfileCoherenceGroup(
            group_id="network_info",
            status="unknown",
            fields=fields,
            sources=field_sources,
            reason="network saveData value is unavailable or malformed",
        )
    return ProfileCoherenceGroup(
        group_id="network_info",
        status="consistent",
        fields=fields,
        sources=field_sources,
        reason="network information values have bounded shape",
    )


def _coherence_fields(source: dict[str, Any], *keys: str) -> dict[str, Any]:
    return {key: source[key] for key in keys if key in source}


def _is_positive_number(value: Any) -> bool:
    return isinstance(value, int | float) and not isinstance(value, bool) and value > 0


def _is_non_negative_number(value: Any) -> bool:
    return isinstance(value, int | float) and not isinstance(value, bool) and value >= 0


def _is_non_empty_string(value: Any) -> bool:
    return isinstance(value, str) and bool(value)


def _platform_family_from_user_agent(value: str) -> str | None:
    lower = value.lower()
    if "android" in lower:
        return "android"
    if any(token in lower for token in ("iphone", "ipad", "ipod")):
        return "ios"
    if "windows" in lower:
        return "windows"
    if "mac os x" in lower or "macintosh" in lower:
        return "macos"
    if "linux" in lower or "x11" in lower:
        return "linux"
    return None


def _platform_family_from_platform(value: str) -> str | None:
    lower = value.lower()
    if lower.startswith("win"):
        return "windows"
    if lower.startswith(("mac", "darwin")):
        return "macos"
    if lower.startswith(("linux", "x11")):
        return "linux"
    if "android" in lower:
        return "android"
    if lower in {"iphone", "ipad", "ipod"}:
        return "ios"
    return None


def _platform_family_from_ua_data_platform(value: str) -> str | None:
    lower = value.lower()
    if lower == "windows":
        return "windows"
    if lower in {"macos", "mac os", "mac"}:
        return "macos"
    if lower == "linux":
        return "linux"
    if lower == "android":
        return "android"
    if lower in {"ios", "iphone", "ipad"}:
        return "ios"
    return None


def _user_agent_has_mobile_token(value: str) -> bool:
    lower = value.lower()
    return any(token in lower for token in ("mobile", "android", "iphone", "ipad", "ipod"))


def _profile_coherence_records(groups: list[ProfileCoherenceGroup]):
    from iv8_rs.experimental_report import ExperimentalDiagnosticRecord

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


def _classify_pressure_category(gap: EnvironmentGap) -> str | None:
    return _GAP_CLASS_TO_PRESSURE_CATEGORY.get(gap.gap_class)


def _classify_target_family(target: str) -> str | None:
    if target in _ALLOWED_TARGET_FAMILIES:
        return target
    if target.startswith(("performance.", "Date.", "timing.")):
        return "timing"
    if target.startswith(("navigator.connection", "networkInformation.")):
        return "network_info"
    for prefix in _GENERIC_TARGET_PREFIXES:
        family = prefix.rstrip(".")
        if target.startswith(prefix) and family in _ALLOWED_TARGET_FAMILIES:
            return family
    for family in _ALLOWED_TARGET_FAMILIES:
        if target.startswith(family + "."):
            return family
    if target in ("localStorage", "sessionStorage"):
        return "window"
    return None


def _map_gaps_to_family_pressures(
    gaps: list[EnvironmentGap],
) -> list[FamilyPressure]:
    buckets: dict[tuple[str, str], list[str]] = {}
    for gap in gaps:
        category = _classify_pressure_category(gap)
        if category is None:
            continue
        family = _classify_target_family(gap.target)
        if family is None:
            continue
        key = (category, family)
        if key not in buckets:
            buckets[key] = []
        if gap.gap_class not in buckets[key]:
            buckets[key].append(gap.gap_class)
    return [
        FamilyPressure(
            pressure_id=f"{category}__{family}",
            category=category,
            target_family=family,
            gap_classes=sorted(gap_classes),
        )
        for (category, family), gap_classes in sorted(buckets.items())
    ]


def _family_pressure_summary_records(
    pressures: list[FamilyPressure],
):
    from iv8_rs.experimental_report import ExperimentalDiagnosticRecord

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


def _native_substrate_review_records(
    groups: list[ProfileCoherenceGroup],
    pressures: list[FamilyPressure],
):
    from iv8_rs.experimental_report import ExperimentalDiagnosticRecord

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


def _coverage_delta(before_run: ProbeRun, after_run: ProbeRun) -> dict[str, int]:
    before_by_id = {observation.probe_id: observation for observation in before_run.observations}
    after_by_id = {observation.probe_id: observation for observation in after_run.observations}
    improved = 0
    regressed = 0
    for probe_id, before in before_by_id.items():
        after = after_by_id.get(probe_id)
        if after is None:
            continue
        if not before.passed and after.passed:
            improved += 1
        elif before.passed and not after.passed:
            regressed += 1
    return {"improved": improved, "regressed": regressed, "unresolved": len(after_run.gaps)}


def _run_single_probe(ctx: Any, probe: ProbeDefinition) -> ProbeObservation:
    try:
        actual = ctx.eval(_probe_eval_source(probe.js))
        passed = actual == probe.expected
        return ProbeObservation.from_probe(probe, actual=actual, passed=passed)
    except Exception as exc:  # noqa: BLE001 - probe failures are diagnostic inputs.
        return ProbeObservation.from_probe(
            probe,
            actual=None,
            passed=False,
            error=str(exc),
        )


def _probe_eval_source(js: str) -> str:
    return f"(function(){{\n{js}\n}})()"


def _coverage_from_observations(observations: list[ProbeObservation]) -> dict[str, int]:
    present = sum(1 for observation in observations if observation.passed)
    missing = sum(
        1
        for observation in observations
        if not observation.passed and observation.gap_class == "missing_api"
    )
    mismatch = len(observations) - present - missing
    return {"present": present, "missing": missing, "mismatch": mismatch}


def _gap_diagnostics(gaps: list[EnvironmentGap]) -> list[dict[str, Any]]:
    diagnostics = []
    for gap in gaps:
        code = "ENV_TOOLCHAIN_GAP_OBSERVED"
        severity = "info"
        if gap.gap_class == "descriptor_mismatch":
            code = "ENV_TOOLCHAIN_DESCRIPTOR_MISMATCH"
            severity = "warn"
        diagnostics.append(_diagnostic(
            code,
            severity,
            "environment.probe",
            f"{gap.gap_class} observed for {gap.target}",
            {
                "probe_id": gap.probe_id,
                "target": gap.target,
                "gap_class": gap.gap_class,
                "error": gap.error,
            },
        ))
    return diagnostics


def _diagnostic(
    code: str,
    severity: str,
    stage: str,
    message: str,
    details: dict[str, Any] | None = None,
) -> dict[str, Any]:
    diagnostic: dict[str, Any] = {
        "code": code,
        "severity": severity,
        "stage": stage,
        "message": message,
    }
    if details is not None:
        diagnostic["details"] = details
    return diagnostic


def _provenance_diagnostic(provenance: AssetProvenance) -> dict[str, Any]:
    details: dict[str, Any] = {
        "asset_type": provenance.asset_type,
        "pack_id": provenance.pack_id,
        "origin": provenance.origin,
    }
    if provenance.version is not None:
        details["version"] = provenance.version
    if provenance.redacted_ref is not None:
        details["redacted_ref"] = provenance.redacted_ref
    return _diagnostic(
        provenance.diagnostic_code,
        "info",
        "environment.asset",
        f"{provenance.asset_type} loaded from {provenance.origin}",
        details,
    )


def _provenance_record(provenance: AssetProvenance):
    from iv8_rs.experimental_report import ExperimentalDiagnosticRecord

    diagnostic = _provenance_diagnostic(provenance)
    return ExperimentalDiagnosticRecord(
        diagnostic["code"],
        diagnostic["severity"],
        diagnostic.get("details"),
    )
