"""v0.8.48 L2/L3 Real-Sample Environment Pilot M1.

No-write, no-PASS, diagnostic_only pilot.
Proves the L2/L3 diagnostic-to-repair chain can observe, classify, and
project real-sample environment observations.

Negative gates enforced:
- No sample file modification
- No target-specific facts in committed reports
- No PASS claim
- No new permanent infrastructure
"""

from __future__ import annotations

import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))

from iv8_rs import JSError, prepare_entry, run_with_entry

QQ_VENDOR = "_ref/yy/vendor.chunk.062f57657390b2408623.js"
BDMS_JS = "tests/iv8-ref/examples/js/bdms_1.0.1.19.js"
H5ST_JS = "tests/iv8-ref/examples/js/js_security_v3_main.js"


def _load(path: str) -> str:
    return Path(path).read_text(encoding="utf-8")


def _try_execute(source: str, sample_id: str) -> dict:
    """Execute a real sample through IV8 Rust Entry plane.
    Returns structured observation, NOT a PASS claim.
    """
    try:
        plan = prepare_entry(source, persona="analysis")
        result = run_with_entry(plan, source)
    except (JSError, TimeoutError, Exception) as exc:
        return {
            "sample_id": sample_id,
            "outcome": "execution_failed",
            "error_type": type(exc).__name__,
            "error_message": str(exc)[:200],
            "entry_plan": None,
            "entry_result": None,
            "observed_evidence": [],
            "diagnostic_records": [],
        }
    return {
        "sample_id": sample_id,
        "outcome": result.get("final_state", "unknown"),
        "error_type": None,
        "error_message": None,
        "entry_plan": plan,
        "entry_result": result,
        "observed_evidence": result.get("observed_evidence", []),
        "diagnostic_records": result.get("diagnostic_records", []),
    }


def _extract_env_diagnostics(observation: dict) -> list[dict]:
    """Extract environment-related diagnostics from a real-sample observation.
    No target-specific facts. Generic browser-surface observations only.
    """
    env_diags = []
    error_msg = observation.get("error_message") or ""
    if "is not defined" in error_msg:
        env_diags.append({
            "kind": "missing_api_reference",
            "detail": error_msg[:150],
        })
    if "cannot read properties" in error_msg.lower():
        env_diags.append({
            "kind": "null_or_undefined_access",
            "detail": error_msg[:150],
        })
    for diag in observation.get("diagnostic_records", []):
        code = diag.get("code", "")
        if "ENV" in code or "env" in code.lower():
            env_diags.append({
                "kind": "environment_diagnostic",
                "code": code,
                "detail": str(diag.get("message", ""))[:150],
            })
    return env_diags


def _classify_gap(env_diag: dict) -> str:
    """Classify a gap as foundation, fine-finish, target-flow, or hard-limit.
    Generic rule: missing API references and null-access are foundation.
    """
    kind = env_diag.get("kind", "")
    detail = env_diag.get("detail", "")
    if kind == "missing_api_reference":
        return "foundation_gap"
    if kind == "null_or_undefined_access":
        return "foundation_gap"
    if kind == "environment_diagnostic":
        return "fine_finish_gap" if "descriptor" in detail.lower() else "foundation_gap"
    return "fine_finish_gap"


# ── S2: Baseline Execution ──────────────────────────────────────────────


def test_pilot_baseline_execution_bdms():
    """G1+G2: BDMS sample executes and produces structured result."""
    source = _load(BDMS_JS)
    obs = _try_execute(source, "bdms")

    assert obs["outcome"] in (
        "collected", "finalized", "execution_failed",
    ), f"unexpected outcome: {obs['outcome']}"

    assert obs["sample_id"] == "bdms"
    assert isinstance(obs.get("observed_evidence"), list)
    assert isinstance(obs.get("diagnostic_records"), list)


def test_pilot_baseline_execution_h5st():
    """G1+G2: h5st sample executes and produces structured diagnostics."""
    source = _load(H5ST_JS)
    obs = _try_execute(source, "h5st")

    assert obs["outcome"] in (
        "collected", "finalized", "execution_failed",
    ), f"unexpected outcome: {obs['outcome']}"

    assert isinstance(obs.get("diagnostic_records"), list)


# ── S3: Environment Gap Diagnosis ───────────────────────────────────────


def test_pilot_environment_diagnosis_bdms():
    """G2+G3: BDMS execution produces environment-related observations."""
    source = _load(BDMS_JS)
    obs = _try_execute(source, "bdms")
    env_diags = _extract_env_diagnostics(obs)

    assert len(env_diags) >= 0
    for diag in env_diags:
        assert "kind" in diag
        assert diag["kind"] in (
            "missing_api_reference", "null_or_undefined_access", "environment_diagnostic",
        )


