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
    """Q070 deep: sequential document.write via html5ever insertAdjacentHTML."""

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


def test_document_write_mixed_markup_and_script_via_parser():
    """Q070: write mixed HTML+script — parser materializes both; script runs."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.page_load("<html><body><p id='p'>P</p></body></html>", "https://ex.test/")
        return str(
            ctx.eval(
                r"""
                (function(){
                  document.write('<div id="n">N</div><script id="ws">window.__w=8;<\/script>');
                  return JSON.stringify({
                    p: !!document.getElementById('p'),
                    n: !!document.getElementById('n'),
                    w: window.__w,
                    scripts: document.scripts.length,
                    ws: !!document.getElementById('ws')
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["p"] and rep["n"] and rep["ws"], rep
    assert rep["w"] == 8, rep
    assert rep["scripts"] >= 1, rep


def test_parse_time_write_inserts_after_running_script():
    """Q070 parse-time: page_load sets currentScript; write inserts after SCRIPT."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.page_load(
            """
            <html><body>
            <div id="before">B</div>
            <script>
              window.__cs = document.currentScript && document.currentScript.tagName;
              document.write('<div id="mid">M</div>');
            </script>
            <div id="after">A</div>
            </body></html>
            """,
            "https://ex.test/",
        )
        return str(
            ctx.eval(
                r"""
                JSON.stringify({
                  cs: window.__cs,
                  mid: !!document.getElementById('mid'),
                  order: Array.from(document.body.children).map(function(n){
                    return n.id || n.tagName;
                  })
                })
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["cs"] == "SCRIPT", rep
    assert rep["mid"] is True, rep
    # mid appears after SCRIPT and before following siblings (parse-time insertion)
    order = rep["order"]
    assert order.index("SCRIPT") < order.index("mid") < order.index("after"), order


def test_document_open_write_close_rebuilds_via_page_load():
    """Q070 deep open: buffer writes until close → full html5ever rebuild."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.page_load(
            "<html><head><title>T1</title></head><body><p>1</p></body></html>",
            "https://ex.test/a",
        )
        return str(
            ctx.eval(
                r"""
                (function(){
                  document.open();
                  document.write('<html><head><title>T2</title></head><body>'
                    + '<p id="p2">2</p><script>window.__o=1;<\/script></body></html>');
                  document.close();
                  return JSON.stringify({
                    title: document.title,
                    p2: !!document.getElementById('p2'),
                    o: window.__o,
                    rs: document.readyState,
                    href: location.href
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["title"] == "T2", rep
    assert rep["p2"] is True, rep
    assert rep["o"] == 1, rep
    assert rep["rs"] == "complete", rep
    assert "ex.test" in rep["href"], rep


def test_document_open_progressive_reparse_between_writes():
    """Q070 tokenizer-adjacent: mid-stream write visible before close."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.page_load("<html><body></body></html>", "https://ex.test/")
        return str(
            ctx.eval(
                r"""
                (function(){
                  document.open();
                  document.write('<html><body><div id="a">A</div>');
                  var mid = !!document.getElementById('a');
                  document.write('<div id="b">B</div></body></html>');
                  document.close();
                  return JSON.stringify({
                    mid: mid,
                    a: !!document.getElementById('a'),
                    b: !!document.getElementById('b')
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["mid"] is True, rep
    assert rep["a"] and rep["b"], rep


def test_document_write_executes_inline_script():
    """Q070 phase A+: write('<script>…') runs classic inline script."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.page_load("<html><body></body></html>", "https://ex.test/")
        return str(
            ctx.eval(
                r"""
                (function(){
                  document.write('<script>window.__fromWrite=7;window.__cs=document.currentScript&&document.currentScript.tagName;<\/script>');
                  return JSON.stringify({
                    v: window.__fromWrite,
                    cs: window.__cs,
                    scripts: document.scripts.length,
                    byTag: document.getElementsByTagName('script').length
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["v"] == 7, rep
    assert rep["cs"] == "SCRIPT", rep
    assert rep["scripts"] >= 1 and rep["byTag"] >= 1, rep


def test_document_write_external_script_from_resource_bundle():
    """Q070: write('<script src=...>') loads sync via ResourceBundle/XHR."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.add_resource(
            "https://ex.test/w.js",
            b"window.__extFromWrite=42;",
            200,
            {"Content-Type": "text/javascript"},
        )
        ctx.page_load("<html><body></body></html>", "https://ex.test/")
        return str(
            ctx.eval(
                r"""
                (function(){
                  document.write('<script src="https://ex.test/w.js"><\/script>');
                  return JSON.stringify({v: window.__extFromWrite});
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["v"] == 42, rep


def test_document_open_clears_content():
    """Q070: document.open() clears children and resets write anchor."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.page_load("<html><body><div id='old'>O</div></body></html>", "https://ex.test/")
        return str(
            ctx.eval(
                r"""
                (function(){
                  document.write('<div id="mid">M</div>');
                  document.open();
                  var afterOpen = document.getElementById('old') || document.getElementById('mid');
                  document.write('<div id="new">N</div>');
                  document.close();
                  return JSON.stringify({
                    cleared: !afterOpen,
                    hasNew: !!document.getElementById('new'),
                    rs: document.readyState
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["cleared"] is True, rep
    assert rep["hasNew"] is True, rep


def test_create_element_script_append_runs_inline():
    """W1 gap: createElement('script')+appendChild runs classic inline code."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.page_load("<html><body></body></html>", "https://ex.test/")
        return str(
            ctx.eval(
                r"""
                (function(){
                  var s = document.createElement('script');
                  s.textContent = 'window.__dynScript=3';
                  document.body.appendChild(s);
                  return JSON.stringify({v: window.__dynScript});
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["v"] == 3, rep


def test_hidden_document_timer_floor():
    """Q082 residual: document.hidden applies timers.hidden_min_interval_ms floor."""

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
                  // Without floor, advance(1) would fire; with 1000ms floor need advance(1000).
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


def test_blank_context_readystate_complete():
    """Blank JSContext stays complete until page_load."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(ctx.eval("document.readyState"))

    assert _run(body) == "complete"


def test_type_module_inline_executes_via_eval_module():
    """K-ESM-LOADER: type=module inline runs after classic/defer (side effects)."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.page_load(
            """
            <html><body>
            <script>window.__classic=1; window.__ord=[]; window.__ord.push('c');</script>
            <script type="module">window.__mod=1; window.__ord.push('m');</script>
            <script defer>window.__ord.push('d');</script>
            </body></html>
            """,
            "https://ex.test/",
        )
        return str(
            ctx.eval(
                "JSON.stringify({mod: window.__mod, classic: window.__classic, ord: window.__ord})"
            )
        )

    rep = json.loads(_run(body))
    assert rep.get("classic") == 1, rep
    assert rep.get("mod") == 1, rep
    # classic then defer then module (offline model)
    assert rep["ord"] == ["c", "d", "m"], rep


def test_document_onreadystatechange_property_fires():
    """Q080 gap: document.onreadystatechange property (not only addEventListener)."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var hits = [];
                  document.onreadystatechange = function(){ hits.push(document.readyState); };
                  document.readyState = 'loading';
                  document.readyState = 'interactive';
                  document.readyState = 'complete';
                  return JSON.stringify(hits);
                })()
                """
            )
        )

    hits = json.loads(_run(body))
    assert hits == ["loading", "interactive", "complete"], hits
