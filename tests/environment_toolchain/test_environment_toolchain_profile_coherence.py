from __future__ import annotations

from iv8_rs.environment_toolchain import toolchain_report_to_dict
from iv8_rs.environment_toolchain_runtime import run_environment_toolchain


def diagnostics(data: dict, code: str) -> list[dict]:
    return [diagnostic for diagnostic in data["diagnostics"] if diagnostic["code"] == code]


def group(data: dict, group_id: str) -> dict:
    return next(
        diagnostic["details"]
        for diagnostic in diagnostics(data, "ENV_TOOLCHAIN_PROFILE_COHERENCE_GROUP")
        if diagnostic["details"]["group_id"] == group_id
    )


def test_profile_coherence_language_consistent_reports_group():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "navigator.language": "en-US",
            "navigator.languages": ["en-US", "en"],
        },
    )
    data = toolchain_report_to_dict(report)

    details = group(data, "language")

    assert details["status"] == "consistent"
    assert details["review_status"] == "review_only"
    assert details["evidence_ceiling"] == "diagnostic_only"
    assert data["writes"] == []


def test_profile_coherence_language_inconsistent_warns():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "navigator.language": "fr-FR",
            "navigator.languages": ["en-US", "en"],
        },
    )
    data = toolchain_report_to_dict(report)

    language_records = [
        diagnostic
        for diagnostic in diagnostics(data, "ENV_TOOLCHAIN_PROFILE_COHERENCE_GROUP")
        if diagnostic["details"]["group_id"] == "language"
    ]

    assert language_records[0]["severity"] == "warn"
    assert language_records[0]["details"]["status"] == "inconsistent"


def test_profile_coherence_language_missing_is_unknown():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={"navigator.language": "en-US"},
    )
    data = toolchain_report_to_dict(report)

    assert group(data, "language")["status"] == "unknown"


def test_profile_coherence_screen_window_consistent_reports_group():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "screen.width": 1920,
            "screen.height": 1080,
            "screen.availWidth": 1920,
            "screen.availHeight": 1040,
            "window.innerWidth": 1280,
            "window.innerHeight": 720,
            "window.devicePixelRatio": 1,
        },
    )
    data = toolchain_report_to_dict(report)

    assert group(data, "screen_window")["status"] == "consistent"


def test_profile_coherence_screen_window_impossible_values_warn():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "screen.width": 1920,
            "screen.height": 1080,
            "screen.availWidth": 2560,
            "screen.availHeight": 1040,
            "window.innerWidth": 1280,
            "window.innerHeight": 720,
            "window.devicePixelRatio": 1,
        },
    )
    data = toolchain_report_to_dict(report)

    screen_records = [
        diagnostic
        for diagnostic in diagnostics(data, "ENV_TOOLCHAIN_PROFILE_COHERENCE_GROUP")
        if diagnostic["details"]["group_id"] == "screen_window"
    ]

    assert screen_records[0]["severity"] == "warn"
    assert screen_records[0]["details"]["status"] == "inconsistent"


def test_profile_coherence_does_not_write_or_apply_patches():
    report = run_environment_toolchain(
        "",
        profile=None,
        candidate_pack=None,
        environment={
            "navigator.language": "fr-FR",
            "navigator.languages": ["en-US", "en"],
        },
    )
    data = toolchain_report_to_dict(report)

    assert data["schema_version"] == "environment-toolchain.v0.1"
    assert data["writes"] == []
    assert data["applied_patches"] == []
    assert any(
        item["kind"] == "environment_profile_coherence_analyzed"
        and item["strength"] == "diagnostic_only"
        for item in data["evidence"]
    )


def test_profile_coherence_summary_counts_groups():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "navigator.language": "en-US",
            "navigator.languages": ["en-US", "en"],
            "screen.width": 1920,
            "screen.height": 1080,
            "screen.availWidth": 1920,
            "screen.availHeight": 1040,
            "window.innerWidth": 1280,
            "window.innerHeight": 720,
            "window.devicePixelRatio": 1,
        },
    )
    data = toolchain_report_to_dict(report)
    summary = diagnostics(data, "ENV_TOOLCHAIN_PROFILE_COHERENCE_SUMMARY")[0]["details"]

    assert summary == {
        "enabled": True,
        "groups": 5,
        "consistent": 5,
        "inconsistent": 0,
        "unknown": 0,
        "review_status": "review_only",
        "evidence_ceiling": "diagnostic_only",
    }


