"""v0.8.100 Band 0/2 detection surface: DET-1 createElement + __iv8 enum hide."""

from __future__ import annotations

import json
import threading

import pytest

threading.stack_size(128 * 1024 * 1024)
iv8_rs = pytest.importorskip("iv8_rs")


def _run(fn):
    box: dict = {}

    def work():
        box["out"] = fn()

    t = threading.Thread(target=work)
    t.start()
    t.join()
    return box["out"]


def test_cookie_httponly_not_visible_to_document_cookie():
    """Q093: HttpOnly cookies are not reflected on document.cookie."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  document.cookie = 'vis=1; Path=/';
                  document.cookie = 'http=1; Path=/; HttpOnly';
                  return document.cookie;
                })()
                """
            )
        )

    cookie = _run(body)
    assert "vis=1" in cookie, cookie
    assert "http=" not in cookie, cookie


def test_page_load_sets_location_title_and_body():
    """Q071/Q072: page_load populates DOM slots and syncs location.href."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.page_load(
            "<html><head><title>T</title></head><body><p id='p'>hi</p></body></html>",
            "https://example.com/path?q=1",
        )
        return str(
            ctx.eval(
                r"""
                JSON.stringify({
                  href: location.href,
                  title: document.title,
                  p: !!document.getElementById('p'),
                  head: !!document.head,
                  body: !!document.body
                })
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["href"] == "https://example.com/path?q=1", rep
    assert rep["title"] == "T", rep
    assert rep["p"] and rep["head"] and rep["body"], rep


