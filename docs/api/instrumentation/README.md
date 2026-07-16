# Instrumentation contracts

## Two product paths

| API | Assumption | Use when |
|---|---|---|
| `ctx.instrument_chaosvm(...)` | Handler table on **globalThis** | Global-table VMs |
| `iv8_rs.instrument_source(src)` | Static rewrite of dispatch sites | **Closure-scoped** handlers (e.g. TDC / many ChaosVM builds) |

**Path A (recommended for TDC-class):** `instrument_source` → `ctx.eval(patched)` → `get_unified_trace()`.  
`instrument_chaosvm` failing with missing global table is **expected**, not an incomplete install.

## `instrument_source` options

| Option | Default | Role |
|---|---|---|
| `mode` | `"auto"` | `"auto"` \| `"chaosvm"` \| `"switch_vm"` |
| `capture_stack_depth` | `3` | Stack elements captured per dispatch |
| `capture_env` | `True` | Inject host Proxies for env observation |
| `env_targets` | curated globals | Allow-list of roots to proxy |
| `limit` | `100000` | Max trace entries |
| `handler_array` / `pc_var` / `stack_var` / `index_array` | auto | Manual detect override |
| `dispatch_pattern` | auto | Exact dispatch expression override |
| `expose_handlers` | `False` | Assign `globalThis.__iv8_vm_handlers__` for analysis |

## Outputs (`info`)

| Field | Meaning |
|---|---|
| `mode` | Detection mode used |
| `handler_array` / `pc_var` / `stack_var` / `index_array` | Detected or overridden names |
| `dispatch_pattern` / `dispatch_offset` | Rewrite site |
| `dispatch_count` / offsets | Multi-site rewrite coverage when present |
| `recommended_api` | Suggested follow-up API string |
| `head_code_length` | Injected head length when present |

Returns: `(patched_source, info)`.

## Env observation

- Default may install host-safe Proxies for selected globals.  
- Control with `env_targets` / `capture_env=False`.  
- Proxies use correct Reflect receivers to avoid Illegal invocation on brand checks.

## `expose_handlers`

Optional rewrite flag (default **false**): assigns `globalThis.__iv8_vm_handlers__` for analysis.  
Detection-surface tradeoff — leave off for production-like runs.

## Unified trace format (summary)

Lines of form `TYPE,PC,target,value` where TYPE is typically `D`/`R`/`C`/`W` (dispatch / read / call / write).  
Diff two runs with `iv8_rs.trace_diff(a, b)`.

## Instance helpers (secondary)

| Method | Role |
|---|---|
| `detect_chaosvm_vars` | Global-table shape detect |
| `instrument_chaosvm` / `uninstrument_chaosvm` | Global table wrap |
| `get_vm_trace` / `get_unified_trace` | Buffers after eval |
| `detect_vm_dispatch` / `trace_vm` | Script-URL oriented helpers |

## Related

- GUIDE §15 / §129  
- [../runtime/module-level.md](../runtime/module-level.md)  
- [../runtime/jscontext.md](../runtime/jscontext.md)  
