//! Type mapping engine — IDL types to Rust/V8 types.
//!
//! Maps Web IDL type names to Rust type representations, default value
//! expressions, and getter return code templates.

/// The result of mapping an IDL type to Rust/V8.
pub struct TypeMap {
    /// Rust type name (e.g., "bool", "f64", "v8::Local<'s, v8::String>")
    pub rust_type: String,
    /// Expression to produce a default value (e.g., "false", "0.0")
    pub default_value: String,
    /// Whether this type needs a scope reference (most V8 types do)
    pub needs_scope: bool,
}

/// Map an IDL type name to its Rust/V8 representation.
pub fn map_idl_type(idl_type: &str) -> TypeMap {
    match idl_type {
        // Primitives
        "boolean" => TypeMap {
            rust_type: "bool".into(),
            default_value: "v8::Boolean::new(scope, false).into()".into(),
            needs_scope: true,
        },
        "byte" | "octet" | "short" | "unsigned short" |
        "long" | "unsigned long" | "long long" | "unsigned long long" => TypeMap {
            rust_type: "i64".into(),
            default_value: "v8::Integer::new(scope, 0).into()".into(),
            needs_scope: true,
        },
        "float" | "double" | "unrestricted float" | "unrestricted double" => TypeMap {
            rust_type: "f64".into(),
            default_value: "v8::Number::new(scope, 0.0).into()".into(),
            needs_scope: true,
        },

        // String types
        "DOMString" | "USVString" | "ByteString" => TypeMap {
            rust_type: "v8::Local<'s, v8::String>".into(),
            default_value: "crate::type_conv::v8_str(scope, \"\")".into(),
            needs_scope: true,
        },

        // Special
        "void" | "undefined" => TypeMap {
            rust_type: "v8::Local<'s, v8::Value>".into(),
            default_value: "v8::undefined(scope).into()".into(),
            needs_scope: true,
        },
        "any" => TypeMap {
            rust_type: "v8::Local<'s, v8::Value>".into(),
            default_value: "v8::undefined(scope).into()".into(),
            needs_scope: true,
        },
        "object" => TypeMap {
            rust_type: "v8::Local<'s, v8::Object>".into(),
            default_value: "v8::Object::new(scope).into()".into(),
            needs_scope: true,
        },
        "Function" => TypeMap {
            rust_type: "v8::Local<'s, v8::Function>".into(),
            default_value: "v8::undefined(scope).into()".into(),
            needs_scope: true,
        },

        // BufferSource types
        "ArrayBuffer" => TypeMap {
            rust_type: "v8::Local<'s, v8::ArrayBuffer>".into(),
            default_value: "v8::ArrayBuffer::new(scope, 0).into()".into(),
            needs_scope: true,
        },
        name if is_buffer_source(name) => TypeMap {
            rust_type: "v8::Local<'s, v8::Value>".into(),
            default_value: format!("crate::type_conv::default_value_for_type(scope, \"{}\")", name),
            needs_scope: true,
        },

        // Interface references — return empty object skeleton
        _ => TypeMap {
            rust_type: "v8::Local<'s, v8::Object>".into(),
            default_value: "v8::Object::new(scope).into()".into(),
            needs_scope: true,
        },
    }
}

fn is_buffer_source(name: &str) -> bool {
    matches!(name,
        "DataView" | "Int8Array" | "Uint8Array" | "Uint8ClampedArray" |
        "Int16Array" | "Uint16Array" | "Int32Array" | "Uint32Array" |
        "Float32Array" | "Float64Array" | "BigInt64Array" | "BigUint64Array"
    )
}

/// Convert an IDL type name to a Rust-safe identifier.
/// Applies standard Rust camelCase/snake_case conventions.
pub fn idl_name_to_rust(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let chars: Vec<char> = name.chars().collect();
    for i in 0..chars.len() {
        let ch = chars[i];
        if ch.is_uppercase() {
            // Insert _ before uppercase letter if:
            // - it's not the first char, AND
            // - previous char was lowercase OR
            // - previous char was uppercase AND next char is lowercase
            if i > 0 {
                let prev = chars[i - 1];
                let next_lower = i + 1 < chars.len() && chars[i + 1].is_lowercase();
                if prev.is_lowercase() || (prev.is_uppercase() && next_lower) {
                    result.push('_');
                }
            }
            result.push(ch.to_ascii_lowercase());
        } else if ch == '-' {
            result.push('_');
        } else {
            result.push(ch);
        }
    }
    result
}

/// Rust keywords that need raw identifier escaping.
pub fn escape_rust_keyword(name: &str) -> String {
    match name {
        "type" | "match" | "impl" | "mod" | "crate" | "self" | "super" |
        "where" | "for" | "loop" | "while" | "if" | "else" | "struct" |
        "enum" | "fn" | "const" | "static" | "let" | "mut" | "ref" |
        "return" | "async" | "await" | "move" | "use" | "pub" | "box" |
        "dyn" | "unsafe" | "extern" | "true" | "false" | "abstract" |
        "become" | "do" | "final" | "macro" | "override" | "priv" |
        "typeof" | "unsized" | "virtual" | "yield" | "in" | "as" |
        "try" | "union" | "trait" => format!("r#{}", name),
        _ => name.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean_mapping() {
        let m = map_idl_type("boolean");
        assert_eq!(m.rust_type, "bool");
        assert_eq!(m.default_value, "v8::Boolean::new(scope, false).into()");
    }

    #[test]
    fn test_string_mapping() {
        let m = map_idl_type("DOMString");
        assert_eq!(m.rust_type, "v8::Local<'s, v8::String>");
        assert_eq!(m.default_value, "crate::type_conv::v8_str(scope, \"\")");
    }

    #[test]
    fn test_long_mapping() {
        let m = map_idl_type("long");
        assert_eq!(m.default_value, "v8::Integer::new(scope, 0).into()");
    }

    #[test]
    fn test_double_mapping() {
        let m = map_idl_type("double");
        assert_eq!(m.default_value, "v8::Number::new(scope, 0.0).into()");
    }

    #[test]
    fn test_void_mapping() {
        let m = map_idl_type("void");
        assert_eq!(m.default_value, "v8::undefined(scope).into()");
    }

    #[test]
    fn test_unknown_mapping() {
        let m = map_idl_type("MyInterface");
        assert_eq!(m.default_value, "v8::null(scope).into()");
    }

    #[test]
    fn test_rust_keyword_escape() {
        assert_eq!(escape_rust_keyword("type"), "r#type");
        assert_eq!(escape_rust_keyword("normal"), "normal");
        assert_eq!(escape_rust_keyword("match"), "r#match");
    }

    #[test]
    fn test_camel_to_snake() {
        assert_eq!(idl_name_to_rust("HTMLDivElement"), "html_div_element");
        assert_eq!(idl_name_to_rust("EventTarget"), "event_target");
        assert_eq!(idl_name_to_rust("DOMString"), "dom_string");
    }
}
