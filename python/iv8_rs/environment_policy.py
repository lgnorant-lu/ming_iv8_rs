"""Environment patch policy decision helpers.

This module implements the policy vocabulary used by Environment Plane
automation: runtime_safe, analysis_only, and unsafe_hook. It is intentionally
side-effect free and never writes profiles, manifests, baselines, or samples.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass, field
from typing import Any, Dict, Iterable, List, Optional


POLICY_LEVELS = {"runtime_safe", "analysis_only", "unsafe_hook"}
PATCH_KINDS = {"value", "object", "getter", "method", "capture", "wrapper", "hook", "rewrite"}
POLICY_DECISIONS = {"applied", "rejected", "blocked", "deferred", "reclassified"}
MUTATION_TARGETS = {"profile", "manifest", "baseline", "sample", "source_tree"}

_RISK_ORDER = {"runtime_safe": 0, "analysis_only": 1, "unsafe_hook": 2}
_MIN_POLICY_BY_KIND = {
    "value": "runtime_safe",
    "object": "runtime_safe",
    "getter": "runtime_safe",
    "method": "runtime_safe",
    "capture": "analysis_only",
    "wrapper": "analysis_only",
    "hook": "unsafe_hook",
    "rewrite": "unsafe_hook",
}


@dataclass(slots=True)
class EnvironmentPatchCandidate:
    patch_id: str
    target: str
    kind: str
    policy: str
    source: str = "builtin_registry"
    value_preview: Any = None
    requires: List[str] = field(default_factory=list)
    risk_reasons: List[str] = field(default_factory=list)

    def __post_init__(self) -> None:
        if self.kind not in PATCH_KINDS:
            raise ValueError(f"invalid patch kind: {self.kind}")
        if self.policy not in POLICY_LEVELS:
            raise ValueError(f"invalid patch policy: {self.policy}")

    @classmethod
    def from_dict(cls, data: Dict[str, Any]) -> "EnvironmentPatchCandidate":
        return cls(
            patch_id=data["patch_id"],
            target=data["target"],
            kind=data["kind"],
            policy=data["policy"],
            source=data.get("source", "builtin_registry"),
            value_preview=data.get("value_preview"),
            requires=list(data.get("requires", [])),
            risk_reasons=list(data.get("risk_reasons", [])),
        )

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


@dataclass(slots=True)
class PatchPolicyOptions:
    persona: str = "runtime"
    allow_analysis_only: bool = False
    allow_unsafe_hook: bool = False
    allow_explicit_override: bool = False
    explicit_environment: Dict[str, Any] = field(default_factory=dict)
    profile_values: Dict[str, Any] = field(default_factory=dict)


@dataclass(slots=True)
class PatchPolicyDecision:
    patch_id: str
    target: str
    requested_policy: str
    effective_policy: str
    persona: str
    decision: str
    reason: str
    risk_reasons: List[str] = field(default_factory=list)
    requires_opt_in: bool = False
    opt_in_present: bool = False
    conflicts: List[str] = field(default_factory=list)
    diagnostic_code: str = "PATCH_POLICY_APPLIED"

    def __post_init__(self) -> None:
        if self.decision not in POLICY_DECISIONS:
            raise ValueError(f"invalid policy decision: {self.decision}")

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


def decide_patch_policy(
    candidate: EnvironmentPatchCandidate | Dict[str, Any],
    *,
    options: Optional[PatchPolicyOptions] = None,
) -> PatchPolicyDecision:
    """Decide whether a candidate may be applied under current policy."""
    cand = candidate if isinstance(candidate, EnvironmentPatchCandidate) else EnvironmentPatchCandidate.from_dict(candidate)
    opts = options or PatchPolicyOptions()
    effective_policy, reclassified = _effective_policy(cand)
    conflicts = _conflicts(cand, opts)

    if reclassified and cand.policy != effective_policy:
        diagnostic_code = "PATCH_POLICY_RECLASSIFIED"
    else:
        diagnostic_code = None

    if conflicts and not opts.allow_explicit_override:
        return PatchPolicyDecision(
            patch_id=cand.patch_id,
            target=cand.target,
            requested_policy=cand.policy,
            effective_policy=effective_policy,
            persona=opts.persona,
            decision="rejected",
            reason="candidate conflicts with explicit environment/profile values",
            risk_reasons=list(cand.risk_reasons),
            conflicts=conflicts,
            diagnostic_code="PATCH_POLICY_CONFLICT",
        )

    if effective_policy == "unsafe_hook":
        if not opts.allow_unsafe_hook:
            return _blocked(cand, opts, effective_policy, "unsafe hook requires explicit opt-in")
        if opts.persona != "analysis":
            return _rejected(cand, opts, effective_policy, "unsafe hook requires analysis persona",
                             diagnostic_code="PATCH_POLICY_PERSONA_MISMATCH")
        return _applied(cand, opts, effective_policy, requires_opt_in=True, opt_in_present=True,
                        diagnostic_code=diagnostic_code)

    if effective_policy == "analysis_only":
        if not opts.allow_analysis_only:
            return _rejected(cand, opts, effective_policy, "analysis-only patch requires explicit opt-in",
                             diagnostic_code="PATCH_POLICY_OPT_IN_MISSING")
        if opts.persona == "runtime" and not opts.allow_explicit_override:
            return _rejected(cand, opts, effective_policy, "runtime persona rejects analysis-only patch",
                             diagnostic_code="PATCH_POLICY_PERSONA_MISMATCH")
        return _applied(cand, opts, effective_policy, requires_opt_in=True, opt_in_present=True,
                        diagnostic_code=diagnostic_code)

    return _applied(cand, opts, effective_policy, diagnostic_code=diagnostic_code)


def block_mutation(target: str, *, reason: Optional[str] = None) -> PatchPolicyDecision:
    """Return a blocking decision for prohibited persistent mutations."""
    if target not in MUTATION_TARGETS:
        raise ValueError(f"unknown mutation target: {target}")
    return PatchPolicyDecision(
        patch_id=f"mutation.{target}",
        target=target,
        requested_policy="unsafe_hook",
        effective_policy="unsafe_hook",
        persona="analysis",
        decision="blocked",
        reason=reason or f"automatic {target} mutation is prohibited",
        risk_reasons=["persistent mutation"],
        requires_opt_in=True,
        opt_in_present=False,
        diagnostic_code="PATCH_POLICY_MUTATION_BLOCKED",
    )


def runtime_safe_candidate(patch_id: str, target: str, value: Any, *, source: str = "builtin_registry") -> EnvironmentPatchCandidate:
    return EnvironmentPatchCandidate(
        patch_id=patch_id,
        target=target,
        kind="value",
        policy="runtime_safe",
        source=source,
        value_preview=value,
    )


def _effective_policy(cand: EnvironmentPatchCandidate) -> tuple[str, bool]:
    min_policy = _MIN_POLICY_BY_KIND[cand.kind]
    if _RISK_ORDER[min_policy] > _RISK_ORDER[cand.policy]:
        return min_policy, True
    return cand.policy, False


def _conflicts(cand: EnvironmentPatchCandidate, opts: PatchPolicyOptions) -> List[str]:
    conflicts: List[str] = []
    if cand.target in opts.explicit_environment:
        conflicts.append("explicit_environment")
    if cand.target in opts.profile_values and cand.source == "builtin_registry":
        conflicts.append("profile_value")
    return conflicts


def _applied(
    cand: EnvironmentPatchCandidate,
    opts: PatchPolicyOptions,
    effective_policy: str,
    *,
    requires_opt_in: bool = False,
    opt_in_present: bool = False,
    diagnostic_code: Optional[str] = None,
) -> PatchPolicyDecision:
    return PatchPolicyDecision(
        patch_id=cand.patch_id,
        target=cand.target,
        requested_policy=cand.policy,
        effective_policy=effective_policy,
        persona=opts.persona,
        decision="applied",
        reason=f"{effective_policy} patch allowed by policy",
        risk_reasons=list(cand.risk_reasons),
        requires_opt_in=requires_opt_in,
        opt_in_present=opt_in_present,
        diagnostic_code=diagnostic_code or "PATCH_POLICY_APPLIED",
    )


def _rejected(
    cand: EnvironmentPatchCandidate,
    opts: PatchPolicyOptions,
    effective_policy: str,
    reason: str,
    *,
    diagnostic_code: str = "PATCH_POLICY_REJECTED",
) -> PatchPolicyDecision:
    return PatchPolicyDecision(
        patch_id=cand.patch_id,
        target=cand.target,
        requested_policy=cand.policy,
        effective_policy=effective_policy,
        persona=opts.persona,
        decision="rejected",
        reason=reason,
        risk_reasons=list(cand.risk_reasons),
        requires_opt_in=effective_policy in {"analysis_only", "unsafe_hook"},
        diagnostic_code=diagnostic_code,
    )


def _blocked(
    cand: EnvironmentPatchCandidate,
    opts: PatchPolicyOptions,
    effective_policy: str,
    reason: str,
) -> PatchPolicyDecision:
    return PatchPolicyDecision(
        patch_id=cand.patch_id,
        target=cand.target,
        requested_policy=cand.policy,
        effective_policy=effective_policy,
        persona=opts.persona,
        decision="blocked",
        reason=reason,
        risk_reasons=list(cand.risk_reasons),
        requires_opt_in=True,
        opt_in_present=False,
        diagnostic_code="PATCH_POLICY_BLOCKED",
    )
