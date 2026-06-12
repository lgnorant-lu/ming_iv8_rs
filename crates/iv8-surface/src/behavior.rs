//! BehaviorCallbackRegistry — deep behavior callbacks for generated stubs.
//!
//! Stores optional callback functions that override the default stub behavior
//! for specific interface members. Callbacks are divided into two groups:
//!
//! V8-bound (Rc<RefCell<>>, !Send): callbacks that receive V8 scope and
//! return v8::Local types. These cannot cross thread boundaries.
//! Send-safe: callbacks without V8 type dependencies, safe to share.

#![expect(
    clippy::type_complexity,
    reason = "Rc<RefCell<Option<Box<dyn Fn(...)>>>> is the standard pattern for optional V8 callbacks"
)]

use std::cell::RefCell;
use std::rc::Rc;

// ── V8-bound callback type aliases (!Send) ──────────────────────────────────

/// Canvas 2D context factory: creates CanvasRenderingContext2D from HTMLCanvasElement.
pub type CanvasContextFactory = Rc<RefCell<Option<Box<dyn for<'s> Fn(&v8::PinScope<'s, '_>, v8::Local<'s, v8::Object>) -> v8::Local<'s, v8::Object>>>>>;

/// Canvas 2D gradient factory: creates CanvasGradient from x0,y0,x1,y1.
pub type GradientFactory = Rc<RefCell<Option<Box<dyn for<'s> Fn(&v8::PinScope<'s, '_>, f64, f64, f64, f64) -> v8::Local<'s, v8::Object>>>>>;

/// WebGL context factory: creates WebGLRenderingContext from HTMLCanvasElement.
pub type WebGLContextFactory = Rc<RefCell<Option<Box<dyn for<'s> Fn(&v8::PinScope<'s, '_>, v8::Local<'s, v8::Object>) -> v8::Local<'s, v8::Object>>>>>;

/// WebGL getParameter: (pname: u32) -> GL value.
pub type WebGLGetParameter = Rc<RefCell<Option<Box<dyn for<'s> Fn(&v8::PinScope<'s, '_>, u32) -> v8::Local<'s, v8::Value>>>>>;

/// WebGL getExtension: (name: &str) -> extension object or null.
pub type WebGLGetExtension = Rc<RefCell<Option<Box<dyn for<'s> Fn(&v8::PinScope<'s, '_>, &str) -> v8::Local<'s, v8::Value>>>>>;

/// Audio context factory: creates AudioContext.
pub type AudioContextFactory = Rc<RefCell<Option<Box<dyn for<'s> Fn(&v8::PinScope<'s, '_>) -> v8::Local<'s, v8::Object>>>>>;

// ── Send-safe callback type alias ────────────────────────────────────────────

/// Send-safe behavior callback: input bytes -> result string or error.
/// Used for operations that don't require V8 scope (e.g. toDataURL encoding).
pub type SendSafeCallback = RefCell<Option<Box<dyn Fn(Vec<u8>) -> Result<String, String> + Send + 'static>>>;

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
