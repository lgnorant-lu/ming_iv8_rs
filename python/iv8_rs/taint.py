"""
Taint Tracking: trace value propagation from sources to sinks (M26).

Tracks how specific input values (e.g. screen.width=1920) flow through
VM execution and reach output fields (e.g. cd[10]).

Method: value-matching propagation (not instruction-level dataflow).
Searches for source values in D entry stack values and W entry outputs.
Does NOT require opcode semantics — works with any trace that has stack values.

Precision: coarse-grained (value identity, not dataflow). May have false
positives if the same numeric value appears coincidentally. Does not track
value transformations (hash/encrypt breaks the chain).

Usage::

    from iv8_rs.taint import TaintEngine

    engine = TaintEngine(trace, sources={
        "screen.width": "1920",
        "screen.height": "1080",
    })
    report = engine.analyze()
    for flow in report.flows:
        print(f"{flow.source.label} -> {flow.sink.target} (via {len(flow.intermediate_pcs)} PCs)")
"""

from __future__ import annotations

from dataclasses import dataclass

from iv8_rs.trace import StructuredTrace


@dataclass
class TaintSource:
    """A tainted input value."""

    label: str
    """Short label (e.g. 'SW' for screen.width)."""

    target: str
    """Full target path (e.g. 'screen.width')."""

    value: str
    """The value to track (string representation)."""

    pc: int
    """PC where first observed in trace (-1 if user-specified, not from R entry)."""


@dataclass
class TaintSink:
    """A location where a tainted value was written."""

    label: str
    """Source label that reached this sink."""

    target: str
    """Write target (e.g. 'cd[10]' or property path)."""

    value: str
    """Value written."""

    pc: int
    """PC of the W entry."""


@dataclass
class TaintFlow:
    """A complete flow from source to sink."""

    source: TaintSource
    """Origin of the tainted value."""

    sink: TaintSink
    """Destination where the value arrived."""

    intermediate_pcs: list[int]
    """PCs of D entries where the value appeared in stack (propagation path)."""


@dataclass
class TaintReport:
    """Result of taint analysis."""

    sources: list[TaintSource]
    """All registered taint sources."""

    sinks: list[TaintSink]
    """All detected sinks (where tainted values were written)."""

    flows: list[TaintFlow]
    """Complete source-to-sink flows."""

    unreached_sources: list[str]
    """Source labels that never reached any sink."""

    stack_hits: dict[str, int]
    """Per-source label: how many D entries contained the value in stack."""


class TaintEngine:
    """Value-matching taint propagation engine.

    Tracks specified values through the trace by searching for them in
    D entry stack values and W entry outputs.

    Value matching uses exact token comparison against comma-split entry values
    to prevent false positives from Python substring matching (BUG-14).

    Args:
        trace: StructuredTrace to analyze (should have stack values in D entries
               for propagation tracking; works without but only finds R\u2192W direct).
        sources: Dict mapping target path to value string, e.g.
                 {"screen.width": "1920", "navigator.userAgent": "Mozilla..."}.
                 Values are matched as exact tokens in comma-split entry values.

    Example::

        engine = TaintEngine(trace, sources={"screen.width": "1920"})
        report = engine.analyze()
        print(report.flows)
    """

    def __init__(self, trace: StructuredTrace, sources: dict[str, str]):
        self.trace = trace
        self._sources = sources
        # Generate short labels from target names
        self._labels: dict[str, str] = {}
        for target in sources:
            parts = target.split(".")
            label = parts[-1][:6].upper() if parts else target[:6].upper()
            # Ensure unique
            base = label
            i = 2
            while label in self._labels.values():
                label = f"{base}{i}"
                i += 1
            self._labels[target] = label

    @staticmethod
    def _value_matches(src_val: str, entry_val: str) -> bool:
        """Exact or token match: prevents substring false positives (BUG-14).

        For comma-separated entry values (D/W stack values), split and check
        each token. For plain entry values (R reads), do exact match.
        """
        if not src_val or not entry_val:
            return False
        if "," in entry_val:
            tokens = [t.strip() for t in entry_val.split(",")]
            return src_val in tokens
        return src_val == entry_val

    def analyze(self) -> TaintReport:
        """Run taint analysis: find where source values appear and reach sinks.

        Returns:
            TaintReport with sources, sinks, flows, and unreached sources.
        """
        # Build TaintSource objects
        taint_sources: list[TaintSource] = []
        for target, value in self._sources.items():
            # Try to find the value in R entries (to get a PC)
            pc = -1
            for entry in self.trace.reads:
                if entry.target == target or target in entry.target:
                    if self._value_matches(value, entry.value):
                        pc = entry.pc
                        break
            taint_sources.append(TaintSource(
                label=self._labels[target],
                target=target,
                value=value,
                pc=pc,
            ))

        # Track each source value through D entries (stack values)
        stack_hits: dict[str, int] = {s.label: 0 for s in taint_sources}
        # intermediate_pcs per source label
        intermediates: dict[str, list[int]] = {s.label: [] for s in taint_sources}

        for entry in self.trace.dispatches:
            # D entry value field may contain: "depth,val1,val2,val3"
            entry_val = entry.value
            if not entry_val:
                continue
            for src in taint_sources:
                if self._value_matches(src.value, entry_val):
                    stack_hits[src.label] += 1
                    intermediates[src.label].append(entry.pc)

        # Find sinks: W entries whose value matches a source value
        sinks: list[TaintSink] = []
        for entry in self.trace.writes:
            for src in taint_sources:
                if self._value_matches(src.value, entry.value):
                    sinks.append(TaintSink(
                        label=src.label,
                        target=entry.target,
                        value=entry.value,
                        pc=entry.pc,
                    ))

        # Also check C entries as potential sinks (function calls with tainted args)
        for entry in self.trace.calls:
            for src in taint_sources:
                if self._value_matches(src.value, entry.value):
                    sinks.append(TaintSink(
                        label=src.label,
                        target=entry.target,
                        value=entry.value,
                        pc=entry.pc,
                    ))

        # Build flows: source → intermediate → sink
        flows: list[TaintFlow] = []
        for sink in sinks:
            # Find the source that matches this sink's label
            src = next((s for s in taint_sources if s.label == sink.label), None)
            if src:
                # Get intermediate PCs that are between source PC and sink PC
                inter = intermediates.get(src.label, [])
                relevant_inter = [pc for pc in inter if pc <= sink.pc]
                flows.append(TaintFlow(
                    source=src,
                    sink=sink,
                    intermediate_pcs=relevant_inter[:50],  # cap for display
                ))

        # Unreached: sources with no sinks
        reached_labels = {s.label for s in sinks}
        unreached = [s.label for s in taint_sources if s.label not in reached_labels]

        return TaintReport(
            sources=taint_sources,
            sinks=sinks,
            flows=flows,
            unreached_sources=unreached,
            stack_hits=stack_hits,
        )
