"""Comprehensive tests for probe_environment with monkeypatched dependencies.

Covers all branches: VM-instrumentation mode, recording mode, trace processing,
console error classification, entry_expr failures, and edge cases.
"""

from __future__ import annotations
import warnings
import pytest


# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

class MockEntry:
    """Minimal entry mock with .type, .pc, .target, .value."""
    def __init__(self, type, pc, target, value):
        self.type = type
        self.pc = pc
        self.target = target
        self.value = value


class MockTrace:
    """StructuredTrace stand-in with .reads, .calls, .writes and .summary()."""
    def __init__(self, entries):
        self.entries = entries
        self.reads = [e for e in entries if e.type == "R"]
        self.calls = [e for e in entries if e.type == "C"]
        self.writes = [e for e in entries if e.type == "W"]

    def summary(self):
        return {
            "total": len(self.entries),
            "counts_by_type": {},
            "pc_range": None,
            "unique_targets": len({e.target for e in self.entries}),
            "unique_opcodes": 0,
        }


def _raise(exc):
    raise exc


def make_ctx(console=None, unified=None, recorded=None, eval_side=None):
    """Build a no-op JSContext mock class parameterized per test."""

    class _MockJSContext:
        def __init__(self, environment=None, random_seed=None,
                     time_freeze=None, time_mode="logical"):
            self.env = environment
            self._closed = False

        def start_recording(self, targets=None, limit=None):
            pass

        def eval(self, source):
            if eval_side:
                eval_side(source)

        def get_unified_trace(self):
            return unified

        def stop_recording(self):
            return recorded

        def get_console_messages(self):
            return console or []

        def close(self):
            self._closed = True

    return _MockJSContext


# ===================================================================
# BASIC STRUCTURE (recording mode — instrument_source raises)
# ===================================================================

def test_probe_basic_report_structure():
    """Verify probe_environment returns the expected top-level keys."""
    from iv8_rs import probe_environment

    report = probe_environment(js_source="var x = 1 + 1;", profile=None, random_seed=42)

    assert isinstance(report, dict)
    assert "reads" in report
    assert "calls" in report
    assert "writes" in report
    assert "missing" in report
    assert "errors" in report
    assert "issues" in report
    assert "coverage" in report
    assert "vm_info" in report
    assert "trace_stats" in report


def test_probe_with_profile():
    """Verify loading a profile does not crash."""
    from iv8_rs import probe_environment

    report = probe_environment(js_source="var x = 1 + 1;", profile="default", random_seed=42)
    assert isinstance(report, dict)


def test_probe_coverage_stats_structure():
    """Verify coverage statistics dict has all expected fields."""
    from iv8_rs import probe_environment

    report = probe_environment(js_source="var x = 1 + 1;", profile=None, random_seed=42)
    cov = report["coverage"]
    assert "total_targets" in cov
    assert "configured" in cov
    assert "missing" in cov
    assert "error_count" in cov
    assert "coverage_pct" in cov
    assert 0.0 <= cov["coverage_pct"] <= 100.0


def test_probe_empty_source():
    """Verify empty source produces a valid report."""
    from iv8_rs import probe_environment

    report = probe_environment(js_source="", profile=None, random_seed=42)
    assert isinstance(report["reads"], dict)
    assert isinstance(report["calls"], dict)
    assert isinstance(report["writes"], dict)
    assert isinstance(report["missing"], list)
    assert isinstance(report["errors"], list)
    assert isinstance(report["issues"], list)


def test_probe_with_entry_expr():
    """Verify entry_expr runs without error after main source."""
    from iv8_rs import probe_environment

    report = probe_environment(
        js_source="var result = 0;", entry_expr="1 + 1",
        profile=None, random_seed=42,
    )
    assert isinstance(report, dict)


def test_probe_with_environment_override():
    """Verify environment parameter is accepted."""
    from iv8_rs import probe_environment

    report = probe_environment(
        js_source="var x = 1 + 1;", environment={"screen.width": 1024},
        profile=None, random_seed=42,
    )
    assert isinstance(report, dict)


# ===================================================================
# VM-instrumentation branch (instrument_source succeeds → lines 71-72, 104)
# ===================================================================

