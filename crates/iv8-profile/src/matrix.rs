use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::source::ProfileSource;
use crate::validation::{self, ValidationResult};

// Domain seeds — deterministic sub-seeds from the profile noise_seed.
const DOMAIN_CANVAS: u64 = 0x0000_0000_0000_0001;
const DOMAIN_WEBGL1: u64 = 0x0000_0000_0000_0002;
const DOMAIN_WEBGL2: u64 = 0x0000_0000_0000_0003;
const DOMAIN_AUDIO: u64 = 0x0000_0000_0000_0004;
const DOMAIN_CLIENT_RECTS: u64 = 0x0000_0000_0000_0005;

/// Materialized internal profile.
///
/// All defaults have been expanded. Deterministic for a given source + seed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfileMatrix {
    pub source_name: String,
    pub schema_version: String,
    pub identity: MatrixIdentity,
    pub navigator: MatrixNavigator,
    pub display: MatrixDisplay,
    pub rendering: MatrixRendering,
    pub locale: MatrixLocale,
    pub network: MatrixNetwork,
    pub permissions: MatrixPermissions,
    pub capabilities: MatrixCapabilities,
    pub storage: MatrixStorage,
    pub timing: MatrixTiming,
    /// Seed-derived scatter values for noise surface generation.
    pub seeds: HashMap<String, u64>,
    /// Flat dot-path compatibility map for legacy consumers.
    pub flat_env: HashMap<String, serde_json::Value>,
}

impl ProfileMatrix {
    /// Materialize a ProfileSource into a fully-expanded ProfileMatrix.
    ///
    /// Runs coherence validation first; returns a ValidationResult alongside
    /// the matrix so callers can inspect issues without rejecting the matrix
    /// unconditionally.
    pub fn from_source(source: &ProfileSource) -> (Self, ValidationResult) {
        let vr = validation::validate(source);
        let seed = u64::from(source.identity.noise_seed);

        let mut seeds = HashMap::new();
        seeds.insert("canvas".into(), seed ^ DOMAIN_CANVAS);
        seeds.insert("webgl1".into(), seed ^ DOMAIN_WEBGL1);
        seeds.insert("webgl2".into(), seed ^ DOMAIN_WEBGL2);
        seeds.insert("audio".into(), seed ^ DOMAIN_AUDIO);
        seeds.insert("client_rects".into(), seed ^ DOMAIN_CLIENT_RECTS);

        let flat_env = build_flat_env(source);

        let matrix = Self {
            source_name: source.meta.name.clone(),
            schema_version: source.meta.schema_version.clone(),
            identity: MatrixIdentity {
                os: source.identity.os.clone(),
                os_version: source.identity.os_version.clone(),
                cpu_arch: source.identity.cpu_arch.clone(),
                cpu_cores: source.identity.cpu_cores,
                memory_gb: source.identity.memory_gb,
                browser_brand: source.identity.browser.brand.clone(),
                browser_version: source.identity.browser.version.clone(),
                gpu_vendor: source.identity.gpu.vendor.clone(),
                gpu_renderer: source.identity.gpu.renderer.clone(),
                webgl_unmasked_vendor: source.identity.gpu.webgl_unmasked_vendor.clone(),
                webgl_unmasked_renderer: source.identity.gpu.webgl_unmasked_renderer.clone(),
                noise_seed: seed,
            },
            navigator: MatrixNavigator {
                user_agent: source.navigator.user_agent.clone(),
                platform: source.navigator.platform.clone(),
                vendor: source.navigator.vendor.clone(),
                language: source.navigator.language.clone(),
                languages: source.navigator.languages.clone(),
                hardware_concurrency: source.navigator.hardware_concurrency,
                device_memory: source.navigator.device_memory,
                max_touch_points: source.navigator.max_touch_points,
                webdriver: source.navigator.webdriver,
                pdf_viewer_enabled: source.navigator.pdf_viewer_enabled,
            },
            display: MatrixDisplay {
                screen_width: source.display.screen.width,
                screen_height: source.display.screen.height,
                avail_width: source.display.screen.avail_width,
                avail_height: source.display.screen.avail_height,
                color_depth: source.display.screen.color_depth,
                pixel_depth: source.display.screen.pixel_depth,
                inner_width: source.display.window.inner_width,
                inner_height: source.display.window.inner_height,
                outer_width: source.display.window.outer_width,
                outer_height: source.display.window.outer_height,
                device_pixel_ratio: source.display.window.device_pixel_ratio,
            },
            rendering: MatrixRendering {
                canvas_mode: source.rendering.canvas_2d.mode.clone(),
                canvas_seed: seeds["canvas"],
                webgl1_mode: source.rendering.webgl_1.mode.clone(),
                webgl2_mode: source.rendering.webgl_2.mode.clone(),
                webgpu_mode: source.rendering.webgpu.mode.clone(),
                audio_mode: source.rendering.audio_context.mode.clone(),
                client_rects_mode: source.rendering.client_rects.mode.clone(),
                fonts_mode: source.rendering.fonts.mode.clone(),
            },
            locale: MatrixLocale {
                timezone: source.locale.timezone.clone(),
                language: source.locale.language.clone(),
                languages: source.locale.languages.clone(),
                accept_language: source.locale.accept_language.clone(),
                geolocation_mode: source.locale.geolocation.mode.clone(),
            },
            network: MatrixNetwork {
                webrtc_mode: source.network.webrtc.mode.clone(),
                proxy_host: source.network.proxy.as_ref().map(|p| p.host.clone()),
                proxy_port: source.network.proxy.as_ref().map(|p| p.port),
            },
            permissions: MatrixPermissions {
                geolocation: source.permissions.geolocation.clone(),
                notifications: source.permissions.notifications.clone(),
            },
            capabilities: MatrixCapabilities {
                window_chrome: source.capabilities.window_chrome,
                webgpu: source.capabilities.webgpu,
            },
            storage: MatrixStorage {
                local_storage: source.storage.local_storage,
                history_length: source.storage.history_length,
            },
            timing: MatrixTiming {
                mode: source.timing.mode.clone(),
                fps: source.timing.fps,
            },
            seeds,
            flat_env,
        };

        (matrix, vr)
    }
}