def test_get_computed_style_basic_prefs():
    """Q073: getComputedStyle returns usable CSSStyleDeclaration values."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var el = document.createElement('div');
                  document.body.appendChild(el);
                  var cs = getComputedStyle(el);
                  return JSON.stringify({
                    display: cs.display,
                    color: cs.color,
                    fontSize: cs.fontSize,
                    activeText: cs.getPropertyValue('ActiveText') || cs.ActiveText || null
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["display"] in ("block", "inline", "inline-block", ""), rep
    assert isinstance(rep["color"], str) and len(rep["color"]) > 0, rep


def test_treewalker_and_xpath_subset():
    """Q075/Q076: XPath snapshot + TreeWalker basic walk."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.page_load(
            "<html><body><div id='a'><span>s</span></div></body></html>",
            "https://ex.test/",
        )
        return str(
            ctx.eval(
                r"""
                (function(){
                  var r = document.evaluate('//div', document, null, XPathResult.ORDERED_NODE_SNAPSHOT_TYPE, null);
                  var tw = document.createTreeWalker(document.body, NodeFilter.SHOW_ELEMENT);
                  var names = [];
                  var n = tw.currentNode;
                  var i = 0;
                  while (n && i++ < 8) { names.push(n.nodeName); n = tw.nextNode(); }
                  return JSON.stringify({snap: r.snapshotLength, names: names});
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["snap"] >= 1, rep
    assert "BODY" in rep["names"] or "DIV" in rep["names"], rep


def test_select_options_html_options_collection():
    """Q077: select.options is HTMLOptionsCollection with length/index access."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.page_load(
            "<html><body><select id='s'><option value='1'>a</option><option value='2'>b</option></select></body></html>",
            "https://ex.test/",
        )
        return str(
            ctx.eval(
                r"""
                (function(){
                  var o = document.getElementById('s').options;
                  return JSON.stringify({
                    ctor: o.constructor.name,
                    len: o.length,
                    o0: o[0] && o[0].value,
                    o1: o[1] && o[1].value,
                    tag: Object.prototype.toString.call(o)
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["ctor"] == "HTMLOptionsCollection", rep
    assert rep["len"] == 2 and rep["o0"] == "1" and rep["o1"] == "2", rep


def test_message_channel_structured_clone_roundtrip():
    """Q079: MessageChannel postMessage structured-clone object/array."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval_promise(
                r"""
                new Promise(function(resolve){
                  var ch = new MessageChannel();
                  ch.port1.onmessage = function(e){ resolve(JSON.stringify(e.data)); };
                  ch.port2.postMessage({a:1, b:[2,3]});
                })
                """,
                100,
            )
        )

    rep = json.loads(_run(body))
    assert rep == {"a": 1, "b": [2, 3]}, rep


def test_rsa_pkcs1_generate_key_and_p521_bound():
    """Q095: RSASSA-PKCS1-v1_5 generateKey works; P-521 remains unsupported bound."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval_promise(
                r"""
                (async function(){
                  var r = {};
                  try {
                    var k = await crypto.subtle.generateKey(
                      {name:'RSASSA-PKCS1-v1_5', modulusLength:2048, publicExponent:new Uint8Array([1,0,1]), hash:'SHA-256'},
                      true, ['sign','verify']
                    );
                    r.pkcs1 = !!(k && k.publicKey && k.privateKey);
                  } catch(e) { r.pkcs1 = String(e.message||e).slice(0,80); }
                  try {
                    await crypto.subtle.generateKey(
                      {name:'ECDSA', namedCurve:'P-521'}, true, ['sign','verify']
                    );
                    r.p521 = 'unexpected-ok';
                  } catch(e) { r.p521 = String(e.message||e).slice(0,80); }
                  return JSON.stringify(r);
                })()
                """,
                800,
            )
        )

    rep = json.loads(_run(body))
    assert rep["pkcs1"] is True, rep
    assert "P-521" in rep["p521"] or "namedCurve" in rep["p521"] or "unsupported" in rep["p521"].lower(), rep


def test_idlharness_inheritance_chain_high_signal():
    """Q053/Q058: HTMLDivElement -> ... -> EventTarget chain complete."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var d = document.createElement('div');
                  var chain = [];
                  var p = Object.getPrototypeOf(d);
                  for (var i = 0; i < 8 && p; i++) {
                    chain.push(p.constructor && p.constructor.name);
                    p = Object.getPrototypeOf(p);
                  }
                  return JSON.stringify(chain);
                })()
                """
            )
        )

    chain = json.loads(_run(body))
    for name in ("HTMLDivElement", "HTMLElement", "Element", "Node", "EventTarget"):
        assert name in chain, chain


def test_idlharness_writable_method_on_event_target():
    """Q052: EventTarget.prototype.addEventListener is writable data property."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var d = Object.getOwnPropertyDescriptor(EventTarget.prototype, 'addEventListener');
                  return JSON.stringify({
                    type: typeof (d && d.value),
                    writable: !!(d && d.writable),
                    configurable: !!(d && d.configurable)
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["type"] == "function" and rep["writable"] is True, rep


def test_canvas_gradient_paint_style_roundtrip():
    """Q110: createLinearGradient + addColorStop + fillStyle assignment."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var c = document.createElement('canvas');
                  var g = c.getContext('2d');
                  var lg = g.createLinearGradient(0, 0, 1, 1);
                  lg.addColorStop(0, '#000');
                  lg.addColorStop(1, '#fff');
                  g.fillStyle = lg;
                  return JSON.stringify({
                    tag: Object.prototype.toString.call(lg),
                    fill: Object.prototype.toString.call(g.fillStyle)
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert "CanvasGradient" in rep["tag"] and "CanvasGradient" in rep["fill"], rep


def test_storage_persist_and_load_roundtrip():
    """Q120: persist_storage/load_storage optional file path."""

    import os
    import tempfile

    def body():
        ctx = iv8_rs.JSContext()
        ctx.eval("localStorage.setItem('q120', 'v');")
        path = os.path.join(tempfile.gettempdir(), "iv8_q120_storage.json")
        try:
            ctx.persist_storage(path)
            ctx2 = iv8_rs.JSContext()
            ctx2.load_storage(path)
            return str(ctx2.eval("localStorage.getItem('q120')"))
        finally:
            try:
                os.remove(path)
            except OSError:
                pass

    assert _run(body) == "v"


def test_window_global_props_own_not_on_window_prototype():
    """Q055: navigator/document/location are own Window properties (not Window.prototype)."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  function place(name){
                    var own = Object.getOwnPropertyDescriptor(window, name);
                    var onProto = Object.getOwnPropertyDescriptor(Window.prototype, name);
                    return {name:name, own:!!own, onProto:!!onProto};
                  }
                  return JSON.stringify(['navigator','document','location'].map(place));
                })()
                """
            )
        )

    rows = json.loads(_run(body))
    for r in rows:
        assert r["own"] is True and r["onProto"] is False, r


def test_navigator_user_agent_getter_call_illegal_invocation():
    """Q063: wrong-this on Navigator.prototype.userAgent getter throws Illegal invocation."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var g = Object.getOwnPropertyDescriptor(Navigator.prototype, 'userAgent').get;
                  var ok = g.call(navigator);
                  try { g.call({}); return JSON.stringify({ok:ok, wrong:'no-throw'}); }
                  catch(e){
                    return JSON.stringify({
                      ok:ok,
                      wrong: e.name === 'TypeError' && /Illegal invocation/i.test(String(e.message))
                    });
                  }
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert isinstance(rep["ok"], str) and len(rep["ok"]) > 10, rep
    assert rep["wrong"] is True, rep


