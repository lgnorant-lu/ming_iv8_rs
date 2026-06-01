"""
Pattern Matching module for iv8-rs.

Four-layer crypto/algorithm detection engine:
  Layer 1: Single constant matching (fast screening, medium FP rate)
  Layer 2: Sequence matching (high confidence, near-zero FP)
  Layer 3: Structure/behavior pattern matching (fallback for no-constant algos)
  Layer 4: Cross-validation (combine layers, ambiguity annotation)

Detects known algorithmic patterns (crypto, hash, cipher) in VM trace data
by matching opcode sequences and value patterns against configurable libraries.
"""

from __future__ import annotations
from dataclasses import dataclass, field
from typing import List, Dict, Optional, Any, Tuple, Set
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


@dataclass
class SequenceMatch:
    """A detected byte/word sequence match in trace values (Layer 2)."""

    name: str
    """Sequence identifier (e.g. 'SHA256_K', 'AES_SBOX')."""

    algorithm: str
    """Associated algorithm."""

    description: str
    """Human-readable description."""

    match_offset: int
    """Offset within the known sequence where match starts."""

    match_length: int
    """Number of consecutive values matched."""

    total_length: int
    """Total length of the known sequence."""

    confidence: float
    """Match confidence (match_length / min_match normalized)."""

    pc_start: int
    """First PC where matched values appear."""

    pc_end: int
    """Last PC where matched values appear."""

    matched_values: List[int]
    """The actual values that matched."""

    trace_indices: List[int]
    """Indices in the trace entries list where matches were found."""


def _load_builtin_patterns() -> Dict[str, Any]:
    """Load the built-in crypto pattern library."""
    data_dir = Path(__file__).parent / "data"
    pattern_file = data_dir / "crypto_patterns.json"
    if pattern_file.exists():
        with open(pattern_file, "r", encoding="utf-8") as f:
            return json.load(f)
    return {}


