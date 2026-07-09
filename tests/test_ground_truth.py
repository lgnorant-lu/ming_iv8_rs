"""
Ground-truth end-to-end validation (Phase 1 closeout, Task 2).

Goes beyond synthetic hand-fed traces: drives REAL V8 execution through
iv8-rs, collects the REAL trace, and feeds it to the detection pipeline.

What this proves (honestly):
  1. The pipeline (real execution -> trace -> parse_trace -> detect engine)
     is connected and robust on real data.
  2. start_recording's 3-field format and instrument_source's 4-field format
     both parse correctly (regression guard for the parser bug found during
     Phase 1 closeout).
  3. Documents the trace-mechanism BOUNDARY: local crypto constants do NOT
     enter the trace (only global property access + dispatch do). Crypto
     constant recall on real traces therefore depends on the target VM
     exposing constants into the trace — a real-sample (TDC) concern, not
     something a synthetic VM can fake. Locked as an assertion so the
     boundary is not silently misunderstood later.

Requires the built Rust extension; skipped gracefully if unavailable.
"""
import pytest

iv8_rs = pytest.importorskip("iv8_rs", reason="Rust extension not built")

from iv8_rs.patterns import detect_constants
from iv8_rs.trace import parse_trace


# ============================================================
# Real start_recording pipeline (3-field format)
# ============================================================
class TestRealRecordingPipeline:
    def test_real_global_reads_flow_into_trace(self):
        """Real JS reading global props produces real R/C entries that parse."""
        ctx = iv8_rs.JSContext(random_seed=42,
                               environment={"screen.width": 1920, "screen.height": 1080})
        ctx.page_load("<html><body></body></html>")
        ctx.start_recording(targets=["screen", "navigator", "Math"], limit=10000)
        ctx.eval("""
            var w = screen.width;
            var h = screen.height;
            var ua = navigator.userAgent;
            var r = Math.floor(Math.random() * 100);
            w + h;
        """)
        entries = ctx.stop_recording()
        ctx.close()

        assert len(entries) > 0, "real recording produced no entries"
        trace = parse_trace(entries)
        # Every entry must parse to a valid type (regression: 3-field format)
        assert all(e.type in ("R", "C", "W") for e in trace.entries)
        # Math calls must be captured as C entries with their target intact
        math_calls = [e for e in trace.calls if e.target.startswith("Math.")]
        assert math_calls, f"expected Math.* calls, got {[e.target for e in trace.calls]}"
        # The target must NOT be empty (the parser bug made target=value, value='')
        assert all(e.target and "." in e.target for e in trace.calls)

    def test_three_field_format_parses_target_and_value(self):
        """Regression guard: 3-field 'C,target,value' keeps target and value distinct."""
        trace = parse_trace(["C,Math.random,0.47", "R,navigator.userAgent,Mozilla/5.0"])
        c = trace.calls[0]
        assert c.target == "Math.random" and c.value == "0.47"
        r = trace.reads[0]
        assert r.target == "navigator.userAgent" and r.value == "Mozilla/5.0"


# ============================================================
# Real instrument_source pipeline (4-field format)
# ============================================================
class TestRealInstrumentPipeline:
    def test_instrument_source_pipeline_connected(self):
        """Real VM JS -> instrument_source -> eval -> get_unified_trace -> parse."""
        vm_js = "var A=[function(){}];var Q=[0];var U=0;var g=[];A[Q[U++]]();"
        patched, info = iv8_rs.instrument_source(
            vm_js, handler_array="A", pc_var="U", stack_var="g", index_array="Q"
        )
        assert "handler_array" in info
        ctx = iv8_rs.JSContext(random_seed=42)
        ctx.eval(patched)
        raw = ctx.get_unified_trace()
        ctx.close()
        # Pipeline must not crash; trace (possibly empty for trivial JS) must parse
        trace = parse_trace(raw)
        assert all(e.type in ("D", "R", "C", "W") for e in trace.entries)

    def test_four_field_format_parses_pc(self):
        """Regression guard: 4-field 'D,pc,opcode,depth' keeps integer pc."""
        trace = parse_trace(["D,100,5,3", "C,150,String.concat,abc"])
        d = trace.dispatches[0]
        assert d.pc == 100 and d.target == "5" and d.value == "3"
        c = trace.calls[0]
        assert c.pc == 150 and c.target == "String.concat" and c.value == "abc"


# ============================================================
# Trace-mechanism boundary (honest, locked as assertion)
# ============================================================
class TestTraceBoundary:
    def test_local_crypto_constant_does_not_enter_trace(self):
        """HONEST BOUNDARY: a local crypto constant (XTEA delta) computed in
        ordinary JS does NOT appear in the instrument_source trace, because the
        trace only records global property access + dispatch, not local values.

        This is why crypto-constant recall on REAL traces depends on the target
        VM exposing constants into the trace (real-sample/TDC concern), and
        cannot be validated with a synthetic local-variable computation.
        """
        js = "var delta = 0x9E3779B9; var x = (delta + 1) >>> 0; x;"
        ctx = iv8_rs.JSContext(random_seed=42)
        ctx.start_recording(targets=["Math", "crypto"], limit=10000)
        ctx.eval(js)
        entries = ctx.stop_recording()
        ctx.close()
        trace = parse_trace(entries)
        consts = detect_constants(trace, min_value=0)
        xtea = [c for c in consts if "XTEA" in c.algorithm or "TEA" in c.algorithm]
        assert not xtea, (
            "Unexpected: local XTEA delta surfaced into trace. If trace mechanism "
            "changed to capture local values, update this boundary test and the "
            "H01 spec's known-limitations section."
        )

    def test_global_crypto_call_is_captured(self):
        """A crypto operation routed through the global crypto object IS captured
        (C entry), unlike a local constant. Demonstrates the boundary direction."""
        ctx = iv8_rs.JSContext(random_seed=42)
        ctx.start_recording(targets=["crypto"], limit=10000)
        ctx.eval("var a = new Uint8Array(8); crypto.getRandomValues(a); a[0];")
        entries = ctx.stop_recording()
        ctx.close()
        trace = parse_trace(entries)
        crypto_calls = [e for e in trace.calls if e.target.startswith("crypto.")]
        assert crypto_calls, f"expected crypto.* call captured, got {[e.target for e in trace.calls]}"
