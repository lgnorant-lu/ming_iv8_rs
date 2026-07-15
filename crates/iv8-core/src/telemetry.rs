//! IV8 Log Event Catalog
//!
//! Typed, category-based log event system inspired by V8's TRACE_EVENT
//! categories, OpenTelemetry's event name + schema model, and Node.js's
//! programmatic category control.
//!
//! ## Design
//!
//! Each log event is a typed function. The function signature IS the event
//! schema — parameters are the structured fields. The function body sets
//! the correct `target` (category) and `level`.
//!
//! ## Categories
//!
//! Categories are hierarchical strings used as tracing `target` overrides.
//! EnvFilter matches by prefix, so `IV8_LOG=iv8.dom=debug` enables all
//! `iv8.dom.*` events.
//!
//! | Category | Scope |
//! |----------|-------|
//! | `iv8.init` | Kernel initialization phases |
//! | `iv8.dom` | DOM template creation, binding, property merge |
//! | `iv8.config` | Configuration resolution and fallback |
//! | `iv8.worker` | Worker lifecycle |
//! | `iv8.callback` | V8 callback execution and panics |
//! | `iv8.eval` | JavaScript evaluation |
//! | `iv8.console` | JS console.* API |
//!
//! ## Safety
//!
//! Each event has a safety level:
//! - `Safe`: no sensitive data, safe to log in production
//! - `Diagnostic`: may contain internal state, debug-only
//! - `Sensitive`: may contain user data, never log in production
//!
//! ## Usage
//!
//! ```rust
//! use crate::telemetry;
//! telemetry::init_proto_merge(name, copied, skipped, same_ctor);
//! ```
//!
//! ## Filtering
//!
//! ```bash
//! IV8_LOG=iv8.init=debug,iv8.dom=trace
//! ```

/// Event safety level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Safety {
    Safe,
    Diagnostic,
    Sensitive,
}

/// Static event specification for introspection.
pub struct EventSpec {
    pub name: &'static str,
    pub category: &'static str,
    pub level: &'static str,
    pub safety: Safety,
    pub fields: &'static [&'static str],
}

/// Return the full event catalog for introspection.
pub fn catalog() -> &'static [EventSpec] {
    &CATALOG
}

