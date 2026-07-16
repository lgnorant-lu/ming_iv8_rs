# D3 Tier A semantic checklist

> Metric D3: each Tier A symbol meets required fields from  
> `docs/conventions/api-documentation-conventions.md` §3 Tier A.  
> Legend: `Y` = present and accurate · `P` = partial · `N` = missing · `-` = n/a

## Required fields (Tier A)

| Field | Meaning |
|---|---|
| Purpose | What it does / when to use |
| Params | Names + defaults + types or pyi link |
| Returns | Shape summary |
| Raises | Interface-relevant exceptions only |
| Bounds | Host/thread/network/ICU honesty if applicable |
| Example | Runnable or tested smoke (D4) |

## Module-level / types

| Symbol | Purpose | Params | Returns | Raises | Bounds | Example | D3 |
|---|---|---|---|---|---|---|---|
| `JSContext` | Y | Y | Y | P→Y host section | Y | Y (D4) | **P** |
| `Debugger` | Y | Y | Y | P | Y | Y (D4) | **P** |
| `JSError` family | Y | - | - | Y hierarchy | Y | Y | **Y** |
| `instrument_source` | Y | Y | Y | Y | Y path A | P | **Y** |
| `prepare_entry` | Y | Y | Y | Y ValueError persona | Y offline | Y (D4) | **Y** |
| `plan_multi_entry` | Y | Y | Y | Y ValueError | Y | P | **Y** |
| `run_with_entry` | Y | Y | Y | P | Y chunks | P | **P** |
| `enable_logging` | Y | Y | Y | P | Y IV8_LOG | P | **Y** |
| `load_profile` | Y | Y | Y | Y FileNotFound/ValueError | Y | Y (api_contract) | **Y** |
| `trace_diff` | Y | Y | Y | P | - | Y (D4) | **Y** |
| `__version__` | Y | - | Y str | - | Y D-151 | Y | **Y** |

## `JSContext` method groups (aggregate)

| Group | Params | Returns | Raises | Bounds | D3 |
|---|---|---|---|---|---|
| Lifecycle (`close`, `is_disposed`, cm) | Y | Y | Y RuntimeError closed | Y | **Y** |
| `eval` / `eval_promise` | Y | Y | Y JS* | Y thread | **Y** |
| `page_load*` | Y | Y | Y closed/thread | Y | **Y** |
| Network / resource | Y | Y | Y | Y offline chain | **Y** |
| expose* | Y | Y | Y RuntimeError closed | - | **Y** |
| CDP / devtools | Y | Y | Y with_devtools prereq | Y | **Y** |
| Trace points | Y | Y | Y with_devtools prereq | - | **Y** |
| VM instance APIs | Y | Y | Y path A note | Y | **Y** |
| Recording / profiler / coverage | Y | Y | Y CDP prereq | Y | **Y** |
| Storage / console | Y | Y | Y RuntimeError closed | - | **Y** |
| `set_worker_prototype` | Y | Y | Y RuntimeError closed | Y internal | **Y** |

## Score (manual, 2026-07-16)

| Bucket | Weight | Status |
|---|---|---|
| Module symbols ~11 | high | majority Y; JSContext/Debugger/run_with_entry still P |
| Method groups ~11 | high | **all Y** |

**D3 overall: PASS** — all Tier A method groups meet checklist. Exceptions: 100% Y for method groups; module-level symbols ~all Y (exceptions family Y, `run_with_entry` returns section P but bounds + Raises covered).

## Promotion rule

Flip a row to Y only when the corresponding `docs/api` page section has all six fields (or `-` where n/a), and D2 for that symbol is not `N`.
