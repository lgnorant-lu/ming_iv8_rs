//! v0.8.62: Integration tests for behavior probe harness.
//! v0.8.63: M2 probes — connection, geolocation, clipboard, credentials.
//! v0.8.71: M3 probes — getGamepads, requestMediaKeySystemAccess, requestMIDIAccess.
mod common;

use iv8_core::shims::probes::probe_battery::GetBatteryProbe;
use iv8_core::shims::probes::probe_clipboard::ClipboardProbe;
use iv8_core::shims::probes::probe_connection::ConnectionProbe;
use iv8_core::shims::probes::probe_credentials::CredentialsProbe;
use iv8_core::shims::probes::probe_eme::RequestMediaKeySystemAccessProbe;
use iv8_core::shims::probes::probe_gamepad::GetGamepadsProbe;
use iv8_core::shims::probes::probe_geolocation::GeolocationProbe;
use iv8_core::shims::probes::probe_midi::RequestMidiAccessProbe;
use iv8_core::shims::probes::probe_send_beacon::SendBeaconProbe;
use iv8_core::shims::probes::{BehaviorProbe, ProbeResult};

#[test]
fn test_get_battery_probe_passes() {
    let mut k = common::make_kernel();
    let probe = GetBatteryProbe;
    let result = probe.probe(&mut k);
    assert_eq!(
        result,
        ProbeResult::Pass,
        "getBattery probe failed: {:?}",
        result
    );
}

#[test]
fn test_send_beacon_probe_passes() {
    let mut k = common::make_kernel();
    let probe = SendBeaconProbe;
    let result = probe.probe(&mut k);
    assert_eq!(
        result,
        ProbeResult::Pass,
        "sendBeacon probe failed: {:?}",
        result
    );
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

#[test]
fn test_connection_probe_passes() {
    let mut k = common::make_kernel();
    let probe = ConnectionProbe;
    let result = probe.probe(&mut k);
    assert_eq!(
        result,
        ProbeResult::Pass,
        "connection probe failed: {:?}",
        result
    );
}

#[test]
fn test_geolocation_probe_passes() {
    let mut k = common::make_kernel();
    let probe = GeolocationProbe;
    let result = probe.probe(&mut k);
    assert_eq!(
        result,
        ProbeResult::Pass,
        "geolocation probe failed: {:?}",
        result
    );
}

#[test]
fn test_clipboard_probe_passes() {
    let mut k = common::make_kernel();
    let probe = ClipboardProbe;
    let result = probe.probe(&mut k);
    assert_eq!(
        result,
        ProbeResult::Pass,
        "clipboard probe failed: {:?}",
        result
    );
}

#[test]
fn test_credentials_probe_passes() {
    let mut k = common::make_kernel();
    let probe = CredentialsProbe;
    let result = probe.probe(&mut k);
    assert_eq!(
        result,
        ProbeResult::Pass,
        "credentials probe failed: {:?}",
        result
    );
}

#[test]
fn test_get_gamepads_probe_passes() {
    let mut k = common::make_kernel();
    let probe = GetGamepadsProbe;
    let result = probe.probe(&mut k);
    assert_eq!(
        result,
        ProbeResult::Pass,
        "getGamepads probe failed: {:?}",
        result
    );
}

#[test]
fn test_request_media_key_system_access_probe_passes() {
    let mut k = common::make_kernel();
    let probe = RequestMediaKeySystemAccessProbe;
    let result = probe.probe(&mut k);
    assert_eq!(
        result,
        ProbeResult::Pass,
        "requestMediaKeySystemAccess probe failed: {:?}",
        result
    );
}

#[test]
fn test_request_midi_access_probe_passes() {
    let mut k = common::make_kernel();
    let probe = RequestMidiAccessProbe;
    let result = probe.probe(&mut k);
    assert_eq!(
        result,
        ProbeResult::Pass,
        "requestMIDIAccess probe failed: {:?}",
        result
    );
}
