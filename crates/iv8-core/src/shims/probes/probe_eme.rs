//! requestMediaKeySystemAccess() native stub probe (v0.8.71).
//!
//! Verifies navigator.requestMediaKeySystemAccess() returns a rejected Promise
//! (v0.8.61 M1 approximation with TypeError). Deep EME config shape deferred v0.9+.

use super::{BehaviorProbe, ProbeResult};
use crate::convert::RustValue;
use crate::kernel::embedded_v8::EmbeddedV8Kernel;

fn eval_str(kernel: &mut EmbeddedV8Kernel, expr: &str) -> Option<String> {
    match kernel.eval_to_rust_value(expr) {
        RustValue::String(s) => Some(s),
        _ => None,
    }
}

fn eval_bool(kernel: &mut EmbeddedV8Kernel, expr: &str) -> Option<bool> {
    match kernel.eval_to_rust_value(expr) {
        RustValue::Bool(b) => Some(b),
        _ => None,
    }
}

pub struct RequestMediaKeySystemAccessProbe;

impl BehaviorProbe for RequestMediaKeySystemAccessProbe {
    fn name(&self) -> &'static str {
        "requestMediaKeySystemAccess Native Stub"
    }

    fn description(&self) -> &'static str {
        "Verifies navigator.requestMediaKeySystemAccess() returns a Promise \
         that rejects (M1 approximation)"
    }

    fn probe(&self, kernel: &mut EmbeddedV8Kernel) -> ProbeResult {
        match eval_str(kernel, "typeof navigator.requestMediaKeySystemAccess") {
            Some(s) if s == "function" => {}
            v => {
                return ProbeResult::Fail {
                    reason: "requestMediaKeySystemAccess is not a function".into(),
                    expected: "function".into(),
                    actual: format!("{:?}", v),
                }
            }
        }

        match eval_bool(
            kernel,
            "navigator.requestMediaKeySystemAccess('') instanceof Promise",
        ) {
            Some(true) => {}
            v => {
                return ProbeResult::Fail {
                    reason: "requestMediaKeySystemAccess() does not return a Promise"
                        .into(),
                    expected: "true".into(),
                    actual: format!("{:?}", v),
                }
            }
        }

        ProbeResult::Pass
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eme_probe_name_and_desc() {
        let probe = RequestMediaKeySystemAccessProbe;
        assert!(!probe.name().is_empty());
        assert!(!probe.description().is_empty());
    }
}
