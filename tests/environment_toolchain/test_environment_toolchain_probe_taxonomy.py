from __future__ import annotations

import pytest
from iv8_rs.environment_toolchain_probe_taxonomy import (
    BRIDGE_CAPABILITIES,
    PROBE_ROLES,
    ROUTE_OWNERS,
    built_in_probe_pack_roles,
    future_placeholder_probe_packs,
    pressure_kind_probe_route,
    pressure_kind_routes,
    probe_pack_role,
    validate_probe_role,
    validate_route_owner,
)
from iv8_rs.environment_toolchain_runtime import available_probe_packs


def test_builtin_probe_pack_roles_classify_existing_assets_only():
    roles = built_in_probe_pack_roles()

    assert roles == {
        "descriptor.m1": "descriptor_surface",
        "fingerprint.m1": "baseline_surface",
    }
    assert sorted(roles) == available_probe_packs()


def test_fingerprint_m1_remains_baseline_surface_only():
    assert probe_pack_role("fingerprint.m1") == "baseline_surface"


def test_descriptor_m1_remains_descriptor_surface():
    assert probe_pack_role("descriptor.m1") == "descriptor_surface"


def test_future_placeholder_probe_packs_are_not_available_builtins():
    placeholders = set(future_placeholder_probe_packs())

    assert placeholders == {
        "bridge-readiness.m1",
        "dom-fixture.m1",
        "event-loop-shape.m1",
        "network-shape.m1",
    }
    assert placeholders.isdisjoint(available_probe_packs())
    assert placeholders.isdisjoint(built_in_probe_pack_roles())


def test_pressure_kind_routes_use_accepted_roles_capabilities_and_owners():
    for route in pressure_kind_routes().values():
        assert route["probe_role"] in PROBE_ROLES
        assert route["bridge_capability"] in BRIDGE_CAPABILITIES
        assert route["route_owner"] in ROUTE_OWNERS


def test_network_surface_routes_to_shape_probe_review_without_replay():
    route = pressure_kind_probe_route("network_surface")

    assert route == {
        "pressure_kind": "network_surface",
        "probe_role": "shape_surface",
        "bridge_capability": "network_shape_stub",
        "route_owner": "probe_pack_review",
    }


def test_descriptor_shape_routes_to_descriptor_surface_probe_review():
    route = pressure_kind_probe_route("descriptor_shape")

    assert route["probe_role"] == "descriptor_surface"
    assert route["bridge_capability"] == "descriptor_probe_surface"
    assert route["route_owner"] == "probe_pack_review"


def test_route_outputs_do_not_use_package_specific_names():
    serialized = repr({
        "roles": sorted(PROBE_ROLES),
        "route_owners": sorted(ROUTE_OWNERS),
        "routes": pressure_kind_routes(),
    }).lower()

    assert all(term not in serialized for term in ("jsdom", "sdenv", "happy-dom", "linkedom"))


def test_unknown_probe_pack_is_not_silently_classified():
    with pytest.raises(ValueError, match="unknown built-in probe pack"):
        probe_pack_role("network-shape.m1")


def test_unknown_pressure_kind_is_not_silently_routed():
    with pytest.raises(ValueError, match="unknown pressure kind"):
        pressure_kind_probe_route("sample_specific_sign_flow")


def test_role_and_route_owner_validation_reject_unknown_names():
    assert validate_probe_role("baseline_surface") == "baseline_surface"
    assert validate_route_owner("probe_pack_review") == "probe_pack_review"

    with pytest.raises(ValueError, match="unknown probe role"):
        validate_probe_role("full_fingerprint_model")
    with pytest.raises(ValueError, match="unknown route owner"):
        validate_route_owner("jsdom_adapter")
