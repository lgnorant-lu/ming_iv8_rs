use serde::{Deserialize, Serialize};

/// Profile verification report — output evidence from probe execution.
///
/// Records: what was observed, what passed/failed, which divergences
/// were expected vs unexpected, and the final verdict.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfileReport {
    pub schema_version: String,
    pub run: RunInfo,
    pub input: ReportInput,
    pub summary: ReportSummary,
    pub verdict: String,
    pub certification: CertificationInfo,
    #[serde(default)]
    pub observations: serde_json::Value,
    #[serde(default)]
    pub probe_results: Vec<ProbeResult>,
    #[serde(default)]
    pub divergences: Vec<ReportDivergence>,
    #[serde(default)]
    pub writes: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RunInfo {
    pub report_id: String,
    pub started_at: String,
    pub duration_ms: u64,
    #[serde(default)]
    pub iv8_version: String,
    #[serde(default)]
    pub host: HostInfo,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct HostInfo {
    #[serde(default)]
    pub os: String,
    #[serde(default)]
    pub arch: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReportInput {
    pub manifest_id: String,
    #[serde(default)]
    pub manifest_hash: String,
    #[serde(default)]
    pub profile_ref: String,
    #[serde(default)]
    pub profile_hash: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReportSummary {
    pub total: u32,
    pub passed: u32,
    pub failed: u32,
    pub material_failures: u32,
    pub expected_divergences: usize,
    pub unexpected_divergences: usize,
    #[serde(default)]
    pub normalized_diffs: usize,
    #[serde(default)]
    pub pass_pct: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CertificationInfo {
    pub level: String,
    pub browser_zero_diff: bool,
    #[serde(default)]
    pub certifies: Vec<String>,
    #[serde(default)]
    pub does_not_certify: Vec<String>,
}

/// Single probe execution result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProbeResult {
    pub probe_id: String,
    pub target: String,
    pub category: String,
    pub status: String,
    #[serde(default)]
    pub expected: serde_json::Value,
    #[serde(default)]
    pub actual: serde_json::Value,
    #[serde(default)]
    pub diff_class: Option<String>,
    #[serde(default)]
    pub evidence_ceiling: String,
}

/// A recorded divergence (expected or unexpected).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReportDivergence {
    pub path: String,
    #[serde(rename = "class")]
    pub class: String,
    pub category: String,
    pub reason: String,
    pub material: bool,
}

impl ProfileReport {
    /// Create a new empty report bound to a manifest.
    pub fn new(manifest_id: &str, profile_ref: &str) -> Self {
        Self {
            schema_version: "iv8-profile-report.v0.1".into(),
            run: RunInfo {
                report_id: format!("{}-{}", manifest_id, chrono_now()),
                started_at: chrono_now(),
                duration_ms: 0,
                iv8_version: env!("CARGO_PKG_VERSION").into(),
                host: HostInfo {
                    os: std::env::consts::OS.into(),
                    arch: std::env::consts::ARCH.into(),
                },
            },
            input: ReportInput {
                manifest_id: manifest_id.into(),
                manifest_hash: String::new(),
                profile_ref: profile_ref.into(),
                profile_hash: String::new(),
            },
            summary: ReportSummary {
                total: 0,
                passed: 0,
                failed: 0,
                material_failures: 0,
                expected_divergences: 0,
                unexpected_divergences: 0,
                normalized_diffs: 0,
                pass_pct: 100.0,
            },
            verdict: "equivalent".into(),
            certification: CertificationInfo {
                level: "deterministic_v8_surface_equivalence".into(),
                browser_zero_diff: false,
                certifies: vec!["profile_loaded".into(), "static_js_values".into()],
                does_not_certify: vec![
                    "chromium_layout_parity".into(),
                    "network_stack_parity".into(),
                    "creepjs_grade".into(),
                    "browserleaks_pass".into(),
                ],
            },
            observations: serde_json::Value::Object(Default::default()),
            probe_results: Vec::new(),
            divergences: Vec::new(),
            writes: Vec::new(),
        }
    }

    /// Record a probe result.
    pub fn add_probe(&mut self, result: ProbeResult) {
        self.summary.total += 1;
        match result.status.as_str() {
            "pass" => self.summary.passed += 1,
            "material" | "fail" => {
                self.summary.failed += 1;
                self.summary.material_failures += 1;
            }
            "expected_divergence" => {
                self.summary.expected_divergences += 1;
            }
            "unexpected_divergence" => {
                self.summary.unexpected_divergences += 1;
            }
            _ => {}
        }
        let denom = self
            .summary
            .total
            .saturating_sub(self.summary.expected_divergences as u32);
        if denom > 0 {
            self.summary.pass_pct = (self.summary.passed as f64 / denom as f64) * 100.0;
        } else {
            self.summary.pass_pct = 100.0;
        }
        self.probe_results.push(result);
    }

    /// Compute final verdict from summary using default acceptance criteria.
    pub fn finalize(&mut self) {
        if self.summary.total == 0 {
            self.verdict = "no_data".into();
        } else if self.summary.material_failures > 0 || self.summary.unexpected_divergences > 0 {
            self.verdict = "failed".into();
        } else if self.summary.pass_pct >= 100.0 {
            self.verdict = "equivalent".into();
        } else {
            self.verdict = "partial".into();
        }
    }
}

/// Build a stable ISO-8601-like timestamp without pulling in chrono crate.
fn chrono_now() -> String {
    use std::time::UNIX_EPOCH;
    let dur = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = dur.as_secs();
    format!("{}-sec", secs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn report_probes_accumulate() {
        let mut r = ProfileReport::new("static-core", "win.profile.json");
        r.add_probe(ProbeResult {
            probe_id: "ua".into(),
            target: "navigator.userAgent".into(),
            category: "value".into(),
            status: "pass".into(),
            expected: serde_json::Value::Null,
            actual: serde_json::Value::Null,
            diff_class: None,
            evidence_ceiling: "v8_surface".into(),
        });
        r.add_probe(ProbeResult {
            probe_id: "font".into(),
            target: "fonts".into(),
            category: "value".into(),
            status: "expected_divergence".into(),
            expected: serde_json::Value::Null,
            actual: serde_json::Value::Null,
            diff_class: None,
            evidence_ceiling: "browser_only".into(),
        });
        r.finalize();
        assert_eq!(r.summary.total, 2);
        assert_eq!(r.summary.passed, 1);
        assert_eq!(r.summary.material_failures, 0);
        assert_eq!(r.verdict, "equivalent");
    }

    #[test]
    fn report_finalize_fails_on_material() {
        let mut r = ProfileReport::new("test", "test.profile.json");
        r.add_probe(ProbeResult {
            probe_id: "bad".into(),
            target: "navigator.platform".into(),
            category: "value".into(),
            status: "material".into(),
            expected: serde_json::json!("Win32"),
            actual: serde_json::json!("Linux"),
            diff_class: None,
            evidence_ceiling: "v8_surface".into(),
        });
        r.finalize();
        assert_eq!(r.verdict, "failed");
        assert!(r.summary.material_failures > 0);
    }

    #[test]
    fn report_finalize_no_data_on_empty_report() {
        let mut r = ProfileReport::new("test", "test.profile.json");
        r.finalize();
        assert_eq!(r.verdict, "no_data");
    }

    #[test]
    fn report_roundtrip() {
        let mut r = ProfileReport::new("id", "ref");
        r.add_probe(ProbeResult {
            probe_id: "p1".into(),
            target: "t".into(),
            category: "c".into(),
            status: "pass".into(),
            expected: serde_json::Value::Null,
            actual: serde_json::Value::Null,
            diff_class: None,
            evidence_ceiling: "e".into(),
        });
        r.finalize();
        let json = serde_json::to_string_pretty(&r).expect("serialize");
        let _back: ProfileReport = serde_json::from_str(&json).expect("deserialize");
    }
}
