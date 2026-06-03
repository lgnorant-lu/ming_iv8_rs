//! Logging configuration for iv8-rs.
//!
//! Provides `enable_logging(level, output)` Python function and
//! respects `IV8_LOG` environment variable.

use pyo3::prelude::*;
use std::sync::Once;
use tracing_subscriber::{fmt, EnvFilter};

static LOGGING_INIT: Once = Once::new();

fn parse_filter(spec: &str) -> EnvFilter {
    EnvFilter::try_new(spec).unwrap_or_else(|_| EnvFilter::new("iv8=info"))
}

/// Enable iv8-rs logging output.
///
/// Args:
///     level: Log level filter (e.g. "info", "debug", "iv8=debug,iv8::core::dom=trace")
///     output: Output target - "stderr" (default), "stdout"
///
/// Notes:
///     - Can only be called once per process. Subsequent calls are ignored with a warning.
///     - If `IV8_LOG` environment variable is set, it takes precedence over the `level` argument.
///     - Default (without calling this function): no logging output.
#[pyfunction]
#[pyo3(signature = (level="info", output="stderr"))]
pub fn enable_logging(level: &str, output: &str) -> PyResult<()> {
    let level = level.to_string();
    let output = output.to_string();

    let mut already_initialized = true;
    LOGGING_INIT.call_once(|| {
        already_initialized = false;

        // IV8_LOG env var takes precedence
        let filter = if let Ok(env_filter) = std::env::var("IV8_LOG") {
            EnvFilter::try_new(env_filter).unwrap_or_else(|_| parse_filter(&format!("iv8={level}")))
        } else {
            parse_filter(&format!("iv8={level}"))
        };

        match output.as_str() {
            "stdout" => {
                fmt()
                    .with_env_filter(filter)
                    .with_writer(std::io::stdout)
                    .init();
            }
            _ => {
                // Default: stderr
                fmt()
                    .with_env_filter(filter)
                    .with_writer(std::io::stderr)
                    .init();
            }
        }
    });

    if already_initialized {
        tracing::warn!("enable_logging called more than once; ignoring");
    }

    Ok(())
}
