//! navigator.credentials object-shape probe (v0.8.63 M2).
//!
//! Verifies that navigator.credentials is a non-null object with get,
//! create, and preventSilentAccess methods. Promise/options shape
//! deferred to v0.9+.

use super::{BehaviorProbe, ProbeResult};
use crate::convert::RustValue;
use crate::kernel::embedded_v8::EmbeddedV8Kernel;

fn eval_bool(kernel: &mut EmbeddedV8Kernel, expr: &str) -> Option<bool> {
    match kernel.eval_to_rust_value(expr) {
        RustValue::Bool(b) => Some(b),
        _ => None,
    }
}

fn eval_str(kernel: &mut EmbeddedV8Kernel, expr: &str) -> Option<String> {
    match kernel.eval_to_rust_value(expr) {
        RustValue::String(s) => Some(s),
        _ => None,
    }
}

pub struct CredentialsProbe;

impl BehaviorProbe for CredentialsProbe {
    fn name(&self) -> &'static str {
        "navigator.credentials Object Shape"
    }

    fn description(&self) -> &'static str {
        "Verifies navigator.credentials is an object with get, create, \
         and preventSilentAccess method properties and native code descriptor"
    }

    fn probe(&self, kernel: &mut EmbeddedV8Kernel) -> ProbeResult {
        match eval_str(kernel, "typeof navigator.credentials") {
            Some(s) if s == "object" => {}
            v => {
                return ProbeResult::Fail {
                    reason: "navigator.credentials is not an object".into(),
                    expected: "object".into(),
                    actual: format!("{:?}", v),
                };
            }
        }

        for method in &["get", "create", "preventSilentAccess"] {
            let expr = format!("typeof navigator.credentials.{}", method);
            match eval_str(kernel, &expr) {
                Some(s) if s == "function" => {}
                v => {
                    return ProbeResult::Fail {
                        reason: format!("navigator.credentials.{} is not a function", method),
                        expected: "function".into(),
                        actual: format!("{:?}", v),
                    };
                }
            }
        }

        match eval_bool(kernel, "'prototype' in Object.getOwnPropertyDescriptor(Navigator.prototype, 'credentials').get") {
            Some(false) => {}
            v => {
                return ProbeResult::Fail {
                    reason: "navigator.credentials getter has prototype (not native code shape)".into(),
                    expected: "false".into(),
                    actual: format!("{:?}", v),
                };
            }
        }

        ProbeResult::Pass
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credentials_probe_name_and_desc() {
        let probe = CredentialsProbe;
        assert!(!probe.name().is_empty());
        assert!(!probe.description().is_empty());
    }
}