fn build_flat_env(source: &ProfileSource) -> HashMap<String, serde_json::Value> {
    let mut env = HashMap::new();

    env.insert(
        "config.features.profile".into(),
        serde_json::json!("chrome147_win"),
    );

    // Navigator
    env.insert(
        "navigator.userAgent".into(),
        serde_json::Value::String(source.navigator.user_agent.clone()),
    );
    env.insert(
        "navigator.platform".into(),
        serde_json::Value::String(source.navigator.platform.clone()),
    );
    env.insert(
        "navigator.vendor".into(),
        serde_json::Value::String(source.navigator.vendor.clone()),
    );
    env.insert(
        "navigator.language".into(),
        serde_json::Value::String(source.navigator.language.clone()),
    );
    env.insert(
        "navigator.languages".into(),
        serde_json::to_value(&source.navigator.languages).unwrap_or_default(),
    );
    env.insert(
        "navigator.hardwareConcurrency".into(),
        serde_json::json!(source.navigator.hardware_concurrency),
    );
    env.insert(
        "navigator.deviceMemory".into(),
        serde_json::json!(source.navigator.device_memory),
    );
    env.insert(
        "navigator.maxTouchPoints".into(),
        serde_json::json!(source.navigator.max_touch_points),
    );
    env.insert(
        "navigator.webdriver".into(),
        serde_json::json!(source.navigator.webdriver),
    );
    env.insert(
        "navigator.pdfViewerEnabled".into(),
        serde_json::json!(source.navigator.pdf_viewer_enabled),
    );

    // Screen
    env.insert(
        "screen.width".into(),
        serde_json::json!(source.display.screen.width),
    );
    env.insert(
        "screen.height".into(),
        serde_json::json!(source.display.screen.height),
    );
    env.insert(
        "screen.availWidth".into(),
        serde_json::json!(source.display.screen.avail_width),
    );
    env.insert(
        "screen.availHeight".into(),
        serde_json::json!(source.display.screen.avail_height),
    );
    env.insert(
        "screen.colorDepth".into(),
        serde_json::json!(source.display.screen.color_depth),
    );
    env.insert(
        "screen.pixelDepth".into(),
        serde_json::json!(source.display.screen.pixel_depth),
    );

    // Window
    env.insert(
        "window.innerWidth".into(),
        serde_json::json!(source.display.window.inner_width),
    );
    env.insert(
        "window.innerHeight".into(),
        serde_json::json!(source.display.window.inner_height),
    );
    env.insert(
        "window.devicePixelRatio".into(),
        serde_json::json!(source.display.window.device_pixel_ratio),
    );

    // Locale
    env.insert(
        "locale.timezone".into(),
        serde_json::Value::String(source.locale.timezone.clone()),
    );
    env.insert(
        "locale.language".into(),
        serde_json::Value::String(source.locale.language.clone()),
    );
    env.insert(
        "navigator.language".into(),
        serde_json::Value::String(source.locale.language.clone()),
    );

    // Identity
    env.insert(
        "config.features.browserVersion".into(),
        serde_json::Value::String(source.identity.browser.version.clone()),
    );

    // Locale/network
    env.insert(
        "location.href".into(),
        serde_json::json!("about:blank"),
    );

    // Rendering modes
    env.insert(
        "canvas.mode".into(),
        serde_json::Value::String(source.rendering.canvas_2d.mode.clone()),
    );
    env.insert(
        "webgl.mode".into(),
        serde_json::Value::String(source.rendering.webgl_1.mode.clone()),
    );

    // Compat overrides
    for (k, v) in &source.compat.flat_env_overrides {
        env.insert(k.clone(), v.clone());
    }

    env
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defaults::default_profile_source;

    #[test]
    fn materialization_same_source_deterministic() {
        let source = default_profile_source();
        let (m1, _) = ProfileMatrix::from_source(&source);
        let (m2, _) = ProfileMatrix::from_source(&source);
        assert_eq!(m1.flat_env, m2.flat_env);
        assert_eq!(m1.seeds, m2.seeds);
    }

    #[test]
    fn materialization_has_flat_env_entries() {
        let source = default_profile_source();
        let (matrix, _) = ProfileMatrix::from_source(&source);
        assert!(matrix.flat_env.len() > 20);
        assert_eq!(
            matrix.flat_env.get("navigator.userAgent").and_then(|v| v.as_str()),
            Some(source.navigator.user_agent.as_str())
        );
    }

    #[test]
    fn materialization_derives_seeds() {
        let source = default_profile_source();
        let (matrix, _) = ProfileMatrix::from_source(&source);
        assert_eq!(matrix.seeds.len(), 5);
        assert_ne!(matrix.seeds["canvas"], matrix.seeds["webgl1"]);
        assert_eq!(
            matrix.rendering.canvas_seed,
            matrix.seeds["canvas"]
        );
    }

    #[test]
    fn materialization_different_seeds_different_seeds() {
        let mut source = default_profile_source();
        source.identity.noise_seed = 999_999;
        let (m1, _) = ProfileMatrix::from_source(&source);
        source.identity.noise_seed = 888_888;
        let (m2, _) = ProfileMatrix::from_source(&source);
        assert_ne!(m1.seeds["canvas"], m2.seeds["canvas"]);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatrixIdentity {
    pub os: String,
    pub os_version: String,
    pub cpu_arch: String,
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub browser_brand: String,
    pub browser_version: String,
    pub gpu_vendor: String,
    pub gpu_renderer: String,
    pub webgl_unmasked_vendor: String,
    pub webgl_unmasked_renderer: String,
    pub noise_seed: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatrixNavigator {
    pub user_agent: String,
    pub platform: String,
    pub vendor: String,
    pub language: String,
    pub languages: Vec<String>,
    pub hardware_concurrency: u32,
    pub device_memory: u32,
    pub max_touch_points: u32,
    pub webdriver: bool,
    pub pdf_viewer_enabled: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatrixDisplay {
    pub screen_width: u32,
    pub screen_height: u32,
    pub avail_width: u32,
    pub avail_height: u32,
    pub color_depth: u32,
    pub pixel_depth: u32,
    pub inner_width: u32,
    pub inner_height: u32,
    pub outer_width: u32,
    pub outer_height: u32,
    pub device_pixel_ratio: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatrixRendering {
    pub canvas_mode: String,
    pub canvas_seed: u64,
    pub webgl1_mode: String,
    pub webgl2_mode: String,
    pub webgpu_mode: String,
    pub audio_mode: String,
    pub client_rects_mode: String,
    pub fonts_mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatrixLocale {
    pub timezone: String,
    pub language: String,
    pub languages: Vec<String>,
    pub accept_language: String,
    pub geolocation_mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatrixNetwork {
    pub webrtc_mode: String,
    pub proxy_host: Option<String>,
    pub proxy_port: Option<u16>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatrixPermissions {
    pub geolocation: String,
    pub notifications: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatrixCapabilities {
    pub window_chrome: bool,
    pub webgpu: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatrixStorage {
    pub local_storage: bool,
    pub history_length: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MatrixTiming {
    pub mode: String,
    pub fps: u32,
}
