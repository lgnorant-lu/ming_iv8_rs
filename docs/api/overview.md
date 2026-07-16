# Product overview & runtime bounds

## What iv8-rs is

A **high-fidelity browser-like JS runtime** exposed to Python (V8 + Rust + PyO3), aimed at:

- Web JS reverse engineering and controlled re-execution  
- Anti-bot / fingerprint **host** simulation  
- Offline multi-bundler entry and instrumentation  

It is **not** a full Chromium product, not a general web scraper framework, and not a one-click “bypass” package.

## What it is not (honest bounds)

| Claim to avoid | Reality |
|---|---|
| “Full Chrome” | Large WebIDL surface + intentional stubs/bounds |
| “Auto fetch all webpack chunks” | Bundler bridge is **offline-first**; caller supplies chunk text (Q163 honesty) |
| “Silent network product” | Default ResourceBundle → optional Python handler → error |
| “Environment toolchain applies fixes automatically” | **Report-only / no-write** by governance unless explicitly authorized elsewhere |
| “Same as package `iv8` 0.1.x” | Related lineage; **iv8-rs** is this repo’s product; dual-engine is oracle, not identity |

## Process / thread / stack

| Topic | Contract |
|---|---|
| **Thread** | A `JSContext` is bound to the creating thread (V8 isolate). Do not move across threads. |
| **Stack** | Creating a full kernel needs a large stack. Module import sets `threading.stack_size(128 * 1024 * 1024)`. Prefer creating contexts on such threads. |
| **Concurrent kernels** | Multiple full `JSContext` on different threads may serialize/init-gate (`K-ISOLATE-INIT-SERIAL`). Close unused contexts. |
| **ICU** | Intl/locale requires ICU 77 data (`icudtl.dat` beside package / `IV8_ICUDTL_PATH`). |

## Environment priority

When constructing a context:

```text
explicit environment= dict  >  profile= file/dict  >  built-in defaults
```

See [profiles.md](profiles.md).

## Time modes

| Mode | Behavior (summary) |
|---|---|
| `system` | Wall clock / real timers (subject to host) |
| `logical` / frozen patterns | Deterministic test / replay (see GUIDE + JSContext ctor) |

Exact flags: [runtime/jscontext.md](runtime/jscontext.md).

## Documentation split

| Doc | Role |
|---|---|
| This overview | Bounds and global contracts |
| [runtime/jscontext.md](runtime/jscontext.md) | Full method surface |
| GUIDE | Long-form tutorials |
| CHANGELOG | Per-version deltas |
