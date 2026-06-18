# v0.8.29 L2 Stage 2 MVP — S4 Report Builder
#
# Assembles Environment Plane Report v0.1 JSON per
# environment-controlled-adaptation-spec.md S8 schema.

from __future__ import annotations

from datetime import datetime, timezone
from typing import Any

from .environment_dry_run_engine import Candidate, ComparisonReport
from .environment_probe_runner import GapList


class ReportBuilder:
    """Build Environment Plane Report v0.1 from stage results."""

    def build(
        self,
        input_info: dict[str, Any],
        gaps: GapList,
        candidates: list[Candidate],
        dry_run: ComparisonReport | None = None,
    ) -> dict[str, Any]:
        diagnostics = self._emit_diagnostics(gaps, candidates, dry_run is not None)

        report: dict[str, Any] = {
            "schema_version": "l2-stage2.v0.1",
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "input": input_info,
            "gaps": gaps.to_dict(),
            "candidates": {
                "selected": len(candidates),
                "eligible": len([c for c in candidates if c.policy == "runtime_safe"]),
                "blocked": len(
                    [c for c in candidates if c.policy != "runtime_safe"]
                ),
                "skipped": 0,
            },
            "diagnostics": diagnostics,
            "writes": [],
            "applied_patches": [],
        }

        if dry_run is not None:
            report["dry_run"] = {
                "kernel_config": {
                    "heap_limits": [536870912, 4294967296],
                },
                "before": dry_run.before,
                "after": dry_run.after,
                "diff": dry_run.diff,
            }

        diagnostics.append(
            {
                "code": "ENV_TOOLCHAIN_COMPARISON_REPORT_BUILT",
                "severity": "info",
                "stage": "S4",
                "details": {"schema_version": "l2-stage2.v0.1"},
            }
        )

        return report

    def _emit_diagnostics(
        self, gaps: GapList, candidates: list[Candidate], dry_run_completed: bool
    ) -> list[dict[str, Any]]:
        diags: list[dict[str, Any]] = []

        diags.append(
            {
                "code": "ENV_TOOLCHAIN_PROBE_RUN_STARTED",
                "severity": "info",
                "stage": "S1",
                "details": {},
            }
        )

        if gaps.missing or gaps.mismatch:
            diags.append(
                {
                    "code": "ENVIRONMENT_GAP_DETECTED",
                    "severity": "info",
                    "details": {
                        "missing_count": len(gaps.missing),
                        "mismatch_count": len(gaps.mismatch),
                    },
                }
            )

        if candidates:
            diags.append(
                {
                    "code": "ENVIRONMENT_CANDIDATE_SELECTED",
                    "severity": "info",
                    "details": {
                        "selected_count": len(candidates),
                        "symbols": [c.symbol for c in candidates],
                    },
                }
            )

            for c in candidates:
                if c.policy == "runtime_safe":
                    diags.append(
                        {
                            "code": "PATCH_POLICY_RUNTIME_SAFE",
                            "severity": "info",
                            "details": {"symbol": c.symbol},
                        }
                    )
                elif c.policy == "analysis_only":
                    diags.append(
                        {
                            "code": "PATCH_POLICY_ANALYSIS_ONLY",
                            "severity": "warn",
                            "details": {"symbol": c.symbol},
                        }
                    )
                elif c.policy == "unsafe_hook":
                    diags.append(
                        {
                            "code": "PATCH_POLICY_UNSAFE_HOOK",
                            "severity": "blocked",
                            "details": {"symbol": c.symbol},
                        }
                    )
                else:
                    diags.append(
                        {
                            "code": "PATCH_POLICY_DRY_RUN_SKIPPED",
                            "severity": "warn",
                            "details": {
                                "symbol": c.symbol,
                                "reason": f"policy_not_handled: {c.policy}",
                            },
                        }
                    )

        if dry_run_completed:
            diags.append(
                {
                    "code": "ENV_TOOLCHAIN_DRY_RUN_STARTED",
                    "severity": "info",
                    "stage": "S3",
                    "details": {},
                }
            )
            diags.append(
                {
                    "code": "ENV_TOOLCHAIN_DRY_RUN_COMPLETED",
                    "severity": "info",
                    "stage": "S3",
                    "details": {},
                }
            )

        return diags
