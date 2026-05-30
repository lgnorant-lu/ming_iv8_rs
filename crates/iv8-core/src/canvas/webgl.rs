//! WebGL parameter stubs: return realistic GPU/renderer info without real rendering.
//!
//! Anti-fingerprint scripts check WebGL parameters to detect headless/virtual environments.
//! We return values matching a real Chrome on Windows with a common GPU.
//!
//! Installed as: canvas.getContext('webgl').getParameter(...)
//! Also: WEBGL_debug_renderer_info extension parameters.

use crate::state::RuntimeState;

/// Install WebGL stubs on the global object.
/// Creates a minimal HTMLCanvasElement.prototype.getContext that returns a WebGL context stub.
pub fn install_webgl_stubs(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    // Install via JS shim that creates the WebGL context stub
    // The shim is evaluated after this function sets up the native getParameter callback
    let get_param_tmpl = v8::FunctionTemplate::builder_raw(webgl_get_parameter).build(scope);
    let get_param_fn = get_param_tmpl.get_function(scope).expect("fn");
    let key = v8::String::new(scope, "__webgl_getParameter__").expect("key");
    global.define_own_property(scope, key.into(), get_param_fn.into(), v8::PropertyAttribute::DONT_ENUM);

    let get_ext_tmpl = v8::FunctionTemplate::builder_raw(webgl_get_extension).build(scope);
    let get_ext_fn = get_ext_tmpl.get_function(scope).expect("fn");
    let ext_key = v8::String::new(scope, "__webgl_getExtension__").expect("key");
    global.define_own_property(scope, ext_key.into(), get_ext_fn.into(), v8::PropertyAttribute::DONT_ENUM);

    let get_supp_ext_tmpl = v8::FunctionTemplate::builder_raw(webgl_get_supported_extensions).build(scope);
    let get_supp_ext_fn = get_supp_ext_tmpl.get_function(scope).expect("fn");
    let supp_key = v8::String::new(scope, "__webgl_getSupportedExtensions__").expect("key");
    global.define_own_property(scope, supp_key.into(), get_supp_ext_fn.into(), v8::PropertyAttribute::DONT_ENUM);
}

