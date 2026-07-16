# Environment plane — signature index (Tier B)

> Depth: **Tier B**. Default product policy: **diagnostic / report-only**.  
> No silent profile write; no automatic substrate mutation unless caller opts in.

## Plane / toolchain

| Symbol | Signature (summary) | Notes |
|---|---|---|
| `run_environment_plane` | `(js_source, *, profile="default", environment=None, random_seed=42, time_freeze=None, time_mode="logical", entry_expr=None, patch_defaults=None, policy="runtime_safe") -> EnvironmentPlaneReport` | probe → patch plan → optional rerun |
| `run_environment_toolchain` | `(js_source, *, probe_pack=..., profile=..., environment=None, candidate_pack=..., apply_runtime_safe=False, adapt_runtime_safe=False, local_overlay=None, max_iterations=1, stop_on_regression=True, random_seed=42, time_freeze=None, time_mode="logical", entry_expr=None, dry_run_planning=False, ...) -> report` | Full toolchain; many flags **default off** |
| `build_environment_patch` | (see module) | Build patch candidates from gaps |

## Policy

| Symbol | Signature | Notes |
|---|---|---|
| `decide_patch_policy` | `(candidate, *, options=None) -> PatchPolicyDecision` | Allow/deny under policy |
| `block_mutation` | `(target, *, reason=None) -> PatchPolicyDecision` | Always-blocking decision helper |
| `runtime_safe_candidate` | (see module) | Runtime-safe candidate filter |

## Pressure

| Symbol | Signature | Notes |
|---|---|---|
| `build_pressure_report` | `(sample_id, source, *, message=None, status=None, sample_count=1) -> EnvironmentPressureReport` | No-write pressure report |
| `run_environment_pressure_samples` / `run_environment_pressure_manifest` | batch runners | See [pressure.md](pressure.md) |

## Honesty

| Claim | Reality |
|---|---|
| Auto-fix production hosts | **Not** default |
| One-click bypass kit | **Not** product API |
| Report models | Tier C catalog: [../reports/README.md](../reports/README.md) |

## Related

- [README.md](README.md) · [toolchain.md](toolchain.md) · [pressure.md](pressure.md)  
