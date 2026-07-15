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


def test_instrument_source_reports_dispatch_count_and_recommended_api():
    """v0.8.101 Q165: info dict includes multi-site metadata."""
    import iv8_rs

    def body():
        src = _REF.read_text(encoding="utf-8", errors="replace")
        patched, info = iv8_rs.instrument_source(src)
        return {
            "mode": info.get("mode"),
            "dispatch_count": info.get("dispatch_count"),
            "recommended": info.get("recommended_api"),
            "has_q165": bool(info.get("q165_note")),
            "has_log": "__iv8i_log__" in patched,
            "wrapped": "(globalThis.__iv8i_pc__=" in patched,
        }

    rep = _run(body)
    assert rep["mode"] == "chaosvm", rep
    assert rep["recommended"] == "instrument_source", rep
    assert rep["has_q165"] is True, rep
    assert isinstance(rep["dispatch_count"], int) and rep["dispatch_count"] >= 1, rep
    assert rep["has_log"] is True and rep["wrapped"] is True, rep


def test_instrument_source_multi_site_synthetic():
    """v0.8.101: all H[I[P++]]() sites rewritten (including offset 0)."""
    import iv8_rs

    def body():
        # Minimal chaosvm-like body: two dispatches, first at offset 0 of body
        # after our head is prepended.
        src = "B[g[D++]]();var x=1;B[g[D++]]();"
        patched, info = iv8_rs.instrument_source(src, mode="chaosvm")
        return {
            "dispatch_count": info.get("dispatch_count"),
            "wrap_count": patched.count("(globalThis.__iv8i_pc__="),
            "log_sites": patched.count("__iv8i_log__"),
            "handler": info.get("handler_array"),
            "offsets": info.get("dispatch_offsets"),
        }

    rep = _run(body)
    assert rep["handler"] == "B", rep
    assert rep["dispatch_count"] == 2, rep
    assert rep["wrap_count"] == 2, rep
    assert rep["log_sites"] >= 2, rep
    assert isinstance(rep["offsets"], list) and len(rep["offsets"]) == 2, rep


def test_tdc_instrument_source_preserves_setdata_and_collect():
    """v0.8.101: host-safe env Proxies (incl. screen) must keep TDC.setData + collect."""
    import iv8_rs

    def body():
        src = _REF.read_text(encoding="utf-8", errors="replace")
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
        # Default full targets (includes screen) after host-safe Reflect fix
        patched, info = iv8_rs.instrument_source(src)
        ctx = iv8_rs.JSContext(environment=env)
        ctx.eval(patched)
        set_t = ctx.eval("typeof TDC !== 'undefined' ? typeof TDC.setData : 'no'")
        get_t = ctx.eval("typeof TDC !== 'undefined' ? typeof TDC.getData : 'no'")
        sw = ctx.eval("typeof screen !== 'undefined' ? screen.width : null")
        if set_t == "function":
            ctx.eval("TDC.setData({ft:'tf'})")
        collect = ctx.eval('decodeURIComponent(TDC.getData(true) || "")') or ""
        ut = ctx.get_unified_trace() or []
        d_n = sum(1 for x in ut if str(x).startswith("D,"))
        targets = info.get("env_targets") or []
        return {
            "setData": set_t,
            "getData": get_t,
            "collect_len": len(collect) if isinstance(collect, str) else -1,
            "d_n": d_n,
            "has_screen_proxy": "screen" in targets,
            "screen_width": sw,
            "env_note": bool(info.get("env_proxy_note")),
        }

    rep = _run(body)
    assert rep["setData"] == "function", rep
    assert rep["getData"] == "function", rep
    assert rep["collect_len"] > 100, rep
    assert rep["d_n"] > 100, rep
    assert rep["has_screen_proxy"] is True, rep  # default full list
    assert rep["screen_width"] is not None, rep
    assert rep["env_note"] is True, rep


def test_instrument_source_env_targets_allowlist():
    """v0.8.101: env_targets controls which globals get Proxies; capture_env=false disables."""
    import iv8_rs

    def body():
        src = "B[g[D++]]();"
        _, info_full = iv8_rs.instrument_source(src, mode="chaosvm")
        _, info_nav = iv8_rs.instrument_source(
            src, mode="chaosvm", env_targets=["navigator"]
        )
        _, info_off = iv8_rs.instrument_source(src, mode="chaosvm", capture_env=False)
        return {
            "full": info_full.get("env_targets"),
            "nav_only": info_nav.get("env_targets"),
            "off": info_off.get("env_targets"),
            "capture_env_off": info_off.get("capture_env"),
        }

    rep = _run(body)
    assert "screen" in (rep["full"] or []), rep
    assert rep["nav_only"] == ["navigator"], rep
    assert rep["off"] == [] or rep["capture_env_off"] is False, rep


def test_instrument_source_expose_handlers_opt_in():
    """v0.8.101 Tier B: expose_handlers assigns __iv8_vm_handlers__ in dispatch scope."""
    import iv8_rs

    def body():
        src = _REF.read_text(encoding="utf-8", errors="replace")
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
        patched, info = iv8_rs.instrument_source(src, expose_handlers=True)
        ctx = iv8_rs.JSContext(environment=env)
        ctx.eval(patched)
        # After VM init + at least one dispatch, handlers may be exposed
        ha_type = ctx.eval("typeof globalThis.__iv8_vm_handlers__")
        set_t = ctx.eval("typeof TDC !== 'undefined' ? typeof TDC.setData : 'no'")
        collect = ""
        if ctx.eval("typeof TDC !== 'undefined' && typeof TDC.getData === 'function'"):
            if set_t == "function":
                ctx.eval("TDC.setData({ft:'tf'})")
            collect = ctx.eval('decodeURIComponent(TDC.getData(true) || "")') or ""
        return {
            "expose_flag": info.get("expose_handlers"),
            "handlers_type": ha_type,
            "setData": set_t,
            "collect_len": len(collect) if isinstance(collect, str) else -1,
            "has_expose_in_src": "__iv8_vm_handlers__" in patched,
        }

    rep = _run(body)
    assert rep["expose_flag"] is True, rep
    assert rep["has_expose_in_src"] is True, rep
    assert rep["setData"] == "function", rep
    assert rep["collect_len"] > 100, rep
    # handlers should be array/object after dispatch ran
    assert rep["handlers_type"] in ("object", "function"), rep
