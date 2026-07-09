"""
Structured Trace module for iv8-rs.

Parses raw unified trace strings (D/R/C/W entries) into typed objects
with filtering, slicing, statistics, and export capabilities.
"""

from __future__ import annotations

import json
from collections.abc import Iterator
from dataclasses import dataclass
from typing import Any


@dataclass(slots=True)
class TraceEntry:
    """A single trace entry (dispatch, read, call, or write)."""

    type: str
    """Entry type: 'D' (dispatch), 'R' (read), 'C' (call), 'W' (write)."""

    pc: int
    """Program counter. -1 if not applicable."""

    target: str
    """Opcode number (D), property path (R/W), or function name (C)."""

    value: str
    """Stack depth (D), property value (R/W), or return value (C)."""

    raw: str
    """Original raw string."""

    @property
    def is_dispatch(self) -> bool:
        return self.type == "D"

    @property
    def is_read(self) -> bool:
        return self.type == "R"

    @property
    def is_call(self) -> bool:
        return self.type == "C"

    @property
    def is_write(self) -> bool:
        return self.type == "W"


def _parse_entry(raw: str) -> TraceEntry | None:
    """Parse a single raw trace string into a TraceEntry.

    Handles two formats:
      4-field (instrument_source / get_unified_trace): "TYPE,pc,target,value"
      3-field (start_recording / stop_recording):       "TYPE,target,value"
    Detection: after split with maxsplit=3, if there are 4 fields, it's 4-field.
    """
    if not raw or len(raw) < 3:
        return None
    parts = raw.split(",", 3)
    if len(parts) < 3:
        return None
    entry_type = parts[0]
    if entry_type not in ("D", "R", "C", "W"):
        return None

    if len(parts) >= 4:
        # 4-field: TYPE,pc,target,value
        # pc must be an integer. If not:
        # - For D entries: parts[1] is an opcode (3-field D,opcode,value),
        #   so target=parts[2], value=parts[3] (original 4-field fallback).
        # - For C/R/W entries: parts[1] is the target (e.g.
        #   "crypto.getRandomValues" in C,crypto.getRandomValues,36,212,...),
        #   so we fall back to 3-field: target=parts[1], value=parts[2:] joined.
        try:
            pc = int(parts[1])
        except ValueError:
            pc = -1
            if entry_type == "D":
                # D,opcode,... format: keep 4-field interpretation
                target = parts[2]
                value = parts[3]
            else:
                # C/R/W with non-integer "pc" → 3-field format
                # target contains a dot, value has commas
                target = parts[1]
                value = ",".join(parts[2:])
        else:
            target = parts[2]
            value = parts[3]
    else:
        # 3-field: TYPE,target,value  (no PC; for D entries, second field is opcode)
        pc = -1
        target = parts[1]
        value = parts[2]

    return TraceEntry(type=entry_type, pc=pc, target=target, value=value, raw=raw)


