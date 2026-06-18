# v0.8.29 L2 Stage 2 MVP — S1 Probe Runner
#
# Executes a probe pack in a JSContext, classifies results as
# missing/mismatch/present per environment-controlled-adaptation-spec.md S4.

from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path

import iv8_rs

ASSETS_DIR = Path(__file__).resolve().parent.parent / "fixtures" / "environment_toolchain" / "probe_packs"


@dataclass
class ProbeResult:
    symbol: str
    expression: str
    gap_kind: str  # missing | mismatch | present
    expected_type: str | None = None
    actual_value: str | None = None
    error: str | None = None


@dataclass
class GapList:
    missing: list[ProbeResult] = field(default_factory=list)
    mismatch: list[ProbeResult] = field(default_factory=list)
    present: list[ProbeResult] = field(default_factory=list)

    @property
    def total(self) -> int:
        return len(self.missing) + len(self.mismatch) + len(self.present)

    def to_dict(self) -> dict:
        return {
            "total": self.total,
            "missing": len(self.missing),
            "mismatch": len(self.mismatch),
            "present": len(self.present),
        }


class ProbeRunner:
    """Execute a probe pack in a JSContext and classify results."""

    def __init__(self, probe_pack: str):
        self.probe_pack = probe_pack
        self.probes = self._load_pack(probe_pack)

    def _load_pack(self, pack_name: str) -> list[dict]:
        path = ASSETS_DIR / f"{pack_name}.json"
        if not path.exists():
            raise FileNotFoundError(f"probe pack {pack_name} not found at {path}")
        with open(path, encoding="utf-8") as f:
            data = json.load(f)
        return data.get("probes", data) if isinstance(data, dict) else data

    def run(self, ctx: iv8_rs.JSContext) -> GapList:
        gaps = GapList()
        for probe in self.probes:
            result = self._evaluate_probe(ctx, probe)
            if result.gap_kind == "missing":
                gaps.missing.append(result)
            elif result.gap_kind == "mismatch":
                gaps.mismatch.append(result)
            else:
                gaps.present.append(result)
        return gaps

    def _evaluate_probe(self, ctx: iv8_rs.JSContext, probe: dict) -> ProbeResult:
        symbol = probe.get("target", probe.get("probe_id", "?"))
        expression = probe.get("js", "")
        expected = probe.get("expected")
        gap_class = probe.get("gap_class", "missing_api")

        try:
            # Wrap in IIFE if the expression contains 'return' (probe packs
            # use return statements for assertions)
            if "return " in expression or expression.strip().startswith("return"):
                expression = f"(function() {{ {expression} }})()"
            actual = ctx.eval(expression)
        except Exception as exc:
            return ProbeResult(
                symbol=symbol,
                expression=expression,
                gap_kind="missing",
                expected_type=str(expected),
                error=str(exc),
            )

        if actual is None:
            return ProbeResult(
                symbol=symbol,
                expression=expression,
                gap_kind="missing",
                expected_type=str(expected),
            )

        actual_str = str(actual).lower()
        if actual_str == "undefined" or actual_str == "null" or actual_str == "none" or actual_str == "":
            return ProbeResult(
                symbol=symbol,
                expression=expression,
                gap_kind="missing",
                expected_type=str(expected),
                actual_value=actual_str,
            )

        # Compare expected value
        expected_str = str(expected).lower()
        if expected is not None and actual_str != expected_str:
            return ProbeResult(
                symbol=symbol,
                expression=expression,
                gap_kind="mismatch",
                expected_type=str(expected),
                actual_value=actual_str,
            )

        return ProbeResult(
            symbol=symbol,
            expression=expression,
            gap_kind="present",
            expected_type=str(expected),
            actual_value=actual_str,
        )
