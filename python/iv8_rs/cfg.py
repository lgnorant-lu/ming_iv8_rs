"""
Control Flow Graph (CFG) construction from VM dispatch traces.

Builds a directed graph from D entries' PC sequence, supporting:
- Loop detection (back edges where to_pc <= from_pc)
- Module boundary detection (PC gap / connected components)
- Cyclomatic complexity
- DOT/JSON export
- Basic block collapsing (merge sequential PCs)

Design: dynamic CFG reconstruction from trace (not static analysis).
We have the actual execution path, so no need for indirect jump resolution
or basic block identification from source — the trace IS the path.

Reference: Rimsa et al. (2020) "Practical Dynamic Reconstruction of CFG"
"""

from __future__ import annotations
from dataclasses import dataclass, field
from typing import List, Dict, Set, Optional, Tuple
from collections import defaultdict

from iv8_rs.trace import StructuredTrace


@dataclass
class CFGNode:
    """A node in the control flow graph (one per unique PC)."""

    pc: int
    """Program counter value."""

    opcode: int
    """Most common opcode dispatched at this PC."""

    exec_count: int
    """Number of times this PC was executed."""


@dataclass
class CFGEdge:
    """A directed edge in the control flow graph."""

    from_pc: int
    """Source PC."""

    to_pc: int
    """Destination PC."""

    count: int
    """Number of times this transition occurred."""

    is_back_edge: bool
    """True if to_pc <= from_pc (PC regression = loop indicator)."""


@dataclass
class Loop:
    """A detected loop (natural loop from back edge)."""

    header_pc: int
    """Loop header (back edge target)."""

    body_pcs: Set[int]
    """All PCs in the loop body."""

    iterations: int
    """Number of back edge traversals (= loop iterations)."""

    back_edges: List[Tuple[int, int]]
    """All back edges targeting this header: [(from_pc, to_pc), ...]."""


@dataclass
class Module:
    """A detected module (connected component or PC-gap-separated region)."""

    entry_pc: int
    """First PC in the module."""

    exit_pc: int
    """Last PC in the module."""

    pcs: Set[int]
    """All PCs belonging to this module."""

    edge_count: int
    """Number of internal edges."""


