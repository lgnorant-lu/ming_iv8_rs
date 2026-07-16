# Analysis & observability modules

These are **first-class library modules**. Deep algorithms: GUIDE.

**Doc depth (honest):** symbols here are mostly **Tier B** (behavioral) or
**Tier C** (DTO / report carriers). This page is a **catalog + role index**,
not Google-level full contracts for every DTO field. For Tier A host APIs see
[../runtime/](../runtime/) and [../instrumentation/](../instrumentation/).

## Trace

| Symbol | Role |
|---|---|
| `parse_trace` / `parse_trace_stream` | Parse unified/raw traces |
| `StructuredTrace` / `CompressedTrace` | Structured / compressed views |
| `compress_trace` | Compression helper |
| `trace_diff` | First-divergence diff (`list[str]` ×2 → dict) |

## CFG

| Symbol | Role |
|---|---|
| `CFG` | Control-flow reconstruction |

## Taint

| Symbol | Role |
|---|---|
| `TaintEngine` / `TaintReport` | Taint tracking |

## Patterns / crypto detection

| Symbol | Role |
|---|---|
| `detect_patterns` / `detect_sequences` / `detect_loops` / `detect_hotspots` / `detect_constants` / `detect_all` | Engines |
| `PatternMatch` / `SequenceMatch` / `ConstantMatch` / `CryptoDetection` | Result types |

## VM diff / isolation

| Symbol | Role |
|---|---|
| `compare_vm_versions` / `DiffReport` / `HandlerDiff` | Cross-version VM compare |
| `exec_vm_handler` | Isolated handler execution |

## Probe

| Symbol | Role |
|---|---|
| `probe_environment` | Environment probe entry |

## Diff analysis

| Symbol | Role |
|---|---|
| `diff_analysis` | Multi-variant diff helper |

## Diagnostics building blocks

| Symbol | Role |
|---|---|
| `DiagnosticRecord` / `EvidenceRecord` / `EvidenceGateResult` / `TraceEvent` / `FallbackAttempt` | Carriers |
| `DIAGNOSTIC_CATALOG` / `TRACE_PREFIX_REGISTRY` | Catalogs |
| `build_trace_diagnostics` / `build_evidence_diagnostics` / `build_trace_events` | Builders |
| `classify_trace_prefix` / `confidence_from_evidence` / `evaluate_evidence_gate` / `evidence_satisfies` | Gates |

## Corpus (offline multi-case)

| Symbol | Role |
|---|---|
| `load_manifest` / `run_corpus_manifest` / `build_corpus_report` | Runner |
| `CorpusManifestItem` / `CorpusRunOptions` | Types |
| `default_executor` | Default executor callable |

## Report models

Schema-backed reports (deobf, string-array, VM, IR, experimental): [../reports/README.md](../reports/README.md).

## Signature index

Tier B signatures (inspect-based): **[signatures.md](signatures.md)**.

## Related

- [../instrumentation/README.md](../instrumentation/README.md)  
- [../entry/README.md](../entry/README.md)  
