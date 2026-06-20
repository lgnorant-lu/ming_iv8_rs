//! Browser Profile — single source of truth for browser identity values.
//!
//! All Navigator/Screen/UAData identity getters derive their fallback defaults from
//! `DEFAULT_PROFILE`. At runtime, `RuntimeState.environment` takes precedence over
//! these compile-time defaults (see `env_str_getter!` / `env_f64_getter!` / `env_bool_getter!`).
//!
//! Cross-ref: docs/roadmap/v0.8/shared/profile-contract.md

/// Compile-time default browser profile representing Chrome 131 on Windows x64, zh-CN locale.
///
/// Individual fields are referenced by getter callbacks as fallback values.
/// The `const` ensures zero runtime overhead — all values are baked into the binary.
#[derive(Debug, Clone)]
pub struct BrowserProfile {
    pub user_agent: &'static str,
    pub app_version: &'static str,
    pub platform: &'static str,
    pub vendor: &'static str,
    pub vendor_sub: &'static str,
    pub product: &'static str,
    pub product_sub: &'static str,
    pub app_name: &'static str,
    pub app_code_name: &'static str,

    pub language: &'static str,
    pub languages: &'static [&'static str],

    pub hardware_concurrency: f64,
    pub device_memory: f64,
    pub max_touch_points: f64,
    pub cookie_enabled: bool,
    pub on_line: bool,
    pub do_not_track: Option<&'static str>,
    pub webdriver: Option<bool>,
    pub pdf_viewer_enabled: bool,

    pub screen_width: f64,
    pub screen_height: f64,
    pub screen_avail_width: f64,
    pub screen_avail_height: f64,
    pub screen_color_depth: f64,
    pub screen_pixel_depth: f64,
    pub screen_avail_left: f64,
    pub screen_avail_top: f64,

    pub ua_brands_json: &'static str,
    pub ua_mobile: bool,
    pub ua_platform: &'static str,
    pub ua_architecture: &'static str,
    pub ua_bitness: &'static str,
    pub ua_model: &'static str,
    pub ua_platform_version: &'static str,
    pub ua_wow64: bool,
    pub ua_full_version_list_json: &'static str,

    pub device_pixel_ratio: f64,
}

pub const DEFAULT_PROFILE: BrowserProfile = BrowserProfile {
    user_agent: "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    app_version: "5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36",
    platform: "Win32",
    vendor: "Google Inc.",
    vendor_sub: "",
    product: "Gecko",
    product_sub: "20030107",
    app_name: "Netscape",
    app_code_name: "Mozilla",

    language: "zh-CN",
    languages: &["zh-CN", "en"],

    hardware_concurrency: 8.0,
    device_memory: 8.0,
    max_touch_points: 0.0,
    cookie_enabled: true,
    on_line: true,
    do_not_track: None,
    webdriver: None,
    pdf_viewer_enabled: true,

    screen_width: 1920.0,
    screen_height: 1080.0,
    screen_avail_width: 1920.0,
    screen_avail_height: 1040.0,
    screen_color_depth: 24.0,
    screen_pixel_depth: 24.0,
    screen_avail_left: 0.0,
    screen_avail_top: 0.0,

    ua_brands_json: r#"[{"brand":"Google Chrome","version":"131"},{"brand":"Chromium","version":"131"}]"#,
    ua_mobile: false,
    ua_platform: "Windows",
    ua_architecture: "x86",
    ua_bitness: "64",
    ua_model: "",
    ua_platform_version: "10.0.0",
    ua_wow64: false,
    ua_full_version_list_json: r#"[{"brand":"Google Chrome","version":"131.0.6778.86"},{"brand":"Chromium","version":"131.0.6778.86"}]"#,

    device_pixel_ratio: 1.0,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_profile_user_agent_contains_chrome() {
        assert!(DEFAULT_PROFILE.user_agent.contains("Chrome/131"));
        assert!(DEFAULT_PROFILE.user_agent.contains("Windows NT 10.0"));
    }

    #[test]
    fn test_default_profile_platform_is_windows() {
        assert_eq!(DEFAULT_PROFILE.platform, "Win32");
        assert_eq!(DEFAULT_PROFILE.ua_platform, "Windows");
    }

    #[test]
    fn test_default_profile_language_is_zh_cn() {
        assert_eq!(DEFAULT_PROFILE.language, "zh-CN");
        assert_eq!(DEFAULT_PROFILE.languages.len(), 2);
        assert_eq!(DEFAULT_PROFILE.languages[0], "zh-CN");
    }

    #[test]
    fn test_default_profile_brands_has_two_entries() {
        assert!(DEFAULT_PROFILE.ua_brands_json.contains("Google Chrome"));
        assert!(DEFAULT_PROFILE.ua_brands_json.contains("Chromium"));
        assert!(!DEFAULT_PROFILE.ua_brands_json.contains("Not/A)Brand"));
    }

    #[test]
    fn test_default_profile_screen_dimensions() {
        assert_eq!(DEFAULT_PROFILE.screen_width, 1920.0);
        assert_eq!(DEFAULT_PROFILE.screen_height, 1080.0);
        assert!(DEFAULT_PROFILE.screen_avail_width <= DEFAULT_PROFILE.screen_width);
    }

    #[test]
    fn test_default_profile_hardware_values_in_range() {
        assert!(DEFAULT_PROFILE.hardware_concurrency >= 1.0);
        assert!(DEFAULT_PROFILE.hardware_concurrency <= 128.0);
        assert!(DEFAULT_PROFILE.device_memory >= 0.5);
        assert!(DEFAULT_PROFILE.device_memory <= 64.0);
    }

    #[test]
    fn test_default_profile_static_strings_not_empty() {
        assert!(!DEFAULT_PROFILE.vendor.is_empty());
        assert!(!DEFAULT_PROFILE.product.is_empty());
        assert!(!DEFAULT_PROFILE.app_name.is_empty());
    }

    #[test]
    fn test_default_profile_uadata_architecture() {
        assert_eq!(DEFAULT_PROFILE.ua_architecture, "x86");
        assert_eq!(DEFAULT_PROFILE.ua_bitness, "64");
        assert!(!DEFAULT_PROFILE.ua_mobile);
    }
}
