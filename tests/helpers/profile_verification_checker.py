"""v0.8.32 Profile Verification Convergence Checker.

Report-only, no-write, bounded-iteration diagnostic tool.
Not full MAPE-K — this is the safety precursor that validates
profile-derived runtime behavior is observable.

Usage (dry-run only):
    checker = ConvergenceChecker(profile_overrides)
    verdict = checker.check_static_core()
    print(verdict)
"""

from __future__ import annotations

import time
from typing import Any

import iv8_rs


class ProbeResult:
    def __init__(self, probe_id, target, category, status, expected, actual, diff_class=None):
        self.probe_id = probe_id
        self.target = target
        self.category = category
        self.status = status
        self.expected = expected
        self.actual = actual
        self.diff_class = diff_class

    def to_dict(self):
        d = {
            "probe_id": self.probe_id,
            "target": self.target,
            "category": self.category,
            "status": self.status,
            "expected": self.expected,
            "actual": self.actual,
            "evidence_ceiling": "v8_surface",
        }
        if self.diff_class:
            d["diff_class"] = self.diff_class
        return d


class CheckerReport:
    def __init__(self, schema_version="iv8-convergence-check.v0.1"):
        self.schema_version = schema_version
        self.run_started_at = time.time()
        self.duration_ms = 0
        self.total = 0
        self.passed = 0
        self.failed = 0
        self.material_failures = 0
        self.expected_divergences = 0
        self.unexpected_divergences = 0
        self.verdict = "no_data"
        self.certifies = []
        self.does_not_certify = [
            "chromium_layout_parity",
            "network_stack_parity",
            "creepjs_grade",
            "browserleaks_pass",
        ]
        self.probe_results: list[ProbeResult] = []
        self.writes: list[str] = []
        self.errors: list[str] = []

    def add_probe(self, result: ProbeResult):
        self.total += 1
        if result.status == "pass":
            self.passed += 1
        elif result.status in ("material", "fail"):
            self.failed += 1
            self.material_failures += 1
        elif result.status == "expected_divergence":
            self.expected_divergences += 1
        elif result.status == "unexpected_divergence":
            self.unexpected_divergences += 1
        self.probe_results.append(result)

    def finalize(self):
        self.duration_ms = int((time.time() - self.run_started_at) * 1000)
        if self.material_failures > 0 or self.unexpected_divergences > 0:
            self.verdict = "failed"
        elif self.total == 0:
            self.verdict = "no_data"
        elif self.passed + self.expected_divergences == self.total:
            self.verdict = "equivalent"
        else:
            self.verdict = "partial"

    def to_dict(self):
        return {
            "schema_version": self.schema_version,
            "run": {
                "started_at": self.run_started_at,
                "duration_ms": self.duration_ms,
            },
            "summary": {
                "total": self.total,
                "passed": self.passed,
                "failed": self.failed,
                "material_failures": self.material_failures,
                "expected_divergences": self.expected_divergences,
                "unexpected_divergences": self.unexpected_divergences,
            },
            "verdict": self.verdict,
            "certification": {
                "level": "deterministic_v8_surface_equivalence",
                "certifies": self.certifies,
                "does_not_certify": self.does_not_certify,
            },
            "probe_results": [r.to_dict() for r in self.probe_results],
            "errors": self.errors,
            "writes": self.writes,
        }


