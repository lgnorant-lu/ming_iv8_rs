# Environment pressure harness

## Purpose

Classify **input / failure / pressure / promotion** signals for diagnostic planning.  
Default-off / explicit invocation — not a background scanner.

## Key symbols (complete export set)

| Symbol | Role |
|---|---|
| `build_pressure_report` | Build report |
| `run_environment_pressure_samples` / `run_environment_pressure_manifest` | Batch runners |
| `PressureSignal` / `PressureSample` / `PressureManifestItem` | Inputs |
| `EnvironmentPressureReport` / `EnvironmentPressureBatch` | Outputs |
| `classify_input_kind` / `classify_failure_kind` / `pressure_from_failure` | Classifiers |
| `default_execution_mode` | Default execution mode helper |
| `promotion_for_pressure` / `PromotionDecision` | Promotion suggestion (diagnostic) |
| `pressure_batch_diagnostics` | Batch → diagnostics |
| `environment_pressure_batch_to_toolchain_diagnostics` | Bridge into toolchain diagnostics |
| `pressure_report_from_dict` / `pressure_report_to_dict` | Serde |
| `INPUT_KINDS` / `FAILURE_KINDS` / `PRESSURE_KINDS` / `PROMOTION_LEVELS` / `EXECUTION_MODES` | Enumerations |
| `ENVIRONMENT_PRESSURE_SCHEMA_VERSION` | Schema version |

## Bounds

- No automatic substrate mutation.  
- Bridge to planning is **review-oriented**.  

## Related

- [toolchain.md](toolchain.md)  
- [../reports/README.md](../reports/README.md)  
