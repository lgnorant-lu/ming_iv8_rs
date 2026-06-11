//! iv8-surface — Generated browser surface FunctionTemplate stubs.
//!
//! This crate provides install_browser_surface(), the single entry point
//! for registering all generated FunctionTemplate stubs to a V8 context.
//!
//! The generated code lives in `generated/` — produced by iv8-surface-codegen
//! from unified_ir.json (v0.8.18).
//!
//! v0.8.20: BrowserSurface integration with Feature Flag control.

pub mod behavior;
pub mod descriptor;
pub mod generated;
pub mod registry;
pub mod type_conv;

pub use registry::BrowserSurfaceRegistry;
pub use behavior::BehaviorCallbackRegistry;

/// Install all generated browser surface stubs into the V8 context.
///
/// v0.8.20: full layer-based installation with callback registry.
pub fn install_browser_surface(
    scope: &v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
    _callbacks: &BehaviorCallbackRegistry,
) -> BrowserSurfaceRegistry {
    generated::install_all::install_all(scope, global);
    BrowserSurfaceRegistry::new()
}