/// JS shim that installs the WebGL context stub on HTMLCanvasElement.
pub const WEBGL_SHIM_JS: &str = r#"
(function() {
    var _getParameter = globalThis.__webgl_getParameter__;
    var _getExtension = globalThis.__webgl_getExtension__;
    var _getSupportedExtensions = globalThis.__webgl_getSupportedExtensions__;

    var WebGLContext = {
        getParameter: _getParameter,
        getExtension: _getExtension,
        getSupportedExtensions: _getSupportedExtensions,
        createBuffer: function() { return {}; },
        createProgram: function() { return {}; },
        createShader: function() { return {}; },
        shaderSource: function() {},
        compileShader: function() {},
        attachShader: function() {},
        linkProgram: function() {},
        useProgram: function() {},
        getShaderParameter: function() { return true; },
        getProgramParameter: function() { return true; },
        getAttribLocation: function() { return 0; },
        getUniformLocation: function() { return {}; },
        enableVertexAttribArray: function() {},
        vertexAttribPointer: function() {},
        bindBuffer: function() {},
        bufferData: function() {},
        drawArrays: function() {},
        drawElements: function() {},
        viewport: function() {},
        clearColor: function() {},
        clear: function() {},
        enable: function() {},
        disable: function() {},
        blendFunc: function() {},
        // Standard WebGL constants (pname values for getParameter)
        VENDOR: 0x1F00,
        RENDERER: 0x1F01,
        VERSION: 0x1F02,
        SHADING_LANGUAGE_VERSION: 0x8B8C,
        MAX_TEXTURE_SIZE: 0x0D33,
        MAX_RENDERBUFFER_SIZE: 0x84E8,
        MAX_VIEWPORT_DIMS: 0x0D3A,
        MAX_VERTEX_ATTRIBS: 0x8869,
        MAX_VERTEX_UNIFORM_VECTORS: 0x8DFB,
        MAX_FRAGMENT_UNIFORM_VECTORS: 0x8DFD,
        MAX_VARYING_VECTORS: 0x8DFC,
        MAX_TEXTURE_IMAGE_UNITS: 0x8872,
        MAX_COMBINED_TEXTURE_IMAGE_UNITS: 0x8B4D,
        MAX_CUBE_MAP_TEXTURE_SIZE: 0x851C,
        ALIASED_LINE_WIDTH_RANGE: 0x846E,
        ALIASED_POINT_SIZE_RANGE: 0x846D,
        // Draw/buffer constants
        VERTEX_SHADER: 35633,
        FRAGMENT_SHADER: 35632,
        ARRAY_BUFFER: 34962,
        STATIC_DRAW: 35044,
        FLOAT: 5126,
        TRIANGLES: 4,
        COLOR_BUFFER_BIT: 16384,
        DEPTH_BUFFER_BIT: 256,
        STENCIL_BUFFER_BIT: 1024,
        DEPTH_TEST: 2929,
        BLEND: 3042,
        CULL_FACE: 2884,
        TEXTURE_2D: 3553,
        RGBA: 6408,
        UNSIGNED_BYTE: 5121,
        NEAREST: 9728,
        LINEAR: 9729,
        TEXTURE_MIN_FILTER: 10241,
        TEXTURE_MAG_FILTER: 10240,
        TEXTURE_WRAP_S: 10242,
        TEXTURE_WRAP_T: 10243,
        CLAMP_TO_EDGE: 33071,
        FRAMEBUFFER: 36160,
        RENDERBUFFER: 36161,
        DEPTH_COMPONENT16: 33189,
        COLOR_ATTACHMENT0: 36064,
        DEPTH_ATTACHMENT: 36096,
        FRAMEBUFFER_COMPLETE: 36053,
        LINK_STATUS: 35714,
        COMPILE_STATUS: 35713,
        ELEMENT_ARRAY_BUFFER: 34963,
        UNSIGNED_SHORT: 5123,
        LINES: 1,
        LINE_STRIP: 3,
        TRIANGLE_STRIP: 5,
        TRIANGLE_FAN: 6,
        POINTS: 0,
        SRC_ALPHA: 770,
        ONE_MINUS_SRC_ALPHA: 771,
        ONE: 1,
        ZERO: 0,
        FUNC_ADD: 32774,
        LEQUAL: 515,
        LESS: 513,
        EQUAL: 514,
        GREATER: 516,
        NOTEQUAL: 517,
        GEQUAL: 518,
        ALWAYS: 519,
        NEVER: 512,
        KEEP: 7680,
        REPLACE: 7681,
        INCR: 7682,
        DECR: 7683,
        INVERT: 5386,
        INCR_WRAP: 34055,
        DECR_WRAP: 34056,
        FRONT: 1028,
        BACK: 1029,
        FRONT_AND_BACK: 1032,
        CCW: 2305,
        CW: 2304,
        DYNAMIC_DRAW: 35048,
        STREAM_DRAW: 35040,
        INT: 5124,
        SHORT: 5122,
        BYTE: 5120,
        UNSIGNED_INT: 5125,
        FLOAT_VEC2: 35664,
        FLOAT_VEC3: 35665,
        FLOAT_VEC4: 35666,
        INT_VEC2: 35667,
        INT_VEC3: 35668,
        INT_VEC4: 35669,
        BOOL: 35670,
        FLOAT_MAT2: 35674,
        FLOAT_MAT3: 35675,
        FLOAT_MAT4: 35676,
        SAMPLER_2D: 35678,
        SAMPLER_CUBE: 35680,
        ACTIVE_ATTRIBUTES: 35721,
        ACTIVE_UNIFORMS: 35718,
        NO_ERROR: 0,
        INVALID_ENUM: 1280,
        INVALID_VALUE: 1281,
        INVALID_OPERATION: 1282,
        OUT_OF_MEMORY: 1285,
        CONTEXT_LOST_WEBGL: 37442,
        // Additional methods
        createTexture: function() { return {}; },
        bindTexture: function() {},
        texImage2D: function() {},
        texParameteri: function() {},
        generateMipmap: function() {},
        createFramebuffer: function() { return {}; },
        bindFramebuffer: function() {},
        framebufferTexture2D: function() {},
        createRenderbuffer: function() { return {}; },
        bindRenderbuffer: function() {},
        renderbufferStorage: function() {},
        framebufferRenderbuffer: function() {},
        checkFramebufferStatus: function() { return 36053; }, // FRAMEBUFFER_COMPLETE
        deleteBuffer: function() {},
        deleteTexture: function() {},
        deleteFramebuffer: function() {},
        deleteRenderbuffer: function() {},
        deleteShader: function() {},
        deleteProgram: function() {},
        getError: function() { return 0; }, // NO_ERROR
        isContextLost: function() { return false; },
        flush: function() {},
        finish: function() {},
        readPixels: function() {},
        pixelStorei: function() {},
        activeTexture: function() {},
        uniform1f: function() {}, uniform2f: function() {}, uniform3f: function() {}, uniform4f: function() {},
        uniform1i: function() {}, uniform2i: function() {}, uniform3i: function() {}, uniform4i: function() {},
        uniform1fv: function() {}, uniform2fv: function() {}, uniform3fv: function() {}, uniform4fv: function() {},
        uniform1iv: function() {}, uniform2iv: function() {}, uniform3iv: function() {}, uniform4iv: function() {},
        uniformMatrix2fv: function() {}, uniformMatrix3fv: function() {}, uniformMatrix4fv: function() {},
        vertexAttrib1f: function() {}, vertexAttrib2f: function() {}, vertexAttrib3f: function() {}, vertexAttrib4f: function() {},
        disableVertexAttribArray: function() {},
        scissor: function() {},
        colorMask: function() {},
        depthMask: function() {},
        depthFunc: function() {},
        depthRange: function() {},
        stencilFunc: function() {},
        stencilOp: function() {},
        stencilMask: function() {},
        cullFace: function() {},
        frontFace: function() {},
        lineWidth: function() {},
        polygonOffset: function() {},
        sampleCoverage: function() {},
        blendColor: function() {},
        blendEquation: function() {},
        blendEquationSeparate: function() {},
        blendFuncSeparate: function() {},
        getActiveAttrib: function() { return null; },
        getActiveUniform: function() { return null; },
        getAttachedShaders: function() { return []; },
        getBufferParameter: function() { return null; },
        getContextAttributes: function() { return {alpha:true,antialias:true,depth:true,failIfMajorPerformanceCaveat:false,powerPreference:'default',premultipliedAlpha:true,preserveDrawingBuffer:false,stencil:false,desynchronized:false}; },
        getFramebufferAttachmentParameter: function() { return null; },
        getProgramInfoLog: function() { return ''; },
        getRenderbufferParameter: function() { return null; },
        getShaderInfoLog: function() { return ''; },
        getShaderPrecisionFormat: function() { return {rangeMin:127,rangeMax:127,precision:23}; },
        getShaderSource: function() { return ''; },
        getTexParameter: function() { return null; },
        getUniform: function() { return null; },
        getVertexAttrib: function() { return null; },
        getVertexAttribOffset: function() { return 0; },
        hint: function() {},
        isBuffer: function() { return false; },
        isEnabled: function() { return false; },
        isFramebuffer: function() { return false; },
        isProgram: function() { return false; },
        isRenderbuffer: function() { return false; },
        isShader: function() { return false; },
        isTexture: function() { return false; },
        validateProgram: function() {},
        drawingBufferWidth: 300,
        drawingBufferHeight: 150,
        canvas: null,
    };

    // Patch document.createElement to return canvas with getContext
    var _origCreateElement = document ? document.createElement : null;
    if (typeof document !== 'undefined' && document.createElement) {
        var origCreate = document.createElement.bind ? document.createElement.bind(document) : document.createElement;
        // We can't easily override createElement here since it's a native function.
        // Instead, provide a global getContext helper.
    }

    // Install on globalThis for direct access
    globalThis.__webglContext__ = WebGLContext;

    // Install __iv8__.gl.callLog (REQ-WGL-002)
    // Records all WebGL API calls for fingerprint analysis
    (function() {
        var callLog = [];
        var iv8 = globalThis.__iv8__;
        if (!iv8) return;
        if (!iv8.gl) iv8.gl = {};
        iv8.gl.callLog = callLog;
        iv8.gl.clearCallLog = function() { callLog.length = 0; };

        // Wrap all WebGL methods to record calls
        var methodsToLog = [
            'getParameter', 'getExtension', 'getSupportedExtensions',
            'createBuffer', 'createProgram', 'createShader', 'shaderSource',
            'compileShader', 'attachShader', 'linkProgram', 'useProgram',
            'getShaderParameter', 'getProgramParameter', 'getAttribLocation',
            'getUniformLocation', 'enableVertexAttribArray', 'vertexAttribPointer',
            'bindBuffer', 'bufferData', 'drawArrays', 'drawElements',
            'viewport', 'clearColor', 'clear', 'enable', 'disable', 'blendFunc',
            'createTexture', 'bindTexture', 'texImage2D', 'texParameteri',
            'generateMipmap', 'createFramebuffer', 'bindFramebuffer',
            'framebufferTexture2D', 'getError', 'readPixels',
        ];
        methodsToLog.forEach(function(name) {
            var orig = WebGLContext[name];
            if (typeof orig !== 'function') return;
            WebGLContext[name] = function() {
                var args = Array.prototype.slice.call(arguments);
                var result = orig.apply(this, args);
                callLog.push({
                    method: name,
                    args: args.map(function(a) {
                        if (a === null || a === undefined) return null;
                        if (typeof a === 'object') return '[object]';
                        return a;
                    }),
                    result: result === undefined ? null : (typeof result === 'object' ? '[object]' : result),
                    timestamp: typeof performance !== 'undefined' ? performance.now() : 0,
                });
                return result;
            };
        });
    })();
})();
"#;

