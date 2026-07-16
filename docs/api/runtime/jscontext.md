# `JSContext` — full method surface

> **Live audit SoT:** runtime `dir(JSContext())` + `crates/iv8-py/src/context.rs`  
> **Stubs:** `python/iv8_rs/_iv8.pyi`  
> **Factory:** `python/iv8_rs/__init__.py` (`profile=` merge)  
> **Narrative:** GUIDE. This page is the **complete contract checklist**.

**Live public instance/class methods (audit):** **50** (see inventory below).  
**Not public:** `assert_thread` / `assert_creator_thread` are **Rust private** helpers (raise `RuntimeError` internally); do not call them from Python.

## Construction

```python
import iv8_rs

ctx = iv8_rs.JSContext(
    environment=None,       # dict overrides (flat or nested; nested auto-flattened)
    config=None,            # timezone / locale / storage_path
    time_mode="logical",    # "logical" | "system"
    js_api="__iv8__",       # internal tool object name
    strict_compat=True,     # True ≈ iv8 0.1.x conversion; False = enhanced types
    random_seed=None,       # int | None — Math.random
    crypto_seed=None,       # int | None — crypto.getRandomValues
    time_freeze=None,       # float | None — Date.now() ms freeze
    worker_mode=False,      # Worker-side construction path
    # factory-only (Python wrapper, not native kwarg name on class):
    profile=None,           # "default" | path — merged under environment
)
```

| Concern | Contract |
|---|---|
| Profile merge | `environment` > `profile` > built-in defaults ([profiles.md](../profiles.md)) |
| `config` keys | `timezone` (IANA), `locale` (BCP 47), `storage_path` |
| Timezone | `environment["timezone"]` / `config["timezone"]` → process `TZ` + V8 Redetect; needs ICU 77 |
| Thread | Create and use on the **same** thread (V8 isolate affinity). Misuse → `RuntimeError` |
| Stack | Module import sets `threading.stack_size(128MB)` |
| Isolate serial | Concurrent full kernels may serialize init; close unused contexts |
| Context manager | `with iv8_rs.JSContext(...) as ctx:` |

### Raises (host / construction / lifecycle)

| Situation | Exception |
|---|---|
| Invalid `time_mode` | `ValueError` |
| Eval after `close` / disposed | `RuntimeError` (`closed`) |
| Use on non-creator thread | `RuntimeError` (thread affinity) |
| JS throw at runtime | `JSError` (or subclass) |
| JS syntax/compile error | `JSCompileError` |
| Kernel init / ICU failure | may surface as `JSError` / `JSMemoryError` / host error — see [exceptions.md](exceptions.md) |

Full exception map: [exceptions.md](exceptions.md).

### Class / factory helpers

| Name | Role |
|---|---|
| `JSContext(...)` | Public factory (`profile=` supported) |
| `JSContext.get_defaults()` | Built-in default environment snapshot (classmethod) |

---

## Lifecycle

| Method | Role |
|---|---|
| `close()` | Dispose isolate/kernel |
| `is_disposed()` | Whether closed |
| `__enter__` / `__exit__` | Context manager (auto-close) |

---

## Execution

| Method | Signature (summary) | Role |
|---|---|---|
| `eval` | `(source, /, name=None, line=-1, col=-1, to_py=False, devtools=True) -> Any` | Evaluate JS |
| `eval_promise` | `(source, max_ticks=1000) -> Any` | Eval + drive loop until Promise settles |

**Exceptions:** [exceptions.md](exceptions.md).

---

## Page / DOM load

| Method | Signature | Role |
|---|---|---|
| `page_load` | `(html, base_url=None)` | Parse HTML, scripts, DOMContentLoaded |
| `page_load_with_headers` | `(html, base_url=None, headers=None)` | Load + response headers (e.g. Set-Cookie) |

---

## Resources & network

| Method | Signature | Role |
|---|---|---|
| `add_resource` | `(url, body, status=200, headers=None)` | ResourceBundle offline entry |
| `set_network_handler` | `(handler)` | Python fallback `(url, method) -> (status, body) \| None` |
| `clear_network_handler` | `()` | Remove handler |

**Bound:** ResourceBundle → Python handler → NetworkError.

---

## Exposing Python into JS

| Method | Role |
|---|---|
| `expose(name_or_data, callable_or_name=None)` | Mode1: `expose(name, callable)`; Mode2: data under tool object |
| `expose_module(module)` | Expose module callables to global |

---

## Storage

| Method | Role |
|---|---|
| `persist_storage(path)` | Persist storage state |
| `load_storage(path)` | Load storage state |

---

## Console

| Method | Role |
|---|---|
| `get_console_messages()` | Read console buffer |
| `clear_console_messages()` | Clear buffer |

---

## Inspector / DevTools / CDP

