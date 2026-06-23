//! Structured probe result records (Track D of v0.8.72).
//!
//! `ProbeRecord` captures the full context of a single probe execution
//! in a machine-readable format. Records can be serialized to JSON for
//! version-to-version diffing and automated coverage tracking.
//!
//! Schema version: v0.8.72 initial.

use serde::Serialize;

use crate::kernel::embedded_v8::EmbeddedV8Kernel;
use crate::shims::probes::ProbeResult;

#[derive(Debug, Clone, Serialize)]
pub struct ProbeRecord {
    pub surface_id: String,
    pub expression: String,
    pub expected_class: String,
    pub actual_class: String,
    pub expected_value_regex: Option<String>,
    pub actual_value: Option<String>,
    pub gap_kind: String,
    pub route: String,
    pub tier: String,
    pub verification_owner: String,
}

impl ProbeRecord {
    pub fn from_probe_result(
        surface_id: &str,
        expression: &str,
        expected_class: &str,
        actual_class: &str,
        result: &ProbeResult,
        route: &str,
        tier: &str,
        verification_owner: &str,
    ) -> Self {
        let gap_kind = match result {
            ProbeResult::Pass => "present".to_string(),
            ProbeResult::Fail { .. } => "mismatch".to_string(),
            ProbeResult::Skip { .. } => "boundary-only".to_string(),
        };
        Self {
            surface_id: surface_id.to_string(),
            expression: expression.to_string(),
            expected_class: expected_class.to_string(),
            actual_class: actual_class.to_string(),
            expected_value_regex: None,
            actual_value: None,
            gap_kind,
            route: route.to_string(),
            tier: tier.to_string(),
            verification_owner: verification_owner.to_string(),
        }
    }

    pub fn with_value_match(
        mut self,
        expected_regex: &str,
        actual_value: &str,
    ) -> Self {
        self.expected_value_regex = Some(expected_regex.to_string());
        self.actual_value = Some(actual_value.to_string());
        self
    }
}

/// Collect structured probe records for all registered behavior probes.
pub fn collect_probe_records(
    kernel: &mut EmbeddedV8Kernel,
) -> Vec<ProbeRecord> {
    let probes: Vec<Box<dyn super::BehaviorProbe>> = vec![
        Box::new(super::probe_battery::GetBatteryProbe),
        Box::new(super::probe_clipboard::ClipboardProbe),
        Box::new(super::probe_connection::ConnectionProbe),
        Box::new(super::probe_credentials::CredentialsProbe),
        Box::new(super::probe_eme::RequestMediaKeySystemAccessProbe),
        Box::new(super::probe_gamepad::GetGamepadsProbe),
        Box::new(super::probe_geolocation::GeolocationProbe),
        Box::new(super::probe_midi::RequestMidiAccessProbe),
        Box::new(super::probe_send_beacon::SendBeaconProbe),
    ];

    let mut records = Vec::new();
    for probe in &probes {
        let result = probe.probe(kernel);
        let record = ProbeRecord::from_probe_result(
            probe.name(),
            probe.name(),
            "n/a",
            "n/a",
            &result,
            "native-high-signal-stub",
            "T2a",
            "test_shims_probes",
        );
        records.push(record);
    }
    records
}

/// Serialize probe records to a JSON string.
pub fn probe_records_to_json(records: &[ProbeRecord]) -> String {
    serde_json::to_string_pretty(records).unwrap_or_else(|_| "[]".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe_record_structure() {
        let record = ProbeRecord {
            surface_id: "navigator.userAgent".to_string(),
            expression: "navigator.userAgent".to_string(),
            expected_class: "string".to_string(),
            actual_class: "string".to_string(),
            expected_value_regex: Some("Mozilla/5\\.0.*".to_string()),
            actual_value: Some("Mozilla/5.0 (Windows NT 10.0; Win64; x64) ...".to_string()),
            gap_kind: "present".to_string(),
            route: "profile-value".to_string(),
            tier: "T2b".to_string(),
            verification_owner: "test_surface_coverage".to_string(),
        };

        assert_eq!(record.surface_id, "navigator.userAgent");
        assert_eq!(record.gap_kind, "present");
        assert_eq!(record.tier, "T2b");
    }

    #[test]
    fn test_probe_record_serialization() {
        let record = ProbeRecord {
            surface_id: "navigator.webdriver".to_string(),
            expression: "navigator.webdriver".to_string(),
            expected_class: "boolean".to_string(),
            actual_class: "boolean".to_string(),
            expected_value_regex: None,
            actual_value: None,
            gap_kind: "present".to_string(),
            route: "descriptor-template".to_string(),
            tier: "T2b".to_string(),
            verification_owner: "test_surface_coverage".to_string(),
        };

        let json = serde_json::to_string(&record).unwrap();
        assert!(json.contains("navigator.webdriver"));
        assert!(json.contains("surface_id"));
        assert!(json.contains("gap_kind"));
    }

    #[test]
    fn test_probe_record_from_pass() {
        let result = ProbeResult::Pass;
        let record = ProbeRecord::from_probe_result(
            "test.surface",
            "test.surface",
            "object",
            "object",
            &result,
            "profile-value",
            "T1",
            "test_surface_coverage",
        );
        assert_eq!(record.gap_kind, "present");
    }

    #[test]
    fn test_probe_record_from_fail() {
        let result = ProbeResult::Fail {
            reason: "type mismatch".to_string(),
            expected: "string".to_string(),
            actual: "number".to_string(),
        };
        let record = ProbeRecord::from_probe_result(
            "test.fail",
            "test.fail",
            "string",
            "number",
            &result,
            "profile-value",
            "T1",
            "test_surface_coverage",
        );
        assert_eq!(record.gap_kind, "mismatch");
    }

    #[test]
    fn test_probe_records_json_array() {
        let records = vec![
            ProbeRecord {
                surface_id: "a".to_string(),
                expression: "a".to_string(),
                expected_class: "string".to_string(),
                actual_class: "string".to_string(),
                expected_value_regex: None,
                actual_value: None,
                gap_kind: "present".to_string(),
                route: "profile-value".to_string(),
                tier: "T1".to_string(),
                verification_owner: "test".to_string(),
            },
        ];
        let json = probe_records_to_json(&records);
        assert!(json.starts_with("["));
        assert!(json.contains("\"surface_id\""));
        assert!(json.contains("\"gap_kind\""));
    }
}