// ─── WebGL Constants ────────────────────────────────────────────────────────

// WEBGL_debug_renderer_info extension constants
const UNMASKED_VENDOR_WEBGL: u32 = 0x9245;
const UNMASKED_RENDERER_WEBGL: u32 = 0x9246;

// Standard WebGL parameter constants
const GL_VENDOR: u32 = 0x1F00;
const GL_RENDERER: u32 = 0x1F01;
const GL_VERSION: u32 = 0x1F02;
const GL_SHADING_LANGUAGE_VERSION: u32 = 0x8B8C;
const GL_MAX_TEXTURE_SIZE: u32 = 0x0D33;
const GL_MAX_RENDERBUFFER_SIZE: u32 = 0x84E8;
const GL_MAX_VIEWPORT_DIMS: u32 = 0x0D3A;
const GL_MAX_VERTEX_ATTRIBS: u32 = 0x8869;
const GL_MAX_VERTEX_UNIFORM_VECTORS: u32 = 0x8DFB;
const GL_MAX_FRAGMENT_UNIFORM_VECTORS: u32 = 0x8DFD;
const GL_MAX_VARYING_VECTORS: u32 = 0x8DFC;
const GL_MAX_TEXTURE_IMAGE_UNITS: u32 = 0x8872;
const GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS: u32 = 0x8B4D;
const GL_ALIASED_LINE_WIDTH_RANGE: u32 = 0x846E;
const GL_ALIASED_POINT_SIZE_RANGE: u32 = 0x846D;
const GL_MAX_CUBE_MAP_TEXTURE_SIZE: u32 = 0x851C;

