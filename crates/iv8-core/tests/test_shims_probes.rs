//! v0.8.62: Integration tests for behavior probe harness.
mod common;

use iv8_core::shims::probes::probe_battery::GetBatteryProbe;
use iv8_core::shims::probes::probe_send_beacon::SendBeaconProbe;
use iv8_core::shims::probes::{BehaviorProbe, ProbeResult};

#[test]
fn test_get_battery_probe_passes() {
    let mut k = common::make_kernel();
    let probe = GetBatteryProbe;
    let result = probe.probe(&mut k);
    assert_eq!(result, ProbeResult::Pass,
        "getBattery probe failed: {:?}", result);
}

#[test]
fn test_send_beacon_probe_passes() {
    let mut k = common::make_kernel();
    let probe = SendBeaconProbe;
    let result = probe.probe(&mut k);
    assert_eq!(result, ProbeResult::Pass,
        "sendBeacon probe failed: {:?}", result);
}

#[test]
fn test_probe_trait_structure() {
    let battery = GetBatteryProbe;
    assert!(!battery.name().is_empty());
    assert!(!battery.description().is_empty());

    let beacon = SendBeaconProbe;
    assert!(!beacon.name().is_empty());
    assert!(!beacon.description().is_empty());
}
