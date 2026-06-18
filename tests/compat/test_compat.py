"""Compatibility tests: iv8-rs output must match iv8 0.1.2 output exactly.

Run with: uv run pytest tests/compat/ -v
"""
import json
import math
import pathlib

import pytest

import iv8_rs

FIXTURES_DIR = pathlib.Path(__file__).parent / "fixtures"


def collect_fixtures():
    """Collect all .js files that have a corresponding .expected.json."""
    fixtures = []
    for js_file in sorted(FIXTURES_DIR.rglob("*.js")):
        expected_file = js_file.with_suffix(".expected.json")
        if expected_file.exists():
            fixtures.append((js_file, expected_file))
    return fixtures


FIXTURES = collect_fixtures()


@pytest.mark.parametrize(
    "js_file,expected_file",
    FIXTURES,
    ids=[str(f[0].relative_to(FIXTURES_DIR)) for f in FIXTURES],
)
def test_compat(ctx, js_file, expected_file):
    """Each fixture's eval output must match iv8 0.1.2's output."""
    source = js_file.read_text(encoding="utf-8")
    expected = json.loads(expected_file.read_text(encoding="utf-8"))

    # All compat fixtures now pass — no xfail needed.
    # convert.rs correctly implements iv8 0.1.2 behavior for all types.

    try:
        actual = ctx.eval(source)
        assert expected["ok"], (
            f"iv8 errored ({expected.get('error_type')}: {expected.get('error')})"
            f" but iv8-rs succeeded with: {actual!r}"
        )
        compare_values(actual, expected["value"], str(js_file.name))
    except iv8_rs.JSError as e:
        assert not expected["ok"], (
            f"iv8 succeeded with {expected.get('value')!r}"
            f" but iv8-rs raised: {e}"
        )


def compare_values(actual, expected, context=""):
    """Compare actual vs expected, handling special float values."""
    if isinstance(expected, float):
        if math.isnan(expected):
            assert isinstance(actual, float) and math.isnan(actual), (
                f"{context}: expected NaN, got {actual!r}"
            )
            return
        if math.isinf(expected):
            assert actual == expected, f"{context}: expected {expected}, got {actual!r}"
            return

    if isinstance(expected, dict) and isinstance(actual, dict):
        for key in expected:
            assert key in actual, f"{context}: missing key '{key}'"
            compare_values(actual[key], expected[key], f"{context}.{key}")
        return

    if isinstance(expected, list) and isinstance(actual, list):
        assert len(actual) == len(expected), (
            f"{context}: list length {len(actual)} != {len(expected)}"
        )
        for i, (a, e) in enumerate(zip(actual, expected)):
            compare_values(a, e, f"{context}[{i}]")
        return

    # For bytes comparison: iv8 returns bytes as list of ints in to_py=True mode
    # but our fixture uses to_py=True so expected might be a list
    if isinstance(actual, bytes) and isinstance(expected, list):
        assert list(actual) == expected, f"{context}: bytes mismatch"
        return

    # None comparison (null/undefined both → None)
    if expected is None:
        assert actual is None, f"{context}: expected None, got {actual!r}"
        return

    assert actual == expected, f"{context}: {actual!r} != {expected!r}"
