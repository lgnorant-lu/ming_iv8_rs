use crate::source::*;

const CHROME_147_WIN10_UA: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
     (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36";

pub fn default_profile_source() -> ProfileSource {
    ProfileSource {
        meta: MetaSection {
            schema_version: "0.8.32".into(),
            name: "chrome147_win10_default".into(),
            description: "Default profile derived from iv8-rs Chrome 147/Windows 10 env defaults".into(),
            profile_version: "1".into(),
            provenance: "derived_from_iv8_defaults".into(),
        },
        identity: IdentitySection {
            os: "windows".into(),
            os_version: "10.0.0".into(),
            cpu_arch: "x64".into(),
            cpu_cores: 8,
            memory_gb: 8,
            browser: BrowserIdentity::default(),
            gpu: GpuIdentity {
                vendor: "NVIDIA".into(),
                renderer: "NVIDIA GeForce GTX 1650".into(),
                webgl_unmasked_vendor: "Google Inc. (NVIDIA)".into(),
                webgl_unmasked_renderer: concat!(
                    "ANGLE (NVIDIA, NVIDIA GeForce GTX 1650 (0x00001F82) ",
                    "Direct3D11 vs_5_0 ps_5_0, D3D11)"
                ).into(),
            },
            noise_seed: 514829086,
        },
        navigator: NavigatorSection {
            user_agent: CHROME_147_WIN10_UA.into(),
            platform: "Win32".into(),
            vendor: "Google Inc.".into(),
            language: "zh-CN".into(),
            languages: vec!["zh-CN".into(), "en".into()],
            hardware_concurrency: 8,
            device_memory: 8,
            max_touch_points: 0,
            webdriver: false,
            pdf_viewer_enabled: true,
            user_agent_data: UserAgentData {
                platform: "Windows".into(),
                platform_version: "10.0.0".into(),
                architecture: "x86".into(),
                bitness: "64".into(),
                mobile: false,
                brands: vec![
                    BrandEntry {
                        brand: "Chromium".into(),
                        version: "147".into(),
                    },
                    BrandEntry {
                        brand: "Google Chrome".into(),
                        version: "147".into(),
                    },
                ],
                full_version_list: vec![
                    BrandEntry {
                        brand: "Chromium".into(),
                        version: "147.0.7727.116".into(),
                    },
                    BrandEntry {
                        brand: "Google Chrome".into(),
                        version: "147.0.7727.116".into(),
                    },
                ],
            },
            connection: ConnectionInfo::default(),
        },
        display: DisplaySection {
            screen: ScreenInfo {
                width: 1920,
                height: 1080,
                avail_width: 1920,
                avail_height: 1040,
                color_depth: 24,
                pixel_depth: 24,
            },
            window: WindowInfo {
                inner_width: 1920,
                inner_height: 969,
                outer_width: 1920,
                outer_height: 1080,
                device_pixel_ratio: 1.0,
            },
            media: MediaPreferences {
                pointer: "fine".into(),
                hover: "hover".into(),
                color_gamut: "srgb".into(),
                prefers_color_scheme: "light".into(),
            },
        },
        rendering: RenderingSection {
            canvas_2d: SignalMode {
                mode: "noise".into(),
                sub_seed: None,
            },
            webgl_1: SignalMode {
                mode: "noise".into(),
                sub_seed: None,
            },
            webgl_2: SignalMode {
                mode: "noise".into(),
                sub_seed: None,
            },
            webgpu: WebGpuMode {
                mode: "unsupported".into(),
            },
            audio_context: SignalMode {
                mode: "noise".into(),
                sub_seed: None,
            },
            client_rects: SignalMode {
                mode: "noise".into(),
                sub_seed: None,
            },
            fonts: FontsConfig {
                mode: "common".into(),
                families: vec![],
            },
        },
        locale: LocaleSection {
            timezone: "Asia/Shanghai".into(),
            language: "zh-CN".into(),
            languages: vec!["zh-CN".into(), "en".into()],
            accept_language: "zh-CN,zh;q=0.9,en;q=0.8".into(),
            geolocation: GeolocationConfig {
                mode: "prompt".into(),
                based_on_ip: true,
            },
        },
        network: NetworkSection {
            proxy: None,
            webrtc: WebRtcConfig {
                mode: "disabled".into(),
            },
            dns: DnsConfig {
                mode: "system".into(),
            },
            headers: HeadersConfig {
                ua: "profile".into(),
                accept_language: "profile".into(),
                client_hints: "profile".into(),
            },
            tls: TlsConfig {
                mode: "unsupported".into(),
            },
        },
        permissions: PermissionsSection {
            geolocation: "prompt".into(),
            notifications: "prompt".into(),
            camera: "prompt".into(),
            microphone: "prompt".into(),
            clipboard_read: "prompt".into(),
            clipboard_write: "granted".into(),
            local_fonts: "prompt".into(),
        },
        capabilities: CapabilitiesSection {
            window_chrome: true,
            notifications: true,
            battery: false,
            bluetooth: false,
            webgpu: false,
            media_devices: true,
            storage: true,
        },
        storage: StorageSection {
            local_storage: true,
            session_storage: true,
            indexed_db: true,
            cookies: true,
            history_length: 1,
        },
        timing: TimingSection {
            mode: "logical".into(),
            fps: 60,
            performance_timing: "generated".into(),
        },
        compat: CompatSection::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_profile_has_chrome_ua() {
        let p = default_profile_source();
        assert!(p.identity.browser.brand == "chrome");
        assert!(p.navigator.user_agent.contains("Chrome/147"));
        assert!(p.navigator.platform == "Win32");
    }

    #[test]
    fn default_profile_roundtrip() {
        let p = default_profile_source();
        let json = serde_json::to_string_pretty(&p).expect("serialize");
        let back: ProfileSource = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.meta.schema_version, "0.8.32");
        assert_eq!(back.identity.noise_seed, 514829086);
    }

    #[test]
    fn unknown_top_level_field_rejected() {
        let json = r#"{"meta":{"schema_version":"1"},"bogus":true}"#;
        let result: Result<ProfileSource, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }
}
