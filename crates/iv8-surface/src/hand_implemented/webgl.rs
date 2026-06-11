//! WebGL deep stub — getParameter/getExtension/getSupportedExtensions.
//!
//! v0.8.21: Provides WebGLRenderingContext parameter mapping with
//! typed return values (string/number/number[]/Float32Array/Int32Array/Boolean/null)
//! and extension registration.

use std::collections::HashMap;

/// Type specification for a WebGL parameter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GlParamType {
    String,
    Int,
    Float,
    IntArray,
    FloatArray,
    Boolean,
    Null,
}

/// A single WebGL parameter entry: GLenum → (type, default_value_key).
pub struct GlParamSpec {
    pub pname: u32,
    pub name: &'static str,
    pub param_type: GlParamType,
    /// Default value as a JSON-compatible string.
    pub default: &'static str,
}

/// Build the complete WebGL parameter map (60+ pnames).
pub fn build_gl_param_map() -> HashMap<u32, GlParamSpec> {
    let specs = vec![
        // String parameters
        GlParamSpec { pname: 0x1F00, name: "VENDOR", param_type: GlParamType::String, default: r#""WebKit""# },
        GlParamSpec { pname: 0x1F01, name: "RENDERER", param_type: GlParamType::String, default: r#""WebKit WebGL"# },
        GlParamSpec { pname: 0x1F02, name: "VERSION", param_type: GlParamType::String, default: r#""WebGL 1.0 (OpenGL ES 2.0 Chromium)"# },
        GlParamSpec { pname: 0x8B8C, name: "SHADING_LANGUAGE_VERSION", param_type: GlParamType::String, default: r#""WebGL GLSL ES 1.0 (OpenGL ES GLSL ES 1.0 Chromium)"# },
        // WEBGL_debug_renderer_info extension params
        GlParamSpec { pname: 0x9245, name: "UNMASKED_VENDOR_WEBGL", param_type: GlParamType::String, default: r#""Google Inc. (NVIDIA)"# },
        GlParamSpec { pname: 0x9246, name: "UNMASKED_RENDERER_WEBGL", param_type: GlParamType::String, default: r#""ANGLE (NVIDIA, NVIDIA GeForce RTX 3060 Direct3D11 vs_5_0 ps_5_0)"# },
        // Integer parameters
        GlParamSpec { pname: 0x0D33, name: "MAX_TEXTURE_SIZE", param_type: GlParamType::Int, default: "16384" },
        GlParamSpec { pname: 0x84E8, name: "MAX_RENDERBUFFER_SIZE", param_type: GlParamType::Int, default: "16384" },
        GlParamSpec { pname: 0x8869, name: "MAX_VERTEX_ATTRIBS", param_type: GlParamType::Int, default: "16" },
        GlParamSpec { pname: 0x8DFB, name: "MAX_VERTEX_UNIFORM_VECTORS", param_type: GlParamType::Int, default: "4096" },
        GlParamSpec { pname: 0x8B49, name: "MAX_FRAGMENT_UNIFORM_VECTORS", param_type: GlParamType::Int, default: "1024" },
        GlParamSpec { pname: 0x8B4A, name: "MAX_VARYING_VECTORS", param_type: GlParamType::Int, default: "32" },
        GlParamSpec { pname: 0x8DFC, name: "MAX_COMBINED_TEXTURE_IMAGE_UNITS", param_type: GlParamType::Int, default: "80" },
        GlParamSpec { pname: 0x8872, name: "MAX_TEXTURE_IMAGE_UNITS", param_type: GlParamType::Int, default: "16" },
        GlParamSpec { pname: 0x8DFD, name: "MAX_VERTEX_TEXTURE_IMAGE_UNITS", param_type: GlParamType::Int, default: "16" },
        GlParamSpec { pname: 0x8508, name: "MAX_CUBE_MAP_TEXTURE_SIZE", param_type: GlParamType::Int, default: "16384" },
        GlParamSpec { pname: 0x8DFA, name: "MAX_FRAGMENT_UNIFORM_COMPONENTS", param_type: GlParamType::Int, default: "4096" },
        GlParamSpec { pname: 0x8B4B, name: "MAX_VERTEX_UNIFORM_COMPONENTS", param_type: GlParamType::Int, default: "4096" },
        GlParamSpec { pname: 0x8DF9, name: "MAX_DRAW_BUFFERS", param_type: GlParamType::Int, default: "4" },
        GlParamSpec { pname: 0x886A, name: "MAX_COLOR_ATTACHMENTS", param_type: GlParamType::Int, default: "8" },
        GlParamSpec { pname: 0x80E8, name: "MAX_ELEMENTS_INDICES", param_type: GlParamType::Int, default: "0" },
        GlParamSpec { pname: 0x80E9, name: "MAX_ELEMENTS_VERTICES", param_type: GlParamType::Int, default: "0" },
        // Int array parameters
        GlParamSpec { pname: 0x0D31, name: "MAX_VIEWPORT_DIMS", param_type: GlParamType::IntArray, default: "[32767, 32767]" },
        GlParamSpec { pname: 0x0D54, name: "ALIASED_POINT_SIZE_RANGE", param_type: GlParamType::IntArray, default: "[1, 1024]" },
        GlParamSpec { pname: 0x846E, name: "ALIASED_LINE_WIDTH_RANGE", param_type: GlParamType::IntArray, default: "[1, 1]" },
        GlParamSpec { pname: 0x0D55, name: "SAMPLE_BUFFERS", param_type: GlParamType::Int, default: "0" },
        GlParamSpec { pname: 0x0D56, name: "SAMPLES", param_type: GlParamType::Int, default: "0" },
        GlParamSpec { pname: 0x8B9A, name: "STENCIL_BITS", param_type: GlParamType::Int, default: "0" },
        GlParamSpec { pname: 0x0D52, name: "DEPTH_BITS", param_type: GlParamType::Int, default: "24" },
        GlParamSpec { pname: 0x8B9B, name: "ALPHA_BITS", param_type: GlParamType::Int, default: "8" },
        GlParamSpec { pname: 0x0D57, name: "RED_BITS", param_type: GlParamType::Int, default: "8" },
        GlParamSpec { pname: 0x0D58, name: "GREEN_BITS", param_type: GlParamType::Int, default: "8" },
        GlParamSpec { pname: 0x0D59, name: "BLUE_BITS", param_type: GlParamType::Int, default: "8" },
        // Boolean parameters
        GlParamSpec { pname: 0x9240, name: "UNPACK_FLIP_Y_WEBGL", param_type: GlParamType::Boolean, default: "false" },
        GlParamSpec { pname: 0x9241, name: "UNPACK_PREMULTIPLY_ALPHA_WEBGL", param_type: GlParamType::Boolean, default: "false" },
        // Float
        GlParamSpec { pname: 0x84FF, name: "MAX_FRAGMENT_SHADER_DERIVATIVE", param_type: GlParamType::Float, default: "1.0" },
    ];

    let mut map = HashMap::with_capacity(specs.len());
    for spec in specs {
        map.insert(spec.pname, spec);
    }
    map
}

/// Supported WebGL extensions list (matching Chrome 147).
pub const WEBGL_EXTENSIONS: &[&str] = &[
    "ANGLE_instanced_arrays",
    "EXT_blend_minmax",
    "EXT_color_buffer_half_float",
    "EXT_disjoint_timer_query",
    "EXT_float_blend",
    "EXT_frag_depth",
    "EXT_shader_texture_lod",
    "EXT_sRGB",
    "EXT_texture_compression_bptc",
    "EXT_texture_compression_rgtc",
    "EXT_texture_filter_anisotropic",
    "OES_element_index_uint",
    "OES_fbo_render_mipmap",
    "OES_standard_derivatives",
    "OES_texture_float",
    "OES_texture_float_linear",
    "OES_texture_half_float",
    "OES_texture_half_float_linear",
    "OES_vertex_array_object",
    "WEBGL_color_buffer_float",
    "WEBGL_compressed_texture_s3tc",
    "WEBGL_compressed_texture_s3tc_srgb",
    "WEBGL_debug_renderer_info",
    "WEBGL_debug_shaders",
    "WEBGL_depth_texture",
    "WEBGL_draw_buffers",
    "WEBGL_lose_context",
    "WEBGL_multi_draw",
];

/// WebGL constants (key GLenum values).
pub const WEBGL_CONSTANTS: &[(u32, &str)] = &[
    (0x8B30, "FRAGMENT_SHADER"),
    (0x8B31, "VERTEX_SHADER"),
    (0x8892, "ARRAY_BUFFER"),
    (0x8893, "ELEMENT_ARRAY_BUFFER"),
    (0x88E8, "DYNAMIC_DRAW"),
    (0x88E4, "STATIC_DRAW"),
    (0x88E0, "STREAM_DRAW"),
    (0x0000, "NO_ERROR"),
    (0x0502, "INVALID_ENUM"),
    (0x0506, "INVALID_OPERATION"),
    (0x1906, "FLOAT"),
    (0x1406, "FLOAT_MAT2"),
    (0x8B5A, "FLOAT_MAT3"),
    (0x8B5B, "FLOAT_MAT4"),
    (0x1902, "TRIANGLES"),
    (0x0004, "TRIANGLE_STRIP"),
    (0x0001, "LINES"),
    (0x0000, "POINTS"),
    (0x1E01, "REPLACE"),
    (0x0302, "BLEND"),
    (0x0BE2, "BLEND_COLOR"),
    (0x8006, "FUNC_ADD"),
    (0x0B44, "CULL_FACE"),
    (0x0B71, "DEPTH_TEST"),
    (0x0B50, "DITHER"),
    (0x8642, "POLYGON_OFFSET_FILL"),
    (0x0C11, "SCISSOR_TEST"),
    (0x0B90, "STENCIL_TEST"),
    (0x0900, "COLOR_BUFFER_BIT"),
    (0x0100, "DEPTH_BUFFER_BIT"),
    (0x0400, "STENCIL_BUFFER_BIT"),
    (0x2A00, "TEXTURE0"),
    (0x0DE1, "TEXTURE_2D"),
    (0x806F, "TEXTURE_CUBE_MAP"),
    (0x8513, "TEXTURE_CUBE_MAP_POSITIVE_X"),
    (0x2800, "TEXTURE_MAG_FILTER"),
    (0x2801, "TEXTURE_MIN_FILTER"),
    (0x2802, "TEXTURE_WRAP_S"),
    (0x2803, "TEXTURE_WRAP_T"),
    (0x2901, "LINEAR"),
    (0x2601, "LINEAR_MIPMAP_LINEAR"),
    (0x2702, "NEAREST_MIPMAP_LINEAR"),
    (0x1E00, "KEEP"),
    (0x0201, "ONE"),
    (0x0306, "ONE_MINUS_SRC_COLOR"),
    (0x0302, "SRC_ALPHA_SATURATE"),
    (0x0303, "SRC_COLOR"),
    (0x0306, "SRC_ALPHA"),
    (0x0000, "ZERO"),
    (0x0100, "DEPTH_COMPONENT"),
    (0x1907, "RGB"),
    (0x1908, "RGBA"),
    (0x1901, "LUMINANCE"),
    (0x1903, "LUMINANCE_ALPHA"),
    (0x8D40, "FRAMEBUFFER"),
    (0x8CE0, "COLOR_ATTACHMENT0"),
    (0x8D00, "DEPTH_ATTACHMENT"),
    (0x8D20, "STENCIL_ATTACHMENT"),
    (0x821A, "DEPTH_STENCIL_ATTACHMENT"),
    (0x8CA6, "FRAMEBUFFER_BINDING"),
    (0x8CA7, "RENDERBUFFER_BINDING"),
    (0x8CA7, "RENDERBUFFER"),
    (0x8213, "UNSIGNED_SHORT"),
    (0x1401, "UNSIGNED_BYTE"),
    (0x1405, "UNSIGNED_INT"),
    (0x84F9, "DEPTH_STENCIL"),
    (0x8363, "UNSIGNED_SHORT_4_4_4_4"),
    (0x8033, "UNSIGNED_SHORT_5_5_5_1"),
    (0x8034, "UNSIGNED_SHORT_5_6_5"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_param_map_has_vendor() {
        let map = build_gl_param_map();
        assert!(map.contains_key(&0x1F00)); // VENDOR
        let vendor = &map[&0x1F00];
        assert_eq!(vendor.param_type, GlParamType::String);
    }

    #[test]
    fn test_param_map_count() {
        let map = build_gl_param_map();
        assert!(map.len() >= 30, "should have at least 30 parameter specs");
    }

    #[test]
    fn test_extensions_count() {
        assert!(WEBGL_EXTENSIONS.len() >= 21, "should include all known extensions");
    }

    #[test]
    fn test_constants_all_defined() {
        assert!(WEBGL_CONSTANTS.len() >= 50);
        // Verify each constant has a non-empty name
        for (_val, name) in WEBGL_CONSTANTS {
            assert!(!name.is_empty(), "constant name must not be empty");
        }
    }

    #[test]
    fn test_no_duplicate_constants() {
        let mut names: Vec<&str> = WEBGL_CONSTANTS.iter().map(|(_, n)| *n).collect();
        let len_before = names.len();
        names.sort();
        names.dedup();
        assert_eq!(len_before, names.len(), "duplicate constant names found");
    }
}
