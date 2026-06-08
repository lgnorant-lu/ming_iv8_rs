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
_ORDERED_RECIPE_RE = re.compile(r"apply\s+.+request\s+.+(?:copy|rerun)", re.IGNORECASE)
_PROBE_PACK_FILES = {
    "descriptor.m1": "descriptor.m1.json",
    "fingerprint.m1": "fingerprint.m1.json",
}
_CANDIDATE_PACK_FILES = {"chrome_generic": "chrome_generic.json"}


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
    random_seed: int | None = 42,
    time_freeze: float | None = None,
    time_mode: str = "logical",
    entry_expr: str | None = None,
):
    """Run the Environment Toolchain flow with optional runtime-safe rerun."""

    from iv8_rs.environment_toolchain import (
        CoverageDelta,
        EnvironmentToolchainReport,
        ToolchainPatchEntry,
    )
    from iv8_rs.experimental_report import ExperimentalDiagnosticRecord, ExperimentalEvidenceRecord

    candidate_pack_object, candidate_provenance = _resolve_candidate_pack(candidate_pack)
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
    profile_suggestions = _profile_suggestions_from_candidates(candidates)
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
