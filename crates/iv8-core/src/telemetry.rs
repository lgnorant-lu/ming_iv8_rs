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
        name: "worker_import_script_not_found",
        category: "iv8.worker",
        level: "WARN",
        safety: Safety::Diagnostic,
        fields: &["url"],
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
        assert!(init_events.len() >= 7, "expected at least 7 init events, got {}", init_events.len());
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
}
