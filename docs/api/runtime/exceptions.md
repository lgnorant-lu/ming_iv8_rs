# Exception types

Exported from `iv8_rs` (native). Hierarchy:

```text
BaseException
└── Exception
    └── JSError
        ├── JSCompileError
        ├── JSTimeoutError
        ├── JSMemoryError
        └── JSPanic
```

| Type | Typical cause | Catch when |
|---|---|---|
| `JSError` | JS runtime throw / general evaluation failure | Broad JS failure handling |
| `JSCompileError` | Syntax / compile failure | Invalid source |
| `JSTimeoutError` | Evaluation exceeded time budget (when configured) | Timeouts |
| `JSMemoryError` | Memory limit / OOM class failures | Resource exhaustion |
| `JSPanic` | Internal Rust panic surfaced across FFI | Should be rare; treat as host bug |

## Host / lifecycle errors (not always `JSError`)

| Condition | Typical Python exception | Notes |
|---|---|---|
| Context already closed | `RuntimeError` (message contains `closed`) | After `close()` / context manager exit |
| Wrong thread | `RuntimeError` (creator-thread message) | V8 isolate affinity |
| Invalid `time_mode` | `ValueError` | Ctor only: expect `logical` \| `system` |
| Invalid entry `persona` | `ValueError` | `prepare_entry` / `plan_multi_entry` |
| Profile missing / bad JSON | `FileNotFoundError` / `ValueError` | `load_profile` |
| No VM dispatch detected | `RuntimeError` | `instrument_source` without manual overrides |

## Contract notes

- Prefer catching **specific subclasses** when branching recovery.
- Message strings are **diagnostic**; do not treat wording as a stable wire protocol.
- Always check `is_disposed()` before reusing a context across uncertain control flow.

## Related

- [jscontext.md](jscontext.md)  
- [module-level.md](module-level.md)  
