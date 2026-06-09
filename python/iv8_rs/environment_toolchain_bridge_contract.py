"""Diagnostic-only bridge contract helpers for Environment Toolchain planning."""

from __future__ import annotations

BRIDGE_LEVELS = frozenset({
    "B0",
    "B1",
    "B2",
    "B3",
    "B4",
})

OBSERVATION_STATUSES = frozenset({
    "not_configured",
    "method_reference_only",
    "synthetic_observed",
    "sidecar_unavailable",
    "sidecar_error",
    "redaction_blocked",
    "boundary_blocked",
    "review_only",
})

BLOCKED_RESULT_STATES = frozenset({
    "pass",
    "strong_evidence",
    "adapter_applied",
    "profile_written",
    "substrate_promoted",
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

_PACKAGE_SPECIFIC_NAMES = frozenset({
    "jsdom",
    "sdenv",
    "happy-dom",
    "linkedom",
    "playwright",
    "cdp",
})

_TARGET_FLOW_TERMS = frozenset({
    "domain",
    "endpoint",
    "cookie",
    "token",
    "signature",
    "nonce",
    "request_body",
    "request body",
    "authorization_header",
    "authorization",
    "secret",
})

ALLOWED_EVIDENCE_CEILINGS = frozenset({
    "diagnostic_only",
})


def validate_bridge_level(level: str) -> str:
    if level not in BRIDGE_LEVELS:
        raise ValueError(f"unknown bridge level: {level}")
    return level


def validate_observation_status(status: str) -> str:
    if status not in OBSERVATION_STATUSES:
        raise ValueError(f"unknown observation status: {status}")
    return status


def validate_evidence_ceiling(ceiling: str) -> str:
    if ceiling not in ALLOWED_EVIDENCE_CEILINGS:
        raise ValueError(
            f"bridge contract evidence ceiling must be diagnostic_only, got: {ceiling}"
        )
    return ceiling


def validate_bridge_capability(capability: str) -> str:
    if capability not in BRIDGE_CAPABILITIES:
        raise ValueError(f"unknown bridge capability: {capability}")
    return capability


def validate_package_neutral(value: str) -> str:
    lowered = value.lower()
    for name in _PACKAGE_SPECIFIC_NAMES:
        if name in lowered:
            raise ValueError(f"package-specific name is not allowed as route owner: {value}")
    return value


def validate_contract_writes(writes: list) -> list:
    if writes:
        raise ValueError(f"bridge contract writes must be empty, got: {writes}")
    return writes


def _scan_text(text: str) -> list[str]:
    lowered = text.lower()
    return [term for term in _TARGET_FLOW_TERMS if term in lowered]


def check_target_flow_terms(payload: dict | str) -> list[str]:
    found: list[str] = []
    if isinstance(payload, dict):
        stack: list = [payload]
        while stack:
            item = stack.pop()
            if isinstance(item, dict):
                for key, value in item.items():
                    found.extend(_scan_text(key))
                    if isinstance(value, (dict, list)):
                        stack.append(value)
                    elif isinstance(value, str):
                        found.extend(_scan_text(value))
            elif isinstance(item, list):
                for element in item:
                    if isinstance(element, (dict, list, str)):
                        stack.append(element)
    elif isinstance(payload, str):
        found.extend(_scan_text(payload))
    return sorted(set(found))
