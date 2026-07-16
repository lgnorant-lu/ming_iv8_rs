# Environment plane (entry)

## Core types / functions

| Symbol | Role |
|---|---|
| `run_environment_plane` | Run environment plane automation |
| `build_environment_patch` | Build patch description |
| `EnvironmentPatch` / `EnvironmentPlaneReport` | Result carriers |
| `decide_patch_policy` / `block_mutation` / `runtime_safe_candidate` | Policy helpers |
| `EnvironmentPatchCandidate` / `PatchPolicyDecision` / `PatchPolicyOptions` | Policy models |

## Governance

- Prefer **diagnostic / bounded** use.  
- **No silent promotion** of patches into production profiles without explicit product authorization.  
- Sample-specific bypass logic does **not** belong in this package API.

## Subpages

- [signatures.md](signatures.md) — Tier B signature index  
- [toolchain.md](toolchain.md) — full toolchain report surface  
- [pressure.md](pressure.md) — pressure harness  

## Related

- GUIDE Environment Toolchain chapters  