def test_worker_constructor_and_postmessage_surface():
    """Q100/Q102: Worker constructor + postMessage surface."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  if (typeof Worker !== 'function') return JSON.stringify({missing:true});
                  var w = new Worker('data:text/javascript,0');
                  return JSON.stringify({
                    type: typeof w,
                    post: typeof w.postMessage,
                    terminate: typeof w.terminate,
                    onmessage: 'onmessage' in w
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep.get("missing") is not True, rep
    assert rep["type"] == "object" and rep["post"] == "function", rep


def test_worker_navigator_profile_and_message_roundtrip():
    """Q100/Q101: WorkerNavigator profile fields + postMessage roundtrip (H06b core)."""

    def body():
        import time

        ctx = iv8_rs.JSContext()
        main_ua = str(ctx.eval("navigator.userAgent"))
        main_plat = str(ctx.eval("navigator.platform"))
        src = (
            "postMessage(JSON.stringify({"
            "ua:navigator.userAgent,"
            "plat:navigator.platform,"
            "ctor:navigator.constructor&&navigator.constructor.name"
            "}))"
        )
        ctx.add_resource("/q100_worker.js", src.encode(), 200, {"Content-Type": "text/javascript"})
        ctx.eval(
            """
            globalThis.__q100=[];
            var w=new Worker('/q100_worker.js');
            w.onmessage=function(e){
              var d=e.data;
              globalThis.__q100.push(typeof d==='string'?d:JSON.stringify(d));
            };
            """
        )
        for _ in range(40):
            ctx.eval("void 0")
            raw = str(ctx.eval("JSON.stringify(globalThis.__q100)"))
            if raw != "[]":
                return json.dumps(
                    {
                        "main_ua": main_ua,
                        "main_plat": main_plat,
                        "msgs": json.loads(raw),
                    }
                )
            time.sleep(0.05)
        return json.dumps({"main_ua": main_ua, "main_plat": main_plat, "msgs": []})

    rep = json.loads(_run(body))
    assert rep["msgs"], rep
    worker = json.loads(rep["msgs"][0])
    assert worker.get("ctor") == "WorkerNavigator", worker
    assert worker.get("ua") == rep["main_ua"], (worker, rep["main_ua"])
    assert worker.get("plat") == rep["main_plat"], (worker, rep["main_plat"])


def test_document_all_htmlallcollection_exotic_basics():
    """Q035: document.all typeof undefined + ToBoolean false (HTMLAllCollection residual)."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                JSON.stringify({
                  t: typeof document.all,
                  eqU: document.all == undefined,
                  bool: !!document.all
                })
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["t"] == "undefined", rep
    assert rep["eqU"] is True, rep
    assert rep["bool"] is False, rep


