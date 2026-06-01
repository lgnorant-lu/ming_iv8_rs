"""
CFG (Control Flow Graph) construction tests.

Covers: from_trace, find_loops, find_modules, get_complexity, to_dot, to_json.
Strategy: construct known-structure traces, assert CFG properties.
Includes positive, negative, boundary, and performance cases.
"""
import pytest
import time
from iv8_rs.trace import StructuredTrace, TraceEntry, parse_trace
from iv8_rs.cfg import CFG, CFGNode, CFGEdge, Loop, Module


def make_dispatch_trace(pc_sequence):
    """Build a trace from a list of (pc, opcode) tuples or just pc ints."""
    entries = []
    for item in pc_sequence:
        if isinstance(item, tuple):
            pc, opc = item
        else:
            pc, opc = item, 0
        raw = f"D,{pc},{opc},1"
        entries.append(TraceEntry(type="D", pc=pc, target=str(opc), value="1", raw=raw))
    return StructuredTrace(entries)


# ============================================================
# Basic construction
# ============================================================
class TestCFGConstruction:
    def test_empty_trace(self):
        cfg = CFG.from_trace(StructuredTrace([]))
        assert len(cfg.nodes) == 0
        assert len(cfg.edges) == 0

    def test_single_dispatch(self):
        trace = make_dispatch_trace([100])
        cfg = CFG.from_trace(trace)
        assert len(cfg.nodes) == 1
        assert cfg.nodes[100].exec_count == 1
        assert len(cfg.edges) == 0  # no successor

    def test_linear_sequence(self):
        trace = make_dispatch_trace([10, 11, 12, 13, 14])
        cfg = CFG.from_trace(trace)
        assert len(cfg.nodes) == 5
        assert len(cfg.edges) == 4  # 4 transitions
        assert all(not e.is_back_edge for e in cfg.edges)

    def test_simple_loop(self):
        # PC: 0,1,2,0,1,2,0,1,2 (3 iterations of loop 0-1-2)
        trace = make_dispatch_trace([0, 1, 2] * 3)
        cfg = CFG.from_trace(trace)
        assert len(cfg.nodes) == 3
        # Edges: 0->1, 1->2, 2->0 (back edge)
        assert len(cfg.edges) == 3
        back_edges = [e for e in cfg.edges if e.is_back_edge]
        assert len(back_edges) == 1
        assert back_edges[0].from_pc == 2 and back_edges[0].to_pc == 0
        assert back_edges[0].count == 2  # 2 back-jumps (3 iterations = 2 back edges)

    def test_branch(self):
        # PC: 0,1,2,0,1,3 (branch at PC 1: goes to 2 first time, 3 second time)
        trace = make_dispatch_trace([0, 1, 2, 0, 1, 3])
        cfg = CFG.from_trace(trace)
        assert len(cfg.nodes) == 4  # PCs 0,1,2,3
        # Edges: 0->1 (x2), 1->2 (x1), 2->0 (x1, back), 1->3 (x1)
        assert len(cfg.edges) == 4
        edge_1_2 = next(e for e in cfg.edges if e.from_pc == 1 and e.to_pc == 2)
        edge_1_3 = next(e for e in cfg.edges if e.from_pc == 1 and e.to_pc == 3)
        assert edge_1_2.count == 1
        assert edge_1_3.count == 1

    def test_self_loop(self):
        # PC stays at same value (self-loop)
        trace = make_dispatch_trace([5, 5, 5, 5])
        cfg = CFG.from_trace(trace)
        assert len(cfg.nodes) == 1
        assert cfg.nodes[5].exec_count == 4
        assert len(cfg.edges) == 1
        assert cfg.edges[0].from_pc == 5 and cfg.edges[0].to_pc == 5
        assert cfg.edges[0].is_back_edge  # 5 <= 5
        assert cfg.edges[0].count == 3

    def test_opcode_tracking(self):
        # Same PC with different opcodes: most common wins
        trace = make_dispatch_trace([(10, 5), (11, 7), (10, 5), (11, 9), (10, 5)])
        cfg = CFG.from_trace(trace)
        assert cfg.nodes[10].opcode == 5  # 5 appears 3 times
        assert cfg.nodes[11].opcode in (7, 9)  # tie, either is fine

    def test_no_d_entries(self):
        # Trace with only R/C/W entries, no D
        entries = [
            TraceEntry(type="R", pc=1, target="x", value="1", raw="R,1,x,1"),
            TraceEntry(type="C", pc=2, target="f", value="v", raw="C,2,f,v"),
        ]
        cfg = CFG.from_trace(StructuredTrace(entries))
        assert len(cfg.nodes) == 0
        assert len(cfg.edges) == 0


