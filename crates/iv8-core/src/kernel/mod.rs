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
        }
    }
}

impl KernelConfig {
    /// Use a materialized profile as the certified v0.8.32 environment source.
    ///
    /// This intentionally projects through `environment_overrides`, because the
    /// active native getters read `RuntimeState.environment` as their single
    /// source of truth. BehaviorConfig/BCR parameterization remains a scaffold
    /// for later native installer specialization.
    pub fn with_profile_matrix(mut self, matrix: &iv8_profile::ProfileMatrix) -> Self {
        self.environment_overrides = Some(matrix.to_environment_overrides());
        self
    }
}
