//! getGamepads() native stub probe (v0.8.71).
//!
//! Verifies navigator.getGamepads() returns an empty array
//! (v0.8.61 native stub). Deep GamepadList shape check deferred v0.9+.

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

pub struct GetGamepadsProbe;

impl BehaviorProbe for GetGamepadsProbe {
    fn name(&self) -> &'static str {
        "getGamepads Native Stub"
    }

    fn description(&self) -> &'static str {
        "Verifies navigator.getGamepads() returns an empty array \
         without throwing"
    }

    fn probe(&self, kernel: &mut EmbeddedV8Kernel) -> ProbeResult {
        match eval_str(kernel, "typeof navigator.getGamepads") {
            Some(s) if s == "function" => {}
            v => {
                return ProbeResult::Fail {
                    reason: "getGamepads is not a function".into(),
                    expected: "function".into(),
                    actual: format!("{:?}", v),
                }
            }
        }

        match eval_bool(kernel, "Array.isArray(navigator.getGamepads())") {
            Some(true) => {}
            v => {
                return ProbeResult::Fail {
                    reason: "getGamepads() does not return an Array".into(),
                    expected: "true".into(),
                    actual: format!("{:?}", v),
                }
            }
        }

        match eval_str(kernel, "String(navigator.getGamepads().length)") {
            Some(s) if s == "0" => {}
            v => {
                return ProbeResult::Fail {
                    reason: "getGamepads() should return empty array (length 0)".into(),
                    expected: "0".into(),
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
    fn test_get_gamepads_probe_name_and_desc() {
        let probe = GetGamepadsProbe;
        assert!(!probe.name().is_empty());
        assert!(!probe.description().is_empty());
    }
}
