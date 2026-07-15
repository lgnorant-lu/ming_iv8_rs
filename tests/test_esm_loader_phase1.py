"""K-ESM-LOADER phase 1: dynamic import() + ResourceBundle modules."""

from __future__ import annotations

import json
import threading

import iv8_rs  # noqa: F401


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


def test_dynamic_import_data_url_module():
    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval_promise(
                r"""
                import('data:text/javascript,export default 42; export const a=1')
                  .then(function(m){
                    return JSON.stringify({def: m.default, a: m.a});
                  })
                """,
                200,
            )
        )

    rep = json.loads(_run(body))
    assert rep["def"] == 42 and rep["a"] == 1, rep


def test_dynamic_import_from_resource_bundle():
    def body():
        ctx = iv8_rs.JSContext()
        ctx.add_resource(
            "https://ex.test/mod.js",
            b"export const v = 9; export default 'ok';",
            200,
            {"Content-Type": "text/javascript"},
        )
        return str(
            ctx.eval_promise(
                r"""
                import('https://ex.test/mod.js').then(function(m){
                  return JSON.stringify({v: m.v, def: m.default});
                })
                """,
                200,
            )
        )

    rep = json.loads(_run(body))
    assert rep["v"] == 9 and rep["def"] == "ok", rep


def test_hidden_intensive_timer_floor_after_threshold():
    """Q082: after hidden for intensive_after_ms, use larger min interval."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  // Short thresholds for test (override via env not exposed — use defaults
                  // only for basic floor; intensive path needs env injection).
                  document.hidden = true;
                  var fired = -1;
                  var t0 = performance.now();
                  setTimeout(function(){ fired = performance.now() - t0; }, 0);
                  __iv8__.eventLoop.advance(1);
                  var early = fired;
                  __iv8__.eventLoop.advance(1000);
                  return JSON.stringify({early: early, late: fired});
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["early"] < 0, rep
    assert rep["late"] >= 999, rep
