"""v0.8.50 RS补環境 attempt: run RS example code in IV8."""
from __future__ import annotations
import sys, json, time, re, io
from pathlib import Path

# Force UTF-8 output
sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8')

# ── Setup paths ──
ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

# ── Download example files if not present ──
import urllib.request

EXAMPLE_DIR = ROOT / "_ref" / "rs-reverse" / "example" / "codes"
EXAMPLE_DIR.mkdir(parents=True, exist_ok=True)

MAIN_JS = EXAMPLE_DIR / "main.js"
TS_JSON = EXAMPLE_DIR / "$_ts.json"

if not MAIN_JS.exists():
    print("Downloading main.js...")
    urllib.request.urlretrieve(
        "https://raw.githubusercontent.com/pysunday/rs-reverse/main/example/codes/main.js",
        MAIN_JS,
    )
    print(f"  -> {MAIN_JS.stat().st_size} bytes")

if not TS_JSON.exists():
    print("Downloading $_ts.json...")
    urllib.request.urlretrieve(
        "https://raw.githubusercontent.com/pysunday/rs-reverse/main/example/codes/%24_ts.json",
        TS_JSON,
    )
    print(f"  -> {TS_JSON.stat().st_size} bytes")

# ── Load files ──
ts_data = json.loads(TS_JSON.read_text(encoding="utf-8"))
main_js = MAIN_JS.read_text(encoding="utf-8")
print(f"\nmain.js: {len(main_js)} chars, ~{len(main_js)//1024}KB")
print(f"$_ts.nsd: {ts_data['nsd']}")
print(f"$_ts.cd:  {ts_data['cd'][:60]}...")

# ── Build RS补環境 in IV8 ──
print("\n=== Building IV8补環境 for 瑞数 ===")

import iv8_rs

ctx = iv8_rs.JSContext()

# Step 1: Set up $_ts
setup_ts = f"""
window.$_ts = {{
    nsd: {ts_data['nsd']},
    cd: "{ts_data['cd']}",
}};
// Verify setup
JSON.stringify({{nsdType: typeof window.$_ts.nsd, cdType: typeof window.$_ts.cd, nsdVal: window.$_ts.nsd}});
"""
verify = ctx.eval(setup_ts)
print(f"[$_ts] verify: {verify}")

# Step 1b: Set up window self-references (RS checks these)
ctx.eval("""
window.top = window;
window.self = window;
window.parent = window;
""")
print("[window refs] self/top/parent = window OK")

# Step 1c: Set document.location
ctx.eval("""
document.location = location;
""")
print("[document.location] set OK")

# Step 2: Hook document.cookie to capture
ctx.eval("""
window.__rs_cookies = [];
(function() {
    var _origDesc = Object.getOwnPropertyDescriptor(Document.prototype, 'cookie') || 
                    Object.getOwnPropertyDescriptor(HTMLDocument.prototype, 'cookie');
    var _cookieVal = '';
    Object.defineProperty(document, 'cookie', {
        get: function() { return _cookieVal; },
        set: function(val) {
            window.__rs_cookies.push(val);
            _cookieVal = val;
        },
        configurable: true,
        enumerable: true
    });
})();
""")
print("[cookie hook] installed OK")

# Step 3: Set up location (RS often checks this)
ctx.eval("""
location.href = 'http://epub.cnipa.gov.cn/';
location.host = 'epub.cnipa.gov.cn';
location.hostname = 'epub.cnipa.gov.cn';
location.protocol = 'http:';
location.pathname = '/';
""")
print("[location] set OK")

# Step 4: Try to execute the RS main.js
print("\n=== Executing RS main.js ===")
t0 = time.time()

try:
    result = ctx.eval(main_js)
    elapsed = time.time() - t0
    print(f"Execution OK in {elapsed:.2f}s")
    print(f"Result: {result}")
except Exception as e:
    elapsed = time.time() - t0
    err = str(e)
    print(f"Execution FAILED in {elapsed:.2f}s")
    print(f"Error: {type(e).__name__}")
    print(f"Message: {err[:500]}")

    # Extract specific errors
    if "is not defined" in err:
        # Find what's missing
        import re
        missing = re.findall(r"(\w+) is not defined", err)
        for m in missing[:10]:
            print(f"  MISSING: {m}")

# Step 5: Check cookie output
cookies = ctx.eval("JSON.stringify(window.__rs_cookies)")
print(f"\nCaptured cookies: {cookies}")

# Step 6: Environment diagnostic
print("\n=== Environment Diagnostic ===")
env_checks = [
    "typeof window",
    "typeof document",
    "typeof navigator",
    "typeof location",
    "typeof screen",
    "typeof window.$_ts",
    "window.$_ts.nsd",
    "typeof window.$_ts.cd",
    "typeof document.cookie",
    "typeof document.createElement",
    "typeof document.getElementsByTagName",
    "typeof document.getElementById",
    "typeof document.addEventListener",
    "typeof window.addEventListener",
    "typeof navigator.userAgent",
    "typeof window.top",
]
for check in env_checks:
    try:
        v = ctx.eval(check)
        vs = str(v)
        if len(vs) > 80:
            vs = vs[:80] + "..."
        print(f"  {check:45s} = {vs}")
    except Exception as e:
        print(f"  {check:45s} = ERROR: {type(e).__name__}")
