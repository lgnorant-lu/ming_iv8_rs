"""
Coverage analysis: which of the 51 patterns are actually detectable AND tested?
"""
import json
from pathlib import Path

ROOT = Path(__file__).parent.parent
DATA_DIR = ROOT / "python" / "iv8_rs" / "data"
TEST_FILE = ROOT / "tests" / "test_crypto_detection.py"

with open(DATA_DIR / "crypto_patterns.json", encoding="utf-8") as f:
    patterns = json.load(f)
with open(DATA_DIR / "crypto_constants.json", encoding="utf-8") as f:
    constants = json.load(f)
with open(DATA_DIR / "crypto_sequences.json", encoding="utf-8") as f:
    sequences = json.load(f)

# All algorithm names from patterns
pattern_names = [k for k in patterns if not k.startswith("_")]

# Build set of algorithms that have constants
const_algos = set()
for name, entry in constants.items():
    if name.startswith("_"):
        continue
    for algo in entry.get("algorithm", "").split("/"):
        const_algos.add(algo.strip())

# Build set of algorithms that have sequences
seq_algos = set()
for name, entry in sequences.items():
    if name.startswith("_"):
        continue
    for algo in entry.get("algorithm", "").split("/"):
        seq_algos.add(algo.strip())

# Read test file to see which algorithms are referenced
test_content = TEST_FILE.read_text(encoding="utf-8")

# Map pattern key -> canonical algorithm name(s) used in constants/sequences
# Pattern keys are like "XTEA", "SHA256", "MD5". Algorithm fields use "XTEA", "SHA-256", "MD5"
# Need a mapping
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

print("=" * 90)
print(f"{'Pattern':<16} {'HasConst':<9} {'HasSeq':<8} {'Tested':<8} {'Detectability'}")
print("=" * 90)

no_signature = []  # algorithms with NO constants AND NO sequences (only Layer 3)
not_tested = []

for pname in pattern_names:
    algo = PATTERN_TO_ALGO.get(pname, pname)
    # Check if algorithm has constants (partial match)
    has_const = any(algo in ca or ca in algo for ca in const_algos if ca)
    # Check if algorithm has sequences
    has_seq = any(algo in sa or sa in algo for sa in seq_algos if sa)
    # Check if mentioned in tests (search for the algo name or pattern name)
    tested = (algo.lower() in test_content.lower() or
              pname.lower() in test_content.lower())

    if has_const and has_seq:
        detect = "L1+L2 (strong)"
    elif has_const:
        detect = "L1 (constant)"
    elif has_seq:
        detect = "L2 (sequence)"
    else:
        detect = "L3 ONLY (weak)"
        no_signature.append(pname)

    if not tested:
        not_tested.append(pname)

    c = "YES" if has_const else "-"
    s = "YES" if has_seq else "-"
    t = "YES" if tested else "NO"
    print(f"{pname:<16} {c:<9} {s:<8} {t:<8} {detect}")

print("=" * 90)
print(f"\nTotal patterns: {len(pattern_names)}")
print(f"\nAlgorithms with NO constant AND NO sequence (Layer 3 only, hard to test): {len(no_signature)}")
for n in no_signature:
    print(f"  - {n}")
print(f"\nAlgorithms NOT referenced in tests: {len(not_tested)}")
for n in not_tested:
    print(f"  - {n}")
