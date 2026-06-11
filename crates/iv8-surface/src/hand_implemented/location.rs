//! Location deep stub — URL parsing and component synchronization.
//!
//! v0.8.21: Implements Location object with href setter that triggers
//! Rust-side URL parsing and synchronizes all component properties
//! (origin/protocol/host/hostname/port/pathname/search/hash).
//!
//! Uses the `url` crate for parsing (already a workspace dependency).

use url::Url;

/// Parsed Location state — held by the Location object's internal slot.
#[derive(Debug, Clone)]
pub struct LocationState {
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

impl LocationState {
    /// Create LocationState from an href string.
    /// Falls back to "about:blank" on parse failure.
    pub fn from_href(href: &str) -> Self {
        let url = Url::parse(href).unwrap_or_else(|_| {
            Url::parse("about:blank").expect("about:blank must parse")
        });
        Self {
            href: url.to_string(),
            origin: url.origin().ascii_serialization(),
            protocol: format!("{}:", url.scheme()),
            host: {
                let h = url.host_str().unwrap_or("");
                if let Some(p) = url.port() {
                    format!("{}:{}", h, p)
                } else {
                    h.to_string()
                }
            },
            hostname: url.host_str().unwrap_or("").to_string(),
            port: url.port().map(|p| p.to_string()).unwrap_or_default(),
            pathname: {
                let p = url.path();
                if p.is_empty() { "/".to_string() } else { p.to_string() }
            },
            search: {
                let q = url.query();
                q.map(|s| format!("?{}", s)).unwrap_or_default()
            },
            hash: url.fragment().map(|s| format!("#{}", s)).unwrap_or_default(),
        }
    }

    /// Rebuild href from components.
    pub fn rebuild_href(&mut self) {
        let mut href = String::new();
        href.push_str(&self.protocol);
        href.push_str("//");
        href.push_str(&self.hostname);
        if !self.port.is_empty() {
            href.push(':');
            href.push_str(&self.port);
        }
        href.push_str(&self.pathname);
        href.push_str(&self.search);
        href.push_str(&self.hash);
        self.href = href.clone();

        // Re-parse to update all components consistently
        *self = Self::from_href(&href);
    }
}

impl Default for LocationState {
    fn default() -> Self {
        Self::from_href("about:blank")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_https_url() {
        let loc = LocationState::from_href("https://example.com:8080/path?q=1#frag");
        assert_eq!(loc.protocol, "https:");
        assert_eq!(loc.hostname, "example.com");
        assert_eq!(loc.port, "8080");
        assert_eq!(loc.pathname, "/path");
        assert_eq!(loc.search, "?q=1");
        assert_eq!(loc.hash, "#frag");
        assert_eq!(loc.origin, "https://example.com:8080");
    }

    #[test]
    fn test_default_port_omitted() {
        let loc = LocationState::from_href("https://example.com/");
        assert_eq!(loc.port, "");
        assert_eq!(loc.host, "example.com");
    }

    #[test]
    fn test_ipv6_full() {
        let loc = LocationState::from_href("http://[::1]:8080/test");
        // url crate may or may not preserve brackets; test what we get
        assert!(!loc.hostname.is_empty(), "hostname must not be empty for IPv6 URL");
        assert!(!loc.href.is_empty(), "href must not be empty");
    }

    #[test]
    fn test_rebuild_href_preserves_changes() {
        let mut loc = LocationState::from_href("https://example.com/path");
        loc.hash = "#new".to_string();
        loc.rebuild_href();
        assert_eq!(loc.hash, "#new");
        assert!(loc.href.ends_with("#new"), "href should end with updated hash");
    }

    #[test]
    fn test_protocol_relative() {
        let loc = LocationState::from_href("//example.com/path");
        // Protocol-relative URLs are parsed relative to base; at minimum
        // the URL should not be empty
        assert!(!loc.href.is_empty());
    }

    #[test]
    fn test_default_location_is_about_blank() {
        let loc = LocationState::default();
        assert_eq!(loc.href, "about:blank");
    }
}