| Method | Signature (summary) | Role |
|---|---|---|
| `with_devtools` | `(port=9229, watch_apis=None, enable_console=True, wait=True) -> self` | Start inspector |
| `get_devtools_url` | `() -> str \| None` | DevTools URL |
| `process_inspector_messages` | `()` | Pump inspector |
| `cdp_set_breakpoint` | `(url, line, column=None, condition=None)` | Breakpoint |
| `cdp_remove_breakpoint` | `(breakpoint_id)` | Remove |
| `cdp_evaluate_on_frame` | `(call_frame_id, expression)` | Eval in frame |
| `cdp_resume` / `cdp_step_over` / `cdp_step_into` / `cdp_step_out` | | Control |
| `cdp_get_call_frames` | `() -> list \| None` | Stack when paused |
| `cdp_get_scope_properties` | `(object_id, own_properties=True)` | Scope props |
| `cdp_process_events` | `() -> bool` | Events; paused? |

**Programmatic CDP:** call `with_devtools(wait=False)` first.

**Raises (precision, from native binding):**

| Condition | Exception |
|---|---|
| Wrong thread / closed context | `RuntimeError` via `assert_thread` |
| CDP op fails (no inspector session, bad id, not paused, …) | `RuntimeError` (`PyRuntimeError` from kernel `map_err`) |
| JS eval errors inside CDP evaluate | may surface as `JSError` family depending on path |

Paused-at-breakpoint is **not** an exception: use `cdp_process_events` / `cdp_get_call_frames`.  
**Thread:** same creator thread as the context.

English note: Chinese prose removed here for public contract consistency; see README.zh-CN for product narrative.

---

## Trace points

| Method | Role |
|---|---|
| `set_trace_point(url, line, column=None, expression="'hit'")` | Register |
| `remove_trace_point(trace_point_id)` | Remove |
| `get_trace_log()` | Read |
| `clear_trace_log()` | Clear |
| `set_trace_limit(max_entries)` | Cap |

**Raises:** 未启动 devtools 时 `set_trace_point` / `remove_trace_point` 抛 `RuntimeError`。`get_trace_log` / `clear_trace_log` / `set_trace_limit` 不影响。

---

## VM instrumentation (instance)

| Method | Role |
|---|---|
| `detect_chaosvm_vars(source)` | Detect global-table ChaosVM shapes |
| `instrument_chaosvm(handler_array, pc_var, stack_var, capture_stack_depth=3, limit=100000)` | Global handler table only |
| `uninstrument_chaosvm(handler_array)` | Undo |
| `get_vm_trace` / `clear_vm_trace` | VM buffer |
| `get_unified_trace` / `clear_unified_trace` | Unified D/R/C/W lines |
| `detect_vm_dispatch(script_url, patterns=None)` | Dispatch detect |
| `trace_vm(url, line, column=None, vars=None, limit=50000)` | Trace helper |

**Prefer** module-level `instrument_source` for closure-scoped handlers.  
See [../instrumentation/README.md](../instrumentation/README.md).

---

## Recording / profiler / coverage

| Method | Role |
|---|---|
| `start_recording(targets=None, record_reads=True, record_writes=True, record_calls=True, limit=50000)` | Start session recording |
| `stop_recording()` | Stop → entries |
| `start_profiler()` / `stop_profiler()` | CPU profile |
| `start_coverage()` / `stop_coverage()` | Coverage |

**Raises:** `start_profiler` / `start_coverage` 需要 `with_devtools(wait=False)` 前置；未启动时抛 `RuntimeError`。  
`start_recording` 不依赖 CDP，**不抛**（但 limit 超限可能静默截断——见 `set_trace_limit` 行为）。

---

## Worker internal

| Method | Role |
|---|---|
| `set_worker_prototype()` | Set `globalThis.__proto__` to DedicatedWorkerGlobalScope.prototype (Worker path; not general app API) |

---

## Live inventory (50 methods)

```text
add_resource
cdp_evaluate_on_frame
cdp_get_call_frames
cdp_get_scope_properties
cdp_process_events
cdp_remove_breakpoint
cdp_resume
cdp_set_breakpoint
cdp_step_into
cdp_step_out
cdp_step_over
clear_console_messages
clear_network_handler
clear_trace_log
clear_unified_trace
clear_vm_trace
close
detect_chaosvm_vars
detect_vm_dispatch
eval
eval_promise
expose
expose_module
get_console_messages
get_defaults
get_devtools_url
get_trace_log
get_unified_trace
get_vm_trace
instrument_chaosvm
is_disposed
load_storage
page_load
page_load_with_headers
persist_storage
process_inspector_messages
remove_trace_point
set_network_handler
set_trace_limit
set_trace_point
set_worker_prototype
start_coverage
start_profiler
start_recording
stop_coverage
stop_profiler
stop_recording
trace_vm
uninstrument_chaosvm
with_devtools
```

Plus dunder: `__enter__`, `__exit__` (and standard object protocol).

## Related

- [module-level.md](module-level.md)  
- [debugger.md](debugger.md)  
- [../profiles.md](../profiles.md)  
