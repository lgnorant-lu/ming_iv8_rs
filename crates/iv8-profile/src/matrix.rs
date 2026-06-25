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
        seeds.insert(
            "canvas".into(),
            source
                .rendering
                .canvas_2d
                .sub_seed
                .unwrap_or(seed ^ DOMAIN_CANVAS),
        );
        seeds.insert(
            "webgl1".into(),
            source
                .rendering
                .webgl_1
                .sub_seed
                .unwrap_or(seed ^ DOMAIN_WEBGL1),
        );
        seeds.insert(
            "webgl2".into(),
            source
                .rendering
                .webgl_2
                .sub_seed
                .unwrap_or(seed ^ DOMAIN_WEBGL2),
        );
        seeds.insert(
            "audio".into(),
            source
                .rendering
                .audio_context
                .sub_seed
                .unwrap_or(seed ^ DOMAIN_AUDIO),
        );
        seeds.insert(
            "client_rects".into(),
            source
                .rendering
                .client_rects
                .sub_seed
                .unwrap_or(seed ^ DOMAIN_CLIENT_RECTS),
        );

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
                camera: source.permissions.camera.clone(),
                microphone: source.permissions.microphone.clone(),
                clipboard_read: source.permissions.clipboard_read.clone(),
                clipboard_write: source.permissions.clipboard_write.clone(),
                local_fonts: source.permissions.local_fonts.clone(),
                extra: source.permissions.extra.clone(),
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

    /// Return the flat environment overrides map for downstream consumers.
    /// Compatible with `iv8_core::KernelConfig::environment_overrides`.
    pub fn to_environment_overrides(&self) -> std::collections::HashMap<String, serde_json::Value> {
        self.flat_env.clone()
    }
}

