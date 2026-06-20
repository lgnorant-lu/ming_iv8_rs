use crate::source::ProfileSource;
use std::fmt;

/// Result of profile validation containing errors and warnings.
#[derive(Clone, Debug)]
pub struct ValidationResult {
    pub errors: Vec<ValidationIssue>,
    pub warnings: Vec<ValidationIssue>,
}

impl ValidationResult {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty() && self.warnings.is_empty()
    }

    pub fn error_count(&self) -> usize {
        self.errors.len()
    }

    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }

    pub fn error(mut self, issue: ValidationIssue) -> Self {
        self.errors.push(issue);
        self
    }

    pub fn warn(mut self, issue: ValidationIssue) -> Self {
        self.warnings.push(issue);
        self
    }

    pub fn merge(mut self, other: Self) -> Self {
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        self
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ValidationIssue {
    pub path: String,
    pub message: String,
}

impl ValidationIssue {
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}

impl fmt::Display for ValidationIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.path, self.message)
    }
}

impl fmt::Display for ValidationResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for e in &self.errors {
            writeln!(f, "ERROR: {}", e)?;
        }
        for w in &self.warnings {
            writeln!(f, "WARN:  {}", w)?;
        }
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// IANA timezone directory (commonly used subset)
// ---------------------------------------------------------------------------

const COMMON_TZ: &[&str] = &[
    "Africa/Abidjan",
    "Africa/Cairo",
    "Africa/Casablanca",
    "Africa/Johannesburg",
    "Africa/Lagos",
    "Africa/Nairobi",
    "America/Argentina/Buenos_Aires",
    "America/Chicago",
    "America/Denver",
    "America/Indianapolis",
    "America/Los_Angeles",
    "America/Mexico_City",
    "America/New_York",
    "America/Sao_Paulo",
    "America/Toronto",
    "America/Vancouver",
    "Asia/Bangkok",
    "Asia/Dubai",
    "Asia/Hong_Kong",
    "Asia/Jakarta",
    "Asia/Kolkata",
    "Asia/Seoul",
    "Asia/Shanghai",
    "Asia/Singapore",
    "Asia/Tokyo",
    "Australia/Melbourne",
    "Australia/Sydney",
    "Europe/Berlin",
    "Europe/Istanbul",
    "Europe/London",
    "Europe/Moscow",
    "Europe/Paris",
    "Europe/Rome",
    "Europe/Stockholm",
    "Pacific/Auckland",
    "Pacific/Honolulu",
    "UTC",
];

fn is_valid_iana_tz(tz: &str) -> bool {
    COMMON_TZ.contains(&tz)
}

// ---------------------------------------------------------------------------
// Coherence validation
// ---------------------------------------------------------------------------