class StructuredTrace:
    """Parsed and queryable trace data.

    Provides typed access, filtering, slicing, statistics, and export
    for unified trace entries (D/R/C/W format).

    Example::

        from iv8_rs.trace import parse_trace
        raw = ctx.get_unified_trace()
        trace = parse_trace(raw)
        print(trace.summary())
        for r in trace.reads:
            if 'screen' in r.target:
                print(f"  {r.target} = {r.value} at PC={r.pc}")
    """

    __slots__ = ("entries",)

    def __init__(self, entries: list[TraceEntry]):
        self.entries = entries

    def __len__(self) -> int:
        return len(self.entries)

    def __iter__(self) -> Iterator[TraceEntry]:
        return iter(self.entries)

    def __getitem__(self, index):
        if isinstance(index, slice):
            return StructuredTrace(self.entries[index])
        return self.entries[index]

    # --- Type-filtered views ---

    @property
    def dispatches(self) -> list[TraceEntry]:
        """All D (dispatch) entries."""
        return [e for e in self.entries if e.type == "D"]

    @property
    def reads(self) -> list[TraceEntry]:
        """All R (read) entries."""
        return [e for e in self.entries if e.type == "R"]

    @property
    def calls(self) -> list[TraceEntry]:
        """All C (call) entries."""
        return [e for e in self.entries if e.type == "C"]

    @property
    def writes(self) -> list[TraceEntry]:
        """All W (write) entries."""
        return [e for e in self.entries if e.type == "W"]

    # --- Filtering ---

    def filter(
        self,
        type: str | None = None,
        target: str | None = None,
        pc_range: tuple | None = None,
    ) -> StructuredTrace:
        """Filter entries by type, target pattern, and/or PC range.

        Args:
            type: Entry type to keep ('D', 'R', 'C', 'W'), or None for all.
            target: Substring match on target field, or None for all.
            pc_range: Tuple (start_pc, end_pc) inclusive, or None for all.

        Returns:
            New StructuredTrace with matching entries.
        """
        result = self.entries
        if type is not None:
            result = [e for e in result if e.type == type]
        if target is not None:
            result = [e for e in result if target in e.target]
        if pc_range is not None:
            lo, hi = pc_range
            result = [e for e in result if lo <= e.pc <= hi]
        return StructuredTrace(result)

    def between(self, pc_start: int, pc_end: int) -> StructuredTrace:
        """Slice entries within a PC range (inclusive).

        Args:
            pc_start: Start PC (inclusive).
            pc_end: End PC (inclusive).

        Returns:
            New StructuredTrace with entries in range.
        """
        return StructuredTrace([e for e in self.entries if pc_start <= e.pc <= pc_end])

    # --- Statistics ---

    def summary(self) -> dict[str, Any]:
        """Compute summary statistics.

        Returns:
            Dict with: total, counts_by_type, pc_range, unique_targets, unique_opcodes.
        """
        if not self.entries:
            return {"total": 0, "counts_by_type": {}, "pc_range": None,
                    "unique_targets": 0, "unique_opcodes": 0}

        counts = {"D": 0, "R": 0, "C": 0, "W": 0}
        pcs = []
        targets = set()
        opcodes = set()

        for e in self.entries:
            counts[e.type] = counts.get(e.type, 0) + 1
            if e.pc >= 0:
                pcs.append(e.pc)
            if e.type != "D":
                targets.add(e.target)
            else:
                opcodes.add(e.target)

        return {
            "total": len(self.entries),
            "counts_by_type": counts,
            "pc_range": (min(pcs), max(pcs)) if pcs else None,
            "unique_targets": len(targets),
            "unique_opcodes": len(opcodes),
        }

    # --- Export ---

    def to_jsonl(self, path: str) -> None:
        """Export to JSON Lines file.

        Each line is a JSON object: {type, pc, target, value}.
        """
        with open(path, "w", encoding="utf-8") as f:
            for e in self.entries:
                f.write(json.dumps(
                    {"type": e.type, "pc": e.pc, "target": e.target, "value": e.value},
                    ensure_ascii=False,
                ) + "\n")

    def to_dataframe(self):
        """Convert to pandas DataFrame (requires pandas).

        Returns:
            pd.DataFrame with columns: type, pc, target, value.

        Raises:
            ImportError: If pandas is not installed.
        """
        try:
            import pandas as pd
        except ImportError:
            raise ImportError("pandas is required for to_dataframe(). Install with: pip install pandas")
        return pd.DataFrame(
            [{"type": e.type, "pc": e.pc, "target": e.target, "value": e.value} for e in self.entries]
        )

    # --- Sequence extraction (for pattern matching) ---

    def pc_sequence(self) -> list[int]:
        """Extract PC sequence from dispatch entries only.

        Returns:
            List of PCs in execution order (D entries only).
            Useful for CFG construction.
        """
        return [e.pc for e in self.entries if e.type == "D"]

    def value_sequence(self) -> list[str]:
        """Extract value sequence from all entries.

        Returns:
            List of value strings in trace order.
            Useful for Layer 2 sequence matching.
        """
        return [e.value for e in self.entries if e.value]

    def unique_pcs(self) -> set:
        """Get set of all unique PCs visited.

        Returns:
            Set of PC values from D entries.
        """
        return {e.pc for e in self.entries if e.type == "D" and e.pc >= 0}

    def index_by_pc(self) -> dict[int, list[TraceEntry]]:
        """Build index: PC -> list of entries at that PC.

        Returns:
            Dict mapping PC to all entries (D/R/C/W) at that PC.
            O(1) lookup after construction.
        """
        idx: dict[int, list[TraceEntry]] = {}
        for e in self.entries:
            if e.pc >= 0:
                idx.setdefault(e.pc, []).append(e)
        return idx

    def index_by_target(self) -> dict[str, list[TraceEntry]]:
        """Build index: target -> list of entries with that target.

        Returns:
            Dict mapping target string to all entries referencing it.
            O(1) lookup after construction.
        """
        idx: dict[str, list[TraceEntry]] = {}
        for e in self.entries:
            if e.target:
                idx.setdefault(e.target, []).append(e)
        return idx

    def __repr__(self) -> str:
        s = self.summary()
        return (
            f"StructuredTrace({s['total']} entries: "
            f"D={s['counts_by_type'].get('D', 0)}, "
            f"R={s['counts_by_type'].get('R', 0)}, "
            f"C={s['counts_by_type'].get('C', 0)}, "
            f"W={s['counts_by_type'].get('W', 0)})"
        )


