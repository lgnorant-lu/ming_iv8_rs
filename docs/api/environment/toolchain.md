# Environment toolchain (diagnostic plane)

## Honesty (non-negotiable)

| Allowed | Not allowed (default product) |
|---|---|
| Produce typed **reports** | Silent disk apply of “fixes” |
| Dry-run / plan / rollback **diagnostics** | Package-specific site bypass adapters |
| Explicit `runtime_safe` paths when gated | Claiming full MAPE-K auto-remediation |

## Primary entry

| Symbol | Role |
|---|---|
| `run_environment_toolchain(...)` | Main runner |
| `EnvironmentToolchainReport` | Report carrier |
| `toolchain_report_from_dict` / `toolchain_report_to_dict` | Serde helpers |
| `CoverageSnapshot` / `CoverageDelta` | Coverage carriers |
| `ProfileSuggestion` / `ToolchainPatchEntry` | Suggestion / patch entries |

## Module decomposition (implementation map)

Public package splits implementation across:

- `environment_toolchain_runtime`  
- `environment_toolchain_diagnostics`  
- `environment_toolchain_boundary`  
- `environment_toolchain_asset_*`  
- `environment_toolchain_candidate_mapping`  
- `environment_toolchain_probe_taxonomy`  
- `environment_toolchain_pressure_planning`  
- `environment_toolchain_pressure_adaptation`  
- `environment_toolchain_models` / `static` / …

**API contract:** use re-exports from `iv8_rs` where provided; treat submodules as implementation detail unless re-exported.

## Related

- [pressure.md](pressure.md)  
- [README.md](README.md)  
