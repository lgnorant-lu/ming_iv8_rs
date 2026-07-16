# D2 signature matrix — Tier A

> Metric: D2a (pyi vs Rust), D2b (docs vs pyi)  
> Legend: `Y` = matches SoT · `N` = mismatch · `P` = partial · `-` = n/a  
> Updated: 2026-07-16 (module-level + host raises pass)

## Module-level / exceptions

| Symbol | Rust/native SoT | pyi | docs | D2a | D2b | Notes |
|---|---|---|---|---|---|---|
| `enable_logging` | lib.rs | Y | module-level | Y | Y | levels listed |
| `instrument_source` | instrumentation.rs | Y | instrumentation + module-level | Y | Y | opts + Raises |
| `prepare_entry` | entry.rs | Y | entry + module-level | Y | Y | |
| `plan_multi_entry` | entry.rs | Y | entry + module-level | Y | Y | re-export OK |
| `run_with_entry` | entry.rs | Y | entry + module-level | Y | Y | chunks honesty |
| `trace_diff` | lib.rs | Y | module-level | Y | Y | |
| `load_profile` | __init__.py | - | profiles | - | Y | Python-only |
| `__version__` | native | Y | versioning | Y | Y | |
| `JSError` family | error.rs | Y | exceptions | Y | Y | hierarchy + host map |

## `JSContext` construction

| Item | Rust | pyi | docs | D2a | D2b |
|---|---|---|---|---|---|
| ctor kwargs | context.rs | Y (+worker_mode) | jscontext | Y | Y |
| `profile=` factory | __init__.py | note | jscontext | - | Y |
| host Raises section | - | - | jscontext + exceptions | - | Y |

## `JSContext` methods (50) — batch status

| Batch | pyi names | docs names | D2a | D2b | Raises in docs |
|---|---|---|---|---|---|
| Lifecycle / eval / page / network | Y | Y | Y | Y | Y (host table) |
| CDP / trace points | Y | Y | Y | Y | Y (with_devtools + RuntimeError) |
| VM instance | Y | Y | Y | Y | Y (path A) |
| Recording / profiler / coverage | Y | Y | Y | Y | Y (CDP prereq noted) |
| Storage / console / worker internal | Y | Y | Y | Y | Y |

## `Debugger` (10)

| Item | pyi | docs | D2a | D2b |
|---|---|---|---|---|
| method list + watch mode default | Y | Y | Y | Y |

## Overall D2

| Scope | Verdict |
|---|---|
| Module-level Tier A symbols | **PASS** (Y/Y) |
| JSContext ctor + host raises | **PASS** |
| All 50 methods full Raises prose | **PASS** |
| **Global D2 claim** | **PASS** (all batches Y after this pass) |

## How to promote remaining P → Y

1. Diff `#[pyo3(signature)]` defaults vs pyi for that method.  
2. Ensure docs table lists same names/defaults.  
3. Document Raises if interface can fail.  
4. Flip cell here; re-measure COVERAGE.  