pub fn validate(source: &ProfileSource) -> ValidationResult {
    let mut r = ValidationResult::default();

    // 1. UA must contain browser brand
    let brand_lower = source.identity.browser.brand.to_lowercase();
    let ua_lower = source.navigator.user_agent.to_lowercase();
    if !ua_lower.contains(&brand_lower) {
        r = r.error(ValidationIssue::new(
            "navigator.user_agent",
            format!(
                "UA does not contain browser brand '{}'",
                source.identity.browser.brand
            ),
        ));
    }

    // 2. UA must contain OS name
    let os_name = os_display_name(&source.identity.os);
    if !ua_lower.contains(&os_name.to_lowercase()) {
        r = r.warn(ValidationIssue::new(
            "navigator.user_agent",
            format!("UA does not contain OS name '{}'", os_name),
        ));
    }

    // 3. navigator.platform should match OS
    let expected_platform = os_platform(&source.identity.os);
    if source.navigator.platform != expected_platform {
        r = r.warn(ValidationIssue::new(
            "navigator.platform",
            format!(
                "expected '{}' for OS '{}', got '{}'",
                expected_platform, source.identity.os, source.navigator.platform
            ),
        ));
    }

    // 4. vendor must match browser family
    match source.identity.browser.brand.to_lowercase().as_str() {
        "chrome" | "chromium" => {
            if source.navigator.vendor != "Google Inc." {
                r = r.warn(ValidationIssue::new(
                    "navigator.vendor",
                    format!(
                        "expected 'Google Inc.' for chrome, got '{}'",
                        source.navigator.vendor
                    ),
                ));
            }
        }
        "firefox" => {
            if source.navigator.vendor != "" {
                r = r.warn(ValidationIssue::new(
                    "navigator.vendor",
                    format!("expected '' for firefox, got '{}'", source.navigator.vendor),
                ));
            }
        }
        _ => {}
    }

    // 5. Language coherence
    if source.navigator.language != source.locale.language {
        r = r.error(ValidationIssue::new(
            "locale.language",
            format!(
                "navigator.language '{}' != locale.language '{}'",
                source.navigator.language, source.locale.language
            ),
        ));
    }
    if !source.navigator.languages.is_empty()
        && source.navigator.languages[0] != source.locale.language
    {
        r = r.warn(ValidationIssue::new(
            "locale.language",
            format!(
                "navigator.languages[0] '{}' != locale.language '{}'",
                source.navigator.languages[0], source.locale.language
            ),
        ));
    }

    // 6. Timezone validation
    if !is_valid_iana_tz(&source.locale.timezone) {
        r = r.warn(ValidationIssue::new(
            "locale.timezone",
            format!(
                "'{}' is not in the common IANA timezone list",
                source.locale.timezone
            ),
        ));
    }

    // 7. Screen size hierarchy: screen >= avail >= inner? outer >= inner?
    let scr = &source.display.screen;
    let win = &source.display.window;
    if scr.width < scr.avail_width {
        r = r.warn(ValidationIssue::new(
            "display.screen.width < display.screen.avail_width",
            format!("{} < {}", scr.width, scr.avail_width),
        ));
    }
    if scr.height < scr.avail_height {
        r = r.warn(ValidationIssue::new(
            "display.screen.height < display.screen.avail_height",
            format!("{} < {}", scr.height, scr.avail_height),
        ));
    }
    if win.outer_width < win.inner_width {
        r = r.warn(ValidationIssue::new(
            "display.window.outer_width < display.window.inner_width",
            format!("{} < {}", win.outer_width, win.inner_width),
        ));
    }
    if win.outer_height < win.inner_height {
        r = r.warn(ValidationIssue::new(
            "display.window.outer_height < display.window.inner_height",
            format!("{} < {}", win.outer_height, win.inner_height),
        ));
    }

    // 8. Non-zero hardware values
    if scr.width == 0 {
        r = r.error(ValidationIssue::new("display.screen.width", "must be > 0"));
    }
    if scr.height == 0 {
        r = r.error(ValidationIssue::new("display.screen.height", "must be > 0"));
    }
    if win.device_pixel_ratio <= 0.0 {
        r = r.error(ValidationIssue::new(
            "display.window.device_pixel_ratio",
            "must be > 0",
        ));
    }
    if source.identity.cpu_cores == 0 {
        r = r.error(ValidationIssue::new("identity.cpu_cores", "must be > 0"));
    }
    if source.identity.memory_gb == 0 {
        r = r.error(ValidationIssue::new("identity.memory_gb", "must be > 0"));
    }

    if source.timing.fps == 0 {
        r = r.error(ValidationIssue::new("timing.fps", "must be > 0"));
    }

    // 9. Mode enum validation.
    for (field, val, allowed) in [
        (
            "rendering.canvas_2d.mode",
            source.rendering.canvas_2d.mode.as_str(),
            &["none", "stable", "noise"] as &[&str],
        ),
        (
            "rendering.webgl_1.mode",
            source.rendering.webgl_1.mode.as_str(),
            &["none", "stable", "noise"],
        ),
        (
            "rendering.webgl_2.mode",
            source.rendering.webgl_2.mode.as_str(),
            &["none", "stable", "noise"],
        ),
        (
            "rendering.audio_context.mode",
            source.rendering.audio_context.mode.as_str(),
            &["none", "stable", "noise"],
        ),
        (
            "rendering.client_rects.mode",
            source.rendering.client_rects.mode.as_str(),
            &["none", "stable", "noise"],
        ),
        (
            "rendering.webgpu.mode",
            source.rendering.webgpu.mode.as_str(),
            &["unsupported", "supported"],
        ),
        (
            "rendering.fonts.mode",
            source.rendering.fonts.mode.as_str(),
            &["none", "common", "custom"],
        ),
        (
            "timing.mode",
            source.timing.mode.as_str(),
            &["logical", "frozen", "real"],
        ),
        (
            "network.webrtc.mode",
            source.network.webrtc.mode.as_str(),
            &["disabled", "proxy", "real"],
        ),
    ] {
        if !allowed.contains(&val) {
            r = r.error(ValidationIssue::new(
                field,
                format!("unexpected mode '{}'; allowed: {}", val, allowed.join(", ")),
            ));
        }
    }

    // 13. WebRTC vs proxy coherence
    if source.network.webrtc.mode == "real" && source.network.proxy.is_none() {
        r = r.warn(ValidationIssue::new(
            "network.webrtc",
            "webrtc mode is 'real' but no proxy is configured",
        ));
    }

    // 14. Permissions state validity
    let valid_states = ["granted", "denied", "prompt"];
    for (field, val) in [
        ("permissions.geolocation", &source.permissions.geolocation),
        (
            "permissions.notifications",
            &source.permissions.notifications,
        ),
        ("permissions.camera", &source.permissions.camera),
        ("permissions.microphone", &source.permissions.microphone),
        (
            "permissions.clipboard-read",
            &source.permissions.clipboard_read,
        ),
        (
            "permissions.clipboard-write",
            &source.permissions.clipboard_write,
        ),
        ("permissions.local-fonts", &source.permissions.local_fonts),
    ] {
        if !valid_states.contains(&val.as_str()) {
            r = r.error(ValidationIssue::new(
                field,
                format!("unexpected permission state '{}'", val),
            ));
        }
    }

    r
}

