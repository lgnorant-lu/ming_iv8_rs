use serde::{Deserialize, Serialize};

/// User-authored profile JSON.
///
/// Compact, strict, reviewable. `deny_unknown_fields` ensures unknown top-level
/// fields are rejected unless placed under `compat`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileSource {
    pub meta: MetaSection,
    pub identity: IdentitySection,
    pub navigator: NavigatorSection,
    pub display: DisplaySection,
    pub rendering: RenderingSection,
    pub locale: LocaleSection,
    pub network: NetworkSection,
    pub permissions: PermissionsSection,
    pub capabilities: CapabilitiesSection,
    pub storage: StorageSection,
    pub timing: TimingSection,
    #[serde(default)]
    pub compat: CompatSection,
}

// ---------------------------------------------------------------------------
// Meta
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MetaSection {
    pub schema_version: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub profile_version: String,
    #[serde(default)]
    pub provenance: String,
}

// ---------------------------------------------------------------------------
// Identity
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IdentitySection {
    pub os: String,
    pub os_version: String,
    pub cpu_arch: String,
    pub cpu_cores: u32,
    pub memory_gb: u32,
    pub browser: BrowserIdentity,
    pub gpu: GpuIdentity,
    pub noise_seed: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BrowserIdentity {
    pub brand: String,
    pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GpuIdentity {
    pub vendor: String,
    pub renderer: String,
    pub webgl_unmasked_vendor: String,
    pub webgl_unmasked_renderer: String,
}

// ---------------------------------------------------------------------------
// Navigator
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NavigatorSection {
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
    #[serde(default)]
    pub user_agent_data: UserAgentData,
    pub connection: ConnectionInfo,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UserAgentData {
    pub platform: String,
    pub platform_version: String,
    pub architecture: String,
    pub bitness: String,
    pub mobile: bool,
    pub brands: Vec<BrandEntry>,
    pub full_version_list: Vec<BrandEntry>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BrandEntry {
    pub brand: String,
    pub version: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectionInfo {
    pub effective_type: String,
    pub rtt: u32,
    pub downlink: f64,
    pub save_data: bool,
}

// ---------------------------------------------------------------------------
// Display
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DisplaySection {
    pub screen: ScreenInfo,
    pub window: WindowInfo,
    pub media: MediaPreferences,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScreenInfo {
    pub width: u32,
    pub height: u32,
    pub avail_width: u32,
    pub avail_height: u32,
    pub color_depth: u32,
    pub pixel_depth: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WindowInfo {
    pub inner_width: u32,
    pub inner_height: u32,
    pub outer_width: u32,
    pub outer_height: u32,
    pub device_pixel_ratio: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MediaPreferences {
    pub pointer: String,
    pub hover: String,
    pub color_gamut: String,
    pub prefers_color_scheme: String,
}

// ---------------------------------------------------------------------------
// Rendering
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RenderingSection {
    pub canvas_2d: SignalMode,
    pub webgl_1: SignalMode,
    pub webgl_2: SignalMode,
    pub webgpu: WebGpuMode,
    pub audio_context: SignalMode,
    pub client_rects: SignalMode,
    pub fonts: FontsConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignalMode {
    pub mode: String,
    #[serde(default)]
    pub sub_seed: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebGpuMode {
    pub mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FontsConfig {
    pub mode: String,
    #[serde(default)]
    pub families: Vec<String>,
}

// ---------------------------------------------------------------------------
// Locale
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LocaleSection {
    pub timezone: String,
    pub language: String,
    pub languages: Vec<String>,
    pub accept_language: String,
    pub geolocation: GeolocationConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GeolocationConfig {
    pub mode: String,
    #[serde(default)]
    pub based_on_ip: bool,
}

// ---------------------------------------------------------------------------
// Network
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NetworkSection {
    pub proxy: Option<ProxyConfig>,
    pub webrtc: WebRtcConfig,
    pub dns: DnsConfig,
    pub headers: HeadersConfig,
    pub tls: TlsConfig,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProxyConfig {
    pub host: String,
    pub port: u16,
    #[serde(default)]
    pub username: Option<String>,
    #[serde(default)]
    pub password: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WebRtcConfig {
    pub mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DnsConfig {
    pub mode: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeadersConfig {
    pub ua: String,
    pub accept_language: String,
    pub client_hints: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TlsConfig {
    pub mode: String,
}

// ---------------------------------------------------------------------------
// Permissions
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PermissionsSection {
    pub geolocation: String,
    pub notifications: String,
    pub camera: String,
    pub microphone: String,
    #[serde(rename = "clipboard-read")]
    pub clipboard_read: String,
    #[serde(rename = "clipboard-write")]
    pub clipboard_write: String,
    #[serde(rename = "local-fonts")]
    pub local_fonts: String,
}

// ---------------------------------------------------------------------------
// Capabilities
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilitiesSection {
    pub window_chrome: bool,
    pub notifications: bool,
    pub battery: bool,
    pub bluetooth: bool,
    pub webgpu: bool,
    pub media_devices: bool,
    pub storage: bool,
}

// ---------------------------------------------------------------------------
// Storage
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StorageSection {
    pub local_storage: bool,
    pub session_storage: bool,
    pub indexed_db: bool,
    pub cookies: bool,
    pub history_length: u32,
}

// ---------------------------------------------------------------------------
// Timing
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TimingSection {
    pub mode: String,
    pub fps: u32,
    pub performance_timing: String,
}

// ---------------------------------------------------------------------------
// Compat
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompatSection {
    #[serde(default)]
    pub flat_env_overrides: std::collections::HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub features: std::collections::HashMap<String, serde_json::Value>,
}

impl Default for CompatSection {
    fn default() -> Self {
        Self {
            flat_env_overrides: std::collections::HashMap::new(),
            features: std::collections::HashMap::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defaults for sections that could have sensible defaults
// ---------------------------------------------------------------------------

impl Default for UserAgentData {
    fn default() -> Self {
        Self {
            platform: String::new(),
            platform_version: String::new(),
            architecture: String::new(),
            bitness: String::new(),
            mobile: false,
            brands: Vec::new(),
            full_version_list: Vec::new(),
        }
    }
}

impl Default for ConnectionInfo {
    fn default() -> Self {
        Self {
            effective_type: "4g".into(),
            rtt: 50,
            downlink: 10.0,
            save_data: false,
        }
    }
}

impl Default for BrowserIdentity {
    fn default() -> Self {
        Self {
            brand: "chrome".into(),
            version: "147.0.7727.116".into(),
        }
    }
}
