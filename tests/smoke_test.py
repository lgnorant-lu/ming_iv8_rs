"""Minimal wheel smoke test for cibuildwheel."""

import iv8_rs


def test_version():
    assert isinstance(iv8_rs.__version__, str)
    assert len(iv8_rs.__version__) > 0


def test_eval_basic():
    with iv8_rs.JSContext() as ctx:
        assert ctx.eval("1 + 1") == 2
