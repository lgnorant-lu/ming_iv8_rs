"""Environment Plane workflow helpers.

This module formalizes the existing Environment Probe into a small
probe -> patch -> rerun workflow. The patch step is intentionally conservative:
it generates deterministic placeholder values for missing targets so callers can
inspect what changed rather than treating the result as an automatic fix.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass
from typing import Any, Dict, List, Optional


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

    patch = build_environment_patch(before, policy=policy, defaults=patch_defaults)
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

    return EnvironmentPlaneReport(
        before=before,
        patch=patch,
        after=after,
        improved_targets=sorted(before_missing - after_missing),
        unresolved_targets=sorted(after_missing),
        workflow=["probe", "patch", "rerun"],
        policy=policy,
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
