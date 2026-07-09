"""Environment Plane workflow helpers.

This module formalizes the existing Environment Probe into a small
probe -> patch -> rerun workflow. The patch step is intentionally conservative:
it generates deterministic placeholder values for missing targets so callers can
inspect what changed rather than treating the result as an automatic fix.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass, field
from typing import Any

from iv8_rs.environment_policy import (
    PatchPolicyOptions,
    decide_patch_policy,
    runtime_safe_candidate,
)


@dataclass
class EnvironmentPatch:
    """Environment overrides produced from a probe report."""

    values: dict[str, Any]
    policy: str = "runtime_safe"
    source: str = "probe_missing_defaults"
    notes: list[str] | None = None

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass
class EnvironmentPlaneReport:
    """Result of a probe -> patch -> rerun Environment Plane pass."""

    before: dict[str, Any]
    patch: EnvironmentPatch
    after: dict[str, Any]
    improved_targets: list[str]
    unresolved_targets: list[str]
    workflow: list[str]
    policy: str = "runtime_safe"
    schema_version: str = "environment-plane.v0.1"
    patch_candidates: list[dict[str, Any]] = field(default_factory=list)
    applied_patches: list[dict[str, Any]] = field(default_factory=list)
    rejected_patches: list[dict[str, Any]] = field(default_factory=list)
    coverage: dict[str, Any] = field(default_factory=dict)
    evidence: list[dict[str, Any]] = field(default_factory=list)
    diagnostics: list[dict[str, Any]] = field(default_factory=list)

    def to_dict(self) -> dict[str, Any]:
        data = asdict(self)
        data["patch"] = self.patch.to_dict()
        return data


def build_environment_patch(
    probe_report: dict[str, Any],
    *,
    policy: str = "runtime_safe",
    defaults: dict[str, Any] | None = None,
) -> EnvironmentPatch:
    """Build deterministic environment overrides from probe missing targets."""
    default_map = defaults or {}
    values: dict[str, Any] = {}
    notes: list[str] = []

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
    profile: str | None = "default",
    environment: dict[str, Any] | None = None,
    random_seed: int | None = 42,
    time_freeze: float | None = None,
    time_mode: str = "logical",
    entry_expr: str | None = None,
    patch_defaults: dict[str, Any] | None = None,
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
    before_missing = before.get("missing", [])
    after_missing = after.get("missing", [])

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
        evidence=_build_environment_evidence(
            applied_patches, improved_targets,
            before_missing,
            [c.to_dict() for c in patch_candidates],
            rejected_patches,
            after_missing,
        ),
        diagnostics=_build_environment_diagnostics(
            decisions, improved_targets,
            before_missing, after_missing,
        ),
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
    probe_report: dict[str, Any],
    *,
    policy: str,
    defaults: dict[str, Any] | None,
    environment: dict[str, Any] | None,
):
    default_map = defaults or {}
    candidates = []
    decisions = []
    values: dict[str, Any] = {}
    notes: list[str] = []
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
    before: dict[str, Any],
    after: dict[str, Any],
    improved_targets: list[str],
    unresolved_targets: list[str],
) -> dict[str, Any]:
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
    applied_patches: list[dict[str, Any]],
    improved_targets: list[str],
    before_missing: list[str],
    candidates: list[dict[str, Any]],
    rejected_patches: list[dict[str, Any]],
    after_missing: list[str],
) -> list[dict[str, Any]]:
    evidence = []
    for target in before_missing:
        evidence.append({
            "kind": "environment_gap_observed",
            "strength": "diagnostic_only",
            "source": "environment_plane",
            "stage": "environment.probe",
            "summary": f"missing target observed: {target}",
            "payload": {"target": target},
        })

    for cand in candidates:
        evidence.append({
            "kind": "environment_patch_candidate",
            "strength": "diagnostic_only",
            "source": "environment_plane",
            "stage": "environment.patch",
            "summary": f"patch candidate considered: {cand.get('target', 'unknown')}",
            "payload": {"patch_id": cand.get("patch_id")},
        })

    for patch in applied_patches:
        evidence.append({
            "kind": "environment_patch_applied",
            "strength": "weak",
            "source": "environment_plane",
            "stage": "environment.patch",
            "summary": f"applied patch for {patch['target']}",
            "payload": {"patch_id": patch["patch_id"]},
        })

    for patch in rejected_patches:
        evidence.append({
            "kind": "environment_patch_rejected",
            "strength": "diagnostic_only",
            "source": "environment_plane",
            "stage": "environment.patch",
            "summary": f"patch rejected for {patch.get('target', 'unknown')}",
            "payload": {"patch_id": patch.get("patch_id")},
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

    if after_missing and len(after_missing) > len(before_missing):
        evidence.append({
            "kind": "environment_coverage_regressed",
            "strength": "diagnostic_only",
            "source": "environment_plane",
            "stage": "environment.rerun",
            "summary": "post-patch probe had more missing targets",
            "payload": {"before": len(before_missing), "after": len(after_missing)},
        })

    return evidence


def _build_environment_diagnostics(
    decisions,
    improved_targets: list[str],
    before_missing: list[str],
    after_missing: list[str],
    unsafe_attempted: bool = False,
    profile_write_attempted: bool = False,
) -> list[dict[str, Any]]:
    diagnostics = []
    for decision in decisions:
        severity = "info" if decision.decision == "applied" else "warn"
        if decision.decision == "blocked":
            severity = "error"
        code = "ENVIRONMENT_PATCH_CANDIDATE"
        if decision.decision == "applied":
            code = "ENVIRONMENT_PATCH_APPLIED"
        elif decision.decision in {"rejected", "deferred"}:
            code = "ENVIRONMENT_PATCH_REJECTED"
            if "conflict" in decision.reason.lower():
                code = "ENVIRONMENT_PATCH_CONFLICT"
        elif decision.decision == "blocked" and "unsafe" in decision.reason.lower():
            code = "ENVIRONMENT_PATCH_UNSAFE"
        diagnostics.append({
            "code": code,
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

    before_set = set(before_missing)
    after_set = set(after_missing)
    if not improved_targets and before_set == after_set:
        diagnostics.append({
            "code": "ENVIRONMENT_RERUN_NO_CHANGE",
            "severity": "warn",
            "stage": "environment.rerun",
            "message": "post-patch probe did not improve any observed gaps",
            "details": {"targets": list(before_set & after_set)},
        })
    elif after_missing and len(after_missing) > len(before_missing):
        diagnostics.append({
            "code": "ENVIRONMENT_RERUN_REGRESSED",
            "severity": "error",
            "stage": "environment.rerun",
            "message": "post-patch probe had more missing targets than before",
            "details": {"before": len(before_missing), "after": len(after_missing)},
        })

    if unsafe_attempted:
        diagnostics.append({
            "code": "ENVIRONMENT_PATCH_UNSAFE",
            "severity": "error",
            "stage": "environment.patch",
            "message": "unsafe hook candidate attempted without opt-in",
        })

    if profile_write_attempted:
        diagnostics.append({
            "code": "ENVIRONMENT_PROFILE_WRITE_BLOCKED",
            "severity": "error",
            "stage": "policy.check",
            "message": "automatic profile write was blocked by mutation guard",
        })

    return diagnostics


def _safe_len(value: Any) -> int:
    if value is None:
        return 0
    try:
        return len(value)
    except TypeError:
        return 0