def test_probe_vm_mode_basic(monkeypatch):
    """instrument_source succeeds => VM mode => get_unified_trace called."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: ("patched", {"vm": "detected"}))
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: None)

    called = {}

    class MockCtx:
        def __init__(self, **kw):
            called["init"] = kw
        def start_recording(self, **kw):
            called["recording"] = kw
        def eval(self, source):
            called["eval"] = source
        def get_unified_trace(self):
            called["trace"] = True
            return None
        def stop_recording(self):
            called["stop"] = True
            return None
        def get_console_messages(self):
            return []
        def close(self):
            called["closed"] = True

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)

    from iv8_rs import probe_environment
    report = probe_environment("var x;", profile=None)

    # VM mode: no start_recording, uses get_unified_trace
    assert "recording" not in called
    assert called.get("trace") is True
    assert called.get("closed") is True
    assert report["vm_info"] == {"vm": "detected"}


def test_probe_vm_mode_with_profile_and_env(monkeypatch):
    """VM mode with profile + environment merge."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: ("patched", {"vm": True}))
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: None)
    monkeypatch.setattr("iv8_rs.load_profile",
                        lambda p: {"navigator.userAgent": "profile_val"})

    init_kw = {}

    class MockCtx:
        def __init__(self, **kw):
            init_kw.update(kw)
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def get_unified_trace(self): return None
        def stop_recording(self): return None
        def get_console_messages(self): return []
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)

    from iv8_rs import probe_environment
    report = probe_environment(
        "var x;", profile="default",
        environment={"screen.width": 1920},
    )

    # Environment should include profile + override
    env = init_kw.get("environment")
    assert env is not None
    assert env.get("navigator.userAgent") == "profile_val"
    assert env.get("screen.width") == 1920


def test_probe_vm_mode_with_trace_reads(monkeypatch):
    """VM mode: trace reads with undefined/null/NaN produce missing."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: ("patched", {"vm": True}))
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: MockTrace([
        MockEntry("R", 10, "navigator.userAgent", "Chrome"),
        MockEntry("R", 11, "navigator.webdriver", "undefined"),
        MockEntry("R", 12, "screen.width", "1920"),
        MockEntry("R", 13, "navigator.language", "null"),
        MockEntry("R", 14, "some.api", "NaN"),
        MockEntry("R", 15, "empty.val", ""),
    ]))

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def get_unified_trace(self): return ["R,10,navigator.userAgent,Chrome"]
        def stop_recording(self): return None
        def get_console_messages(self): return []
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)

    from iv8_rs import probe_environment
    report = probe_environment("js", profile=None)

    assert report["reads"]["navigator.userAgent"] == "Chrome"
    assert report["reads"]["navigator.webdriver"] == "undefined"
    assert report["reads"]["screen.width"] == "1920"
    assert report["reads"]["navigator.language"] == "null"
    assert report["reads"]["some.api"] == "NaN"
    assert report["reads"]["empty.val"] == ""

    assert sorted(report["missing"]) == ["empty.val", "navigator.language",
                                          "navigator.webdriver", "some.api"]

    not_exist = {i["target"] for i in report["issues"] if i["type"] == "NOT_EXIST"}
    assert not_exist == {"empty.val", "navigator.language",
                         "navigator.webdriver", "some.api"}
    assert report["coverage"]["total_targets"] == 6
    assert report["coverage"]["missing"] == 4
    assert report["coverage"]["configured"] == 2


def test_probe_vm_mode_with_trace_calls(monkeypatch):
    """VM mode: trace calls with error values produce CALL_ERROR."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: ("patched", {"vm": True}))
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: MockTrace([
        MockEntry("C", 20, "Math.random", "0.5"),
        MockEntry("C", 21, "Math.round", "3"),
        MockEntry("C", 22, "JSON.parse", "Error: Unexpected token"),
        MockEntry("C", 23, "JSON.stringify", '{"a":1}'),
    ]))

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def get_unified_trace(self): return ["dummy"]
        def stop_recording(self): return None
        def get_console_messages(self): return []
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)

    from iv8_rs import probe_environment
    report = probe_environment("js", profile=None)

    assert report["calls"]["Math.random"] == 1
    assert report["calls"]["Math.round"] == 1
    assert report["calls"]["JSON.parse"] == 1
    assert report["calls"]["JSON.stringify"] == 1

    call_errors = [i for i in report["issues"] if i["type"] == "CALL_ERROR"]
    assert len(call_errors) == 1
    assert call_errors[0]["target"] == "JSON.parse"


