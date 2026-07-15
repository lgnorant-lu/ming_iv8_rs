"""Q165: instrument_source works for closure-scoped TDC ChaosVM (local _ref sample)."""

from __future__ import annotations

import json
import os
import threading
from pathlib import Path

import pytest

_REF = Path(__file__).resolve().parents[1] / "_ref" / "samples" / "tdc-chaosvm" / "tdc_live.js"

pytestmark = pytest.mark.skipif(
    not _REF.is_file(),
    reason="local _ref TDC sample missing (gitignored)",
)


def _run(fn):
    threading.stack_size(128 * 1024 * 1024)
    box: list = []
    err: list = []

    def body():
        try:
            box.append(fn())
        except Exception as e:
            err.append(e)

    t = threading.Thread(target=body)
    t.start()
    t.join()
    if err:
        raise err[0]
    return box[0]


def test_instrument_source_yields_dispatch_trace_on_tdc_live():
    import iv8_rs

    def body():
        src = _REF.read_text(encoding="utf-8", errors="replace")
        patched, info = iv8_rs.instrument_source(src)
        assert info.get("mode") == "chaosvm"
        assert info.get("dispatch_pattern")
        assert "++" in str(info.get("dispatch_pattern"))

        env = {
            "location": {
                "href": "https://turing.captcha.qcloud.com/cap_union_new_show?sess=s0&sid=1",
                "hostname": "turing.captcha.qcloud.com",
                "protocol": "https:",
                "origin": "https://turing.captcha.qcloud.com",
            },
            "window": {
                "innerWidth": 360,
                "innerHeight": 360,
                "outerWidth": 1920,
                "outerHeight": 1200,
            },
        }
        ctx = iv8_rs.JSContext(environment=env)
        ctx.eval(patched)
        assert ctx.eval("typeof TDC !== 'undefined' && typeof TDC.getData === 'function'")
        collect = ctx.eval('decodeURIComponent(TDC.getData(true) || "")')
        ut = ctx.get_unified_trace()
        d_n = sum(1 for x in (ut or []) if str(x).startswith("D,"))
        return {
            "collect_len": len(collect) if isinstance(collect, str) else -1,
            "trace_n": len(ut or []),
            "d_n": d_n,
            "pattern": info.get("dispatch_pattern"),
        }

    rep = _run(body)
    assert rep["collect_len"] > 100, rep
    assert rep["d_n"] > 100, rep
    assert rep["trace_n"] >= rep["d_n"], rep


def test_instrument_chaosvm_errors_with_instrument_source_hint():
    import iv8_rs

    def body():
        src = _REF.read_text(encoding="utf-8", errors="replace")
        _, info = iv8_rs.instrument_source(src)
        ha = info.get("handler_array") or "A"
        pc = info.get("pc_var") or "U"
        st = info.get("stack_var") or "S"
        ctx = iv8_rs.JSContext()
        ctx.eval(src)
        try:
            ctx.instrument_chaosvm(ha, pc_var=pc, stack_var=st, limit=100)
            return {"ok": True}
        except Exception as e:
            msg = str(e)
            return {
                "ok": False,
                "hint": "instrument_source" in msg,
                "msg": msg[:200],
            }

    rep = _run(body)
    assert rep["ok"] is False, rep
    assert rep["hint"] is True, rep
