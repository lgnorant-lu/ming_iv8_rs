"""
End-to-end pipeline test (v0.5).

Chains the full analysis pipeline: instrument + execute + collect + parse + detect.
Verifies that real V8 execution produces trace data that flows correctly
through all layers without data corruption or silent failures.

Uses instrument_chaosvm for dispatch traces and start_recording for
property-access traces. Detection accuracy is tested in test_crypto_detection.py
with controlled inputs; here we verify the pipeline connectivity.

Requires the built Rust extension; skipped gracefully if unavailable.
"""
import pytest

pytestmark = pytest.mark.e2e

iv8_rs = pytest.importorskip("iv8_rs", reason="Rust extension not built")

from iv8_rs.patterns import detect_all
from iv8_rs.trace import parse_trace

SIMPLE_VM = r"""
var A = [
    function(){ g.push(42); },
    function(){ g.push(g.pop() + g.pop()); },
    function(){ return g.pop(); },
];
var g = [];
var Q = 0;
"""


class TestE2EPipeline:
    def test_instrument_chaosvm_produces_trace(self):
        """Verify instrument_chaosvm dispatches produce trace entries."""
        ctx = iv8_rs.JSContext(random_seed=42)
        ctx.eval(SIMPLE_VM)
        ctx.instrument_chaosvm("A", "Q", "g", capture_stack_depth=3)
        for _ in range(10):
            ctx.eval("A[Q++ % 3]();")
        raw = ctx.get_vm_trace()
        ctx.close()

        assert len(raw) > 0

    def test_recording_produces_parsed_trace(self):
        """Verify recording mode -> parse produces typed entries."""
        ctx = iv8_rs.JSContext(random_seed=42)
        ctx.start_recording(targets=["Math"], limit=10000)
        ctx.eval("var x = Math.random();")
        raw = ctx.stop_recording()
        ctx.close()

        assert len(raw) > 0
        trace = parse_trace(raw)
        assert len(trace) > 0
        assert trace.summary()["total"] > 0

    def test_recording_to_detect_all_runs(self):
        """Verify detect_all completes without error on recording trace."""
        ctx = iv8_rs.JSContext(random_seed=42)
        ctx.start_recording(targets=["Math"], limit=10000)
        ctx.eval("var x = Math.random();")
        raw = ctx.stop_recording()
        ctx.close()

        trace = parse_trace(raw)
        results = detect_all(trace)
        assert isinstance(results, list)
