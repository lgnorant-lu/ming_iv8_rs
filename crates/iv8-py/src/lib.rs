//! iv8-py: Python binding for iv8-rs via PyO3.

// Allow expect_used in PyO3 binding code where panics are caught by PyO3's
// exception handling mechanism.
#![allow(clippy::expect_used)]
#![allow(clippy::unwrap_used)]
#![allow(clippy::too_many_arguments)]

use pyo3::prelude::*;

mod context;
mod debugger;
mod error;
mod expose;
mod logging;
mod value_convert;

/// The iv8_rs Python module.
#[pymodule]
fn _iv8(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<context::JSContext>()?;
    m.add_class::<debugger::Debugger>()?;
    m.add_function(wrap_pyfunction!(logging::enable_logging, m)?)?;
    error::register(m)?;
    Ok(())
}
