//! BehaviorCallbackRegistry — deep behavior callbacks for generated stubs.
//!
//! Stores optional callback functions that override the default stub behavior
//! for specific interface members. Callbacks are divided into two groups:
//!
//! V8-bound (Rc<RefCell<>>, !Send): callbacks that return v8::Local types
//! Send-safe (Box<dyn Fn + Send>): callbacks without V8 type dependencies

use std::cell::RefCell;
use std::rc::Rc;

// ── Callback type aliases ───────────────────────────────────────────────────

/// Canvas 2D context factory: (scope, width, height) -> rendering context object.
pub type CanvasContextFactory = Rc<RefCell<Option<Box<dyn Fn()>>>>;

/// Canvas 2D create gradient callback.
pub type GradientFactory = Rc<RefCell<Option<Box<dyn Fn()>>>>;

/// WebGL context factory callback.
pub type WebGLContextFactory = Rc<RefCell<Option<Box<dyn Fn()>>>>;

/// WebGL getParameter callback.
pub type WebGLGetParameter = Rc<RefCell<Option<Box<dyn Fn()>>>>;

/// WebGL getExtension callback.
pub type WebGLGetExtension = Rc<RefCell<Option<Box<dyn Fn()>>>>;

/// Audio context factory callback.
pub type AudioContextFactory = Rc<RefCell<Option<Box<dyn Fn()>>>>;

/// Callbacks without V8 type dependencies (Send-safe).
pub type SendSafeCallback = RefCell<Option<Box<dyn Fn() + Send + 'static>>>;

// ── Registry struct ──────────────────────────────────────────────────────────

pub struct BehaviorCallbackRegistry {
    // V8-bound group (!Send)
    pub canvas_2d_factory: CanvasContextFactory,
    pub canvas_2d_gradient: GradientFactory,
    pub webgl_factory: WebGLContextFactory,
    pub webgl_get_parameter: WebGLGetParameter,
    pub webgl_get_extension: WebGLGetExtension,
    pub audio_factory: AudioContextFactory,

    // Send-safe group
    pub on_behavior_1: SendSafeCallback,
    pub on_behavior_2: SendSafeCallback,
    pub on_behavior_3: SendSafeCallback,
    pub on_behavior_4: SendSafeCallback,
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
            on_behavior_1: RefCell::new(None),
            on_behavior_2: RefCell::new(None),
            on_behavior_3: RefCell::new(None),
            on_behavior_4: RefCell::new(None),
        }
    }
}

impl Default for BehaviorCallbackRegistry {
    fn default() -> Self {
        Self::new()
    }
}
