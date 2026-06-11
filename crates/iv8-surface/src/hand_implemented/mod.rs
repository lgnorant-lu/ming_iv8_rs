//! Hand-implemented behavior objects — Canvas 2D / WebGL deep stubs.
//!
//! v0.8.21: replaces JS shim (canvas/binding.rs) with native V8 FunctionTemplate
//! stubs wired through BehaviorCallbackRegistry.

use crate::behavior::{CanvasContextFactory, GradientFactory};
use crate::hand_implemented::canvas2d::create_canvas_rendering_context_2d_template;

/// Register Canvas 2D factory callback into BehaviorCallbackRegistry.
/// The factory creates CanvasRenderingContext2D instances when
/// HTMLCanvasElement.getContext('2d') is called in JS.
pub fn register_canvas_2d_callbacks(
    factory: &CanvasContextFactory,
    _gradient: &GradientFactory,
) {
    // Wrap the template creation in a closure that can be stored.
    // v0.8.21: placeholder — full V8 scope injection in Task 2 continuation.
    tracing::info!("Canvas 2D deep stub factory registered (placeholder)");
    let _ = factory;
}

/// Default property values for CanvasRenderingContext2D.
pub const CANVAS_2D_DEFAULTS: &[(&str, &str)] = &[
    ("fillStyle", "\"#000000\""),
    ("strokeStyle", "\"#000000\""),
    ("lineWidth", "1"),
    ("lineCap", "\"butt\""),
    ("lineJoin", "\"miter\""),
    ("miterLimit", "10"),
    ("font", "\"10px sans-serif\""),
    ("textAlign", "\"start\""),
    ("textBaseline", "\"alphabetic\""),
    ("globalAlpha", "1"),
    ("globalCompositeOperation", "\"source-over\""),
    ("imageSmoothingEnabled", "true"),
    ("imageSmoothingQuality", "\"low\""),
    ("shadowBlur", "0"),
    ("shadowColor", "\"rgba(0, 0, 0, 0)\""),
    ("shadowOffsetX", "0"),
    ("shadowOffsetY", "0"),
    ("direction", "\"inherit\""),
    ("letterSpacing", "\"0px\""),
    ("wordSpacing", "\"0px\""),
    ("textRendering", "\"auto\""),
    ("fontKerning", "\"auto\""),
    ("fontStretch", "\"100%\""),
    ("fontVariantCaps", "\"normal\""),
];

/// Canvas 2D method names (stub implementations).
pub const CANVAS_2D_METHODS: &[&str] = &[
    "fillRect", "strokeRect", "clearRect",
    "fillText", "strokeText", "measureText",
    "beginPath", "closePath", "moveTo", "lineTo",
    "arc", "arcTo", "bezierCurveTo", "quadraticCurveTo",
    "rect", "ellipse", "roundRect",
    "fill", "stroke", "clip",
    "save", "restore",
    "scale", "rotate", "translate", "transform", "setTransform", "resetTransform",
    "createLinearGradient", "createRadialGradient", "createConicGradient",
    "createPattern", "createImageData", "getImageData", "putImageData",
    "drawImage", "drawFocusIfNeeded", "scrollPathIntoView",
    "getTransform", "getContextAttributes",
    "isPointInPath", "isPointInStroke",
];

pub mod canvas2d;
pub mod location;