/// getParameter(pname) → return realistic values for common WebGL parameters.
unsafe extern "C" fn webgl_get_parameter(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 { return; }
        let pname = args.get(0).uint32_value(scope).unwrap_or(0);

        let isolate: &v8::Isolate = &*scope;
        let state = RuntimeState::get(isolate);
        let env = &state.environment;

        // Read from environment if available, otherwise use defaults
        let vendor = env.get_str("webgl.vendor").unwrap_or("Google Inc. (NVIDIA)");
        let renderer = env.get_str("webgl.renderer").unwrap_or("ANGLE (NVIDIA, NVIDIA GeForce GTX 1080 Direct3D11 vs_5_0 ps_5_0, D3D11)");

        match pname {
            GL_VENDOR | UNMASKED_VENDOR_WEBGL => {
                if let Some(s) = v8::String::new(scope, vendor) { rv.set(s.into()); }
            }
            GL_RENDERER | UNMASKED_RENDERER_WEBGL => {
                if let Some(s) = v8::String::new(scope, renderer) { rv.set(s.into()); }
            }
            GL_VERSION => {
                if let Some(s) = v8::String::new(scope, "WebGL 1.0 (OpenGL ES 2.0 Chromium)") { rv.set(s.into()); }
            }
            GL_SHADING_LANGUAGE_VERSION => {
                if let Some(s) = v8::String::new(scope, "WebGL GLSL ES 1.0 (OpenGL ES GLSL ES 1.0 Chromium)") { rv.set(s.into()); }
            }
            GL_MAX_TEXTURE_SIZE => { rv.set(v8::Integer::new(scope, 16384).into()); }
            GL_MAX_RENDERBUFFER_SIZE => { rv.set(v8::Integer::new(scope, 16384).into()); }
            GL_MAX_VERTEX_ATTRIBS => { rv.set(v8::Integer::new(scope, 16).into()); }
            GL_MAX_VERTEX_UNIFORM_VECTORS => { rv.set(v8::Integer::new(scope, 4096).into()); }
            GL_MAX_FRAGMENT_UNIFORM_VECTORS => { rv.set(v8::Integer::new(scope, 1024).into()); }
            GL_MAX_VARYING_VECTORS => { rv.set(v8::Integer::new(scope, 30).into()); }
            GL_MAX_TEXTURE_IMAGE_UNITS => { rv.set(v8::Integer::new(scope, 16).into()); }
            GL_MAX_COMBINED_TEXTURE_IMAGE_UNITS => { rv.set(v8::Integer::new(scope, 32).into()); }
            GL_MAX_CUBE_MAP_TEXTURE_SIZE => { rv.set(v8::Integer::new(scope, 16384).into()); }
            GL_MAX_VIEWPORT_DIMS | GL_ALIASED_LINE_WIDTH_RANGE | GL_ALIASED_POINT_SIZE_RANGE => {
                // Return a Float32Array [1, max]
                let arr = v8::Array::new(scope, 2);
                arr.set_index(scope, 0, v8::Number::new(scope, 1.0).into());
                arr.set_index(scope, 1, v8::Number::new(scope, 16384.0).into());
                rv.set(arr.into());
            }
            _ => {
                // Unknown parameter → return null
                rv.set(v8::null(scope).into());
            }
        }
    }));
}