def test_probe_vm_mode_with_trace_writes(monkeypatch):
    """VM mode: trace writes populate report."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: ("patched", {"vm": True}))
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: MockTrace([
        MockEntry("W", 30, "document.cookie", "test=1"),
        MockEntry("W", 31, "localStorage.key", "value"),
    ]))

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def get_unified_trace(self): return ["dummy"]
        def stop_recording(self): return None
        def get_console_messages(self): return []
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)

    from iv8_rs import probe_environment
    report = probe_environment("js", profile=None)

    assert report["writes"]["document.cookie"] == "test=1"
    assert report["writes"]["localStorage.key"] == "value"


def test_probe_vm_mode_no_trace(monkeypatch):
    """VM mode with empty trace => trace_stats is None, coverage is empty."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: ("patched", {"vm": True}))
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: None)

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def get_unified_trace(self): return None
        def stop_recording(self): return None
        def get_console_messages(self): return []
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)

    from iv8_rs import probe_environment
    report = probe_environment("js", profile=None)

    assert report["trace_stats"] is None
    assert report["coverage"]["total_targets"] == 0
    assert report["coverage"]["coverage_pct"] == 0.0


# ===================================================================
# entry_expr failure path (lines 99-100)
# ===================================================================

def test_probe_entry_expr_failure(monkeypatch):
    """entry_expr eval failure produces a warning (lines 99-100)."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: (_ for _ in ()).throw(Exception("no VM")))

    raised = []

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, **kw): pass
        def eval(self, source):
            if source == "bad_expr":
                raised.append(True)
                raise RuntimeError("eval failed")
        def stop_recording(self): return None
        def get_unified_trace(self): return None
        def get_console_messages(self): return []
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)

    from iv8_rs import probe_environment
    with warnings.catch_warnings(record=True) as w:
        warnings.simplefilter("always")
        report = probe_environment("var x;", entry_expr="bad_expr", profile=None)

    assert raised
    assert any("entry_expr failed" in str(msg.message) for msg in w)
    assert isinstance(report, dict)


# ===================================================================
# Console error classification (lines 153-178)
# ===================================================================

def test_probe_console_not_function(monkeypatch):
    """'is not a function' console error => NOT_FUNCTION issue."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: (_ for _ in ()).throw(Exception("x")))

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def stop_recording(self): return None
        def get_unified_trace(self): return None
        def get_console_messages(self):
            return [{"text": "navigator.foo is not a function", "level": "error"}]
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: None)

    from iv8_rs import probe_environment
    report = probe_environment("navigator.foo();", profile=None)

    not_func = [i for i in report["issues"] if i["type"] == "NOT_FUNCTION"]
    assert len(not_func) >= 1
    assert not_func[0]["detail"] == "navigator.foo is not a function"


def test_probe_console_not_defined(monkeypatch):
    """'is not defined' console error => NOT_DEFINED issue."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: (_ for _ in ()).throw(Exception("x")))

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def stop_recording(self): return None
        def get_unified_trace(self): return None
        def get_console_messages(self):
            return [{"text": "foo is not defined", "level": "error"}]
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: None)

    from iv8_rs import probe_environment
    report = probe_environment("foo;", profile=None)

    not_def = [i for i in report["issues"] if i["type"] == "NOT_DEFINED"]
    assert len(not_def) >= 1
    assert not_def[0]["detail"] == "foo is not defined"


def test_probe_console_cannot_read(monkeypatch):
    """'Cannot read' console error => NULL_ACCESS issue."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: (_ for _ in ()).throw(Exception("x")))

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def stop_recording(self): return None
        def get_unified_trace(self): return None
        def get_console_messages(self):
            return [{"text": "Cannot read properties of undefined (reading 'foo')", "level": "error"}]
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: None)

    from iv8_rs import probe_environment
    report = probe_environment("undefined.foo;", profile=None)

    null_acc = [i for i in report["issues"] if i["type"] == "NULL_ACCESS"]
    assert len(null_acc) >= 1


