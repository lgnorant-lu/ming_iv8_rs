//! Canvas 2D native bindings: bridge JS canvas operations to Rust Canvas2D.
//!
//! Installs native callbacks:
//! - __canvas_cmd__(id, cmd, ...args) → execute a draw command on canvas `id`
//! - __canvas_to_data_url__(id, type, quality) → render canvas to data URL
//! - __canvas_get_image_data__(id, x, y, w, h) → get pixel data as JSON array
//! - __canvas_set_size__(id, width, height) → set canvas dimensions

use crate::canvas::canvas2d::{base64_encode, Canvas2D, DrawCmd};
use crate::state::RuntimeState;

/// Install canvas native callbacks on the global object.
pub fn install_canvas_bindings(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    macro_rules! install_fn {
        ($name:literal, $cb:ident) => {
            let tmpl = v8::FunctionTemplate::builder_raw($cb).build(scope);
            let func = crate::v8_utils::v8_fn(scope, &*tmpl);
            let key = crate::v8_utils::v8_string(scope, $name);
            global.define_own_property(
                scope,
                key.into(),
                func.into(),
                v8::PropertyAttribute::DONT_ENUM,
            );
        };
    }

    install_fn!("__canvas_cmd__", canvas_cmd_callback);
    install_fn!("__canvas_to_data_url__", canvas_to_data_url_callback);
    install_fn!("__canvas_get_image_data__", canvas_get_image_data_callback);
    install_fn!("__canvas_set_size__", canvas_set_size_callback);
}

