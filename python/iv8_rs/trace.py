"""
Structured Trace module for iv8-rs.

Parses raw unified trace strings (D/R/C/W entries) into typed objects
with filtering, slicing, statistics, and export capabilities.
"""

from __future__ import annotations
from dataclasses import dataclass, field
from typing import List, Optional, Dict, Any, Iterator
import json


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


def _parse_entry(raw: str) -> Optional[TraceEntry]:
    """Parse a single raw trace string into a TraceEntry."""
    if not raw or len(raw) < 3:
        return None
    parts = raw.split(",", 3)
    if len(parts) < 3:
        return None
    entry_type = parts[0]
    if entry_type not in ("D", "R", "C", "W"):
        return None
    try:
        pc = int(parts[1])
    except (ValueError, IndexError):
        pc = -1
    target = parts[2] if len(parts) > 2 else ""
    value = parts[3] if len(parts) > 3 else ""
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

    def __init__(self, entries: List[TraceEntry]):
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
    def dispatches(self) -> List[TraceEntry]:
        """All D (dispatch) entries."""
        return [e for e in self.entries if e.type == "D"]

    @property
    def reads(self) -> List[TraceEntry]:
        """All R (read) entries."""
        return [e for e in self.entries if e.type == "R"]

    @property
    def calls(self) -> List[TraceEntry]:
        """All C (call) entries."""
        return [e for e in self.entries if e.type == "C"]

    @property
    def writes(self) -> List[TraceEntry]:
        """All W (write) entries."""
        return [e for e in self.entries if e.type == "W"]

    # --- Filtering ---

    def filter(
        self,
        type: Optional[str] = None,
        target: Optional[str] = None,
        pc_range: Optional[tuple] = None,
    ) -> "StructuredTrace":
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

    def between(self, pc_start: int, pc_end: int) -> "StructuredTrace":
        """Slice entries within a PC range (inclusive).

        Args:
            pc_start: Start PC (inclusive).
            pc_end: End PC (inclusive).

        Returns:
            New StructuredTrace with entries in range.
        """
        return StructuredTrace([e for e in self.entries if pc_start <= e.pc <= pc_end])

    # --- Statistics ---

    def summary(self) -> Dict[str, Any]:
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

    def __repr__(self) -> str:
        s = self.summary()
        return (
            f"StructuredTrace({s['total']} entries: "
            f"D={s['counts_by_type'].get('D', 0)}, "
            f"R={s['counts_by_type'].get('R', 0)}, "
            f"C={s['counts_by_type'].get('C', 0)}, "
            f"W={s['counts_by_type'].get('W', 0)})"
        )


def parse_trace(raw: List[str]) -> StructuredTrace:
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
