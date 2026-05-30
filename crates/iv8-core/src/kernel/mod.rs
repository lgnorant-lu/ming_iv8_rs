//! Kernel abstraction layer.
//!
//! `KernelImpl` trait defines how to run JS. v0.1 has one implementation:
//! `EmbeddedV8Kernel`. v0.3+ can add `CdpKernel` etc.

pub mod embedded_v8;


/// Options for a single eval call.
#[derive(Debug, Clone, Default)]
pub struct EvalOpts {
    pub source_url: Option<String>,
    pub line_offset: i32,
    pub column_offset: i32,
}

/// Configuration for creating a kernel.
#[derive(Debug, Clone)]
pub struct KernelConfig {
    pub strict_compat: bool,
    pub time_mode: crate::state::TimeMode,
    pub js_api_name: String,
    pub environment_overrides: Option<std::collections::HashMap<String, serde_json::Value>>,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            strict_compat: true,
            time_mode: crate::state::TimeMode::Logical,
            js_api_name: "__iv8__".to_string(),
            environment_overrides: None,
        }
    }
}
