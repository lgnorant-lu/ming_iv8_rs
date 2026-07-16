# Profiles & environment

## `load_profile(path) -> dict`

Load a flat fingerprint/environment dict from JSON.

| `path` | Meaning |
|---|---|
| `"default"` | Built-in Chrome-line default under `iv8_rs/profiles/` |
| filesystem path | User JSON |

Keys use **dot-path** form compatible with `JSContext(environment=...)`.  
Keys starting with `_meta.` are stripped.

| | |
|---|---|
| **Returns** | `dict[str, Any]` suitable for `environment=` |
| **Raises** | `FileNotFoundError` if path missing; `ValueError` if JSON invalid |

## Construction merge

```python
ctx = iv8_rs.JSContext(
    profile="default",           # or path
    environment={                 # wins over profile
        "timezone": "Asia/Shanghai",
        "navigator.userAgent": "...",
    },
    time_mode="system",
)
```

**Priority:** `environment` > `profile` > built-in defaults.

## Timezone (v0.8.102+)

| Mechanism | Role |
|---|---|
| `environment["timezone"]` | Sets process `TZ` and triggers V8 `TimeZoneDetection::Redetect` |
| ICU data | Required for `Intl` / `toLocaleString` (package `icudtl.dat`) |

Do not rely on JS monkey-patching `Intl.DateTimeFormat` for TZ.

## Related

- [overview.md](overview.md)  
- [runtime/jscontext.md](runtime/jscontext.md)  
