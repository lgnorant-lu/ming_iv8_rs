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

from collections import defaultdict
from dataclasses import dataclass

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

    body_pcs: set[int]
    """All PCs in the loop body."""

    iterations: int
    """Number of back edge traversals (= loop iterations)."""

    back_edges: list[tuple[int, int]]
    """All back edges targeting this header: [(from_pc, to_pc), ...]."""


@dataclass
class Module:
    """A detected module (connected component or PC-gap-separated region)."""

    entry_pc: int
    """First PC in the module."""

    exit_pc: int
    """Last PC in the module."""

    pcs: set[int]
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

    def __init__(self, nodes: dict[int, CFGNode], edges: list[CFGEdge]):
        self.nodes = nodes
        self.edges = edges
        # Build adjacency for traversal
        self._successors: dict[int, list[int]] = defaultdict(list)
        self._predecessors: dict[int, list[int]] = defaultdict(list)
        self._edge_map: dict[tuple[int, int], CFGEdge] = {}
        for e in edges:
            self._successors[e.from_pc].append(e.to_pc)
            self._predecessors[e.to_pc].append(e.from_pc)
            self._edge_map[(e.from_pc, e.to_pc)] = e

    @classmethod
    def from_trace(cls, trace: StructuredTrace) -> CFG:
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
        pc_counts: dict[int, int] = defaultdict(int)
        pc_opcodes: dict[int, dict[int, int]] = defaultdict(lambda: defaultdict(int))

        for d in dispatches:
            pc_counts[d.pc] += 1
            try:
                opc = int(d.target)
            except (ValueError, TypeError):
                opc = -1
            pc_opcodes[d.pc][opc] += 1

        # Build nodes
        nodes: dict[int, CFGNode] = {}
        for pc, count in pc_counts.items():
            # Most common opcode at this PC
            opc_counts = pc_opcodes[pc]
            best_opc = max(opc_counts, key=opc_counts.get) if opc_counts else -1
            nodes[pc] = CFGNode(pc=pc, opcode=best_opc, exec_count=count)

        # Build edges from consecutive dispatch pairs
        edge_counts: dict[tuple[int, int], int] = defaultdict(int)
        for i in range(len(dispatches) - 1):
            from_pc = dispatches[i].pc
            to_pc = dispatches[i + 1].pc
            if from_pc != to_pc or True:  # include self-loops
                edge_counts[(from_pc, to_pc)] += 1

        edges: list[CFGEdge] = []
        for (from_pc, to_pc), count in edge_counts.items():
            edges.append(CFGEdge(
                from_pc=from_pc,
                to_pc=to_pc,
                count=count,
                is_back_edge=(to_pc <= from_pc),
            ))

        return cls(nodes, edges)

    def find_loops(self) -> list[Loop]:
        """Detect loops via back edges (to_pc <= from_pc).

        For each back edge target (loop header), computes the natural loop body
        using the classic algorithm: all predecessors of the back edge source
        that can reach the source without passing through the header.

        Returns:
            List of Loop objects, sorted by iteration count (descending).
        """
        # Group back edges by header
        header_back_edges: dict[int, list[tuple[int, int]]] = defaultdict(list)
        for e in self.edges:
            if e.is_back_edge:
                header_back_edges[e.to_pc].append((e.from_pc, e.to_pc))

        loops: list[Loop] = []
        for header, back_edges in header_back_edges.items():
            # Natural loop body: header + all nodes that can reach back edge
            # sources without going through header (Benno 2024 classic).
            # PC-range constraint prevents unrelated nodes from entering body
            # when entry-block dominance is ambiguous (BUG-11).
            body: set[int] = {header}
            stack: list[int] = []
            total_iterations = 0
            max_body_pc = header  # upper bound for body nodes

            for from_pc, _ in back_edges:
                total_iterations += self._edge_map.get((from_pc, header), CFGEdge(0, 0, 0, False)).count
                if from_pc not in body:
                    body.add(from_pc)
                    stack.append(from_pc)
                if from_pc > max_body_pc:
                    max_body_pc = from_pc

            while stack:
                node = stack.pop()
                for pred in self._predecessors.get(node, []):
                    if pred == header:
                        continue  # BUG-12: stop at loop header boundary
                    if pred not in body and header <= pred <= max_body_pc:  # BUG-11: PC-range guard
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

    def find_modules(self, gap_threshold: int = 100) -> list[Module]:
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
        modules: list[Module] = []
        current_pcs: list[int] = [sorted_pcs[0]]

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
        visited: set[int] = set()
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

    def to_dot(self, path: str | None = None) -> str:
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

    def to_dataframe(self):
        """Convert nodes to pandas DataFrame (optional, requires pandas).

        Returns:
            pd.DataFrame with columns: pc, opcode, exec_count, in_degree, out_degree.

        Raises:
            ImportError: If pandas is not installed.
        """
        try:
            import pandas as pd
        except ImportError:
            raise ImportError("pandas required for to_dataframe(). Install: pip install pandas")

        rows = []
        for pc, node in sorted(self.nodes.items()):
            rows.append({
                "pc": node.pc,
                "opcode": node.opcode,
                "exec_count": node.exec_count,
                "in_degree": len(self._predecessors.get(pc, [])),
                "out_degree": len(self._successors.get(pc, [])),
            })
        return pd.DataFrame(rows)

    def collapse_to_blocks(self) -> CFG:
        """Collapse sequential PCs into basic blocks.

        A basic block is a maximal sequence of PCs where each has exactly
        one predecessor and one successor (no branching). The collapsed CFG
        uses the first PC of each block as the representative node.

        Returns:
            New CFG with fewer nodes (blocks instead of individual PCs).
        """
        if not self.nodes:
            return CFG({}, [])

        # Find block boundaries: PCs with in_degree != 1 or out_degree != 1
        # or that are targets of non-sequential predecessor edges.
        block_heads: set[int] = set()
        sorted_pcs = sorted(self.nodes.keys())
        pc_order = {pc: i for i, pc in enumerate(sorted_pcs)}  # position in sorted PC order

        # First PC is always a block head
        if sorted_pcs:
            block_heads.add(sorted_pcs[0])

        for pc in sorted_pcs:
            preds = self._predecessors.get(pc, [])
            succs = self._successors.get(pc, [])
            # Multiple predecessors or multiple successors = block boundary
            if len(preds) != 1 or len(succs) > 1:
                block_heads.add(pc)
            # Target of a non-sequential edge = block head
            pc_idx = pc_order.get(pc, 0)
            for pred in preds:
                pred_idx = pc_order.get(pred, 0)
                if pred_idx != pc_idx - 1:  # not immediate predecessor in PC order
                    block_heads.add(pc)
                    break

        # Build blocks: each block starts at a head and extends until next head
        blocks: dict[int, list[int]] = {}  # head_pc -> [pcs in block]
        current_head = sorted_pcs[0]
        current_block = [current_head]

        for pc in sorted_pcs[1:]:
            if pc in block_heads:
                blocks[current_head] = current_block
                current_head = pc
                current_block = [pc]
            else:
                current_block.append(pc)
        blocks[current_head] = current_block

        # Build collapsed nodes
        new_nodes: dict[int, CFGNode] = {}
        pc_to_block: dict[int, int] = {}  # map each PC to its block head
        for head, pcs in blocks.items():
            total_exec = sum(self.nodes[p].exec_count for p in pcs if p in self.nodes)
            opc = self.nodes[head].opcode if head in self.nodes else -1
            new_nodes[head] = CFGNode(pc=head, opcode=opc, exec_count=total_exec)
            for p in pcs:
                pc_to_block[p] = head

        # Build collapsed edges
        edge_counts: dict[tuple[int, int], int] = defaultdict(int)
        for e in self.edges:
            from_block = pc_to_block.get(e.from_pc, e.from_pc)
            to_block = pc_to_block.get(e.to_pc, e.to_pc)
            if from_block != to_block:  # skip intra-block edges
                edge_counts[(from_block, to_block)] += e.count

        new_edges = [
            CFGEdge(from_pc=f, to_pc=t, count=c, is_back_edge=(t <= f))
            for (f, t), c in edge_counts.items()
        ]

        return CFG(new_nodes, new_edges)
