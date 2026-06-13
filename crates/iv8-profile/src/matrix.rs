use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
