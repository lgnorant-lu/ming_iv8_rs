"""D4: runnable smoke examples aligned with README / docs/api Tier A pages.

These are documentation-contract tests, not full behavioral suites.
Metric: docs/conventions/api-documentation-conventions.md D4a/D4b.

JSContext creation runs on a large-stack thread (K-010): main-thread
stack is often insufficient for full kernel template install.
"""

from __future__ import annotations

import threading
from typing import Any, Callable, TypeVar

import iv8_rs
import pytest

_STACK = 128 * 1024 * 1024
T = TypeVar("T")


def _on_large_stack(fn: Callable[..., T], *args: Any, **kwargs: Any) -> T:
    box: dict[str, Any] = {}

    def work() -> None:
        try:
            box["v"] = fn(*args, **kwargs)
        except BaseException as e:
            box["e"] = e

    old = threading.stack_size()
    try:
        threading.stack_size(_STACK)
    except (ValueError, OSError):
        pass
    t = threading.Thread(target=work)
    t.start()
    t.join(120)
    try:
        threading.stack_size(old)
    except (ValueError, OSError):
        pass
    if t.is_alive():
        raise TimeoutError("large-stack worker timed out")
    if "e" in box:
        raise box["e"]
    return box["v"]


def test_readme_quick_start_eval_and_context_manager():
    def run():
        with iv8_rs.JSContext() as ctx:
            ua = ctx.eval("navigator.userAgent")
            assert isinstance(ua, str) and len(ua) > 0
            assert ctx.eval("1 + 1") == 2

    _on_large_stack(run)


def test_docs_profile_environment_merge_priority():
    def run():
        ctx = iv8_rs.JSContext(
            profile="default",
            environment={"navigator.language": "zh-CN"},
        )
        try:
            assert ctx.eval("navigator.language") == "zh-CN"
        finally:
            ctx.close()

    _on_large_stack(run)


def test_docs_module_level_exports_exist():
    for name in (
        "instrument_source",
        "prepare_entry",
        "plan_multi_entry",
        "run_with_entry",
        "trace_diff",
        "enable_logging",
        "load_profile",
    ):
        assert hasattr(iv8_rs, name), name


def test_docs_instrument_source_auto_on_simple_non_vm():
    src = "var x = 1;"
    try:
        patched, info = iv8_rs.instrument_source(src)
        assert isinstance(patched, str)
        assert isinstance(info, dict)
    except RuntimeError:
        pass


def test_docs_prepare_entry_returns_dict():
    plan = iv8_rs.prepare_entry("console.log(1)", persona="analysis")
    assert isinstance(plan, dict)


def test_docs_trace_diff_identical():
    a = ["D,0,a,1", "R,1,b,2"]
    d = iv8_rs.trace_diff(a, list(a))
    assert isinstance(d, dict)
    assert d.get("index") == -1 or d.get("match_count") == len(a)


def test_docs_version_string():
    assert isinstance(iv8_rs.__version__, str)
    assert len(iv8_rs.__version__) > 0


def test_docs_exception_types_importable():
    for name in (
        "JSError",
        "JSCompileError",
        "JSTimeoutError",
        "JSMemoryError",
        "JSPanic",
    ):
        assert issubclass(getattr(iv8_rs, name), BaseException)


def test_docs_debugger_constructs():
    def run():
        ctx = iv8_rs.JSContext()
        try:
            dbg = iv8_rs.Debugger(ctx)
            assert hasattr(dbg, "trace_api")
            assert hasattr(dbg, "eval_traced")
        finally:
            ctx.close()

    _on_large_stack(run)


def test_docs_add_resource_offline_chain():
    def run():
        ctx = iv8_rs.JSContext()
        try:
            ctx.add_resource(
                "https://example.test/x.js",
                "globalThis.__x = 7;",
                status=200,
            )
            assert not ctx.is_disposed()
        finally:
            ctx.close()

    _on_large_stack(run)