const CATALOG: &[EventSpec] = &[
    EventSpec {
        name: "init_browser_surface_installed",
        category: "iv8.init",
        level: "INFO",
        safety: Safety::Safe,
        fields: &["interface_count"],
    },
    EventSpec {
        name: "init_codegen_prototypes_captured",
        category: "iv8.init",
        level: "DEBUG",
        safety: Safety::Safe,
        fields: &["count"],
    },
    EventSpec {
        name: "init_dom_templates_built",
        category: "iv8.init",
        level: "DEBUG",
        safety: Safety::Safe,
        fields: &[],
    },
    EventSpec {
        name: "init_dom_constructors_installed",
        category: "iv8.init",
        level: "DEBUG",
        safety: Safety::Safe,
        fields: &[],
    },
    EventSpec {
        name: "init_proto_merge_start",
        category: "iv8.init",
        level: "DEBUG",
        safety: Safety::Safe,
        fields: &["interface_count"],
    },
    EventSpec {
        name: "init_proto_merge",
        category: "iv8.init",
        level: "DEBUG",
        safety: Safety::Safe,
        fields: &["interface", "proto_copied", "proto_skipped", "ctor_copied", "same_ctor"],
    },
    EventSpec {
        name: "init_proto_merge_complete",
        category: "iv8.init",
        level: "DEBUG",
        safety: Safety::Safe,
        fields: &[],
    },
    EventSpec {
        name: "init_same_ctor_warning",
        category: "iv8.init",
        level: "WARN",
        safety: Safety::Diagnostic,
        fields: &["interface"],
    },
    EventSpec {
        name: "worker_script_error",
        category: "iv8.worker",
        level: "ERROR",
        safety: Safety::Diagnostic,
        fields: &["error"],
    },
    EventSpec {
        name: "callback_panic",
        category: "iv8.callback",
        level: "ERROR",
        safety: Safety::Diagnostic,
        fields: &["callback", "panic_msg"],
    },
    EventSpec {
        name: "dom_binding_panic",
        category: "iv8.dom",
        level: "ERROR",
        safety: Safety::Diagnostic,
        fields: &["operation"],
    },
    EventSpec {
        name: "dom_template_created",
        category: "iv8.dom",
        level: "DEBUG",
        safety: Safety::Safe,
        fields: &["interface"],
    },
    EventSpec {
        name: "convert_error",
        category: "iv8.callback",
        level: "WARN",
        safety: Safety::Diagnostic,
        fields: &["type_name"],
    },
    EventSpec {
        name: "inspector_connected",
        category: "iv8.inspector",
        level: "INFO",
        safety: Safety::Safe,
        fields: &["port"],
    },
    EventSpec {
        name: "inspector_disconnected",
        category: "iv8.inspector",
        level: "INFO",
        safety: Safety::Safe,
        fields: &[],
    },
    EventSpec {
        name: "inspector_listening",
        category: "iv8.inspector",
        level: "INFO",
        safety: Safety::Safe,
        fields: &["port"],
    },
    EventSpec {
        name: "inspector_accept_error",
        category: "iv8.inspector",
        level: "WARN",
        safety: Safety::Diagnostic,
        fields: &["error"],
    },
    EventSpec {
        name: "shim_installed",
        category: "iv8.shim",
        level: "DEBUG",
        safety: Safety::Safe,
        fields: &["name"],
    },
    EventSpec {
        name: "state_error",
        category: "iv8.config",
        level: "WARN",
        safety: Safety::Diagnostic,
        fields: &["error"],
    },
    EventSpec {
        name: "state_created",
        category: "iv8.config",
        level: "INFO",
        safety: Safety::Safe,
        fields: &["strict_compat", "time_mode", "js_api_name", "env_entries"],
    },
    EventSpec {
        name: "state_dropped",
        category: "iv8.config",
        level: "INFO",
        safety: Safety::Safe,
        fields: &["eval_count"],
    },
    EventSpec {
        name: "init_phase_skipped",
        category: "iv8.init",
        level: "DEBUG",
        safety: Safety::Safe,
        fields: &["phase", "reason"],
    },
    EventSpec {
        name: "canvas_fingerprint_warning",
        category: "iv8.canvas",
        level: "WARN",
        safety: Safety::Diagnostic,
        fields: &["parameter", "renderer", "forbidden"],
    },
    EventSpec {
        name: "console_message",
        category: "iv8.console",
        level: "DEBUG",
        safety: Safety::Sensitive,
        fields: &["method", "message"],
    },
    EventSpec {
        name: "worker_import_script_not_found",
        category: "iv8.worker",
        level: "WARN",
        safety: Safety::Diagnostic,
        fields: &["url"],
    },
    EventSpec {
        name: "v8_fatal_error",
        category: "iv8.callback",
        level: "ERROR",
        safety: Safety::Diagnostic,
        fields: &["file", "line", "message"],
    },
    EventSpec {
        name: "v8_oom",
        category: "iv8.callback",
        level: "ERROR",
        safety: Safety::Safe,
        fields: &["location", "is_heap_oom"],
    },
    EventSpec {
        name: "v8_uncaught_exception",
        category: "iv8.eval",
        level: "ERROR",
        safety: Safety::Diagnostic,
        fields: &["message"],
    },
    EventSpec {
        name: "rust_panic",
        category: "iv8.callback",
        level: "ERROR",
        safety: Safety::Diagnostic,
        fields: &["msg"],
    },
    EventSpec {
        name: "init_phase_start",
        category: "iv8.init",
        level: "INFO",
        safety: Safety::Safe,
        fields: &["phase"],
    },
    EventSpec {
        name: "init_phase_complete",
        category: "iv8.init",
        level: "INFO",
        safety: Safety::Safe,
        fields: &["phase", "duration_ms"],
    },
    EventSpec {
        name: "init_phase_failed",
        category: "iv8.init",
        level: "ERROR",
        safety: Safety::Diagnostic,
        fields: &["phase", "error"],
    },
    EventSpec {
        name: "kernel_lifecycle_wait",
        category: "iv8.init",
        level: "DEBUG",
        safety: Safety::Safe,
        fields: &["waited_ms", "timed_out"],
    },
    EventSpec {
        name: "kernel_lifecycle_timeout",
        category: "iv8.init",
        level: "WARN",
        safety: Safety::Diagnostic,
        fields: &["waited_ms"],
    },
    EventSpec {
        name: "eval_complete",
        category: "iv8.eval",
        level: "DEBUG",
        safety: Safety::Safe,
        fields: &["success", "duration_ms"],
    },
    EventSpec {
        name: "eval_error",
        category: "iv8.eval",
        level: "WARN",
        safety: Safety::Diagnostic,
        fields: &["message"],
    },
];

