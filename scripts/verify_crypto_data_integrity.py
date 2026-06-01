"""
Crypto Data Integrity Verification (Category A of H01 harness).

Consolidates what were 6 separate verify_*.py scripts into one, organized by
logical verification method rather than arbitrary "round" numbering:

  Section 1: hex/int consistency (every constant)
  Section 2: independent math computation (SHA-256 K, MD5 T, CRC32 table, SM4 CK)
  Section 3: authoritative reference tables (FIPS/RFC/GM-T spec values)
  Section 4: structural/adversarial properties (permutation, inverse, bit constraints)
  Section 5: external cross-validation (hashlib/zlib/XTEA test vectors)
  Section 6: cross-database consistency (sequence vs constant, metadata)
  Section 7: integrity guards (hex/int conflicts, non-negative values)

Authoritative sources: FIPS 180-4 / FIPS 197 / FIPS 46-3 / RFC 1319 / RFC 8439 /
GM/T 0002-2012 / GM/T 0004-2012 / Schneier Applied Cryptography.

Output ends with "Total checks: N" + "Errors: N" for harness parsing.
Exit code 0 = all pass, 1 = errors found.
"""
import sys
import json
import math
import struct
import hashlib
import zlib
from collections import Counter
from pathlib import Path

DATA_DIR = Path(__file__).parent.parent / "python" / "iv8_rs" / "data"

errors = []
checks = 0


def check(condition, msg):
    global checks
    checks += 1
    if not condition:
        errors.append(msg)
    return condition


def section(title):
    print(f"\n[{title}]")


# --- Load databases ---
with open(DATA_DIR / "crypto_constants.json", encoding="utf-8") as f:
    constants = json.load(f)
with open(DATA_DIR / "crypto_sequences.json", encoding="utf-8") as f:
    sequences = json.load(f)
with open(DATA_DIR / "crypto_patterns.json", encoding="utf-8") as f:
    patterns = json.load(f)


