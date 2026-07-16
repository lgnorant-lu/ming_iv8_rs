# Experimental / schema-backed report models

These carriers serialize analysis/toolchain outputs with versioned schemas.  
**Policy:** reports are **data**, not execution engines. Pin schema version fields when persisting.

**Doc depth (honest):** **Tier C catalog** — one-line roles + serde helpers.
Not a full field-by-field IDL for every nested type; use the Python types /
`*_from_dict` schemas as machine SoT.

## Experimental

| Symbol | Role |
|---|---|
| `ExperimentalReport` | Top-level experimental report |
| `ExperimentalEvidenceRecord` / `ExperimentalDiagnosticRecord` | Nested records |
| `experimental_report_from_dict` / `experimental_report_to_dict` | Serde |
| `experimental_report_roundtrip` | Round-trip helper |
| `EXPERIMENTAL_SCHEMA_VERSIONS` | Schema version map |

## Deobf registry / validation

| Symbol | Role |
|---|---|
| `DeobfRegistryReport` / `RegistryEntry` / `SelectionReport` | Registry carriers |
| `registry_report_from_dict` / `registry_report_to_dict` | Serde |
| `ValidationReport` / `ValidationCheck` | Validation carriers |
| `validation_report_from_dict` / `validation_report_to_dict` | Serde |

## String array

| Symbol | Role |
|---|---|
| `StringArrayReport` / `StringArrayCandidate` | Report + candidates |
| `StringDecoder` / `RotationIIFE` / `ReplacementSite` | Decoder / site models |
| `string_array_report_from_dict` / `string_array_report_to_dict` | Serde |

## VM analysis / handler table

| Symbol | Role |
|---|---|
| `VMAnalysisReport` / `vm_analysis_report_from_dict` / `vm_analysis_report_to_dict` | Analysis report |
| `VMHandlerTable` / `vm_handler_table_from_dict` / `vm_handler_table_to_dict` | Handler table |
| `HandlerEntry` / `BytecodeCandidate` / `OpcodeHint` | Table entries / hints |
| `HandlerTableSummary` / `TraceSummary` / `StateModel` | Summaries |

## IR nodes

| Symbol | Role |
|---|---|
| `IRNodeReport` / `IRNode` / `ConfidenceSummary` | IR report graph |
| `ir_node_report_from_dict` / `ir_node_report_to_dict` | Serde |

## Toolchain / pressure (cross-link)

See [../environment/toolchain.md](../environment/toolchain.md) and [../environment/pressure.md](../environment/pressure.md) for:

- `EnvironmentToolchainReport`, `toolchain_report_*`, `CoverageSnapshot`, `CoverageDelta`, `ProfileSuggestion`, `ToolchainPatchEntry`
- Pressure reports, classifiers, `ENVIRONMENT_PRESSURE_SCHEMA_VERSION`, …

## Related

- GUIDE Runtime Report Models chapters  
- [../analysis/README.md](../analysis/README.md)  
