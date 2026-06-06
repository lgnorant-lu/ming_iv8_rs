//! Python exception class hierarchy for iv8-rs.

use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

use iv8_core::IV8Error;

// Exception hierarchy:
// Exception
//   ├── JSError (base for all JS-originated errors)
//   │     ├── JSCompileError (syntax errors)
//   │     ├── JSTimeoutError (script terminated)
//   │     └── JSMemoryError (V8 OOM)
//   └── JSPanic (Rust panic caught at FFI boundary — not a JS error)

create_exception!(
    iv8_rs,
    JSError,
    PyException,
    "Base exception for JavaScript errors."
);
create_exception!(
    iv8_rs,
    JSCompileError,
    JSError,
    "JavaScript compile/syntax error."
);
create_exception!(
    iv8_rs,
    JSTimeoutError,
    JSError,
    "JavaScript execution terminated (timeout)."
);
create_exception!(iv8_rs, JSMemoryError, JSError, "V8 out of memory.");
create_exception!(
    iv8_rs,
    JSPanic,
    PyException,
    "Rust panic caught at FFI boundary."
);

/// Convert IV8Error to PyErr.
pub fn iv8_error_to_pyerr(err: IV8Error) -> PyErr {
    match err {
        IV8Error::Js {
            name,
            message,
            stack,
            ..
        } => {
            let msg = if stack.is_empty() {
                format!("{name}: {message}")
            } else {
                format!("{name}: {message}\n{stack}")
            };
            // Create JSError with structured attributes via args tuple
            // PyO3 create_exception doesn't support custom fields directly,
            // so we encode name/message/stack in the error message string.
            // Users can parse: str(e).split(': ', 1) to get name and message.
            JSError::new_err(msg)
        }
        IV8Error::Compile {
            message,
            line,
            column,
        } => JSCompileError::new_err(format!("{message} at {line}:{column}")),
        IV8Error::Terminated => JSTimeoutError::new_err("script terminated"),
        IV8Error::OutOfMemory { details } => JSMemoryError::new_err(details),
        IV8Error::Internal(msg) => pyo3::exceptions::PyRuntimeError::new_err(msg),
    }
}

/// Register exception classes on the module.
pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("JSError", m.py().get_type::<JSError>())?;
    m.add("JSCompileError", m.py().get_type::<JSCompileError>())?;
    m.add("JSTimeoutError", m.py().get_type::<JSTimeoutError>())?;
    m.add("JSMemoryError", m.py().get_type::<JSMemoryError>())?;
    m.add("JSPanic", m.py().get_type::<JSPanic>())?;
    Ok(())
}