def test_probe_console_warn_level_also_collected(monkeypatch):
    """'warn' level messages are also collected as errors."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: (_ for _ in ()).throw(Exception("x")))

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def stop_recording(self): return None
        def get_unified_trace(self): return None
        def get_console_messages(self):
            return [{"text": "something is not defined", "level": "warn"}]
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: None)

    from iv8_rs import probe_environment
    report = probe_environment("x", profile=None)

    assert "something is not defined" in report["errors"]
    not_def = [i for i in report["issues"] if i["type"] == "NOT_DEFINED"]
    assert len(not_def) == 1


def test_probe_console_log_level_not_collected(monkeypatch):
    """'log' level messages are NOT collected as errors."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: (_ for _ in ()).throw(Exception("x")))

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def stop_recording(self): return None
        def get_unified_trace(self): return None
        def get_console_messages(self):
            return [{"text": "foo is not defined", "level": "log"}]
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: None)

    from iv8_rs import probe_environment
    report = probe_environment("x", profile=None)

    # 'log' is not in ("error", "warn") so not collected
    assert "foo is not defined" not in report["errors"]


# ===================================================================
# Recording-mode with trace data (lines 126-149 from recording path)
# ===================================================================

def test_probe_recording_mode_with_trace(monkeypatch):
    """Recording mode: parse_trace receives stop_recording output."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: (_ for _ in ()).throw(Exception("x")))

    parsed = []

    def _parse(raw):
        parsed.append(raw)
        return MockTrace([
            MockEntry("R", -1, "navigator.userAgent", "Chrome"),
            MockEntry("R", -1, "navigator.webdriver", "undefined"),
            MockEntry("C", -1, "Math.random", "0.5"),
            MockEntry("W", -1, "document.cookie", "test=1"),
        ])

    monkeypatch.setattr("iv8_rs.trace.parse_trace", _parse)

    stop_result = ["R,-1,navigator.userAgent,Chrome"]

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, **kw):
            self._recording = True
        def eval(self, source): pass
        def stop_recording(self): return stop_result
        def get_unified_trace(self): return None
        def get_console_messages(self): return []
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)

    from iv8_rs import probe_environment
    report = probe_environment("navigator.userAgent;", profile=None)

    assert parsed == [stop_result]
    assert report["reads"]["navigator.userAgent"] == "Chrome"
    assert report["reads"]["navigator.webdriver"] == "undefined"
    assert report["calls"]["Math.random"] == 1
    assert report["writes"]["document.cookie"] == "test=1"


# ===================================================================
# Edge case: profile=None but environment explicitly passed
# ===================================================================

def test_probe_no_profile_custom_env(monkeypatch):
    """profile=None, environment=dict => env passed directly."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: (_ for _ in ()).throw(Exception("x")))

    init_env = []

    class MockCtx:
        def __init__(self, **kw):
            init_env.append(kw.get("environment"))
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def stop_recording(self): return None
        def get_unified_trace(self): return None
        def get_console_messages(self): return []
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: None)

    from iv8_rs import probe_environment
    report = probe_environment("js", environment={"a": 1}, profile=None)

    assert init_env[0] == {"a": 1}


def test_probe_no_profile_no_env(monkeypatch):
    """profile=None, environment=None => final_env empty => ctx gets None."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: (_ for _ in ()).throw(Exception("x")))

    init_env = []

    class MockCtx:
        def __init__(self, **kw):
            init_env.append(kw.get("environment"))
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def stop_recording(self): return None
        def get_unified_trace(self): return None
        def get_console_messages(self): return []
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: None)

    from iv8_rs import probe_environment
    report = probe_environment("js", profile=None)

    assert init_env[0] is None


# ===================================================================
# time_freeze and time_mode passthrough
# ===================================================================

def test_probe_time_freeze_passthrough(monkeypatch):
    """time_freeze and time_mode passed to JSContext."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: (_ for _ in ()).throw(Exception("x")))
    kw = {}

    class MockCtx:
        def __init__(self, **kwargs):
            kw.update(kwargs)
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def stop_recording(self): return None
        def get_unified_trace(self): return None
        def get_console_messages(self): return []
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: None)

    from iv8_rs import probe_environment
    probe_environment("js", profile=None, time_freeze=1000.0, time_mode="fast")

    assert kw["time_freeze"] == 1000.0
    assert kw["time_mode"] == "fast"


