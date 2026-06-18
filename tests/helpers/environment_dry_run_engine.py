# v0.8.29 L2 Stage 2 MVP — S3 Dry-Run Engine
#
# Creates fresh EmbeddedV8Kernel per candidate, applies candidate
# via JS eval, compares before/after probe results.
# No writes — all kernels are temporary and discarded after comparison.

from __future__ import annotations

import json
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

import iv8_rs

from .environment_probe_runner import GapList, ProbeRunner

ASSETS_DIR = Path(__file__).resolve().parent.parent / "fixtures" / "environment_toolchain" / "candidate_packs"


@dataclass
class Candidate:
    symbol: str
    patch_js: str
    policy: str  # runtime_safe | analysis_only | unsafe_hook
    description: str = ""

    def matches(self, gap_symbol: str) -> bool:
        return self.symbol.lower() == gap_symbol.lower()


@dataclass
class ComparisonReport:
    before: dict
    after: dict
    diff: dict


class DryRunEngine:
    """Apply candidates in fresh kernels, produce before/after comparison."""

    def __init__(self, candidate_pack: str = "default"):
        self.candidate_pack = candidate_pack
        self.candidates = self._load_pack(candidate_pack)

    def _load_pack(self, pack_name: str) -> list[Candidate]:
        path = ASSETS_DIR / f"{pack_name}.json"
        if not path.exists():
            return []
        with open(path, encoding="utf-8") as f:
            data = json.load(f)
        raw = data.get("candidates", data) if isinstance(data, dict) else data
        return [Candidate(**c) for c in raw]

    def select(self, gaps: GapList) -> list[Candidate]:
        """Select runtime_safe candidates for each missing gap."""
        selected = []
        seen = set()
        for gap in gaps.missing:
            for candidate in self.candidates:
                if candidate.policy != "runtime_safe":
                    continue
                if candidate.matches(gap.symbol) and candidate.symbol not in seen:
                    selected.append(candidate)
                    seen.add(candidate.symbol)
                    break
        return selected

    def apply(
        self,
        candidate: Candidate,
        probe_pack: str = "fingerprint.m1",
    ) -> ComparisonReport:
        baseline = iv8_rs.JSContext()
        before = ProbeRunner(probe_pack).run(baseline)
        baseline.close()

        adapted = iv8_rs.JSContext()
        try:
            adapted.eval(candidate.patch_js)
        except Exception:
            pass
        after = ProbeRunner(probe_pack).run(adapted)
        adapted.close()

        return ComparisonReport(
            before=before.to_dict(),
            after=after.to_dict(),
            diff=self._compute_diff(before, after),
        )

    def _compute_diff(self, before: GapList, after: GapList) -> dict[str, int]:
        return {
            "missing_delta": len(after.missing) - len(before.missing),
            "mismatch_delta": len(after.mismatch) - len(before.mismatch),
            "gaps_closed": len(before.missing) - len(after.missing),
        }
