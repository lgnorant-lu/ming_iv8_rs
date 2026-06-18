"""
Property-based tests for v0.5 analysis modules using hypothesis.

Tests invariants that must hold for ANY valid input, catching
edge cases that manual tests would miss.
"""
from __future__ import annotations

from hypothesis import given, strategies as st, assume, settings
from iv8_rs.trace import parse_trace, compress_trace, TraceEntry
from iv8_rs.cfg import CFG
from iv8_rs.patterns import detect_constants, detect_all, detect_loops, detect_hotspots
from iv8_rs.taint import TaintEngine


# ─── Strategy: generate a single trace line ──────────────────────────────────

_TYPES = st.sampled_from(["D", "R", "C", "W"])
_PCS = st.integers(min_value=-1, max_value=9999)
_TARGETS = st.text(min_size=0, max_size=20)
_VALUES = st.text(min_size=0, max_size=20)


@st.composite
def trace_line_4field(draw):
    """Generate a valid 4-field trace line: TYPE,pc,target,value."""
    t = draw(_TYPES)
    pc = draw(_PCS)
    target = draw(_TARGETS).replace(",", "_")
    value = draw(_VALUES).replace(",", "_")
    return f"{t},{pc},{target},{value}"


@st.composite
def trace_line_3field(draw):
    """Generate a valid 3-field trace line: TYPE,target,value."""
    t = draw(_TYPES)
    target = draw(_TARGETS).replace(",", "_")
    value = draw(_VALUES).replace(",", "_")
    return f"{t},{target},{value}"


trace_lines = st.one_of(trace_line_4field(), trace_line_3field())


# ─── Strategy: generate a trace entry ────────────────────────────────────────

@st.composite
def trace_entry(draw):
    return TraceEntry(
        type=draw(_TYPES),
        pc=draw(_PCS),
        target=draw(_TARGETS),
        value=draw(_VALUES),
        raw="",
    )


# ═══════════════════════════════════════════════════════════════════════════════
# 1. parse_trace: robustness and invariants
# ═══════════════════════════════════════════════════════════════════════════════


@given(st.lists(trace_lines, max_size=50))
@settings(max_examples=500)
def test_parse_trace_never_crashes(lines):
    """parse_trace should never crash on any list of strings."""
    trace = parse_trace(lines)
    assert len(trace) == len(trace.entries)
    assert isinstance(trace.dispatches, list)
    assert isinstance(trace.reads, list)
    assert isinstance(trace.calls, list)
    assert isinstance(trace.writes, list)


@given(st.lists(trace_lines, max_size=30))
@settings(max_examples=500)
def test_parse_trace_entry_types_sum_to_total(lines):
    """dispatch + reads + calls + writes should sum to total entries."""
    trace = parse_trace(lines)
    total = len(trace)
    typed = len(trace.dispatches) + len(trace.reads) + len(trace.calls) + len(trace.writes)
    assert typed == total


@given(st.lists(trace_lines, max_size=50))
@settings(max_examples=200)
def test_parse_trace_filter_soundness(lines):
    """filter(type=X) should only keep entries of type X."""
    trace = parse_trace(lines)
    for t in ("D", "R", "C", "W"):
        filtered = trace.filter(type=t)
        for entry in filtered:
            assert entry.type == t, f"filter(type={t}) returned entry with type {entry.type}"


@given(st.lists(trace_lines, max_size=30))
@settings(max_examples=200)
def test_parse_trace_summary_structure(lines):
    """summary() should always return a dict with expected keys."""
    trace = parse_trace(lines)
    s = trace.summary()
    assert isinstance(s, dict)
    assert "total" in s
    assert "counts_by_type" in s
    assert "pc_range" in s
    assert "unique_targets" in s
    assert "unique_opcodes" in s
    assert s["total"] == len(trace)


# ═══════════════════════════════════════════════════════════════════════════════
# 2. compress_trace: round-trip invariance
# ═══════════════════════════════════════════════════════════════════════════════


@given(st.lists(trace_lines, max_size=50))
@settings(max_examples=200)
def test_compress_expand_roundtrip(lines):
    """compress then expand preserves total entry count and dispatch type info."""
    trace = parse_trace(lines)
    assume(len(trace) > 0)
    ct = compress_trace(trace)
    expanded = ct.expand()
    # Total count and dispatch count must match
    assert len(expanded) == len(trace), f"entry count mismatch: {len(expanded)} != {len(trace)}"
    assert ct.total_dispatches == len(trace.dispatches), \
        f"dispatch count mismatch: {ct.total_dispatches} != {len(trace.dispatches)}"
    # Compression ratio should be reasonable
    assert 0.0 < ct.compression_ratio <= 1.0


# ═══════════════════════════════════════════════════════════════════════════════
# 3. CFG: invariant properties
# ═══════════════════════════════════════════════════════════════════════════════


@st.composite
def dispatch_trace(draw):
    """Generate a trace with D entries for CFG testing."""
    entries = []
    n = draw(st.integers(min_value=1, max_value=30))
    for _ in range(n):
        pc = draw(st.integers(min_value=0, max_value=100))
        target = str(draw(st.integers(min_value=0, max_value=20)))
        entries.append(f"D,{pc},{target},0")
    return entries


