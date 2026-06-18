from __future__ import annotations

import pytest
from iv8_rs.environment_toolchain_boundary import (
    validate_bypass_boundary as boundary_validate_bypass_boundary,
)
from iv8_rs.environment_toolchain_runtime import (
    BoundaryDecision,
    ToolchainCandidate,
    validate_bypass_boundary,
)


def runtime_safe_candidate(**overrides) -> ToolchainCandidate:
    data = {
        "patch_id": "navigator.languages.default.v0",
        "target": "navigator.languages",
        "target_family": "environment_value",
        "kind": "value",
        "policy": "runtime_safe",
        "source": "builtin_registry",
        "value_preview": ["en-US", "en"],
        "requires": [],
        "risk_reasons": [],
        "reversible": True,
        "validation": {"probe_pack": "fingerprint.m1"},
    }
    data.update(overrides)
    return ToolchainCandidate.from_dict(data)


def test_boundary_allows_generic_runtime_safe_candidate():
    decision = validate_bypass_boundary(runtime_safe_candidate())

    assert decision.decision == "allowed"
    assert decision.blocked_terms == []


def test_boundary_module_direct_import_matches_runtime_reexport():
    payload = {"target": "navigator.languages", "note": "generic environment value"}

    assert boundary_validate_bypass_boundary(payload).to_dict() == validate_bypass_boundary(
        payload
    ).to_dict()


def test_boundary_module_does_not_import_runtime():
    import iv8_rs.environment_toolchain_boundary as boundary_module

    module_names = {
        value.__name__
        for value in vars(boundary_module).values()
        if getattr(value, "__name__", None)
    }

    assert "iv8_rs.environment_toolchain_runtime" not in module_names


def test_boundary_allows_generic_cookie_enabled_target():
    candidate = runtime_safe_candidate(
        patch_id="navigator.cookieEnabled.default.v0",
        target="navigator.cookieEnabled",
        value_preview=True,
    )

    decision = validate_bypass_boundary(candidate)

    assert decision.decision == "allowed"


def test_boundary_blocks_cookie_value_payload():
    candidate = runtime_safe_candidate(value_preview="cookie=sessionid=abc")
    decision = validate_bypass_boundary(candidate)

    assert decision.decision == "blocked"
    assert "cookie" in decision.blocked_terms


def test_boundary_blocks_token_signature_and_endpoint_payloads():
    payload = {
        "target": "navigator.languages",
        "notes": ["copy token from endpoint", "signature value required"],
    }
    decision = validate_bypass_boundary(payload)

    assert decision.decision == "blocked"
    assert {"endpoint", "signature", "token"}.issubset(set(decision.blocked_terms))


def test_boundary_blocks_ordered_recipe():
    payload = {
        "target": "navigator.languages",
        "note": "apply patch A, request B, copy cookie C, rerun signer D",
    }
    decision = validate_bypass_boundary(payload)

    assert decision.decision == "blocked"
    assert "ordered_recipe" in decision.blocked_terms


def test_boundary_blocks_planner_artifact_target_flow_payload():
    payload = {
        "plan_id": "dry_run.environment.generic",
        "target": "navigator.language",
        "items": [
            {
                "candidate_id": "navigator.language.default.v0",
                "blocked_reason": "copy token from endpoint before signature rerun",
            }
        ],
        "notes": ["authorization header and nonce are target-flow material"],
    }

    decision = validate_bypass_boundary(payload)

    assert decision.decision == "blocked"
    assert {"endpoint", "signature", "token", "authorization", "nonce"}.issubset(
        set(decision.blocked_terms)
    )


def test_boundary_allows_generic_planner_artifact_targets():
    payload = {
        "plan_id": "dry_run.environment.generic",
        "target": "navigator.language",
        "items": [
            {
                "candidate_id": "navigator.language.default.v0",
                "target": "navigator.languages",
                "target_family": "navigator",
                "planning_status": "eligible_for_review",
                "expected_delta": ["navigator.languages"],
            },
            {
                "candidate_id": "screen.width.default.v0",
                "target": "screen.width",
                "target_family": "screen",
                "planning_status": "eligible_for_review",
                "expected_delta": ["screen.width"],
            },
        ],
    }

    decision = validate_bypass_boundary(payload)

    assert decision.decision == "allowed"
    assert decision.blocked_terms == []


def test_boundary_blocks_rollback_artifact_target_flow_payload():
    payload = {
        "record_id": "rollback.navigator.language.v0",
        "target": "navigator.language",
        "scope": "ephemeral_report",
        "capture_before": ["navigator.language"],
        "restore_strategy": "replace_value",
        "blocked_reasons": [
            "rollback cannot include cookie value or request_body payload",
        ],
    }

    decision = validate_bypass_boundary(payload)

    assert decision.decision == "blocked"
    assert {"cookie", "request_body"}.issubset(set(decision.blocked_terms))


def test_boundary_blocks_rollback_artifact_raw_local_path():
    payload = {
        "record_id": "rollback.navigator.language.v0",
        "target": "navigator.language",
        "scope": "ephemeral_report",
        "redacted_ref": "profile.json",
        "source_path": "C:\\Users\\Lenovo\\secret\\profile.json",
    }

    decision = validate_bypass_boundary(payload)

    assert decision.decision == "blocked"
    assert "raw_path" in decision.blocked_terms


def test_boundary_decision_rejects_invalid_state():
    with pytest.raises(ValueError, match="invalid boundary decision"):
        BoundaryDecision(decision="deferred", reason="bad")
