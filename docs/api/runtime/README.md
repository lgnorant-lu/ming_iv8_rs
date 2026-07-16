# Runtime API (L0–L1)

## Contents

| Page | Scope |
|---|---|
| [jscontext.md](jscontext.md) | `JSContext` — full method inventory |
| [exceptions.md](exceptions.md) | Exception types |
| [debugger.md](debugger.md) | `Debugger` helper class |
| [module-level.md](module-level.md) | Module-level functions on `iv8_rs` |
| [../profiles.md](../profiles.md) | Profiles and environment merge |

## Primary import surface

```python
import iv8_rs

ctx = iv8_rs.JSContext(...)           # factory wrapping native context
iv8_rs.instrument_source(src)         # static rewrite helper
iv8_rs.prepare_entry(...)             # entry plane
iv8_rs.run_with_entry(...)
iv8_rs.enable_logging(...)
iv8_rs.trace_diff(...)
iv8_rs.Debugger(ctx)
```

## Design notes

- `JSContext` is a **factory function** (not a pure Python subclass of the frozen PyO3 class).
- Native type lives in `iv8_rs._iv8`; public docs describe the **supported** surface, not private `_` helpers.
