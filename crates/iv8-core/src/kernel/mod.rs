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
    /// If set, Math.random() returns a deterministic sequence seeded by this value.
    pub random_seed: Option<u64>,
    /// If set, crypto.getRandomValues() uses this seed for deterministic output.
    pub crypto_seed: Option<u64>,
    /// If set, Date.now() / performance.now() / new Date() return this fixed timestamp (ms).
    pub time_freeze: Option<f64>,
    /// User-defined property overrides applied after all system initialization.
    pub user_overrides: crate::user_overrides::UserOverrides,
    /// Use pre-v0.8.26 init chain (install_environment → undetect_shims → dom_templates).
    /// Default false: uses install_browser_surface_init (1284 IDL + 14 native behaviors).
    pub use_old_chain: bool,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            strict_compat: true,
            time_mode: crate::state::TimeMode::Logical,
            js_api_name: "__iv8__".to_string(),
            environment_overrides: None,
            random_seed: None,
            crypto_seed: None,
            time_freeze: None,
            user_overrides: crate::user_overrides::UserOverrides::new(),
            use_old_chain: false,
        }
    }
}