def parse_trace(raw: list[str]) -> StructuredTrace:
    """Parse raw unified trace strings into a StructuredTrace.

    Args:
        raw: List of raw trace strings from ctx.get_unified_trace().

    Returns:
        StructuredTrace with parsed entries (invalid lines skipped).

    Example::

        raw = ctx.get_unified_trace()
        trace = parse_trace(raw)
        print(trace)  # StructuredTrace(100000 entries: D=99933, R=33, C=34, W=0)
    """
    entries = []
    for line in raw:
        entry = _parse_entry(line)
        if entry is not None:
            entries.append(entry)
    return StructuredTrace(entries)


def parse_trace_stream(iterable) -> StructuredTrace:
    """Parse trace from any iterable (file, generator, etc.).

    Supports streaming from large files without loading all into memory first.
    For files > 1M entries, prefer this over parse_trace(list).

    Args:
        iterable: Any iterable yielding raw trace strings (file object,
                  generator, list, etc.).

    Returns:
        StructuredTrace with parsed entries.

    Example::

        # From file
        with open("trace.log") as f:
            trace = parse_trace_stream(f)

        # From generator
        def gen():
            yield "D,100,5,3"
            yield "R,100,screen.width,1920"
        trace = parse_trace_stream(gen())
    """
    entries = []
    for line in iterable:
        if isinstance(line, bytes):
            line = line.decode("utf-8", errors="replace")
        line = line.rstrip("\n\r")
        entry = _parse_entry(line)
        if entry is not None:
            entries.append(entry)
    return StructuredTrace(entries)


@dataclass(slots=True)
class CompressedEntry:
    """A compressed dispatch entry (consecutive same-PC dispatches merged)."""

    type: str
    pc: int
    target: str
    value: str
    count: int
    """Number of consecutive dispatches at this PC."""


class CompressedTrace:
    """Memory-efficient trace with consecutive same-PC dispatches merged.

    Reduces memory for traces with tight loops (e.g. dispatch loop
    executing 50000 times at the same PC → 1 CompressedEntry with count=50000).
    """

    __slots__ = ("entries",)

    def __init__(self, entries: list[CompressedEntry]):
        self.entries = entries

    def __len__(self) -> int:
        return len(self.entries)

    @property
    def total_dispatches(self) -> int:
        """Total dispatch count (sum of all counts)."""
        return sum(e.count for e in self.entries if e.type == "D")

    @property
    def compression_ratio(self) -> float:
        """Ratio of compressed entries to original (lower = better compression)."""
        total = sum(e.count for e in self.entries)
        return len(self.entries) / total if total > 0 else 1.0

    def expand(self) -> StructuredTrace:
        """Expand back to full StructuredTrace (for compatibility)."""
        expanded = []
        for ce in self.entries:
            raw = f"{ce.type},{ce.pc},{ce.target},{ce.value}"
            for _ in range(ce.count):
                expanded.append(TraceEntry(
                    type=ce.type, pc=ce.pc, target=ce.target,
                    value=ce.value, raw=raw,
                ))
        return StructuredTrace(expanded)

    def __repr__(self) -> str:
        total = sum(e.count for e in self.entries)
        return f"CompressedTrace({len(self.entries)} entries, {total} original, ratio={self.compression_ratio:.3f})"


def compress_trace(trace: StructuredTrace) -> CompressedTrace:
    """Compress a trace by merging consecutive same-PC dispatch entries.

    Non-dispatch entries (R/C/W) are kept as-is with count=1.
    Consecutive D entries at the same PC+target+value are merged into one
    with count=N. Entries with differing values are preserved separately
    to avoid losing intermediate state transitions (BUG-18).

    Args:
        trace: StructuredTrace to compress.

    Returns:
        CompressedTrace with merged entries.

    Example::

        trace = parse_trace(raw)
        compressed = compress_trace(trace)
        print(compressed)  # CompressedTrace(500 entries, 100000 original, ratio=0.005)
    """
    if not trace.entries:
        return CompressedTrace([])

    result: list[CompressedEntry] = []
    prev = trace.entries[0]
    count = 1

    for entry in trace.entries[1:]:
        if (entry.type == "D" and prev.type == "D"
                and entry.pc == prev.pc and entry.target == prev.target
                and entry.value == prev.value):  # BUG-18: same value required
            count += 1
        else:
            result.append(CompressedEntry(
                type=prev.type, pc=prev.pc, target=prev.target,
                value=prev.value, count=count,
            ))
            prev = entry
            count = 1

    # Don't forget the last entry
    result.append(CompressedEntry(
        type=prev.type, pc=prev.pc, target=prev.target,
        value=prev.value, count=count,
    ))

    return CompressedTrace(result)
