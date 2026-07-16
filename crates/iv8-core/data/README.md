# icudtl.dat — ICU 77 common data for V8 Intl

## Provenance (official-class)

| Field | Value |
|---|---|
| ICU major | **77** (must match rusty_v8 / V8 14.7 `*_77` symbols) |
| Source class | Chromium ICU common package (`icudt77`) |
| Obtained from | Deno CLI **v2.9.3** Windows x64 embed (same ICU major as rusty_v8 147) |
| Size | 10876560 bytes |
| NOT compatible | Chrome 149 `icudt78`, ref-iv8 package `icudt73` |

## Load path

1. Python: `IV8_ICUDTL_PATH` → this file (see `__init__.py`)
2. Rust: `v8::icu::set_common_data_77` + `udata_setFileAccess_77(ONLY_PACKAGES)`
3. Optional embed fallback: `crates/iv8-core/data/icudtl.dat`

## Refresh

When upgrading V8 major, re-extract ICU data matching that major from:
- Deno release for the same V8 line, or
- Chromium `third_party/icu/common/icudtl.dat` at the ICU revision used by that V8,
  **after verifying** the `icudtNN` tag inside the file matches `U_ICU_VERSION_MAJOR`.