def test_profile_coherence_ua_platform_consistent_reports_group():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "navigator.userAgent": (
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 "
                "(KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"
            ),
            "navigator.platform": "Win32",
            "navigator.userAgentData.platform": "Windows",
            "navigator.userAgentData.mobile": False,
        },
    )
    data = toolchain_report_to_dict(report)

    details = group(data, "ua_platform")
    assert details["status"] == "consistent"
    assert details["review_status"] == "review_only"
    assert details["evidence_ceiling"] == "diagnostic_only"


def test_profile_coherence_ua_platform_platform_contradiction_warns():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "navigator.userAgent": "Mozilla/5.0 (X11; Linux x86_64) Chrome/120.0.0.0",
            "navigator.platform": "Win32",
            "navigator.userAgentData.platform": "Windows",
            "navigator.userAgentData.mobile": False,
        },
    )
    data = toolchain_report_to_dict(report)
    records = [
        diagnostic
        for diagnostic in diagnostics(data, "ENV_TOOLCHAIN_PROFILE_COHERENCE_GROUP")
        if diagnostic["details"]["group_id"] == "ua_platform"
    ]

    assert records[0]["severity"] == "warn"
    assert records[0]["details"]["status"] == "inconsistent"


def test_profile_coherence_ua_platform_mobile_contradiction_warns():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "navigator.userAgent": (
                "Mozilla/5.0 (iPhone; CPU iPhone OS 17_0 like Mac OS X) "
                "Mobile/15E148"
            ),
            "navigator.platform": "iPhone",
            "navigator.userAgentData.platform": "iOS",
            "navigator.userAgentData.mobile": False,
        },
    )
    data = toolchain_report_to_dict(report)

    assert group(data, "ua_platform")["status"] == "inconsistent"


def test_profile_coherence_ua_platform_missing_is_unknown():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "navigator.userAgent": None,
            "navigator.platform": "Win32",
        },
    )
    data = toolchain_report_to_dict(report)

    assert group(data, "ua_platform")["status"] == "unknown"


def test_profile_coherence_network_info_consistent_reports_group():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "navigator.connection.effectiveType": "4g",
            "navigator.connection.downlink": 10,
            "navigator.connection.rtt": 50,
            "navigator.connection.saveData": False,
            "navigator.connection.type": "wifi",
        },
    )
    data = toolchain_report_to_dict(report)

    details = group(data, "network_info")
    assert details["status"] == "consistent"
    assert details["review_status"] == "review_only"
    assert details["evidence_ceiling"] == "diagnostic_only"


def test_profile_coherence_network_info_negative_values_warn():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "navigator.connection.effectiveType": "4g",
            "navigator.connection.downlink": -1,
            "navigator.connection.rtt": 50,
            "navigator.connection.saveData": False,
            "navigator.connection.type": "wifi",
        },
    )
    data = toolchain_report_to_dict(report)
    records = [
        diagnostic
        for diagnostic in diagnostics(data, "ENV_TOOLCHAIN_PROFILE_COHERENCE_GROUP")
        if diagnostic["details"]["group_id"] == "network_info"
    ]

    assert records[0]["severity"] == "warn"
    assert records[0]["details"]["status"] == "inconsistent"


def test_profile_coherence_network_info_malformed_is_unknown():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "navigator.connection.effectiveType": "4g",
            "navigator.connection.saveData": "false",
        },
    )
    data = toolchain_report_to_dict(report)

    assert group(data, "network_info")["status"] == "unknown"


def test_profile_coherence_timezone_locale_consistent_reports_group():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "config.timezone": "Asia/Shanghai",
            "timezone": "Asia/Shanghai",
            "navigator.language": "zh-CN",
            "navigator.languages": ["zh-CN", "zh"],
        },
    )
    data = toolchain_report_to_dict(report)

    details = group(data, "timezone_locale")
    assert details["status"] == "consistent"
    assert details["review_status"] == "review_only"
    assert details["evidence_ceiling"] == "diagnostic_only"