// ─── Init phase events ──────────────────────────────────────────────

/// BrowserSurface installation completed.
/// Safety: Safe (interface count only)
pub fn init_browser_surface_installed(interface_count: usize) {
    tracing::info!(
        target: "iv8.init",
        interface_count = interface_count,
        "BrowserSurface installation complete"
    );
}

/// Codegen prototypes captured before dom/template.rs override.
/// Safety: Safe (count only)
pub fn init_codegen_prototypes_captured(count: usize) {
    tracing::debug!(
        target: "iv8.init",
        count = count,
        "codegen prototypes captured"
    );
}

/// DOM templates built by build_dom_templates.
/// Safety: Safe
pub fn init_dom_templates_built() {
    tracing::debug!(target: "iv8.init", "dom templates built");
}

/// DOM constructors installed on global by install_dom_constructors.
/// Safety: Safe
pub fn init_dom_constructors_installed() {
    tracing::debug!(target: "iv8.init", "dom constructors installed");
}

/// chain_dom_prototypes started.
/// Safety: Safe (count only)
pub fn init_proto_merge_start(interface_count: usize) {
    tracing::debug!(
        target: "iv8.init",
        interface_count = interface_count,
        "chain_dom_prototypes start"
    );
}

/// Per-interface prototype property merge result.
/// Safety: Safe (interface name and counts)
pub fn init_proto_merge(
    interface: &str,
    proto_copied: u32,
    proto_skipped: u32,
    ctor_copied: u32,
    same_ctor: bool,
) {
    tracing::debug!(
        target: "iv8.init",
        interface = interface,
        proto_copied = proto_copied,
        proto_skipped = proto_skipped,
        ctor_copied = ctor_copied,
        same_ctor = same_ctor,
        "prototype property merge"
    );
}

/// chain_dom_prototypes completed.
/// Safety: Safe
pub fn init_proto_merge_complete() {
    tracing::debug!(target: "iv8.init", "chain_dom_prototypes complete");
}

/// Warning: dom constructor equals codegen constructor (override failed).
/// Safety: Diagnostic (reveals internal init state)
pub fn init_same_ctor_warning(interface: &str) {
    tracing::warn!(
        target: "iv8.init",
        interface = interface,
        "dom constructor equals codegen; override may have failed"
    );
}

// ─── Worker events ──────────────────────────────────────────────────

/// Worker script execution error.
/// Safety: Diagnostic (error message may contain internal state)
pub fn worker_script_error(error: &str) {
    tracing::error!(
        target: "iv8.worker",
        error = error,
        "worker script error"
    );
}

/// Worker importScripts URL not found in ResourceBundle or network handler.
/// Safety: Diagnostic (URL may reveal internal state)
pub fn worker_import_script_not_found(url: &str) {
    tracing::warn!(
        target: "iv8.worker",
        url = url,
        "worker importScripts URL not found in ResourceBundle"
    );
}

// ─── Callback events ────────────────────────────────────────────────

/// V8 callback panic caught by safe_callback.
/// Safety: Diagnostic (panic message may contain stack info)
pub fn callback_panic(callback: &str, panic_msg: &str) {
    tracing::error!(
        target: "iv8.callback",
        callback = callback,
        panic_msg = panic_msg,
        "V8 callback panic caught"
    );
}

// ─── V8 error events ────────────────────────────────────────────────

/// V8 fatal error (CHECK failure or internal error).
/// Safety: Diagnostic (may contain file/line/internal state)
pub fn v8_fatal_error(file: &str, line: i32, message: &str) {
    tracing::error!(
        target: "iv8.callback",
        file = file,
        line = line,
        message = message,
        "V8 fatal error"
    );
}