def test_storage_buckets_surface_open_callable():
    """Q123: navigator.storageBuckets surface (codegen depth residual)."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                JSON.stringify({
                  sb: typeof navigator.storageBuckets,
                  open: typeof (navigator.storageBuckets && navigator.storageBuckets.open),
                  storage: typeof navigator.storage
                })
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["sb"] == "object", rep
    assert rep["open"] == "function", rep


def test_high_signal_interfaces_are_non_constructable():
    """Q056: Navigator/Screen/History/Location throw on new."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  return JSON.stringify(['Navigator','Screen','History','Location'].map(function(n){
                    try { new globalThis[n](); return {n:n, ok:false}; }
                    catch(e){ return {n:n, ok: e.name === 'TypeError'}; }
                  }));
                })()
                """
            )
        )

    rows = json.loads(_run(body))
    bad = [r for r in rows if not r.get("ok")]
    assert bad == [], bad


def test_xhr_readystate_sequence_1_2_3_4_with_handler():
    """Q090: async XHR readyState sequence with network handler + eval_promise drain."""

    def body():
        ctx = iv8_rs.JSContext()
        ctx.set_network_handler(lambda url, method: (200, b"ok"))
        ctx.add_resource("/q090", b"ok", 200, {"Content-Type": "text/plain"})
        return str(
            ctx.eval_promise(
                r"""
                new Promise(function(resolve){
                  var states = [];
                  var x = new XMLHttpRequest();
                  x.onreadystatechange = function(){
                    states.push(x.readyState);
                    if (x.readyState === 4) {
                      resolve(JSON.stringify({states: states, status: x.status, text: x.responseText}));
                    }
                  };
                  x.open('GET', '/q090');
                  x.send();
                })
                """,
                200,
            )
        )

    rep = json.loads(_run(body))
    assert rep["states"] == [1, 2, 3, 4], rep
    assert rep["status"] == 200, rep
    assert rep["text"] == "ok", rep


def test_websocket_open_close_lifecycle_stub():
    """Q092: WebSocket constructor + readyState transitions (stub path)."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  if (typeof WebSocket === 'undefined') return JSON.stringify({missing:true});
                  var ws = new WebSocket('wss://example.invalid/q092');
                  var openState = ws.readyState;
                  var closed = false;
                  try { ws.close(); closed = true; } catch(e) {}
                  return JSON.stringify({
                    openState: openState,
                    afterClose: ws.readyState,
                    closed: closed,
                    CONNECTING: WebSocket.CONNECTING,
                    OPEN: WebSocket.OPEN,
                    CLOSING: WebSocket.CLOSING,
                    CLOSED: WebSocket.CLOSED
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep.get("missing") is not True, rep
    assert rep["CONNECTING"] == 0 and rep["OPEN"] == 1, rep
    assert rep["closed"] is True, rep


def test_window_and_location_getter_names_use_get_prefix():
    """Q012: high-signal accessor getters named get <prop>."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                JSON.stringify({
                  innerWidth: Object.getOwnPropertyDescriptor(window, 'innerWidth').get.name,
                  href: Object.getOwnPropertyDescriptor(Location.prototype, 'href').get.name,
                  userAgent: Object.getOwnPropertyDescriptor(Navigator.prototype, 'userAgent').get.name
                })
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["innerWidth"] == "get innerWidth", rep
    assert rep["href"] == "get href", rep
    assert rep["userAgent"] == "get userAgent", rep


def test_mutation_observer_fires_on_attribute_and_childlist():
    """Q074: shallow MutationObserver delivers records for observed nodes."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var seen = 0;
                  var types = [];
                  var mo = new MutationObserver(function(recs){
                    seen += recs.length;
                    recs.forEach(function(r){ types.push(r.type); });
                  });
                  var el = document.createElement('div');
                  document.body.appendChild(el);
                  mo.observe(el, {attributes: true, childList: true});
                  el.setAttribute('data-q', '1');
                  el.appendChild(document.createTextNode('t'));
                  return JSON.stringify({seen: seen, types: types});
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["seen"] >= 2, rep
    assert "attributes" in rep["types"] and "childList" in rep["types"], rep


def test_url_search_params_basic():
    """Q122: URLSearchParams get/has basics."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var u = new URL('https://ex.test/p?a=1&b=2');
                  var sp = u.searchParams;
                  return JSON.stringify({a: sp.get('a'), hasB: sp.has('b'), s: String(sp)});
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["a"] == "1", rep
    assert rep["hasB"] is True, rep


def test_document_cookie_set_get_roundtrip():
    """Q093: document.cookie set/get basic path (attribute storage residual)."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  document.cookie = 'q093=v; Path=/; SameSite=Lax';
                  return document.cookie;
                })()
                """
            )
        )

    cookie = _run(body)
    assert "q093=v" in cookie, cookie


def test_event_istrusted_on_dispatch():
    """Q078: isTrusted true for dispatchEvent, false for new Event."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var e = new Event('x');
                  var trusted = null;
                  document.addEventListener('q078', function(ev){ trusted = ev.isTrusted; });
                  document.dispatchEvent(new Event('q078'));
                  var d = Object.getOwnPropertyDescriptor(Event.prototype, 'isTrusted');
                  return JSON.stringify({
                    constructed: e.isTrusted,
                    dispatched: trusted,
                    isAccessor: !!(d && d.get)
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["constructed"] is False, rep
    assert rep["dispatched"] is True, rep
    assert rep["isAccessor"] is True, rep


def test_xpath_history_storage_basics():
    """Q060: XPathResult + history + storage basic surface."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var r = document.evaluate('//div', document, null, XPathResult.ORDERED_NODE_SNAPSHOT_TYPE, null);
                  localStorage.setItem('q060', '1');
                  return JSON.stringify({
                    xpathCtor: r && r.constructor && r.constructor.name,
                    snap: r && r.snapshotLength,
                    historyPush: typeof history.pushState,
                    ls: localStorage.getItem('q060')
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["xpathCtor"] == "XPathResult", rep
    assert rep["historyPush"] == "function", rep
    assert rep["ls"] == "1", rep


def test_postmessage_and_scroll_receiver_and_argc():
    """Q059: postMessage argc + scrollTo/scrollBy wrong-this Illegal invocation."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  function t(label, fn, wantIllegal) {
                    try { fn(); return {label:label, ok:false, msg:'no-throw'}; }
                    catch(e) {
                      var msg = String(e.message);
                      var ok = wantIllegal
                        ? (e.name === 'TypeError' && /Illegal invocation/i.test(msg))
                        : (e.name === 'TypeError' && /argument/i.test(msg));
                      return {label:label, ok:ok, msg:msg.slice(0,90)};
                    }
                  }
                  return JSON.stringify([
                    t('pm-argc', function(){ postMessage(); }, false),
                    t('pm-this', function(){ Window.prototype.postMessage.call({}, 'x'); }, true),
                    t('scrollTo-this', function(){ window.scrollTo.call({}, 0, 0); }, true),
                    t('scrollBy-this', function(){ window.scrollBy.call({}, 0, 0); }, true)
                  ]);
                })()
                """
            )
        )

    rows = json.loads(_run(body))
    bad = [r for r in rows if not r.get("ok")]
    assert bad == [], bad


def test_high_signal_method_name_length_shapes():
    """Q051: high-signal method name/length after NAME_LENGTH residual fix."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                JSON.stringify({
                  postMessage: [postMessage.name, postMessage.length],
                  getContext: [HTMLCanvasElement.prototype.getContext.name, HTMLCanvasElement.prototype.getContext.length],
                  toDataURL: [HTMLCanvasElement.prototype.toDataURL.name, HTMLCanvasElement.prototype.toDataURL.length],
                  setTransform: [CanvasRenderingContext2D.prototype.setTransform.name, CanvasRenderingContext2D.prototype.setTransform.length],
                  initEvent: [Event.prototype.initEvent.name, Event.prototype.initEvent.length],
                  createElement: [document.createElement.name, document.createElement.length]
                })
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["postMessage"] == ["postMessage", 1], rep
    assert rep["getContext"] == ["getContext", 1], rep
    assert rep["toDataURL"] == ["toDataURL", 0], rep
    assert rep["setTransform"] == ["setTransform", 0], rep
    assert rep["initEvent"] == ["initEvent", 1], rep
    assert rep["createElement"] == ["createElement", 1], rep


