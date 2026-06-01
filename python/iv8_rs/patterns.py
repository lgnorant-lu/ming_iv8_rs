"""
Pattern Matching module for iv8-rs.

Detects known algorithmic patterns (crypto, hash, cipher) in VM trace data
by matching opcode sequences against a configurable pattern library.
"""

from __future__ import annotations
from dataclasses import dataclass
from typing import List, Dict, Optional, Any
from pathlib import Path
import json

from iv8_rs.trace import StructuredTrace, TraceEntry


@dataclass
class PatternMatch:
    """A detected pattern match in the trace."""

    name: str
    """Pattern name (e.g. 'XTEA', 'MD5_ROUND')."""

    description: str
    """Human-readable description."""

    pc_start: int
    """Start PC of the match."""

    pc_end: int
    """End PC of the match."""

    confidence: float
    """Match confidence (0.0 - 1.0)."""

    matched_opcodes: List[int]
    """The actual opcode sequence that matched."""

    window_index: int
    """Index in the dispatch sequence where match starts."""


def _load_builtin_patterns() -> Dict[str, Any]:
    """Load the built-in crypto pattern library."""
    data_dir = Path(__file__).parent / "data"
    pattern_file = data_dir / "crypto_patterns.json"
    if pattern_file.exists():
        with open(pattern_file, "r", encoding="utf-8") as f:
            return json.load(f)
    return {}


def _sequence_similarity(seq_a: List[int], seq_b: List[int]) -> float:
    """Compute similarity between two opcode sequences (0.0 - 1.0)."""
    if not seq_a or not seq_b:
        return 0.0
    min_len = min(len(seq_a), len(seq_b))
    matches = sum(1 for i in range(min_len) if seq_a[i] == seq_b[i])
    return matches / max(len(seq_a), len(seq_b))


def detect_patterns(
    trace: StructuredTrace,
    patterns: Optional[Dict[str, Any]] = None,
    window_size: int = 20,
    min_confidence: float = 0.6,
) -> List[PatternMatch]:
    """Detect known algorithmic patterns in a trace.

    Scans the dispatch entries with a sliding window, comparing opcode
    sequences against known patterns (crypto, hash, cipher).

    Args:
        trace: StructuredTrace to analyze.
        patterns: Custom pattern dict (same format as crypto_patterns.json).
                  If None, uses built-in library.
        window_size: Sliding window size for scanning (default 20).
        min_confidence: Minimum confidence threshold (default 0.6).

    Returns:
        List of PatternMatch objects, sorted by confidence (descending).

    Example::

        from iv8_rs.trace import parse_trace
        from iv8_rs.patterns import detect_patterns

        trace = parse_trace(ctx.get_unified_trace())
        matches = detect_patterns(trace)
        for m in matches:
            print(f"{m.name} at PC {m.pc_start}-{m.pc_end} (conf={m.confidence:.2f})")
    """
    if patterns is None:
        patterns = _load_builtin_patterns()

    dispatches = trace.dispatches
    if not dispatches:
        return []

    # Extract opcode sequence
    opcodes = []
    pcs = []
    for d in dispatches:
        try:
            opcodes.append(int(d.target))
            pcs.append(d.pc)
        except (ValueError, TypeError):
            opcodes.append(-1)
            pcs.append(d.pc)

    matches: List[PatternMatch] = []

    for pattern_name, pattern_def in patterns.items():
        if pattern_name.startswith("_"):
            continue  # Skip metadata entries
        pat_seq = pattern_def.get("opcode_sequence") or pattern_def.get("behavior_pattern", [])
        pat_min_conf = pattern_def.get("min_confidence", min_confidence)
        pat_window = pattern_def.get("min_window", len(pat_seq))
        description = pattern_def.get("description", "")

        if not pat_seq:
            continue

        # Sliding window scan
        for i in range(len(opcodes) - pat_window + 1):
            window = opcodes[i:i + len(pat_seq)]
            conf = _sequence_similarity(window, pat_seq)

            if conf >= max(pat_min_conf, min_confidence):
                matches.append(PatternMatch(
                    name=pattern_name,
                    description=description,
                    pc_start=pcs[i],
                    pc_end=pcs[min(i + len(pat_seq) - 1, len(pcs) - 1)],
                    confidence=conf,
                    matched_opcodes=window,
                    window_index=i,
                ))

    # Deduplicate overlapping matches (keep highest confidence)
    matches.sort(key=lambda m: m.confidence, reverse=True)
    deduplicated = []
    used_ranges = set()
    for m in matches:
        key = (m.name, m.window_index // (len(patterns.get(m.name, {}).get("opcode_sequence", [1])) or 1))
        if key not in used_ranges:
            deduplicated.append(m)
            used_ranges.add(key)

    return deduplicated


def detect_loops(trace: StructuredTrace, min_iterations: int = 10) -> List[Dict[str, Any]]:
    """Detect repeated opcode patterns (loops) in dispatch trace.

    Args:
        trace: StructuredTrace to analyze.
        min_iterations: Minimum repetitions to count as a loop (default 10).

    Returns:
        List of detected loops: [{pc, opcode, count, type}]
    """
    dispatches = trace.dispatches
    if not dispatches:
        return []

    # Count how many times each PC is visited
    pc_counts: Dict[int, int] = {}
    for d in dispatches:
        pc_counts[d.pc] = pc_counts.get(d.pc, 0) + 1

    loops = []
    for pc, count in sorted(pc_counts.items(), key=lambda x: -x[1]):
        if count >= min_iterations:
            # Find the opcode at this PC
            opcode = next((d.target for d in dispatches if d.pc == pc), "?")
            loops.append({
                "pc": pc,
                "opcode": opcode,
                "count": count,
                "type": "dispatch_loop" if count > 1000 else "inner_loop",
            })

    return loops


def detect_hotspots(trace: StructuredTrace, top_n: int = 10) -> List[Dict[str, Any]]:
    """Find the most frequently executed PCs (hotspots).

    Args:
        trace: StructuredTrace to analyze.
        top_n: Number of top hotspots to return (default 10).

    Returns:
        List of hotspots: [{pc, opcode, count, percentage}]
    """
    dispatches = trace.dispatches
    if not dispatches:
        return []

    total = len(dispatches)
    pc_counts: Dict[int, int] = {}
    pc_opcodes: Dict[int, str] = {}
    for d in dispatches:
        pc_counts[d.pc] = pc_counts.get(d.pc, 0) + 1
        pc_opcodes[d.pc] = d.target

    hotspots = []
    for pc, count in sorted(pc_counts.items(), key=lambda x: -x[1])[:top_n]:
        hotspots.append({
            "pc": pc,
            "opcode": pc_opcodes.get(pc, "?"),
            "count": count,
            "percentage": round(count / total * 100, 2),
        })

    return hotspots