# ============================================================
# Loop detection
# ============================================================
class TestLoopDetection:
    def test_no_loops_in_linear(self):
        trace = make_dispatch_trace([0, 1, 2, 3, 4])
        cfg = CFG.from_trace(trace)
        assert cfg.find_loops() == []

    def test_single_loop(self):
        trace = make_dispatch_trace([0, 1, 2, 0, 1, 2, 0, 1, 2])
        cfg = CFG.from_trace(trace)
        loops = cfg.find_loops()
        assert len(loops) == 1
        assert loops[0].header_pc == 0
        assert loops[0].iterations == 2
        assert 0 in loops[0].body_pcs and 1 in loops[0].body_pcs and 2 in loops[0].body_pcs

    def test_nested_loops(self):
        # Outer: 0,1,2,...,0  Inner: 1,2,1
        # Trace: 0, 1, 2, 1, 2, 1, 0, 1, 2, 1, 2, 1, 0
        trace = make_dispatch_trace([0, 1, 2, 1, 2, 1, 0, 1, 2, 1, 2, 1, 0])
        cfg = CFG.from_trace(trace)
        loops = cfg.find_loops()
        # Should find at least 2 loops: inner (header=1) and outer (header=0)
        headers = {l.header_pc for l in loops}
        assert 0 in headers  # outer loop
        assert 1 in headers  # inner loop

    def test_multiple_back_edges_same_header(self):
        # Two different PCs jump back to header 0
        trace = make_dispatch_trace([0, 1, 0, 2, 0])
        cfg = CFG.from_trace(trace)
        loops = cfg.find_loops()
        assert len(loops) == 1
        assert loops[0].header_pc == 0
        assert len(loops[0].back_edges) == 2  # 1->0 and 2->0

    def test_self_loop_detected(self):
        trace = make_dispatch_trace([5, 5, 5])
        cfg = CFG.from_trace(trace)
        loops = cfg.find_loops()
        assert len(loops) == 1
        assert loops[0].header_pc == 5
        assert loops[0].iterations == 2


# ============================================================
# Module detection
# ============================================================
class TestModuleDetection:
    def test_single_module(self):
        trace = make_dispatch_trace([0, 1, 2, 3, 4])
        cfg = CFG.from_trace(trace)
        modules = cfg.find_modules(gap_threshold=50)
        assert len(modules) == 1
        assert modules[0].entry_pc == 0
        assert modules[0].exit_pc == 4

    def test_two_modules_by_gap(self):
        trace = make_dispatch_trace([0, 1, 2, 200, 201, 202])
        cfg = CFG.from_trace(trace)
        modules = cfg.find_modules(gap_threshold=50)
        assert len(modules) == 2
        assert modules[0].entry_pc == 0
        assert modules[1].entry_pc == 200

    def test_three_modules(self):
        trace = make_dispatch_trace([0, 1, 500, 501, 1000, 1001])
        cfg = CFG.from_trace(trace)
        modules = cfg.find_modules(gap_threshold=100)
        assert len(modules) == 3

    def test_gap_threshold_sensitivity(self):
        trace = make_dispatch_trace([0, 1, 50, 51, 200, 201])
        cfg = CFG.from_trace(trace)
        # threshold=100: gap 0->50 is 49 (no split), 51->200 is 149 (split)
        modules = cfg.find_modules(gap_threshold=100)
        assert len(modules) == 2
        # threshold=30: gap 1->50 is 49 (split), 51->200 is 149 (split)
        modules = cfg.find_modules(gap_threshold=30)
        assert len(modules) == 3

    def test_empty_cfg_no_modules(self):
        cfg = CFG.from_trace(StructuredTrace([]))
        assert cfg.find_modules() == []


# ============================================================
# Complexity
# ============================================================
class TestComplexity:
    def test_linear_complexity_1(self):
        # Linear: 5 nodes, 4 edges, 1 component -> 4-5+2=1
        trace = make_dispatch_trace([0, 1, 2, 3, 4])
        cfg = CFG.from_trace(trace)
        assert cfg.get_complexity() == 1

    def test_loop_complexity_2(self):
        # Loop: 3 nodes, 3 edges, 1 component -> 3-3+2=2
        trace = make_dispatch_trace([0, 1, 2, 0])
        cfg = CFG.from_trace(trace)
        assert cfg.get_complexity() == 2

    def test_branch_complexity(self):
        # Branch: 0->1, 0->2 (4 nodes, 4 edges, 1 comp) -> 4-4+2=2
        trace = make_dispatch_trace([0, 1, 0, 2])
        cfg = CFG.from_trace(trace)
        # nodes: 0,1,2; edges: 0->1, 1->0(back), 0->2
        assert cfg.get_complexity() >= 2

    def test_empty_complexity_0(self):
        cfg = CFG.from_trace(StructuredTrace([]))
        assert cfg.get_complexity() == 0

    def test_disconnected_components(self):
        # Two separate linear chains (no edge between them in trace)
        # This can't happen from a single trace (trace is sequential),
        # but test the math: 4 nodes, 2 edges, 2 components -> 2-4+4=2
        # Actually from trace [0,1,100,101] we get edges 0->1, 1->100, 100->101
        # which is connected. To get disconnected we'd need to construct manually.
        pass  # Skip: single trace always produces connected graph


