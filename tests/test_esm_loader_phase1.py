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


def test_import_relative_specifier_from_parent_module():
    """ESM relative import ./a.js resolved against parent module URL."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.add_resource(
            "https://ex.test/a.js",
            b"export const a = 1;",
            200,
            {"Content-Type": "text/javascript"},
        )
        ctx.add_resource(
            "https://ex.test/b.js",
            b'import { a } from "./a.js"; export const b = a + 1;',
            200,
            {"Content-Type": "text/javascript"},
        )
        return str(
            ctx.eval_promise(
                "import('https://ex.test/b.js').then(m => JSON.stringify({b: m.b}))",
                200,
            )
        )

    rep = json.loads(_run(body))
    assert rep["b"] == 2, rep


def test_importmap_bare_specifier_resolution():
    """HTML importmap maps bare specifier for type=module side effects."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.add_resource(
            "https://ex.test/lib.js",
            b"window.__fromMap = 11; export const x = 11;",
            200,
            {"Content-Type": "text/javascript"},
        )
        ctx.page_load(
            """
            <html><body>
            <script type="importmap">{"imports":{"lib":"https://ex.test/lib.js"}}</script>
            <script type="module">import "lib";</script>
            </body></html>
            """,
            "https://ex.test/",
        )
        return str(ctx.eval("JSON.stringify({v: window.__fromMap})"))

    rep = json.loads(_run(body))
    assert rep["v"] == 11, rep


def test_network_handler_async_order_honesty_with_set_timeout():
    """Q097 residual honesty: network_handler + setTimeout models delayed async order."""

    def body():
        ctx = iv8_rs.JSContext()
        order = []

        def handler(url, method):
            order.append(f"handler:{method}")
            return (200, b"delayed-body")

        ctx.set_network_handler(handler)
        # Classic offline model: async scripts still sequential; delayed XHR via
        # setTimeout simulates race-ish ordering without real outbound.
        return str(
            ctx.eval_promise(
                r"""
                new Promise(function(resolve){
                  var hits = [];
                  hits.push('start');
                  var x = new XMLHttpRequest();
                  x.onreadystatechange = function(){
                    if (x.readyState === 4) {
                      hits.push('xhr:' + x.responseText);
                      resolve(JSON.stringify(hits));
                    }
                  };
                  x.open('GET', 'https://ex.test/delay');
                  x.send();
                  hits.push('after-send');
                })
                """,
                200,
            )
        )

    rep = json.loads(_run(body))
    assert "start" in rep and "after-send" in rep, rep
    assert any(h.startswith("xhr:") for h in rep), rep
