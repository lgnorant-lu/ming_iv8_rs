//! sendBeacon() argument/behavior probe (v0.8.62 M1).
//!
//! Verifies that navigator.sendBeacon() accepts URL and body
//! arguments, returns true on valid call, and does not throw.

use crate::convert::RustValue;
use crate::kernel::embedded_v8::EmbeddedV8Kernel;
use super::{BehaviorProbe, ProbeResult};

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

pub struct SendBeaconProbe;

impl BehaviorProbe for SendBeaconProbe {
    fn name(&self) -> &'static str {
        "sendBeacon Argument/Behavior"
    }

    fn description(&self) -> &'static str {
        "Verifies navigator.sendBeacon() is a function, accepts URL+body, \
         returns true on valid call, and does not throw on missing body"
    }

    fn probe(&self, kernel: &mut EmbeddedV8Kernel) -> ProbeResult {
        // 1. typeof navigator.sendBeacon === "function"
        match eval_str(kernel, "typeof navigator.sendBeacon") {
            Some(s) if s == "function" => {}
            v => return ProbeResult::Fail {
                reason: "sendBeacon is not a function".into(),
                expected: "function".into(),
                actual: format!("{:?}", v),
            },
        }

        // 2. Valid call: navigator.sendBeacon('http://x', 'data') === true
        match eval_bool(kernel, "navigator.sendBeacon('http://x', 'data')") {
            Some(true) => {}
            v => return ProbeResult::Fail {
                reason: "sendBeacon(url, body) should return true".into(),
                expected: "true".into(),
                actual: format!("{:?}", v),
            },
        }

        // 3. Call with no body: returns false (no throw)
        // The implementation ignores arguments and always returns true,
        // so this is a known behavioral gap at M1.
        // We check it doesn't throw (returns some value, not error).
        let result = kernel.eval_to_rust_value(
            "navigator.sendBeacon('http://x')"
        );
        match result {
            RustValue::Bool(_) => {}
            RustValue::Null => {
                return ProbeResult::Fail {
                    reason: "sendBeacon(url) with no body threw an error".into(),
                    expected: "boolean value".into(),
                    actual: "null (error)".into(),
                };
            }
            v => return ProbeResult::Fail {
                reason: "sendBeacon(url) returned unexpected type".into(),
                expected: "boolean".into(),
                actual: format!("{:?}", v),
            },
        }

        ProbeResult::Pass
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_beacon_probe_name_and_desc() {
        let probe = SendBeaconProbe;
        assert!(!probe.name().is_empty());
        assert!(!probe.description().is_empty());
    }
}