@given(st.lists(st.integers(min_value=0, max_value=100), min_size=1, max_size=50))
@settings(max_examples=500)
def test_cfg_back_edge_invariant(pc_sequence):
    """All back edges must go to a PC <= from_pc."""
    lines = [f"D,{pc},0,0" for pc in pc_sequence]
    trace = parse_trace(lines)
    cfg = CFG.from_trace(trace)
    for edge in cfg.edges:
        if edge.is_back_edge:
            assert edge.to_pc <= edge.from_pc, \
                f"back edge {edge.from_pc} -> {edge.to_pc} violates invariant"


@given(dispatch_trace())
@settings(max_examples=200)
def test_cfg_from_trace_never_crashes(lines):
    """CFG.from_trace should never crash on valid dispatch traces."""
    trace = parse_trace(lines)
    cfg = CFG.from_trace(trace)
    assert len(cfg.nodes) <= len(trace.dispatches)
    assert len(cfg.edges) <= max(1, len(cfg.nodes) * len(cfg.nodes))


@given(st.lists(st.integers(min_value=0, max_value=200), min_size=1, max_size=50))
@settings(max_examples=200)
def test_cfg_loop_header_in_nodes(pc_sequence):
    """All loop headers must exist in cfg.nodes."""
    lines = [f"D,{pc},0,0" for pc in pc_sequence]
    trace = parse_trace(lines)
    cfg = CFG.from_trace(trace)
    loops = cfg.find_loops()
    for loop in loops:
        assert loop.header_pc in cfg.nodes, \
            f"loop header {loop.header_pc} not in nodes"


# ═══════════════════════════════════════════════════════════════════════════════
# 4. detect_*: robustness
# ═══════════════════════════════════════════════════════════════════════════════


@given(st.lists(trace_lines, max_size=50))
@settings(max_examples=200)
def test_detect_constants_never_crashes(lines):
    """detect_constants should never crash on any trace."""
    trace = parse_trace(lines)
    results = detect_constants(trace, min_value=0)
    assert isinstance(results, list)


@given(st.lists(trace_lines, max_size=50))
@settings(max_examples=200)
def test_detect_all_never_crashes(lines):
    """detect_all should never crash on any trace."""
    trace = parse_trace(lines)
    results = detect_all(trace)
    assert isinstance(results, list)


@given(st.lists(trace_lines, max_size=50))
@settings(max_examples=200)
def test_detect_loops_never_crashes(lines):
    """detect_loops should never crash on any trace."""
    trace = parse_trace(lines)
    results = detect_loops(trace)
    assert isinstance(results, list)


@given(st.lists(trace_lines, max_size=50))
@settings(max_examples=200)
def test_detect_hotspots_never_crashes(lines):
    """detect_hotspots should never crash on any trace."""
    trace = parse_trace(lines)
    results = detect_hotspots(trace)
    assert isinstance(results, list)


# ═══════════════════════════════════════════════════════════════════════════════
# 5. TaintEngine: robustness
# ═══════════════════════════════════════════════════════════════════════════════


@given(st.lists(trace_lines, max_size=30), st.dictionaries(
    st.text(min_size=1, max_size=10), st.text(min_size=0, max_size=10), max_size=3,
))
@settings(max_examples=200)
def test_taint_never_crashes(lines, sources):
    """TaintEngine.analyze should never crash on any input."""
    trace = parse_trace(lines)
    engine = TaintEngine(trace, sources=sources)
    report = engine.analyze()
    assert isinstance(report.sources, list)
    assert isinstance(report.sinks, list)
    assert isinstance(report.flows, list)
    assert isinstance(report.unreached_sources, list)


@given(st.lists(trace_lines, max_size=30))
@settings(max_examples=100)
def test_taint_empty_engine(lines):
    """TaintEngine with empty sources should produce empty report."""
    trace = parse_trace(lines)
    engine = TaintEngine(trace, sources={})
    report = engine.analyze()
    assert len(report.sources) == 0
    assert len(report.flows) == 0


@given(st.lists(trace_line_4field, min_size=1, max_size=200))
@settings(max_examples=300)
def test_cfg_edge_count_bounded(lines):
    """CFG edge count should never exceed N*N for N nodes."""
    trace = parse_trace(lines)
    cfg = CFG.from_trace(trace)
    n = len(cfg.nodes)
    assert len(cfg.edges) <= n * n


@given(st.lists(trace_line_4field, min_size=3, max_size=200))
@settings(max_examples=200)
def test_compress_trace_reduces_or_preserves_count(lines):
    """compress_trace should not increase entry count vs original."""
    trace = parse_trace(lines)
    compressed = compress_trace(trace)
    assert compressed.total_dispatches <= len(trace.entries)


@given(st.lists(trace_line_4field, min_size=1, max_size=300))
@settings(max_examples=200)
def test_detect_loops_pc_range(lines):
    """detect_loops should only report PCs that exist in the trace."""
    trace = parse_trace(lines)
    loops = detect_loops(trace, min_iterations=3)
    trace_pcs = {e.pc for e in trace.entries if e.pc >= 0}
    for loop in loops:
        assert loop.pc in trace_pcs, f"loop PC {loop.pc} not in trace"


@given(st.lists(trace_line_4field, min_size=1, max_size=200))
@settings(max_examples=200)
def test_detect_hotspots_no_overflow(lines):
    """detect_hotspots should never return more hotspots than unique PCs."""
    trace = parse_trace(lines)
    hotspots = detect_hotspots(trace, top_n=50)
    unique_pcs = len({e.pc for e in trace.entries if e.pc >= 0})
    assert len(hotspots) <= unique_pcs
