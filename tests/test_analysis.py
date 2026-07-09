"""
Tests for the Diff Analysis Framework (iv8_rs.analysis).

Covers both _find_diff_positions (pure function) and diff_analysis
(which internally creates JSContext — monkeypatched to avoid real V8).
"""

from __future__ import annotations

from typing import Any

import pytest
from iv8_rs.analysis import _find_diff_positions, diff_analysis

# ═══════════════════════════════════════════════════════════════════════
# Mock JSContext — avoids importing the real Rust extension for tests
# ═══════════════════════════════════════════════════════════════════════

class FakeJSContext:
    """Deterministic mock: eval(expr) returns sorted env items as string.

    The second eval call (the eval_expr) produces output that always
    reflects the full env dict, so any env change is visible as a diff.
    """
    _all_instances: list[FakeJSContext] = []

    def __init__(
        self,
        environment: dict[str, Any] | None = None,
        random_seed: int | None = None,
        time_freeze: float | None = None,
        time_mode: str = "logical",
    ) -> None:
        self.env = environment or {}
        self.random_seed = random_seed
        self.time_freeze = time_freeze
        self.time_mode = time_mode
        self._call_count = 0
        self.closed = False
        FakeJSContext._all_instances.append(self)

    def eval(self, code: str) -> str | None:
        self._call_count += 1
        if self._call_count == 1:
            return None  # js_source eval — no return value
        return str(sorted(self.env.items()))

    def close(self) -> None:
        self.closed = True


@pytest.fixture(autouse=True)
def _patch_jscontext(monkeypatch: pytest.MonkeyPatch) -> None:
    import iv8_rs._iv8
    FakeJSContext._all_instances.clear()
    monkeypatch.setattr(iv8_rs._iv8, "JSContext", FakeJSContext)


# ═══════════════════════════════════════════════════════════════════════
# _find_diff_positions  —  pure function, no mocking needed
# ═══════════════════════════════════════════════════════════════════════

class TestFindDiffPositions:
    def test_identical_strings(self) -> None:
        assert _find_diff_positions("hello", ["hello"]) == []
        assert _find_diff_positions("", [""]) == []

    def test_single_difference(self) -> None:
        assert _find_diff_positions("abc", ["abd"]) == [2]

    def test_multiple_differences(self) -> None:
        assert _find_diff_positions("abc", ["xyz"]) == [0, 1, 2]

    def test_multiple_variants(self) -> None:
        assert _find_diff_positions("abc", ["abd", "xbc"]) == [0, 2]

    def test_longer_variant(self) -> None:
        assert _find_diff_positions("abc", ["abcdef"]) == [3, 4, 5]

    def test_shorter_variant(self) -> None:
        assert _find_diff_positions("abcdef", ["abc"]) == [3, 4, 5]

    def test_one_shorter_one_longer(self) -> None:
        assert _find_diff_positions("abc", ["ab", "abcd"]) == [2, 3]

    def test_unicode_same(self) -> None:
        assert _find_diff_positions("héllo", ["héllo"]) == []

    def test_empty_variants(self) -> None:
        assert _find_diff_positions("abc", []) == []

    def test_all_variants_same(self) -> None:
        assert _find_diff_positions("test", ["test", "test"]) == []

    def test_empty_base_with_content(self) -> None:
        assert _find_diff_positions("", ["abc"]) == [0, 1, 2]

    def test_empty_base_multiple_variants(self) -> None:
        assert _find_diff_positions("", ["a", "ab"]) == [0, 1]

    def test_base_vs_empty_variant(self) -> None:
        assert _find_diff_positions("abc", [""]) == [0, 1, 2]

    def test_identical_unicode(self) -> None:
        assert _find_diff_positions("café", ["café"]) == []

    def test_many_variants_no_diff(self) -> None:
        assert _find_diff_positions("same", ["same", "same", "same"]) == []


# ═══════════════════════════════════════════════════════════════════════
# diff_analysis  —  basic behaviour
# ═══════════════════════════════════════════════════════════════════════

