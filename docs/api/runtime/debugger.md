# `Debugger`

Helper around a live `JSContext` for API tracing and property watches.

```python
import iv8_rs

ctx = iv8_rs.JSContext()
dbg = iv8_rs.Debugger(ctx)
dbg.trace_api("Math.random")
dbg.watch_property("navigator", "userAgent", mode="both")
result, log = dbg.eval_traced("Math.random()")
ctx.close()
```

**Bounds:** same thread as the bound `JSContext`. Prefer large-stack thread for kernel creation (K-010).

## Construction

| Item | Contract |
|---|---|
| `Debugger(ctx)` | Bind to an open `JSContext` |
| Returns | `Debugger` instance |

## Methods (complete list — 10 live)

| Method | Signature (summary) | Returns | Role |
|---|---|---|---|
| `trace_api` | `(api_path: str)` | `None` | Trace one API path |
| `trace_apis` | `(paths: list[str])` | `None` | Trace many |
| `get_call_log` | `()` | `list[dict]` | Call log entries |
| `clear_call_log` | `()` | `None` | Clear log |
| `get_traced_apis` | `()` | `list[str]` | Currently traced paths |
| `eval_traced` | `(source: str)` | `(result, log)` | Eval + log snapshot |
| `snapshot` | `()` | object/dict | Environment/API snapshot |
| `watch_property` | `(obj_path, prop, mode="both")` | `None` | Watch read/write/both |
| `get_call_summary` | `()` | `dict[str, int]` | Aggregated counts |
| `schedule_pause` | `()` | `None` | Request pause cooperation |

## Raises

| Situation | Exception |
|---|---|
| Underlying context closed / wrong thread | `RuntimeError` |
| JS errors inside `eval_traced` | `JSError` family (same as `ctx.eval`) |

## Related

- GUIDE Debugger / CDP sections  
- [jscontext.md](jscontext.md) for low-level CDP  
- [exceptions.md](exceptions.md)  
