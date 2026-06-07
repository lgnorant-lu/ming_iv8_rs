"""Runtime helpers for Environment Toolchain probe packs.

This module starts with the v0.8.1 probe-pack model only. It intentionally does
not run JavaScript, apply patches, or write profiles/manifests/baselines.
"""

from __future__ import annotations

from dataclasses import asdict, dataclass, field
from typing import Any

__all__ = [
    "ProbeDefinition",
    "ProbePack",
    "available_probe_packs",
    "load_probe_pack",
    "probe_pack_from_dict",
    "probe_pack_to_dict",
]


_ALLOWED_EVIDENCE_CEILINGS = {"diagnostic_only", "weak"}
_ALLOWED_PROBE_CATEGORIES = {"presence", "descriptor", "behavior", "value"}


@dataclass(frozen=True, slots=True)
class ProbeDefinition:
    probe_id: str
    target: str
    category: str
    js: str
    expected: Any
    gap_class: str
    side_effects: list[str] = field(default_factory=list)
    cleanup: str = "none"
    evidence_ceiling: str = "diagnostic_only"

    def __post_init__(self) -> None:
        if not self.probe_id:
            raise ValueError("probe_id must not be empty")
        if not self.target:
            raise ValueError("target must not be empty")
        if self.category not in _ALLOWED_PROBE_CATEGORIES:
            raise ValueError(f"invalid probe category: {self.category}")
        if self.evidence_ceiling not in _ALLOWED_EVIDENCE_CEILINGS:
            raise ValueError(f"invalid evidence ceiling: {self.evidence_ceiling}")
        if self.evidence_ceiling == "weak":
            raise ValueError("probe definitions cannot claim weak evidence before runner review")

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ProbeDefinition:
        return cls(
            probe_id=data["probe_id"],
            target=data["target"],
            category=data["category"],
            js=data["js"],
            expected=data["expected"],
            gap_class=data["gap_class"],
            side_effects=list(data.get("side_effects", [])),
            cleanup=data.get("cleanup", "none"),
            evidence_ceiling=data.get("evidence_ceiling", "diagnostic_only"),
        )

    def to_dict(self) -> dict[str, Any]:
        return asdict(self)


@dataclass(frozen=True, slots=True)
class ProbePack:
    probe_pack: str
    version: int
    description: str
    evidence_ceiling: str
    probes: list[ProbeDefinition]

    def __post_init__(self) -> None:
        if not self.probe_pack:
            raise ValueError("probe_pack must not be empty")
        if self.version < 1:
            raise ValueError("probe pack version must be positive")
        if self.evidence_ceiling != "diagnostic_only":
            raise ValueError("probe packs must be diagnostic_only before runner review")
        if not self.probes:
            raise ValueError("probe pack must contain at least one probe")
        probe_ids = [probe.probe_id for probe in self.probes]
        duplicates = sorted({probe_id for probe_id in probe_ids if probe_ids.count(probe_id) > 1})
        if duplicates:
            raise ValueError(f"duplicate probe ids: {duplicates}")

    @classmethod
    def from_dict(cls, data: dict[str, Any]) -> ProbePack:
        return cls(
            probe_pack=data["probe_pack"],
            version=int(data["version"]),
            description=data["description"],
            evidence_ceiling=data["evidence_ceiling"],
            probes=[ProbeDefinition.from_dict(probe) for probe in data.get("probes", [])],
        )

    def to_dict(self) -> dict[str, Any]:
        data = asdict(self)
        data["probes"] = [probe.to_dict() for probe in self.probes]
        return data


_FINGERPRINT_M1: dict[str, Any] = {
    "probe_pack": "fingerprint.m1",
    "version": 1,
    "description": "baseline navigator, screen, webdriver, and descriptor probes",
    "evidence_ceiling": "diagnostic_only",
    "probes": [
        {
            "probe_id": "navigator.languages.present",
            "target": "navigator.languages",
            "category": "presence",
            "js": "return Array.isArray(navigator.languages) && navigator.languages.length > 0;",
            "expected": True,
            "gap_class": "missing_api",
            "side_effects": [],
            "cleanup": "none",
            "evidence_ceiling": "diagnostic_only",
        },
        {
            "probe_id": "navigator.webdriver.value",
            "target": "navigator.webdriver",
            "category": "value",
            "js": (
                "return navigator.webdriver === false || "
                "typeof navigator.webdriver === 'undefined';"
            ),
            "expected": True,
            "gap_class": "value_mismatch",
            "side_effects": [],
            "cleanup": "none",
            "evidence_ceiling": "diagnostic_only",
        },
        {
            "probe_id": "screen.width.present",
            "target": "screen.width",
            "category": "presence",
            "js": "return typeof screen.width === 'number' && screen.width > 0;",
            "expected": True,
            "gap_class": "missing_api",
            "side_effects": [],
            "cleanup": "none",
            "evidence_ceiling": "diagnostic_only",
        },
        {
            "probe_id": "navigator.userAgent.descriptor",
            "target": "navigator.userAgent",
            "category": "descriptor",
            "js": (
                "var d = Object.getOwnPropertyDescriptor(navigator, 'userAgent') || "
                "Object.getOwnPropertyDescriptor(Object.getPrototypeOf(navigator), 'userAgent'); "
                "return !!d && typeof d.get === 'function';"
            ),
            "expected": True,
            "gap_class": "descriptor_mismatch",
            "side_effects": [],
            "cleanup": "none",
            "evidence_ceiling": "diagnostic_only",
        },
        {
            "probe_id": "navigator.userAgentData.shape",
            "target": "navigator.userAgentData",
            "category": "behavior",
            "js": (
                "return typeof navigator.userAgentData === 'object' && "
                "Array.isArray(navigator.userAgentData.brands);"
            ),
            "expected": True,
            "gap_class": "behavior_mismatch",
            "side_effects": [],
            "cleanup": "none",
            "evidence_ceiling": "diagnostic_only",
        },
    ],
}

_BUILTIN_PROBE_PACKS = {"fingerprint.m1": _FINGERPRINT_M1}


def available_probe_packs() -> list[str]:
    return sorted(_BUILTIN_PROBE_PACKS)


def load_probe_pack(probe_pack: str) -> ProbePack:
    try:
        data = _BUILTIN_PROBE_PACKS[probe_pack]
    except KeyError as exc:
        available = ", ".join(available_probe_packs())
        raise ValueError(f"unknown probe pack: {probe_pack}; available: {available}") from exc
    return ProbePack.from_dict(data)


def probe_pack_from_dict(data: dict[str, Any]) -> ProbePack:
    return ProbePack.from_dict(data)


def probe_pack_to_dict(probe_pack: ProbePack) -> dict[str, Any]:
    return probe_pack.to_dict()