class TestDiffAnalysisBasic:
    def test_single_variable_single_value(self) -> None:
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1},
            test_variables={"a": [2]},
        )
        assert "a" in report
        assert report["a"]["affected"] is True
        assert report["a"]["values_tested"] == [2]
        assert len(report["a"]["results"]) == 1

    def test_no_difference(self) -> None:
        """Same value as base → no diff."""
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1},
            test_variables={"a": [1]},
        )
        assert report["a"]["affected"] is False
        assert report["a"]["positions"] == []

    def test_multiple_variables(self) -> None:
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1, "b": 2},
            test_variables={"a": [10], "b": [20]},
        )
        assert "a" in report
        assert "b" in report
        assert report["a"]["affected"] is True
        assert report["b"]["affected"] is True

    def test_multiple_values_per_variable(self) -> None:
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1},
            test_variables={"a": [2, 3, 4]},
        )
        assert report["a"]["affected"] is True
        assert report["a"]["values_tested"] == [2, 3, 4]
        assert len(report["a"]["results"]) == 3

    def test_empty_test_variables(self) -> None:
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1},
            test_variables={},
        )
        assert report == {}

    def test_base_result_in_report(self) -> None:
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1, "b": 2},
            test_variables={"a": [10]},
        )
        assert "base_result" in report["a"]
        assert "('a', 1)" in report["a"]["base_result"]
        assert "('b', 2)" in report["a"]["base_result"]


# ═══════════════════════════════════════════════════════════════════════
# diff_analysis  —  execution modes & callbacks
# ═══════════════════════════════════════════════════════════════════════

class TestDiffAnalysisExecution:
    def test_serial_path(self) -> None:
        """max_workers=1 forces the serial execution branch."""
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1},
            test_variables={"a": [2]},
            max_workers=1,
        )
        assert report["a"]["affected"] is True

    def test_progress_callback_parallel(self) -> None:
        calls: list[tuple] = []

        def cb(var_name: str, value: Any, result: str) -> None:
            calls.append((var_name, value, result))

        diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1, "b": 2},
            test_variables={"a": [2], "b": [3]},
            progress_callback=cb,
        )
        assert len(calls) == 2
        names = {c[0] for c in calls}
        assert names == {"a", "b"}

    def test_progress_callback_serial(self) -> None:
        calls: list[tuple] = []

        def cb(var_name: str, value: Any, result: str) -> None:
            calls.append((var_name, value))

        diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1},
            test_variables={"a": [2]},
            max_workers=1,
            progress_callback=cb,
        )
        assert len(calls) == 1

    def test_custom_seed_and_time(self) -> None:
        """Custom params forwarded to all JSContext instances."""
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1},
            test_variables={"a": [2]},
            random_seed=123,
            time_freeze=999.0,
            time_mode="frozen",
        )
        assert report["a"]["affected"] is True

    def test_default_params(self) -> None:
        """Default random_seed=42 and time_freeze=None / time_mode='logical'."""
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1},
            test_variables={"a": [2]},
        )
        assert report["a"]["affected"] is True
        for inst in FakeJSContext._all_instances:
            if inst.env.get("a") == 2:
                assert inst.random_seed == 42
                assert inst.time_freeze is None
                assert inst.time_mode == "logical"

    def test_parallel_with_many_tasks(self) -> None:
        """ThreadPoolExecutor exercised with many tasks."""
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={f"k{i}": i for i in range(8)},
            test_variables={f"k{i}": [i + 100] for i in range(8)},
            max_workers=4,
        )
        assert len(report) == 8
        for v in report.values():
            assert v["affected"] is True

    def test_parallel_path_single_task(self) -> None:
        """max_workers>1 but only 1 task → serial path anyway."""
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1},
            test_variables={"a": [2]},
            max_workers=4,
        )
        assert report["a"]["affected"] is True


