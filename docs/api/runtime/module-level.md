# Module-level functions (`import iv8_rs`)

Native symbols are re-exported from `iv8_rs._iv8` via `python/iv8_rs/__init__.py`.

## Logging

| Function | Signature | Role |
|---|---|---|
| `enable_logging` | `(level="info")` | Enable tracing; also `IV8_LOG` env (e.g. `iv8.init=debug`) |

Levels: `trace` / `debug` / `info` / `warn` / `error`.  
**Returns:** `None`. **Raises:** typically none for valid level strings (invalid levels may be ignored or mapped by the tracing bridge — prefer documented levels).

## Instrumentation

| Function | Role |
|---|---|
| `instrument_source` | Static rewrite of VM dispatch + optional env Proxies |

```python
patched, info = iv8_rs.instrument_source(
    source,
    mode="auto",                 # "auto" | "chaosvm" | "switch_vm"
    capture_stack_depth=3,
    capture_env=True,
    env_targets=None,            # default: navigator, screen, document, location, Math, crypto, performance
    limit=100000,
    handler_array=None,          # manual detect override
    pc_var=None,
    stack_var=None,
    index_array=None,
    dispatch_pattern=None,
    expose_handlers=False,       # assign globalThis.__iv8_vm_handlers__ (off by default)
)
```

| Return | Role |
|---|---|
| `patched` | Instrumented source string |
| `info` | Detection metadata: `mode`, `handler_array`, `pc_var`, `stack_var`, `dispatch_offset`, `dispatch_count` / offsets, `recommended_api`, … |

**Raises:** `RuntimeError` if no dispatch pattern detected and no manual overrides.

**Path A vs instance API:** see [../instrumentation/README.md](../instrumentation/README.md).

## Entry plane

| Function | Signature (summary) | Role |
|---|---|---|
| `prepare_entry` | `(source, persona="analysis", entry_targets=None) -> dict` | Single-source entry plan |
| `plan_multi_entry` | `(sources, persona="analysis") -> dict` | Multi named sources → multi-entry plan |
| `run_with_entry` | `(plan, source, chunks=None, entry_expr=None) -> dict` | Execute plan; **caller-supplied** chunk texts only |

| Arg | Contract |
|---|---|
| `persona` | `"runtime"` \| `"analysis"`; else **`ValueError`** |
| `sources` (multi) | `list[(name, source_text)]` |
| `chunks` | Ordered JS **source strings** evaluated before main; **no URL fetch** |
| Returns | Plan/result **dict** (JSON-shaped); not always the typed `EntryPlan` class |

**Raises (entry):** `ValueError` (bad persona); `RuntimeError` on serialization/execution failures; JS failures during `run_with_entry` may surface as `JSError` family when evaluation runs inside a kernel.

See [../entry/README.md](../entry/README.md).

## Trace utilities

| Function | Signature | Role |
|---|---|---|
| `trace_diff` | `(trace_a: list[str], trace_b: list[str]) -> dict` | First divergence: `index`, `a`, `b`, `total_a`, `total_b`, `match_count` |

**Returns:** dict as above; `index == -1` when identical. **Raises:** type errors if non-list inputs (Python).

## Version

| Symbol | Role |
|---|---|
| `__version__` | Package version string (D-151 package track) |

## Related Python modules

Re-exported analysis helpers (`parse_trace`, `CFG`, `detect_patterns`, `probe_environment`, report builders, …) are indexed under [../analysis/README.md](../analysis/README.md) and [../environment/](../environment/).

| Factory / profile | Role |
|---|---|
| `load_profile` | JSON profile → flat env dict ([../profiles.md](../profiles.md)) |
| `JSContext` | Factory with `profile=` |

## Related

- [jscontext.md](jscontext.md)  
- [../instrumentation/README.md](../instrumentation/README.md)  
- [../entry/README.md](../entry/README.md)  
