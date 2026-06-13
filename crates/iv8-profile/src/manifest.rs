use serde::{Deserialize, Serialize};

/// Profile verification manifest — defines the input contract for verification.
///
/// Specifies: what profile to verify, which probes to run, what divergences
/// are expected, and what acceptance thresholds apply.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileManifest {
    pub schema_version: String,
    pub manifest_id: String,
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub iv8_version: String,
    #[serde(default)]
    pub profile_ref: String,
    #[serde(default)]
    pub profile_hash: String,
    pub scope: VerificationScope,
    #[serde(default)]
    pub normalization: NormalizationRules,
    #[serde(default)]
    pub probe_suites: Vec<ProbeSuite>,
    #[serde(default)]
    pub expected_divergences: Vec<ExpectedDivergence>,
    pub acceptance: AcceptanceCriteria,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationScope {
    pub engine: String,
    pub runtime: String,
    pub certification_level: String,
    #[serde(default)]
    pub not_browser_zero_diff: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NormalizationRules {
    #[serde(default)]
    pub stable_sort_paths: Vec<String>,
    #[serde(default)]
    pub volatile_paths: Vec<String>,
    #[serde(default)]
    pub ignore_paths: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProbeSuite {
    pub suite_id: String,
    #[serde(default)]
    pub source: String,
    pub probes: Vec<ProbeDef>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProbeDef {
    pub probe_id: String,
    #[serde(default)]
    pub target: String,
    pub category: String,
    pub surface: String,
    pub js: String,
    pub expect: ProbeExpectation,
    #[serde(default)]
    pub evidence_ceiling: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProbeExpectation {
    pub op: String,
    #[serde(default)]
    pub value: serde_json::Value,
    #[serde(default)]
    pub path: String,
}

impl ProbeExpectation {
    pub fn equals(value: serde_json::Value) -> Self {
        Self { op: "equals".into(), value, path: String::new() }
    }
    pub fn equals_profile_path(path: &str) -> Self {
        Self { op: "equals_profile_path".into(), value: serde_json::Value::Null, path: path.into() }
    }
    pub fn json_superset(value: serde_json::Value) -> Self {
        Self { op: "json_superset".into(), value, path: String::new() }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpectedDivergence {
    pub path: String,
    pub category: String,
    pub reason: String,
    #[serde(default)]
    pub evidence_ceiling: String,
    #[serde(default)]
    pub material: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AcceptanceCriteria {
    #[serde(default = "default_verdict")]
    pub verdict: String,
    pub max_material_failures: u32,
    pub max_unexpected_divergences: u32,
    #[serde(default)]
    pub require_deterministic_rerun: bool,
    #[serde(default)]
    pub require_no_writes: bool,
    #[serde(default)]
    pub minimum_probe_pass_pct: f64,
}

fn default_verdict() -> String {
    "equivalent".into()
}

impl Default for NormalizationRules {
    fn default() -> Self {
        Self {
            stable_sort_paths: Vec::new(),
            volatile_paths: vec![
                "$.run.started_at".into(),
                "$.run.duration_ms".into(),
            ],
            ignore_paths: Vec::new(),
        }
    }
}

impl Default for AcceptanceCriteria {
    fn default() -> Self {
        Self {
            verdict: "equivalent".into(),
            max_material_failures: 0,
            max_unexpected_divergences: 0,
            require_deterministic_rerun: true,
            require_no_writes: true,
            minimum_probe_pass_pct: 100.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Static core probe suite for navigator/screen static values
// ---------------------------------------------------------------------------

/// Build a minimal static-core probe suite for navigator + screen verification.
/// This is the v0.8.32 minimum verification set.
pub fn static_core_probe_suite() -> ProbeSuite {
    let probes = vec![
        simple_value_probe("navigator.userAgent.value", "value", "navigator", "navigator.userAgent"),
        simple_value_probe("navigator.platform.value", "value", "navigator", "navigator.platform"),
        simple_value_probe("navigator.vendor.value", "value", "navigator", "navigator.vendor"),
        simple_value_probe("navigator.language.value", "value", "navigator", "navigator.language"),
        simple_value_probe("navigator.hardwareConcurrency.value", "value", "navigator", "navigator.hardwareConcurrency"),
        simple_value_probe("navigator.deviceMemory.value", "value", "navigator", "navigator.deviceMemory"),
        simple_boolean_probe("navigator.webdriver.false", "value", "navigator", "navigator.webdriver", false),
        atomic_probe("navigator.webdriver.descriptor", "descriptor", "navigator", "(function(){var d=Object.getOwnPropertyDescriptor(Navigator.prototype,'webdriver')||Object.getOwnPropertyDescriptor(navigator,'webdriver');return {exists:!!d,enumerable:d&&d.enumerable,configurable:d&&d.configurable,hasGetter:!!(d&&d.get)};})()"),
        simple_value_probe("screen.width.value", "value", "screen", "screen.width"),
        simple_value_probe("screen.height.value", "value", "screen", "screen.height"),
        simple_value_probe("screen.colorDepth.value", "value", "screen", "screen.colorDepth"),
        simple_value_probe("window.devicePixelRatio.value", "value", "window", "window.devicePixelRatio"),
        atomic_probe("navigator.languages.length", "value", "navigator", "navigator.languages.length"),
    ];

    ProbeSuite {
        suite_id: "static-core".into(),
        source: "builtin".into(),
        probes,
    }
}

fn simple_value_probe(id: &str, category: &str, surface: &str, js_expr: &str) -> ProbeDef {
    ProbeDef {
        probe_id: id.into(),
        target: js_expr.into(),
        category: category.into(),
        surface: surface.into(),
        js: js_expr.into(),
        expect: ProbeExpectation {
            op: "check_by_report".into(),
            value: serde_json::Value::Null,
            path: String::new(),
        },
        evidence_ceiling: "v8_surface".into(),
    }
}

fn simple_boolean_probe(id: &str, category: &str, surface: &str, js_expr: &str, expected: bool) -> ProbeDef {
    ProbeDef {
        probe_id: id.into(),
        target: js_expr.into(),
        category: category.into(),
        surface: surface.into(),
        js: js_expr.into(),
        expect: ProbeExpectation::equals(serde_json::json!(expected)),
        evidence_ceiling: "v8_surface".into(),
    }
}

fn atomic_probe(id: &str, category: &str, surface: &str, js: &str) -> ProbeDef {
    ProbeDef {
        probe_id: id.into(),
        target: String::new(),
        category: category.into(),
        surface: surface.into(),
        js: js.into(),
        expect: ProbeExpectation::json_superset(
            serde_json::json!({"exists": true, "enumerable": true, "configurable": true}),
        ),
        evidence_ceiling: "v8_surface".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_core_suite_has_probes() {
        let suite = static_core_probe_suite();
        assert!(suite.probes.len() >= 10);
    }

    #[test]
    fn manifest_roundtrip() {
        let m = ProfileManifest {
            schema_version: "iv8-profile-manifest.v0.1".into(),
            manifest_id: "test".into(),
            created_at: String::new(),
            iv8_version: "0.8.32".into(),
            profile_ref: "test.profile.json".into(),
            profile_hash: String::new(),
            scope: VerificationScope {
                engine: "v8".into(),
                runtime: "iv8-rs".into(),
                certification_level: "deterministic_v8_surface_equivalence".into(),
                not_browser_zero_diff: true,
            },
            normalization: NormalizationRules::default(),
            probe_suites: vec![static_core_probe_suite()],
            expected_divergences: vec![ExpectedDivergence {
                path: "$.observations.fonts".into(),
                category: "v8_only_limit".into(),
                reason: "iv8-rs has no font rasterization engine".into(),
                evidence_ceiling: "browser_only_validation".into(),
                material: false,
            }],
            acceptance: AcceptanceCriteria::default(),
        };

        let json = serde_json::to_string_pretty(&m).expect("serialize");
        let _back: ProfileManifest = serde_json::from_str(&json).expect("deserialize");
    }
}