/// JS shim that replaces __getCanvasContext__ with a full Canvas2D implementation.
pub const CANVAS2D_SHIM_JS: &str = r#"
(function() {
    // Replace the stub __getCanvasContext__ with a real Canvas2D implementation
    // that delegates draw operations to the Rust backend.
    //
    // Strategy: patch HTMLCanvasElement.prototype.getContext and toDataURL
    // to use the Rust canvas backend. We do NOT override document.createElement
    // because page_load re-installs DOM bindings and would undo the patch.
    // Instead, we use lazy initialization: assign __canvasId__ on first getContext call.

    window.__getCanvasContext__ = function(canvasId, type) {
        if (type === '2d') {
            return createCanvas2DContext(canvasId);
        }
        if (type === 'webgl' || type === 'experimental-webgl' ||
            type === 'webgl2' || type === 'experimental-webgl2') {
            return window.__webglContext__ || null;
        }
        return null;
    };

    // Helper: get or create canvas ID for an element
    function getOrCreateCanvasId(el) {
        if (!el.__canvasId__) {
            el.__canvasId__ = '__canvas_' + Math.random().toString(36).slice(2) + '__';
            __canvas_set_size__(el.__canvasId__, el.width || 300, el.height || 150);
        }
        return el.__canvasId__;
    }

    // Patch HTMLCanvasElement.prototype if available
    if (typeof HTMLCanvasElement !== 'undefined') {
        HTMLCanvasElement.prototype.getContext = function(type, attrs) {
            var id = getOrCreateCanvasId(this);
            __canvas_set_size__(id, this.width || 300, this.height || 150);
            var ctx = window.__getCanvasContext__(id, type);
            if (ctx) ctx.canvas = this;
            return ctx;
        };
        Object.defineProperty(HTMLCanvasElement.prototype.getContext, 'length', {value: 1});

        HTMLCanvasElement.prototype.toDataURL = function(type, quality) {
            var id = getOrCreateCanvasId(this);
            return __canvas_to_data_url__(id, type || 'image/png', quality || 0.92);
        };
        Object.defineProperty(HTMLCanvasElement.prototype.toDataURL, 'length', {value: 0});
    }

    // Also install a global helper that patches any canvas element on demand
    // This is called by the DOM binding when a canvas element is accessed
    window.__patchCanvasElement__ = function(el) {
        if (!el || el.tagName !== 'CANVAS') return;
        if (el.__canvasPatchedByIv8__) return;
        el.__canvasPatchedByIv8__ = true;
        var id = getOrCreateCanvasId(el);
        el.getContext = function(type, attrs) {
            __canvas_set_size__(id, this.width || 300, this.height || 150);
            var ctx = window.__getCanvasContext__(id, type);
            if (ctx) ctx.canvas = this;
            return ctx;
        };
        el.toDataURL = function(type, quality) {
            return __canvas_to_data_url__(id, type || 'image/png', quality || 0.92);
        };
    };

    function createCanvas2DContext(canvasId) {
        var state = {
            fillStyle: '#000000',
            strokeStyle: '#000000',
            lineWidth: 1,
            font: '10px sans-serif',
            textAlign: 'start',
            textBaseline: 'alphabetic',
            globalAlpha: 1,
            globalCompositeOperation: 'source-over',
            shadowBlur: 0,
            shadowColor: 'rgba(0,0,0,0)',
            shadowOffsetX: 0,
            shadowOffsetY: 0,
            lineCap: 'butt',
            lineJoin: 'miter',
            miterLimit: 10,
            imageSmoothingEnabled: true,
        };
        var _stateStack = [];

        function cmd() {
            var args = Array.prototype.slice.call(arguments);
            args.unshift(canvasId);
            __canvas_cmd__.apply(null, args);
        }

        // Use CanvasRenderingContext2D.prototype as __proto__ if available
        // so idlharness assert_inherits checks pass.
        var proto = (typeof CanvasRenderingContext2D !== 'undefined' && CanvasRenderingContext2D.prototype)
            ? CanvasRenderingContext2D.prototype : null;
        var ctx = {
            get fillStyle() { return state.fillStyle; },
            set fillStyle(v) { state.fillStyle = v; },
            get strokeStyle() { return state.strokeStyle; },
            set strokeStyle(v) { state.strokeStyle = v; },
            get lineWidth() { return state.lineWidth; },
            set lineWidth(v) { state.lineWidth = v; },
            get font() { return state.font; },
            set font(v) { state.font = v; },
            get textAlign() { return state.textAlign; },
            set textAlign(v) { state.textAlign = v; },
            get textBaseline() { return state.textBaseline; },
            set textBaseline(v) { state.textBaseline = v; },
            get globalAlpha() { return state.globalAlpha; },
            set globalAlpha(v) { state.globalAlpha = v; },
            get globalCompositeOperation() { return state.globalCompositeOperation; },
            set globalCompositeOperation(v) { state.globalCompositeOperation = v; },
            get shadowBlur() { return state.shadowBlur; },
            set shadowBlur(v) { state.shadowBlur = v; },
            get shadowColor() { return state.shadowColor; },
            set shadowColor(v) { state.shadowColor = v; },
            get shadowOffsetX() { return state.shadowOffsetX; },
            set shadowOffsetX(v) { state.shadowOffsetX = v; },
            get shadowOffsetY() { return state.shadowOffsetY; },
            set shadowOffsetY(v) { state.shadowOffsetY = v; },
            get lineCap() { return state.lineCap; },
            set lineCap(v) { state.lineCap = v; },
            get lineJoin() { return state.lineJoin; },
            set lineJoin(v) { state.lineJoin = v; },
            get miterLimit() { return state.miterLimit; },
            set miterLimit(v) { state.miterLimit = v; },
            get imageSmoothingEnabled() { return state.imageSmoothingEnabled; },
            set imageSmoothingEnabled(v) { state.imageSmoothingEnabled = v; },

            // Draw operations
            fillRect: function(x, y, w, h) {
                cmd('fillRect', x, y, w, h, state.fillStyle, state.globalAlpha);
            },
            strokeRect: function(x, y, w, h) {
                cmd('strokeRect', x, y, w, h, state.strokeStyle, state.lineWidth, state.globalAlpha);
            },
            clearRect: function(x, y, w, h) {
                cmd('clearRect', x, y, w, h);
            },
            fillText: function(text, x, y, maxWidth) {
                cmd('fillText', String(text), x, y, state.fillStyle, state.font, state.globalAlpha);
            },
            strokeText: function(text, x, y, maxWidth) {
                cmd('strokeText', String(text), x, y, state.strokeStyle, state.font, state.lineWidth);
            },

            // Path operations
            beginPath: function() { cmd('beginPath'); },
            closePath: function() { cmd('closePath'); },
            moveTo: function(x, y) { cmd('moveTo', x, y); },
            lineTo: function(x, y) { cmd('lineTo', x, y); },
            arc: function(x, y, r, start, end, ccw) {
                cmd('arc', x, y, r, start, end, ccw ? 1 : 0, state.fillStyle, state.strokeStyle);
            },
            arcTo: function(x1, y1, x2, y2, r) { cmd('arcTo', x1, y1, x2, y2, r); },
            bezierCurveTo: function(cp1x, cp1y, cp2x, cp2y, x, y) {
                cmd('bezierCurveTo', cp1x, cp1y, cp2x, cp2y, x, y);
            },
            quadraticCurveTo: function(cpx, cpy, x, y) {
                cmd('quadraticCurveTo', cpx, cpy, x, y);
            },
            rect: function(x, y, w, h) { cmd('rect', x, y, w, h); },
            fill: function(rule) { cmd('fill', state.fillStyle, state.globalAlpha); },
            stroke: function() { cmd('stroke', state.strokeStyle, state.lineWidth, state.globalAlpha); },
            clip: function() { cmd('clip'); },

            // Transform
            save: function() {
                // Save current state to stack
                _stateStack.push({
                    fillStyle: state.fillStyle,
                    strokeStyle: state.strokeStyle,
                    lineWidth: state.lineWidth,
                    font: state.font,
                    textAlign: state.textAlign,
                    textBaseline: state.textBaseline,
                    globalAlpha: state.globalAlpha,
                    globalCompositeOperation: state.globalCompositeOperation,
                    shadowBlur: state.shadowBlur,
                    shadowColor: state.shadowColor,
                    shadowOffsetX: state.shadowOffsetX,
                    shadowOffsetY: state.shadowOffsetY,
                    lineCap: state.lineCap,
                    lineJoin: state.lineJoin,
                    miterLimit: state.miterLimit,
                    imageSmoothingEnabled: state.imageSmoothingEnabled,
                });
                cmd('save');
            },
            restore: function() {
                // Restore state from stack
                if (_stateStack.length > 0) {
                    var saved = _stateStack.pop();
                    for (var k in saved) { state[k] = saved[k]; }
                }
                cmd('restore');
            },
            scale: function(x, y) { cmd('scale', x, y); },
            rotate: function(angle) { cmd('rotate', angle); },
            translate: function(x, y) { cmd('translate', x, y); },
            transform: function(a, b, c, d, e, f) { cmd('transform', a, b, c, d, e, f); },
            setTransform: function(a, b, c, d, e, f) { cmd('setTransform', a, b, c, d, e, f); },
            resetTransform: function() { cmd('setTransform', 1, 0, 0, 1, 0, 0); },

            // Image data
            createImageData: function(w, h) {
                return {width: w, height: h, data: new Uint8ClampedArray(w*h*4)};
            },
            getImageData: function(x, y, w, h) {
                var raw = __canvas_get_image_data__(canvasId, x, y, w, h);
                var data = new Uint8ClampedArray(raw);
                return {width: w, height: h, data: data};
            },
            putImageData: function(imageData, x, y) {
                cmd('putImageData', x, y, Array.from(imageData.data), imageData.width, imageData.height);
            },

            // Gradients/patterns (stubs)
            createLinearGradient: function() { return {addColorStop: function(){}}; },
            createRadialGradient: function() { return {addColorStop: function(){}}; },
            createPattern: function() { return null; },

            // Other
            drawImage: function() {},
            getLineDash: function() { return []; },
            setLineDash: function() {},
            isPointInPath: function() { return false; },
            isPointInStroke: function() { return false; },

            // measureText: font-aware width estimation
            measureText: function(text) {
                var font = state.font || '10px sans-serif';
                var sizeMatch = font.match(/(\d+(?:\.\d+)?)(px|pt|em)/);
                var fontSize = sizeMatch ? parseFloat(sizeMatch[1]) : 10;
                if (sizeMatch && sizeMatch[2] === 'pt') fontSize *= 1.333;
                // Check if the font family is in the profile's font list
                var fontPrefs = (typeof globalThis.__iv8FontPrefs === 'object' && globalThis.__iv8FontPrefs) ? globalThis.__iv8FontPrefs : {};
                var families = fontPrefs.families || [];
                var fontMatch = font.match(/["']?([^"']+)["']?\s*$/);
                var fontName = fontMatch ? fontMatch[1].toLowerCase() : '';
                var isKnown = families.some(function(f) { return f.toLowerCase() === fontName; });
                var isMonospace = /monospace|courier|mono/i.test(font) || (isKnown && /consolas|menlo|monaco|lucida/i.test(fontName));
                var isSerif = /serif/i.test(font) && !/sans-serif/i.test(font) || (isKnown && /times|georgia|garamond/i.test(fontName));
                var charWidth = isMonospace ? fontSize * 0.6 : isSerif ? fontSize * 0.55 : fontSize * 0.5;
                var width = (text || '').length * charWidth;
                return {
                    width: width,
                    actualBoundingBoxAscent: fontSize * 0.8,
                    actualBoundingBoxDescent: fontSize * 0.2,
                    actualBoundingBoxLeft: 0,
                    actualBoundingBoxRight: width,
                    fontBoundingBoxAscent: fontSize,
                    fontBoundingBoxDescent: fontSize * 0.25,
                };
            },

            canvas: null,  // Set by HTMLCanvasElement
        };

        // Set __proto__ to CanvasRenderingContext2D.prototype if available
        // so idlharness assert_inherits checks pass.
        // Then delete own properties that also exist on the prototype,
        // so assert_inherits finds them in the prototype chain (not on instance).
        if (proto) {
            try {
                Object.setPrototypeOf(ctx, proto);
                // Delete own properties that shadow prototype methods/attributes.
                // idlharness assert_inherits requires properties to NOT be own
                // properties but to exist in the prototype chain.
                var protoNames = Object.getOwnPropertyNames(proto);
                for (var pi = 0; pi < protoNames.length; pi++) {
                    var pn = protoNames[pi];
                    if (pn === 'constructor') continue;
                    if (ctx.hasOwnProperty(pn)) {
                        try { delete ctx[pn]; } catch(e) {}
                    }
                }
            } catch(e) {}
        }

        return ctx;
    }

    // Patch HTMLCanvasElement.prototype.getContext
    if (typeof HTMLCanvasElement !== 'undefined') {
        HTMLCanvasElement.prototype.getContext = function(type, attrs) {
            var id = this.__canvasId__;
            if (!id) {
                id = '__canvas_' + Math.random().toString(36).slice(2) + '__';
                this.__canvasId__ = id;
                // Register canvas with Rust backend
                __canvas_set_size__(id, this.width || 300, this.height || 150);
            }
            var ctx = window.__getCanvasContext__(id, type);
            if (ctx) ctx.canvas = this;
            return ctx;
        };

        HTMLCanvasElement.prototype.toDataURL = function(type, quality) {
            var id = this.__canvasId__;
            if (!id) {
                // Initialize canvas ID and register with Rust backend
                id = '__canvas_' + Math.random().toString(36).slice(2) + '__';
                this.__canvasId__ = id;
                __canvas_set_size__(id, this.width || 300, this.height || 150);
            }
            return __canvas_to_data_url__(id, type || 'image/png', quality || 0.92);
            return __canvas_to_data_url__(id, type || 'image/png', quality || 0.92);
        };

        // Override width/height setters to update Rust backend
        var _origWidthDesc = Object.getOwnPropertyDescriptor(HTMLCanvasElement.prototype, 'width');
        var _origHeightDesc = Object.getOwnPropertyDescriptor(HTMLCanvasElement.prototype, 'height');
    }

})();
"#;
// ─── Native callbacks ─────────────────────────────────────────────────────────