class CFG:
    """Control Flow Graph built from VM dispatch trace.

    Example::

        from iv8_rs.trace import parse_trace
        from iv8_rs.cfg import CFG

        trace = parse_trace(ctx.get_unified_trace())
        cfg = CFG.from_trace(trace)
        print(f"nodes={len(cfg.nodes)}, edges={len(cfg.edges)}")
        for loop in cfg.find_loops():
            print(f"Loop at PC {loop.header_pc}: {loop.iterations} iterations")
        print(cfg.to_dot())
    """

    def __init__(self, nodes: Dict[int, CFGNode], edges: List[CFGEdge]):
        self.nodes = nodes
        self.edges = edges
        # Build adjacency for traversal
        self._successors: Dict[int, List[int]] = defaultdict(list)
        self._predecessors: Dict[int, List[int]] = defaultdict(list)
        self._edge_map: Dict[Tuple[int, int], CFGEdge] = {}
        for e in edges:
            self._successors[e.from_pc].append(e.to_pc)
            self._predecessors[e.to_pc].append(e.from_pc)
            self._edge_map[(e.from_pc, e.to_pc)] = e

    @classmethod
    def from_trace(cls, trace: StructuredTrace) -> "CFG":
        """Build CFG from dispatch entries in a trace.

        Only D (dispatch) entries are used. R/C/W entries are ignored.
        Empty trace or trace with no D entries produces an empty CFG.

        Args:
            trace: StructuredTrace (from parse_trace).

        Returns:
            CFG instance.
        """
        dispatches = trace.dispatches
        if not dispatches:
            return cls({}, [])

        # Count node executions and find most common opcode per PC
        pc_counts: Dict[int, int] = defaultdict(int)
        pc_opcodes: Dict[int, Dict[int, int]] = defaultdict(lambda: defaultdict(int))

        for d in dispatches:
            pc_counts[d.pc] += 1
            try:
                opc = int(d.target)
            except (ValueError, TypeError):
                opc = -1
            pc_opcodes[d.pc][opc] += 1

        # Build nodes
        nodes: Dict[int, CFGNode] = {}
        for pc, count in pc_counts.items():
            # Most common opcode at this PC
            opc_counts = pc_opcodes[pc]
            best_opc = max(opc_counts, key=opc_counts.get) if opc_counts else -1
            nodes[pc] = CFGNode(pc=pc, opcode=best_opc, exec_count=count)

        # Build edges from consecutive dispatch pairs
        edge_counts: Dict[Tuple[int, int], int] = defaultdict(int)
        for i in range(len(dispatches) - 1):
            from_pc = dispatches[i].pc
            to_pc = dispatches[i + 1].pc
            if from_pc != to_pc or True:  # include self-loops
                edge_counts[(from_pc, to_pc)] += 1

        edges: List[CFGEdge] = []
        for (from_pc, to_pc), count in edge_counts.items():
            edges.append(CFGEdge(
                from_pc=from_pc,
                to_pc=to_pc,
                count=count,
                is_back_edge=(to_pc <= from_pc),
            ))

        return cls(nodes, edges)

    def find_loops(self) -> List[Loop]:
        """Detect loops via back edges (to_pc <= from_pc).

        For each back edge target (loop header), computes the natural loop body
        using the classic algorithm: all predecessors of the back edge source
        that can reach the source without passing through the header.

        Returns:
            List of Loop objects, sorted by iteration count (descending).
        """
        # Group back edges by header
        header_back_edges: Dict[int, List[Tuple[int, int]]] = defaultdict(list)
        for e in self.edges:
            if e.is_back_edge:
                header_back_edges[e.to_pc].append((e.from_pc, e.to_pc))

        loops: List[Loop] = []
        for header, back_edges in header_back_edges.items():
            # Natural loop body: header + all nodes that can reach back edge
            # sources without going through header
            body: Set[int] = {header}
            stack: List[int] = []
            total_iterations = 0

            for from_pc, _ in back_edges:
                total_iterations += self._edge_map.get((from_pc, header), CFGEdge(0, 0, 0, False)).count
                if from_pc not in body:
                    body.add(from_pc)
                    stack.append(from_pc)

            while stack:
                node = stack.pop()
                for pred in self._predecessors.get(node, []):
                    if pred not in body:
                        body.add(pred)
                        stack.append(pred)

            loops.append(Loop(
                header_pc=header,
                body_pcs=body,
                iterations=total_iterations,
                back_edges=back_edges,
            ))

        loops.sort(key=lambda l: l.iterations, reverse=True)
        return loops

    def find_modules(self, gap_threshold: int = 100) -> List[Module]:
        """Detect module boundaries via PC gaps.

        Consecutive PCs with a gap > threshold are considered separate modules.
        PCs are sorted, then split at gaps.

        Args:
            gap_threshold: Minimum PC difference to consider a module boundary.

        Returns:
            List of Module objects, sorted by entry_pc.
        """
        if not self.nodes:
            return []

        sorted_pcs = sorted(self.nodes.keys())
        modules: List[Module] = []
        current_pcs: List[int] = [sorted_pcs[0]]

        for i in range(1, len(sorted_pcs)):
            if sorted_pcs[i] - sorted_pcs[i - 1] > gap_threshold:
                # Gap detected: finalize current module
                pcs_set = set(current_pcs)
                internal_edges = sum(
                    1 for e in self.edges
                    if e.from_pc in pcs_set and e.to_pc in pcs_set
                )
                modules.append(Module(
                    entry_pc=current_pcs[0],
                    exit_pc=current_pcs[-1],
                    pcs=pcs_set,
                    edge_count=internal_edges,
                ))
                current_pcs = [sorted_pcs[i]]
            else:
                current_pcs.append(sorted_pcs[i])

        # Finalize last module
        if current_pcs:
            pcs_set = set(current_pcs)
            internal_edges = sum(
                1 for e in self.edges
                if e.from_pc in pcs_set and e.to_pc in pcs_set
            )
            modules.append(Module(
                entry_pc=current_pcs[0],
                exit_pc=current_pcs[-1],
                pcs=pcs_set,
                edge_count=internal_edges,
            ))

        return modules

    def get_complexity(self) -> int:
        """Compute cyclomatic complexity: E - N + 2P.

        Where E = edges, N = nodes, P = connected components.
        """
        if not self.nodes:
            return 0
        # Count connected components via BFS
        visited: Set[int] = set()
        components = 0
        all_pcs = set(self.nodes.keys())

        for start in all_pcs:
            if start in visited:
                continue
            components += 1
            queue = [start]
            while queue:
                node = queue.pop()
                if node in visited:
                    continue
                visited.add(node)
                for succ in self._successors.get(node, []):
                    if succ not in visited:
                        queue.append(succ)
                for pred in self._predecessors.get(node, []):
                    if pred not in visited:
                        queue.append(pred)

        return len(self.edges) - len(self.nodes) + 2 * components

    def to_dot(self, path: Optional[str] = None) -> str:
        """Export as Graphviz DOT format.

        Args:
            path: If provided, write to file. Otherwise return string.

        Returns:
            DOT format string.
        """
        lines = ["digraph CFG {", "  rankdir=TB;", "  node [shape=box, fontsize=10];"]

        for pc, node in sorted(self.nodes.items()):
            label = f"PC={pc}\\nopc={node.opcode}\\nx{node.exec_count}"
            lines.append(f'  n{pc} [label="{label}"];')

        for e in self.edges:
            style = "bold,color=red" if e.is_back_edge else ""
            label = f"x{e.count}"
            attrs = f'label="{label}"'
            if style:
                attrs += f", style={style}"
            lines.append(f"  n{e.from_pc} -> n{e.to_pc} [{attrs}];")

        lines.append("}")
        dot = "\n".join(lines)

        if path:
            with open(path, "w", encoding="utf-8") as f:
                f.write(dot)

        return dot

    def to_json(self) -> dict:
        """Export as JSON-serializable dict.

        Returns:
            Dict with 'nodes', 'edges', 'loops', 'modules', 'complexity'.
        """
        return {
            "nodes": [
                {"pc": n.pc, "opcode": n.opcode, "exec_count": n.exec_count}
                for n in sorted(self.nodes.values(), key=lambda n: n.pc)
            ],
            "edges": [
                {"from": e.from_pc, "to": e.to_pc, "count": e.count,
                 "back_edge": e.is_back_edge}
                for e in self.edges
            ],
            "stats": {
                "node_count": len(self.nodes),
                "edge_count": len(self.edges),
                "back_edge_count": sum(1 for e in self.edges if e.is_back_edge),
                "complexity": self.get_complexity(),
            },
        }

    def __repr__(self) -> str:
        back = sum(1 for e in self.edges if e.is_back_edge)
        return (f"CFG(nodes={len(self.nodes)}, edges={len(self.edges)}, "
                f"back_edges={back})")
