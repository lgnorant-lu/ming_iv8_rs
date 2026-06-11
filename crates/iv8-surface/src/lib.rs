//! iv8-surface — Generated browser surface FunctionTemplate stubs.
//!
//! This crate provides install_browser_surface(), the single entry point
//! for registering all generated FunctionTemplate stubs to a V8 context.
//!
//! The generated code lives in `generated/` — produced by iv8-surface-codegen
//! from unified_ir.json (v0.8.18).
//!
//! v0.8.19: compile verification only — not yet integrated with iv8-core.

pub mod behavior;
pub mod descriptor;
pub mod generated;
pub mod registry;
pub mod type_conv;

/// Install all generated browser surface stubs into the V8 context.
///
/// Called once per context creation. Installs FunctionTemplate constructors
/// on the global object and sets up prototype chains. The stubs return
/// type-correct default values — deep behavior is implemented in v0.8.21+.
pub fn install_browser_surface(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    generated::install_all::install_all(scope, global);
}