# --- Authoritative reference tables (single source) ---
AES_SBOX_REF = [
    0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76,
    0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0,
    0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15,
    0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75,
    0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84,
    0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf,
    0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8,
    0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2,
    0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73,
    0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb,
    0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79,
    0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08,
    0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a,
    0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e,
    0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf,
    0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16,
]
SM4_SBOX_REF = [
    0xD6,0x90,0xE9,0xFE,0xCC,0xE1,0x3D,0xB7,0x16,0xB6,0x14,0xC2,0x28,0xFB,0x2C,0x05,
    0x2B,0x67,0x9A,0x76,0x2A,0xBE,0x04,0xC3,0xAA,0x44,0x13,0x26,0x49,0x86,0x06,0x99,
    0x9C,0x42,0x50,0xF4,0x91,0xEF,0x98,0x7A,0x33,0x54,0x0B,0x43,0xED,0xCF,0xAC,0x62,
    0xE4,0xB3,0x1C,0xA9,0xC9,0x08,0xE8,0x95,0x80,0xDF,0x94,0xFA,0x75,0x8F,0x3F,0xA6,
    0x47,0x07,0xA7,0xFC,0xF3,0x73,0x17,0xBA,0x83,0x59,0x3C,0x19,0xE6,0x85,0x4F,0xA8,
    0x68,0x6B,0x81,0xB2,0x71,0x64,0xDA,0x8B,0xF8,0xEB,0x0F,0x4B,0x70,0x56,0x9D,0x35,
    0x1E,0x24,0x0E,0x5E,0x63,0x58,0xD1,0xA2,0x25,0x22,0x7C,0x3B,0x01,0x21,0x78,0x87,
    0xD4,0x00,0x46,0x57,0x9F,0xD3,0x27,0x52,0x4C,0x36,0x02,0xE7,0xA0,0xC4,0xC8,0x9E,
    0xEA,0xBF,0x8A,0xD2,0x40,0xC7,0x38,0xB5,0xA3,0xF7,0xF2,0xCE,0xF9,0x61,0x15,0xA1,
    0xE0,0xAE,0x5D,0xA4,0x9B,0x34,0x1A,0x55,0xAD,0x93,0x32,0x30,0xF5,0x8C,0xB1,0xE3,
    0x1D,0xF6,0xE2,0x2E,0x82,0x66,0xCA,0x60,0xC0,0x29,0x23,0xAB,0x0D,0x53,0x4E,0x6F,
    0xD5,0xDB,0x37,0x45,0xDE,0xFD,0x8E,0x2F,0x03,0xFF,0x6A,0x72,0x6D,0x6C,0x5B,0x51,
    0x8D,0x1B,0xAF,0x92,0xBB,0xDD,0xBC,0x7F,0x11,0xD9,0x5C,0x41,0x1F,0x10,0x5A,0xD8,
    0x0A,0xC1,0x31,0x88,0xA5,0xCD,0x7B,0xBD,0x2D,0x74,0xD0,0x12,0xB8,0xE5,0xB4,0xB0,
    0x89,0x69,0x97,0x4A,0x0C,0x96,0x77,0x7E,0x65,0xB9,0xF1,0x09,0xC5,0x6E,0xC6,0x84,
    0x18,0xF0,0x7D,0xEC,0x3A,0xDC,0x4D,0x20,0x79,0xEE,0x5F,0x3E,0xD7,0xCB,0x39,0x48,
]
BLOWFISH_P_REF = [
    0x243F6A88, 0x85A308D3, 0x13198A2E, 0x03707344, 0xA4093822, 0x299F31D0,
    0x082EFA98, 0xEC4E6C89, 0x452821E6, 0x38D01377, 0xBE5466CF, 0x34E90C6C,
    0xC0AC29B7, 0xC97C50DD, 0x3F84D5B5, 0xB5470917, 0x9216D5D9, 0x8979FB1B,
]
BF_SBOX0_REF = [
    0xD1310BA6, 0x98DFB5AC, 0x2FFD72DB, 0xD01ADFB7, 0xB8E1AFED, 0x6A267E96,
    0xBA7C9045, 0xF12C7F99, 0x24A19947, 0xB3916CF7, 0x0801F2E2, 0x858EFC16,
    0x636920D8, 0x71574E69, 0xA458FEA3, 0xF4933D7E, 0x0D95748F, 0x728EB658,
    0x718BCD58, 0x82154AEE, 0x7B54A41D, 0xC25A59B5, 0x9C30D539, 0x2AF26013,
    0xC5D1B023, 0x286085F0, 0xCA417918, 0xB8DB38EF, 0x8E79DCB0, 0x603A180E,
    0x6C9E0E8B, 0xB01E8A3E,
]
DES_IP_REF = [
    58,50,42,34,26,18,10,2, 60,52,44,36,28,20,12,4,
    62,54,46,38,30,22,14,6, 64,56,48,40,32,24,16,8,
    57,49,41,33,25,17, 9,1, 59,51,43,35,27,19,11,3,
    61,53,45,37,29,21,13,5, 63,55,47,39,31,23,15,7,
]
DES_E_REF = [
    32, 1, 2, 3, 4, 5, 4, 5, 6, 7, 8, 9, 8, 9,10,11,12,13,12,13,14,15,16,17,
    16,17,18,19,20,21,20,21,22,23,24,25,24,25,26,27,28,29,28,29,30,31,32, 1,
]
SHA256_IV_FIPS = [0x6A09E667, 0xBB67AE85, 0x3C6EF372, 0xA54FF53A,
                  0x510E527F, 0x9B05688C, 0x1F83D9AB, 0x5BE0CD19]
SM4_FK_STANDARD = [0xA3B1BAC6, 0x56AA3350, 0x677D9197, 0xB27022DC]
XXHASH_PRIMES_REF = [0x9E3779B1, 0x85EBCA77, 0xC2B2AE3D, 0x27D4EB2F, 0x165667B1]
MURMUR3_REF = [0xCC9E2D51, 0x1B873593, 0x85EBCA6B, 0xC2B2AE35]


# --- Helper computations ---
def first_n_primes(n):
    primes = []
    c = 2
    while len(primes) < n:
        if all(c % p != 0 for p in primes if p * p <= c):
            primes.append(c)
        c += 1
    return primes


