//! navigator.geolocation object-shape probe (v0.8.63 M2).
//!
//! Verifies that navigator.geolocation is a non-null object with three
//! method-typed properties (getCurrentPosition, watchPosition, clearWatch).
//! Callback shape and permission behavior deferred to v0.9+.

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

pub struct GeolocationProbe;

impl BehaviorProbe for GeolocationProbe {
    fn name(&self) -> &'static str {
        "navigator.geolocation Object Shape"
    }

    fn description(&self) -> &'static str {
        "Verifies navigator.geolocation is an object with \
         getCurrentPosition, watchPosition, clearWatch methods \
         and native code descriptor"
    }

    fn probe(&self, kernel: &mut EmbeddedV8Kernel) -> ProbeResult {
        match eval_str(kernel, "typeof navigator.geolocation") {
            Some(s) if s == "object" => {}
            v => {
                return ProbeResult::Fail {
                    reason: "navigator.geolocation is not an object".into(),
                    expected: "object".into(),
                    actual: format!("{:?}", v),
                };
            }
        }

        for method in &["getCurrentPosition", "watchPosition", "clearWatch"] {
            let expr = format!("typeof navigator.geolocation.{}", method);
            match eval_str(kernel, &expr) {
                Some(s) if s == "function" => {}
                v => {
                    return ProbeResult::Fail {
                        reason: format!("navigator.geolocation.{} is not a function", method),
                        expected: "function".into(),
                        actual: format!("{:?}", v),
                    };
                }
            }
        }

        match eval_bool(kernel, "'prototype' in Object.getOwnPropertyDescriptor(navigator.__proto__, 'geolocation').get") {
            Some(false) => {}
            v => {
                return ProbeResult::Fail {
                    reason: "navigator.geolocation getter has prototype (not native code shape)".into(),
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
    fn test_geolocation_probe_name_and_desc() {
        let probe = GeolocationProbe;
        assert!(!probe.name().is_empty());
        assert!(!probe.description().is_empty());
    }
}
