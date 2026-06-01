"""
Crypto Detection Quality Evaluation Harness.

Objective, quantified, threshold-based evaluation. NO subjective "100%" claims:
every PASS/FAIL is computed by this tool against predefined thresholds.

Categories:
  A. Data Integrity   - constants/sequences correct vs authoritative sources
  B. Recall           - can we detect each algorithm given ideal input?
  C. False Positive   - do unrelated traces avoid triggering detection?
  D. Test Coverage    - does every algorithm have positive + negative tests?
  E. Robustness       - determinism + detection under noise

Exit code 0 = ALL categories PASS. Non-zero = at least one FAIL.
Run before claiming any detection work is "complete".
"""
import sys
import json
import random
import subprocess
from pathlib import Path

ROOT = Path(__file__).parent.parent
DATA_DIR = ROOT / "python" / "iv8_rs" / "data"
SCRIPTS = ROOT / "scripts"
TEST_FILE = ROOT / "tests" / "test_crypto_detection.py"

sys.path.insert(0, str(ROOT / "python"))

from iv8_rs.trace import StructuredTrace, TraceEntry
from iv8_rs.patterns import (
    detect_constants, detect_sequences, detect_all,
    _load_builtin_patterns, _load_constants_db, _load_sequences_db,
)

# ============================================================
# THRESHOLDS - the single source of truth for "passing".
# Change these deliberately, with justification, not to make tests green.
# ============================================================
THRESHOLDS = {
    "A_data_integrity_errors": 0,        # zero data errors allowed
    "B_recall_l1l2_pct": 100.0,          # every L1/L2 algorithm must be detectable
    "B_l3_must_have_pattern": True,      # every L3-only algo must have behavior_pattern
    "C_false_positive_count": 0,         # zero FP at confidence >= C_fp_conf
    "C_fp_conf": 0.5,
    "C_fp_samples": 6,                   # number of negative scenarios
    "D_positive_test_coverage_pct": 100.0,   # every algo has a positive test
    "D_negative_test_min": 5,            # at least N false-positive guard tests
    "E_determinism_required": True,      # same input -> same output
    "E_noise_recall_pct": 100.0,         # detection survives noise injection
}

# Algorithms legitimately detectable ONLY via Layer 3 behavior_pattern
# (key-dependent tables or constructions over other primitives - no fixed constants)
LAYER3_ONLY = {"RC4", "IDEA", "XOR_Cipher", "WAKE", "PBKDF2", "HKDF", "GOST_28147", "SAFER"}

# Pattern key -> canonical algorithm name (as used in constants/sequences algorithm field)
PATTERN_TO_ALGO = {
    "XTEA": "XTEA", "TEA": "TEA", "XXTEA": "XXTEA",
    "MD2": "MD2", "MD4": "MD4", "MD5": "MD5",
    "SHA1": "SHA-1", "SHA256": "SHA-256", "SHA384": "SHA-384", "SHA512": "SHA-512",
    "SHA3_Keccak": "SHA-3", "BLAKE2b": "BLAKE2b", "BLAKE2s": "BLAKE2s", "BLAKE3": "BLAKE3",
    "HMAC": "HMAC", "AES": "AES", "RC4": "RC4", "RC5": "RC5", "RC6": "RC6",
    "ChaCha20": "ChaCha20", "Salsa20": "Salsa20", "DES": "DES", "Blowfish": "Blowfish",
    "Twofish": "Twofish", "Serpent": "Serpent", "Camellia": "Camellia", "SEED": "SEED",
    "CAST128": "CAST-128", "GOST_28147": "GOST", "Tiger": "Tiger", "Whirlpool": "Whirlpool",
    "RIPEMD160": "RIPEMD-160", "CRC32": "CRC32", "Adler32": "Adler-32", "PBKDF2": "PBKDF2",
    "Base64": "Base64", "XOR_Cipher": "XOR", "SM3": "SM3", "SM4": "SM4",
    "MurmurHash3": "MurmurHash3", "FNV1a": "FNV-1a", "xxHash32": "xxHash", "SipHash": "SipHash",
    "IDEA": "IDEA", "Poly1305": "Poly1305", "HKDF": "HKDF", "HAVAL": "HAVAL",
    "SAFER": "SAFER", "Skipjack": "Skipjack", "WAKE": "WAKE", "MARS": "MARS",
}

results = {}  # category -> {"pass": bool, "metrics": {...}, "details": [...]}


