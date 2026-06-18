# Naming Conventions

> Created: 2026-06-07
> Status: accepted
> Scope: post-v0.6 planning, runtime skeletons, tests, and docs

## Purpose

This document prevents roadmap version labels from leaking into long-lived API
names. Version labels are useful for release planning and schema evolution, but
runtime modules, public classes, public functions, and tests should be named by
capability and behavior.

## Core Rule

Use version labels for scope and schema only. Use capability names for code.

| Surface | Good | Avoid |
|---|---|---|
| schema version | `deobf-validation.v0.1` | unversioned schemas |
| roadmap docs | `v0.8.0-implementation-task-plan.md` | hiding release scope |
| Python module | `deobf_reports.py` | `v08_deobf.py` |
| Python class | `ExperimentalReport` | `V08Report` |
| Python function | `experimental_report_from_dict` | `v08_report_from_dict` |
| test file | `test_deobf_reports_runtime.py` | `test_v08_deobf.py` |
| helper module | `experimental_contract_helpers.py` | `v08_contract_helpers.py` |

## Python Modules

Python module names MUST describe the capability or report family:

- `experimental_report.py`
- `environment_toolchain.py`
- `deobf_reports.py`
- `string_array_reports.py`
- `vm_reports.py`
- `ir_reports.py`

Module names SHOULD NOT include roadmap labels such as `v08`, `v0_8`, `m8`, or
release tags unless the file is a migration, compatibility shim, or release-only
artifact.

## Public Classes

Public class names MUST describe the model they represent:

- `ExperimentalReport`
- `ExperimentalEvidenceRecord`
- `EnvironmentToolchainReport`
- `DeobfRegistryReport`
- `ValidationReport`
- `StringArrayReport`
- `VMAnalysisReport`
- `VMHandlerTable`
- `IRNodeReport`

Class names SHOULD NOT encode release phase. A class that remains useful after
v0.8.0 should not carry `V08` in its name.

## Public Functions

Conversion helpers SHOULD use `<capability>_from_dict` and `<capability>_to_dict`
when they expose schema-backed data models:

- `experimental_report_from_dict`
- `toolchain_report_from_dict`
- `registry_report_from_dict`
- `validation_report_from_dict`
- `string_array_report_from_dict`
- `vm_analysis_report_from_dict`
- `vm_handler_table_from_dict`
- `ir_node_report_from_dict`

## Test Files

Test names SHOULD describe what is under test:

- contract tests: `test_<capability>_contract.py`
- runtime skeleton tests: `test_<capability>_runtime.py`
- report model tests: `test_<capability>_reports_runtime.py`

Do not prefix test files with roadmap labels when the test targets a stable or
reusable capability model.

## Documentation Files

Roadmap and release-scope documents MAY include release labels when the document
is explicitly about that scope:

- `v0.8.0-implementation-task-plan.md`
- `v0.8-contract-gate-plan.md`
- `v0.8-maturity-assessment.md`

Capability specs SHOULD be named by capability:

- `deobf-validation-spec.md`
- `vm-handler-extraction-spec.md`
- `environment-toolchain-expansion-spec.md`

## Schema Versions

Schema versions SHOULD remain explicit and local to serialized data:

```json
{"schema_version": "vm-analysis.v0.1"}
```

Schema version strings are not Python API names. They are persisted contract
identifiers and may retain semantic version suffixes.

## Review Checklist

- [x] Roadmap labels are allowed in roadmap docs and serialized schema versions. (`accepted_default`)
- [x] Roadmap labels are blocked from public Python module, class, and function names. (`accepted_default`)
- [x] Runtime skeleton tests are named by capability behavior. (`accepted_default`)
- [x] Helper modules are named by role, not release phase. (`accepted_default`)
