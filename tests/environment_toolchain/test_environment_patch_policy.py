from __future__ import annotations

import pytest
from iv8_rs import (
    EnvironmentPatchCandidate,
    PatchPolicyOptions,
    block_mutation,
    decide_patch_policy,
    runtime_safe_candidate,
)


def test_runtime_safe_value_patch_is_applied_by_default():
    candidate = runtime_safe_candidate(
        "navigator.language.default",
        "navigator.language",
        "en-US",
    )
    decision = decide_patch_policy(candidate)

    assert decision.decision == "applied"
    assert decision.effective_policy == "runtime_safe"
    assert decision.requires_opt_in is False
    assert decision.diagnostic_code == "PATCH_POLICY_APPLIED"


def test_explicit_environment_conflict_rejects_builtin_candidate():
    candidate = runtime_safe_candidate(
        "navigator.language.default",
        "navigator.language",
        "en-US",
    )
    decision = decide_patch_policy(
        candidate,
        options=PatchPolicyOptions(explicit_environment={"navigator.language": "fr-FR"}),
    )

    assert decision.decision == "rejected"
    assert decision.diagnostic_code == "PATCH_POLICY_CONFLICT"
    assert decision.conflicts == ["explicit_environment"]


def test_analysis_only_patch_requires_opt_in_and_analysis_persona():
    candidate = EnvironmentPatchCandidate(
        patch_id="eval.capture",
        target="eval",
        kind="capture",
        policy="analysis_only",
    )

    rejected = decide_patch_policy(candidate)
    assert rejected.decision == "rejected"
    assert rejected.requires_opt_in is True

    runtime_rejected = decide_patch_policy(
        candidate,
        options=PatchPolicyOptions(persona="runtime", allow_analysis_only=True),
    )
    assert runtime_rejected.decision == "rejected"
    assert "runtime persona" in runtime_rejected.reason

    applied = decide_patch_policy(
        candidate,
        options=PatchPolicyOptions(persona="analysis", allow_analysis_only=True),
    )
    assert applied.decision == "applied"
    assert applied.effective_policy == "analysis_only"
    assert applied.opt_in_present is True


def test_unsafe_hook_is_blocked_without_opt_in():
    candidate = EnvironmentPatchCandidate(
        patch_id="anti_debug.override",
        target="antiDebug",
        kind="hook",
        policy="unsafe_hook",
        risk_reasons=["changes protected control flow"],
    )

    blocked = decide_patch_policy(candidate)
    assert blocked.decision == "blocked"
    assert blocked.diagnostic_code == "PATCH_POLICY_BLOCKED"

    applied = decide_patch_policy(
        candidate,
        options=PatchPolicyOptions(persona="analysis", allow_unsafe_hook=True),
    )
    assert applied.decision == "applied"
    assert applied.effective_policy == "unsafe_hook"


def test_patch_kind_reclassifies_to_minimum_safe_policy():
    candidate = EnvironmentPatchCandidate(
        patch_id="require.wrap",
        target="__webpack_require__",
        kind="wrapper",
        policy="runtime_safe",
    )

    decision = decide_patch_policy(candidate)
    assert decision.effective_policy == "analysis_only"
    assert decision.decision == "rejected"


def test_mutation_targets_are_blocked():
    decision = block_mutation("manifest")
    assert decision.decision == "blocked"
    assert decision.diagnostic_code == "PATCH_POLICY_MUTATION_BLOCKED"
    assert "manifest" in decision.reason

    with pytest.raises(ValueError, match="unknown mutation target"):
        block_mutation("cache")


def test_candidate_and_decision_roundtrip_to_dict():
    candidate = EnvironmentPatchCandidate(
        patch_id="screen.width.default",
        target="screen.width",
        kind="value",
        policy="runtime_safe",
        value_preview=1920,
    )
    rebuilt = EnvironmentPatchCandidate.from_dict(candidate.to_dict())
    assert rebuilt == candidate

    decision = decide_patch_policy(rebuilt)
    data = decision.to_dict()
    assert data["patch_id"] == "screen.width.default"
    assert data["decision"] == "applied"


def test_analysis_only_with_runtime_persona_and_explicit_override():
    candidate = EnvironmentPatchCandidate(
        patch_id="eval.capture",
        target="eval",
        kind="capture",
        policy="analysis_only",
    )
    applied = decide_patch_policy(
        candidate,
        options=PatchPolicyOptions(
            persona="runtime",
            allow_analysis_only=True,
            allow_explicit_override=True,
        ),
    )
    assert applied.decision == "applied"
    assert applied.effective_policy == "analysis_only"


def test_wrapper_kind_reclassification_emits_reclassified_diagnostic():
    candidate = EnvironmentPatchCandidate(
        patch_id="require.wrap",
        target="__webpack_require__",
        kind="wrapper",
        policy="runtime_safe",
    )
    applied = decide_patch_policy(
        candidate,
        options=PatchPolicyOptions(persona="analysis", allow_analysis_only=True),
    )
    assert applied.diagnostic_code == "PATCH_POLICY_RECLASSIFIED"
    assert applied.effective_policy == "analysis_only"
    assert applied.decision == "applied"


def test_missing_opt_in_emits_opt_in_missing_diagnostic():
    candidate = EnvironmentPatchCandidate(
        patch_id="eval.capture",
        target="eval",
        kind="capture",
        policy="analysis_only",
    )
    decision = decide_patch_policy(candidate)
    assert decision.diagnostic_code == "PATCH_POLICY_OPT_IN_MISSING"
    assert decision.requires_opt_in is True


def test_runtime_persona_rejects_analysis_only_with_persona_mismatch():
    candidate = EnvironmentPatchCandidate(
        patch_id="eval.capture",
        target="eval",
        kind="capture",
        policy="analysis_only",
    )
    decision = decide_patch_policy(
        candidate,
        options=PatchPolicyOptions(persona="runtime", allow_analysis_only=True),
    )
    assert decision.diagnostic_code == "PATCH_POLICY_PERSONA_MISMATCH"
    assert decision.decision == "rejected"


def test_two_builtin_candidates_same_target_both_applied_with_profile_opt_out():
    candidate_a = EnvironmentPatchCandidate(
        patch_id="navigator.language.a", target="navigator.language",
        kind="value", policy="runtime_safe", source="builtin_registry",
    )
    candidate_b = EnvironmentPatchCandidate(
        patch_id="navigator.language.b", target="navigator.language",
        kind="value", policy="runtime_safe", source="builtin_registry",
    )
    dec_a = decide_patch_policy(candidate_a)
    dec_b = decide_patch_policy(candidate_b)
    assert dec_a.decision == "applied"
    assert dec_b.decision == "applied"


def test_persona_runtime_default_policy_is_runtime_safe():
    candidate = EnvironmentPatchCandidate(
        patch_id="eval.capture", target="eval",
        kind="capture", policy="analysis_only",
    )
    decision = decide_patch_policy(
        candidate,
        options=PatchPolicyOptions(persona="runtime"),
    )
    assert decision.decision in {"rejected", "blocked"}
    assert decision.decision != "applied"


def test_invalid_candidate_fields_fail_clearly():
    with pytest.raises(ValueError, match="invalid patch kind"):
        EnvironmentPatchCandidate("bad", "x", "side_effect", "runtime_safe")

    with pytest.raises(ValueError, match="invalid patch policy"):
        EnvironmentPatchCandidate("bad", "x", "value", "safeish")