fn os_display_name(os: &str) -> &str {
    match os.to_lowercase().as_str() {
        "windows" => "Windows",
        "macos" | "mac" => "Mac OS X",
        "linux" => "Linux",
        "android" => "Android",
        "ios" => "iPhone OS",
        _ => os,
    }
}

fn os_platform(os: &str) -> &str {
    match os.to_lowercase().as_str() {
        "windows" => "Win32",
        "macos" | "mac" => "MacIntel",
        "linux" => "Linux x86_64",
        "android" => "Linux armv8l",
        _ => "Unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::defaults::default_profile_source;

    #[test]
    fn valid_default_profile() {
        let p = default_profile_source();
        let r = validate(&p);
        assert!(r.is_valid(), "default profile should validate: {}", r);
    }

    #[test]
    fn rejects_ua_without_brand() {
        let mut p = default_profile_source();
        p.identity.browser.brand = "safari".into();
        p.navigator.user_agent = "Mozilla/5.0 (macOS) AppleWebKit/605.1".into();
        let r = validate(&p);
        assert!(r.error_count() > 0);
    }

    #[test]
    fn rejects_language_mismatch() {
        let mut p = default_profile_source();
        p.navigator.language = "en-US".into();
        p.locale.language = "zh-CN".into();
        let r = validate(&p);
        assert!(r.error_count() > 0);
    }

    #[test]
    fn warns_on_unknown_timezone() {
        let mut p = default_profile_source();
        p.locale.timezone = "Mars/Olympus".into();
        let r = validate(&p);
        assert!(r.warning_count() > 0);
    }

    #[test]
    fn rejects_zero_cpu_cores() {
        let mut p = default_profile_source();
        p.identity.cpu_cores = 0;
        let r = validate(&p);
        assert!(r.error_count() > 0);
    }

    #[test]
    fn rejects_zero_timing_fps() {
        let mut p = default_profile_source();
        p.timing.fps = 0;
        let r = validate(&p);
        assert!(r.error_count() > 0);
    }

    #[test]
    fn rejects_unknown_modes() {
        let mut p = default_profile_source();
        p.rendering.canvas_2d.mode = "magic".into();
        let r = validate(&p);
        assert!(r.error_count() > 0);
    }

    #[test]
    fn rejects_invalid_extended_permission_state() {
        let mut p = default_profile_source();
        p.permissions.local_fonts = "sometimes".into();
        let r = validate(&p);
        assert!(r.error_count() > 0);
    }

    #[test]
    fn warns_incoherent_screen_sizes() {
        let mut p = default_profile_source();
        p.display.screen.width = 1024;
        p.display.screen.avail_width = 1920;
        let r = validate(&p);
        assert!(r.warning_count() > 0);
    }

    #[test]
    fn warns_webrtc_real_without_proxy() {
        let mut p = default_profile_source();
        p.network.webrtc.mode = "real".into();
        let r = validate(&p);
        assert!(r.warning_count() > 0);
    }
}