def frac_part_32(x):
    return int((x - int(x)) * (2**32)) & 0xFFFFFFFF


def compute_crc32_table():
    table = []
    for i in range(256):
        crc = i
        for _ in range(8):
            crc = (crc >> 1) ^ 0xEDB88320 if crc & 1 else crc >> 1
        table.append(crc)
    return table


def compute_sm4_ck():
    ck = []
    for i in range(32):
        b = [((4 * i + j) * 7) % 256 for j in range(4)]
        ck.append((b[0] << 24) | (b[1] << 16) | (b[2] << 8) | b[3])
    return ck


def keccak_rc_lfsr():
    RC, R = [], 1
    for _ in range(24):
        rc = 0
        for j in range(7):
            if R & 1:
                rc |= (1 << ((1 << j) - 1))
            R = ((R << 1) ^ ((R >> 7) * 0x171)) & 0xFF
        RC.append(rc)
    return RC


def xtea_encrypt(v0, v1, key, rounds=32):
    delta, s = 0x9E3779B9, 0
    for _ in range(rounds):
        v0 = (v0 + (((v1 << 4 ^ v1 >> 5) + v1) ^ (s + key[s & 3]))) & 0xFFFFFFFF
        s = (s + delta) & 0xFFFFFFFF
        v1 = (v1 + (((v0 << 4 ^ v0 >> 5) + v0) ^ (s + key[(s >> 11) & 3]))) & 0xFFFFFFFF
    return v0, v1


# ============================================================
# SECTION 1: hex/int consistency (every constant)
# ============================================================
section("1. hex/int consistency (all constants)")
for name, entry in constants.items():
    if name.startswith("_"):
        continue
    hex_val, int_val = entry.get("value", ""), entry.get("int")
    if not hex_val or int_val is None:
        continue
    try:
        check(int(hex_val, 16) == int_val,
              f"  {name}: hex {hex_val} -> {int(hex_val, 16)}, int field {int_val}")
    except ValueError:
        check(False, f"  {name}: invalid hex '{hex_val}'")
print(f"  checked {len([k for k in constants if not k.startswith('_')])} constants")


# ============================================================
# SECTION 2: independent math computation
# ============================================================
section("2. independent math computation (SHA-256 K, MD5 T, CRC32, SM4 CK)")

# SHA-256 K = cube roots of first 64 primes
sha256_k_computed = [frac_part_32(p ** (1 / 3)) for p in first_n_primes(64)]
sha256_k = sequences["SHA256_K"]["values"]
for i, (c, s) in enumerate(zip(sha256_k_computed, sha256_k)):
    check(c == s, f"  SHA256_K[{i}]: computed {c:#x}, stored {s:#x}")

# MD5 T = floor(abs(sin(i)) * 2^32)
md5_t = sequences["MD5_T"]["values"]
for i in range(64):
    exp = int((2**32) * abs(math.sin(i + 1))) & 0xFFFFFFFF
    check(md5_t[i] == exp, f"  MD5_T[{i}]: computed {exp:#x}, stored {md5_t[i]:#x}")

# CRC32 table from polynomial
crc32_computed = compute_crc32_table()
crc32_table = sequences["CRC32_TABLE"]["values"]
for i, s in enumerate(crc32_table):
    check(crc32_computed[i] == s, f"  CRC32_TABLE[{i}]: computed {crc32_computed[i]:#x}, stored {s:#x}")

# SM4 CK from formula
sm4_ck_computed = compute_sm4_ck()
sm4_ck = sequences["SM4_CK"]["values"]
for i, s in enumerate(sm4_ck):
    check(sm4_ck_computed[i] == s, f"  SM4_CK[{i}]: computed {sm4_ck_computed[i]:#x}, stored {s:#x}")

# Keccak RC from LFSR
keccak_computed = keccak_rc_lfsr()
keccak_rc = sequences["KECCAK_RC"]["values"]
for i, s in enumerate(keccak_rc):
    check(keccak_computed[i] == s, f"  KECCAK_RC[{i}]: computed {keccak_computed[i]:#x}, stored {s:#x}")