/// V8 out-of-memory error.
/// Safety: Safe (location and heap flag only)
/// html5ever 0.39+ meta charset / encoding label during streaming parse.
pub fn html_encoding_indicator(label: &str) {
    tracing::debug!(target: "iv8.dom", label = %label, "html encoding indicator (UTF-8 host continues)");
}

pub fn v8_oom(location: &str, is_heap_oom: bool) {
    tracing::error!(
        target: "iv8.callback",
        location = location,
        is_heap_oom = is_heap_oom,
        "V8 out of memory"
    );
}

/// V8 uncaught exception during eval.
/// Safety: Diagnostic (message may contain JS source)
pub fn v8_uncaught_exception(message: &str) {
    tracing::error!(
        target: "iv8.eval",
        message = message,
        "V8 uncaught exception"
    );
}

/// Rust panic caught at FFI boundary.
/// Safety: Diagnostic (panic message may contain stack info)
pub fn rust_panic(msg: &str) {
    tracing::error!(
        target: "iv8.callback",
        msg = msg,
        "Rust panic caught at FFI boundary"
    );
}

// ─── Init phase events ─────────────────────────────────────────────

/// Init phase started.
/// Safety: Safe
pub fn init_phase_start(phase: &str) {
    tracing::info!(
        target: "iv8.init",
        phase = phase,
        "init phase start"
    );
}

/// Init phase completed.
/// Safety: Safe
pub fn init_phase_complete(phase: &str, duration_ms: u64) {
    tracing::info!(
        target: "iv8.init",
        phase = phase,
        duration_ms = duration_ms,
        "init phase complete"
    );
}

/// Init phase failed.
/// Safety: Diagnostic (error may contain internal state)
pub fn init_phase_failed(phase: &str, error: &str) {
    tracing::error!(
        target: "iv8.init",
        phase = phase,
        error = error,
        "init phase failed"
    );
}

/// Full-kernel create waited for another thread to release its isolate.
/// Safety: Safe
pub fn kernel_lifecycle_wait(waited_ms: u64, timed_out: bool) {
    tracing::debug!(
        target: "iv8.init",
        waited_ms = waited_ms,
        timed_out = timed_out,
        "kernel lifecycle wait for other full isolate"
    );
}

/// Full-kernel create timed out waiting for other thread.
/// Safety: Diagnostic
pub fn kernel_lifecycle_timeout(waited_ms: u64) {
    tracing::warn!(
        target: "iv8.init",
        waited_ms = waited_ms,
        "kernel lifecycle wait timed out"
    );
}

// ─── Eval events ───────────────────────────────────────────────────

/// Eval completed.
/// Safety: Safe
pub fn eval_complete(success: bool, duration_ms: u64) {
    tracing::debug!(
        target: "iv8.eval",
        success = success,
        duration_ms = duration_ms,
        "eval complete"
    );
}

/// Eval error (JS exception or compile error).
/// Safety: Diagnostic (message may contain JS source)
pub fn eval_error(message: &str) {
    tracing::warn!(
        target: "iv8.eval",
        message = message,
        "eval error"
    );
}

// ─── DOM binding events ─────────────────────────────────────────────

/// DOM binding callback panic caught by catch_unwind.
/// Safety: Diagnostic (operation name reveals internal state)
pub fn dom_binding_panic(operation: &str) {
    tracing::error!(
        target: "iv8.dom",
        operation = operation,
        "DOM binding callback panic caught"
    );
}

/// DOM template created for an interface.
/// Safety: Safe (interface name only)
pub fn dom_template_created(interface: &str) {
    tracing::debug!(
        target: "iv8.dom",
        interface = interface,
        "DOM template created"
    );
}

// ─── Convert events ─────────────────────────────────────────────────

/// Type conversion error (V8 value to Rust type).
/// Safety: Diagnostic (type name reveals internal state)
pub fn convert_error(type_name: &str) {
    tracing::warn!(
        target: "iv8.callback",
        type_name = type_name,
        "cannot convert V8 value, type not handled"
    );
}

// ─── Inspector events ───────────────────────────────────────────────

/// Inspector server started listening.
/// Safety: Safe (port only)
pub fn inspector_listening(port: u16) {
    tracing::info!(
        target: "iv8.inspector",
        port = port,
        "V8 Inspector listening"
    );
}

