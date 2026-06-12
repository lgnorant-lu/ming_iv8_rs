//! Hand-implemented behavior objects — Canvas 2D / WebGL deep stubs.
//!
//! v0.8.21: replaces JS shim (canvas/binding.rs) with native V8 FunctionTemplate
//! stubs wired through BehaviorCallbackRegistry.
//! v0.8.28: BCR Step A — canvas_2d_factory + webgl_factory populated with
//! working closures from existing install_X functions.

use crate::behavior::{
    BehaviorInstaller, CanvasContextFactory, GradientFactory, SendSafeCallback,
    WebGLContextFactory, WebGLGetExtension, WebGLGetParameter,
};

/// Register Canvas 2D factory callback into BehaviorCallbackRegistry.
/// The factory creates CanvasRenderingContext2D instances when
/// HTMLCanvasElement.getContext('2d') is called in JS.
pub fn register_canvas_2d_callbacks(
    factory: &CanvasContextFactory,
    gradient: &GradientFactory,
) {
    *factory.borrow_mut() = Some(Box::new(
        |scope: &v8::PinScope<'_, '_>, _canvas_element: v8::Local<v8::Object>| {
            match canvas2d::create_canvas_2d_context_instance(scope) {
                Some(obj) => obj,
                None => v8::Object::new(scope),
            }
        },
    ));

    *gradient.borrow_mut() = Some(Box::new(
        |scope, x0, y0, x1, y1| {
            let obj = v8::Object::new(scope);
            // CanvasGradient stub with addColorStop method
            let key = v8::String::new(scope, "addColorStop").unwrap();
            let tmpl = v8::FunctionTemplate::builder_raw(empty_add_color_stop).build(scope);
            obj.set(scope, key.into(), tmpl.get_function(scope).unwrap().into());
            let key_x0 = v8::String::new(scope, "x0").unwrap();
            let key_y0 = v8::String::new(scope, "y0").unwrap();
            let key_x1 = v8::String::new(scope, "x1").unwrap();
            let key_y1 = v8::String::new(scope, "y1").unwrap();
            obj.set(scope, key_x0.into(), v8::Number::new(scope, x0).into());
            obj.set(scope, key_y0.into(), v8::Number::new(scope, y0).into());
            obj.set(scope, key_x1.into(), v8::Number::new(scope, x1).into());
            obj.set(scope, key_y1.into(), v8::Number::new(scope, y1).into());
            obj
        },
    ));
}

unsafe extern "C" fn empty_add_color_stop(_info: *const v8::FunctionCallbackInfo) {}

/// Register WebGL callbacks into BehaviorCallbackRegistry.
pub fn register_webgl_callbacks(
    webgl_factory: &WebGLContextFactory,
    webgl_get_parameter: &WebGLGetParameter,
    webgl_get_extension: &WebGLGetExtension,
) {
    let gl_params = webgl::build_gl_param_map();

    // WebGL context factory: creates a WebGLRenderingContext stub
    *webgl_factory.borrow_mut() = Some(Box::new(
        move |scope: &v8::PinScope<'_, '_>, _canvas_element: v8::Local<v8::Object>| {
            webgl::create_webgl_rendering_context_instance(scope)
        },
    ));

    // WebGL getParameter: dispatches pname -> typed value
    *webgl_get_parameter.borrow_mut() = Some(Box::new(
        move |scope: &v8::PinScope<'_, '_>, pname: u32| -> v8::Local<v8::Value> {
            webgl::gl_get_parameter_value(scope, &gl_params, pname)
        },
    ));

    // WebGL getExtension: looks up known extensions
    *webgl_get_extension.borrow_mut() = Some(Box::new(
        move |scope: &v8::PinScope<'_, '_>, name: &str| -> v8::Local<v8::Value> {
            webgl::gl_get_extension_value(scope, name)
        },
    ));
}

/// Populate send-safe canvas callbacks (toDataURL, getImageData, setSize).
/// These operate on raw pixel data, no V8 scope required.
/// Actual behavior continues via native globals (__canvas_to_data_url__ etc.)
/// and JS shim. BCR callbacks provide a future direct-injection path.
pub fn register_canvas_send_safe_callbacks(
    to_data_url: &SendSafeCallback,
    get_image_data: &SendSafeCallback,
    set_size: &SendSafeCallback,
) {
    // Simple base64 encoder (no external dependency)
    fn simple_base64_encode(data: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut out = String::new();
        for chunk in data.chunks(3) {
            let b0 = chunk[0] as u32;
            let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
            let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
            let n = (b0 << 16) | (b1 << 8) | b2;
            out.push(CHARS[((n >> 18) & 63) as usize] as char);
            out.push(CHARS[((n >> 12) & 63) as usize] as char);
            if chunk.len() > 1 {
                out.push(CHARS[((n >> 6) & 63) as usize] as char);
            } else {
                out.push('=');
            }
            if chunk.len() > 2 {
                out.push(CHARS[(n & 63) as usize] as char);
            } else {
                out.push('=');
            }
        }
        out
    }

    *to_data_url.borrow_mut() = Some(Box::new(|pixmap_bytes: Vec<u8>| {
        if pixmap_bytes.is_empty() {
            Ok("data:image/png;base64,".to_string())
        } else {
            Ok(format!(
                "data:image/png;base64,{}",
                simple_base64_encode(&pixmap_bytes)
            ))
        }
    }));

    *get_image_data.borrow_mut() = Some(Box::new(|pixmap_bytes: Vec<u8>| {
        // Return RGBA pixel values as JSON array: [[r,g,b,a], ...]
        let pixels: Vec<String> = pixmap_bytes
            .chunks(4)
            .map(|c| format!("[{},{},{},{}]", c[0], c[1], c[2], c.get(3).copied().unwrap_or(255)))
            .collect();
        Ok(format!("[{}]", pixels.join(",")))
    }));

    *set_size.borrow_mut() = Some(Box::new(|_pixmap_bytes: Vec<u8>| {
        Ok("ok".to_string())
    }));
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
pub mod navigator;
pub mod verification;
pub mod webgl;