/// getExtension(name) → return extension object or null.
unsafe extern "C" fn webgl_get_extension(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            rv.set(v8::null(scope).into());
            return;
        }

        let name = args.get(0).to_rust_string_lossy(scope);

        match name.as_str() {
            "WEBGL_debug_renderer_info" => {
                let obj = v8::Object::new(scope);
                let vendor_key = v8::String::new(scope, "UNMASKED_VENDOR_WEBGL").expect("key");
                obj.set(scope, vendor_key.into(), v8::Integer::new(scope, UNMASKED_VENDOR_WEBGL as i32).into());
                let renderer_key = v8::String::new(scope, "UNMASKED_RENDERER_WEBGL").expect("key");
                obj.set(scope, renderer_key.into(), v8::Integer::new(scope, UNMASKED_RENDERER_WEBGL as i32).into());
                rv.set(obj.into());
            }
            "EXT_texture_filter_anisotropic" | "WEBKIT_EXT_texture_filter_anisotropic" => {
                let obj = v8::Object::new(scope);
                let key = v8::String::new(scope, "MAX_TEXTURE_MAX_ANISOTROPY_EXT").expect("key");
                obj.set(scope, key.into(), v8::Integer::new(scope, 0x84FF_u32 as i32).into());
                rv.set(obj.into());
            }
            _ => {
                rv.set(v8::null(scope).into());
            }
        }
    }));
}

/// getSupportedExtensions() → array of extension name strings.
unsafe extern "C" fn webgl_get_supported_extensions(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let extensions = [
            "ANGLE_instanced_arrays",
            "EXT_blend_minmax",
            "EXT_color_buffer_half_float",
            "EXT_float_blend",
            "EXT_frag_depth",
            "EXT_shader_texture_lod",
            "EXT_texture_filter_anisotropic",
            "OES_element_index_uint",
            "OES_standard_derivatives",
            "OES_texture_float",
            "OES_texture_float_linear",
            "OES_texture_half_float",
            "OES_texture_half_float_linear",
            "OES_vertex_array_object",
            "WEBGL_color_buffer_float",
            "WEBGL_compressed_texture_s3tc",
            "WEBGL_debug_renderer_info",
            "WEBGL_debug_shaders",
            "WEBGL_depth_texture",
            "WEBGL_draw_buffers",
            "WEBGL_lose_context",
        ];

        let arr = v8::Array::new(scope, extensions.len() as i32);
        for (i, ext) in extensions.iter().enumerate() {
            if let Some(s) = v8::String::new(scope, ext) {
                arr.set_index(scope, i as u32, s.into());
            }
        }
        rv.set(arr.into());
    }));
}