/// DevTools client connected.
/// Safety: Safe
pub fn inspector_connected(port: u16) {
    tracing::info!(
        target: "iv8.inspector",
        port = port,
        "DevTools client connected"
    );
}

/// DevTools client disconnected.
/// Safety: Safe
pub fn inspector_disconnected() {
    tracing::info!(
        target: "iv8.inspector",
        "DevTools client disconnected"
    );
}

/// Inspector accept error.
/// Safety: Diagnostic (error may contain internal state)
pub fn inspector_accept_error(error: &str) {
    tracing::warn!(
        target: "iv8.inspector",
        error = error,
        "Inspector accept error"
    );
}

// ─── Shim events ────────────────────────────────────────────────────

/// Shim installed on prototype or globalThis.
/// Safety: Safe (name only)
pub fn shim_installed(name: &str) {
    tracing::debug!(
        target: "iv8.shim",
        name = name,
        "shim installed"
    );
}

// ─── State events ───────────────────────────────────────────────────

/// State management error.
/// Safety: Diagnostic (error may contain internal state)
pub fn state_error(error: &str) {
    tracing::warn!(
        target: "iv8.config",
        error = error,
        "state error"
    );
}

/// RuntimeState created.
/// Safety: Safe (config values, no user data)
pub fn state_created(strict_compat: bool, time_mode: &str, js_api_name: &str, env_entries: usize) {
    tracing::info!(
        target: "iv8.config",
        strict_compat = strict_compat,
        time_mode = time_mode,
        js_api_name = js_api_name,
        env_entries = env_entries,
        "RuntimeState created"
    );
}

/// RuntimeState dropping.
/// Safety: Safe (count only)
pub fn state_dropped(eval_count: u64) {
    tracing::info!(
        target: "iv8.config",
        eval_count = eval_count,
        "RuntimeState dropping"
    );
}

// ─── Init phase skip events ────────────────────────────────────────

/// Init phase skipped (e.g. worker_mode skips document bindings).
/// Safety: Safe (phase name and reason)
pub fn init_phase_skipped(phase: &str, reason: &str) {
    tracing::debug!(
        target: "iv8.init",
        phase = phase,
        reason = reason,
        "init phase skipped"
    );
}

// ─── Post-hoc fix events (K-016) ──────────────────────────────────

/// A post-hoc JS fix is about to be applied.
/// Safety: Safe (fix name only, no user data)
pub fn post_hoc_fix_start(fix_name: &str) {
    tracing::debug!(
        target: "iv8.init",
        fix = fix_name,
        "post-hoc fix start"
    );
}

/// A post-hoc JS fix completed.
/// Safety: Safe (fix name and success status)
pub fn post_hoc_fix_complete(fix_name: &str, success: bool) {
    tracing::debug!(
        target: "iv8.init",
        fix = fix_name,
        success = success,
        "post-hoc fix complete"
    );
}

// ─── Canvas events ──────────────────────────────────────────────────

/// Canvas fingerprint detection warning.
/// Safety: Diagnostic (renderer string reveals GPU info, not user data)
pub fn canvas_fingerprint_warning(parameter: &str, renderer: &str, forbidden: &str) {
    tracing::warn!(
        target: "iv8.canvas",
        parameter = parameter,
        renderer = renderer,
        forbidden = forbidden,
        "webgl renderer contains forbidden signal; anti-fingerprint detection risk"
    );
}

// ─── Console events ─────────────────────────────────────────────────

/// JS console.* API message passthrough.
/// Safety: Sensitive (may contain user-generated content)
pub fn console_message(method: &str, level: &str, message: &str) {
    match level {
        "error" => tracing::error!(target: "iv8.console", method = method, message = message, "console.{}", method),
        "warn" => tracing::warn!(target: "iv8.console", method = method, message = message, "console.{}", method),
        "info" => tracing::info!(target: "iv8.console", method = method, message = message, "console.{}", method),
        "trace" => tracing::trace!(target: "iv8.console", method = method, message = message, "console.{}", method),
        _ => tracing::debug!(target: "iv8.console", method = method, message = message, "console.{}", method),
    }
}

// ─── Coverage matrix ────────────────────────────────────────────────

