"""
AUDIT: False positive testing.
Feed unrelated/random traces and check the detector does NOT hallucinate crypto.
This is the most important correctness test - if random data matches crypto,
the whole engine is untrustworthy.
"""
import random
from iv8_rs.trace import StructuredTrace, TraceEntry
from iv8_rs.patterns import detect_constants, detect_sequences, detect_all

random.seed(12345)  # deterministic

def make_trace(entries_data):
    entries = []
    for t, pc, target, value in entries_data:
        raw = f"{t},{pc},{target},{value}"
        entries.append(TraceEntry(type=t, pc=pc, target=target, value=str(value), raw=raw))
    return StructuredTrace(entries)

print("=" * 70)
print("FALSE POSITIVE AUDIT")
print("=" * 70)

# ============================================================
# Scenario 1: Pure dispatch loop (typical VM execution, no crypto)
# Small opcode numbers 0-65, repeated
# ============================================================
print("\n[1] Pure VM dispatch loop (opcodes 0-65, 10000 iterations)...")
entries = []
for i in range(10000):
    opcode = i % 66
    entries.append(("D", 1000 + (i % 50), str(opcode), str(i % 8)))
trace = make_trace(entries)
seqs = detect_sequences(trace, min_match_length=4)
consts = detect_constants(trace)
print(f"    Sequences detected: {len(seqs)}  {[s.algorithm for s in seqs[:5]]}")
print(f"    Constants detected: {len(consts)}  {[c.algorithm for c in consts[:5]]}")

# ============================================================
# Scenario 2: Random small integers (0-255) - simulates byte processing
# ============================================================
print("\n[2] Random bytes 0-255 (5000 reads, simulates string/byte ops)...")
entries = []
for i in range(5000):
    entries.append(("R", 2000 + i, "byte", str(random.randint(0, 255))))
trace = make_trace(entries)
seqs = detect_sequences(trace, min_match_length=8)
print(f"    Sequences detected (min_match=8): {len(seqs)}  {[(s.algorithm, s.match_length) for s in seqs[:8]]}")

# ============================================================
# Scenario 3: Sequential small integers 0-15 (array indices)
# This could collide with RIPEMD160_S/R_LEFT (values 0-15)
# ============================================================
print("\n[3] Sequential array indices 0-15 repeated (simulates loops)...")
entries = []
for i in range(2000):
    entries.append(("R", 3000 + i, "idx", str(i % 16)))
trace = make_trace(entries)
seqs = detect_sequences(trace, min_match_length=8)
print(f"    Sequences detected: {len(seqs)}  {[s.algorithm for s in seqs[:8]]}")

# ============================================================
# Scenario 4: Permutation-like data 0-63 (could collide DES tables)
# ============================================================
print("\n[4] Values 1-64 in various orders (could collide DES IP/FP)...")
entries = []
for trial in range(50):
    perm = list(range(1, 65))
    random.shuffle(perm)
    for i, v in enumerate(perm):
        entries.append(("R", 4000 + i, "perm", str(v)))
trace = make_trace(entries)
seqs = detect_sequences(trace, min_match_length=8)
print(f"    Sequences detected: {len(seqs)}  {[s.algorithm for s in seqs[:8]]}")

# ============================================================
# Scenario 5: Realistic non-crypto JS trace (DOM/string operations)
# Mix of property reads with typical values
# ============================================================
print("\n[5] Realistic non-crypto JS trace (DOM/timing/string values)...")
entries = []
realistic_values = [
    1920, 1080, 24, 8, 1, 0, 60, 100, 200, 404, 500,  # screen, http codes
    1234567890, 1717171717,  # timestamps
    16, 32, 64, 128, 256, 512, 1024,  # powers of 2
    255, 256, 65535, 65536,  # boundaries
]
for i in range(3000):
    v = random.choice(realistic_values)
    entries.append(("R", 5000 + (i % 100), "prop", str(v)))
trace = make_trace(entries)
seqs = detect_sequences(trace, min_match_length=4)
consts = detect_constants(trace)
dets = detect_all(trace, min_confidence=0.5)
print(f"    Sequences: {len(seqs)}  Constants: {len(consts)}  Detections(conf>=0.5): {len(dets)}")
print(f"    Detections: {[(d.algorithm, d.confidence) for d in dets[:8]]}")

# ============================================================
# Scenario 6: Empty / tiny traces
# ============================================================
print("\n[6] Edge cases (empty, single entry)...")
empty = make_trace([])
print(f"    Empty trace: seqs={len(detect_sequences(empty))}, consts={len(detect_constants(empty))}, all={len(detect_all(empty))}")
single = make_trace([("R", 1, "x", "42")])
print(f"    Single entry: seqs={len(detect_sequences(single))}, consts={len(detect_constants(single))}")

print("\n" + "=" * 70)
print("ANALYSIS: Any non-zero detection above with HIGH confidence is a FALSE POSITIVE.")
print("Small-integer sequences (DES_IP/RIPEMD S/R, Base64) are the main risk.")
print("=" * 70)
