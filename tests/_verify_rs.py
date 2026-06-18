"""Verify RS runs without manual top/self/parent after Phase A fix."""
import json, time
from pathlib import Path

ROOT = Path(r"D:\dogepy\Tools\IV8")
RS = ROOT / "_ref" / "rs-reverse"
ts = json.loads((RS / "_rs_live_ts.json").read_text("utf-8"))
live_js = (RS / "_rs_live_js.js").read_text("utf-8")

import iv8_rs
ctx = iv8_rs.JSContext()

# $_ts setup
ctx.eval(f"window.$_ts = {{ nsd: {ts['nsd']}, cd: \"{ts['cd']}\" }};")

# Location setup
ctx.eval("location.href='http://epub.cnipa.gov.cn/'; location.host='epub.cnipa.gov.cn'; location.hostname='epub.cnipa.gov.cn'; location.protocol='http:'; location.pathname='/';")

# Cookie hook
ctx.eval("""
window.__c=[];
(function(){var v='';Object.defineProperty(document,'cookie',{get:function(){return v},set:function(x){window.__c.push(x);v=x}});})();
""")

# NO manual top/self/parent — relying on new defaults!
t0 = time.time()
ctx.eval(live_js)
elapsed = time.time() - t0

cookies = ctx.eval("JSON.stringify(window.__c)")

print(f"Execution: {elapsed:.2f}s")
print(f"Cookie: {cookies[:150]}...")
print(f"\nNo manual top/self/parent needed:")
print(f"  top===window:    {ctx.eval('window.top===window')}")
print(f"  self===window:   {ctx.eval('window.self===window')}")
print(f"  parent===window: {ctx.eval('window.parent===window')}")
print(f"\nPhase A verification:")
print(f"  plugins.length:  {ctx.eval('navigator.plugins.length')}")
print(f"  mimeTypes.length:{ctx.eval('navigator.mimeTypes.length')}")
