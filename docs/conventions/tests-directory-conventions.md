# Tests Directory Conventions

> Created: 2026-06-18
> Status: proposed
> Scope: All files and directories under `tests/`
> Supersedes: §10 (Organization) of `python-testing-conventions.md`
> Related: `testing-conventions.md`, `naming-conventions.md`

## Purpose

Define how the `tests/` directory tree is organized, what goes in root vs.
subdirectories, how helpers are named, when to split into subdirectories, and
how archive/stale content is handled. This document complements the per-file
and per-function rules in `python-testing-conventions.md`.

---

## 1. Directory Tree Overview

```text
tests/
  conftest.py                        ← Shared pytest fixtures
  experimental_contract_helpers.py   ← Contract test helpers (schema.json loader)

  # --- Test Files (pytest-collected) ---
  test_smoke.py                      ← M0: basic eval, context lifecycle
  test_api_contract.py               ← API surface behavioral contracts
  test_cfg.py                        ← Per-module behavioral tests
  test_taint.py
  test_isolation.py
  test_trace.py
  test_crypto_detection.py
  test_crypto_full.py
  test_cdp.py
  test_expose.py
  test_feedback_loop.py
  test_probe.py
  test_properties.py                 ← Hypothesis property-based tests
  test_typed_array.py
  test_strict_compat_false.py
  test_fingerprint.py
  test_ground_truth.py
  test_diagnostics.py
  test_diagnostic_bridge.py
  test_convergence.py
  test_trace_evidence_diagnostics.py
  test_memory_stability.py
  test_network_handler.py
  test_vm_diff.py
  test_cross_reference.py
  test_webpack_bridge_solidification.py
  test_witness_reports.py
  test_e2e_pipeline.py
  test_entry_plane.py
  test_environment_plane.py
  test_environment_plane_automation.py
  test_environment_policy.py
  test_environment_patch_policy.py
  test_environment_controlled_adaptation.py
  test_environment_pressure_manifest_smoke.py

  # --- Environment Toolchain (grouped tests) ---
  test_environment_toolchain_core.py
  test_environment_toolchain_candidates.py
  test_environment_toolchain_boundary.py
  test_environment_toolchain_bridge_contract.py
  test_environment_toolchain_assets.py
  test_environment_toolchain_pressure.py
  test_environment_toolchain_pressure_adaptation.py
  test_environment_toolchain_pressure_to_plan.py
  test_environment_toolchain_family_pressure.py
  test_environment_toolchain_diagnostic.py
  test_environment_toolchain_dry_run_planning.py
  test_environment_toolchain_iterative_adaptation.py
  test_environment_toolchain_native_review.py
  test_environment_toolchain_probe_packs.py
  test_environment_toolchain_probe_taxonomy.py
  test_environment_toolchain_profile_coherence.py
  test_environment_toolchain_profile_suggestions.py
  test_environment_toolchain_report_only_runner.py
  test_environment_toolchain_rollback_diagnostics.py
  test_environment_toolchain_runner.py
  test_environment_toolchain_runtime.py
  test_environment_toolchain_runtime_safe_rerun.py
  test_environment_toolchain_scaffold_gaps.py
  test_environment_toolchain_substrate_coverage.py

  # --- Parametrized Contract Tests ---
  test_contracts_boundary.py         ← wasm, framework, interpreter, ir, cff, anti-debug
  test_contracts_deobf.py            ← deobf-registry, deobf-validation, deobf-sandbox, deobf-string-array
  test_contracts_environment.py      ← environment-notes, environment-toolchain, multi-bundler
  test_contracts_vm.py               ← vm-analysis, vm-handler, vm-bytecode, vm-trace
  test_corpus_runner_contract.py     ← CorpusRunner API contract (behavioral)
  test_experimental_report_contract.py  ← ExperimentalReport typed round-trip

  # --- Report Runtime Tests ---
  test_deobf_reports_runtime.py
  test_ir_reports_runtime.py
  test_vm_reports_runtime.py
  test_string_array_reports_runtime.py

  # --- Milestone/Feature Area Tests ---
  test_m2_features.py
  test_m2_dom_events.py
  test_m4_debugger.py
  test_m5_examples.py

  # --- Version Acceptance Tests ---
  test_acceptance_v06.py
  test_acceptance_v07.py
  test_acceptance_v07_real_samples.py
  test_environment_pilot.py

  # --- IDL Probe Tests ---
  test_idl_probe_compatibility.py
  test_idl_probe_generation.py

  # --- Non-Test Helpers (not pytest-collected) ---
  experimental_contract_helpers.py   ← load_fixture, assert_fields, etc.
  profile_verification_checker.py    ← ConvergenceChecker diagnostic tool
  environment_probe_runner.py        ← ProbeRunner, GapList
  environment_dry_run_engine.py      ← DryRunEngine, Candidate
  environment_report_builder.py      ← ReportBuilder
  probe_compare.py                   ← iv8 vs iv8-rs probe diff tool

  # --- Data Files ---
  probe_results.json                 ← Cached/sample probe output

  # --- Subdirectories ---
  compat/                            ← Compatibility test suite (iv8 0.1.2 parity)
  fixtures/                          ← Test fixture data (JS, JSON, schema)
  iv8-ref/                           ← Reference examples/demos (external, not tests)
  _archive/                          ← Stale/obsolete artifacts (not collected)

  # --- Non-Test Scripts (underscore prefix) ---
  _audit_full.py
  _audit_m2.py
  _capture_api_output.py
  _diff_globals.py
  _e2e_abogus_like.py
  _generate_tier1_stub.py
  _get_full_diff.py
  _probe_iv8_doc.py
  _probe_netlog.py
  _probe_xhr_async.py
  _qq_baseline.py
  _run_abogus.py
  _samples_baseline.py
  _samples_deep.py
  _samples_eval.py
  _samples_full.py
  _samples_page.py
  _samples_triple.py
  _verify_gaps.py
  _verify_rs_env.py
  _verify_rs_multi.py
  _verify_rs.py
  _verify_tasks_64_67.py
  _webpack_exec.py
```