print(f"  SHA-256 K(64) + MD5 T(64) + CRC32({len(crc32_table)}) + SM4 CK(32) + Keccak RC(24)")


# ============================================================
# SECTION 3: authoritative reference tables
# ============================================================
section("3. authoritative reference tables (FIPS/RFC/GM-T)")

# AES S-box (FIPS 197)
aes_sbox = sequences["AES_SBOX"]["values"]
check(len(aes_sbox) == 256, f"  AES_SBOX length {len(aes_sbox)}")
for i, (r, s) in enumerate(zip(AES_SBOX_REF, aes_sbox)):
    check(r == s, f"  AES_SBOX[{i}]: ref {r:#x}, stored {s:#x}")

# AES inverse S-box (derived from forward)
inv_ref = [0] * 256
for i in range(256):
    inv_ref[AES_SBOX_REF[i]] = i
inv_sbox = sequences["AES_INV_SBOX"]["values"]
check(len(inv_sbox) == 256, f"  AES_INV_SBOX length {len(inv_sbox)}")
for i in range(min(256, len(inv_sbox))):
    check(inv_ref[i] == inv_sbox[i], f"  AES_INV_SBOX[{i}]: ref {inv_ref[i]}, stored {inv_sbox[i]}")

# SM4 S-box (GM/T 0002-2012)
sm4_sbox = sequences["SM4_SBOX"]["values"]
check(len(sm4_sbox) == 256, f"  SM4_SBOX length {len(sm4_sbox)}")
for i, (r, s) in enumerate(zip(SM4_SBOX_REF, sm4_sbox)):
    check(r == s, f"  SM4_SBOX[{i}]: ref {r:#x}, stored {s:#x}")

# Blowfish P-array (Schneier)
bf_p = sequences["BLOWFISH_P"]["values"]
for i, (r, s) in enumerate(zip(BLOWFISH_P_REF, bf_p)):
    check(r == s, f"  BLOWFISH_P[{i}]: ref {r:#x}, stored {s:#x}")

# Blowfish S-box 0 first 32 (Schneier)
bf_sbox0 = sequences["BLOWFISH_SBOX0_FIRST64"]["values"]
for i in range(min(32, len(bf_sbox0))):
    check(BF_SBOX0_REF[i] == bf_sbox0[i], f"  BF_SBOX0[{i}]: ref {BF_SBOX0_REF[i]:#x}, stored {bf_sbox0[i]:#x}")

# DES IP (FIPS 46-3)
des_ip = sequences["DES_IP"]["values"]
for i, (r, s) in enumerate(zip(DES_IP_REF, des_ip)):
    check(r == s, f"  DES_IP[{i}]: ref {r}, stored {s}")

# DES FP = inverse of IP
des_fp_ref = [0] * 64
for i in range(64):
    des_fp_ref[DES_IP_REF[i] - 1] = i + 1
des_fp = sequences["DES_FP"]["values"]
for i in range(64):
    check(des_fp_ref[i] == des_fp[i], f"  DES_FP[{i}]: ref {des_fp_ref[i]}, stored {des_fp[i]}")

# DES E-bit (FIPS 46-3)
des_e = sequences["DES_E_BIT"]["values"]
for i in range(48):
    check(DES_E_REF[i] == des_e[i], f"  DES_E_BIT[{i}]: ref {DES_E_REF[i]}, stored {des_e[i]}")

# SHA-256 IV (FIPS 180-4)
sha256_iv = sequences["SHA256_IV"]["values"]
for i, (r, s) in enumerate(zip(SHA256_IV_FIPS, sha256_iv)):
    check(r == s, f"  SHA256_IV[{i}]: ref {r:#x}, stored {s:#x}")

# ChaCha20 sigma ('expand 32-byte k', RFC 8439)
sigma_str = b"expand 32-byte k"
sigma_ref = [struct.unpack("<I", sigma_str[i:i + 4])[0] for i in range(0, 16, 4)]
chacha = sequences["CHACHA_SIGMA"]["values"]
for i, (r, s) in enumerate(zip(sigma_ref, chacha)):
    check(r == s, f"  CHACHA_SIGMA[{i}]: ref {r:#x}, stored {s:#x}")

