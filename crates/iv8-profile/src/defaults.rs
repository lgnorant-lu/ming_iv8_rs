use crate::source::*;

const CHROME_147_WIN10_UA: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
     (KHTML, like Gecko) Chrome/147.0.0.0 Safari/537.36";

/// Default WebGL extensions for Chrome 147 desktop (NVIDIA GPU via ANGLE/D3D11).
pub const WEBGL_EXTENSIONS_JSON: &str = r#"["ANGLE_instanced_arrays","EXT_blend_minmax","EXT_color_buffer_half_float","EXT_disjoint_timer_query","EXT_float_blend","EXT_frag_depth","EXT_shader_texture_lod","EXT_texture_compression_bptc","EXT_texture_compression_rgtc","EXT_texture_filter_anisotropic","EXT_sRGB","OES_element_index_uint","OES_fbo_render_mipmap","OES_standard_derivatives","OES_texture_float","OES_texture_float_linear","OES_texture_half_float","OES_texture_half_float_linear","OES_vertex_array_object","WEBGL_color_buffer_float","WEBGL_compressed_texture_s3tc","WEBGL_compressed_texture_s3tc_srgb","WEBGL_debug_renderer_info","WEBGL_debug_shaders","WEBGL_depth_texture","WEBGL_draw_buffers","WEBGL_lose_context","WEBGL_multi_draw"]"#;

pub fn default_profile_source() -> ProfileSource {
    ProfileSource {
        meta: MetaSection {
            schema_version: "0.8.32".into(),
            name: "chrome147_win10_default".into(),
            description: "Default profile derived from iv8-rs Chrome 147/Windows 10 env defaults"
                .into(),
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
                renderer: "NVIDIA GeForce RTX 4060".into(),
                webgl_unmasked_vendor: "Google Inc. (NVIDIA)".into(),
                webgl_unmasked_renderer: concat!(
                    "ANGLE (NVIDIA, NVIDIA GeForce RTX 4060 (0x00002882) ",
                    "Direct3D11 vs_5_0 ps_5_0, D3D11)"
                )
                .into(),
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
                prefers_contrast: "no-preference".into(),
                prefers_reduced_motion: "no-preference".into(),
                prefers_reduced_data: "no-preference".into(),
                forced_colors: "none".into(),
                dynamic_range: "srgb".into(),
                scripting: "yes".into(),
                update: "fast".into(),
                any_pointer: "coarse".into(),
                any_hover: "none".into(),
                display_mode: "browser".into(),
                inverted_colors: "none".into(),
                prefers_reduced_transparency: "no-preference".into(),
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
            extra: {
                let mut m = std::collections::HashMap::new();
                // Auto-granted (Chrome default)
                m.insert("accelerometer".into(), "granted".into());
                m.insert("gyroscope".into(), "granted".into());
                m.insert("magnetometer".into(), "granted".into());
                m.insert("ambient-light-sensor".into(), "granted".into());
                m.insert("background-sync".into(), "granted".into());
                m.insert("midi".into(), "granted".into());
                m.insert("screen-wake-lock".into(), "granted".into());
                // Prompt (Chrome default)
                m.insert("push".into(), "prompt".into());
                m.insert("bluetooth".into(), "prompt".into());
                m.insert("persistent-storage".into(), "prompt".into());
                m.insert("idle-detection".into(), "prompt".into());
                m.insert("nfc".into(), "prompt".into());
                m.insert("storage-access".into(), "prompt".into());
                m.insert("window-management".into(), "prompt".into());
                m.insert("payment-handler".into(), "prompt".into());
                m.insert("periodic-background-sync".into(), "prompt".into());
                m
            },
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
