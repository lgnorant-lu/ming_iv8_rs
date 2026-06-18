from __future__ import annotations

from iv8_rs.environment_toolchain import toolchain_report_to_dict
from iv8_rs.environment_toolchain_runtime import run_environment_toolchain


def diagnostics(data: dict, code: str) -> list[dict]:
    return [diagnostic for diagnostic in data["diagnostics"] if diagnostic["code"] == code]


def native_review(data: dict) -> dict:
    records = diagnostics(data, "ENV_TOOLCHAIN_NATIVE_SUBSTRATE_REVIEW")
    assert len(records) == 1
    return records[0]


def custom_probe_pack(probes: list[dict]) -> dict:
    return {
        "probe_pack": "custom.native_review",
        "version": 1,
        "description": "generic native substrate review test pack",
        "evidence_ceiling": "diagnostic_only",
        "probes": probes,
    }


def probe(
    *,
    probe_id: str,
    target: str,
    category: str,
    gap_class: str,
    js: str = "return false;",
    expected: bool = True,
) -> dict:
    return {
        "probe_id": probe_id,
        "target": target,
        "category": category,
        "js": js,
        "expected": expected,
        "gap_class": gap_class,
        "side_effects": [],
        "cleanup": "none",
        "evidence_ceiling": "diagnostic_only",
    }


def test_native_review_summary_has_no_candidates_for_clean_generic_run():
    report = run_environment_toolchain(
        "",
        profile=None,
        candidate_pack=None,
        probe_pack=custom_probe_pack([
            probe(
                probe_id="navigator.language.present",
                target="navigator.language",
                category="presence",
                gap_class="missing_api",
                js="return true;",
            )
        ]),
    )
    data = toolchain_report_to_dict(report)
    review = native_review(data)

    assert review["severity"] == "info"
    assert review["details"]["candidate_areas"] == []
    assert review["details"]["review_status"] == "review_only"
    assert review["details"]["evidence_ceiling"] == "diagnostic_only"


def test_native_review_flags_descriptor_prototype_pressure():
    report = run_environment_toolchain(
        "",
        profile=None,
        candidate_pack=None,
        probe_pack=custom_probe_pack([
            probe(
                probe_id="navigator.userAgent.descriptor",
                target="navigator.userAgent",
                category="descriptor",
                gap_class="descriptor_mismatch",
            )
        ]),
    )
    data = toolchain_report_to_dict(report)
    review = native_review(data)

    assert review["severity"] == "warn"
    assert review["details"]["candidate_areas"] == ["descriptor_prototype"]
    assert review["details"]["review_status"] == "requires_review"
    assert data["writes"] == []
    assert data["applied_patches"] == []


def test_native_review_flags_ua_platform_contradiction_without_apply():
    report = run_environment_toolchain(
        "",
        profile=None,
        candidate_pack=None,
        environment={
            "navigator.userAgent": "Mozilla/5.0 (X11; Linux x86_64) Chrome/120.0.0.0",
            "navigator.platform": "Win32",
            "navigator.userAgentData.platform": "Windows",
            "navigator.userAgentData.mobile": False,
        },
    )
    data = toolchain_report_to_dict(report)
    review = native_review(data)

    assert review["details"]["candidate_areas"] == ["navigator_ua_data"]
    assert "rust_native_hardening_without_review" in review["details"]["blocked_actions"]
    assert data["writes"] == []
    assert data["applied_patches"] == []


def test_native_review_flags_timezone_locale_contradiction_without_apply():
    report = run_environment_toolchain(
        "",
        profile=None,
        candidate_pack=None,
        environment={
            "config.timezone": "Asia/Shanghai",
            "timezone": "UTC",
            "navigator.language": "zh-CN",
            "navigator.languages": ["zh-CN", "zh"],
        },
    )
    data = toolchain_report_to_dict(report)
    review = native_review(data)

    assert review["details"]["candidate_areas"] == ["timezone_intl"]
    assert review["details"]["review_status"] == "requires_review"
    assert data["writes"] == []
    assert data["applied_patches"] == []
