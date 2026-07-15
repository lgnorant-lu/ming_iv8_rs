# icudtl.dat (ICU 77 common package)

Required for V8 Intl. Must match rusty_v8 / V8 14.7 ICU **major 77**
(symbols `*_77`). Chrome 149 ships icudt78 — **incompatible**.

Loaded by `v8_init.rs` via `IV8_ICUDTL_PATH` (set in `python/iv8_rs/__init__.py`)
using `v8::icu::set_common_data_77` + `udata_setFileAccess_77(ONLY_PACKAGES)`.

Refresh: extract from Deno/Chromium build matching V8 14.7 ICU 77, or
rusty_v8 release assets when they publish `third_party/icu/common/icudtl.dat`.

Do not substitute random Chrome icudtl without checking `icudtNN` tag inside the file.