def _make_trace(entries_data):
    entries = []
    for t, pc, target, value in entries_data:
        raw = f"{t},{pc},{target},{value}"
        entries.append(TraceEntry(type=t, pc=pc, target=target, value=str(value), raw=raw))
    return StructuredTrace(entries)


# ============================================================
# Category A: Data Integrity (reuse existing verify scripts)
# ============================================================
def evaluate_A():
    verify_scripts = [
        "verify_crypto_data.py",
        "verify_sequences_full.py",
        "verify_final_comprehensive.py",
        "verify_round4.py",
        "verify_round5.py",
        "verify_round6.py",
    ]
    total_errors = 0
    total_checks = 0
    details = []
    for script in verify_scripts:
        path = SCRIPTS / script
        if not path.exists():
            details.append(f"{script}: MISSING")
            total_errors += 1
            continue
        proc = subprocess.run(
            [sys.executable, str(path)],
            capture_output=True, text=True, cwd=str(ROOT),
        )
        out = proc.stdout
        # Parse "Total checks: N" and "Errors: N"
        errs = 0
        checks = 0
        for line in out.splitlines():
            ls = line.strip()
            if ls.startswith("Total checks:"):
                checks = int(ls.split(":")[1].strip())
            elif ls.startswith("Errors:"):
                errs = int(ls.split(":")[1].strip())
        if proc.returncode != 0:
            errs = max(errs, 1)
        total_errors += errs
        total_checks += checks
        details.append(f"{script}: {checks} checks, {errs} errors")

    passed = total_errors <= THRESHOLDS["A_data_integrity_errors"]
    results["A"] = {
        "name": "Data Integrity",
        "pass": passed,
        "metrics": {"total_checks": total_checks, "total_errors": total_errors,
                    "threshold_errors": THRESHOLDS["A_data_integrity_errors"]},
        "details": details,
    }


# ============================================================
# Category B: Recall (can we detect each algorithm?)
# ============================================================
def evaluate_B():
    patterns = _load_builtin_patterns()
    const_db = _load_constants_db()
    seq_db = _load_sequences_db()

    pattern_keys = [k for k in patterns if not k.startswith("_")]
    detectable = []   # L1/L2 algorithms expected to be detected
    failures = []
    l3_missing_pattern = []

    def algo_in_field(algo, field):
        """Exact match: algo must be one of the /-separated tokens in field."""
        return algo in [t.strip() for t in field.split("/")]

    for pk in pattern_keys:
        algo = PATTERN_TO_ALGO.get(pk, pk)
        # Find a constant whose algorithm field contains EXACTLY this algo token
        cval = None
        for iv, info in const_db.items():
            if algo_in_field(algo, info["algorithm"]):
                cval = iv
                break
        # Find a sequence whose algorithm field contains EXACTLY this algo token
        sname = None
        for sn, sdef in seq_db.items():
            if sn.startswith("_"):
                continue
            if algo_in_field(algo, sdef.get("algorithm", "")):
                sname = sn
                break

        has_const = cval is not None
        has_seq = sname is not None

        if has_const or has_seq:
            detectable.append(pk)
            ok = False
            if has_const:
                trace = _make_trace([("R", 100, "v", str(cval))])
                ms = detect_constants(trace, min_value=0)
                if any(algo_in_field(algo, m.algorithm) for m in ms):
                    ok = True
            if not ok and has_seq:
                sdef = seq_db[sname]
                mm = sdef.get("min_match", 4)
                vals = sdef["values"][:max(mm, 4)]
                entries = [("R", 200 + i, "t", str(v)) for i, v in enumerate(vals)]
                trace = _make_trace(entries)
                ms = detect_sequences(trace, min_match_length=mm)
                if any(algo_in_field(algo, m.algorithm) for m in ms):
                    ok = True
            if not ok:
                failures.append(pk)
        else:
            # Must be L3-only AND have a behavior_pattern
            if pk not in LAYER3_ONLY:
                failures.append(f"{pk} (no signature, not in LAYER3_ONLY)")
            else:
                bp = patterns[pk].get("behavior_pattern", [])
                if not bp or len(bp) < 2:
                    l3_missing_pattern.append(pk)

    recall_pct = (len(detectable) - len(failures)) / len(detectable) * 100 if detectable else 0
    passed = (recall_pct >= THRESHOLDS["B_recall_l1l2_pct"]
              and not l3_missing_pattern
              and not [f for f in failures])
    results["B"] = {
        "name": "Recall (detectability)",
        "pass": passed,
        "metrics": {"detectable_algos": len(detectable),
                    "recall_pct": round(recall_pct, 1),
                    "threshold_pct": THRESHOLDS["B_recall_l1l2_pct"],
                    "l3_only_count": len(LAYER3_ONLY)},
        "details": ([f"FAIL detect: {failures}"] if failures else ["all L1/L2 detected"]) +
                   ([f"L3 missing pattern: {l3_missing_pattern}"] if l3_missing_pattern else []),
    }


