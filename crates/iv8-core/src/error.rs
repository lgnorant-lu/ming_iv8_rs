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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn js_error_display() {
        let e = IV8Error::Js {
            name: "TypeError".into(),
            message: "foo is not a function".into(),
            stack: "at <anonymous>:1:5".into(),
            value: None,
        };
        let s = e.to_string();
        assert!(s.contains("TypeError"), "display should contain name");
        assert!(
            s.contains("foo is not a function"),
            "display should contain message"
        );
    }

    #[test]
    fn compile_error_display() {
        let e = IV8Error::Compile {
            message: "unexpected token".into(),
            line: 3,
            column: 8,
        };
        let s = e.to_string();
        assert!(s.contains("compile error"), "compile error prefix");
        assert!(s.contains("unexpected token"), "message in display");
        assert!(s.contains("3:8"), "line:column in display");
    }

    #[test]
    fn terminated_display() {
        let e = IV8Error::Terminated;
        assert_eq!(e.to_string(), "script terminated");
    }

    #[test]
    fn out_of_memory_display() {
        let e = IV8Error::OutOfMemory {
            details: "heap exhausted".into(),
        };
        let s = e.to_string();
        assert!(s.contains("out of memory"), "OOM prefix");
        assert!(s.contains("heap exhausted"), "details in display");
    }

    #[test]
    fn internal_error_display() {
        let e = IV8Error::Internal("logic error: bad state".into());
        let s = e.to_string();
        assert!(s.contains("internal error"), "internal prefix");
        assert!(s.contains("logic error: bad state"), "message in display");
    }

    #[test]
    fn extract_panic_msg_non_string() {
        let result = std::panic::catch_unwind(|| {
            panic!("{:?}", vec![1, 2, 3]);
        });
        let err = result.unwrap_err();
        let msg = extract_panic_msg(&err);
        assert!(!msg.is_empty(), "should produce non-empty fallback");
    }
}