class ConvergenceChecker:
    """Profile-to-runtime verification checker (report-only, no-write)."""

    def __init__(
        self,
        environment_overrides: dict[str, Any],
        *,
        runtime_environment: dict[str, Any] | None = None,
        max_iterations: int = 0,
    ):
        self._overrides = environment_overrides
        self._runtime_environment = runtime_environment if runtime_environment is not None else environment_overrides
        self._max_iterations = max_iterations
        self._iteration = 0
        self._previous_pass_count = 0

    def check_static_core(self) -> CheckerReport:
        report = CheckerReport()
        report.certifies = [
            "profile_loaded",
            "static_js_values",
        ]

        ctx = self._make_context()

        # --- navigator probes ---
        self._probe_expected(report, ctx, "navigator.userAgent", "navigator.userAgent")
        self._probe_expected(report, ctx, "navigator.platform", "navigator.platform")
        self._probe_expected(report, ctx, "navigator.vendor", "navigator.vendor")
        self._probe_expected(report, ctx, "navigator.language", "navigator.language")
        self._probe_expected(report, ctx, "navigator.webdriver", "navigator.webdriver")
        self._probe_expected(report, ctx, "navigator.hardwareConcurrency", "navigator.hardwareConcurrency")
        self._probe_expected(report, ctx, "navigator.deviceMemory", "navigator.deviceMemory")
        self._probe_expected(report, ctx, "navigator.maxTouchPoints", "navigator.maxTouchPoints")

        # --- screen probes ---
        self._probe_expected(report, ctx, "screen.width", "screen.width")
        self._probe_expected(report, ctx, "screen.height", "screen.height")
        self._probe_expected(report, ctx, "screen.availWidth", "screen.availWidth")
        self._probe_expected(report, ctx, "screen.availHeight", "screen.availHeight")
        self._probe_expected(report, ctx, "screen.colorDepth", "screen.colorDepth")
        self._probe_expected(report, ctx, "screen.pixelDepth", "screen.pixelDepth")

        # --- known divergences ---
        report.add_probe(ProbeResult(
            "fonts.value", "fonts", "value",
            "expected_divergence",
            expected="fonts_available", actual="unsupported",
        ))
        report.add_probe(ProbeResult(
            "webrtc.value", "webrtc", "value",
            "expected_divergence",
            expected="real_browser_webrtc", actual="v8_only_limit",
        ))

        report.writes = []
        report.finalize()
        return report

    def _make_context(self) -> iv8_rs.JSContext:
        env = dict(self._runtime_environment)
        return iv8_rs.JSContext(environment=env)

    def _probe_expected(self, report, ctx, probe_id, js_expr):
        if probe_id not in self._overrides:
            report.add_probe(ProbeResult(
                probe_id, js_expr, "value", "material",
                expected="<override_present>", actual="<missing_override>",
                diff_class="missing_expected_value",
            ))
            return

        expected = self._overrides[probe_id]
        try:
            actual = ctx.eval(js_expr)
            status = "pass" if actual == expected else "material"
        except Exception as e:
            actual = f"<eval_error: {e}>"
            status = "material"
        report.add_probe(ProbeResult(
            probe_id, js_expr, "value", status,
            expected=expected, actual=actual,
            diff_class=None if status == "pass" else "value_mismatch",
        ))

    def _probe_str(self, report, ctx, probe_id, js_expr, expected=None):
        try:
            actual = ctx.eval(js_expr)
            status = "pass"
            if expected is not None and str(actual) != str(expected):
                status = "material"
        except Exception as e:
            actual = f"<eval_error: {e}>"
            status = "material"
        report.add_probe(ProbeResult(
            probe_id, js_expr, "value", status,
            expected=str(expected) if expected else "<present>",
            actual=str(actual),
        ))

    def _probe_bool(self, report, ctx, probe_id, js_expr, expected):
        try:
            actual = ctx.eval(js_expr)
            expected_bool = bool(expected)
            status = "pass" if actual == expected_bool else "material"
        except Exception as e:
            actual = f"<eval_error: {e}>"
            status = "material"
        report.add_probe(ProbeResult(
            probe_id, js_expr, "value", status,
            expected=str(expected), actual=str(actual),
        ))

    def _probe_int(self, report, ctx, probe_id, js_expr, min_val=0):
        try:
            actual = ctx.eval(js_expr)
            actual_val = int(actual) if actual is not None else 0
            status = "pass" if actual_val >= min_val else "material"
        except Exception as e:
            actual = f"<eval_error: {e}>"
            status = "material"
        report.add_probe(ProbeResult(
            probe_id, js_expr, "value", status,
            expected=f">= {min_val}",
            actual=str(actual),
        ))


# ---------------------------------------------------------------------------
# Chrome 147 / Windows 10 default profile overrides (minimal static core dict)
# ---------------------------------------------------------------------------

def chrome147_win10_overrides() -> dict[str, Any]:
    return {
        "navigator.userAgent": (
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) "
            "AppleWebKit/537.36 (KHTML, like Gecko) "
            "Chrome/147.0.0.0 Safari/537.36"
        ),
        "navigator.platform": "Win32",
        "navigator.vendor": "Google Inc.",
        "navigator.language": "zh-CN",
        "navigator.hardwareConcurrency": 8,
        "navigator.deviceMemory": 8,
        "navigator.maxTouchPoints": 0,
        "navigator.webdriver": False,
        "screen.width": 1920,
        "screen.height": 1080,
        "screen.availWidth": 1920,
        "screen.availHeight": 1040,
        "screen.colorDepth": 24,
        "screen.pixelDepth": 24,
    }
