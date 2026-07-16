# API documentation measurements

> **Metric definitions (SoT):** [`docs/conventions/api-documentation-conventions.md`](../conventions/api-documentation-conventions.md)  
> **Docstring format:** [`docs/conventions/docstring-conventions.md`](../conventions/docstring-conventions.md)  
> This file only records **numbers** against those metrics.

## Final measurement: 2026-07-16 (post Sphinx pipeline)

| Metric ID | Result | Pass? | Notes |
|---|---|---|---|
| **D1a** export name in `docs/api/**` | 144 / 144 | **PASS** | inventory script |
| **D1b** JSContext live methods in `jscontext.md` | 50 / 50 | **PASS** | |
| **D1c** Debugger live methods in `debugger.md` | 10 / 10 | **PASS** | |
| **D1d** live JSContext vs `_iv8.pyi` | 50 / 50 | **PASS** | stubs aligned |
| **D2a/b** Tier A signature matrix | all batches Y | **PASS** | [D2_TIER_A_MATRIX.md](D2_TIER_A_MATRIX.md) |
| **D3** Tier A checklist | all method groups Y | **PASS** | [D3_TIER_A_CHECKLIST.md](D3_TIER_A_CHECKLIST.md) |
| **D4** runnable doc examples | 10 smoke tests | **PASS** | `pytest tests/test_api_doc_examples.py` |
| **D6c** TIER_MAP partition | A=15 B=42 C=87 | **PASS** | no overlap / no stray |
| **Sphinx build** | 0 warnings, 0 errors | **PASS** | `uv run sphinx-build` |
| **D5** public symbol test coverage | 94/129 (72.9%) | **NOT PASS** | Tier C DTOs missing explicit tests |

## D5 detail

Missing tests are all Tier C: report DTOs, serde helpers, schema constants.
Most are tested indirectly through parent functions. Round-trip tests for
`*_from_dict` / `*_to_dict` pairs are planned but not yet written.

## Commands

```powershell
$env:RUST_MIN_STACK='134217728'
uv run python scripts/_api_doc_inventory.py
uv run python -m pytest tests/test_api_doc_examples.py -q
uv run maturin develop --target-dir D:\Caches\cargo-target --strip --profile dev
uv run sphinx-build -b html docs/source docs/source/_build/html
```

## Related

- [TIER_MAP.md](TIER_MAP.md)
- [D2_TIER_A_MATRIX.md](D2_TIER_A_MATRIX.md)
- [D3_TIER_A_CHECKLIST.md](D3_TIER_A_CHECKLIST.md)
- Conventions: `api-documentation-conventions.md`, `docstring-conventions.md`
