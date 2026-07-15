//! V8 platform initialization (process-global, once only).

use std::sync::Once;

static V8_INIT: Once = Once::new();

/// Minimum stack size (bytes) for V8 template creation.
///
/// V8 FunctionTemplate creation (1287 interfaces, 9223 members after mixin
/// merge) recurses deeply in C++ stack. The value is virtual memory with
/// lazy physical commit — actual RSS is far smaller.
const MIN_STACK_SIZE: &str = "1342177128";

/// Chromium/ICU common data (icudtl.dat). Required for Intl / Date locale.
///
/// Without this, V8 reports `TypeError: Internal error. Icu error` or a
/// fatal scavenger thrash mislabeled as OOM on `Intl.DateTimeFormat` /
/// `Date.toLocaleString`. Upstream iv8 ships the same file beside the
/// extension; Deno/Node embed ICU before `V8::Initialize`.
///
/// Source: Chromium ICU common bundle (aligned with ref iv8 package copy).
// Path relative to this source file: crates/iv8-core/src/ -> crates/iv8-core/data/
static ICU_DATA: &[u8] =
    align_data::include_aligned!(align_data::Align16, "../data/icudtl.dat");

/// Initialize V8 platform and engine. Safe to call multiple times —
/// only the first call has effect.
///
/// ## Stack size (K-010)
///
/// Three layers ensure sufficient stack for V8 template creation:
///
/// 1. **`.cargo/config.toml` `[env]`** — sets `RUST_MIN_STACK` for all
///    cargo commands (build, test, run). The Rust test harness reads this
///    when spawning test threads.
/// 2. **This function** — sets `RUST_MIN_STACK` if unset, as a safety net
///    for embedders who don't use cargo. Affects threads spawned after
///    this call (not the calling thread).
/// 3. **Python `__init__.py`** — calls `threading.stack_size(128MB)` at
///    module import time, before any JSContext creation.
///
/// ## ICU (K-ICU-DATA, 2026-07-16)
///
/// Call `v8::icu::set_common_data_77` **before** `V8::initialize`, matching
/// rusty_v8 test harness and production embeds. Do **not** wrap
/// `Intl.DateTimeFormat` in JS to fake timezone — that path re-enters and
/// fatals. Prefer process/ICU default TZ + optional
/// `Isolate::date_time_configuration_change_notification(Redetect)`.
///
/// External validation: Deno docs set `TZ` env for process zone; V8 API
/// `TimeZoneDetection::Redetect` reloads host zone after env change;
/// formatjs/Node issues show DTF construction is heavy — never polyfill
/// the constructor in the embed hot path.
///
/// ## V8 internal stack limit
///
/// V8's `--stack-size` flag controls the central-stack window (separate
/// from the OS thread stack). Set to 8MB (8192 KB) as a conservative
/// balance — 128MB caused a non-unwinding panic in WPT test runner
/// (PyEvent_IsSet thread crash). The OS thread stack (via RUST_MIN_STACK)
/// provides the actual recursion depth capacity.
///
/// ## Platform
///
/// Uses `new_default_platform(0, false)` (multi-threaded). V8's background
/// GC/compilation threads are enabled. This is required for Worker isolate
/// creation — without it, V8's shared ReadOnlyHeap GC crashes with
/// `IsOnCentralStack` when Worker isolate creates FunctionTemplates that
/// trigger GC.
///
/// See: docs/roadmap/v0.8/analysis/worker-execution-environment-design.md §9.8
pub fn ensure_v8_initialized() {
    V8_INIT.call_once(|| {
        if std::env::var("RUST_MIN_STACK").is_err() {
            // SAFETY: process-global stack hint for subsequent threads; only set if unset.
            unsafe {
                std::env::set_var("RUST_MIN_STACK", MIN_STACK_SIZE);
            }
        }

        // ICU (must precede V8::initialize) — dual strategy, exclusive:
        //
        // A) File path via V8::InitializeICU (Chromium/d8/ref-iv8): mmap
        //    icudtl.dat + udata_setCommonData + UDATA_ONLY_PACKAGES.
        //    Prefer when IV8_ICUDTL_PATH is set and FILE-mode ICU is linked.
        // B) Memory via set_common_data_77 (rusty_v8 test harness): embed
        //    aligned bytes. Used when no file path or A fails.
        //
        // Do NOT call both: double registration confuses ICU lookup.
        // Do NOT JS-wrap Intl.DateTimeFormat (re-entrancy fatal).
        // Locale: set_default_locale after successful data load only.
        //
        // Note: On ICU_UTIL_DATA_STATIC builds, InitializeICU(path) may
        // return true as a no-op — verify with a follow-up Intl probe in
        // tests; still register via set_common_data_77 if needed.
        // Single registration path (avoid double udata_setCommonData):
        // Prefer IV8_ICUDTL_PATH file bytes → set_common_data_77 +
        // udata_setFileAccess(ONLY_PACKAGES). Fall back to embedded.
        // Do not call V8::InitializeICU when using set_common_data_77 — on
        // STATIC ICU builds it is a no-op; on FILE builds it would re-register.
        let mut icu_ok = false;
        let mut data_src = "none";
        if let Ok(p) = std::env::var("IV8_ICUDTL_PATH") {
            let path = std::path::Path::new(&p);
            if path.is_file() {
                if let Ok(bytes) = std::fs::read(path) {
                    let leaked: &'static [u8] = Box::leak(bytes.into_boxed_slice());
                    eprintln!(
                        "iv8-rs: file ICU {} bytes={} align8={}",
                        p,
                        leaked.len(),
                        (leaked.as_ptr() as usize) % 8 == 0
                    );
                    match v8::icu::set_common_data_77(leaked) {
                        Ok(()) => {
                            icu_ok = true;
                            data_src = "file";
                            eprintln!("iv8-rs: set_common_data_77(file) OK");
                        }
                        Err(code) => {
                            eprintln!("iv8-rs: set_common_data_77(file) FAIL code={code}");
                        }
                    }
                }
            } else {
                eprintln!("iv8-rs: IV8_ICUDTL_PATH not a file: {p}");
            }
        }
        if !icu_ok {
            eprintln!(
                "iv8-rs: embedded set_common_data_77 bytes={} align_mod16={}",
                ICU_DATA.len(),
                (ICU_DATA.as_ptr() as usize) % 16
            );
            match v8::icu::set_common_data_77(ICU_DATA) {
                Ok(()) => {
                    icu_ok = true;
                    data_src = "embedded";
                    eprintln!("iv8-rs: set_common_data_77(embedded) OK");
                }
                Err(code) => {
                    eprintln!(
                        "iv8-rs: set_common_data_77(embedded) FAIL code={code} len={}",
                        ICU_DATA.len()
                    );
                }
            }
        }
        if icu_ok {
            let fa = crate::v8_extra::icu_set_file_access_only_packages();
            eprintln!(
                "iv8-rs: ICU ready src={data_src} setFileAccess(ONLY_PACKAGES)={fa}"
            );
            v8::icu::set_default_locale("zh-CN");
        } else {
            crate::telemetry::v8_fatal_error(
                "v8_init.rs",
                line!() as i32,
                "ICU data not loaded; Intl/Date.toLocaleString will fail",
            );
        }

        v8::V8::set_flags_from_string("--stack-size=8192");

        let platform = v8::new_default_platform(0, false).make_shared();
        v8::V8::initialize_platform(platform);
        v8::V8::initialize();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ensure_v8_initialized_is_idempotent() {
        ensure_v8_initialized();
        ensure_v8_initialized();
        ensure_v8_initialized();
    }

    #[test]
    fn test_rust_min_stack_is_set_after_init() {
        ensure_v8_initialized();
        let val = std::env::var("RUST_MIN_STACK").expect("RUST_MIN_STACK must be set");
        assert!(
            val.parse::<usize>().unwrap() >= 134_217_728,
            "RUST_MIN_STACK must be >= 128MB, got {}",
            val
        );
    }
}