# SM4 FK (GM/T 0002-2012)
sm4_fk = sequences["SM4_FK"]["values"]
for i, (r, s) in enumerate(zip(SM4_FK_STANDARD, sm4_fk)):
    check(r == s, f"  SM4_FK[{i}]: ref {r:#x}, stored {s:#x}")

# xxHash32 primes + MurmurHash3 constants
for i, (r, s) in enumerate(zip(XXHASH_PRIMES_REF, sequences["XXHASH32_PRIMES"]["values"])):
    check(r == s, f"  XXHASH32_PRIMES[{i}]: ref {r:#x}, stored {s:#x}")
for i, (r, s) in enumerate(zip(MURMUR3_REF, sequences["MURMUR3_CONSTANTS"]["values"])):
    check(r == s, f"  MURMUR3[{i}]: ref {r:#x}, stored {s:#x}")

print("  AES(256+256) + SM4(256) + Blowfish(18+32) + DES(64+64+48) + SHA-256 IV + sigma + FK + xxHash + Murmur")


# ============================================================
# SECTION 4: structural / adversarial properties
# ============================================================
section("4. structural properties (permutation, inverse, bit constraints)")

# AES S-box is a permutation
check(sorted(aes_sbox) == list(range(256)), "  AES_SBOX not a permutation of 0..255")
check(aes_sbox[0] == 0x63 and aes_sbox[0xFF] == 0x16, "  AES_SBOX boundary values wrong")

# AES inverse S-box is a permutation AND true inverse
check(sorted(inv_sbox) == list(range(256)), "  AES_INV_SBOX not a permutation")
for i in range(256):
    check(inv_sbox[aes_sbox[i]] == i, f"  inv_sbox[sbox[{i}]] != {i}")
    check(aes_sbox[inv_sbox[i]] == i, f"  sbox[inv_sbox[{i}]] != {i}")

# SM4 S-box is a permutation
check(sorted(sm4_sbox) == list(range(256)), "  SM4_SBOX not a permutation")
check(sm4_sbox[0] == 0xD6 and sm4_sbox[255] == 0x48, "  SM4_SBOX boundary values wrong")

# DES IP and FP are mutual inverses
for i in range(64):
    check(des_fp[des_ip[i] - 1] == i + 1, f"  FP[IP[{i}]-1] != {i+1}")
    check(des_ip[des_fp[i] - 1] == i + 1, f"  IP[FP[{i}]-1] != {i+1}")

# DES E-bit: each input bit 1-32 appears 1 or 2 times, wrap boundaries
check(des_e[0] == 32 and des_e[47] == 1, "  DES_E_BIT wrap boundaries wrong")
e_counts = Counter(des_e)
for bit in range(1, 33):
    check(e_counts.get(bit, 0) in (1, 2), f"  DES_E_BIT bit {bit} appears {e_counts.get(bit,0)} times")

# Keccak RC: bits only at positions 2^j - 1
valid_bits = {0, 1, 3, 7, 15, 31, 63}
for idx, rc in enumerate(keccak_rc):
    for bit in range(64):
        if rc & (1 << bit):
            check(bit in valid_bits, f"  KECCAK_RC[{idx}] bit {bit} not in {valid_bits}")
check(keccak_rc[0] == 1 and keccak_rc[1] == 0x8082, "  KECCAK_RC[0..1] wrong")

# Blowfish P[0..1] = pi hex digits
check(bf_p[0] == 0x243F6A88 and bf_p[1] == 0x85A308D3, "  BLOWFISH_P[0..1] wrong")

# SHA-256 K boundaries + 32-bit range
check(sha256_k[0] == 0x428A2F98 and sha256_k[63] == 0xC67178F2, "  SHA256_K boundaries wrong")
for i, k in enumerate(sha256_k):
    check(0 <= k <= 0xFFFFFFFF, f"  SHA256_K[{i}] out of 32-bit range")