/// __canvas_cmd__(id, cmd_name, ...args) → execute a draw command
unsafe extern "C" fn canvas_cmd_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 2 {
            return;
        }

        let canvas_id = args.get(0).to_rust_string_lossy(scope);
        let cmd_name = args.get(1).to_rust_string_lossy(scope);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let mut canvases = state.canvases.borrow_mut();

        let canvas = canvases
            .entry(canvas_id)
            .or_insert_with(|| Canvas2D::new(300, 150));

        // Parse command
        let cmd = match cmd_name.as_str() {
            "fillRect" => {
                if args.length() < 6 {
                    return;
                }
                let x = args.get(2).number_value(scope).unwrap_or(0.0) as f32;
                let y = args.get(3).number_value(scope).unwrap_or(0.0) as f32;
                let w = args.get(4).number_value(scope).unwrap_or(0.0) as f32;
                let h = args.get(5).number_value(scope).unwrap_or(0.0) as f32;
                let color_str = if args.length() > 6 {
                    args.get(6).to_rust_string_lossy(scope)
                } else {
                    "#000000".to_string()
                };
                let alpha = if args.length() > 7 {
                    args.get(7).number_value(scope).unwrap_or(1.0) as f32
                } else {
                    1.0
                };
                let mut color = Canvas2D::parse_color(&color_str);
                color[3] = (color[3] as f32 * alpha) as u8;
                Some(DrawCmd::FillRect { x, y, w, h, color })
            }
            "strokeRect" => {
                if args.length() < 6 {
                    return;
                }
                let x = args.get(2).number_value(scope).unwrap_or(0.0) as f32;
                let y = args.get(3).number_value(scope).unwrap_or(0.0) as f32;
                let w = args.get(4).number_value(scope).unwrap_or(0.0) as f32;
                let h = args.get(5).number_value(scope).unwrap_or(0.0) as f32;
                let color_str = if args.length() > 6 {
                    args.get(6).to_rust_string_lossy(scope)
                } else {
                    "#000000".to_string()
                };
                let line_width = if args.length() > 7 {
                    args.get(7).number_value(scope).unwrap_or(1.0) as f32
                } else {
                    1.0
                };
                let color = Canvas2D::parse_color(&color_str);
                Some(DrawCmd::StrokeRect {
                    x,
                    y,
                    w,
                    h,
                    color,
                    line_width,
                })
            }
            "clearRect" => {
                if args.length() < 6 {
                    return;
                }
                let x = args.get(2).number_value(scope).unwrap_or(0.0) as f32;
                let y = args.get(3).number_value(scope).unwrap_or(0.0) as f32;
                let w = args.get(4).number_value(scope).unwrap_or(0.0) as f32;
                let h = args.get(5).number_value(scope).unwrap_or(0.0) as f32;
                Some(DrawCmd::ClearRect { x, y, w, h })
            }
            "fillText" => {
                if args.length() < 6 {
                    return;
                }
                let text = args.get(2).to_rust_string_lossy(scope);
                let x = args.get(3).number_value(scope).unwrap_or(0.0) as f32;
                let y = args.get(4).number_value(scope).unwrap_or(0.0) as f32;
                let color_str = if args.length() > 5 {
                    args.get(5).to_rust_string_lossy(scope)
                } else {
                    "#000000".to_string()
                };
                let font_str = if args.length() > 6 {
                    args.get(6).to_rust_string_lossy(scope)
                } else {
                    "10px sans-serif".to_string()
                };
                let color = Canvas2D::parse_color(&color_str);
                let font_size = Canvas2D::parse_font_size(&font_str);
                Some(DrawCmd::FillText {
                    text,
                    x,
                    y,
                    color,
                    font_size,
                })
            }
            "beginPath" => Some(DrawCmd::BeginPath),
            "closePath" => Some(DrawCmd::ClosePath),
            "moveTo" => {
                if args.length() < 4 {
                    return;
                }
                let x = args.get(2).number_value(scope).unwrap_or(0.0) as f32;
                let y = args.get(3).number_value(scope).unwrap_or(0.0) as f32;
                Some(DrawCmd::MoveTo { x, y })
            }
            "lineTo" => {
                if args.length() < 4 {
                    return;
                }
                let x = args.get(2).number_value(scope).unwrap_or(0.0) as f32;
                let y = args.get(3).number_value(scope).unwrap_or(0.0) as f32;
                Some(DrawCmd::LineTo { x, y })
            }
            "arc" => {
                if args.length() < 7 {
                    return;
                }
                let x = args.get(2).number_value(scope).unwrap_or(0.0) as f32;
                let y = args.get(3).number_value(scope).unwrap_or(0.0) as f32;
                let r = args.get(4).number_value(scope).unwrap_or(0.0) as f32;
                let start = args.get(5).number_value(scope).unwrap_or(0.0) as f32;
                let end = args.get(6).number_value(scope).unwrap_or(0.0) as f32;
                let color_str = if args.length() > 8 {
                    args.get(8).to_rust_string_lossy(scope)
                } else {
                    "#000000".to_string()
                };
                let color = Canvas2D::parse_color(&color_str);
                Some(DrawCmd::Arc {
                    x,
                    y,
                    r,
                    start,
                    end,
                    color,
                    fill: true,
                })
            }
            "fill" => {
                let color_str = if args.length() > 2 {
                    args.get(2).to_rust_string_lossy(scope)
                } else {
                    "#000000".to_string()
                };
                let color = Canvas2D::parse_color(&color_str);
                Some(DrawCmd::Fill { color })
            }
            "stroke" => {
                let color_str = if args.length() > 2 {
                    args.get(2).to_rust_string_lossy(scope)
                } else {
                    "#000000".to_string()
                };
                let line_width = if args.length() > 3 {
                    args.get(3).number_value(scope).unwrap_or(1.0) as f32
                } else {
                    1.0
                };
                let color = Canvas2D::parse_color(&color_str);
                Some(DrawCmd::Stroke { color, line_width })
            }
            "save" => Some(DrawCmd::Save),
            "restore" => Some(DrawCmd::Restore),
            "translate" => {
                if args.length() < 4 {
                    return;
                }
                let x = args.get(2).number_value(scope).unwrap_or(0.0) as f32;
                let y = args.get(3).number_value(scope).unwrap_or(0.0) as f32;
                Some(DrawCmd::Translate { x, y })
            }
            "scale" => {
                if args.length() < 4 {
                    return;
                }
                let x = args.get(2).number_value(scope).unwrap_or(1.0) as f32;
                let y = args.get(3).number_value(scope).unwrap_or(1.0) as f32;
                Some(DrawCmd::Scale { x, y })
            }
            "rotate" => {
                if args.length() < 3 {
                    return;
                }
                let angle = args.get(2).number_value(scope).unwrap_or(0.0) as f32;
                Some(DrawCmd::Rotate { angle })
            }
            "setTransform" => {
                if args.length() < 8 {
                    return;
                }
                let a = args.get(2).number_value(scope).unwrap_or(1.0) as f32;
                let b = args.get(3).number_value(scope).unwrap_or(0.0) as f32;
                let c = args.get(4).number_value(scope).unwrap_or(0.0) as f32;
                let d = args.get(5).number_value(scope).unwrap_or(1.0) as f32;
                let e = args.get(6).number_value(scope).unwrap_or(0.0) as f32;
                let f = args.get(7).number_value(scope).unwrap_or(0.0) as f32;
                Some(DrawCmd::SetTransform { a, b, c, d, e, f })
            }
            _ => None,
        };

        if let Some(cmd) = cmd {
            // Update path state for path commands
            match &cmd {
                DrawCmd::BeginPath => {
                    canvas.path_points.clear();
                    canvas.path_started = true;
                }
                DrawCmd::MoveTo { x, y } => {
                    canvas.path_points.clear();
                    canvas.path_points.push((*x, *y));
                }
                DrawCmd::LineTo { x, y } => {
                    canvas.path_points.push((*x, *y));
                }
                _ => {}
            }
            canvas.commands.push(cmd);
        }
    }));
}

