"""Regression: relative-URL fetch must appear in __iv8__.netLog.entries.

Re-verify of D-NETLOG-01 (2026-07-15): earlier whitebox misread; simple and
cap7-shaped relative POST both record method/url/body.
"""

import json
import threading

threading.stack_size(128 * 1024 * 1024)


def test_relative_fetch_post_recorded_in_netlog():
    import iv8_rs

    box = {}

    def work():
        ctx = iv8_rs.JSContext()
        ctx.set_network_handler(lambda url, method: (200, b'{"ok":true}'))
        ctx.eval(
            """
            globalThis.__done = 0;
            fetch('/api/rel', {method: 'POST', body: 'payload-1',
              headers: {'content-type': 'text/plain'}})
              .then(function(){ globalThis.__done = 1; })
              .catch(function(){ globalThis.__done = 2; });
            """
        )
        for _ in range(40):
            if hasattr(ctx, "run_microtasks"):
                try:
                    ctx.run_microtasks()
                except Exception:
                    pass
            if ctx.eval("globalThis.__done"):
                break
        raw = ctx.eval("JSON.stringify(__iv8__.netLog.entries)")
        box["entries"] = json.loads(str(raw))

    t = threading.Thread(target=work)
    t.start()
    t.join()
    entries = box.get("entries") or []
    assert entries, "netLog.entries empty for relative fetch"
    hit = [e for e in entries if e.get("url") == "/api/rel"]
    assert hit, entries
    assert hit[0].get("method") == "POST"
    assert hit[0].get("body") == "payload-1"
