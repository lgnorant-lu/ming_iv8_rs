"""
Adversarial usability audit of M27 crypto detection.
Goes BEYOND the green harness: tests REALISTIC (not idealized) conditions
to expose the gap between "harness PASS" and "actually usable in the field".
"""
import sys
import random
from pathlib import Path
sys.path.insert(0, str(Path(__file__).parent.parent / "python"))

from iv8_rs.trace import StructuredTrace, TraceEntry
from iv8_rs.patterns import (
    detect_constants, detect_sequences, detect_all, detect_patterns,
    _load_sequences_db, _load_builtin_patterns,
)

def mk(entries):
    return StructuredTrace([TraceEntry(type=t, pc=p, target=tg, value=str(v),
                                       raw=f"{t},{p},{tg},{v}") for t, p, tg, v in entries])

seq_db = _load_sequences_db()
rng = random.Random(2024)
print("=" * 72)
print(" M27 REAL-WORLD ADVERSARIAL AUDIT (beyond the green harness)")
print("=" * 72)

# ------------------------------------------------------------
# TEST 1: Constant encoding variations (real traces aren't clean hex)
# ------------------------------------------------------------
print("\n[1] Constant detection under encoding variations")
xtea = 0x9E3779B9  # 2654435769
variants = {
    "decimal":        str(xtea),
    "hex_lower":      hex(xtea),
    "hex_upper":      "0x" + format(xtea, "X"),
    "signed_neg":     str(xtea - 2**32),       # -1640531527 (32-bit two's complement)
    "with_spaces":    f"  {xtea}  ",
}
for name, val in variants.items():
    t = mk([("R", 1, "v", val)])
    ms = detect_constants(t, min_value=0)
    hit = any("XTEA" in m.algorithm for m in ms)
    print(f"    {name:<14} {val:<16} -> {'DETECTED' if hit else 'MISSED'}")

# ------------------------------------------------------------
# TEST 2: Sequence detection when table is INTERLEAVED with noise
# (real VMs don't read S-box entries back-to-back)
# ------------------------------------------------------------
print("\n[2] Sequence detection with INTERLEAVED noise (realistic VM access)")
sha256_k = seq_db["SHA256_K"]["values"][:16]
for gap in [0, 1, 2, 3]:
    entries = []
    pc = 100
    for v in sha256_k:
        entries.append(("R", pc, "k", v)); pc += 1
        for _ in range(gap):
            entries.append(("R", pc, "noise", rng.randint(0, 2**32 - 1))); pc += 1
    t = mk(entries)
    ms = detect_sequences(t, min_match_length=4, max_gap=gap)
    best = max((m.match_length for m in ms if "SHA-256" in m.algorithm), default=0)
    print(f"    gap={gap}: best SHA-256 run = {best}/16  {'OK' if best >= 4 else 'MISS'}")

# ------------------------------------------------------------
# TEST 3: Partial table (VM only touches part of S-box for given input)
# ------------------------------------------------------------
print("\n[3] Partial table coverage (only some S-box entries accessed)")
aes = seq_db["AES_SBOX"]["values"]
for frac, label in [(256, "full"), (64, "quarter"), (16, "16 entries"), (8, "8 entries")]:
    subset = aes[:frac]
    entries = [("R", 100 + i, "sb", v) for i, v in enumerate(subset)]
    t = mk(entries)
    ms = detect_sequences(t, min_match_length=12)
    hit = any("AES" in m.algorithm for m in ms)
    print(f"    {label:<12} ({frac} entries) -> {'DETECTED' if hit else 'MISSED (below min_match=12)'}")

# ------------------------------------------------------------
# TEST 4: Shuffled access (VM accesses S-box in input-dependent order)
# ------------------------------------------------------------
print("\n[4] Shuffled S-box access (input-dependent, NOT table order)")
shuffled = aes[:32]; rng.shuffle(shuffled)
t = mk([("R", 100 + i, "sb", v) for i, v in enumerate(shuffled)])
ms = detect_sequences(t, min_match_length=12)
hit = any("AES" in m.algorithm for m in ms)
print(f"    shuffled 32 AES bytes -> {'DETECTED' if hit else 'MISSED'} "
      f"(expected MISS: sequence matching needs table order)")

# ------------------------------------------------------------
# TEST 5: Layer 3 behavior_pattern - does detect_patterns EVER fire?
# This is the documented gap. Test honestly.
# ------------------------------------------------------------
print("\n[5] Layer 3 (detect_patterns) - the documented gap")
patterns = _load_builtin_patterns()
# Build a dispatch trace whose opcodes spell out XTEA behavior_pattern
# But opcodes are per-VM numbers; behavior_pattern is ["shl","xor","add",...]
# detect_patterns matches opcode_sequence/behavior_pattern as NUMERIC opcodes.
# Try feeding the literal behavior tokens mapped to fake opcodes:
xtea_bp = patterns["XTEA"].get("behavior_pattern", [])
print(f"    XTEA behavior_pattern = {xtea_bp}")
# detect_patterns expects numeric opcode_sequence; behavior_pattern is strings.
# Construct dispatch entries using a synthetic opcode mapping:
opmap = {op: i + 10 for i, op in enumerate(set(xtea_bp))}
disp = [("D", 100 + i, str(opmap[op]), "3") for i, op in enumerate(xtea_bp)]
t = mk(disp)
ms = detect_patterns(t)
print(f"    feeding mapped opcodes -> {len(ms)} pattern matches "
      f"{[m.name for m in ms[:3]]}")
print(f"    NOTE: detect_patterns compares numeric opcodes; behavior_pattern")
print(f"    is string tokens. Without a per-VM opcode->semantic map, L3 cannot fire.")

# ------------------------------------------------------------
# TEST 6: L3-only algorithms - can ANY of the 8 be detected at all?
# ------------------------------------------------------------
print("\n[6] L3-only algorithms detectability (RC4/IDEA/XOR/WAKE/PBKDF2/HKDF/GOST/SAFER)")
l3 = ["RC4", "IDEA", "XOR_Cipher", "WAKE", "PBKDF2", "HKDF", "GOST_28147", "SAFER"]
for algo in l3:
    bp = patterns[algo].get("behavior_pattern", [])
    consts = patterns[algo].get("constants", [])
    print(f"    {algo:<12} behavior_pattern={len(bp)} tokens, constants={consts if consts else 'none'}")
print(f"    -> These 8 have NO constants/sequences. detect_constants/sequences")
print(f"       return nothing. Only detect_patterns could help, but see TEST 5.")

print("\n" + "=" * 72)
print(" HONEST VERDICT")
print("=" * 72)
print("""
 L1 (constants)  : reliable IF value appears as plain decimal/hex.
                   Signed/split-64bit/string encodings may be MISSED.
 L2 (sequences)  : works for in-order, mostly-contiguous table access.
                   Fails on shuffled access; needs min_match consecutive.
 L3 (behavior)   : NOT functional. Needs per-VM opcode->semantic map we lack.
                   The 8 L3-only algorithms are effectively undetectable today.
 L4 (cross-val)  : reliable only as far as L1/L2 feed it.

 Bottom line: M27 is a usable CONSTANT/SEQUENCE scanner, NOT yet a general
 crypto identifier. Honest coverage = 43/51 algorithms have a working path
 (L1/L2); 8/51 (L3-only) are spec'd but non-functional.
""")
