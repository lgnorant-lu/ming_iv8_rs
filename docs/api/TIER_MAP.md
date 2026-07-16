# Public export tier map

> SoT: [`docs/conventions/api-documentation-conventions.md`](../conventions/api-documentation-conventions.md) §3  
> **A=15 · B=42 · C=87 · total=144** (= `len(iv8_rs.__all__)`)  
> Updated: 2026-07-16  
> Check: `uv run python scripts/_api_doc_inventory.py`

## Tier A (15) — full contract required

```
__version__
Debugger
JSCompileError
JSContext
JSError
JSMemoryError
JSPanic
JSTimeoutError
enable_logging
instrument_source
load_profile
plan_multi_entry
prepare_entry
run_with_entry
trace_diff
```

Docs: [runtime/](runtime/), [instrumentation/](instrumentation/), [entry/](entry/), [profiles.md](profiles.md), [versioning.md](versioning.md).

## Tier B (42) — behavioral module API

```
CFG
TaintEngine
block_mutation
build_corpus_report
build_environment_patch
build_evidence_diagnostics
build_pressure_report
build_trace_diagnostics
build_trace_events
classify_failure_kind
classify_input_kind
classify_trace_prefix
compare_vm_versions
compress_trace
confidence_from_evidence
decide_patch_policy
default_execution_mode
default_executor
detect_all
detect_constants
detect_hotspots
detect_loops
detect_patterns
detect_sequences
diff_analysis
environment_pressure_batch_to_toolchain_diagnostics
evaluate_evidence_gate
evidence_satisfies
exec_vm_handler
load_manifest
parse_trace
parse_trace_stream
pressure_batch_diagnostics
pressure_from_failure
probe_environment
promotion_for_pressure
run_corpus_manifest
run_environment_plane
run_environment_pressure_manifest
run_environment_pressure_samples
run_environment_toolchain
runtime_safe_candidate
```

Docs: [analysis/](analysis/), [environment/](environment/), [entry/](entry/).

## Tier C (87) — catalog index only

All `__all__` symbols not in A or B (DTO/report types, schema constants, serde helpers, match carriers, policy option types).

Docs: [reports/README.md](reports/README.md) + analysis/environment indexes.

## Maintenance

1. New public export → add to A/B here (else it is C by residual).  
2. Keep A∪B∪C partition of `__all__` (C = residual).  
3. Run inventory script; update [COVERAGE.md](COVERAGE.md).