# XTEA delta arithmetic: delta = golden ratio, delta*32 = DELTA_NEG
delta = constants.get("XTEA_DELTA", {}).get("int")
delta_neg = constants.get("XTEA_DELTA_NEG", {}).get("int")
if delta and delta_neg:
    check(delta == int((math.sqrt(5) - 1) / 2 * (2**32)) & 0xFFFFFFFF, "  XTEA delta != golden ratio")
    check(delta_neg == (delta * 32) & 0xFFFFFFFF, "  XTEA_DELTA_NEG != delta*32")

# HMAC ipad/opad
hmac_ipad = constants.get("HMAC_IPAD", {}).get("int")
hmac_opad = constants.get("HMAC_OPAD", {}).get("int")
if hmac_ipad:
    check(hmac_ipad == 0x36363636, "  HMAC_IPAD wrong")
if hmac_opad:
    check(hmac_opad == 0x5C5C5C5C, "  HMAC_OPAD wrong")

# MD2 S-box permutation (if present)
if "MD2_SBOX" in sequences:
    md2 = sequences["MD2_SBOX"]["values"]
    check(len(md2) == 256 and sorted(md2) == list(range(256)), "  MD2_SBOX not a permutation")
    check(md2[0] == 0x29 and md2[1] == 0x2E and md2[255] == 0x14, "  MD2_SBOX boundary values wrong")

print("  permutations + inverses + DES mutual-inverse + Keccak bits + XTEA/HMAC arithmetic")


# ============================================================
# SECTION 5: external cross-validation (hashlib/zlib/XTEA)
# ============================================================
section("5. external cross-validation (hashlib/zlib/XTEA test vectors)")

# Hash empty-string test vectors (validates IV usage in real implementations)
check(hashlib.sha256(b"").hexdigest() ==
      "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855", "  SHA-256('') mismatch")
check(hashlib.md5(b"").hexdigest() == "d41d8cd98f00b204e9800998ecf8427e", "  MD5('') mismatch")
check(hashlib.sha1(b"").hexdigest() == "da39a3ee5e6b4b0d3255bfef95601890afd80709", "  SHA-1('') mismatch")
check(hashlib.sha512(b"").hexdigest() ==
      "cf83e1357eefb8bdf1542850d66d8007d620e4050b5715dc83f4a921d36ce9ce"
      "47d0d13c5d85f2b0ff8318d2877eec2f63b931bd47417a81a538327af927da3e", "  SHA-512('') mismatch")
check(hashlib.sha384(b"").hexdigest() ==
      "38b060a751ac96384cd9327eb1b1e36a21fdb71114be07434c0cc7bf63f6e1da"
      "274edebfe76f65fbd51ad2f14898b95b", "  SHA-384('') mismatch")

# CRC32 multiple test vectors
for data, exp in [(b"", 0), (b"123456789", 0xCBF43926), (b"a", 0xE8B7BE43),
                  (b"abc", 0x352441C2),
                  (b"The quick brown fox jumps over the lazy dog", 0x414FA339)]:
    check((zlib.crc32(data) & 0xFFFFFFFF) == exp, f"  CRC32({data[:16]!r}) mismatch")

# XTEA encryption test vector (validates delta usage)
v0, v1 = xtea_encrypt(0, 0, [0, 0, 0, 0])
check(v0 == 0xDEE9D4D8 and v1 == 0xF7131ED9, f"  XTEA test vector mismatch: {v0:#x},{v1:#x}")

# SHA-512 K (FIPS 180-4) if present
if "SHA512_K" in sequences:
    for i, v in enumerate([0x428A2F98D728AE22, 0x7137449123EF65CD, 0xB5C0FBCFEC4D3B2F]):
        check(sequences["SHA512_K"]["values"][i] == v, f"  SHA512_K[{i}] mismatch")

# SHA-512 IV constants (FIPS 180-4) if present
for k, v in [("SHA512_INIT_H0", 0x6A09E667F3BCC908), ("SHA512_INIT_H1", 0xBB67AE8584CAA73B),
             ("SHA512_INIT_H7", 0x5BE0CD19137E2179)]:
    if k in constants:
        check(constants[k]["int"] == v, f"  {k} mismatch")

# Base64 alphabet if present
if "BASE64_ALPHABET" in sequences:
    b64_ref = [ord(c) for c in
               "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"]
    check(sequences["BASE64_ALPHABET"]["values"] == b64_ref, "  Base64 alphabet mismatch")

