//! Cookie security model basics (Track B of v0.8.72).
//!
//! `CookieRecord` models individual cookie attributes. `CookieJar`
//! provides the canonical Rust model for cookie set/get with attribute
//! parsing and visibility filtering, mirroring the JS shim behavior
//! in `document_props.rs`. The actual runtime uses the JS shim for
//! backward compatibility with document lifecycle; this Rust model
//! serves as the reference implementation for testing and future
//! native migration.
//!
//! Non-goals (v0.9+):
//! - Full RFC 6265 date parser
//! - Domain matching / domain scope expansion
//! - httpOnly / SameSite enforcement (requires HTTP layer)
//! - CookieStore API

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CookieRecord {
    pub name: String,
    pub value: String,
    pub path: Option<String>,
    pub domain: Option<String>,
    pub expires: Option<String>,
    pub max_age: Option<i64>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct CookieJar {
    cookies: Vec<CookieRecord>,
}

impl CookieJar {
    pub fn new() -> Self {
        Self {
            cookies: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.cookies.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cookies.is_empty()
    }

    pub fn set_cookie(&mut self, raw: &str) {
        let parts: Vec<&str> = raw.split(';').collect();
        if parts.is_empty() {
            return;
        }
        let first = parts[0].trim();
        let kv: Vec<&str> = first.splitn(2, '=').collect();
        if kv.len() < 2 {
            return;
        }
        let name = kv[0].trim().to_string();
        let value = kv[1].trim().to_string();

        let mut record = CookieRecord {
            name: name.clone(),
            value: value.clone(),
            path: None,
            domain: None,
            expires: None,
            max_age: None,
            secure: false,
            http_only: false,
            same_site: None,
        };

        for attr in &parts[1..] {
            let attr = attr.trim();
            let lower = attr.to_lowercase();
            if lower == "secure" {
                record.secure = true;
            } else if lower == "httponly" {
                record.http_only = true;
            } else if lower.starts_with("path=") {
                record.path = Some(attr[5..].to_string());
            } else if lower.starts_with("domain=") {
                record.domain = Some(attr[7..].to_string());
            } else if lower.starts_with("samesite=") {
                record.same_site = Some(attr[9..].to_string());
            } else if lower.starts_with("expires=") {
                record.expires = Some(attr[8..].to_string());
            } else if lower.starts_with("max-age=") {
                if let Ok(ma) = attr[8..].parse::<i64>() {
                    if ma <= 0 {
                        self.cookies.retain(|c| c.name != name);
                        return;
    #[test]
    fn test_jar_path_prefix_boundary() {
        let mut jar = CookieJar::new();
        jar.set_cookie("x=1; Path=/app");
        // /app matches
        assert_eq!(jar.get_cookie_string("/app", true), "x=1");
        // /app/page matches (next char is /)
        assert_eq!(jar.get_cookie_string("/app/page", true), "x=1");
        // /application does NOT match (next char is 'l', not /)
        assert_eq!(
            jar.get_cookie_string("/application", true),
            "",
            "Path=/app must not match /application"
        );
        // /appx does NOT match
        assert_eq!(jar.get_cookie_string("/appx", true), "");
    }
}
                    record.max_age = Some(ma);
                }
            }
        }

        // Replace existing cookie with same name (case-sensitive)
        self.cookies.retain(|c| c.name != name);
        self.cookies.push(record);
    }

    fn is_secure_context(is_secure: bool) -> bool {
        is_secure
    }

    pub fn get_cookie_string(
        &self,
        document_path: &str,
        is_secure_context: bool,
    ) -> String {
        let parts: Vec<String> = self
            .cookies
            .iter()
            .filter(|c| cookie_visible(c, document_path, is_secure_context))
            .map(|c| format!("{}={}", c.name, c.value))
            .collect();
        parts.join("; ")
    }

    pub fn get_cookies_for_path(
        &self,
        document_path: &str,
    ) -> Vec<&CookieRecord> {
        self.cookies
            .iter()
            .filter(|c| cookie_visible(c, document_path, true))
            .collect()
    }
}

fn cookie_visible(
    cookie: &CookieRecord,
    document_path: &str,
    is_secure_context: bool,
) -> bool {
    if let Some(ref path) = cookie.path {
        if path != "/" && !path_matches(document_path, path) {
            return false;
        }
    }
    if cookie.secure && !is_secure_context {
        return false;
    }
    true
}

/// RFC 6265 path-match: cookie-path is a prefix of request-path,
/// and either paths are equal or the next character after the
/// cookie-path in the request-path is '/'.
fn path_matches(request_path: &str, cookie_path: &str) -> bool {
    if request_path == cookie_path {
        return true;
    }
    if !request_path.starts_with(cookie_path) {
        return false;
    }
    request_path[cookie_path.len()..].starts_with('/')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jar_set_and_get_simple() {
        let mut jar = CookieJar::new();
        jar.set_cookie("a=1");
        assert_eq!(jar.len(), 1);
        assert_eq!(jar.get_cookie_string("/", true), "a=1");
    }

    #[test]
    fn test_jar_multiple_cookies() {
        let mut jar = CookieJar::new();
        jar.set_cookie("x=hello");
        jar.set_cookie("y=world");
        assert_eq!(jar.len(), 2);
        let s = jar.get_cookie_string("/", true);
        assert!(s.contains("x=hello"));
        assert!(s.contains("y=world"));
    }

    #[test]
    fn test_jar_max_age_zero_removes() {
        let mut jar = CookieJar::new();
        jar.set_cookie("temp=1");
        assert_eq!(jar.len(), 1);
        jar.set_cookie("temp=; Max-Age=0");
        assert_eq!(jar.len(), 0);
    }

    #[test]
    fn test_jar_secure_attribute() {
        let mut jar = CookieJar::new();
        jar.set_cookie("sec=1; Secure");
        assert_eq!(jar.get_cookie_string("/", true), "sec=1");
        assert_eq!(jar.get_cookie_string("/", false), "");
    }

    #[test]
    fn test_jar_path_filtering() {
        let mut jar = CookieJar::new();
        jar.set_cookie("global=1; Path=/");
        jar.set_cookie("private=secret; Path=/app");
        // At root path, global is visible, private is not
        let s = jar.get_cookie_string("/", true);
        assert!(s.contains("global=1"));
        assert!(!s.contains("private"));
        // At /app path, both are visible
        let s2 = jar.get_cookie_string("/app/page", true);
        assert!(s2.contains("global=1"));
        assert!(s2.contains("private=secret"));
    }

    #[test]
    fn test_jar_set_replaces_by_name() {
        let mut jar = CookieJar::new();
        jar.set_cookie("x=1");
        jar.set_cookie("x=2");
        assert_eq!(jar.len(), 1);
        assert_eq!(jar.get_cookie_string("/", true), "x=2");
    }

    #[test]
    fn test_jar_parse_all_attributes() {
        let mut jar = CookieJar::new();
        jar.set_cookie("full=value; Path=/app; Secure; SameSite=Lax; Max-Age=3600; Domain=example.com");
        assert_eq!(jar.len(), 1);
        let cookies = jar.get_cookies_for_path("/app/page");
        assert_eq!(cookies.len(), 1);
        let c = &cookies[0];
        assert_eq!(c.name, "full");
        assert_eq!(c.value, "value");
        assert_eq!(c.path.as_deref(), Some("/app"));
        assert!(c.secure);
        assert_eq!(c.same_site.as_deref(), Some("Lax"));
        assert_eq!(c.max_age, Some(3600));
        assert_eq!(c.domain.as_deref(), Some("example.com"));
    }

    #[test]
    fn test_jar_http_only_stored_not_enforced() {
        let mut jar = CookieJar::new();
        jar.set_cookie("http_only=1; HttpOnly");
        // httpOnly is stored but NOT enforced (document.cookie returns it)
        assert!(jar.get_cookie_string("/", true).contains("http_only"));
        let cookies = jar.get_cookies_for_path("/");
        assert!(cookies[0].http_only);
    }

    #[test]
    fn test_jar_empty_input() {
        let mut jar = CookieJar::new();
        jar.set_cookie("");
        assert_eq!(jar.len(), 0);
        jar.set_cookie("no_equals");
        assert_eq!(jar.len(), 0);
    }

    #[test]
    fn test_jar_value_with_equals() {
        let mut jar = CookieJar::new();
        jar.set_cookie("token=abc=def=ghi");
        assert_eq!(jar.len(), 1);
        let s = jar.get_cookie_string("/", true);
        assert!(s.contains("token=abc=def=ghi"));
    }

    #[test]
    fn test_jar_path_prefix_boundary() {
        let mut jar = CookieJar::new();
        jar.set_cookie("x=1; Path=/app");
        assert_eq!(jar.get_cookie_string("/app", true), "x=1");
        assert_eq!(jar.get_cookie_string("/app/page", true), "x=1");
        // Must NOT match /application (next char after /app is 'l', not /)
        assert_eq!(jar.get_cookie_string("/application", true), "");
        assert_eq!(jar.get_cookie_string("/appx", true), "");
    }
}
