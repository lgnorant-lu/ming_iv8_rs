"""Environment Plane workflow helpers.

This module formalizes the existing Environment Probe into a small
probe -> patch -> rerun workflow. The patch step is intentionally conservative:
it generates deterministic placeholder values for missing targets so callers can
inspect what changed rather than treating the result as an automatic fix.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass, field
from typing import Any, Dict, List, Optional

from iv8_rs.environment_policy import (
    PatchPolicyOptions,
    decide_patch_policy,
    runtime_safe_candidate,
)


@dataclass
class EnvironmentPatch:
    """Environment overrides produced from a probe report."""

    values: Dict[str, Any]
    policy: str = "runtime_safe"
    source: str = "probe_missing_defaults"
    notes: Optional[List[str]] = None

    def to_dict(self) -> Dict[str, Any]:
        return asdict(self)


@dataclass
class EnvironmentPlaneReport:
    """Result of a probe -> patch -> rerun Environment Plane pass."""

    before: Dict[str, Any]
    patch: EnvironmentPatch
    after: Dict[str, Any]
    improved_targets: List[str]
    unresolved_targets: List[str]
    workflow: List[str]
    policy: str = "runtime_safe"
    schema_version: str = "environment-plane.v0.1"
    patch_candidates: List[Dict[str, Any]] = field(default_factory=list)
    applied_patches: List[Dict[str, Any]] = field(default_factory=list)
    rejected_patches: List[Dict[str, Any]] = field(default_factory=list)
    coverage: Dict[str, Any] = field(default_factory=dict)
    evidence: List[Dict[str, Any]] = field(default_factory=list)
    diagnostics: List[Dict[str, Any]] = field(default_factory=list)

    def to_dict(self) -> Dict[str, Any]:
        data = asdict(self)
        data["patch"] = self.patch.to_dict()
        return data


def build_environment_patch(
    probe_report: Dict[str, Any],
    *,
    policy: str = "runtime_safe",
    defaults: Optional[Dict[str, Any]] = None,
) -> EnvironmentPatch:
    """Build deterministic environment overrides from probe missing targets."""
    default_map = defaults or {}
    values: Dict[str, Any] = {}
    notes: List[str] = []

    for target in sorted(set(probe_report.get("missing", []))):
        values[target] = default_map.get(target, _default_value_for_target(target))
        notes.append(f"{target}: generated conservative placeholder")

    return EnvironmentPatch(
        values=values,
        policy=policy,
        source="probe_missing_defaults",
        notes=notes,
    )


def run_environment_plane(
    js_source: str,
    *,
    profile: Optional[str] = "default",
    environment: Optional[Dict[str, Any]] = None,
    random_seed: Optional[int] = 42,
    time_freeze: Optional[float] = None,
    time_mode: str = "logical",
    entry_expr: Optional[str] = None,
    patch_defaults: Optional[Dict[str, Any]] = None,
    policy: str = "runtime_safe",
) -> EnvironmentPlaneReport:
    """Run probe -> patch -> rerun and return a structured report."""
    from iv8_rs.probe import probe_environment

    before = probe_environment(
        js_source,
        profile=profile,
        environment=environment,
        random_seed=random_seed,
        time_freeze=time_freeze,
        time_mode=time_mode,
        entry_expr=entry_expr,
    )

    patch, patch_candidates, decisions = _build_policy_checked_patch(
        before,
        policy=policy,
        defaults=patch_defaults,
        environment=environment,
    )
    merged_env = dict(environment or {})
    merged_env.update(patch.values)

    after = probe_environment(
        js_source,
        profile=profile,
        environment=merged_env if merged_env else None,
        random_seed=random_seed,
        time_freeze=time_freeze,
        time_mode=time_mode,
        entry_expr=entry_expr,
    )

    before_missing = set(before.get("missing", []))
    after_missing = set(after.get("missing", []))
    improved_targets = sorted(before_missing - after_missing)
    unresolved_targets = sorted(after_missing)
    applied_patches = [d.to_dict() for d in decisions if d.decision == "applied"]
    rejected_patches = [d.to_dict() for d in decisions if d.decision != "applied"]

    return EnvironmentPlaneReport(
        before=before,
        patch=patch,
        after=after,
        improved_targets=improved_targets,
        unresolved_targets=unresolved_targets,
        workflow=["probe", "patch", "rerun"],
        policy=policy,
        patch_candidates=[c.to_dict() for c in patch_candidates],
        applied_patches=applied_patches,
        rejected_patches=rejected_patches,
        coverage=_build_coverage(before, after, improved_targets, unresolved_targets),
        evidence=_build_environment_evidence(applied_patches, improved_targets),
        diagnostics=_build_environment_diagnostics(decisions, improved_targets),
    )


def _default_value_for_target(target: str) -> Any:
    if target.endswith(".webdriver"):
        return False
    if target.endswith(".language"):
        return "en-US"
    if target.endswith(".languages"):
        return ["en-US", "en"]
    if target.endswith(".platform"):
        return "Win32"
    if target.endswith(".width"):
        return 1920
    if target.endswith(".height"):
        return 1080
    if target.endswith(".deviceMemory"):
        return 8
    if target.endswith(".hardwareConcurrency"):
        return 8
    return {}


def _build_policy_checked_patch(
    probe_report: Dict[str, Any],
    *,
    policy: str,
    defaults: Optional[Dict[str, Any]],
    environment: Optional[Dict[str, Any]],
):
    default_map = defaults or {}
    candidates = []
    decisions = []
    values: Dict[str, Any] = {}
    notes: List[str] = []
    opts = PatchPolicyOptions(
        persona="analysis" if policy == "analysis_only" else "runtime",
        allow_analysis_only=policy == "analysis_only",
        allow_unsafe_hook=policy == "unsafe_hook",
        explicit_environment=dict(environment or {}),
    )

    for target in sorted(set(probe_report.get("missing", []))):
        value = default_map.get(target, _default_value_for_target(target))
        source = "caller_defaults" if target in default_map else "builtin_registry"
        candidate = runtime_safe_candidate(
            f"{target}.default",
            target,
            value,
            source=source,
        )
        candidates.append(candidate)
        decision = decide_patch_policy(candidate, options=opts)
        decisions.append(decision)
        if decision.decision == "applied":
            values[target] = value
            notes.append(f"{target}: generated conservative placeholder")
        else:
            notes.append(f"{target}: patch {decision.decision}: {decision.reason}")

    return EnvironmentPatch(
        values=values,
        policy=policy,
        source="probe_missing_defaults",
        notes=notes,
    ), candidates, decisions


def _build_coverage(
    before: Dict[str, Any],
    after: Dict[str, Any],
    improved_targets: List[str],
    unresolved_targets: List[str],
) -> Dict[str, Any]:
    before_missing = len(before.get("missing", []))
    after_missing = len(after.get("missing", []))
    return {
        "probe_coverage_before": {
            "missing": before_missing,
            "present": _safe_len(before.get("present")),
            "mismatch": _safe_len(before.get("mismatch")),
        },
        "probe_coverage_after": {
            "missing": after_missing,
            "present": _safe_len(after.get("present")),
            "mismatch": _safe_len(after.get("mismatch")),
        },
        "coverage_delta": {
            "improved": len(improved_targets),
            "unresolved": len(unresolved_targets),
        },
    }


def _build_environment_evidence(
    applied_patches: List[Dict[str, Any]],
    improved_targets: List[str],
) -> List[Dict[str, Any]]:
    evidence = []
    for patch in applied_patches:
        evidence.append({
            "kind": "environment_patch_applied",
            "strength": "weak",
            "source": "environment_plane",
            "stage": "environment.patch",
            "summary": f"applied patch for {patch['target']}",
            "payload": {"patch_id": patch["patch_id"]},
        })
    if improved_targets:
        evidence.append({
            "kind": "environment_coverage_improved",
            "strength": "weak",
            "source": "environment_plane",
            "stage": "environment.rerun",
            "summary": "post-patch probe improved environment coverage",
            "payload": {"targets": improved_targets},
        })
    return evidence


def _build_environment_diagnostics(decisions, improved_targets: List[str]) -> List[Dict[str, Any]]:
    diagnostics = []
    for decision in decisions:
        severity = "info" if decision.decision == "applied" else "warn"
        if decision.decision == "blocked":
            severity = "error"
        diagnostics.append({
            "code": "ENVIRONMENT_PATCH_APPLIED" if decision.decision == "applied" else "ENVIRONMENT_PATCH_REJECTED",
            "severity": severity,
            "stage": "environment.patch",
            "message": decision.reason,
            "details": decision.to_dict(),
        })
    if improved_targets:
        diagnostics.append({
            "code": "ENVIRONMENT_RERUN_IMPROVED",
            "severity": "info",
            "stage": "environment.rerun",
            "message": "post-patch probe improved observed gaps",
            "details": {"targets": improved_targets},
        })
    return diagnostics


def _safe_len(value: Any) -> int:
    if value is None:
        return 0
    try:
        return len(value)
    except TypeError:
        return 0
