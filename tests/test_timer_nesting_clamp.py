"""Q082: HTML nested setTimeout clamping (nesting level > 5 → min 4ms)."""

from __future__ import annotations

import threading

import pytest

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


def test_nested_settimeout_zero_clamped_after_depth_six():
    """Chain of setTimeout(0): levels 1–5 may be 0ms; level 6+ due >= +4ms logical."""

    def body():
        ctx = iv8_rs.JSContext()
        # Record logical time at each nesting fire via performance.now() after advances.
        # Build chain: each timeout schedules the next with delay 0.
        src = r"""
        (function(){
          var times = [];
          function step(n) {
            times.push({n:n, t: performance.now()});
            if (n < 8) {
              setTimeout(function(){ step(n+1); }, 0);
            }
          }
          setTimeout(function(){ step(1); }, 0);
          // Drain enough logical time for unclamped zeros + clamped 4ms steps.
          for (var i = 0; i < 20; i++) {
            __iv8__.eventLoop.advance(1);
          }
          return JSON.stringify(times);
        })()
        """
        return str(ctx.eval(src))

    times = __import__("json").loads(_run(body))
    assert len(times) >= 7, times
    # After nesting level 6, successive 0ms timeouts should not all share the same
    # timestamp if clamp applies (4ms). Levels 1–5 can fire in the same advance burst.
    # Find first time where n>=6 and compare delta to previous.
    by_n = {row["n"]: row["t"] for row in times}
    # Level 6 scheduled while level 5 ran: parent nesting 5 → task level 6 → clamp.
    # So t[6] - t[5] should be >= 4 (logical ms) when only advancing 1ms steps.
    if 5 in by_n and 6 in by_n:
        assert by_n[6] - by_n[5] >= 3.5, times
    if 6 in by_n and 7 in by_n:
        assert by_n[7] - by_n[6] >= 3.5, times


def test_top_level_settimeout_zero_not_forced_to_four():
    """Top-level setTimeout(0) must not be clamped to 4ms."""

    def body():
        ctx = iv8_rs.JSContext()
        return str(
            ctx.eval(
                r"""
                (function(){
                  var t0 = performance.now();
                  var fired = -1;
                  setTimeout(function(){ fired = performance.now() - t0; }, 0);
                  __iv8__.eventLoop.advance(1);
                  return JSON.stringify({fired: fired});
                })()
                """
            )
        )

    rep = __import__("json").loads(_run(body))
    assert rep["fired"] >= 0, rep
    assert rep["fired"] < 4.0, rep