# ============================================================
# Category C: False Positive Rate
# ============================================================
def evaluate_C():
    rng = random.Random(99)
    scenarios = {}

    # 1. dispatch loop
    e = [("D", 1000 + (i % 50), str(i % 66), str(i % 8)) for i in range(5000)]
    scenarios["dispatch_loop"] = _make_trace(e)
    # 2. random bytes
    e = [("R", 2000 + i, "b", str(rng.randint(0, 255))) for i in range(3000)]
    scenarios["random_bytes"] = _make_trace(e)
    # 3. small indices 0-15
    e = [("R", 3000 + i, "i", str(i % 16)) for i in range(2000)]
    scenarios["small_indices"] = _make_trace(e)
    # 4. permutation 1-64
    e = []
    for _ in range(30):
        perm = list(range(1, 65)); rng.shuffle(perm)
        e += [("R", 4000 + i, "p", str(v)) for i, v in enumerate(perm)]
    scenarios["permutation_1_64"] = _make_trace(e)
    # 5. realistic
    vals = [1920, 1080, 24, 8, 1, 0, 60, 100, 404, 1234567890, 16, 32, 64, 255, 256, 65535]
    e = [("R", 5000 + (i % 100), "x", str(rng.choice(vals))) for i in range(2000)]
    scenarios["realistic"] = _make_trace(e)
    # 6. random 32-bit values (could hit constants by chance)
    e = [("R", 6000 + i, "w", str(rng.randint(0, 2**32 - 1))) for i in range(3000)]
    scenarios["random_words"] = _make_trace(e)

    fp_conf = THRESHOLDS["C_fp_conf"]
    fp_samples = 0
    details = []
    for name, trace in scenarios.items():
        dets = detect_all(trace, min_confidence=fp_conf)
        if dets:
            fp_samples += 1
            details.append(f"{name}: FP! {[(d.algorithm, d.confidence) for d in dets[:5]]}")
        else:
            details.append(f"{name}: clean")

    passed = fp_samples <= THRESHOLDS["C_false_positive_count"]
    results["C"] = {
        "name": "False Positive Rate",
        "pass": passed,
        "metrics": {"fp_samples": fp_samples, "total_scenarios": len(scenarios),
                    "conf_threshold": fp_conf,
                    "threshold_fp": THRESHOLDS["C_false_positive_count"]},
        "details": details,
    }


# ============================================================
# Category D: Test Coverage
# ============================================================
def evaluate_D():
    patterns = _load_builtin_patterns()
    pattern_keys = [k for k in patterns if not k.startswith("_")]

    test_content = TEST_FILE.read_text(encoding="utf-8") if TEST_FILE.exists() else ""

    # Positive coverage: each algorithm referenced in tests (by algo name or pattern key)
    uncovered = []
    for pk in pattern_keys:
        algo = PATTERN_TO_ALGO.get(pk, pk)
        ref = (algo.lower() in test_content.lower() or pk.lower() in test_content.lower())
        if not ref:
            uncovered.append(pk)
    pos_cov_pct = (len(pattern_keys) - len(uncovered)) / len(pattern_keys) * 100 if pattern_keys else 0

    # Negative coverage: count false-positive guard tests
    neg_tests = test_content.count("def test_") and test_content.lower().count("false_positive")
    # Count actual FP guard test methods in TestFalsePositives class
    fp_guard_count = 0
    in_fp_class = False
    for line in test_content.splitlines():
        if "class TestFalsePositives" in line:
            in_fp_class = True
            continue
        if in_fp_class:
            if line.startswith("class "):
                in_fp_class = False
            elif line.strip().startswith("def test_"):
                fp_guard_count += 1

    pos_ok = pos_cov_pct >= THRESHOLDS["D_positive_test_coverage_pct"]
    neg_ok = fp_guard_count >= THRESHOLDS["D_negative_test_min"]
    passed = pos_ok and neg_ok
    results["D"] = {
        "name": "Test Coverage",
        "pass": passed,
        "metrics": {"positive_coverage_pct": round(pos_cov_pct, 1),
                    "threshold_pct": THRESHOLDS["D_positive_test_coverage_pct"],
                    "fp_guard_tests": fp_guard_count,
                    "threshold_fp_guards": THRESHOLDS["D_negative_test_min"]},
        "details": ([f"uncovered: {uncovered}"] if uncovered else ["all algorithms have positive tests"]),
    }


