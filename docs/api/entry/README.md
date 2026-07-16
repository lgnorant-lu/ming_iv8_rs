# Entry plane & multi-bundler

## Functions

| Function | Signature (summary) | Role |
|---|---|---|
| `prepare_entry` | `(source, persona="analysis", entry_targets=None) -> dict` | Analyze one source → EntryPlan-shaped dict |
| `plan_multi_entry` | `(sources: list[(name, text)], persona="analysis") -> dict` | Multi-bundle plan (`iv8-multi-entry-plan.v0.1`) |
| `run_with_entry` | `(plan, source, chunks=None, entry_expr=None) -> dict` | Execute plan; optional pre-eval chunk texts |

```python
import iv8_rs

plan = iv8_rs.prepare_entry(runtime_src, persona="analysis")
result = iv8_rs.run_with_entry(
    plan,
    page_src,
    chunks=[vendor_src, runtime_src],  # ordered source strings only
    entry_expr=None,
)

multi = iv8_rs.plan_multi_entry(
    [("runtime.js", runtime_src), ("vendor.js", vendor_src)],
    persona="analysis",
)
```

## Persona

| Value | Role |
|---|---|
| `"analysis"` | Default; planning for analysis workflows |
| `"runtime"` | Runtime-oriented planning |

Invalid persona → `ValueError`.

## Raises

| Function | Exception | When |
|---|---|---|
| `prepare_entry` / `plan_multi_entry` | `ValueError` | `persona` not `runtime`\|`analysis` |
| `prepare_entry` / `plan_multi_entry` / `run_with_entry` | `RuntimeError` | serialization / planner execution failure |
| `run_with_entry` | `JSError` family | JS evaluation failure inside the run (when kernel eval runs) |

**Returns:** JSON-shaped **dict** (not necessarily an instance of `EntryPlan` / `EntryResult` classes).

## Models (Python)

| Type | Role |
|---|---|
| `EntryPlan` | Planned strategies / probes (typed helper) |
| `EntryResult` | Execution outcome |
| `SelectedStrategy` | Chosen strategy metadata |

Plan/result from native entry APIs are **dicts** compatible with these models where applicable.

## Offline-first network honesty (Q163)

| Browser webpack | iv8-rs product |
|---|---|
| `ensureChunk` may fetch URL | **No** silent HTTP chunk download as core feature |
| — | Caller downloads if needed, passes **source text** into entry APIs / ResourceBundle |

Dynamic `require(expr)` is **known incomplete by design** (Q164) unless pre-resolved by adapter.

## Supported bridge families (capability, not parity claim)

Webpack (including named `webpackChunk*`, factory install, edges/cycles), Parcel (`$parcel$` / `parcelRequire`), Browserify, Vite-adjacent ESM helpers (import.meta / dynamic import / TLA minimal).  
Detection + bridge quality evolve; residual gaps live in bundler TODO ledgers (private).

## Corpus (related)

| Symbol | Role |
|---|---|
| `load_manifest` / `run_corpus_manifest` / `build_corpus_report` | Offline multi-case runner |
| `CorpusManifestItem` / `CorpusRunOptions` | Manifest types |
| `corpus_main` | CLI entry |

## Related

- GUIDE EntryPlane sections  
- [../runtime/module-level.md](../runtime/module-level.md)  
- Private honesty: `docs/todo/TODO-bundler.md`  