def detect_patterns(
    trace: StructuredTrace,
    patterns: Optional[Dict[str, Any]] = None,
    opcode_map: Optional[Dict[int, str]] = None,
    window_size: int = 20,
    min_confidence: float = 0.6,
) -> List[PatternMatch]:
    """Layer 3: Detect algorithmic structure patterns via opcode semantics.

    IMPORTANT — Layer 3 requires an ``opcode_map``:

    A VM's dispatch opcodes are arbitrary per-VM numbers (e.g. opcode 5).
    The pattern library's ``behavior_pattern`` is a list of *semantic* tokens
    (e.g. "shl", "xor", "add"). To match them, the caller must supply a map
    from this VM's numeric opcodes to semantic tokens. This map comes from
    manual reverse-engineering of the VM's handler table (the standard
    workflow, e.g. "77 opcodes mapped" in published JS-VM reversing) or from
    differential opcode-semantic inference (Phase 2).

    Without ``opcode_map`` this function returns an EMPTY list — it does NOT
    guess, because comparing numeric opcodes against string tokens is
    meaningless. This is a deliberate, honest contract (see H01 spec).

    Args:
        trace: StructuredTrace to analyze.
        patterns: Custom pattern dict (same format as crypto_patterns.json).
                  If None, uses built-in library.
        opcode_map: REQUIRED for any matching. Maps this VM's numeric opcodes
                    to semantic tokens, e.g. {5: "xor", 7: "add", 12: "shl"}.
                    Tokens should match those used in behavior_pattern
                    (shl/shr/rot/xor/and/or/not/add/sub/mul/load/store/...).
                    If None, returns [].
        window_size: Sliding window size for scanning (default 20).
        min_confidence: Minimum confidence threshold (default 0.6).

    Returns:
        List of PatternMatch objects, sorted by confidence (descending).
        Empty list if opcode_map is None.

    Example::

        # After reversing the VM's handler table:
        opmap = {5: "xor", 7: "add", 12: "shl", 13: "shr"}
        matches = detect_patterns(trace, opcode_map=opmap)
        for m in matches:
            print(f"{m.name} at PC {m.pc_start}-{m.pc_end} (conf={m.confidence:.2f})")
    """
    if patterns is None:
        patterns = _load_builtin_patterns()

    # Layer 3 cannot operate without a VM opcode -> semantic token map.
    # Returning [] here is the honest contract: we do not fabricate matches
    # by comparing numeric opcodes against semantic string tokens.
    if not opcode_map:
        return []

    dispatches = trace.dispatches
    if not dispatches:
        return []

    # Translate this VM's numeric opcodes into semantic tokens via the map.
    # Unmapped opcodes become None (a token that matches nothing).
    sem_tokens: List[Optional[str]] = []
    pcs: List[int] = []
    for d in dispatches:
        try:
            opc = int(d.target)
        except (ValueError, TypeError):
            sem_tokens.append(None)
            pcs.append(d.pc)
            continue
        sem_tokens.append(opcode_map.get(opc))
        pcs.append(d.pc)

    def token_similarity(window: List[Optional[str]], pat: List[str]) -> float:
        """Fraction of positions where the translated token equals the pattern token."""
        n = min(len(window), len(pat))
        if n == 0:
            return 0.0
        hits = sum(1 for i in range(n) if window[i] is not None and window[i] == pat[i])
        return hits / max(len(window), len(pat))

    matches: List[PatternMatch] = []

    for pattern_name, pattern_def in patterns.items():
        if pattern_name.startswith("_"):
            continue
        pat_seq = pattern_def.get("behavior_pattern") or pattern_def.get("opcode_sequence", [])
        # Only string-token behavior patterns are matchable via opcode_map.
        if not pat_seq or not all(isinstance(t, str) for t in pat_seq):
            continue
        pat_min_conf = pattern_def.get("min_confidence", min_confidence)
        description = pattern_def.get("description", "")
        plen = len(pat_seq)

        for i in range(len(sem_tokens) - plen + 1):
            window = sem_tokens[i:i + plen]
            conf = token_similarity(window, pat_seq)
            if conf >= max(pat_min_conf, min_confidence):
                matches.append(PatternMatch(
                    name=pattern_name,
                    description=description,
                    pc_start=pcs[i],
                    pc_end=pcs[min(i + plen - 1, len(pcs) - 1)],
                    confidence=round(conf, 3),
                    matched_opcodes=[t for t in window if t is not None][:plen],
                    window_index=i,
                ))

    # Deduplicate overlapping matches (keep highest confidence per name+region)
    matches.sort(key=lambda m: m.confidence, reverse=True)
    deduplicated = []
    seen = set()
    for m in matches:
        plen = len(patterns.get(m.name, {}).get("behavior_pattern", [1])) or 1
        key = (m.name, m.window_index // plen)
        if key not in seen:
            deduplicated.append(m)
            seen.add(key)

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


# --- Constant-based detection (most reliable for custom VMs) ---

_CONSTANTS_CACHE: Optional[Dict[int, Dict[str, str]]] = None


def _load_constants_db() -> Dict[int, Dict[str, str]]:
    """Load the crypto constants database. Returns {int_value: {name, algorithm, description}}.

    When multiple constants share the same integer value (e.g. 0x9E3779B9 is used by
    XTEA, TEA, Serpent, RC5, SEED, xxHash), the algorithm field is merged to list all.
    """
    global _CONSTANTS_CACHE
    if _CONSTANTS_CACHE is not None:
        return _CONSTANTS_CACHE

    data_dir = Path(__file__).parent / "data"
    const_file = data_dir / "crypto_constants.json"
    if not const_file.exists():
        _CONSTANTS_CACHE = {}
        return _CONSTANTS_CACHE

    with open(const_file, "r", encoding="utf-8") as f:
        raw = json.load(f)

    db: Dict[int, Dict[str, str]] = {}
    for name, entry in raw.items():
        if name.startswith("_"):
            continue
        int_val = entry.get("int")
        if int_val is None:
            continue
        if int_val in db:
            # Merge: append algorithm and name info
            existing = db[int_val]
            existing_algos = set(existing["algorithm"].split("/"))
            new_algos = set(entry.get("algorithm", "").split("/"))
            merged_algos = "/".join(sorted(existing_algos | new_algos - {""}))
            existing["algorithm"] = merged_algos
            existing["name"] += f"/{name}"
            existing["description"] += f" | {entry.get('description', '')}"
        else:
            db[int_val] = {
                "name": name,
                "algorithm": entry.get("algorithm", ""),
                "description": entry.get("description", ""),
                "hex": entry.get("value", ""),
            }
    _CONSTANTS_CACHE = db
    return db


@dataclass
class ConstantMatch:
    """A detected cryptographic constant in the trace."""

    name: str
    """Constant identifier (e.g. 'XTEA_DELTA')."""

    algorithm: str
    """Associated algorithm (e.g. 'XTEA/TEA')."""

    description: str
    """Human-readable description."""

    value: int
    """The integer value found."""

    hex_str: str
    """Hex representation."""

    pc: int
    """PC where the constant was found."""

    entry_type: str
    """Trace entry type where found (D/R/C/W)."""

    context: str
    """The full trace entry target/value for context."""


def detect_constants(
    trace: StructuredTrace,
    constants_db: Optional[Dict[int, Dict[str, str]]] = None,
    min_value: int = 0x10000,
) -> List[ConstantMatch]:
    """Detect known cryptographic constants in trace values.

    This is the MOST RELIABLE detection method for custom VMs because
    magic constants (like 0x9E3779B9 for XTEA) cannot be hidden by
    opcode remapping — they must appear as literal values.

    Searches all trace entries' value fields for known constants.

    Args:
        trace: StructuredTrace to analyze.
        constants_db: Custom constants dict {int_value: {name, algorithm, ...}}.
                      If None, uses built-in database.
        min_value: Minimum constant value to match (default 0x10000 = 65536).
                   Values below this (single bytes, small magic numbers like
                   Keccak RC[0]=1) collide with normal program data and cause
                   false positives, so they are filtered out. Set to 0 to
                   match all values (NOT recommended for noisy traces).

    Returns:
        List of ConstantMatch objects, sorted by PC.

    Example::

        from iv8_rs.trace import parse_trace
        from iv8_rs.patterns import detect_constants

        trace = parse_trace(ctx.get_unified_trace())
        constants = detect_constants(trace)
        for c in constants:
            print(f"{c.algorithm}: {c.name} ({c.hex_str}) at PC={c.pc}")
    """
    if constants_db is None:
        constants_db = _load_constants_db()

    if not constants_db:
        return []

    matches: List[ConstantMatch] = []

    for entry in trace.entries:
        # Try to extract integer values from the entry
        values_to_check: List[int] = []

        # Check the value field. For D entries with stack capture, the value
        # field may contain comma-separated values: "stack_depth,val1,val2,..."
        val_str = entry.value.strip()
        if val_str:
            # Split on commas to handle multi-value D entries (stack capture)
            val_parts = val_str.split(",") if "," in val_str else [val_str]
            for part in val_parts:
                part = part.strip()
                if not part:
                    continue
                try:
                    if part.startswith("0x") or part.startswith("0X"):
                        values_to_check.append(int(part, 16))
                    elif part.lstrip("-").isdigit():
                        v = int(part)
                        if v >= 0:
                            values_to_check.append(v)
                        if v < 0:
                            values_to_check.append(v & 0xFFFFFFFF)
                except (ValueError, OverflowError):
                    pass

        # For dispatch entries, check the target (opcode) as potential constant
        if entry.type == "D":
            try:
                opc = int(entry.target)
                # Only check large values (small opcodes are not constants)
                if opc > 65535:
                    values_to_check.append(opc)
            except (ValueError, TypeError):
                pass

        # Match against database
        for v in values_to_check:
            # Filter out small values that collide with normal program data
            if v < min_value:
                continue
            if v in constants_db:
                info = constants_db[v]
                matches.append(ConstantMatch(
                    name=info["name"],
                    algorithm=info["algorithm"],
                    description=info["description"],
                    value=v,
                    hex_str=info.get("hex", hex(v)),
                    pc=entry.pc,
                    entry_type=entry.type,
                    context=f"{entry.target}={entry.value}" if entry.type != "D" else f"opcode={entry.target}",
                ))

    # Sort by PC, deduplicate same constant at same PC
    seen = set()
    deduplicated = []
    for m in sorted(matches, key=lambda x: x.pc):
        key = (m.name, m.pc)
        if key not in seen:
            deduplicated.append(m)
            seen.add(key)

    return deduplicated


# --- Layer 2: Sequence Matching (highest confidence) ---

_SEQUENCES_CACHE: Optional[Dict[str, Any]] = None


def _load_sequences_db() -> Dict[str, Any]:
    """Load the crypto sequences database for sliding window matching."""
    global _SEQUENCES_CACHE
    if _SEQUENCES_CACHE is not None:
        return _SEQUENCES_CACHE

    data_dir = Path(__file__).parent / "data"
    seq_file = data_dir / "crypto_sequences.json"
    if not seq_file.exists():
        _SEQUENCES_CACHE = {}
        return _SEQUENCES_CACHE

    with open(seq_file, "r", encoding="utf-8") as f:
        raw = json.load(f)

    db = {k: v for k, v in raw.items() if not k.startswith("_")}
    _SEQUENCES_CACHE = db
    return db


def _extract_trace_values(trace: StructuredTrace) -> Tuple[List[int], List[int], List[int]]:
    """Extract integer values from trace entries for sequence matching.

    Returns:
        (values, pcs, indices) - parallel lists of extracted int values,
        their PCs, and their indices in trace.entries.
    """
    values: List[int] = []
    pcs: List[int] = []
    indices: List[int] = []

    for idx, entry in enumerate(trace.entries):
        val_str = entry.value.strip()
        if not val_str:
            continue
        try:
            if val_str.startswith("0x") or val_str.startswith("0X"):
                v = int(val_str, 16)
            elif val_str.lstrip("-").isdigit():
                v = int(val_str)
                if v < 0:
                    v = v & 0xFFFFFFFF  # unsigned interpretation
            else:
                continue
            values.append(v)
            pcs.append(entry.pc)
            indices.append(idx)
        except (ValueError, OverflowError):
            continue

    return values, pcs, indices


def detect_sequences(
    trace: StructuredTrace,
    sequences_db: Optional[Dict[str, Any]] = None,
    min_match_length: int = 4,
    fuzzy: bool = False,
    fuzzy_tolerance: int = 0,
    max_gap: int = 0,
) -> List[SequenceMatch]:
    """Layer 2: Detect known crypto table subsequences in trace values.

    Searches for consecutive values in the trace that match subsequences
    of known cryptographic tables (S-boxes, K-tables, P-arrays, etc.).

    This is the HIGHEST CONFIDENCE detection method because matching
    8+ consecutive values from a known table is near-impossible by chance.

    Args:
        trace: StructuredTrace to analyze.
        sequences_db: Custom sequences dict. If None, uses built-in.
        min_match_length: Minimum consecutive matches to report (default 4).
        fuzzy: Enable fuzzy matching (allow +/-tolerance on values).
        fuzzy_tolerance: Value tolerance for fuzzy matching (default 0 = exact).
        max_gap: Allow up to N non-matching values between matches (default 0).

    Returns:
        List of SequenceMatch objects, sorted by confidence (descending).

    Example::

        from iv8_rs.trace import parse_trace
        from iv8_rs.patterns import detect_sequences

        trace = parse_trace(ctx.get_unified_trace())
        seqs = detect_sequences(trace)
        for s in seqs:
            print(f"{s.algorithm}: {s.name} ({s.match_length}/{s.total_length} values) conf={s.confidence:.2f}")
    """
    if sequences_db is None:
        sequences_db = _load_sequences_db()

    if not sequences_db:
        return []

    trace_values, trace_pcs, trace_indices = _extract_trace_values(trace)
    if not trace_values:
        return []

    matches: List[SequenceMatch] = []

    for seq_name, seq_def in sequences_db.items():
        known_seq = seq_def.get("values", [])
        if not known_seq:
            continue

        seq_min_match = max(min_match_length, seq_def.get("min_match", min_match_length))
        algorithm = seq_def.get("algorithm", "")
        description = seq_def.get("description", "")
        total_len = len(known_seq)

        # Build a set for O(1) lookup of known values
        known_set = set(known_seq)
        if fuzzy and fuzzy_tolerance > 0:
            # Expand set with tolerance range
            expanded_set: Set[int] = set()
            for kv in known_seq:
                for delta in range(-fuzzy_tolerance, fuzzy_tolerance + 1):
                    expanded_set.add(kv + delta)
            known_set = expanded_set

        # Sliding window: find runs of trace values that appear in the known sequence
        # in the correct ORDER (subsequence matching)
        i = 0
        while i < len(trace_values):
            v = trace_values[i]
            if v not in known_set:
                i += 1
                continue

            # Find where this value appears in the known sequence
            start_positions = [
                j for j, kv in enumerate(known_seq)
                if (kv == v if not fuzzy else abs(kv - v) <= fuzzy_tolerance)
            ]

            best_run_len = 0
            best_run_offset = 0
            best_run_values: List[int] = []
            best_run_trace_indices: List[int] = []

            for start_pos in start_positions:
                # Try to extend the match from this position
                run_len = 0
                run_values: List[int] = []
                run_trace_idx: List[int] = []
                gaps_used = 0
                ti = i
                si = start_pos

                while ti < len(trace_values) and si < total_len:
                    tv = trace_values[ti]
                    sv = known_seq[si]
                    if (tv == sv if not fuzzy else abs(tv - sv) <= fuzzy_tolerance):
                        run_len += 1
                        run_values.append(tv)
                        run_trace_idx.append(trace_indices[ti])
                        ti += 1
                        si += 1
                        gaps_used = 0
                    elif max_gap > 0 and gaps_used < max_gap:
                        # Allow gap in trace (skip one trace value)
                        ti += 1
                        gaps_used += 1
                    else:
                        break

                if run_len > best_run_len:
                    best_run_len = run_len
                    best_run_offset = start_pos
                    best_run_values = run_values
                    best_run_trace_indices = run_trace_idx

            if best_run_len >= seq_min_match:
                # Confidence: ratio of matched length to minimum required
                conf = min(1.0, best_run_len / max(seq_min_match * 2, 8))
                # Boost confidence for longer matches
                if best_run_len >= 16:
                    conf = min(1.0, conf + 0.2)
                elif best_run_len >= 8:
                    conf = min(1.0, conf + 0.1)

                first_idx = best_run_trace_indices[0]
                last_idx = best_run_trace_indices[-1]

                matches.append(SequenceMatch(
                    name=seq_name,
                    algorithm=algorithm,
                    description=description,
                    match_offset=best_run_offset,
                    match_length=best_run_len,
                    total_length=total_len,
                    confidence=round(conf, 3),
                    pc_start=trace.entries[first_idx].pc if first_idx < len(trace.entries) else -1,
                    pc_end=trace.entries[last_idx].pc if last_idx < len(trace.entries) else -1,
                    matched_values=best_run_values[:20],  # Cap for display
                    trace_indices=best_run_trace_indices[:20],
                ))
                # Skip past this match to avoid overlapping reports
                i += best_run_len
            else:
                i += 1

    # Sort by confidence descending, then by match_length descending
    matches.sort(key=lambda m: (-m.confidence, -m.match_length))
    return matches


# --- Layer 4: Cross-validation + Comprehensive Analysis ---

@dataclass
class CryptoDetection:
    """Comprehensive crypto detection result combining all layers."""

    algorithm: str
    """Detected algorithm name."""

    confidence: float
    """Overall confidence (0.0 - 1.0), boosted by cross-validation."""

    layers_matched: List[str]
    """Which layers contributed: ['constant', 'sequence', 'pattern']."""

    constants_found: List[ConstantMatch]
    """Layer 1 matches for this algorithm."""

    sequences_found: List[SequenceMatch]
    """Layer 2 matches for this algorithm."""

    patterns_found: List[PatternMatch]
    """Layer 3 matches for this algorithm."""

    pc_range: Tuple[int, int]
    """Estimated PC range where this algorithm operates."""

    ambiguity: List[str]
    """Other algorithms that share constants with this one (if any)."""

    notes: str
    """Additional context or warnings."""


def detect_all(
    trace: StructuredTrace,
    min_confidence: float = 0.5,
    enable_fuzzy: bool = False,
    context_window: int = 50,
    opcode_map: Optional[Dict[int, str]] = None,
) -> List[CryptoDetection]:
    """Layer 4: Cross-validated comprehensive crypto detection.

    Runs all three detection layers and combines results:
    - Constants found near structure patterns → boosted confidence
    - Multiple layers agreeing → higher confidence
    - Shared constants → ambiguity annotation

    Args:
        trace: StructuredTrace to analyze.
        min_confidence: Minimum confidence to include in results.
        enable_fuzzy: Enable fuzzy matching for sequences.
        context_window: PC range window for cross-validation proximity.
        opcode_map: Optional VM opcode -> semantic token map. Only when
                    provided does Layer 3 (structure/behavior) participate;
                    without it L1/L2/L4 still work on constants/sequences.

    Returns:
        List of CryptoDetection objects, sorted by confidence.

    Example::

        from iv8_rs.trace import parse_trace
        from iv8_rs.patterns import detect_all

        trace = parse_trace(ctx.get_unified_trace())
        detections = detect_all(trace)
        for d in detections:
            print(f"{d.algorithm} (conf={d.confidence:.2f}, layers={d.layers_matched})")
            if d.ambiguity:
                print(f"  [WARN] Shared constants with: {d.ambiguity}")
    """
    # Run all layers. Layer 3 only fires when an opcode_map is supplied.
    constants = detect_constants(trace)
    sequences = detect_sequences(trace, fuzzy=enable_fuzzy)
    patterns = detect_patterns(trace, opcode_map=opcode_map, min_confidence=min_confidence)

    # Group by algorithm
    algo_constants: Dict[str, List[ConstantMatch]] = {}
    algo_sequences: Dict[str, List[SequenceMatch]] = {}
    algo_patterns: Dict[str, List[PatternMatch]] = {}

    for c in constants:
        # An algorithm field may contain multiple (e.g. "XTEA/TEA")
        for algo in c.algorithm.split("/"):
            algo = algo.strip()
            if algo:
                algo_constants.setdefault(algo, []).append(c)

    for s in sequences:
        for algo in s.algorithm.split("/"):
            algo = algo.strip()
            if algo:
                algo_sequences.setdefault(algo, []).append(s)

    for p in patterns:
        algo_patterns.setdefault(p.name, []).append(p)

    # Merge all detected algorithms
    all_algos = set(algo_constants.keys()) | set(algo_sequences.keys()) | set(algo_patterns.keys())

    # Build shared-constant map for ambiguity detection
    constant_to_algos: Dict[int, List[str]] = {}
    for c in constants:
        for algo in c.algorithm.split("/"):
            algo = algo.strip()
            if algo:
                constant_to_algos.setdefault(c.value, []).append(algo)

    results: List[CryptoDetection] = []

    for algo in all_algos:
        consts = algo_constants.get(algo, [])
        seqs = algo_sequences.get(algo, [])
        pats = algo_patterns.get(algo, [])

        layers: List[str] = []
        if consts:
            layers.append("constant")
        if seqs:
            layers.append("sequence")
        if pats:
            layers.append("pattern")

        if not layers:
            continue

        # Base confidence from best individual layer
        best_const_conf = max((0.6 for _ in consts), default=0.0) if consts else 0.0
        best_seq_conf = max((s.confidence for s in seqs), default=0.0)
        best_pat_conf = max((p.confidence for p in pats), default=0.0)
        base_conf = max(best_const_conf, best_seq_conf, best_pat_conf)

        # Cross-validation boost
        boost = 0.0
        if len(layers) >= 2:
            boost += 0.1  # Two layers agree
        if len(layers) >= 3:
            boost += 0.1  # All three layers agree

        # Proximity boost: constants near patterns
        if consts and pats:
            for c in consts:
                for p in pats:
                    if abs(c.pc - p.pc_start) <= context_window:
                        boost += 0.05
                        break
                if boost > 0.15:
                    break

        # Sequence match is very strong evidence
        if seqs:
            max_seq_len = max(s.match_length for s in seqs)
            if max_seq_len >= 16:
                boost += 0.1
            elif max_seq_len >= 8:
                boost += 0.05

        final_conf = min(1.0, base_conf + boost)

        # PC range
        all_pcs: List[int] = []
        for c in consts:
            all_pcs.append(c.pc)
        for s in seqs:
            all_pcs.extend([s.pc_start, s.pc_end])
        for p in pats:
            all_pcs.extend([p.pc_start, p.pc_end])
        all_pcs = [pc for pc in all_pcs if pc >= 0]
        pc_range = (min(all_pcs), max(all_pcs)) if all_pcs else (-1, -1)

        # Ambiguity: which other algorithms share our constants?
        ambiguity: List[str] = []
        for c in consts:
            shared = constant_to_algos.get(c.value, [])
            for other in shared:
                if other != algo and other not in ambiguity:
                    ambiguity.append(other)

        # Notes
        notes_parts: List[str] = []
        if ambiguity:
            notes_parts.append(f"Shared constants with {ambiguity} - verify with structure")
        if len(consts) >= 3:
            notes_parts.append(f"{len(consts)} constants found (high confidence)")
        if seqs and max(s.match_length for s in seqs) >= 8:
            notes_parts.append("Strong sequence match (near-certain)")

        if final_conf >= min_confidence:
            results.append(CryptoDetection(
                algorithm=algo,
                confidence=round(final_conf, 3),
                layers_matched=layers,
                constants_found=consts,
                sequences_found=seqs,
                patterns_found=pats,
                pc_range=pc_range,
                ambiguity=ambiguity,
                notes="; ".join(notes_parts) if notes_parts else "",
            ))

    results.sort(key=lambda r: -r.confidence)
    return results
