//! iv8-surface — Generated browser surface FunctionTemplate stubs.
//!
//! This crate provides install_browser_surface(), the single entry point
//! for registering all generated FunctionTemplate stubs to a V8 context.
//!
//! The generated code lives in `generated/` — produced by iv8-surface-codegen
//! from unified_ir.json (v0.8.18).
//!
//! v0.8.20: BrowserSurface integration with Feature Flag control.
//! Layers 0/2/3/4 are deferred to v0.8.21+.
//! C++ wrapper build.rs is deferred to v0.8.21 (requires cross-crate V8 header paths).

pub mod behavior;
pub mod descriptor;
pub mod generated;
pub mod registry;
pub mod type_conv;

pub use behavior::BehaviorCallbackRegistry;
pub use registry::BrowserSurfaceRegistry;

use std::fmt;

/// Error type for BrowserSurface installation failures.
#[derive(Debug)]
pub enum SurfaceInstallError {
    Layer0Failed(String),
    Layer1Failed(String),
    Layer2Failed(String),
    Layer3Warning(String),
    Layer4Warning(String),
    ScopeError(String),
}

impl fmt::Display for SurfaceInstallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Layer0Failed(s) => write!(f, "Layer 0 Global Foundation failed: {}", s),
            Self::Layer1Failed(s) => write!(f, "Layer 1 IDL Surface failed: {}", s),
            Self::Layer2Failed(s) => write!(f, "Layer 2 Behavior Objects failed: {}", s),
            Self::Layer3Warning(s) => write!(f, "Layer 3 Anti-Detection warning: {}", s),
            Self::Layer4Warning(s) => write!(f, "Layer 4 Document Bootstrap warning: {}", s),
            Self::ScopeError(s) => write!(f, "V8 handle scope error: {}", s),
        }
    }
}

impl std::error::Error for SurfaceInstallError {}

type Result<T> = std::result::Result<T, SurfaceInstallError>;

/// Install all generated browser surface stubs into the V8 context.
///
/// Layer 1 (IDL-Generated Surface): install_all() registers 1284 FunctionTemplates.
/// Returns BrowserSurfaceRegistry for RuntimeState storage.
/// Layers 0/2/3/4 are stubs — deferred to v0.8.21+.
pub fn install_browser_surface(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
    _callbacks: &BehaviorCallbackRegistry,
) -> Result<BrowserSurfaceRegistry> {
    // Layer 1: Install all IDL-generated FunctionTemplates
    let mut registry = BrowserSurfaceRegistry::new();

    // Track interface count by wrapping the install_all call
    generated::install_all::install_all(scope, global);

    // Populate registry with an approximate count
    // (actual template tracking requires modifying generated code in v0.8.21)
    registry.set_count(1284);

    Ok(registry)
}