def test_navigator_connection_and_plugins_shape():
    """Q033/Q034: connection values + plugins/mimeTypes arrays."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var c = navigator.connection;
                  var p = navigator.plugins;
                  var m = navigator.mimeTypes;
                  return JSON.stringify({
                    tag: Object.prototype.toString.call(c),
                    rtt: c.rtt,
                    downlink: c.downlink,
                    effectiveType: c.effectiveType,
                    saveData: c.saveData,
                    pluginsLen: p.length,
                    pluginsInst: p instanceof PluginArray,
                    mimeLen: m.length,
                    mimeInst: m instanceof MimeTypeArray
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["tag"] == "[object NetworkInformation]", rep
    assert rep["rtt"] >= 0 and rep["downlink"] > 0, rep
    assert rep["effectiveType"] in ("4g", "3g", "2g", "slow-2g"), rep
    assert rep["pluginsLen"] >= 1 and rep["pluginsInst"] is True, rep
    assert rep["mimeLen"] >= 1 and rep["mimeInst"] is True, rep


def test_document_node_methods_not_own_on_document():
    """Q037: Node methods live on Node.prototype, not document own."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var methods=['appendChild','removeChild','insertBefore','cloneNode'];
                  var bad=[];
                  methods.forEach(function(m){
                    if (Object.prototype.hasOwnProperty.call(document, m)) bad.push(m+':own');
                    if (typeof Node.prototype[m] !== 'function') bad.push(m+':missing-proto');
                  });
                  return JSON.stringify(bad);
                })()
                """
            )
        )

    bad = json.loads(_run(body))
    assert bad == [], bad


def test_high_signal_methods_throw_illegal_invocation_on_wrong_this():
    """Q050: high-signal DOM/XHR methods reject wrong this with Illegal invocation."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  function check(label, fn) {
                    try { fn.call({}); return {label: label, ok: false, msg: 'no-throw'}; }
                    catch(e) {
                      return {
                        label: label,
                        ok: e.name === 'TypeError' && /Illegal invocation/i.test(String(e.message)),
                        msg: String(e.message).slice(0, 80)
                      };
                    }
                  }
                  return JSON.stringify([
                    check('appendChild', Node.prototype.appendChild),
                    check('getAttribute', Element.prototype.getAttribute),
                    check('createElement', document.createElement),
                    check('click', HTMLElement.prototype.click),
                    check('xhr.open', XMLHttpRequest.prototype.open),
                    check('ael', function(){ return EventTarget.prototype.addEventListener.call({}, 'x', function(){}); })
                  ]);
                })()
                """
            )
        )

    rows = json.loads(_run(body))
    bad = [r for r in rows if not r.get("ok")]
    assert bad == [], bad


def test_user_agent_data_proto_shape_and_grease_brands():
    """Q030/Q031: brands GREASE + NavigatorUAData prototype accessors."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var uad = navigator.userAgentData;
                  var brands = (uad.brands || []).map(function(b){ return b.brand; });
                  return JSON.stringify({
                    instOf: uad instanceof NavigatorUAData,
                    ownKeys: Object.keys(uad),
                    protoBrands: !!Object.getOwnPropertyDescriptor(Object.getPrototypeOf(uad), 'brands'),
                    brands: brands,
                    hasGrease: brands.some(function(b){ return String(b).indexOf('Not') >= 0 || String(b).indexOf('Brand') >= 0; }),
                    hasChrome: brands.indexOf('Google Chrome') >= 0,
                    hasChromium: brands.indexOf('Chromium') >= 0
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["instOf"] is True, rep
    assert rep["ownKeys"] == [], rep
    assert rep["protoBrands"] is True, rep
    assert rep["hasChrome"] is True, rep
    assert rep["hasChromium"] is True, rep
    assert rep["hasGrease"] is True, rep


def test_default_profile_high_signal_value_coherence():
    """Q032: default profile Window/Nav/Screen high-signal keys are populated and sane."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  return JSON.stringify({
                    platform: navigator.platform,
                    language: navigator.language,
                    hw: navigator.hardwareConcurrency,
                    mem: navigator.deviceMemory,
                    sw: screen.width,
                    sh: screen.height,
                    iw: innerWidth,
                    ih: innerHeight,
                    dpr: devicePixelRatio,
                    uaWin: navigator.userAgent.indexOf('Windows') >= 0
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["platform"] in ("Win32", "Win64", "MacIntel", "Linux x86_64"), rep
    assert isinstance(rep["language"], str) and len(rep["language"]) >= 2, rep
    assert rep["hw"] >= 1, rep
    assert rep["sw"] >= 800 and rep["sh"] >= 600, rep
    assert rep["iw"] >= 1 and rep["ih"] >= 1, rep
    assert rep["dpr"] > 0, rep
    assert rep["uaWin"] is True or "Mac" in rep["platform"] or "Linux" in rep["platform"], rep


def test_get_computed_style_active_text_system_color():
    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var el = document.createElement('div');
                  document.body.appendChild(el);
                  var cs = getComputedStyle(el);
                  var v = cs.getPropertyValue('ActiveText') || cs.ActiveText || '';
                  return JSON.stringify({
                    activeText: v,
                    nonEmpty: String(v).length > 0,
                    linkText: cs.getPropertyValue('LinkText') || cs.LinkText || ''
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["nonEmpty"] is True, rep
    assert "rgb" in rep["activeText"].lower() or rep["activeText"].startswith("#"), rep


def test_document_create_element_tostring_is_native_code():
    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var methods = ['createElement','createElementNS','createTextNode',
                    'getElementById','querySelector'];
                  var bad = [];
                  methods.forEach(function(m){
                    var s = String(document[m].toString());
                    if (s.indexOf('[native code]') < 0) bad.push(m + ':' + s.slice(0,80));
                    if (s.indexOf('{') >= 0 && s.indexOf('[native code]') < 0) bad.push(m + ':src');
                  });
                  return JSON.stringify(bad);
                })()
                """
            )
        )

    bad = json.loads(_run(body))
    assert bad == [], bad