---

## 2. File Naming Rules

### 2.1 Test Files (`test_*.py`)

All files collected by pytest MUST start with `test_` and MUST contain at least
one `def test_*()` function.

```text
test_<capability>.py              ← Behavioral tests for a module
test_<capability>_contract.py     ← Parametrized schema validation (when multiple families exist)
test_<capability>_runtime.py      ← Runtime skeleton/end-to-end report tests
```

**Rules:**
- Use **capability names**, not version labels (`test_deobf_reports_runtime.py`, not `test_v08_deobf.py`)
- Use **capability names**, not milestone labels (`test_debugger.py`, not `test_m4_debugger.py`)
- Use **capability names**, not acceptance-version labels (`test_acceptance_v07.py` → should be `test_compat_v07.py` or merged into behavioral tests)
- Exception: `test_acceptance_v*` for historical cross-comparison with specific iv8 releases — these MUST be documented as frozen snapshots

### 2.2 Non-Test Scripts (`_*.py`)

Scripts that are NOT collected by pytest MUST use the `_` prefix:

```text
_<purpose>.py                     ← Audit, diagnostic, or development scripts
```

These are invisible to `pytest` collection. They may import from test helpers
and from `iv8_rs` directly.

### 2.3 Helper Modules

Helper modules that provide shared functions/classes for test files are named
by their role, without `test_` prefix and without `_` prefix:

```text
<role>_helpers.py                 ← Contract test helpers
<domain>_<role>.py                ← Domain-specific helpers
```

Helper modules reside in:
- **Root `tests/`** — when used by 3+ test files across multiple domains (e.g., `experimental_contract_helpers.py`)
- **Subdirectory** — when only used within that subdirectory's tests

### 2.4 Data Files

Data files (JSON, JS, etc.) reside in `tests/fixtures/`, organized by version
then capability:

```text
fixtures/
  v0.7/                            ← Legacy fixtures by version
    webpack-chunk.js
    dispatch-switch.js
    ...
  v0.8/                            ← Current fixtures by capability family
    deobf-registry/
      schema.json
    deobf-sandbox/
      schema.json
    ...
  environment_toolchain/
    candidate_packs/
      custom.valid.json
      custom.malformed.json
    probe_packs/
      fingerprint.m1.json
      descriptor.m1.json
```

---

## 3. Root vs. Subdirectory Placement

### 3.1 Root (`tests/`)

Place test files in root when:

1. The module under test is a top-level Python module in `python/iv8_rs/` (1:1 mapping)
2. The test file covers a cross-cutting concern not specific to one domain
3. The test file is a smoke/entry-point test

Examples:
- `test_cfg.py` → maps to `python/iv8_rs/cfg.py`
- `test_taint.py` → maps to `python/iv8_rs/taint.py`
- `test_smoke.py` → cross-cutting core eval
- `test_properties.py` → cross-cutting hypothesis tests

### 3.2 When to Create a Subdirectory

Create a subdirectory in `tests/` when EITHER:

