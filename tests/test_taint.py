"""Taint Tracking tests (M26)."""
import pytest
from iv8_rs.trace import StructuredTrace, TraceEntry
from iv8_rs.taint import TaintEngine, TaintReport


def make_trace(entries_data):
    entries = []
    for t, pc, target, value in entries_data:
        raw = f"{t},{pc},{target},{value}"
        entries.append(TraceEntry(type=t, pc=pc, target=target, value=str(value), raw=raw))
    return StructuredTrace(entries)


class TestBasicTaint:
    def test_value_flows_from_read_to_write(self):
        """Source value appears in R entry, propagates through D stack, reaches W entry."""
        trace = make_trace([
            ("R", 10, "screen.width", "1920"),
            ("D", 20, "5", "3,1920,,"),       # stack contains 1920
            ("D", 21, "7", "4,1920,100,"),    # still in stack
            ("W", 30, "cd[10]", "1920"),       # written to output
        ])
        engine = TaintEngine(trace, sources={"screen.width": "1920"})
        report = engine.analyze()

        assert len(report.sources) == 1
        assert report.sources[0].value == "1920"
        assert report.sources[0].pc == 10  # found in R entry
        assert len(report.sinks) == 1
        assert report.sinks[0].target == "cd[10]"
        assert len(report.flows) == 1
        assert report.flows[0].source.target == "screen.width"
        assert report.flows[0].sink.target == "cd[10]"
        assert len(report.flows[0].intermediate_pcs) == 2  # PCs 20, 21
        assert report.unreached_sources == []
        assert report.stack_hits["WIDTH"] == 2

    def test_value_not_reaching_sink(self):
        """Source value appears in stack but never in W entry."""
        trace = make_trace([
            ("R", 10, "screen.width", "1920"),
            ("D", 20, "5", "3,1920,,"),
            ("D", 21, "7", "2,42,,"),
        ])
        engine = TaintEngine(trace, sources={"screen.width": "1920"})
        report = engine.analyze()

        assert len(report.sinks) == 0
        assert len(report.flows) == 0
        assert "WIDTH" in report.unreached_sources

    def test_multiple_sources(self):
        """Multiple sources tracked independently."""
        trace = make_trace([
            ("R", 10, "screen.width", "1920"),
            ("R", 11, "screen.height", "1080"),
            ("D", 20, "5", "3,1920,,"),
            ("D", 21, "7", "3,1080,,"),
            ("W", 30, "cd[10]", "1920"),
            ("W", 31, "cd[11]", "1080"),
        ])
        engine = TaintEngine(trace, sources={
            "screen.width": "1920",
            "screen.height": "1080",
        })
        report = engine.analyze()

        assert len(report.sources) == 2
        assert len(report.sinks) == 2
        assert len(report.flows) == 2
        assert report.unreached_sources == []

    def test_no_stack_values_still_finds_direct_rw(self):
        """Even without D stack values, direct R→W value match works."""
        trace = make_trace([
            ("R", 10, "screen.width", "1920"),
            ("D", 20, "5", "3"),  # no stack values
            ("W", 30, "cd[10]", "1920"),
        ])
        engine = TaintEngine(trace, sources={"screen.width": "1920"})
        report = engine.analyze()

        assert len(report.sinks) == 1  # W entry value matches
        assert report.stack_hits["WIDTH"] == 0  # no stack hits (no values in D)

    def test_empty_trace(self):
        trace = make_trace([])
        engine = TaintEngine(trace, sources={"screen.width": "1920"})
        report = engine.analyze()

        assert len(report.sources) == 1
        assert len(report.sinks) == 0
        assert len(report.flows) == 0
        assert "WIDTH" in report.unreached_sources

    def test_user_specified_source_no_r_entry(self):
        """Source specified by user but not found in R entries → pc=-1."""
        trace = make_trace([
            ("D", 20, "5", "3,1920,,"),
            ("W", 30, "output", "1920"),
        ])
        engine = TaintEngine(trace, sources={"screen.width": "1920"})
        report = engine.analyze()

        assert report.sources[0].pc == -1  # not found in R entries
        assert len(report.sinks) == 1  # still finds the W match

    def test_call_entry_as_sink(self):
        """C entries (function calls) can also be sinks."""
        trace = make_trace([
            ("R", 10, "screen.width", "1920"),
            ("C", 30, "String.concat", "1920"),
        ])
        engine = TaintEngine(trace, sources={"screen.width": "1920"})
        report = engine.analyze()

        assert len(report.sinks) == 1
        assert report.sinks[0].target == "String.concat"

    def test_substring_matching(self):
        """Value matching uses exact token comparison (not substring)."""
        trace = make_trace([
            ("D", 20, "5", "3,1920,42,"),
            ("W", 30, "result", "1920"),
        ])
        engine = TaintEngine(trace, sources={"screen.width": "1920"})
        report = engine.analyze()

        assert report.stack_hits["WIDTH"] == 1
        assert len(report.sinks) == 1

    def test_large_value_no_false_positive(self):
        """A large unique value should not match unrelated entries."""
        trace = make_trace([
            ("D", 20, "5", "3,42,100,200"),
            ("D", 21, "7", "4,999,888,777"),
            ("W", 30, "output", "12345"),
        ])
        engine = TaintEngine(trace, sources={"custom.value": "2654435769"})
        report = engine.analyze()

        assert report.stack_hits[report.sources[0].label] == 0
        assert len(report.sinks) == 0
        assert len(report.unreached_sources) == 1

    def test_long_propagation_chain(self):
        """Value propagates through a long chain of D entries."""
        entries = [("R", 1, "source", "42")]
        for i in range(2, 102):
            entries.append(("D", i, str(i % 10), f"3,42,{i},"))
        entries.append(("W", 200, "output", "42"))
        trace = make_trace(entries)
        engine = TaintEngine(trace, sources={"source": "42"})
        report = engine.analyze()
        assert len(report.sinks) == 1
        assert len(report.flows) == 1
        assert len(report.flows[0].intermediate_pcs) >= 50

    def test_overlapping_values_not_confused(self):
        """Source '12' should not match value '12345'."""
        trace = make_trace([
            ("D", 20, "5", "3,12345,,"),
            ("W", 30, "output", "12345"),
        ])
        engine = TaintEngine(trace, sources={"source": "12"})
        report = engine.analyze()
        assert len(report.sinks) == 0

    def test_call_sink_with_value_match(self):
        """C entry as sink with matching value."""
        trace = make_trace([
            ("R", 10, "navigator.userAgent", "Mozilla"),
            ("C", 40, "someFunction", "Mozilla"),
        ])
        engine = TaintEngine(trace, sources={"navigator.userAgent": "Mozilla"})
        report = engine.analyze()
        assert len(report.sinks) == 1
        assert report.sinks[0].target == "someFunction"


