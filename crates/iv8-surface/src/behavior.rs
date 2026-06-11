//! BehaviorCallbackRegistry — deep behavior callbacks for generated stubs.
//!
//! Stores optional callback functions that override the default stub behavior
//! for specific interface members. Callbacks are divided into two groups:
//!
//! V8-bound (Rc<RefCell<>>, !Send): callbacks that return v8::Local types
//! Send-safe (Box<dyn Fn + Send>): callbacks without V8 type dependencies
//!
//! v0.8.20: placeholder types (all Box<dyn Fn()>).
//! Actual callback signatures will be refined in v0.8.21 when Canvas/WebGL
//! deep stubs are implemented (scope/v8.0.2/Canvas2D/v8_frame_rgba, etc.).

use std::cell::RefCell;
use std::rc::Rc;

// ── Callback type aliases (placeholder — actual signatures in v0.8.21) ──────

/// Canvas 2D context factory — deferred to v0.8.21.
pub type CanvasContextFactory = Rc<RefCell<Option<Box<dyn Fn()>>>>;

/// Canvas 2D create gradient — deferred to v0.8.21.
pub type GradientFactory = Rc<RefCell<Option<Box<dyn Fn()>>>>;

/// WebGL context factory — deferred to v0.8.21.
pub type WebGLContextFactory = Rc<RefCell<Option<Box<dyn Fn()>>>>;

/// WebGL getParameter — deferred to v0.8.21.
pub type WebGLGetParameter = Rc<RefCell<Option<Box<dyn Fn()>>>>;

/// WebGL getExtension — deferred to v0.8.21.
pub type WebGLGetExtension = Rc<RefCell<Option<Box<dyn Fn()>>>>;

/// Audio context factory — deferred to v0.8.21.
pub type AudioContextFactory = Rc<RefCell<Option<Box<dyn Fn()>>>>;

/// Send-safe callbacks — deferred to v0.8.21.
pub type SendSafeCallback = RefCell<Option<Box<dyn Fn() + Send + 'static>>>;

// ── Registry struct ──────────────────────────────────────────────────────────

pub struct BehaviorCallbackRegistry {
    // V8-bound group (!Send) — deferred signatures to v0.8.21
    pub canvas_2d_factory: CanvasContextFactory,
    pub canvas_2d_gradient: GradientFactory,
    pub webgl_factory: WebGLContextFactory,
    pub webgl_get_parameter: WebGLGetParameter,
    pub webgl_get_extension: WebGLGetExtension,
    pub audio_factory: AudioContextFactory,

    // Send-safe group — deferred to v0.8.21
    pub canvas_2d_to_data_url: SendSafeCallback,
    pub canvas_2d_get_image_data: SendSafeCallback,
    pub canvas_2d_set_size: SendSafeCallback,
    pub reserved_behavior: SendSafeCallback,
}

impl BehaviorCallbackRegistry {
    pub fn new() -> Self {
        Self {
            canvas_2d_factory: Rc::new(RefCell::new(None)),
            canvas_2d_gradient: Rc::new(RefCell::new(None)),
            webgl_factory: Rc::new(RefCell::new(None)),
            webgl_get_parameter: Rc::new(RefCell::new(None)),
            webgl_get_extension: Rc::new(RefCell::new(None)),
            audio_factory: Rc::new(RefCell::new(None)),
            canvas_2d_to_data_url: RefCell::new(None),
            canvas_2d_get_image_data: RefCell::new(None),
            canvas_2d_set_size: RefCell::new(None),
            reserved_behavior: RefCell::new(None),
        }
    }
}

impl Default for BehaviorCallbackRegistry {
    fn default() -> Self {
        Self::new()
    }
}
