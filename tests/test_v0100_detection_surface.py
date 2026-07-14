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
