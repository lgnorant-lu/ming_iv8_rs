//! Extended Attribute handler — Web IDL Extended Attribute rules.
//!
//! Processes extended attributes from IDL definitions and generates
//! appropriate Rust code annotations, descriptor flags, and registration
//! logic.

use crate::ir::Definition;

/// Result of processing extended attributes for a definition.
pub struct EaResult {
    /// Whether to expose on Window global (default: true for interfaces)
    pub exposed_window: bool,
    /// Whether to expose on Worker global
    pub exposed_worker: bool,
    /// Whether to skip global constructor registration (NoInterfaceObject)
    pub no_interface_object: bool,
    /// Whether the interface has [LegacyUnforgeable] — properties on instance
    pub legacy_unforgeable: bool,
    /// Whether attributes are [Replaceable] — needs special setter
    pub has_replaceable: Vec<String>,
    /// Constructor alias name (from [NamedConstructor])
    pub named_constructor: Option<String>,
    /// Whether to mark as secure-context-only
    pub secure_context: bool,
    /// Cross-origin accessible
    pub cross_origin_accessible: bool,
    /// Whether interface is [Global] — available as globalThis
    pub is_global: bool,
    /// Raw EA names for logging
    pub raw_attrs: Vec<String>,
}

impl EaResult {
    pub fn new() -> Self {
        Self {
            exposed_window: true,
            exposed_worker: false,
            no_interface_object: false,
            legacy_unforgeable: false,
            has_replaceable: Vec::new(),
            named_constructor: None,
            secure_context: false,
            cross_origin_accessible: false,
            is_global: false,
            raw_attrs: Vec::new(),
        }
    }
}

/// Process extended attributes for an interface definition.
pub fn process_interface_ea(def: &Definition) -> EaResult {
    let mut result = EaResult::new();

    for attr in &def.ext_attrs {
        result.raw_attrs.push(attr.clone());
        match attr.as_str() {
            "Exposed" => { /* handled below */ }
            "NoInterfaceObject" | "LegacyNoInterfaceObject" => {
                result.no_interface_object = true;
            }
            "LegacyUnforgeable" => {
                result.legacy_unforgeable = true;
            }
            "SecureContext" => {
                result.secure_context = true;
            }
            "CrossOrigin" | "CrossOriginIsolated" | "CrossOriginReadable" => {
                result.cross_origin_accessible = true;
            }
            "Global" => {
                result.is_global = true;
            }
            a if a.starts_with("NamedConstructor") || a.starts_with("LegacyFactoryFunction") => {
                // [NamedConstructor=Image] or [LegacyFactoryFunction=Image]
                if let Some((_, val)) = a.split_once('=') {
                    result.named_constructor = Some(val.trim_matches('"').to_string());
                }
            }
            _ => {}
        }
    }

    // Check for [Exposed=Worker] or [Exposed=(Window,Worker)]
    for attr in &def.ext_attrs {
        if attr.starts_with("Exposed") {
            if attr.contains("Worker") {
                result.exposed_worker = true;
            }
            if attr.contains("Window") {
                result.exposed_window = true;
            }
        }
    }

    // Check members for per-member EAs — not supported in simplified MemberData
    // MemberData currently only stores name/kind/type info, not per-member ext_attrs.
    // Per-member Replaceable detection deferred to v0.8.20.

    result
}

/// Generate the Exposed guard code for an interface.
/// Returns a Rust expression that evaluates to true if the interface
/// should be registered in the current execution context.
pub fn exposed_guard(ea: &EaResult) -> String {
    if ea.exposed_window && ea.exposed_worker {
        "// Exposed=(Window,Worker): always register".to_string()
    } else if ea.exposed_worker && !ea.exposed_window {
        "// Exposed=Worker only: skip Window registration".to_string()
    } else {
        "// Exposed=Window: register on Window global".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::{Definition, MemberData};

    fn make_def(name: &str, ext_attrs: Vec<&str>) -> Definition {
        Definition {
            kind: "interface".into(),
            name: Some(name.into()),
            source: Some("w3c".into()),
            inheritance: None,
            ext_attrs: ext_attrs.iter().map(|s| s.to_string()).collect(),
            members: vec![],
            partial: false,
            values: vec![],
            target: None,
            includes: None,
        }
    }

    #[test]
    fn test_no_interface_object() {
        let def = make_def("Test", vec!["NoInterfaceObject"]);
        let ea = process_interface_ea(&def);
        assert!(ea.no_interface_object);
    }

    #[test]
    fn test_secure_context() {
        let def = make_def("Test", vec!["SecureContext", "Exposed"]);
        let ea = process_interface_ea(&def);
        assert!(ea.secure_context);
    }

    #[test]
    fn test_exposed_window() {
        let def = make_def("Test", vec!["Exposed"]);
        let ea = process_interface_ea(&def);
        assert!(ea.exposed_window);
    }

    #[test]
    fn test_named_constructor() {
        let def = make_def("HTMLImageElement", vec!["LegacyFactoryFunction=Image"]);
        let ea = process_interface_ea(&def);
        assert_eq!(ea.named_constructor, Some("Image".to_string()));
    }

    #[test]
    fn test_default_exposed() {
        let def = make_def("Test", vec![]);
        let ea = process_interface_ea(&def);
        assert!(ea.exposed_window);
        assert!(!ea.no_interface_object);
    }
}
