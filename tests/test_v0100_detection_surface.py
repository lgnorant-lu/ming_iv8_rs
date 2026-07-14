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