| Condition | Threshold |
|---|---|
| **Test file count** | Domain has ≥6 `test_*` files |
| **Fixture count** | Domain has ≥10 fixture files or ≥3 fixture subdirectories |
| **Self-contained suite** | Test suite has its own helpers, generators, and fixtures |

**Examples:**

| Domain | File Count | Decision |
|---|---|---|
| `environment_toolchain_*` | ~25 test files | → subdirectory `tests/environment_toolchain/` |
| `compat/` | Own fixtures, generators, test file | → subdirectory (already exists) |
| `contracts_*` (parametrized) | 4 contract files | → stay in root (lightweight, shared helpers) |

### 3.3 Subdirectory Structure Convention

```text
tests/<domain>/
  conftest.py                      ← Domain-specific fixtures (optional)
  helpers/                         ← Domain-specific helpers (optional)
  fixtures/                        ← Domain-specific fixtures (optional)
  test_<sub_module>.py             ← Test files
  __init__.py                      ← Empty (package marker, optional for pytest)
```

**Subdirectories MUST NOT contain `_`-prefixed scripts.** Scripts are always in
root `tests/` or in the project's `tools/` directory.

---

## 4. Contract vs. Behavioral Test Placement

### 4.1 Contract Tests (Schema Validation)

| Type | Placement | Pattern |
|---|---|---|
| **Parametrized contract** — validates `schema.json` for multiple families | `tests/test_contracts_<area>.py` | `@pytest.mark.parametrize` over family/fields/code tuples |
| **API surface contract** — validates Python API shape (no real JS execution) | `tests/test_<module>_contract.py` | Flat test functions, uses `pytest.raises`, not parametrized |

**Rules:**
- One parametrized file per capability area (boundary, deobf, environment, vm)
- Single-fixture contract files (1-2 tests per family) are **prohibited** — merge into parametrized file
- API surface contract files (`test_corpus_runner_contract.py`, `test_api_contract.py`) are behavioral in nature but named `_contract` to indicate API stability guarantee — these are acceptable exceptions

### 4.2 Behavioral Tests

| Type | Placement | Pattern |
|---|---|---|
| **Module behavioral** — tests real JS execution via `iv8_rs` | `tests/test_<module>.py` | Uses `ctx`/`ctx_custom` fixtures from conftest |
| **Runtime report** — tests runtime execution end-to-end | `tests/test_<capability>_reports_runtime.py` | Full pipeline: load fixture → run → assert report fields |
| **Cross-cutting** — hypothesis, e2e pipeline, integration | `tests/test_<concern>.py` | Standalone, may combine multiple domains |

### 4.3 Decision Flow

```
Is the test validating a schema.json fixture?
  YES → Is it for 1 family or many?
    Many → Parametrized file: test_contracts_<area>.py
    One  → Is it experimental? → Merge into existing parametrized file
           Is it standalone?   → Only allowed if family has >5 fields and unique assertions
  NO  → Does it create a JSContext and eval JS?
    YES → Behavioral test file: test_<module>.py
    NO  → API surface contract test: test_<module>_contract.py
```

---

## 5. Helper Module Naming and Location

### 5.1 Classification

| Type | Example | Location |
|---|---|---|
| **Shared contract helpers** | `load_fixture()`, `assert_fields()`, `assert_no_strong_evidence()` | `tests/experimental_contract_helpers.py` |
| **Domain-specific helpers** | `ProbeRunner`, `GapList`, `Candidate` | `tests/` root (currently) or `tests/<domain>/helpers/` after subdirectory split |
| **Diagnostic/verification tools** | `ConvergenceChecker`, `DryRunEngine`, `ReportBuilder` | `tests/` root (prefer `_` prefix if not imported by tests) |
| **Comparison/probe scripts** | `probe_compare.py` | `tests/` root or `tools/` if project-wide |

### 5.2 Import Rules

Helper modules MUST NOT use `test_` prefix. They MAY use `_` prefix if they are
not imported by any test file (i.e., standalone scripts).

```python
# Correct — imported by test files
from experimental_contract_helpers import load_fixture

# Correct — standalone diagnostic, not imported
from _verify_rs_env import ...  # but _verify_rs_env is a script, not a module

# Wrong — helper module with test_ prefix (confuses pytest)
import test_my_helpers  # pytest will try to collect this
```

When a helper module grows beyond ~300 lines OR is imported by 6+ test files,
consider splitting it into a `tests/helpers/` package:

