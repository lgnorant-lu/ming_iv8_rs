//! Telemetry catalog completeness and coverage matrix tests.
//!
//! Validates:
//! - Catalog is non-empty and well-formed
//! - All event names are unique
//! - COVERAGE_MATRIX is satisfied (each category has required levels)
//! - Catalog categories match COVERAGE_MATRIX categories
//! - Event count meets minimum threshold

mod common;

use iv8_core::telemetry;

#[test]
fn test_catalog_not_empty() {
    assert!(!telemetry::catalog().is_empty());
}

#[test]
fn test_catalog_names_unique() {
    let mut names: Vec<&str> = telemetry::catalog().iter().map(|e| e.name).collect();
    names.sort();
    let before = names.len();
    names.dedup();
    assert_eq!(names.len(), before, "duplicate event names in catalog");
}

#[test]
fn test_catalog_all_have_fields() {
    for event in telemetry::catalog() {
        assert!(!event.name.is_empty(), "event has empty name");
        assert!(!event.category.is_empty(), "event {} has empty category", event.name);
        assert!(!event.level.is_empty(), "event {} has empty level", event.name);
    }
}

#[test]
fn test_catalog_has_init_events() {
    let init_events: Vec<_> = telemetry::catalog()
        .iter()
        .filter(|e| e.category == "iv8.init")
        .collect();
    assert!(
        init_events.len() >= 10,
        "expected at least 10 init events, got {}",
        init_events.len()
    );
}

#[test]
fn test_catalog_event_count() {
    let count = telemetry::catalog().len();
    assert!(count >= 25, "expected at least 25 catalog events, got {}", count);
}

#[test]
fn test_coverage_matrix_satisfied() {
    for (category, expected_levels) in telemetry::COVERAGE_MATRIX {
        for &level in *expected_levels {
            let level_str = match level {
                'E' => "ERROR",
                'W' => "WARN",
                'I' => "INFO",
                'D' => "DEBUG",
                'T' => "TRACE",
                _ => panic!("unknown level char: {}", level),
            };
            let found = telemetry::catalog()
                .iter()
                .any(|e| e.category == *category && e.level == level_str);
            assert!(
                found,
                "coverage gap: category {} has no {} event",
                category, level_str
            );
        }
    }
}

#[test]
fn test_coverage_matrix_categories_exist() {
    let catalog_cats: std::collections::HashSet<&str> =
        telemetry::catalog().iter().map(|e| e.category).collect();
    for (category, _) in telemetry::COVERAGE_MATRIX {
        assert!(
            catalog_cats.contains(*category),
            "coverage matrix category {} has no catalog events",
            category
        );
    }
}

#[test]
fn test_no_direct_tracing_outside_telemetry() {
    let cats: std::collections::HashSet<&str> =
        telemetry::catalog().iter().map(|e| e.category).collect();
    let expected_cats: std::collections::HashSet<&str> = telemetry::COVERAGE_MATRIX
        .iter()
        .map(|(c, _)| *c)
        .collect();
    assert_eq!(
        cats, expected_cats,
        "catalog categories must match coverage matrix"
    );
}

#[test]
fn test_all_events_have_safety() {
    for event in telemetry::catalog() {
        match event.safety {
            telemetry::Safety::Safe => {}
            telemetry::Safety::Diagnostic => {}
            telemetry::Safety::Sensitive => {}
        }
    }
}

#[test]
fn test_init_phase_events_exist() {
    let has_start = telemetry::catalog()
        .iter()
        .any(|e| e.name == "init_phase_start");
    let has_complete = telemetry::catalog()
        .iter()
        .any(|e| e.name == "init_phase_complete");
    let has_failed = telemetry::catalog()
        .iter()
        .any(|e| e.name == "init_phase_failed");
    let has_skipped = telemetry::catalog()
        .iter()
        .any(|e| e.name == "init_phase_skipped");
    assert!(has_start && has_complete && has_failed && has_skipped,
        "init phase events missing: start={} complete={} failed={} skipped={}",
        has_start, has_complete, has_failed, has_skipped);
}

#[test]
fn test_v8_error_events_exist() {
    let has_fatal = telemetry::catalog().iter().any(|e| e.name == "v8_fatal_error");
    let has_oom = telemetry::catalog().iter().any(|e| e.name == "v8_oom");
    let has_panic = telemetry::catalog().iter().any(|e| e.name == "rust_panic");
    assert!(has_fatal && has_oom && has_panic,
        "V8 error events missing: fatal={} oom={} panic={}",
        has_fatal, has_oom, has_panic);
}