def test_profile_coherence_timezone_locale_timezone_contradiction_warns():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "config.timezone": "Asia/Shanghai",
            "timezone": "UTC",
            "navigator.language": "zh-CN",
            "navigator.languages": ["zh-CN", "zh"],
        },
    )
    data = toolchain_report_to_dict(report)
    records = [
        diagnostic
        for diagnostic in diagnostics(data, "ENV_TOOLCHAIN_PROFILE_COHERENCE_GROUP")
        if diagnostic["details"]["group_id"] == "timezone_locale"
    ]

    assert records[0]["severity"] == "warn"
    assert records[0]["details"]["status"] == "inconsistent"


def test_profile_coherence_timezone_locale_malformed_is_unknown():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "config.timezone": "",
            "navigator.language": "zh-CN",
            "navigator.languages": ["zh-CN", "zh"],
        },
    )
    data = toolchain_report_to_dict(report)

    assert group(data, "timezone_locale")["status"] == "unknown"


def test_local_overlay_dict_provenance():
    report = run_environment_toolchain(
        "",
        profile=None,
        local_overlay={
            "navigator.language": "en-GB",
            "navigator.languages": ["en-GB", "en"],
        },
    )
    data = toolchain_report_to_dict(report)
    prov = diagnostics(data, "ENV_TOOLCHAIN_LOCAL_OVERLAY_PROVENANCE")

    assert len(prov) == 1
    assert prov[0]["severity"] == "info"
    assert prov[0]["details"]["asset_type"] == "local_overlay"
    assert prov[0]["details"]["origin"] == "custom_dict"
    assert prov[0]["details"]["key_count"] == 2
    assert data["writes"] == []


def test_local_overlay_path_redacted(tmp_path):
    import json

    overlay_path = tmp_path / "my-overlay.json"
    overlay_path.write_text(
        json.dumps({"navigator.language": "de-DE"}),
        encoding="utf-8",
    )
    report = run_environment_toolchain(
        "",
        profile=None,
        local_overlay=overlay_path,
    )
    data = toolchain_report_to_dict(report)
    prov = diagnostics(data, "ENV_TOOLCHAIN_LOCAL_OVERLAY_PROVENANCE")

    assert len(prov) == 1
    assert prov[0]["details"]["origin"] == "custom_path"
    assert prov[0]["details"]["redacted_ref"] == "my-overlay.json"
    assert str(tmp_path) not in repr(data)
    assert data["writes"] == []


def test_local_overlay_blocked_vocabulary():
    report = run_environment_toolchain(
        "",
        profile=None,
        local_overlay={"endpoint": "/api/login"},
    )
    data = toolchain_report_to_dict(report)
    rej = diagnostics(data, "ENV_TOOLCHAIN_LOCAL_OVERLAY_REJECTED")

    assert len(rej) == 1
    assert rej[0]["severity"] == "warn"
    assert rej[0]["details"]["reason"] == "local overlay contains non-generic keys"


def test_local_overlay_values_visible_in_coherence():
    report = run_environment_toolchain(
        "",
        profile=None,
        environment={
            "navigator.language": "en-US",
            "navigator.languages": ["en-US", "en"],
        },
        local_overlay={
            "screen.width": 2560,
            "screen.height": 1440,
            "screen.availWidth": 2560,
            "screen.availHeight": 1400,
            "window.innerWidth": 1920,
            "window.innerHeight": 1080,
            "window.devicePixelRatio": 1,
        },
    )
    data = toolchain_report_to_dict(report)

    screen = group(data, "screen_window")
    assert screen["status"] == "consistent"
    assert screen["fields"].get("screen.width") == 2560
    assert data["writes"] == []


def test_local_overlay_does_not_create_patches():
    report = run_environment_toolchain(
        "",
        profile=None,
        candidate_pack=None,
        environment={
            "navigator.language": "fr-FR",
            "navigator.languages": ["en-US", "en"],
        },
        local_overlay={"navigator.language": "de-DE"},
    )
    data = toolchain_report_to_dict(report)

    assert data["applied_patches"] == []
    assert data["writes"] == []