def test_error_stack_rewrites_anonymous_to_eval():
    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  try { throw new Error('probe'); } catch(e) {
                    return String(e.stack);
                  }
                })()
                """
            )
        )

    stack = _run(body)
    assert "at eval:" in stack or "at eval (" in stack, stack
    assert "at <anonymous>:" not in stack, stack


def test_chrome_runtime_methods_on_prototype_not_object_keys():
    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var keys = Object.keys(chrome.runtime);
                  var hasConnectOwn = Object.prototype.hasOwnProperty.call(chrome.runtime, 'connect');
                  var hasSendOwn = Object.prototype.hasOwnProperty.call(chrome.runtime, 'sendMessage');
                  var proto = Object.getPrototypeOf(chrome.runtime);
                  return JSON.stringify({
                    keys: keys,
                    hasConnectOwn: hasConnectOwn,
                    hasSendOwn: hasSendOwn,
                    protoHasConnect: proto && typeof proto.connect === 'function',
                    protoHasSend: proto && typeof proto.sendMessage === 'function',
                    connectNative: /\[native code\]/.test(String(chrome.runtime.connect)),
                    enumsOk: keys.indexOf('connect') < 0 && keys.indexOf('sendMessage') < 0
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["enumsOk"] is True, rep
    assert rep["hasConnectOwn"] is False, rep
    assert rep["hasSendOwn"] is False, rep
    assert rep["protoHasConnect"] is True, rep
    assert rep["protoHasSend"] is True, rep
    assert rep["connectNative"] is True, rep


def test_window_iv8_internal_keys_not_in_object_keys():
    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var keys = Object.keys(window).filter(function(k){
                    return k.indexOf('__iv8') === 0
                      || k.indexOf('__webgl') === 0
                      || k.indexOf('__canvas') === 0
                      || k.indexOf('__xhr') === 0;
                  });
                  var forIn = [];
                  for (var k in window) {
                    if (String(k).indexOf('__iv8') === 0) forIn.push(k);
                  }
                  // still readable for shims
                  var readable = typeof globalThis.__iv8MediaPrefs !== 'undefined'
                    || typeof globalThis.__iv8NavInst__ !== 'undefined'
                    || typeof window.__iv8DumpLocalStorage === 'function'
                    || typeof globalThis.__iv8EventShimInstalled === 'boolean';
                  return JSON.stringify({
                    objectKeys: keys,
                    forIn: forIn,
                    readable: readable
                  });
                })()
                """
            )
        )

    rep = json.loads(_run(body))
    assert rep["objectKeys"] == [], rep
    assert rep["forIn"] == [], rep