/// __canvas_to_data_url__(id, type, quality) → data URL string
unsafe extern "C" fn canvas_to_data_url_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let canvas_id = if args.length() > 0 {
            args.get(0).to_rust_string_lossy(scope)
        } else {
            return;
        };
        let mime_type = if args.length() > 1 {
            args.get(1).to_rust_string_lossy(scope)
        } else {
            "image/png".to_string()
        };

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);

        // Check for fixed fingerprint in environment
        let fixed_fp = {
            let env = &state.environment;
            let key = if mime_type.contains("jpeg") || mime_type.contains("jpg") {
                "canvas.fingerprint.toDataURL.jpeg"
            } else {
                "canvas.fingerprint.toDataURL.png"
            };
            env.get_str(key).map(|s| s.to_string())
        };

        if let Some(fp) = fixed_fp {
            if !fp.is_empty() {
                if let Some(s) = v8::String::new(scope, &fp) {
                    rv.set(s.into());
                    return;
                }
            }
        }

        // Render with tiny-skia
        let canvases = state.canvases.borrow();
        let data_url = if let Some(canvas) = canvases.get(&canvas_id) {
            // Check noise intensity from environment (0.0 = no noise, 1.0 = full noise)
            let noise_intensity = {
                let env = &state.environment;
                env.get_f64("canvas.noise.intensity").unwrap_or(1.0)
            };
            let noise_seed = if noise_intensity > 0.0 {
                // Use canvas ID as noise seed for deterministic but unique fingerprint
                let seed: u64 = canvas_id
                    .bytes()
                    .fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
                Some(seed)
            } else {
                None
            };
            let png_data = canvas.to_png(noise_seed);
            if png_data.is_empty() {
                "data:image/png;base64,".to_string()
            } else {
                format!("data:image/png;base64,{}", base64_encode(&png_data))
            }
        } else {
            "data:image/png;base64,".to_string()
        };

        if let Some(s) = v8::String::new(scope, &data_url) {
            rv.set(s.into());
        }
    }));
}

