# Analysis / observability — signature index (Tier B)

> Depth: **Tier B** — purpose + signature + one-line return.  
> Full algorithms: GUIDE. Host runtime: [../runtime/](../runtime/).

Signatures from live `inspect.signature` (package **0.8.12**).  
If this page drifts, prefer code + re-run inventory.

## Trace

| Symbol | Signature | Returns / notes |
|---|---|---|
| `parse_trace` | `(raw: list[str]) -> StructuredTrace` | Parse unified trace lines |
| `parse_trace_stream` | `(iterable) -> StructuredTrace` | File/generator input |
| `compress_trace` | `(trace: StructuredTrace) -> CompressedTrace` | Merge consecutive same-PC dispatches |
| `trace_diff` | module-level on package | First divergence between two `list[str]` |

## Probe / multi-variant

| Symbol | Signature | Notes |
|---|---|---|
| `probe_environment` | `(js_source, profile=..., environment=None, random_seed=42, time_freeze=None, time_mode="logical", entry_expr=None) -> dict` | Instrumented env interaction report |
| `diff_analysis` | `(js_source, eval_expr, base_env, test_variables, random_seed=42, time_freeze=None, time_mode="logical", max_workers=4, progress_callback=None) -> dict` | Which env keys affect output |

## Patterns / crypto

| Symbol | Signature | Notes |
|---|---|---|
| `detect_patterns` | `(trace, patterns=None, opcode_map=None, window_size=20, min_confidence=0.6) -> list[PatternMatch]` | Opcode-structure patterns |
| `detect_all` | `(trace, min_confidence=0.5, enable_fuzzy=False, context_window=50, opcode_map=None) -> list[CryptoDetection]` | Cross-validated crypto detection |

## VM compare / isolation

| Symbol | Signature | Notes |
|---|---|---|
| `compare_vm_versions` | `(source_a, source_b, handler_array="A", similarity_threshold=0.95) -> DiffReport` | Handler-array oriented diff |
| `exec_vm_handler` | `(ctx, handler_expr, stack_var="g", stack_input=None, pc_var=None, pc_value=None, mock_env=None) -> dict` | Single-handler isolation |

## CFG / taint (types)

| Symbol | Construction | Notes |
|---|---|---|
| `CFG` | `(nodes: dict[int, CFGNode], edges: list[CFGEdge])` | Graph from dispatch trace |
| `TaintEngine` | `(trace: StructuredTrace, sources: dict[str, str])` | Value-matching taint |

## Corpus

| Symbol | Signature | Notes |
|---|---|---|
| `load_manifest` | `(path) -> list[CorpusManifestItem]` | Markdown table → records |
| `run_corpus_manifest` | `(manifest_path, *, options=None) -> dict` | Report without mutating manifest |
| `build_corpus_report` | `(items, *, manifest_path, options=None, executor=None) -> dict` | Draft report builder |

## Related

- Catalog index: [README.md](README.md)  
- Entry plane: [../entry/README.md](../entry/README.md)  
- Environment: [../environment/](../environment/)  
