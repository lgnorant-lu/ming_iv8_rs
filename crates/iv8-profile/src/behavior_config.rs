use serde::{Deserialize, Serialize};

/// Runtime-ready normalized behavior data for BCR installer construction.
///
/// This is NOT the public profile schema. It is the install-time runtime plan
/// produced by materializing a profile and extracting behavior-relevant fields.
/// No V8 lifetimes or V8 closure types appear here — this is pure data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BehaviorConfig {
    pub identity: ConfigIdentity,
    pub navigator: ConfigNavigator,
    pub screen: ConfigScreen,
    pub window: ConfigWindow,
    pub location: ConfigLocation,
    pub webgl: ConfigWebGl,
    pub canvas: ConfigCanvas,
    pub crypto: ConfigCrypto,
    pub time: ConfigTime,
    pub timers: ConfigTimers,
    pub permissions: ConfigPermissions,
    pub user_agent_data: ConfigUserAgentData,
}

// ---------------------------------------------------------------------------
// Identity
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigIdentity {
    pub user_agent: String,
    pub platform: String,
    pub browser_brand: String,
    pub browser_version: String,
    pub vendor: String,
    pub noise_seed: u64,
}

impl Default for ConfigIdentity {
    fn default() -> Self {
        Self {
            user_agent: String::new(),
            platform: "Win32".into(),
            browser_brand: "chrome".into(),
            browser_version: String::new(),
            vendor: "Google Inc.".into(),
            noise_seed: 514829086,
        }
    }
}

// ---------------------------------------------------------------------------
// Navigator
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigNavigator {
    pub language: String,
    pub languages: Vec<String>,
    pub hardware_concurrency: u32,
    pub device_memory: u32,
    pub max_touch_points: u32,
    pub webdriver: bool,
    pub pdf_viewer_enabled: bool,
}

