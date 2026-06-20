//! getBattery() Promise shape probe (v0.8.62 M1).
//!
//! Verifies that navigator.getBattery() returns a native Promise
//! with correct constructor chain, then-ability, and native descriptor.
//! BatteryManager object shape inspection deferred to v0.9+
//! (requires async .then() callback inspection).

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

pub struct GetBatteryProbe;

impl BehaviorProbe for GetBatteryProbe {
    fn name(&self) -> &'static str {
        "getBattery Promise Shape"
    }

    fn description(&self) -> &'static str {
        "Verifies navigator.getBattery() returns a native Promise \
         with instanceof Promise, constructor === Promise, .then method, \
         and native code descriptor (no prototype property)"
    }

    fn probe(&self, kernel: &mut EmbeddedV8Kernel) -> ProbeResult {
        // 1. typeof navigator.getBattery === "function"
        match eval_str(kernel, "typeof navigator.getBattery") {
            Some(s) if s == "function" => {}
            v => return ProbeResult::Fail {
                reason: "getBattery is not a function".into(),
                expected: "function".into(),
                actual: format!("{:?}", v),
            },
        }

        // 2. navigator.getBattery() instanceof Promise
        match eval_bool(kernel, "navigator.getBattery() instanceof Promise") {
            Some(true) => {}
            v => return ProbeResult::Fail {
                reason: "getBattery() is not instanceof Promise".into(),
                expected: "true".into(),
                actual: format!("{:?}", v),
            },
        }

        // 3. constructor chain
        match eval_bool(kernel, "navigator.getBattery().constructor === Promise") {
            Some(true) => {}
            v => return ProbeResult::Fail {
                reason: "getBattery() constructor is not Promise".into(),
                expected: "true".into(),
                actual: format!("{:?}", v),
            },
        }

        // 4. then-able
        match eval_str(kernel, "typeof navigator.getBattery().then") {
            Some(s) if s == "function" => {}
            v => return ProbeResult::Fail {
                reason: "getBattery() Promise has no .then method".into(),
                expected: "function".into(),
                actual: format!("{:?}", v),
            },
        }

        // 5. Native code descriptor: no prototype property
        match eval_bool(kernel, "'prototype' in navigator.getBattery") {
            Some(false) => {}
            v => return ProbeResult::Fail {
                reason: "getBattery function has prototype (not native code shape)".into(),
                expected: "false".into(),
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
    fn test_get_battery_probe_name_and_desc() {
        let probe = GetBatteryProbe;
        assert!(!probe.name().is_empty());
        assert!(!probe.description().is_empty());
    }
}