print("  SHA-256/384/512/MD5/SHA-1 hashes + CRC32(5) + XTEA + SHA-512 K/IV + Base64")


# ============================================================
# SECTION 6: cross-database consistency + metadata
# ============================================================
section("6. cross-database consistency (sequence vs constants, metadata)")

const_count = len([k for k in constants if not k.startswith("_")])
seq_count = len([k for k in sequences if not k.startswith("_")])
pat_count = len([k for k in patterns if not k.startswith("_")])
check(constants["_meta"].get("total_constants") == const_count,
      f"  constants _meta.total_constants {constants['_meta'].get('total_constants')} != {const_count}")
check(patterns["_meta"].get("total_patterns") == pat_count,
      f"  patterns _meta.total_patterns {patterns['_meta'].get('total_patterns')} != {pat_count}")

# Sequence values must match corresponding constants
def cross(seq_name, key_fmt, n):
    seq = sequences[seq_name]["values"]
    for i in range(n):
        key = key_fmt(i)
        if key in constants:
            check(seq[i] == constants[key]["int"],
                  f"  {seq_name}[{i}] {seq[i]} != {key} {constants[key]['int']}")

cross("SHA256_K", lambda i: f"SHA256_K{i:02d}", 64)
cross("BLOWFISH_P", lambda i: f"BLOWFISH_P{i:02d}", 18)
cross("SHA256_IV", lambda i: f"SHA256_INIT_H{i}", 8)
cross("CHACHA_SIGMA", lambda i: f"CHACHA_CONST_{i}", 4)
cross("SM3_IV", lambda i: f"SM3_IV_{i}", 8)
cross("SM4_FK", lambda i: f"SM4_FK_{i}", 4)
cross("SHA1_K", lambda i: ["SHA1_K1", "SHA1_K2", "SHA1_K3", "SHA1_K4"][i], 4)

# MD5 T sequence first/last vs constants
md5_t_seq = sequences["MD5_T"]["values"]
if "MD5_T01" in constants:
    check(md5_t_seq[0] == constants["MD5_T01"]["int"], "  MD5_T[0] != MD5_T01")
if "MD5_T64" in constants:
    check(md5_t_seq[63] == constants["MD5_T64"]["int"], "  MD5_T[63] != MD5_T64")

print(f"  counts({const_count}/{seq_count}/{pat_count}) + 7 sequence-vs-constant cross-checks")


# ============================================================
# SECTION 7: integrity guards
# ============================================================
section("7. integrity guards (hex/int conflicts, value sanity)")

# No two constants share the same int with DIFFERENT hex
int_to_hex = {}
for name, entry in constants.items():
    if name.startswith("_"):
        continue
    iv, hx = entry.get("int"), entry.get("value", "")
    if iv is None:
        continue
    if iv in int_to_hex:
        check(int_to_hex[iv].lower() == hx.lower(),
              f"  CONFLICT: int {iv} has hex '{int_to_hex[iv]}' AND '{hx}' ({name})")
    else:
        int_to_hex[iv] = hx

# All sequence values are non-negative ints
for sname, sdef in sequences.items():
    if sname.startswith("_"):
        continue
    for i, v in enumerate(sdef.get("values", [])):
        check(isinstance(v, int) and v >= 0, f"  {sname}[{i}]={v} not a non-negative int")

print("  hex/int conflict-free + all sequence values non-negative ints")


# ============================================================
# Summary
# ============================================================
print("\n" + "=" * 60)
print("CRYPTO DATA INTEGRITY VERIFICATION")
print("=" * 60)
print(f"  Total checks: {checks}")
print(f"  Errors: {len(errors)}")
if errors:
    print("\n  ERRORS:")
    for e in errors[:50]:
        print(f"    {e}")
    if len(errors) > 50:
        print(f"    ... and {len(errors) - 50} more")
    print(f"\n  [FAIL] {len(errors)} errors found")
    sys.exit(1)
else:
    print("\n  [OK] All data integrity checks passed")
    sys.exit(0)
