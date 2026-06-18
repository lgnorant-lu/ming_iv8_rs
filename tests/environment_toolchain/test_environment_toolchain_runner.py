from __future__ import annotations

from iv8_rs.environment_toolchain_runtime import (
    ProbeDefinition,
    ProbePack,
    run_probe_pack,
)


def make_pack(*probes: ProbeDefinition) -> ProbePack:
    return ProbePack(
        probe_pack="test.pack",
        version=1,
        description="synthetic test pack",
        evidence_ceiling="diagnostic_only",
        probes=list(probes),
    )


def test_run_probe_pack_executes_fingerprint_m1_in_fresh_context():
    run = run_probe_pack("", probe_pack="fingerprint.m1", profile=None)
    data = run.to_dict()

    assert data["probe_pack"] == "fingerprint.m1"
    assert len(data["observations"]) == 14
    assert set(data["coverage"]) == {"present", "missing", "mismatch"}
    total = data["coverage"]["present"] + data["coverage"]["missing"] + data["coverage"]["mismatch"]
    assert total == 14
    assert any(item["code"] == "ENV_TOOLCHAIN_PROBE_PACK_RUN" for item in data["diagnostics"])


def test_run_probe_pack_classifies_missing_api_gap():
    pack = make_pack(ProbeDefinition(
        probe_id="custom.missing.present",
        target="custom.missing",
        category="presence",
        js="return typeof custom !== 'undefined';",
        expected=True,
        gap_class="missing_api",
    ))

    run = run_probe_pack("", probe_pack=pack, profile=None)

    assert run.coverage == {"present": 0, "missing": 1, "mismatch": 0}
    assert run.gaps[0].gap_class == "missing_api"
    assert run.gaps[0].target == "custom.missing"


def test_run_probe_pack_classifies_descriptor_mismatch_diagnostic():
    pack = make_pack(ProbeDefinition(
        probe_id="navigator.language.descriptor",
        target="navigator.language",
        category="descriptor",
        js="return false;",
        expected=True,
        gap_class="descriptor_mismatch",
    ))

    run = run_probe_pack("", probe_pack=pack, profile=None)

    assert run.coverage == {"present": 0, "missing": 0, "mismatch": 1}
    assert any(
        diagnostic["code"] == "ENV_TOOLCHAIN_DESCRIPTOR_MISMATCH"
        for diagnostic in run.diagnostics
    )


def test_run_probe_pack_evaluates_entry_expr_without_inferring_entry():
    pack = make_pack(ProbeDefinition(
        probe_id="entry.side.effect",
        target="globalThis.entryValue",
        category="value",
        js="return globalThis.entryValue === 7;",
        expected=True,
        gap_class="value_mismatch",
    ))

    run = run_probe_pack(
        "function targetFunction(){ globalThis.entryValue = 7; }",
        probe_pack=pack,
        profile=None,
        entry_expr="targetFunction()",
    )

    assert run.coverage == {"present": 1, "missing": 0, "mismatch": 0}
    assert not run.gaps


def test_run_probe_pack_preserves_entry_expr_failure_as_diagnostic():
    pack = make_pack(ProbeDefinition(
        probe_id="basic.pass",
        target="globalThis",
        category="presence",
        js="return true;",
        expected=True,
        gap_class="missing_api",
    ))

    run = run_probe_pack("", probe_pack=pack, profile=None, entry_expr="missingFunction()")

    assert run.coverage == {"present": 1, "missing": 0, "mismatch": 0}
    assert any(
        diagnostic["code"] == "ENV_TOOLCHAIN_ENTRY_EXPR_FAILED"
        for diagnostic in run.diagnostics
    )
