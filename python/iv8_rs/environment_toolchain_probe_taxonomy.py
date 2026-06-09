"""Diagnostic-only probe taxonomy helpers for Environment Toolchain planning."""

from __future__ import annotations

from dataclasses import dataclass

PROBE_ROLES = frozenset({
    "baseline_surface",
    "descriptor_surface",
    "shape_surface",
    "semantics_surface",
    "bridge_readiness",
})

ROUTE_OWNERS = frozenset({
    "probe_pack_review",
    "candidate_pack_review",
    "profile_pack_review",
    "bridge_capability_review",
    "native_substrate_review",
    "local_prelude_review",
    "observe_only",
    "blocked_target_flow",
})

BRIDGE_CAPABILITIES = frozenset({
    "dom_fixture_runtime",
    "browser_like_window",
    "script_tag_bootstrap",
    "prelude_bootstrap",
    "network_shape_stub",
    "descriptor_probe_surface",
    "event_loop_semantics_probe",
    "external_environment_sidecar",
    "native_substrate_candidate",
})

FUTURE_PLACEHOLDER_PROBE_PACKS = frozenset({
    "network-shape.m1",
    "event-loop-shape.m1",
    "dom-fixture.m1",
    "bridge-readiness.m1",
})


@dataclass(frozen=True)
class ProbeRoute:
    """Review-only route alignment for one pressure kind."""

    pressure_kind: str
    probe_role: str
    bridge_capability: str
    route_owner: str

    def to_dict(self) -> dict[str, str]:
        return {
            "pressure_kind": self.pressure_kind,
            "probe_role": self.probe_role,
            "bridge_capability": self.bridge_capability,
            "route_owner": self.route_owner,
        }


_BUILTIN_PROBE_PACK_ROLES = {
    "fingerprint.m1": "baseline_surface",
    "descriptor.m1": "descriptor_surface",
}

_PRESSURE_KIND_ROUTES = {
    "input_normalization": ProbeRoute(
        pressure_kind="input_normalization",
        probe_role="bridge_readiness",
        bridge_capability="script_tag_bootstrap",
        route_owner="local_prelude_review",
    ),
    "web_api_surface": ProbeRoute(
        pressure_kind="web_api_surface",
        probe_role="shape_surface",
        bridge_capability="browser_like_window",
        route_owner="probe_pack_review",
    ),
    "dom_surface": ProbeRoute(
        pressure_kind="dom_surface",
        probe_role="shape_surface",
        bridge_capability="dom_fixture_runtime",
        route_owner="bridge_capability_review",
    ),
    "descriptor_shape": ProbeRoute(
        pressure_kind="descriptor_shape",
        probe_role="descriptor_surface",
        bridge_capability="descriptor_probe_surface",
        route_owner="probe_pack_review",
    ),
    "page_lifecycle": ProbeRoute(
        pressure_kind="page_lifecycle",
        probe_role="bridge_readiness",
        bridge_capability="script_tag_bootstrap",
        route_owner="bridge_capability_review",
    ),
    "prelude_contract": ProbeRoute(
        pressure_kind="prelude_contract",
        probe_role="bridge_readiness",
        bridge_capability="prelude_bootstrap",
        route_owner="local_prelude_review",
    ),
    "network_surface": ProbeRoute(
        pressure_kind="network_surface",
        probe_role="shape_surface",
        bridge_capability="network_shape_stub",
        route_owner="probe_pack_review",
    ),
    "timing_surface": ProbeRoute(
        pressure_kind="timing_surface",
        probe_role="semantics_surface",
        bridge_capability="event_loop_semantics_probe",
        route_owner="probe_pack_review",
    ),
    "event_loop_semantics": ProbeRoute(
        pressure_kind="event_loop_semantics",
        probe_role="semantics_surface",
        bridge_capability="event_loop_semantics_probe",
        route_owner="probe_pack_review",
    ),
    "runtime_stability": ProbeRoute(
        pressure_kind="runtime_stability",
        probe_role="baseline_surface",
        bridge_capability="native_substrate_candidate",
        route_owner="native_substrate_review",
    ),
    "analysis_observability": ProbeRoute(
        pressure_kind="analysis_observability",
        probe_role="bridge_readiness",
        bridge_capability="external_environment_sidecar",
        route_owner="observe_only",
    ),
}


def built_in_probe_pack_roles() -> dict[str, str]:
    """Return diagnostic roles for currently available built-in probe packs."""
    return dict(_BUILTIN_PROBE_PACK_ROLES)


def probe_pack_role(pack_id: str) -> str:
    """Return the diagnostic role for a built-in probe pack id."""
    try:
        return _BUILTIN_PROBE_PACK_ROLES[pack_id]
    except KeyError as exc:
        raise ValueError(f"unknown built-in probe pack for taxonomy: {pack_id}") from exc


def future_placeholder_probe_packs() -> list[str]:
    """Return planning-only future probe pack names.

    These names are not available built-ins and must not be treated as assets.
    """
    return sorted(FUTURE_PLACEHOLDER_PROBE_PACKS)


def pressure_kind_routes() -> dict[str, dict[str, str]]:
    """Return review-only probe route alignment for known pressure kinds."""
    return {kind: route.to_dict() for kind, route in _PRESSURE_KIND_ROUTES.items()}


def pressure_kind_probe_route(pressure_kind: str) -> dict[str, str]:
    """Return review-only route alignment for one pressure kind."""
    try:
        return _PRESSURE_KIND_ROUTES[pressure_kind].to_dict()
    except KeyError as exc:
        raise ValueError(f"unknown pressure kind for probe taxonomy: {pressure_kind}") from exc


def validate_probe_role(role: str) -> str:
    """Validate and return a probe taxonomy role."""
    if role not in PROBE_ROLES:
        raise ValueError(f"unknown probe role: {role}")
    return role


def validate_route_owner(route_owner: str) -> str:
    """Validate and return a review-only route owner."""
    if route_owner not in ROUTE_OWNERS:
        raise ValueError(f"unknown route owner: {route_owner}")
    return route_owner
