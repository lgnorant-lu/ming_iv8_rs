//! navigator.connection object-shape probe (v0.8.63 M2).
//!
//! Verifies that navigator.connection is a non-null object with
//! NetworkInformation-like shape (downlink, rtt, effectiveType).
//! Value range validation and behavior depth deferred to v0.9+.

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

pub struct ConnectionProbe;

impl BehaviorProbe for ConnectionProbe {
    fn name(&self) -> &'static str {
        "navigator.connection Object Shape"
    }

    fn description(&self) -> &'static str {
        "Verifies navigator.connection is an object with downlink (number), \
         rtt (number), effectiveType (string), and native code descriptor"
    }

    fn probe(&self, kernel: &mut EmbeddedV8Kernel) -> ProbeResult {
        match eval_str(kernel, "typeof navigator.connection") {
            Some(s) if s == "object" => {}
            v => {
                return ProbeResult::Fail {
                    reason: "navigator.connection is not an object".into(),
                    expected: "object".into(),
                    actual: format!("{:?}", v),
                };
            }
        }

        match eval_str(kernel, "typeof navigator.connection.downlink") {
            Some(s) if s == "number" => {}
            v => {
                return ProbeResult::Fail {
                    reason: "navigator.connection.downlink is not a number".into(),
                    expected: "number".into(),
                    actual: format!("{:?}", v),
                };
            }
        }

        match eval_str(kernel, "typeof navigator.connection.rtt") {
            Some(s) if s == "number" => {}
            v => {
                return ProbeResult::Fail {
                    reason: "navigator.connection.rtt is not a number".into(),
                    expected: "number".into(),
                    actual: format!("{:?}", v),
                };
            }
        }

        match eval_str(kernel, "typeof navigator.connection.effectiveType") {
            Some(s) if s == "string" => {}
            v => {
                return ProbeResult::Fail {
                    reason: "navigator.connection.effectiveType is not a string".into(),
                    expected: "string".into(),
                    actual: format!("{:?}", v),
                };
            }
        }

        match eval_bool(kernel, "'prototype' in Object.getOwnPropertyDescriptor(Navigator.prototype, 'connection').get") {
            Some(false) => {}
            v => {
                return ProbeResult::Fail {
                    reason: "navigator.connection getter has prototype (not native code shape)".into(),
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
    fn test_connection_probe_name_and_desc() {
        let probe = ConnectionProbe;
        assert!(!probe.name().is_empty());
        assert!(!probe.description().is_empty());
    }
}