impl Default for ConfigNavigator {
    fn default() -> Self {
        Self {
            language: "zh-CN".into(),
            languages: vec!["zh-CN".into(), "en".into()],
            hardware_concurrency: 8,
            device_memory: 8,
            max_touch_points: 0,
            webdriver: false,
            pdf_viewer_enabled: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Screen
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigScreen {
    pub width: u32,
    pub height: u32,
    pub avail_width: u32,
    pub avail_height: u32,
    pub color_depth: u32,
    pub pixel_depth: u32,
}

impl Default for ConfigScreen {
    fn default() -> Self {
        Self {
            width: 1920,
            height: 1080,
            avail_width: 1920,
            avail_height: 1040,
            color_depth: 24,
            pixel_depth: 24,
        }
    }
}

// ---------------------------------------------------------------------------
// Window
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigWindow {
    pub inner_width: u32,
    pub inner_height: u32,
    pub outer_width: u32,
    pub outer_height: u32,
    pub device_pixel_ratio: f64,
}

impl Default for ConfigWindow {
    fn default() -> Self {
        Self {
            inner_width: 1920,
            inner_height: 969,
            outer_width: 1920,
            outer_height: 1080,
            device_pixel_ratio: 1.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Location
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigLocation {
    pub href: String,
    pub origin: String,
    pub protocol: String,
    pub host: String,
    pub hostname: String,
    pub port: String,
    pub pathname: String,
    pub search: String,
    pub hash: String,
}

impl Default for ConfigLocation {
    fn default() -> Self {
        Self {
            href: "about:blank".into(),
            origin: "null".into(),
            protocol: "about:".into(),
            host: String::new(),
            hostname: String::new(),
            port: String::new(),
            pathname: String::new(),
            search: String::new(),
            hash: String::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// WebGL
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigWebGl {
    pub mode: String,
    pub vendor: String,
    pub renderer: String,
    pub unmasked_vendor: String,
    pub unmasked_renderer: String,
}

impl Default for ConfigWebGl {
    fn default() -> Self {
        Self {
            mode: "noise".into(),
            vendor: "Google Inc. (NVIDIA)".into(),
            renderer: concat!(
                "ANGLE (NVIDIA, NVIDIA GeForce GTX 1650 (0x00001F82) ",
                "Direct3D11 vs_5_0 ps_5_0, D3D11)"
            ).into(),
            unmasked_vendor: "Google Inc. (NVIDIA)".into(),
            unmasked_renderer: concat!(
                "ANGLE (NVIDIA, NVIDIA GeForce GTX 1650 (0x00001F82) ",
                "Direct3D11 vs_5_0 ps_5_0, D3D11)"
            ).into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Canvas
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigCanvas {
    pub mode: String,
    pub noise_seed: u64,
    pub default_width: u32,
    pub default_height: u32,
}

impl Default for ConfigCanvas {
    fn default() -> Self {
        Self {
            mode: "noise".into(),
            noise_seed: 514829086,
            default_width: 300,
            default_height: 150,
        }
    }
}

// ---------------------------------------------------------------------------
// Crypto
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigCrypto {
    pub deterministic_seed: u64,
    pub deterministic_random_uuid: bool,
    pub max_random_values_bytes: u32,
}

impl Default for ConfigCrypto {
    fn default() -> Self {
        Self {
            deterministic_seed: 514829086,
            deterministic_random_uuid: false,
            max_random_values_bytes: 65536,
        }
    }
}

// ---------------------------------------------------------------------------
// Time
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigTime {
    pub mode: String,
    pub epoch_ms: f64,
    pub freeze_ms: Option<f64>,
    pub performance_origin_ms: f64,
    pub precision_us: u32,
}

impl Default for ConfigTime {
    fn default() -> Self {
        Self {
            mode: "logical".into(),
            epoch_ms: 1_704_067_200_000.0,
            freeze_ms: None,
            performance_origin_ms: 0.0,
            precision_us: 1,
        }
    }
}

// ---------------------------------------------------------------------------
// Timers
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigTimers {
    pub raf_interval_ms: f64,
    pub min_interval_ms: f64,
}

impl Default for ConfigTimers {
    fn default() -> Self {
        Self {
            raf_interval_ms: 16.67,
            min_interval_ms: 1.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Permissions
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigPermissions {
    pub geolocation: String,
    pub notifications: String,
    pub camera: String,
    pub microphone: String,
}

impl Default for ConfigPermissions {
    fn default() -> Self {
        Self {
            geolocation: "prompt".into(),
            notifications: "prompt".into(),
            camera: "prompt".into(),
            microphone: "prompt".into(),
        }
    }
}

// ---------------------------------------------------------------------------
// User-Agent Client Hints
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigUserAgentData {
    pub platform: String,
    pub platform_version: String,
    pub architecture: String,
    pub bitness: String,
    pub mobile: bool,
    pub model: String,
}

impl Default for ConfigUserAgentData {
    fn default() -> Self {
        Self {
            platform: "Windows".into(),
            platform_version: "10.0.0".into(),
            architecture: "x86".into(),
            bitness: "64".into(),
            mobile: false,
            model: String::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Build from ProfileMatrix
// ---------------------------------------------------------------------------

impl BehaviorConfig {
    /// Construct BehaviorConfig from a materialized ProfileMatrix.
    pub fn from_matrix(matrix: &crate::ProfileMatrix) -> Self {
        Self {
            identity: ConfigIdentity {
                user_agent: matrix.navigator.user_agent.clone(),
                platform: matrix.navigator.platform.clone(),
                browser_brand: matrix.identity.browser_brand.clone(),
                browser_version: matrix.identity.browser_version.clone(),
                vendor: matrix.navigator.vendor.clone(),
                noise_seed: matrix.identity.noise_seed,
            },
            navigator: ConfigNavigator {
                language: matrix.navigator.language.clone(),
                languages: matrix.navigator.languages.clone(),
                hardware_concurrency: matrix.navigator.hardware_concurrency,
                device_memory: matrix.navigator.device_memory,
                max_touch_points: matrix.navigator.max_touch_points,
                webdriver: matrix.navigator.webdriver,
                pdf_viewer_enabled: matrix.navigator.pdf_viewer_enabled,
            },
            screen: ConfigScreen {
                width: matrix.display.screen_width,
                height: matrix.display.screen_height,
                avail_width: matrix.display.avail_width,
                avail_height: matrix.display.avail_height,
                color_depth: matrix.display.color_depth,
                pixel_depth: matrix.display.pixel_depth,
            },
            window: ConfigWindow {
                inner_width: matrix.display.inner_width,
                inner_height: matrix.display.inner_height,
                outer_width: matrix.display.outer_width,
                outer_height: matrix.display.outer_height,
                device_pixel_ratio: matrix.display.device_pixel_ratio,
            },
            location: ConfigLocation::default(),
            webgl: ConfigWebGl {
                mode: matrix.rendering.webgl1_mode.clone(),
                vendor: matrix.identity.gpu_vendor.clone(),
                renderer: matrix.identity.gpu_renderer.clone(),
                unmasked_vendor: matrix.identity.webgl_unmasked_vendor.clone(),
                unmasked_renderer: matrix.identity.webgl_unmasked_renderer.clone(),
            },
            canvas: ConfigCanvas {
                mode: matrix.rendering.canvas_mode.clone(),
                noise_seed: matrix.rendering.canvas_seed,
                default_width: 300,
                default_height: 150,
            },
            crypto: ConfigCrypto {
                deterministic_seed: matrix.identity.noise_seed,
                deterministic_random_uuid: false,
                max_random_values_bytes: 65536,
            },
            time: ConfigTime {
                mode: matrix.timing.mode.clone(),
                ..ConfigTime::default()
            },
            timers: ConfigTimers {
                raf_interval_ms: if matrix.timing.fps > 0 {
                    1000.0 / matrix.timing.fps as f64
                } else {
                    ConfigTimers::default().raf_interval_ms
                },
                min_interval_ms: ConfigTimers::default().min_interval_ms,
            },
            permissions: ConfigPermissions {
                geolocation: matrix.permissions.geolocation.clone(),
                notifications: matrix.permissions.notifications.clone(),
                camera: "prompt".into(),
                microphone: "prompt".into(),
            },
            user_agent_data: ConfigUserAgentData {
                platform: matrix
                    .flat_env
                    .get("navigator.userAgentData.platform")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Windows")
                    .into(),
                platform_version: matrix
                    .flat_env
                    .get("navigator.userAgentData.platformVersion")
                    .and_then(|v| v.as_str())
                    .unwrap_or("10.0.0")
                    .into(),
                architecture: matrix
                    .flat_env
                    .get("navigator.userAgentData.architecture")
                    .and_then(|v| v.as_str())
                    .unwrap_or("x86")
                    .into(),
                bitness: matrix
                    .flat_env
                    .get("navigator.userAgentData.bitness")
                    .and_then(|v| v.as_str())
                    .unwrap_or("64")
                    .into(),
                mobile: matrix
                    .flat_env
                    .get("navigator.userAgentData.mobile")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false),
                model: matrix
                    .flat_env
                    .get("navigator.userAgentData.model")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .into(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defaults::default_profile_source;

    #[test]
    fn behavior_config_from_matrix() {
        let source = default_profile_source();
        let (matrix, _) = crate::ProfileMatrix::from_source(&source);
        let config = BehaviorConfig::from_matrix(&matrix);
        assert!(config.identity.user_agent.contains("Chrome/147"));
        assert_eq!(config.screen.width, 1920);
        assert_eq!(config.screen.height, 1080);
    }

    #[test]
    fn behavior_config_defaults_roundtrip() {
        let config = BehaviorConfig::from_matrix(
            &crate::ProfileMatrix::from_source(&default_profile_source()).0,
        );
        let json = serde_json::to_string_pretty(&config).expect("serialize");
        let _back: BehaviorConfig = serde_json::from_str(&json).expect("deserialize");
    }

    #[test]
    fn behavior_config_uses_canvas_seed() {
        let source = default_profile_source();
        let (matrix, _) = crate::ProfileMatrix::from_source(&source);
        let config = BehaviorConfig::from_matrix(&matrix);
        assert_eq!(config.canvas.noise_seed, matrix.rendering.canvas_seed);
    }

    #[test]
    fn behavior_config_maps_timing_and_user_agent_data() {
        let mut source = default_profile_source();
        source.timing.fps = 120;
        source.navigator.user_agent_data.platform = "Android".into();
        source.navigator.user_agent_data.mobile = true;
        let (matrix, _) = crate::ProfileMatrix::from_source(&source);
        let config = BehaviorConfig::from_matrix(&matrix);
        assert_eq!(config.time.mode, matrix.timing.mode);
        assert!((config.timers.raf_interval_ms - 8.333333333333334).abs() < f64::EPSILON);
        assert_eq!(config.user_agent_data.platform, "Android");
        assert!(config.user_agent_data.mobile);
    }
}
