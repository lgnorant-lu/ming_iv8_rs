#!/usr/bin/env python3
"""H06b: Window vs Worker navigator consistency (L5).

Compares a subset of navigator properties between the main window and a
same-process Worker. Worker uses a dedicated isolate (方案A); values should
match the parent profile for fingerprint-visible props.

Usage:
  .venv\\Scripts\\python.exe scripts/evaluate_h06b_window_worker.py
"""
from __future__ import annotations

import json
import sys
import threading
import time
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent
STATUS_DIR = REPO_ROOT / "status"
OUTPUT_PATH = STATUS_DIR / "h06b-window-worker.json"

PROPS = [
    "userAgent",
    "platform",
    "language",
    "hardwareConcurrency",
    "onLine",
]

THRESHOLDS = {
    "max_value_diff": 0,
    "max_throw": 0,
    "min_coverage_pct": 80.0,
}


def _run_in_thread(fn, *args, **kwargs):
    box = [None, None]

    def target():
        try:
            box[0] = fn(*args, **kwargs)
        except Exception as e:
            box[1] = e

    old = threading.stack_size()
    threading.stack_size(128 * 1024 * 1024)
    try:
        t = threading.Thread(target=target)
        t.start()
        t.join(timeout=60)
    finally:
        threading.stack_size(old)
    if box[1]:
        raise box[1]
    return box[0]


def _audit():
    sys.path.insert(0, str(REPO_ROOT))
    from iv8_rs import JSContext

    ctx = JSContext()
    ctx.page_load("<!DOCTYPE html><html><body></body></html>", None)

    # Collect window navigator values
    win_js = (
        "JSON.stringify({"
        + ",".join(f"{p}: navigator.{p}" for p in PROPS)
        + "})"
    )
    win = json.loads(ctx.eval(win_js))

    # Spawn worker that posts navigator snapshot
    worker_src = (
        "self.onmessage=function(){};"
        "self.postMessage({"
        + ",".join(f"{p}: self.navigator.{p}" for p in PROPS)
        + "});"
    )
    # data URL
    import urllib.parse

    data_url = "data:text/javascript," + urllib.parse.quote(worker_src)
    ctx.eval(
        f"""
        globalThis.__h06b = null;
        globalThis.__h06b_err = null;
        try {{
          var w = new Worker({json.dumps(data_url)});
          w.onmessage = function(e) {{ globalThis.__h06b = e.data; }};
          w.onerror = function(e) {{ globalThis.__h06b_err = String(e.message||e); }};
        }} catch(e) {{ globalThis.__h06b_err = String(e); }}
        """
    )

    worker = None
    for _ in range(50):
        # Drain timers / worker messages if API exists
        try:
            ctx.eval(
                "if (typeof eventLoop !== 'undefined') {"
                "  if (eventLoop.tick) eventLoop.tick();"
                "  if (eventLoop.advance) eventLoop.advance(1);"
                "}"
            )
        except Exception:
            pass
        try:
            raw = ctx.eval(
                "globalThis.__h06b_err || (globalThis.__h06b ? JSON.stringify(globalThis.__h06b) : null)"
            )
            if raw and not str(raw).startswith("Error") and raw != "null":
                if str(raw).startswith("{"):
                    worker = json.loads(raw)
                    break
                else:
                    # error string
                    worker = {"__error": str(raw)}
                    break
        except Exception as e:
            worker = {"__error": str(e)}
            break
        time.sleep(0.05)

    ctx.close()

    results = []
    stats = {"PASS": 0, "VALUE_DIFF": 0, "THROW": 0, "SKIP": 0}
    if worker is None:
        for p in PROPS:
            results.append(
                {
                    "property": p,
                    "classification": "SKIP",
                    "detail": "worker message timeout",
                }
            )
            stats["SKIP"] += 1
    elif "__error" in worker:
        for p in PROPS:
            results.append(
                {
                    "property": p,
                    "classification": "THROW",
                    "detail": worker["__error"][:120],
                }
            )
            stats["THROW"] += 1
    else:
        for p in PROPS:
            wv = win.get(p)
            wv2 = worker.get(p)
            if wv == wv2:
                results.append(
                    {
                        "property": p,
                        "classification": "PASS",
                        "window": wv,
                        "worker": wv2,
                    }
                )
                stats["PASS"] += 1
            else:
                results.append(
                    {
                        "property": p,
                        "classification": "VALUE_DIFF",
                        "window": wv,
                        "worker": wv2,
                    }
                )
                stats["VALUE_DIFF"] += 1

    total = len(results)
    coverage = (stats["PASS"] + stats["VALUE_DIFF"]) / max(total, 1) * 100
    cat_a = stats["VALUE_DIFF"] <= THRESHOLDS["max_value_diff"] and stats[
        "THROW"
    ] <= THRESHOLDS["max_throw"]
    cat_d = coverage >= THRESHOLDS["min_coverage_pct"]
    overall = cat_a and cat_d and stats["SKIP"] == 0

    report = {
        "schema_version": "h06b-window-worker.v0.1",
        "timestamp": time.strftime("%Y-%m-%dT%H:%M:%S"),
        "summary": {
            "total": total,
            **stats,
            "coverage_pct": round(coverage, 1),
        },
        "window": win,
        "worker": worker,
        "results": results,
    }
    OUTPUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUTPUT_PATH.write_text(
        json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8"
    )

    print("=" * 60)
    print("H06b Window vs Worker Consistency — Summary")
    print("=" * 60)
    print(f"Total: {total}")
    for k in ["PASS", "VALUE_DIFF", "THROW", "SKIP"]:
        print(f"  {k:12s} {stats.get(k, 0)}")
    print(f"Category A: {'PASS' if cat_a else 'FAIL'}")
    print(f"Category D: {'PASS' if cat_d else 'FAIL'}")
    print(f"OVERALL: {'PASS' if overall else 'FAIL'}")
    if not overall:
        for r in results:
            if r["classification"] != "PASS":
                print(f"  {r['classification']}: {r['property']} — {r.get('detail', r)}")
    return 0 if overall else 1


def main():
    return _run_in_thread(_audit)


if __name__ == "__main__":
    sys.exit(main())
