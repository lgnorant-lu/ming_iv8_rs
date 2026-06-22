//! Behavior probe harness — foundation for semantic-behavior-probe route.
//!
//! v0.8.62 M1: ProbeResult + BehaviorProbe trait.
//! Probes validate that native stubs exhibit correct behavioral shape
//! beyond simple type checks. L2 tests instantiate probes and assert
//! ProbeResult::Pass.
//!
//! BatteryManager object shape inspection via .then() callbacks is
//! deferred to v0.9+ (requires async microtask orchestration).

use crate::kernel::embedded_v8::EmbeddedV8Kernel;

pub mod probe_battery;
pub mod probe_clipboard;
pub mod probe_connection;
pub mod probe_credentials;
pub mod probe_eme;
pub mod probe_gamepad;
pub mod probe_geolocation;
pub mod probe_midi;
pub mod probe_send_beacon;

/// Result of a single probe execution.
#[derive(Debug, PartialEq)]
pub enum ProbeResult {
    /// Probe passed all checks.
    Pass,
    /// Probe failed with diagnostic details.
    Fail {
        reason: String,
        expected: String,
        actual: String,
    },
    /// Probe was skipped (e.g., feature not applicable).
    Skip { reason: String },
}

/// A behavior probe that validates a specific browser API shape.
pub trait BehaviorProbe {
    /// Human-readable name of the probe.
    fn name(&self) -> &'static str;

    /// What the probe verifies.
    fn description(&self) -> &'static str;

    /// Execute the probe against a kernel instance.
    fn probe(&self, kernel: &mut EmbeddedV8Kernel) -> ProbeResult;
}