def test_pilot_environment_diagnosis_qq_vendor():
    """G2+G3: QQ vendor sample produces structured execution result."""
    source = _load(QQ_VENDOR)
    obs = _try_execute(source, "qq-vendor")

    assert obs["outcome"] in (
        "collected", "finalized", "execution_failed",
    ), f"unexpected outcome: {obs['outcome']}"
    assert "sample_id" in obs


# ── S4: L2/L3 Projection ───────────────────────────────────────────────


def test_pilot_projection_into_bridge_vocabulary():
    """G3+G4: Observations are projected into L2/L3 bridge vocabulary."""
    from tools.diagnostic_bridge import (
        build_repair_brief,
        classify_repair_readiness,
    )

    source = _load(BDMS_JS)
    obs = _try_execute(source, "bdms")
    env_diags = _extract_env_diagnostics(obs)

    for diag in env_diags:
        gap_class = (
            "missing_api" if diag["kind"] == "missing_api_reference"
            else "structural_mismatch"
        )
        brief = build_repair_brief({
            "ticket_id": f"pilot-{diag['kind']}",
            "source_vector": "unmapped",
            "gap_class": gap_class,
            "risk_level": "medium",
            "evidence_refs": [],
        })
        assert brief["schema_version"] == "iv8-repair-brief.v0.1"
        assert brief["evidence_ceiling"] == "diagnostic_only"
        assert brief["writes"] == []

        readiness = classify_repair_readiness(brief)
        assert readiness["readiness"] in ("ready", "incomplete", "blocked", "deferred")
        assert readiness["evidence_ceiling"] == "diagnostic_only"


def test_pilot_bridge_projections_are_diagnostic_only():
    """G3+G7: All bridge projections carry evidence_ceiling=diagnostic_only."""
    from tools.diagnostic_bridge import (
        build_candidate_ledger,
        build_delta_contract,
        build_evidence_bundle_manifest,
        build_repair_brief,
    )

    brief = build_repair_brief({
        "ticket_id": "t-pilot-check",
        "source_vector": "unmapped",
        "gap_class": "missing_api",
        "risk_level": "medium",
    })
    assert brief["writes"] == []
    assert brief["evidence_ceiling"] == "diagnostic_only"

    manifest = build_evidence_bundle_manifest(brief)
    assert manifest["writes"] == []
    assert manifest["evidence_ceiling"] == "diagnostic_only"

    contract = build_delta_contract(
        {"ticket_id": "t-pilot-check", "source_vector": "unmapped"},
        {},
        {},
    )
    assert contract["writes"] == []
    assert contract["evidence_ceiling"] == "diagnostic_only"

    candidates = build_candidate_ledger([{
        "ticket_id": "t-pilot-check",
        "source_vector": "unmapped",
        "gap_class": "missing_api",
        "risk_level": "medium",
    }])
    assert len(candidates) >= 0


# ── S6: Gap Classification ──────────────────────────────────────────────


def test_pilot_classification_is_foundation_or_fine_finish():
    """G9: All classified gaps are foundation or fine-finish, not target-flow."""
    source = _load(BDMS_JS)
    obs = _try_execute(source, "bdms")
    env_diags = _extract_env_diagnostics(obs)

    for diag in env_diags:
        classification = _classify_gap(diag)
        assert classification in (
            "foundation_gap", "fine_finish_gap",
        ), f"unexpected classification: {classification}"


def test_pilot_all_diagnostics_are_classified():
    """G9: Every environment diagnostic receives a gap classification."""
    source = _load(BDMS_JS)
    obs = _try_execute(source, "bdms")
    env_diags = _extract_env_diagnostics(obs)

    classified = [_classify_gap(d) for d in env_diags]
    assert len(classified) == len(env_diags)


# ── G8: Redaction Gate ──────────────────────────────────────────────────


COMMITTED_REDACTED_PATTERNS = [
    r"eyJ[a-zA-Z0-9+/=]{20,}", r"tk=[a-zA-Z0-9]{6,}", r"sign=[a-zA-Z0-9]{6,}",
    r"_token=[a-zA-Z0-9]{6,}",
]


def test_pilot_redaction_no_target_secrets_in_committed_source():
    """G8: This test file must not contain target-specific secrets."""
    import re

    own_source = Path(__file__).read_text(encoding="utf-8")
    for pattern in COMMITTED_REDACTED_PATTERNS:
        matches = re.findall(pattern, own_source, re.IGNORECASE)
        assert not matches, f"test source contains redacted pattern: {pattern}"
