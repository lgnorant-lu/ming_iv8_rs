# ICU common data for V8 Intl

`icudtl.dat` — Chromium ICU common bundle (same class as ref iv8 / Chrome).

Loaded via `v8::icu::set_common_data_77` in `v8_init.rs` **before**
`V8::Initialize`. Required for `Intl.*` and `Date.toLocaleString`.

Do not delete. Refresh from Chromium/ref iv8 package when upgrading V8 major.
