//! Webpack runtime detection module.
//!
//! Detects and classifies webpack-like bundler runtimes from JS source.

/// Result of webpack runtime detection.
#[derive(Debug, Clone)]
pub struct WebpackDetection {
    /// Whether a webpack-like runtime was detected.
    pub detected: bool,
    /// The detected runtime flavor.
    pub flavor: WebpackFlavor,
    /// Which helpers are present (.m, .c, .d, .e, .l, .o, .p, .r, .u, .f).
    pub helpers_present: Vec<String>,
    /// Module IDs extracted from the modules table (up to a limit).
    pub module_ids: Vec<String>,
    /// Number of modules found in the modules table.
    pub module_count: usize,
}

/// Webpack runtime flavor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebpackFlavor {
    Webpack4,
    Webpack5,
    UnknownWebpackLike,
}

/// Detect webpack runtime from JS source.
pub fn detect(source: &str) -> WebpackDetection {
    let mut helpers: Vec<String> = Vec::new();
    let flavor;
    // Check for main __webpack_require__ marker
    if !source.contains("__webpack_require__") && !source.contains("window.webpackJsonp") {
        return WebpackDetection {
            detected: false,
            flavor: WebpackFlavor::UnknownWebpackLike,
            helpers_present: helpers,
            module_ids: Vec::new(),
            module_count: 0,
        };
    }

    // Detect standard helpers
    let helper_patterns = [
        ".m", ".c", ".d", ".e", ".l", ".o", ".p", ".r", ".u", ".f",
    ];
    for h in &helper_patterns {
        let pattern = format!("__webpack_require__{}", h);
        if source.contains(&pattern) {
            helpers.push(h.to_string());
        }
    }

    // Flavor detection via signature patterns
    if source.contains("self.webpackChunk") || source.contains("webpackChunk") {
        // webpack 5+ uses globalThis.webpackChunk
        flavor = WebpackFlavor::Webpack5;
    } else if source.contains("webpackJsonpCallback") || source.contains("jsonpArray") {
        flavor = WebpackFlavor::Webpack4;
    } else if helpers.contains(&".e".to_string()) {
        // __webpack_require__.e is more common in webpack 5
        flavor = WebpackFlavor::Webpack5;
    } else if helpers.contains(&".r".to_string()) {
        // __webpack_require__.r (define ES module) is webpack 5+ feature
        flavor = WebpackFlavor::Webpack5;
    } else {
        flavor = WebpackFlavor::UnknownWebpackLike;
    }

    // Extract module IDs from modules table definition
    let mut module_ids = Vec::new();
    let module_count = extract_module_count(source);

    // Try to extract specific module IDs
    if let Some(mods) = extract_module_ids(source) {
        module_ids = mods;
    }

    WebpackDetection {
        detected: true,
        flavor,
        helpers_present: helpers,
        module_ids,
        module_count,
    }
}

/// Find the body of the modules table (content between outermost braces).
fn find_modules_body(source: &str) -> Option<&str> {
    let marker = "__webpack_require__.m";
    let pos = source.find(marker)?;
    let after = source[pos + marker.len()..].trim_start();
    let after = after.trim_start_matches('=').trim_start();
    let brace_start = after.find('{')?;
    let body = &after[brace_start + 1..];
    Some(body)
}

/// Estimate module count from line-based heuristics.
fn extract_module_count(source: &str) -> usize {
    let body = find_modules_body(source).unwrap_or("");
    if let Some(end) = body.find('}') {
        let table = &body[..end];
        return table.matches(':').count().min(10000);
    }
    0
}

/// Extract module IDs up to a reasonable limit.
fn extract_module_ids(source: &str) -> Option<Vec<String>> {
    let mut ids = Vec::new();
    let limit = 200;

    let body = find_modules_body(source)?;
    let bytes = body.as_bytes();
    let mut idx = 0;

    while idx < body.len().min(5000) && ids.len() < limit {
        // Skip whitespace/newlines
        while idx < body.len() && (bytes[idx] == b' ' || bytes[idx] == b'\n'
            || bytes[idx] == b'\r' || bytes[idx] == b'\t')
        {
            idx += 1;
        }
        if idx >= body.len() || bytes[idx] == b'}' {
            break;
        }
        if bytes[idx] == b',' {
            idx += 1;
            continue;
        }
        // Read until ':' for the key
        let start = idx;
        while idx < body.len() && bytes[idx] != b':' {
            idx += 1;
        }
        if idx < body.len() && bytes[idx] == b':' {
            let key = body[start..idx].trim();
            if !key.is_empty() {
                let clean = key.trim_matches('"').trim_matches('\'');
                ids.push(clean.to_string());
            }
        }
        // Skip past the value to the next comma or closing brace
        // Track balanced braces so we correctly handle function values
        let mut depth: i32 = 0;
        let mut found_value_end = false;
        while idx < body.len() && !found_value_end {
            match bytes[idx] {
                b'{' | b'[' | b'(' => depth += 1,
                b'}' | b']' | b')' => {
                    if depth == 0 {
                        // We hit a closing brace at depth 0 — the entry value ended
                        found_value_end = true;
                    } else {
                        depth -= 1;
                    }
                }
                b',' if depth == 0 => {
                    found_value_end = true;
                }
                _ => {}
            }
            if !found_value_end {
                idx += 1;
            }
        }
    }

    if ids.is_empty() { None } else { Some(ids) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_webpack() {
        let d = detect("var x = 1;");
        assert!(!d.detected);
    }

    #[test]
    fn test_basic_webpack4() {
        let src = r#"
(function(modules) {
    function __webpack_require__(moduleId) {
        // ...
    }
    __webpack_require__.m = modules;
    __webpack_require__.c = {};
    __webpack_require__.d = function() {};
    return __webpack_require__(0);
})
"#;
        let d = detect(src);
        assert!(d.detected);
        assert!(d.helpers_present.contains(&".m".to_string()));
        assert!(d.helpers_present.contains(&".c".to_string()));
        assert!(d.helpers_present.contains(&".d".to_string()));
    }

    #[test]
    fn test_webpack5_helpers() {
        let src = r#"
__webpack_require__.m = {};
__webpack_require__.c = {};
__webpack_require__.d = function() {};
__webpack_require__.e = function() {};
__webpack_require__.r = function() {};
"#;
        let d = detect(src);
        assert!(d.detected);
        assert!(d.helpers_present.contains(&".e".to_string()));
        assert!(d.helpers_present.contains(&".r".to_string()));
    }

    #[test]
    fn test_module_id_extraction() {
        let src = r#"
__webpack_require__.m = {
    0: function(module) {},
    7: function(module) {},
    42: function(module) {},
    100: function(module) {},
};
"#;
        let d = detect(src);
        assert!(d.detected);
        assert!(d.module_ids.contains(&"0".to_string()));
        assert!(d.module_ids.contains(&"7".to_string()));
        assert!(d.module_ids.contains(&"42".to_string()));
        assert!(d.module_ids.contains(&"100".to_string()));
    }

    #[test]
    fn test_webpackflavor_5() {
        let src = r#"
var __webpack_require__ = {};
__webpack_require__.e = function() {};
globalThis.webpackChunk = [];
"#;
        let d = detect(src);
        assert_eq!(d.flavor, WebpackFlavor::Webpack5);
    }
}