# ═══════════════════════════════════════════════════════════════════════
# diff_analysis  —  error handling
# ═══════════════════════════════════════════════════════════════════════

class TestDiffAnalysisErrors:
    """Uses a mock that raises on request."""

    class FailOnValueJSContext:
        """Raises RuntimeError when env has a key whose value is '__fail__'."""
        def __init__(
            self,
            environment: dict[str, Any] | None = None,
            **kwargs: Any,
        ) -> None:
            self.env = environment or {}
            self.closed = False

        def eval(self, code: str) -> str:
            if "__fail__" in str(self.env.values()):
                raise RuntimeError("Intentional test failure")
            return str(sorted(self.env.items()))

        def close(self) -> None:
            self.closed = True

    @pytest.fixture(autouse=True)
    def _patch_failing(self, monkeypatch: pytest.MonkeyPatch) -> None:
        import iv8_rs._iv8
        monkeypatch.setattr(iv8_rs._iv8, "JSContext", self.FailOnValueJSContext)

    def test_error_result_prefix(self) -> None:
        """Exception captured as __ERROR__: prefixed result."""
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1},
            test_variables={"a": ["__fail__"]},
        )
        assert "__ERROR__" in report["a"]["results"][0]
        assert "Intentional test failure" in report["a"]["results"][0]

    def test_error_still_produces_diff(self) -> None:
        """Error result differs from success base → affected=True."""
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1},
            test_variables={"a": ["__fail__"]},
        )
        assert report["a"]["affected"] is True
        assert "__ERROR__" in report["a"]["results"][0]

    def test_base_also_errors(self) -> None:
        """Both base and variants error the same way → no diff."""
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": "__fail__"},
            test_variables={"a": ["__fail__"]},
        )
        assert report["a"]["affected"] is False

    def test_parallel_with_errors(self) -> None:
        """ThreadPoolExecutor path with some errors."""
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1, "b": 2},
            test_variables={"a": ["__fail__"], "b": [999]},
            max_workers=2,
        )
        assert "__ERROR__" in report["a"]["results"][0]
        assert report["b"]["affected"] is True

    def test_serial_with_errors(self) -> None:
        """Serial path with errors."""
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1},
            test_variables={"a": ["__fail__"]},
            max_workers=1,
        )
        assert "__ERROR__" in report["a"]["results"][0]
        assert report["a"]["affected"] is True


# ═══════════════════════════════════════════════════════════════════════
# diff_analysis  —  edge cases
# ═══════════════════════════════════════════════════════════════════════

class TestDiffAnalysisEdgeCases:
    def test_none_result_converted_to_empty_string(self) -> None:
        """When eval returns None, run_single returns ''."""
        class NoneJSContext:
            def __init__(self, environment=None, **kwargs):
                self.closed = False
            def eval(self, code):
                return None
            def close(self):
                self.closed = True

        import iv8_rs._iv8
        monkeypatch = pytest.MonkeyPatch()
        monkeypatch.setattr(iv8_rs._iv8, "JSContext", NoneJSContext)
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1},
            test_variables={"a": [2]},
        )
        monkeypatch.undo()
        assert report["a"]["base_result"] == ""
        assert report["a"]["results"] == [""]
        assert report["a"]["affected"] is False

    def test_multiple_vars_no_effect(self) -> None:
        """Multiple variables at values equal to base → no diffs."""
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1, "b": 2},
            test_variables={"a": [1], "b": [2]},
        )
        assert report["a"]["affected"] is False
        assert report["b"]["affected"] is False

    def test_mixed_effect(self) -> None:
        """One variable affects output, another doesn't."""
        report = diff_analysis(
            js_source="var x = 1;",
            eval_expr="x",
            base_env={"a": 1, "b": 2},
            test_variables={"a": [99], "b": [2]},
        )
        assert report["a"]["affected"] is True
        assert report["b"]["affected"] is False
        assert len(report["a"]["positions"]) > 0
        assert report["b"]["positions"] == []
