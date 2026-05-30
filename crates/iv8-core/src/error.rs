//! Error types for iv8-core.

use thiserror::Error;

/// Top-level error enum for iv8-rs.
/// Maps to Python exception hierarchy via PyO3 in iv8-py.
#[derive(Debug, Error)]
pub enum IV8Error {
    /// JavaScript runtime exception (throw / ReferenceError / TypeError etc.)
    #[error("{name}: {message}")]
    Js {
        name: String,
        message: String,
        stack: String,
        value: Option<serde_json::Value>,
    },

    /// JavaScript compile error (SyntaxError)
    #[error("compile error at {line}:{column}: {message}")]
    Compile {
        message: String,
        line: i32,
        column: i32,
    },

    /// Script terminated (timeout or user-initiated terminate_execution)
    #[error("script terminated")]
    Terminated,

    /// V8 out-of-memory
    #[error("out of memory: {details}")]
    OutOfMemory { details: String },

    /// Internal error (Rust-side logic error, should not happen)
    #[error("internal error: {0}")]
    Internal(String),
}

/// Extract a human-readable message from a panic payload.
pub fn extract_panic_msg(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(s) = payload.downcast_ref::<&str>() {
        s.to_string()
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.clone()
    } else {
        "panic (non-string payload)".to_string()
    }
}
