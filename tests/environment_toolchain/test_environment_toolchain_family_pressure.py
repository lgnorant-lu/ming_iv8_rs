from __future__ import annotations

from iv8_rs.environment_toolchain import toolchain_report_to_dict
from iv8_rs.environment_toolchain_runtime import run_environment_toolchain


def diagnostics(data: dict, code: str) -> list[dict]:
    return [diagnostic for diagnostic in data["diagnostics"] if diagnostic["code"] == code]


def custom_probe_pack(probes: list[dict]) -> dict:
    return {
        "probe_pack": "custom.family_pressure",
        "version": 1,
        "description": "generic family pressure taxonomy test pack",
        "evidence_ceiling": "diagnostic_only",
        "probes": probes,
    }


def gap_probe(
    *,
    probe_id: str,
    target: str,
    category: str,
    gap_class: str,
) -> dict:
    return {
        "probe_id": probe_id,
        "target": target,
        "category": category,
        "js": "return false;",
        "expected": True,
        "gap_class": gap_class,
        "side_effects": [],
        "cleanup": "none",
        "evidence_ceiling": "diagnostic_only",
    }


def family_pressure_summary(data: dict) -> dict:
    records = diagnostics(data, "ENV_TOOLCHAIN_FAMILY_PRESSURE_SUMMARY")
    assert len(records) == 1
    return records[0]["details"]


def test_family_pressure_summary_groups_generic_gaps():
    report = run_environment_toolchain(
        "",
        profile=None,
        candidate_pack=None,
        probe_pack=custom_probe_pack([
            gap_probe(
                probe_id="navigator.language.value",
                target="navigator.language",
                category="value",
                gap_class="value_mismatch",
            ),
            gap_probe(
                probe_id="screen.width.descriptor",
                target="screen.width",
                category="descriptor",
                gap_class="descriptor_mismatch",
            ),
            gap_probe(
                probe_id="navigator.prototype",
                target="navigator",
                category="descriptor",
                gap_class="prototype_chain_mismatch",
            ),
        ]),
    )
    data = toolchain_report_to_dict(report)
    summary = family_pressure_summary(data)

    assert summary["enabled"] is True
    assert summary["pressures"] == 3
    assert summary["category_counts"]["value_mismatch"] == 1
    assert summary["category_counts"]["descriptor_mismatch"] == 1
    assert summary["category_counts"]["prototype_mismatch"] == 1
    assert summary["family_counts"]["navigator"] == 2
    assert summary["family_counts"]["screen"] == 1
    assert summary["review_status"] == "review_only"
    assert summary["evidence_ceiling"] == "diagnostic_only"
    assert data["writes"] == []
    assert data["applied_patches"] == []


def test_family_pressure_entries_use_allowed_fields_only():
    report = run_environment_toolchain(
        "",
        profile=None,
        candidate_pack=None,
        probe_pack=custom_probe_pack([
            gap_probe(
                probe_id="storage.behavior",
                target="localStorage",
                category="behavior",
                gap_class="behavior_mismatch",
            )
        ]),
    )
    data = toolchain_report_to_dict(report)
    entries = family_pressure_summary(data)["entries"]

    assert entries == [
        {
            "pressure_id": "behavior_mismatch__window",
            "category": "behavior_mismatch",
            "target_family": "window",
            "gap_classes": ["behavior_mismatch"],
            "review_status": "review_only",
            "evidence_ceiling": "diagnostic_only",
        }
    ]


def test_family_pressure_maps_timing_and_network_info_families():
    report = run_environment_toolchain(
        "",
        profile=None,
        candidate_pack=None,
        probe_pack=custom_probe_pack([
            gap_probe(
                probe_id="performance.now.behavior",
                target="performance.now",
                category="behavior",
                gap_class="behavior_mismatch",
            ),
            gap_probe(
                probe_id="navigator.connection.missing",
                target="navigator.connection.effectiveType",
                category="presence",
                gap_class="missing_api",
            ),
        ]),
    )
    data = toolchain_report_to_dict(report)
    summary = family_pressure_summary(data)

    assert summary["family_counts"]["timing"] == 1
    assert summary["family_counts"]["network_info"] == 1


def test_family_pressure_does_not_promote_evidence():
    report = run_environment_toolchain(
        "",
        profile=None,
        candidate_pack=None,
        probe_pack=custom_probe_pack([
            gap_probe(
                probe_id="document.missing",
                target="document.readyState",
                category="presence",
                gap_class="missing_api",
            )
        ]),
    )
    data = toolchain_report_to_dict(report)

    assert any(
        item["kind"] == "environment_family_pressure_analyzed"
        and item["strength"] == "diagnostic_only"
        for item in data["evidence"]
    )
    assert [
        item["kind"]
        for item in data["evidence"]
        if item["kind"] == "environment_family_pressure_analyzed"
    ] == ["environment_family_pressure_analyzed"]
    assert all(item["strength"] != "strong" for item in data["evidence"])
    assert data["schema_version"] == "environment-toolchain.v0.1"
