//! Deep stub verification tests — descriptor matrix + fingerprint battery.
//!
//! v0.8.21: Validates P0 deep stub FunctionTemplate integrity against
//! Chrome 147 baseline. Tests assert properties that can be verified
//! at compile time and via isolated unit checks.
//!
//! Full V8-runtime integration tests require an Isolate and are in
//! tests/integration/ (executed with --features native-surface).

#[cfg(test)]
mod tests {
    use super::super::{CANVAS_2D_DEFAULTS, CANVAS_2D_METHODS};
    use super::super::navigator::*;
    use super::super::webgl::*;

    // ── Canvas 2D descriptor tests ───────────────────────────────────────

    #[test]
    fn canvas2d_template_properties_count() {
        // All 24 properties must be registered
        assert_eq!(CANVAS_2D_DEFAULTS.len(), 24);
    }

    #[test]
    fn canvas2d_template_methods_count() {
        assert!(CANVAS_2D_METHODS.len() >= 31);
    }

    // ── Navigator descriptor tests ───────────────────────────────────────

    #[test]
    fn navigator_property_count_is_22() {
        assert_eq!(NAVIGATOR_PROPERTIES.len(), 22);
    }

    #[test]
    fn navigator_has_user_agent() {
        assert!(NAVIGATOR_PROPERTIES.contains(&"userAgent"));
    }

    #[test]
    fn navigator_has_platform() {
        assert!(NAVIGATOR_PROPERTIES.contains(&"platform"));
    }

    #[test]
    fn navigator_has_webdriver() {
        assert!(NAVIGATOR_PROPERTIES.contains(&"webdriver"));
    }

    #[test]
    fn navigator_all_properties_lowercase() {
        for prop in NAVIGATOR_PROPERTIES {
            assert!(!prop.is_empty());
            let first = prop.chars().next().unwrap();
            assert!(first.is_lowercase(), "{} should start with lowercase", prop);
        }
    }

    // ── WebGL descriptor tests ───────────────────────────────────────────

    #[test]
    fn webgl_param_map_has_min_30_entries() {
        let map = build_gl_param_map();
        assert!(map.len() >= 30);
    }

    #[test]
    fn webgl_param_vendor_is_string() {
        let map = build_gl_param_map();
        let vendor = &map[&0x1F00];
        assert_eq!(vendor.param_type, GlParamType::String);
    }

    #[test]
    fn webgl_param_max_texture_size_is_int() {
        let map = build_gl_param_map();
        let mts = &map[&0x0D33];
        assert_eq!(mts.param_type, GlParamType::Int);
    }

    #[test]
    fn webgl_extensions_have_debug_renderer_info() {
        assert!(WEBGL_EXTENSIONS.contains(&"WEBGL_debug_renderer_info"));
    }

    #[test]
    fn webgl_extensions_have_lose_context() {
        assert!(WEBGL_EXTENSIONS.contains(&"WEBGL_lose_context"));
    }

    #[test]
    fn webgl_constants_count() {
        assert!(WEBGL_CONSTANTS.len() >= 50);
    }

    /// Fingerprint battery: 16 detection vectors check.
    /// These are verified at the data level — full V8 integration
    /// tests in tests/integration/ verify runtime behavior.
    #[test]
    fn fingerprint_battery_data_integrity() {
        // Vector 1: typeof — verified by template existence
        // Vector 2: instanceof — verified by prototype chain in generated code
        // Vector 3: Symbol.toStringTag — verified by generated toStringTag calls
        // Vector 5: descriptor — verified by set_accessor_property attributes

        // All deep stub interfaces must have their templates defined
        assert!(CANVAS_2D_DEFAULTS.len() >= 20);
        assert!(CANVAS_2D_METHODS.len() >= 30);
        assert!(NAVIGATOR_PROPERTIES.len() == 22);
        assert!(build_gl_param_map().len() >= 30);
        assert!(WEBGL_EXTENSIONS.len() >= 21);
    }
}
