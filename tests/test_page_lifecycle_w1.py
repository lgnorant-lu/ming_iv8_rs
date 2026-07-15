"""W1: Q080 readyState lifecycle, Q081 script classic/defer/async, Q070 write phase A."""

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


def test_page_load_readystate_order_and_events():
    """Q080: loading → classic → interactive → defer → DCL → complete → load."""

    def body():
        ctx = iv8_rs.JSContext()
        html = """
        <html><head></head><body>
        <script>
          window.__lc = [];
          window.__lc.push('classic:' + document.readyState);
          document.addEventListener('readystatechange', function() {
            window.__lc.push('rsc:' + document.readyState);
          });
          document.addEventListener('DOMContentLoaded', function() {
            window.__lc.push('dcl:' + document.readyState);
          });
          window.addEventListener('load', function() {
            window.__lc.push('load:' + document.readyState);
          });
        </script>
        <script defer>
          window.__lc.push('defer:' + document.readyState);
        </script>
        <script async>
          window.__lc.push('async:' + document.readyState);
        </script>
        </body></html>
        """
        ctx.page_load(html, "https://ex.test/p")
        return str(ctx.eval("JSON.stringify({lc: window.__lc, rs: document.readyState})"))

    rep = json.loads(_run(body))
    lc = rep["lc"]
    assert rep["rs"] == "complete", rep
    assert "classic:loading" in lc, lc
    # async after classic, still before interactive in our offline model
    assert lc.index("classic:loading") < lc.index("async:loading"), lc
    assert "defer:interactive" in lc, lc
    assert "dcl:interactive" in lc, lc
    assert "rsc:interactive" in lc and "rsc:complete" in lc, lc
    assert "load:complete" in lc, lc
    # defer before DCL, DCL before complete
    assert lc.index("defer:interactive") < lc.index("dcl:interactive"), lc
    assert lc.index("dcl:interactive") < lc.index("rsc:complete"), lc


def test_script_defer_runs_after_classic_before_dcl():
    """Q081: document-order defer after classic, before DOMContentLoaded."""

    def body():
        ctx = iv8_rs.JSContext()
        html = """
        <html><body>
        <script>window.__ord=[]; window.__ord.push('c1');</script>
        <script defer>window.__ord.push('d1');</script>
        <script>window.__ord.push('c2');</script>
        <script defer>window.__ord.push('d2');</script>
        <script>
          document.addEventListener('DOMContentLoaded', function() {
            window.__ord.push('dcl');
          });
        </script>
        </body></html>
        """
        ctx.page_load(html, "https://ex.test/")
        return str(ctx.eval("JSON.stringify(window.__ord)"))

    ord_ = json.loads(_run(body))
    assert ord_ == ["c1", "c2", "d1", "d2", "dcl"], ord_


def test_document_write_sequential_phase_a():
    """Q070 phase A: sequential document.write appends into body."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.page_load("<html><body></body></html>", "https://ex.test/")
        return str(
            ctx.eval(
                r"""
                (function(){
                  document.write('<div id="a">A</div>');
                  document.write('<div id="b">B</div>');
                  return JSON.stringify({
                    a: !!document.getElementById('a'),
                    b: !!document.getElementById('b'),
                    order: document.getElementById('a').nextElementSibling
                      && document.getElementById('a').nextElementSibling.id
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["a"] and rep["b"], rep
    assert rep["order"] == "b", rep


def test_blank_context_readystate_complete():
    """Blank JSContext stays complete until page_load."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(ctx.eval("document.readyState"))

    assert _run(body) == "complete"
