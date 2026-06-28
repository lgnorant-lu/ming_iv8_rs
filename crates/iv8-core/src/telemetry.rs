//! Telemetry helpers for structured logging.
//!
//! This module provides span helpers and standard field names for
//! consistent tracing across the IV8 codebase. See
//! `docs/conventions/logging-conventions.md` for the full specification.

/// Create a debug-level span for a DOM prototype merge operation.
#[macro_export]
macro_rules! iv8_merge_span {
    ($interface:expr) => {
        tracing::debug_span!(
            "merge",
            interface = %$interface,
        )
    };
}

/// Create a debug-level span for a template creation operation.
#[macro_export]
macro_rules! iv8_template_span {
    ($interface:expr, $members:expr) => {
        tracing::debug_span!(
            "create_template",
            interface = %$interface,
            members = $members,
        )
    };
}

/// Create a debug-level span for a config resolution operation.
#[macro_export]
macro_rules! iv8_config_span {
    ($key:expr) => {
        tracing::debug_span!(
            "config_resolve",
            key = %$key,
        )
    };
}