/// __canvas_get_image_data__(id, x, y, w, h) → Uint8Array-compatible array
unsafe extern "C" fn canvas_get_image_data_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 5 {
            return;
        }

        let canvas_id = args.get(0).to_rust_string_lossy(scope);
        let x = args.get(1).uint32_value(scope).unwrap_or(0);
        let y = args.get(2).uint32_value(scope).unwrap_or(0);
        let w = args.get(3).uint32_value(scope).unwrap_or(0);
        let h = args.get(4).uint32_value(scope).unwrap_or(0);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let canvases = state.canvases.borrow();

        let pixel_data = if let Some(canvas) = canvases.get(&canvas_id) {
            canvas.get_image_data(x, y, w, h)
        } else {
            vec![0u8; (w * h * 4) as usize]
        };

        // Return as V8 Array of numbers
        let arr = v8::Array::new(scope, pixel_data.len() as i32);
        for (i, &byte) in pixel_data.iter().enumerate() {
            arr.set_index(scope, i as u32, v8::Integer::new(scope, byte as i32).into());
        }
        rv.set(arr.into());
    }));
}

/// __canvas_set_size__(id, width, height) → set canvas dimensions
unsafe extern "C" fn canvas_set_size_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);

        if args.length() < 3 {
            return;
        }

        let canvas_id = args.get(0).to_rust_string_lossy(scope);
        let width = args.get(1).uint32_value(scope).unwrap_or(300).max(1);
        let height = args.get(2).uint32_value(scope).unwrap_or(150).max(1);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let mut canvases = state.canvases.borrow_mut();

        let canvas = canvases
            .entry(canvas_id)
            .or_insert_with(|| Canvas2D::new(width, height));
        if canvas.width != width || canvas.height != height {
            // Resize: create new canvas (clears content, matching browser behavior)
            *canvas = Canvas2D::new(width, height);
        }
    }));
}