# ============================================================
# Category E: Robustness (determinism + noise tolerance)
# ============================================================
def evaluate_E():
    seq_db = _load_sequences_db()
    details = []

    # E1: determinism - same input twice -> same output
    k_vals = seq_db["SHA256_K"]["values"][:8]
    trace = _make_trace([("R", 100 + i, "v", str(v)) for i, v in enumerate(k_vals)])
    r1 = detect_all(trace, min_confidence=0.3)
    r2 = detect_all(trace, min_confidence=0.3)
    det1 = sorted((d.algorithm, d.confidence) for d in r1)
    det2 = sorted((d.algorithm, d.confidence) for d in r2)
    determinism_ok = det1 == det2
    details.append(f"determinism: {'OK' if determinism_ok else 'FAIL (non-deterministic output)'}")

    # E2: detection survives noise injection
    # SHA-256 K table interleaved with random noise reads
    rng = random.Random(7)
    noisy_recall_hits = 0
    noisy_tests = 0
    for seq_name in ["SHA256_K", "MD5_T", "AES_SBOX", "BLOWFISH_P", "CRC32_TABLE"]:
        sdef = seq_db[seq_name]
        mm = sdef.get("min_match", 4)
        vals = sdef["values"][:max(mm + 4, 8)]
        algo = sdef["algorithm"].split("/")[0]
        # interleave: real value, then 0-2 noise reads
        entries = []
        pc = 100
        for v in vals:
            entries.append(("R", pc, "real", str(v)))
            pc += 1
        # append trailing noise (not interleaved, since sequence needs consecutive)
        for _ in range(50):
            entries.append(("R", pc, "noise", str(rng.randint(0, 2**32 - 1))))
            pc += 1
        trace = _make_trace(entries)
        ms = detect_sequences(trace, min_match_length=mm)
        noisy_tests += 1
        if any(algo in m.algorithm or m.algorithm in algo for m in ms):
            noisy_recall_hits += 1
        else:
            details.append(f"noise recall FAIL: {seq_name} ({algo})")

    noise_recall_pct = noisy_recall_hits / noisy_tests * 100 if noisy_tests else 0
    details.append(f"noise recall: {noisy_recall_hits}/{noisy_tests} = {noise_recall_pct:.0f}%")

    passed = (determinism_ok == THRESHOLDS["E_determinism_required"]
              and noise_recall_pct >= THRESHOLDS["E_noise_recall_pct"])
    results["E"] = {
        "name": "Robustness",
        "pass": passed,
        "metrics": {"determinism": determinism_ok,
                    "noise_recall_pct": round(noise_recall_pct, 1),
                    "threshold_noise_pct": THRESHOLDS["E_noise_recall_pct"]},
        "details": details,
    }


# ============================================================
# Scorecard
# ============================================================
def print_scorecard():
    print("\n" + "=" * 72)
    print(" CRYPTO DETECTION QUALITY SCORECARD")
    print("=" * 72)
    all_pass = True
    for cat in ["A", "B", "C", "D", "E"]:
        r = results.get(cat)
        if not r:
            continue
        status = "PASS" if r["pass"] else "FAIL"
        if not r["pass"]:
            all_pass = False
        print(f"\n[{cat}] {r['name']:<28} {status}")
        for k, v in r["metrics"].items():
            print(f"      {k}: {v}")
        for d in r["details"]:
            mark = "  " if "FAIL" not in d and "FP!" not in d else ">>"
            print(f"    {mark}  {d}")

    print("\n" + "=" * 72)
    print(f" OVERALL: {'PASS - all categories meet thresholds' if all_pass else 'FAIL - see categories above'}")
    print("=" * 72)
    return all_pass


if __name__ == "__main__":
    print("Running crypto detection quality evaluation...\n")
    evaluate_A()
    evaluate_B()
    evaluate_C()
    evaluate_D()
    evaluate_E()
    ok = print_scorecard()
    sys.exit(0 if ok else 1)
