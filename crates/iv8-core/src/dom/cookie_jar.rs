//! Cookie security model basics (Track B of v0.8.72).
//!
//! `CookieRecord` models individual cookie attributes. The actual runtime
//! parsing and filtering executes in the JS shim (`document_props.rs`) to
//! maintain compatibility with the document lifecycle (document objects are
//! dynamically created/destroyed by page.load()). The Rust struct here serves
//! as the canonical model for documentation, future native migration, and
//! test assertions.
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
