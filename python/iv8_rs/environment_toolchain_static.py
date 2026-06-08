"""Static data for Environment Toolchain runtime.

This module contains only constants, regexes, and static inventory data extracted
from `environment_toolchain_runtime.py`. It has no runtime behavior and performs
no writes.
"""

from __future__ import annotations

import re

_ALLOWED_EVIDENCE_CEILINGS = {"diagnostic_only", "weak"}
_ALLOWED_PROBE_CATEGORIES = {"presence", "descriptor", "behavior", "value"}
_GENERIC_TARGET_PREFIXES = (
    "navigator.",
    "screen.",
    "document.",
    "window.",
    "location.",
    "performance.",
    "Math.",
    "Date.",
    "crypto.",
)
_BLOCKED_BOUNDARY_TERMS = (
    "domain",
    "endpoint",
    "cookie",
    "token",
    "signature",
    "nonce",
    "request_body",
    "request body",
    "authorization",
    "secret",
)
_ALLOWED_PRESSURE_CATEGORIES = frozenset({
    "descriptor_mismatch",
    "prototype_mismatch",
    "value_mismatch",
    "missing_api",
    "behavior_mismatch",
})
_ALLOWED_TARGET_FAMILIES = frozenset({
    "navigator",
    "screen",
    "window",
    "document",
    "timing",
    "network_info",
})
_GAP_CLASS_TO_PRESSURE_CATEGORY = {
    "missing_api": "missing_api",
    "value_mismatch": "value_mismatch",
    "descriptor_mismatch": "descriptor_mismatch",
    "behavior_mismatch": "behavior_mismatch",
    "prototype_chain_mismatch": "prototype_mismatch",
}
_ORDERED_RECIPE_RE = re.compile(r"apply\s+.+request\s+.+(?:copy|rerun)", re.IGNORECASE)
_RAW_LOCAL_PATH_RE = re.compile(r"(?:^[a-zA-Z]:[\\/]|^[/\\]{1,2}(?!/)|\s[a-zA-Z]:[\\/])")
_PROBE_PACK_FILES = {
    "descriptor.m1": "descriptor.m1.json",
    "fingerprint.m1": "fingerprint.m1.json",
}
_CANDIDATE_PACK_FILES = {"chrome_generic": "chrome_generic.json"}
_ADAPTATION_STOP_REASONS = {
    "disabled",
    "completed",
    "budget_exhausted",
    "no_candidate",
    "policy_blocked",
    "boundary_blocked",
    "regression_detected",
    "no_progress",
    "entry_failure",
    "asset_error",
}
_DRY_RUN_ALLOWED_STATUSES = frozenset({
    "eligible_for_review",
    "blocked_by_boundary",
    "blocked_by_policy",
    "blocked_by_conflict",
    "requires_rollback_design",
    "requires_native_review",
    "review_only_signal",
})
_CANDIDATE_METADATA_FIELDS = frozenset({
    "coherence_group",
    "substrate_family",
    "dependency_kind",
    "expected_probe_delta",
    "evidence_ceiling",
    "planning_status",
    "rollback_scope",
    "rollback_hint",
    "boundary_checked",
    "blocked_reasons",
})
_CANDIDATE_PLANNING_STATUSES = frozenset({"not_planned"}) | _DRY_RUN_ALLOWED_STATUSES
_CANDIDATE_DEPENDENCY_KINDS = frozenset({
    "probe_pass",
    "probe_gap",
    "coherence_group_status",
    "explicit_environment_absent",
    "candidate_pack_enabled",
    "rollback_metadata_present",
    "native_review_completed",
})
_ROLLBACK_ALLOWED_SCOPES = frozenset({"context_only", "ephemeral_report"})
_ROLLBACK_BLOCKED_SCOPES = frozenset({
    "profile_file",
    "manifest",
    "baseline",
    "sample",
    "source_tree",
    "native_substrate",
    "blocked",
})
_SUBSTRATE_COVERAGE_ITEMS = (
    {
        "surface_id": "probe_pack_loading",
        "surface_family": "python_orchestration",
        "owner": "environment_toolchain_runtime.py",
        "coverage": "built-in/custom tests and provenance diagnostics",
        "boundary": "report_only",
        "review_status": "accepted",
        "gap_class": "substrate_gap",
        "gap": "no first-class substrate coverage index before v0.8.6 diagnostics",
    },
    {
        "surface_id": "candidate_pack_loading",
        "surface_family": "python_orchestration",
        "owner": "environment_toolchain_runtime.py",
        "coverage": "candidate tests, metadata validation, boundary tests",
        "boundary": "runtime_safe_only_explicit",
        "review_status": "accepted",
        "gap_class": "candidate_gap",
        "gap": "metadata is review-only and cannot authorize apply",
    },
    {
        "surface_id": "profile_coherence",
        "surface_family": "diagnostics",
        "owner": "ProfileCoherenceGroup",
        "coverage": "language, screen_window, ua_platform, network_info, timezone_locale",
        "boundary": "diagnostic_only",
        "review_status": "accepted",
        "gap_class": "evidence_gap",
        "gap": "coherence groups remain input signals, not candidate generators",
    },
    {
        "surface_id": "navigator_connection",
        "surface_family": "rust_substrate",
        "owner": "navigator_extras.rs",
        "coverage": "network_info coherence diagnostics",
        "boundary": "js_shim_stub_no_apply",
        "review_status": "requires_review",
        "gap_class": "substrate_gap",
        "gap": "hardcoded connection values are not linked to EnvironmentMap defaults",
    },
    {
        "surface_id": "navigator_ua_data",
        "surface_family": "rust_substrate",
        "owner": "user_agent_data.rs",
        "coverage": "shape probe and ua_platform coherence diagnostics",
        "boundary": "native_review_gated",
        "review_status": "accepted_with_review_gate",
        "gap_class": "probe_gap",
        "gap": "high-entropy behavior needs generic probe coverage before hardening",
    },
    {
        "surface_id": "plugins_mime_types",
        "surface_family": "rust_substrate",
        "owner": "navigator_extras.rs",
        "coverage": "basic runtime surface only",
        "boundary": "js_shim_stub_no_apply",
        "review_status": "requires_review",
        "gap_class": "probe_gap",
        "gap": "empty array-like descriptors need generic review coverage",
    },
    {
        "surface_id": "timezone_intl",
        "surface_family": "rust_substrate",
        "owner": "embedded_v8.rs",
        "coverage": "timezone_locale coherence diagnostics",
        "boundary": "native_review_gated",
        "review_status": "requires_review",
        "gap_class": "substrate_gap",
        "gap": "config.timezone vs timezone key contract remains unresolved",
    },
    {
        "surface_id": "rollback_diagnostics",
        "surface_family": "planning_scaffold",
        "owner": "run_environment_toolchain",
        "coverage": "rollback summary/record diagnostics and negative tests",
        "boundary": "diagnostic_only_no_write",
        "review_status": "accepted",
        "gap_class": "rollback_gap",
        "gap": "persistent rollback scopes remain blocked without write contract",
    },
)
_SCAFFOLD_GAP_ITEMS = (
    {
        "gap_id": "G-086-SUB-001",
        "gap_class": "substrate_gap",
        "surface_family": "timezone_intl",
        "priority": "high",
        "current_evidence": "config.timezone default and timezone runtime key diverge",
        "next_artifact": "timezone key contract review",
        "review_gate": "native_substrate_review",
        "negative_gate": "no_rust_edit_without_review",
    },
    {
        "gap_id": "G-086-SUB-002",
        "gap_class": "substrate_gap",
        "surface_family": "network_info",
        "priority": "high",
        "current_evidence": "navigator.connection defaults and hardcoded shim diverge",
        "next_artifact": "network info substrate review",
        "review_gate": "native_substrate_review",
        "negative_gate": "no_runtime_apply_from_coherence",
    },
    {
        "gap_id": "G-086-PROBE-001",
        "gap_class": "probe_gap",
        "surface_family": "navigator_ua_data",
        "priority": "high",
        "current_evidence": "UAData has shape probe but limited high-entropy coverage",
        "next_artifact": "UAData probe coverage map",
        "review_gate": "probe_pack_review",
        "negative_gate": "no_browser_version_equivalence",
    },
    {
        "gap_id": "G-086-CAND-001",
        "gap_class": "candidate_gap",
        "surface_family": "environment_value",
        "priority": "high",
        "current_evidence": "chrome_generic values need bounded planning metadata",
        "next_artifact": "candidate metadata validation",
        "review_gate": "candidate_schema_review",
        "negative_gate": "no_default_apply_from_metadata",
    },
    {
        "gap_id": "G-086-POL-001",
        "gap_class": "policy_gap",
        "surface_family": "planner",
        "priority": "medium",
        "current_evidence": "apply policy cannot represent review-only planning states",
        "next_artifact": "planner policy state design",
        "review_gate": "policy_review",
        "negative_gate": "no_hidden_authorization",
    },
    {
        "gap_id": "G-086-EVD-001",
        "gap_class": "evidence_gap",
        "surface_family": "diagnostics",
        "priority": "medium",
        "current_evidence": "planning and rollback evidence must stay diagnostic-only",
        "next_artifact": "evidence wording and negative gates",
        "review_gate": "evidence_review",
        "negative_gate": "no_pass_from_plan_or_readiness",
    },
    {
        "gap_id": "G-086-ROLL-001",
        "gap_class": "rollback_gap",
        "surface_family": "runtime_safe",
        "priority": "high",
        "current_evidence": "explicit fresh-context rerun needs rollback prerequisite record",
        "next_artifact": "rollback diagnostics",
        "review_gate": "rollback_review",
        "negative_gate": "no_writes_by_default",
    },
    {
        "gap_id": "G-086-NEG-001",
        "gap_class": "negative_gate_gap",
        "surface_family": "planner_artifact",
        "priority": "high",
        "current_evidence": "planner-like artifacts need boundary validation",
        "next_artifact": "planner boundary negative tests",
        "review_gate": "boundary_review",
        "negative_gate": "reject_target_flow_vocabulary",
    },
)