class TestIndirectAssignment:
    """BUG-10: Indirect assignment behavior is a design limitation, not a code bug.

    The taint engine uses value-string matching, not dataflow analysis. Indirect
    variable assignments (b = a) don't generate trace entries unless the value
    eventually reaches a proxied property write (W) or call return (C).
    """

    def test_value_on_stack_but_no_sink(self):
        """Indirect assignment (b=a): value persists on stack but no W/C → no sink.

        b = a is a JS variable assignment — no Proxy recording entry is generated.
        The value "Mozilla" stays on the V8 stack (visible in D entries) but
        without a W or C entry with the matching value, no sink is detected.
        This is by design: value-matching is not dataflow analysis.
        """
        trace = make_trace([
            ("R", 10, "navigator.userAgent", "Mozilla"),
            ("D", 20, "5", "3,Mozilla,,"),
            ("D", 21, "7", "4,Mozilla,42,"),
        ])
        engine = TaintEngine(trace, sources={"navigator.userAgent": "Mozilla"})
        report = engine.analyze()

        assert len(report.sources) == 1
        assert report.sources[0].value == "Mozilla"
        assert report.stack_hits["USERAG"] == 2
        assert len(report.sinks) == 0
        assert "USERAG" in report.unreached_sources

    def test_indirect_then_write_detected(self):
        """b = a; c[b] → value reaches W entry → detected.

        If the indirectly-assigned value is eventually written to a proxied
        property (W entry), the taint engine detects it because the W entry's
        value string matches the source value.
        """
        trace = make_trace([
            ("R", 10, "navigator.userAgent", "Mozilla/5.0"),
            ("D", 20, "5", "3,Mozilla/5.0,,"),
            ("D", 21, "7", "4,Mozilla/5.0,42,"),
            ("W", 30, "cd[42]", "Mozilla/5.0"),
        ])
        engine = TaintEngine(trace, sources={"navigator.userAgent": "Mozilla/5.0"})
        report = engine.analyze()

        assert len(report.sinks) == 1
        assert report.sinks[0].target == "cd[42]"
        assert len(report.flows) == 1
        assert "USERAG" not in report.unreached_sources

    def test_indirect_then_call_return_match(self):
        """b = a; sink(b) where sink returns the same value → C entry match.

        The Proxy recording captures the return value of proxied function calls.
        If a proxied function returns the same value it received, the C entry
        matches. This is a narrow case — most functions transform their input.
        """
        trace = make_trace([
            ("R", 10, "navigator.userAgent", "Mozilla"),
            ("D", 20, "5", "3,Mozilla,,"),
            ("C", 40, "identity", "Mozilla"),
        ])
        engine = TaintEngine(trace, sources={"navigator.userAgent": "Mozilla"})
        report = engine.analyze()

        assert len(report.sinks) == 1
        assert report.sinks[0].target == "identity"

    def test_indirect_call_return_mismatch(self):
        """b = a; sink(b) where sink returns something else → NOT detected.

        The C entry records the return value of the proxied call, not the
        argument. Most functions transform their input, so the C entry value
        won't match the source value.
        """
        trace = make_trace([
            ("R", 10, "navigator.userAgent", "Mozilla"),
            ("D", 20, "5", "3,Mozilla,,"),
            ("C", 40, "sink", "ok"),
        ])
        engine = TaintEngine(trace, sources={"navigator.userAgent": "Mozilla"})
        report = engine.analyze()

        assert len(report.sinks) == 0
        assert "USERAG" in report.unreached_sources