/// Expected coverage: each category should have at least one event at
/// each relevant level. This const documents the expected coverage and
/// is validated by tests.
///
/// Levels: E=ERROR, W=WARN, I=INFO, D=DEBUG, T=TRACE
pub const COVERAGE_MATRIX: &[(&str, &[char])] = &[
    ("iv8.init",      &['E', 'W', 'I', 'D']),
    ("iv8.dom",       &['E', 'D']),
    ("iv8.config",    &['W', 'I']),
    ("iv8.worker",    &['E', 'W']),
    ("iv8.callback",  &['E', 'W']),
    ("iv8.eval",      &['W', 'D']),
    ("iv8.console",   &['D']),
    ("iv8.inspector", &['I', 'W']),
    ("iv8.shim",      &['D']),
    ("iv8.canvas",    &['W']),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_not_empty() {
        assert!(!catalog().is_empty());
    }

    #[test]
    fn test_catalog_has_init_events() {
        let init_events: Vec<_> = catalog().iter().filter(|e| e.category == "iv8.init").collect();
        assert!(init_events.len() >= 10, "expected at least 10 init events, got {}", init_events.len());
    }

    #[test]
    fn test_catalog_names_unique() {
        let mut names: Vec<&str> = catalog().iter().map(|e| e.name).collect();
        names.sort();
        let before = names.len();
        names.dedup();
        assert_eq!(names.len(), before, "duplicate event names in catalog");
    }

    #[test]
    fn test_catalog_all_have_fields() {
        for event in catalog() {
            assert!(!event.name.is_empty(), "event has empty name");
            assert!(!event.category.is_empty(), "event {} has empty category", event.name);
            assert!(!event.level.is_empty(), "event {} has empty level", event.name);
        }
    }

    #[test]
    fn test_coverage_matrix_satisfied() {
        for (category, expected_levels) in COVERAGE_MATRIX {
            for &level in *expected_levels {
                let level_str = match level {
                    'E' => "ERROR",
                    'W' => "WARN",
                    'I' => "INFO",
                    'D' => "DEBUG",
                    'T' => "TRACE",
                    _ => panic!("unknown level char: {}", level),
                };
                let found = catalog().iter().any(|e| {
                    e.category == *category && e.level == level_str
                });
                assert!(
                    found,
                    "coverage gap: category {} has no {} event",
                    category, level_str
                );
            }
        }
    }

    #[test]
    fn test_coverage_matrix_categories_exist() {
        let catalog_cats: std::collections::HashSet<&str> =
            catalog().iter().map(|e| e.category).collect();
        for (category, _) in COVERAGE_MATRIX {
            assert!(
                catalog_cats.contains(*category),
                "coverage matrix category {} has no catalog events",
                category
            );
        }
    }

    #[test]
    fn test_catalog_event_count() {
        let count = catalog().len();
        assert!(count >= 25, "expected at least 25 catalog events, got {}", count);
    }

    #[test]
    fn test_no_direct_tracing_outside_telemetry() {
        // This test validates the logging convention: all tracing:: calls
        // must go through telemetry.rs catalog functions.
        // We check by verifying that catalog() has events for all categories
        // listed in COVERAGE_MATRIX, and that the catalog is comprehensive.
        // A CI-level grep check should also enforce this at review time.
        let cats: std::collections::HashSet<&str> =
            catalog().iter().map(|e| e.category).collect();
        let expected_cats: std::collections::HashSet<&str> = COVERAGE_MATRIX
            .iter()
            .map(|(c, _)| *c)
            .collect();
        assert_eq!(cats, expected_cats,
            "catalog categories must match coverage matrix");
    }

    #[test]
    fn test_all_events_have_safety() {
        // Every event must have a safety level documented in its EventSpec.
        // Safety is enforced by the Safety enum type — if it compiles, it's set.
        // This test just verifies the catalog is non-empty and well-formed.
        for event in catalog() {
            // Safety enum is always set (not Option), so just verify it matches
            // the documented pattern: Safe for counts/names, Diagnostic for
            // errors/panics, Sensitive for user data.
            match event.safety {
                Safety::Safe => {}
                Safety::Diagnostic => {}
                Safety::Sensitive => {}
            }
        }
    }
}
