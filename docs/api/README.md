# iv8-rs API Documentation

> Status: **living contract** (Tier A host contracts strong; Tier B/C catalog + selected signatures)  
> Package: **0.8.12** · Milestone continuum through **v0.8.102**  
> Principle: **complete surface, clear layers** — not a single mega-file, not a dumbed-down subset.

## How to read (dual track — intentional)

| Track | Artifact | Use for |
|---|---|---|
| **Contracts (read first)** | **This tree** (`docs/api/`) | Stable calling contracts, host bounds, offline/network honesty, tiers |
| **Generated reference** | Sphinx (`docs/source/` → local `sphinx-build`) | Browsable pages from live `__doc__` after `maturin develop` |
| **Types** | `python/iv8_rs/_iv8.pyi` | IDE / typecheck shape |
| **Tutorials (public)** | [../GUIDE.public.md](../GUIDE.public.md) | Public cut §1–16 |
| **Tutorials (full / private-oriented)** | [../GUIDE.md](../GUIDE.md) | Includes version waterfall; not on public keep list |

Sphinx HTML under `docs/source/_build/` is **generated** (gitignored). Do not treat build output as SoT.

| If you need… | Go to |
|---|---|
| Install + 30-second mental model | root [README](../../README.md), [../GUIDE.public.md](../GUIDE.public.md) |
| **Stable calling contracts** | This tree |
| Version-to-version deltas | [../../CHANGELOG.md](../../CHANGELOG.md) |
| Quality gates / matrices | [../quality-harness/](../quality-harness/) |
| Toolchain choice (Sphinx vs MkDocs) | [../conventions/docs-toolchain-selection.md](../conventions/docs-toolchain-selection.md) |

## Layer map

```text
┌─────────────────────────────────────────────────────────┐
│  L0  Native runtime (_iv8)                              │
│      JSContext · Debugger · exceptions · instrument_…   │
├─────────────────────────────────────────────────────────┤
│  L1  Runtime Python helpers                             │
│      load_profile · environment merge · thin wrappers   │
├─────────────────────────────────────────────────────────┤
│  L2  Observability & analysis                           │
│      trace · cfg · taint · patterns · vm_diff · probe   │
├─────────────────────────────────────────────────────────┤
│  L3  Entry / corpus                                     │
│      prepare_entry · run_with_entry · corpus runner     │
├─────────────────────────────────────────────────────────┤
│  L4  Environment toolchain (diagnostic plane)           │
│      report-only · no silent apply · not “bypass kit”   │
└─────────────────────────────────────────────────────────┘
```

## Index

### Product overview

| Doc | Contents |
|---|---|
| [overview.md](overview.md) | Product bounds, threads/stack, ICU, time modes, environment priority |
| [versioning.md](versioning.md) | D-151 dual-track (milestone tag vs package version) |

### L0–L1 Runtime

| Doc | Contents |
|---|---|
| [runtime/README.md](runtime/README.md) | Runtime index |
| [runtime/jscontext.md](runtime/jscontext.md) | **Full** `JSContext` method surface (grouped) |
| [runtime/exceptions.md](runtime/exceptions.md) | `JSError` family |
| [runtime/debugger.md](runtime/debugger.md) | `Debugger` class |
| [runtime/module-level.md](runtime/module-level.md) | Module-level functions (`instrument_source`, entry, logging, …) |
| [profiles.md](profiles.md) | `load_profile` / `environment` / defaults |

### L2–L3 Analysis & entry

| Doc | Contents |
|---|---|
| [instrumentation/README.md](instrumentation/README.md) | ChaosVM / `instrument_source` contracts |
| [entry/README.md](entry/README.md) | Multi-entry / bundler offline contracts |
| [analysis/README.md](analysis/README.md) | Trace, CFG, taint, patterns, VM diff |
| [analysis/signatures.md](analysis/signatures.md) | Tier B analysis signatures |
| [environment/signatures.md](environment/signatures.md) | Tier B environment signatures |

### L4 Environment toolchain

| Doc | Contents |
|---|---|
| [environment/README.md](environment/README.md) | Plane + policy entry |
| [environment/toolchain.md](environment/toolchain.md) | Toolchain reports (diagnostic-only) |
| [environment/pressure.md](environment/pressure.md) | Pressure harness bounds |
| [reports/README.md](reports/README.md) | Experimental report models |

### Meta

| Doc | Contents |
|---|---|
| [migration.md](migration.md) | Mapping from GUIDE sections; breaking-change notes |
| [TIER_MAP.md](TIER_MAP.md) | Tier A/B/C classification of all `__all__` exports |
| [COVERAGE.md](COVERAGE.md) | Measured metrics D1–D6 (not feelings) |
| [D2_TIER_A_MATRIX.md](D2_TIER_A_MATRIX.md) | Signature fidelity matrix (D2) |
| [D3_TIER_A_CHECKLIST.md](D3_TIER_A_CHECKLIST.md) | Semantic field checklist (D3) |

## Completeness policy

- Metrics and tiers: [`docs/conventions/api-documentation-conventions.md`](../conventions/api-documentation-conventions.md).
- **Do not omit** a public export from the index because it is “advanced”.
- **Do** put advanced material in the correct layer page, with honest bounds.
- **Do not** paste entire Environment Toolchain source into one page; link modules and document **contracts**.
- Sample adapters under `docs/samples/adapters/` are **not** product API (see that tree’s README).
- Inventory gate: `uv run python scripts/_api_doc_inventory.py` · examples: `pytest tests/test_api_doc_examples.py`.

## Related private docs (not part of public keep set)

Structure discussion: `docs/roadmap/v0.8/analysis/readme-api-structure-design.md`.