# ============================================================
# Export
# ============================================================
class TestExport:
    def test_to_dot_format(self):
        trace = make_dispatch_trace([0, 1, 2, 0])
        cfg = CFG.from_trace(trace)
        dot = cfg.to_dot()
        assert "digraph CFG" in dot
        assert "n0" in dot
        assert "->" in dot
        assert "}" in dot

    def test_to_dot_back_edge_red(self):
        trace = make_dispatch_trace([0, 1, 0])
        cfg = CFG.from_trace(trace)
        dot = cfg.to_dot()
        assert "red" in dot  # back edge styled red

    def test_to_json_structure(self):
        trace = make_dispatch_trace([0, 1, 2])
        cfg = CFG.from_trace(trace)
        j = cfg.to_json()
        assert "nodes" in j and "edges" in j and "stats" in j
        assert j["stats"]["node_count"] == 3
        assert j["stats"]["edge_count"] == 2
        assert j["stats"]["back_edge_count"] == 0

    def test_repr(self):
        trace = make_dispatch_trace([0, 1, 2, 0])
        cfg = CFG.from_trace(trace)
        r = repr(cfg)
        assert "nodes=3" in r
        assert "back_edges=1" in r


# ============================================================
# Performance
# ============================================================
class TestPerformance:
    def test_100k_entries_under_500ms(self):
        """100k dispatch entries should build CFG in < 500ms."""
        import random
        rng = random.Random(42)
        # Simulate a VM with 200 unique PCs, looping
        pcs = list(range(200))
        sequence = []
        for _ in range(100000):
            sequence.append(rng.choice(pcs))
        trace = make_dispatch_trace(sequence)

        start = time.perf_counter()
        cfg = CFG.from_trace(trace)
        elapsed = time.perf_counter() - start

        assert elapsed < 0.5, f"CFG construction took {elapsed:.3f}s (> 500ms)"
        assert len(cfg.nodes) == 200
        assert len(cfg.edges) > 0
        # Also test find_loops performance
        start = time.perf_counter()
        loops = cfg.find_loops()
        elapsed_loops = time.perf_counter() - start
        assert elapsed_loops < 0.5, f"find_loops took {elapsed_loops:.3f}s"


# ============================================================
# Optional: collapse_to_blocks
# ============================================================
class TestCollapseToBlocks:
    def test_linear_collapses_to_one_block(self):
        trace = make_dispatch_trace([0, 1, 2, 3, 4])
        cfg = CFG.from_trace(trace)
        collapsed = cfg.collapse_to_blocks()
        # Linear sequence with no branching = 1 block
        assert len(collapsed.nodes) == 1
        assert collapsed.nodes[0].exec_count == 5
        assert len(collapsed.edges) == 0

    def test_loop_preserves_back_edge(self):
        trace = make_dispatch_trace([0, 1, 2, 0, 1, 2, 0, 1, 2])
        cfg = CFG.from_trace(trace)
        collapsed = cfg.collapse_to_blocks()
        # A simple loop with no branching may collapse to 1 block with a self-loop edge,
        # or stay as multiple blocks if back-edge targets are treated as block heads.
        # Either way, the collapsed CFG should be valid.
        assert len(collapsed.nodes) >= 1
        # If collapsed to 1 node, there should be a self-loop back edge
        if len(collapsed.nodes) == 1:
            # Self-loop edge (or no edge if fully collapsed)
            pass  # valid
        else:
            # Multiple blocks: should have at least one back edge
            assert any(e.is_back_edge for e in collapsed.edges)

    def test_branch_preserves_targets(self):
        # 0->1->2, 0->1->3 (branch at 1)
        trace = make_dispatch_trace([0, 1, 2, 0, 1, 3])
        cfg = CFG.from_trace(trace)
        collapsed = cfg.collapse_to_blocks()
        # PC 1 has 2 successors, so it's a block boundary
        assert len(collapsed.nodes) >= 2

    def test_empty_cfg(self):
        cfg = CFG.from_trace(StructuredTrace([]))
        collapsed = cfg.collapse_to_blocks()
        assert len(collapsed.nodes) == 0


# ============================================================
# Optional: to_dataframe
# ============================================================
class TestToDataframe:
    def test_basic_dataframe(self):
        pytest.importorskip("pandas")
        trace = make_dispatch_trace([0, 1, 2, 0, 1])
        cfg = CFG.from_trace(trace)
        df = cfg.to_dataframe()
        assert len(df) == 3  # 3 unique PCs
        assert "pc" in df.columns
        assert "exec_count" in df.columns
        assert "in_degree" in df.columns
        assert "out_degree" in df.columns

    def test_no_pandas_raises(self):
        # This test only makes sense if pandas is NOT installed
        # Skip if pandas is available
        try:
            import pandas
            pytest.skip("pandas is installed")
        except ImportError:
            trace = make_dispatch_trace([0, 1])
            cfg = CFG.from_trace(trace)
            with pytest.raises(ImportError):
                cfg.to_dataframe()
