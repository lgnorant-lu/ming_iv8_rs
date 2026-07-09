"""Behavioral tests for iv8_rs.environment_policy — patch decision engine."""
import pytest

iv8_rs = pytest.importorskip("iv8_rs")


def test_runtime_safe_candidate_applied_by_default():
    from iv8_rs.environment_policy import decide_patch_policy, runtime_safe_candidate
    cand = runtime_safe_candidate("nav.lang", "navigator.language", "en-US")
    decision = decide_patch_policy(cand)
    assert decision.decision == "applied"


def test_candidate_rejected_when_explicit_env_conflicts():
    from iv8_rs.environment_policy import (
        EnvironmentPatchCandidate,
        PatchPolicyOptions,
        decide_patch_policy,
    )
    cand = EnvironmentPatchCandidate(
        patch_id="test", target="navigator.language",
        kind="value", policy="runtime_safe",
    )
    opts = PatchPolicyOptions(explicit_environment={"navigator.language": "fr-FR"})
    decision = decide_patch_policy(cand, options=opts)
    assert decision.decision == "rejected"


def test_analysis_only_rejected_without_opt_in():
    from iv8_rs.environment_policy import EnvironmentPatchCandidate, decide_patch_policy
    cand = EnvironmentPatchCandidate(
        patch_id="eval.capture", target="eval",
        kind="capture", policy="analysis_only",
    )
    decision = decide_patch_policy(cand)
    assert decision.decision == "rejected"


def test_unsafe_hook_blocked_by_default():
    from iv8_rs.environment_policy import EnvironmentPatchCandidate, decide_patch_policy
    cand = EnvironmentPatchCandidate(
        patch_id="anti.debug", target="antiDebug",
        kind="hook", policy="unsafe_hook",
    )
    decision = decide_patch_policy(cand)
    assert decision.decision == "blocked"


def test_block_mutation_returns_blocked():
    from iv8_rs.environment_policy import block_mutation
    decision = block_mutation("profile", reason="auto-write not allowed")
    assert decision.decision == "blocked"


def test_patch_candidate_from_dict():
    from iv8_rs.environment_policy import EnvironmentPatchCandidate
    d = {"patch_id": "test", "target": "navigator.userAgent", "kind": "value", "policy": "runtime_safe"}
    c = EnvironmentPatchCandidate.from_dict(d)
    assert c.patch_id == "test"
    assert c.policy == "runtime_safe"
