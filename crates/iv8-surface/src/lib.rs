//! iv8-surface — Generated browser surface FunctionTemplate stubs.
//!
//! This crate provides install_browser_surface(), the single entry point
//! for registering all generated FunctionTemplate stubs to a V8 context.
//!
//! The generated code lives in `generated/` — produced by iv8-surface-codegen
//! from unified_ir.json (v0.8.18).
//!
//! v0.8.21: P0 deep stub implementation — Canvas 2D, WebGL, Location, Navigator.
//! SubtleCrypto integration deferred to v0.8.22.
//! Layers 0/3/4 (Global Foundation, Anti-Detection, Document Bootstrap)
//! and C++ wrapper build.rs are deferred to v0.8.22+.

pub mod behavior;
pub mod descriptor;
pub mod generated;
pub mod hand_implemented;
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
/// Layer 2 (Deep Stubs): Canvas 2D factory + WebGL data + Location URL parser +
///     Navigator verification — registered via BehaviorCallbackRegistry.
/// Returns BrowserSurfaceRegistry for RuntimeState storage.
pub fn install_browser_surface(
    scope: &mut v8::PinScope<'_, '_>,
    global: v8::Local<v8::Object>,
    callbacks: &BehaviorCallbackRegistry,
) -> Result<BrowserSurfaceRegistry> {
    // Layer 1: Install all IDL-generated FunctionTemplates
    generated::install_all::install_all(scope, global);

    // Layer 2: Register deep stub callbacks via BehaviorCallbackRegistry
    //
    // Canvas 2D: factory callback creates CanvasRenderingContext2D instances.
    hand_implemented::register_canvas_2d_callbacks(
        &callbacks.canvas_2d_factory,
        &callbacks.canvas_2d_gradient,
    );

    // Canvas 2D send-safe: toDataURL, getImageData, setSize.
    hand_implemented::register_canvas_send_safe_callbacks(
        &callbacks.canvas_2d_to_data_url,
        &callbacks.canvas_2d_get_image_data,
        &callbacks.canvas_2d_set_size,
    );

    // WebGL: factory + getParameter + getExtension.
    hand_implemented::register_webgl_callbacks(
        &callbacks.webgl_factory,
        &callbacks.webgl_get_parameter,
        &callbacks.webgl_get_extension,
    );

    // WebGL: parameter map is data-only — used by getParameter callback above.
    let gl_params = hand_implemented::webgl::build_gl_param_map();
    debug_assert!(gl_params.len() >= 30, "WebGL parameter map incomplete");

    // Location: URL parser is data-only — V8 Location FunctionTemplate in v0.8.22.
    let loc = hand_implemented::location::LocationState::default();
    debug_assert!(!loc.href.is_empty(), "LocationState default href empty");

    // Navigator: 22 getter names verified at data level.
    debug_assert!(hand_implemented::navigator::verify_navigator_getters());

    // Populate registry
    let mut registry = BrowserSurfaceRegistry::new();
    registry.set_count(1284);
    Ok(registry)
}
