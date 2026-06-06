//! Runtime Hook Pack — transparent and aggressive hook modules.
//!
//! Each hook module provides a JS prelude (code to eval before main source)
//! and a collector (extract captured evidence from the runtime).

pub mod aggressive;
pub mod transparent;
