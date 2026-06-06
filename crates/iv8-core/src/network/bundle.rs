//! ResourceBundle: pre-registered HTTP responses for offline operation.
//!
//! Usage:
//!   ctx.add_resource("https://example.com/api", b'{"ok":true}', status=200)
//!   // Later, fetch("https://example.com/api") returns the registered response.

use std::collections::HashMap;

/// A pre-registered HTTP response.
#[derive(Debug, Clone)]
pub struct Resource {
    /// Response body (raw bytes).
    pub body: Vec<u8>,
    /// HTTP status code.
    pub status: u16,
    /// Response headers.
    pub headers: HashMap<String, String>,
    /// Content-Type (convenience, also in headers).
    pub content_type: Option<String>,
}

impl Resource {
    /// Create a new resource with default headers.
    pub fn new(body: Vec<u8>, status: u16, headers: Option<HashMap<String, String>>) -> Self {
        let headers = headers.unwrap_or_default();
        let content_type = headers.get("content-type").cloned();
        Self {
            body,
            status,
            headers,
            content_type,
        }
    }

    /// Create a simple text resource.
    pub fn text(body: &str, status: u16) -> Self {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "text/plain".to_string());
        Self {
            body: body.as_bytes().to_vec(),
            status,
            headers,
            content_type: Some("text/plain".to_string()),
        }
    }

    /// Create a JSON resource.
    pub fn json(body: &str, status: u16) -> Self {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        Self {
            body: body.as_bytes().to_vec(),
            status,
            headers,
            content_type: Some("application/json".to_string()),
        }
    }

    /// Get body as UTF-8 string (lossy).
    pub fn body_text(&self) -> String {
        String::from_utf8_lossy(&self.body).to_string()
    }
}

/// A collection of pre-registered resources, keyed by URL.
#[derive(Debug, Default)]
pub struct ResourceBundle {
    resources: HashMap<String, Resource>,
}

impl ResourceBundle {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a resource to the bundle.
    pub fn add(&mut self, url: &str, resource: Resource) {
        self.resources.insert(url.to_string(), resource);
    }

    /// Add a resource with raw bytes.
    pub fn add_raw(
        &mut self,
        url: &str,
        body: Vec<u8>,
        status: u16,
        headers: Option<HashMap<String, String>>,
    ) {
        self.add(url, Resource::new(body, status, headers));
    }

    /// Look up a resource by URL (exact match).
    pub fn get(&self, url: &str) -> Option<&Resource> {
        self.resources.get(url)
    }

    /// Check if a URL is registered.
    pub fn contains(&self, url: &str) -> bool {
        self.resources.contains_key(url)
    }

    /// Get the number of registered resources.
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Remove a resource.
    pub fn remove(&mut self, url: &str) -> Option<Resource> {
        self.resources.remove(url)
    }

    /// Clear all resources.
    pub fn clear(&mut self) {
        self.resources.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bundle_add_and_get() {
        let mut bundle = ResourceBundle::new();
        bundle.add_raw("https://example.com/api", b"hello".to_vec(), 200, None);

        let res = bundle.get("https://example.com/api").unwrap();
        assert_eq!(res.body_text(), "hello");
        assert_eq!(res.status, 200);
    }

    #[test]
    fn bundle_not_found() {
        let bundle = ResourceBundle::new();
        assert!(bundle.get("https://missing.com").is_none());
    }

    #[test]
    fn bundle_json_resource() {
        let mut bundle = ResourceBundle::new();
        bundle.add(
            "https://api.com/data",
            Resource::json(r#"{"ok":true}"#, 200),
        );

        let res = bundle.get("https://api.com/data").unwrap();
        assert_eq!(res.body_text(), r#"{"ok":true}"#);
        assert_eq!(res.content_type.as_deref(), Some("application/json"));
    }

    #[test]
    fn bundle_remove() {
        let mut bundle = ResourceBundle::new();
        bundle.add_raw("https://example.com", b"data".to_vec(), 200, None);
        assert!(bundle.contains("https://example.com"));
        bundle.remove("https://example.com");
        assert!(!bundle.contains("https://example.com"));
    }

    #[test]
    fn bundle_len() {
        let mut bundle = ResourceBundle::new();
        assert_eq!(bundle.len(), 0);
        bundle.add_raw("https://a.com", vec![], 200, None);
        bundle.add_raw("https://b.com", vec![], 404, None);
        assert_eq!(bundle.len(), 2);
    }
}