```text
tests/
  helpers/
    __init__.py
    contract.py           ← load_fixture, assert_fields (from experimental_contract_helpers.py)
    probe.py              ← ProbeRunner, GapList (from environment_probe_runner.py)
    engine.py             ← DryRunEngine, Candidate (from environment_dry_run_engine.py)
    builder.py            ← ReportBuilder (from environment_report_builder.py)
```

---

## 6. Archive / Stale Test Policy

### 6.1 `tests/_archive/` Directory

`tests/_archive/` stores **stale test artifacts** that are no longer part of
the active test suite but may be referenced for historical comparison.

**What goes in `_archive/`:**
- Old comparison results (`iv8_comparison_results/`)
- Previously-generated expected outputs that are no longer used
- Frozen snapshots of test output from superseded engine versions
- Any data file that is no longer referenced by a `test_*` file

**What does NOT go in `_archive/`:**
- Test files that still pass and cover current behavior
- Helper modules still imported by active tests
- Fixtures still referenced by active test files

### 6.2 Archive Rules

| Rule | Detail |
|---|---|
| **Prefix** | Directory uses `_` prefix to signal non-collection |
| **No test files** | `_archive/` MUST NOT contain `test_*` files |
| **No imports** | Active test files MUST NOT import from `_archive/` |
| **Referenced** | Archive contents may be referenced in commit messages, PR descriptions, or historical docs |
| **Cleanup** | Archive contents older than 6 months without reference in any open issue or doc may be deleted |

### 6.3 When to Archive

A test file or fixture enters archive when:

1. The capability it tests has been removed from the codebase
2. The fixture schema version is superseded and no active test references it
3. The comparison data compares against an iv8 version that is no longer supported

Move to `_archive/` (do NOT delete) so history is preserved. Add a note in the
commit message: `archive: <reason> for <path>`.

---

## 7. Special Directories

### 7.1 `tests/compat/`

The compatibility test suite is a **self-contained subdirectory** with its own
lifecycle:

```text
compat/
  __init__.py                      ← Package marker
  test_compat.py                   ← Parametrized compat test (eval output comparison)
  generate_expected.py             ← Generates .expected.json from iv8 0.1.2
  generate_m3_fixtures.py          ← Generates M3-specific fixtures
  fixtures/                        ← .js + .expected.json pairs
    wrap_native/
      001_basic.js
      001_basic.expected.json
      ...
```

**Rules:**
- `test_compat.py` is the only test file — parametrized over all `.js` fixtures
- Run with `pytest tests/compat/ -v` (not collected by default `pytest` run from root unless recursive)
- Generate scripts run in iv8's venv (`.venv-probe`), not iv8-rs venv
- Compat fixtures are immutable — regenerating requires explicit decision

### 7.2 `tests/iv8-ref/`

Contains reference demos and examples from the iv8 project. This is a
**read-only reference**, not an active test suite.

**Rules:**
- Files in `iv8-ref/` are NOT collected by pytest
- `test_m5_examples.py` references these examples by path — that is allowed
- `iv8-ref/` content is version-pinned (git submodule or frozen copy)
- Do NOT add new test files that depend on `iv8-ref/` internals — use `tests/fixtures/` instead

### 7.3 `tests/fixtures/`

Central fixture storage, organized by version then capability:

```text
fixtures/
  v0.7/                            ← Frozen v0.7 acceptance fixtures (.js files)
  v0.8/                            ← Current schema fixtures (each family has schema.json)
    anti-debug/
    cff/
    deobf-registry/
    ...
  environment_toolchain/           ← Environment toolchain test assets
    candidate_packs/
    probe_packs/
```

**Rules:**
- Fixtures are grouped by version IF the fixture shape differs across versions
- Within a version, fixtures are grouped by **capability family** (not by test file)
- Each `v0.8/<family>/schema.json` is the single source of truth for that contract
- Version labels in fixture path names are **explicitly allowed** — they describe data version, not code version

---

## 8. Review Checklist

- [ ] Test file name uses capability, not version/milestone label
- [ ] Non-test scripts use `_` prefix (no `test_` prefix without test functions)
- [ ] Helper modules do NOT use `test_` prefix
- [ ] Parametrized contract tests exist instead of per-family thin files
- [ ] Domain with ≥6 test files is identified for potential subdirectory
- [ ] `tests/_archive/` contains only stale, unreferenced artifacts
- [ ] `tests/compat/` is self-contained and documented
- [ ] `tests/fixtures/` follows version → capability organization
- [ ] No active test file imports from `_archive/`
- [ ] `test_m*` milestone files are documented with a plan to rename to capability names
- [ ] Data files (`probe_results.json`) have a documented purpose and owner