# ===================================================================
# Edge case: instrument_source raises for other reasons
# ===================================================================

def test_probe_instrument_source_raises_fallback(monkeypatch):
    """When instrument_source raises, falls back to recording mode."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: (_ for _ in ()).throw(ValueError("bad source")))

    recorded = []

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, targets=None, limit=None):
            recorded.append(("start_recording", targets, limit))
        def eval(self, source): pass
        def stop_recording(self): return None
        def get_unified_trace(self): return None
        def get_console_messages(self): return []
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: None)

    from iv8_rs import probe_environment
    report = probe_environment("js", profile=None)

    assert len(recorded) == 1
    assert recorded[0][0] == "start_recording"
    assert report["vm_info"] is None


# ===================================================================
# Combined: multiple error types in one report
# ===================================================================

def test_probe_multiple_console_errors(monkeypatch):
    """All three error patterns in one console dump produce correct issues."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: (_ for _ in ()).throw(Exception("x")))

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def stop_recording(self): return None
        def get_unified_trace(self): return None
        def get_console_messages(self):
            return [
                {"text": "navigator.bar is not a function", "level": "error"},
                {"text": "baz is not defined", "level": "error"},
                {"text": "Cannot read properties of undefined (reading 'x')", "level": "error"},
            ]
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)
    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: None)

    from iv8_rs import probe_environment
    report = probe_environment("bad.js", profile=None)

    types = {i["type"] for i in report["issues"]}
    assert types == {"NOT_FUNCTION", "NOT_DEFINED", "NULL_ACCESS"}
    assert len(report["errors"]) == 3


# ===================================================================
# Full integration: recording mode with complex trace + console errors
# ===================================================================

def test_probe_full_recording_report(monkeypatch):
    """Recording mode with trace and console errors produces full report."""
    monkeypatch.setattr("iv8_rs._iv8.instrument_source",
                        lambda js: (_ for _ in ()).throw(Exception("x")))

    monkeypatch.setattr("iv8_rs.trace.parse_trace", lambda raw: MockTrace([
        MockEntry("R", -1, "navigator.userAgent", "Mozilla/5.0"),
        MockEntry("R", -1, "navigator.webdriver", "undefined"),
        MockEntry("R", -1, "screen.width", "1920"),
        MockEntry("C", -1, "Math.random", "0.5"),
        MockEntry("C", -1, "JSON.parse", "Error: parse failed"),
        MockEntry("W", -1, "document.cookie", "a=1"),
    ]))

    class MockCtx:
        def __init__(self, **kw): pass
        def start_recording(self, **kw): pass
        def eval(self, source): pass
        def stop_recording(self):
            return ["R,-1,navigator.userAgent,Mozilla/5.0"]
        def get_unified_trace(self): return None
        def get_console_messages(self):
            return [
                {"text": "foo is not defined", "level": "error"},
            ]
        def close(self): pass

    monkeypatch.setattr("iv8_rs._iv8.JSContext", MockCtx)

    from iv8_rs import probe_environment
    report = probe_environment("js", profile=None)

    assert report["reads"]["navigator.userAgent"] == "Mozilla/5.0"
    assert report["reads"]["navigator.webdriver"] == "undefined"
    assert report["reads"]["screen.width"] == "1920"
    assert report["calls"]["Math.random"] == 1
    assert report["calls"]["JSON.parse"] == 1
    assert report["writes"]["document.cookie"] == "a=1"
    assert "navigator.webdriver" in report["missing"]
    assert "foo is not defined" in report["errors"]
    assert any(i["type"] == "NOT_DEFINED" for i in report["issues"])
    assert any(i["type"] == "CALL_ERROR" for i in report["issues"])
    assert report["coverage"]["total_targets"] == 3
    assert report["trace_stats"] is not None