fn build_flat_env(source: &ProfileSource) -> HashMap<String, serde_json::Value> {
    let mut env = HashMap::new();

    // Profile identity
    env.insert(
        "config.features.profile".into(),
        serde_json::json!("chrome147_win"),
    );
    env.insert(
        "config.features.browserVersion".into(),
        serde_json::Value::String(source.identity.browser.version.clone()),
    );

    // === navigator.* (native_env.rs getters) ===
    env.insert(
        "navigator.userAgent".into(),
        serde_json::Value::String(source.navigator.user_agent.clone()),
    );
    env.insert("navigator.appVersion".into(), serde_json::json!("5.0"));
    env.insert(
        "navigator.platform".into(),
        serde_json::Value::String(source.navigator.platform.clone()),
    );
    env.insert(
        "navigator.vendor".into(),
        serde_json::Value::String(source.navigator.vendor.clone()),
    );
    env.insert("navigator.vendorSub".into(), serde_json::json!(""));
    env.insert("navigator.product".into(), serde_json::json!("Gecko"));
    env.insert("navigator.productSub".into(), serde_json::json!("20030107"));
    env.insert(
        "navigator.language".into(),
        serde_json::Value::String(source.navigator.language.clone()),
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
    env.insert("navigator.cookieEnabled".into(), serde_json::json!(true));
    env.insert("navigator.onLine".into(), serde_json::json!(true));
    env.insert("navigator.doNotTrack".into(), serde_json::Value::Null);
    env.insert(
        "navigator.webdriver".into(),
        serde_json::json!(source.navigator.webdriver),
    );
    env.insert("navigator.appName".into(), serde_json::json!("Netscape"));
    env.insert("navigator.appCodeName".into(), serde_json::json!("Mozilla"));
    env.insert(
        "navigator.pdfViewerEnabled".into(),
        serde_json::json!(source.navigator.pdf_viewer_enabled),
    );
    env.insert(
        "navigator.languages".into(),
        serde_json::to_value(&source.navigator.languages).unwrap_or_default(),
    );

    // === navigator.userAgentData (user_agent_data.rs) ===
    env.insert(
        "navigator.userAgentData.brands".into(),
        serde_json::to_value(&source.navigator.user_agent_data.brands).unwrap_or_default(),
    );
    env.insert(
        "navigator.userAgentData.mobile".into(),
        serde_json::json!(source.navigator.user_agent_data.mobile),
    );
    env.insert(
        "navigator.userAgentData.platform".into(),
        serde_json::Value::String(source.navigator.user_agent_data.platform.clone()),
    );
    env.insert(
        "navigator.userAgentData.architecture".into(),
        serde_json::Value::String(source.navigator.user_agent_data.architecture.clone()),
    );
    env.insert(
        "navigator.userAgentData.bitness".into(),
        serde_json::Value::String(source.navigator.user_agent_data.bitness.clone()),
    );
    env.insert(
        "navigator.userAgentData.model".into(),
        serde_json::json!(""),
    );
    env.insert(
        "navigator.userAgentData.platformVersion".into(),
        serde_json::Value::String(source.navigator.user_agent_data.platform_version.clone()),
    );
    env.insert(
        "navigator.userAgentData.wow64".into(),
        serde_json::json!(false),
    );
    env.insert(
        "navigator.userAgentData.fullVersionList".into(),
        serde_json::to_value(&source.navigator.user_agent_data.full_version_list)
            .unwrap_or_default(),
    );

    // === screen.* (native_env.rs screen getters) ===
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
    env.insert("screen.availLeft".into(), serde_json::json!(0));
    env.insert("screen.availTop".into(), serde_json::json!(0));

    // === window.* ===
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
    env.insert(
        "window.outerWidth".into(),
        serde_json::json!(source.display.window.outer_width),
    );
    env.insert(
        "window.outerHeight".into(),
        serde_json::json!(source.display.window.outer_height),
    );

    // === location.* (location.rs) ===
    env.insert("location.href".into(), serde_json::json!("about:blank"));
    env.insert("location.origin".into(), serde_json::json!("null"));
    env.insert("location.protocol".into(), serde_json::json!("about:"));
    env.insert("location.host".into(), serde_json::json!(""));
    env.insert("location.hostname".into(), serde_json::json!(""));
    env.insert("location.port".into(), serde_json::json!(""));
    env.insert("location.pathname".into(), serde_json::json!("blank"));
    env.insert("location.search".into(), serde_json::json!(""));
    env.insert("location.hash".into(), serde_json::json!(""));

    // === webgl.* (webgl.rs getParameter) ===
    env.insert(
        "webgl.VENDOR".into(),
        serde_json::Value::String(source.identity.gpu.vendor.clone()),
    );
    env.insert(
        "webgl.RENDERER".into(),
        serde_json::Value::String(source.identity.gpu.renderer.clone()),
    );
    env.insert(
        "webgl.UNMASKED_VENDOR_WEBGL".into(),
        serde_json::Value::String(source.identity.gpu.webgl_unmasked_vendor.clone()),
    );
    env.insert(
        "webgl.UNMASKED_RENDERER_WEBGL".into(),
        serde_json::Value::String(source.identity.gpu.webgl_unmasked_renderer.clone()),
    );

    // === rendering modes ===
    env.insert(
        "canvas.mode".into(),
        serde_json::Value::String(source.rendering.canvas_2d.mode.clone()),
    );
    env.insert(
        "webgl.mode".into(),
        serde_json::Value::String(source.rendering.webgl_1.mode.clone()),
    );

    // === permissions.* (native_env.rs permissions_query_cb) ===
    // Emit all 7 named fields + all extra fields as dot-path keys.
    // The runtime reads `permissions.<name>` and falls back to "prompt".
    for (name, state) in [
        ("geolocation", &source.permissions.geolocation),
        ("notifications", &source.permissions.notifications),
        ("camera", &source.permissions.camera),
        ("microphone", &source.permissions.microphone),
        ("clipboard-read", &source.permissions.clipboard_read),
        ("clipboard-write", &source.permissions.clipboard_write),
        ("local-fonts", &source.permissions.local_fonts),
    ] {
        env.insert(
            format!("permissions.{}", name),
            serde_json::Value::String(state.clone()),
        );
    }
    for (name, state) in &source.permissions.extra {
        env.insert(
            format!("permissions.{}", name),
            serde_json::Value::String(state.clone()),
        );
    }

    // === media.* (geometry.rs matchMedia shim) ===
    // Emit all 16 media preference fields as dot-path keys.
    for (name, val) in [
        ("pointer", &source.display.media.pointer),
        ("hover", &source.display.media.hover),
        ("color-gamut", &source.display.media.color_gamut),
        ("prefers-color-scheme", &source.display.media.prefers_color_scheme),
        ("prefers-contrast", &source.display.media.prefers_contrast),
        ("prefers-reduced-motion", &source.display.media.prefers_reduced_motion),
        ("prefers-reduced-data", &source.display.media.prefers_reduced_data),
        ("forced-colors", &source.display.media.forced_colors),
        ("dynamic-range", &source.display.media.dynamic_range),
        ("scripting", &source.display.media.scripting),
        ("update", &source.display.media.update),
        ("any-pointer", &source.display.media.any_pointer),
        ("any-hover", &source.display.media.any_hover),
        ("display-mode", &source.display.media.display_mode),
        ("inverted-colors", &source.display.media.inverted_colors),
        ("prefers-reduced-transparency", &source.display.media.prefers_reduced_transparency),
    ] {
        env.insert(
            format!("media.{}", name),
            serde_json::Value::String(val.clone()),
        );
    }

    // === fonts.* (canvas/binding.rs measureText + document.fonts) ===
    env.insert(
        "fonts.mode".into(),
        serde_json::Value::String(source.rendering.fonts.mode.clone()),
    );
    env.insert(
        "fonts.families".into(),
        serde_json::to_value(&source.rendering.fonts.families).unwrap_or_default(),
    );

    // === audio.* (audio_context.rs) ===
    env.insert(
        "audio.mode".into(),
        serde_json::Value::String(source.rendering.audio_context.mode.clone()),
    );

    // === timers ===
    let raf_interval_ms = if source.timing.fps > 0 {
        1000.0 / source.timing.fps as f64
    } else {
        16.67
    };
    env.insert(
        "timers.raf_interval_ms".into(),
        serde_json::json!(raf_interval_ms),
    );
    env.insert("timers.min_interval_ms".into(), serde_json::json!(1.0));

    // === compat overrides ===
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
        assert!(
            matrix.flat_env.len() > 50,
            "expanded env should have >50 keys, got {}",
            matrix.flat_env.len()
        );
        assert_eq!(
            matrix
                .flat_env
                .get("navigator.userAgent")
                .and_then(|v| v.as_str()),
            Some(source.navigator.user_agent.as_str())
        );
        // Verify critical expanded keys exist
        assert!(matrix
            .flat_env
            .contains_key("navigator.userAgentData.brands"));
        assert!(matrix.flat_env.contains_key("webgl.UNMASKED_VENDOR_WEBGL"));
        assert!(matrix.flat_env.contains_key("screen.availLeft"));
        assert!(matrix.flat_env.contains_key("timers.raf_interval_ms"));
    }

    #[test]
    fn materialization_derives_seeds() {
        let source = default_profile_source();
        let (matrix, _) = ProfileMatrix::from_source(&source);
        assert_eq!(matrix.seeds.len(), 5);
        assert_ne!(matrix.seeds["canvas"], matrix.seeds["webgl1"]);
        assert_eq!(matrix.rendering.canvas_seed, matrix.seeds["canvas"]);
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

    #[test]
    fn materialization_honors_surface_sub_seed() {
        let mut source = default_profile_source();
        source.rendering.canvas_2d.sub_seed = Some(42);
        let (matrix, _) = ProfileMatrix::from_source(&source);
        assert_eq!(matrix.seeds["canvas"], 42);
        assert_eq!(matrix.rendering.canvas_seed, 42);
    }

    #[test]
    fn materialization_derives_raf_interval_from_fps() {
        let mut source = default_profile_source();
        source.timing.fps = 120;
        let (matrix, _) = ProfileMatrix::from_source(&source);
        let raf = matrix
            .flat_env
            .get("timers.raf_interval_ms")
            .and_then(|v| v.as_f64())
            .expect("raf interval should be present");
        assert!((raf - 8.333333333333334).abs() < f64::EPSILON);
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
    pub camera: String,
    pub microphone: String,
    pub clipboard_read: String,
    pub clipboard_write: String,
    pub local_fonts: String,
    pub extra: std::collections::HashMap<String, String>,
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
