"""v0.8.50 Multi-sample RS补環境 diagnostic."""
from __future__ import annotations
import sys, json, time, io
from pathlib import Path

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')
sys.stderr = io.TextIOWrapper(sys.stderr.buffer, encoding='utf-8')

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

RS_DIR = ROOT / "_ref" / "rs-reverse" / "example" / "codes"
OUTDIR = ROOT / "_ref" / "rs-samples"
OUTDIR.mkdir(parents=True, exist_ok=True)

def safe_eval(ctx, expr):
    try:
        return str(ctx.eval(expr))
    except Exception as e:
        es = str(e)
        if len(es) > 150:
            es = es[:150] + "..."
        return f"ERROR: {es}"

def run_sample(sample_id, js_file, ts_file, extra_setup=""):
    """Run a single RS sample in IV8 and return results."""
    print(f"\n{'='*60}")
    print(f"  Sample: {sample_id}")
    print(f"{'='*60}")

    js_src = Path(js_file).read_text(encoding="utf-8")
    ts_data = json.loads(Path(ts_file).read_text(encoding="utf-8"))

    print(f"  JS size:  ~{len(js_src)//1024}KB")
    print(f"  ts keys:  {list(ts_data.keys())[:8]}")
    print(f"  ts.nsd:   {ts_data.get('nsd','N/A')}")

    import iv8_rs
    ctx = iv8_rs.JSContext()

    # Set up $_ts FIRST (RS checks it immediately)
    if 'nsd' in ts_data:
        nsd = ts_data['nsd']
        cd = ts_data.get('cd', '')
        ctx.eval(f"window.$_ts = {{ nsd: {nsd}, cd: \"{cd}\" }};")
    else:
        # $_ts-full format: cp, jf, scj, aebi, cd
        ts_json = json.dumps(ts_data)
        ctx.eval(f"window.$_ts = {ts_json};")

    ctx.eval("location.href = 'http://epub.cnipa.gov.cn/'; location.host = 'epub.cnipa.gov.cn'; location.hostname = 'epub.cnipa.gov.cn'; location.protocol = 'http:'; location.pathname = '/';")
    ctx.eval("window.top = window; window.self = window; window.parent = window; document.location = location;")

    # Cookie hook AFTER $_ts and location
    ctx.eval("""
window.__rs_cookies = [];
(function() {
    var _cv = '';
    Object.defineProperty(document, 'cookie', {
        get: function() { return _cv; },
        set: function(val) { window.__rs_cookies.push(val); _cv = val; },
        configurable: true, enumerable: true
    });
})();
    """)

    t0 = time.time()
    try:
        result = ctx.eval(js_src)
        elapsed = time.time() - t0
        status = "OK"
        error = None
        errors = []
    except Exception as e:
        elapsed = time.time() - t0
        status = "FAILED"
        error = str(e)
        # Extract error patterns
        import re
        errors = re.findall(r"(\w+) is not defined|Cannot read.*?undefined.*?reading '(\w+)'|(\w+) is not a function", error)
        errors = [e for e in errors if any(e)]

    # Check cookies and env
    cookies = safe_eval(ctx, "JSON.stringify(window.__rs_cookies)")

    env_probes = {}
    for p in ['typeof window.top','typeof window.self','typeof document.cookie',
              'typeof navigator.userAgent','typeof screen','typeof document.createElement',
              'typeof document.getElementsByTagName','typeof document.getElementById']:
        env_probes[p] = safe_eval(ctx, p)

    result = {
        "sample_id": sample_id,
        "status": status,
        "elapsed_sec": round(elapsed, 3),
        "js_size_kb": len(js_src)//1024,
        "cookies": cookies,
        "error": error,
        "errors_parsed": errors[:5] if errors else [],
        "env_probes": env_probes,
    }

    print(f"  Result:   {status} ({elapsed:.2f}s)")
    if error:
        print(f"  Error:    {error[:200]}")
    print(f"  Cookies:  {cookies[:120]}...")
    return result

# ── Run samples ──
results = []

# Sample 1: main.js + $_ts.json (original example)
r1 = run_sample("rs-example-main+basic_ts",
    RS_DIR / "main.js",
    RS_DIR / "$_ts.json")
results.append(r1)

# Sample 2: main.js + $_ts-full.json (larger ts data)
r2 = run_sample("rs-example-main+full_ts",
    RS_DIR / "main.js",
    RS_DIR / "$_ts-full.json")
results.append(r2)

# Sample 3: code.js + $_ts-full.json (decoded algorithm)
r3 = run_sample("rs-example-code+full_ts",
    RS_DIR / "code.js",
    RS_DIR / "$_ts-full.json")
results.append(r3)

# ── Summary ──
print(f"\n{'='*60}")
print(f"  Multi-Sample Summary")
print(f"{'='*60}")
for r in results:
    status = r['status']
    print(f"  {r['sample_id']:40s} {status:8s} {r['elapsed_sec']:6.2f}s {r['js_size_kb']:5d}KB")
    if r['errors_parsed']:
        print(f"    Errors: {r['errors_parsed']}")

ok = sum(1 for r in results if r['status'] == 'OK')
print(f"\n  Total: {len(results)} samples, {ok} OK, {len(results)-ok} FAILED")

# ── L2 Projection ──
print(f"\n{'='*60}")
print(f"  L2 Diagnostic Bridge Projection")
print(f"{'='*60}")

from tools.diagnostic_bridge import OWNER_ROUTING_TABLE

print(f"  Routing table entries: {len(OWNER_ROUTING_TABLE)}")
print(f"\n  Environment surface used by RS samples:")
env_surface = [
    ("Navigator", ["userAgent", "platform"], "V001-V009"),
    ("Window", ["top", "self", "parent"], "V015-V021 (iv8-surface)"),
    ("Document", ["cookie", "createElement", "getElementsByTagName"], "V022-V041 (iv8-surface)"),
    ("Location", ["href", "host", "hostname"], "V012-V013 (iv8-core/shims)"),
    ("Screen", ["width", "height"], "V015-V021 (iv8-surface)"),
]
for owner, surfaces, vectors in env_surface:
    print(f"    {owner:20s}: {', '.join(surfaces):50s} -> {vectors}")

# Check which vectors RS touched
rs_probed = ["V001", "V005", "V006", "V015", "V022", "V085", "V012", "V013"]
mapped = [v for v in rs_probed if v in OWNER_ROUTING_TABLE]
unmapped = [v for v in rs_probed if v not in OWNER_ROUTING_TABLE]
print(f"\n  RS-probed vectors in routing table: {len(mapped)}/{len(rs_probed)}")
if unmapped:
    print(f"  Unmapped: {unmapped}")

# Save results
outfile = OUTDIR / "multi_sample_results.json"
with open(outfile, 'w', encoding='utf-8') as f:
    json.dump({r['sample_id']: {k: str(v) if not isinstance(v, (int, float, bool, list, dict, type(None))) else v
                                for k, v in r.items()} for r in results},
              f, indent=2, ensure_ascii=False)
print(f"\n  Results saved to: {outfile}")
