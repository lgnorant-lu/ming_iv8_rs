//! v0.8.93 B2 — override-order matrix for D-111 identity paths.
//!
//! Order under test for native high-signal getters:
//!   environment_overrides (user_keys) > BrowserProfile > DEFAULT_PROFILE
//!
//! Channels intentionally not mixed with late JS UserOverrides here.

mod common;

use iv8_core::kernel::embedded_v8::EmbeddedV8Kernel;
use iv8_core::kernel::KernelConfig;
use iv8_core::shims::browser_profile::{BrowserProfile, DEFAULT_PROFILE};
use std::collections::HashMap;

fn profile_with_markers() -> BrowserProfile {
    BrowserProfile {
        user_agent: "ProfileAgent/1.0",
        platform: "ProfileOS",
        language: "profile-LANG",
        languages: &["profile-LANG", "en"],
        hardware_concurrency: 4.0,
        device_memory: 2.0,
        screen_width: 1111.0,
        screen_height: 777.0,
        window_inner_width: 801.0,
        window_inner_height: 601.0,
        window_outer_width: 901.0,
        window_outer_height: 701.0,
        device_pixel_ratio: 1.75,
        ua_platform: "ProfilePlatform",
        ..DEFAULT_PROFILE
    }
}

#[test]
fn test_env_override_beats_browser_profile_for_navigator() {
    let mut overrides = HashMap::new();
    overrides.insert(
        "navigator.userAgent".to_string(),
        serde_json::json!("EnvAgent/9.9"),
    );
    overrides.insert(
        "navigator.platform".to_string(),
        serde_json::json!("EnvOS"),
    );
    overrides.insert(
        "navigator.language".to_string(),
        serde_json::json!("env-LANG"),
    );
    overrides.insert(
        "navigator.hardwareConcurrency".to_string(),
        serde_json::json!(16),
    );

    let config = KernelConfig {
        environment_overrides: Some(overrides),
        browser_profile: Some(Box::new(profile_with_markers())),
        ..Default::default()
    };
    let mut k = EmbeddedV8Kernel::new(config).unwrap();

    common::assert_js_str(&mut k, "navigator.userAgent", "EnvAgent/9.9");
    common::assert_js_str(&mut k, "navigator.platform", "EnvOS");
    common::assert_js_str(&mut k, "navigator.language", "env-LANG");
    common::assert_js_str(&mut k, "navigator.hardwareConcurrency", "16");
}

#[test]
fn test_env_override_beats_browser_profile_for_window_dims() {
    let mut overrides = HashMap::new();
    overrides.insert(
        "window.innerWidth".to_string(),
        serde_json::json!(1234.0),
    );
    overrides.insert(
        "window.innerHeight".to_string(),
        serde_json::json!(567.0),
    );
    overrides.insert(
        "window.devicePixelRatio".to_string(),
        serde_json::json!(3.0),
    );

    let config = KernelConfig {
        environment_overrides: Some(overrides),
        browser_profile: Some(Box::new(profile_with_markers())),
        ..Default::default()
    };
    let mut k = EmbeddedV8Kernel::new(config).unwrap();

    common::assert_js_str(&mut k, "window.innerWidth", "1234");
    common::assert_js_str(&mut k, "window.innerHeight", "567");
    common::assert_js_str(&mut k, "window.devicePixelRatio", "3");
}

#[test]
fn test_env_override_beats_default_for_window_scroll_and_screen_pos() {
    let mut overrides = HashMap::new();
    overrides.insert("window.scrollX".to_string(), serde_json::json!(42.0));
    overrides.insert("window.scrollY".to_string(), serde_json::json!(7.0));
    overrides.insert("window.screenX".to_string(), serde_json::json!(10.0));
    // intentionally no pageXOffset / screenLeft keys — alias paths

    let config = KernelConfig {
        environment_overrides: Some(overrides),
        browser_profile: Some(Box::new(profile_with_markers())),
        ..Default::default()
    };
    let mut k = EmbeddedV8Kernel::new(config).unwrap();

    common::assert_js_str(&mut k, "window.scrollX", "42");
    common::assert_js_str(&mut k, "window.scrollY", "7");
    common::assert_js_str(&mut k, "window.screenX", "10");
    common::assert_js_str(&mut k, "window.pageXOffset", "42");
    common::assert_js_str(&mut k, "window.screenLeft", "10");
}

#[test]
fn test_browser_profile_beats_default_when_no_user_override() {
    let config = KernelConfig {
        browser_profile: Some(Box::new(profile_with_markers())),
        ..Default::default()
    };
    let mut k = EmbeddedV8Kernel::new(config).unwrap();

    common::assert_js_str(&mut k, "navigator.userAgent", "ProfileAgent/1.0");
    common::assert_js_str(&mut k, "navigator.platform", "ProfileOS");
    common::assert_js_str(&mut k, "navigator.language", "profile-LANG");
    common::assert_js_str(&mut k, "navigator.hardwareConcurrency", "4");
    common::assert_js_str(&mut k, "navigator.deviceMemory", "2");
    common::assert_js_str(&mut k, "screen.width", "1111");
    common::assert_js_str(&mut k, "screen.height", "777");
    common::assert_js_str(&mut k, "window.innerWidth", "801");
    common::assert_js_str(&mut k, "window.innerHeight", "601");
    common::assert_js_str(&mut k, "window.devicePixelRatio", "1.75");
}

#[test]
fn test_env_override_beats_profile_for_languages() {
    let mut overrides = HashMap::new();
    overrides.insert(
        "navigator.languages".to_string(),
        serde_json::json!(["env-A", "env-B"]),
    );

    let config = KernelConfig {
        environment_overrides: Some(overrides),
        browser_profile: Some(Box::new(profile_with_markers())),
        ..Default::default()
    };
    let mut k = EmbeddedV8Kernel::new(config).unwrap();

    common::assert_js_str(&mut k, "navigator.languages[0]", "env-A");
    common::assert_js_str(&mut k, "navigator.languages[1]", "env-B");
    common::assert_js_str(&mut k, "navigator.languages.length", "2");
}

#[test]
fn test_profile_languages_when_no_user_override() {
    let config = KernelConfig {
        browser_profile: Some(Box::new(profile_with_markers())),
        ..Default::default()
    };
    let mut k = EmbeddedV8Kernel::new(config).unwrap();

    common::assert_js_str(&mut k, "navigator.languages[0]", "profile-LANG");
    common::assert_js_str(&mut k, "navigator.languages[1]", "en");
}

#[test]
fn test_uadata_platform_follows_profile_when_no_user_override() {
    let config = KernelConfig {
        browser_profile: Some(Box::new(profile_with_markers())),
        ..Default::default()
    };
    let mut k = EmbeddedV8Kernel::new(config).unwrap();

    common::assert_js_str(
        &mut k,
        "navigator.userAgentData.platform",
        "ProfilePlatform",
    );
}
