"""Minimal smoke tests (local pytest). CI wheel test uses python -c (no conftest)."""

from __future__ import annotations

import iv8_rs


def test_version_string():
    assert isinstance(iv8_rs.__version__, str)
    assert len(iv8_rs.__version__) > 0


def test_eval_one_plus_one():
    ctx = iv8_rs.JSContext()
    try:
        assert ctx.eval("1 + 1") == 2
    finally:
        ctx.close()
