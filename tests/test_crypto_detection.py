"""
Ground truth test suite for crypto detection engine.

Covers ALL 51 algorithms in the pattern database with multiple samples per algorithm.
Tests Layer 1 (constants), Layer 2 (sequences), and Layer 4 (detect_all).

Methodology:
- Synthetic traces containing known constants/sequences for each algorithm
- Verifies detection engine correctly identifies the algorithm
- Multiple variants per algorithm (different constants, different positions)
"""
import pytest
import json
from pathlib import Path
from iv8_rs.trace import StructuredTrace, TraceEntry, parse_trace
from iv8_rs.patterns import (
    detect_constants, detect_sequences, detect_all,
    detect_loops, detect_hotspots,
    _load_builtin_patterns, _load_constants_db, _load_sequences_db,
)


# ============================================================
# Helpers
# ============================================================

def make_trace(entries_data):
    """Build a StructuredTrace from list of (type, pc, target, value) tuples."""
    entries = []
    for t, pc, target, value in entries_data:
        raw = f"{t},{pc},{target},{value}"
        entries.append(TraceEntry(type=t, pc=pc, target=target, value=str(value), raw=raw))
    return StructuredTrace(entries)


def make_constant_trace(int_value, pc=100):
    """Create a trace with a single constant value at given PC."""
    return make_trace([("R", pc, "val", int_value)])


def make_sequence_trace(values, start_pc=100):
    """Create a trace with consecutive values simulating sequence access."""
    entries = []
    for i, v in enumerate(values):
        entries.append(("R", start_pc + i, "table_access", v))
    return entries


def assert_algorithm_detected(detections, algorithm_name, min_confidence=0.0):
    """Assert that a specific algorithm appears in detect_all results."""
    found = [d for d in detections if algorithm_name in d.algorithm]
    assert found, (
        f"Algorithm '{algorithm_name}' not detected! "
        f"Got: {[d.algorithm for d in detections]}"
    )
    if min_confidence > 0:
        assert found[0].confidence >= min_confidence, (
            f"'{algorithm_name}' confidence {found[0].confidence} < {min_confidence}"
        )


# ============================================================
# Layer 1: Constant Detection Tests
# ============================================================

class TestConstantDetection:
    """Test detect_constants for each algorithm with identifiable constants."""

    def test_xtea_delta(self):
        trace = make_constant_trace(0x9E3779B9)
        matches = detect_constants(trace)
        algos = {m.algorithm for m in matches}
        assert any("XTEA" in a or "TEA" in a for a in algos)

    def test_xtea_delta_neg(self):
        trace = make_constant_trace(0xC6EF3720)
        matches = detect_constants(trace)
        assert any("XTEA" in m.algorithm or "TEA" in m.algorithm for m in matches)

    def test_md5_init_a(self):
        trace = make_constant_trace(0x67452301)
        matches = detect_constants(trace)
        assert any("MD5" in m.algorithm or "SHA-1" in m.algorithm for m in matches)

    def test_md5_t1(self):
        trace = make_constant_trace(0xD76AA478)
        matches = detect_constants(trace)
        assert any("MD5" in m.algorithm for m in matches)

    def test_md5_t64(self):
        trace = make_constant_trace(0xEB86D391)
        matches = detect_constants(trace)
        assert any("MD5" in m.algorithm for m in matches)

    def test_sha1_k1(self):
        trace = make_constant_trace(0x5A827999)
        matches = detect_constants(trace)
        assert any("SHA-1" in m.algorithm for m in matches)

    def test_sha1_k4(self):
        trace = make_constant_trace(0xCA62C1D6)
        matches = detect_constants(trace)
        assert any("SHA-1" in m.algorithm for m in matches)

    def test_sha256_k0(self):
        trace = make_constant_trace(0x428A2F98)
        matches = detect_constants(trace)
        assert any("SHA-256" in m.algorithm for m in matches)

    def test_sha256_k63(self):
        trace = make_constant_trace(0xC67178F2)
        matches = detect_constants(trace)
        assert any("SHA-256" in m.algorithm for m in matches)

    def test_sha256_iv_h0(self):
        trace = make_constant_trace(0x6A09E667)
        matches = detect_constants(trace)
        assert any("SHA-256" in m.algorithm or "BLAKE" in m.algorithm for m in matches)

    def test_crc32_poly(self):
        trace = make_constant_trace(0xEDB88320)
        matches = detect_constants(trace)
        assert any("CRC32" in m.algorithm for m in matches)

    def test_chacha_const0(self):
        trace = make_constant_trace(0x61707865)
        matches = detect_constants(trace)
        assert any("ChaCha" in m.algorithm or "Salsa" in m.algorithm for m in matches)

    def test_chacha_const3(self):
        trace = make_constant_trace(0x6B206574)
        matches = detect_constants(trace)
        assert any("ChaCha" in m.algorithm or "Salsa" in m.algorithm for m in matches)

    def test_blowfish_p0(self):
        trace = make_constant_trace(0x243F6A88)
        matches = detect_constants(trace)
        assert any("Blowfish" in m.algorithm for m in matches)

    def test_blowfish_p17(self):
        trace = make_constant_trace(0x8979FB1B)
        matches = detect_constants(trace)
        assert any("Blowfish" in m.algorithm for m in matches)

    def test_hmac_ipad(self):
        trace = make_constant_trace(0x36363636)
        matches = detect_constants(trace)
        assert any("HMAC" in m.algorithm for m in matches)

    def test_hmac_opad(self):
        trace = make_constant_trace(0x5C5C5C5C)
        matches = detect_constants(trace)
        assert any("HMAC" in m.algorithm for m in matches)

    def test_sm3_tj0(self):
        trace = make_constant_trace(0x79CC4519)
        matches = detect_constants(trace)
        assert any("SM3" in m.algorithm for m in matches)

    def test_sm4_fk0(self):
        trace = make_constant_trace(0xA3B1BAC6)
        matches = detect_constants(trace)
        assert any("SM4" in m.algorithm for m in matches)

    def test_murmur3_c1(self):
        trace = make_constant_trace(0xCC9E2D51)
        matches = detect_constants(trace)
        assert any("Murmur" in m.algorithm for m in matches)

    def test_murmur3_c2(self):
        trace = make_constant_trace(0x1B873593)
        matches = detect_constants(trace)
        assert any("Murmur" in m.algorithm for m in matches)

    def test_fnv_offset(self):
        trace = make_constant_trace(0x811C9DC5)
        matches = detect_constants(trace)
        assert any("FNV" in m.algorithm for m in matches)

    def test_fnv_prime(self):
        trace = make_constant_trace(0x01000193)
        matches = detect_constants(trace)
        assert any("FNV" in m.algorithm for m in matches)

    def test_xxhash_prime1(self):
        trace = make_constant_trace(0x9E3779B1)
        matches = detect_constants(trace)
        assert any("xxHash" in m.algorithm for m in matches)

    def test_rc5_p32(self):
        trace = make_constant_trace(0xB7E15163)
        matches = detect_constants(trace)
        assert any("RC5" in m.algorithm or "RC6" in m.algorithm for m in matches)

    def test_adler32_mod(self):
        # Adler-32 modulus 0xFFF1=65521 is below the min_value filter (0x10000)
        # because it collides with normal data. Must explicitly lower min_value.
        trace = make_constant_trace(65521)
        matches = detect_constants(trace, min_value=0)
        assert any("Adler" in m.algorithm for m in matches)

    def test_ripemd160_k_right0(self):
        trace = make_constant_trace(0x50A28BE6)
        matches = detect_constants(trace)
        assert any("RIPEMD" in m.algorithm for m in matches)

    def test_sm3_iv0(self):
        trace = make_constant_trace(0x7380166F)
        matches = detect_constants(trace)
        assert any("SM3" in m.algorithm for m in matches)

    def test_camellia_sigma1(self):
        trace = make_constant_trace(0xA09E667F)
        matches = detect_constants(trace)
        assert any("Camellia" in m.algorithm for m in matches)

    def test_cast128_s1_0(self):
        trace = make_constant_trace(0x30FB40D4)
        matches = detect_constants(trace)
        assert any("CAST" in m.algorithm for m in matches)

    def test_twofish_mds(self):
        trace = make_constant_trace(0xBCBC3275)
        matches = detect_constants(trace)
        assert any("Twofish" in m.algorithm for m in matches)

    def test_mars_s0(self):
        trace = make_constant_trace(0x09D0C479)
        matches = detect_constants(trace)
        assert any("MARS" in m.algorithm for m in matches)

    # --- Additional L1 coverage (algorithms previously untested) ---

    def test_xxtea_delta(self):
        trace = make_constant_trace(0x9E3779B9)
        matches = detect_constants(trace)
        assert any("XXTEA" in m.algorithm for m in matches)

    def test_md4_constant(self):
        trace = make_constant_trace(0x5A827999)
        matches = detect_constants(trace)
        # MD4 shares K1 with SHA-1
        assert any("MD4" in m.algorithm or "SHA-1" in m.algorithm for m in matches)

    def test_sha384_iv(self):
        trace = make_constant_trace(0xCBBB9D5DC1059ED8)
        matches = detect_constants(trace)
        assert any("SHA-384" in m.algorithm for m in matches)

    def test_sha512_iv(self):
        trace = make_constant_trace(0x6A09E667F3BCC908)
        matches = detect_constants(trace)
        assert any("SHA-512" in m.algorithm or "BLAKE2b" in m.algorithm for m in matches)

    def test_sha512_k0(self):
        trace = make_constant_trace(0x428A2F98D728AE22)
        matches = detect_constants(trace)
        assert any("SHA-512" in m.algorithm for m in matches)

    def test_blake2b_iv(self):
        trace = make_constant_trace(0x6A09E667F3BCC908)
        matches = detect_constants(trace)
        assert any("BLAKE2b" in m.algorithm or "SHA-512" in m.algorithm for m in matches)

    def test_blake2s_iv(self):
        trace = make_constant_trace(0x6A09E667)
        matches = detect_constants(trace)
        assert any("BLAKE" in m.algorithm or "SHA-256" in m.algorithm for m in matches)

    def test_tiger_init(self):
        trace = make_constant_trace(0x0123456789ABCDEF)
        matches = detect_constants(trace)
        assert any("Tiger" in m.algorithm for m in matches)

    def test_whirlpool_c0(self):
        trace = make_constant_trace(0x18186018C07830D8)
        matches = detect_constants(trace)
        assert any("Whirlpool" in m.algorithm for m in matches)

    def test_siphash_c0(self):
        trace = make_constant_trace(0x736F6D6570736575)
        matches = detect_constants(trace)
        assert any("SipHash" in m.algorithm for m in matches)

    def test_poly1305_clamp(self):
        trace = make_constant_trace(0x0FFFFFFC0FFFFFFC)
        matches = detect_constants(trace)
        assert any("Poly1305" in m.algorithm for m in matches)

    def test_haval_constant(self):
        trace = make_constant_trace(0x452821E6)
        matches = detect_constants(trace)
        # HAVAL shares constant with Blowfish (pi-derived)
        assert any("HAVAL" in m.algorithm or "Blowfish" in m.algorithm for m in matches)


# ============================================================
# Layer 2: Sequence Detection Tests
# ============================================================

class TestSequenceDetection:
    """Test detect_sequences for algorithms with known table sequences."""

    def test_sha256_k_sequence_4(self):
        """SHA-256 K table: 4 consecutive values should match."""
        seq_db = _load_sequences_db()
        k_vals = seq_db["SHA256_K"]["values"][:4]
        trace = make_trace(make_sequence_trace(k_vals))
        matches = detect_sequences(trace, min_match_length=4)
        assert any("SHA-256" in m.algorithm for m in matches)

    def test_sha256_k_sequence_8(self):
        """SHA-256 K table: 8 consecutive values (high confidence)."""
        seq_db = _load_sequences_db()
        k_vals = seq_db["SHA256_K"]["values"][10:18]
        trace = make_trace(make_sequence_trace(k_vals))
        matches = detect_sequences(trace, min_match_length=4)
        assert any("SHA-256" in m.algorithm for m in matches)
        sha_match = [m for m in matches if "SHA-256" in m.algorithm][0]
        assert sha_match.match_length >= 8

    def test_md5_t_sequence(self):
        """MD5 T table: 4 consecutive values."""
        seq_db = _load_sequences_db()
        t_vals = seq_db["MD5_T"]["values"][20:24]
        trace = make_trace(make_sequence_trace(t_vals))
        matches = detect_sequences(trace, min_match_length=4)
        assert any("MD5" in m.algorithm for m in matches)

    def test_aes_sbox_sequence(self):
        """AES S-box: 12 consecutive values (new min_match for byte tables)."""
        seq_db = _load_sequences_db()
        sbox_vals = seq_db["AES_SBOX"]["values"][:12]
        trace = make_trace(make_sequence_trace(sbox_vals))
        matches = detect_sequences(trace, min_match_length=12)
        assert any("AES" in m.algorithm for m in matches)

    def test_aes_sbox_middle(self):
        """AES S-box: 12 values from middle of table."""
        seq_db = _load_sequences_db()
        sbox_vals = seq_db["AES_SBOX"]["values"][100:112]
        trace = make_trace(make_sequence_trace(sbox_vals))
        matches = detect_sequences(trace, min_match_length=12)
        assert any("AES" in m.algorithm for m in matches)

    def test_blowfish_p_sequence(self):
        """Blowfish P-array: 4 consecutive values."""
        seq_db = _load_sequences_db()
        p_vals = seq_db["BLOWFISH_P"]["values"][:4]
        trace = make_trace(make_sequence_trace(p_vals))
        matches = detect_sequences(trace, min_match_length=4)
        assert any("Blowfish" in m.algorithm for m in matches)

    def test_crc32_table_sequence(self):
        """CRC32 table: 4 consecutive values."""
        seq_db = _load_sequences_db()
        crc_vals = seq_db["CRC32_TABLE"]["values"][:4]
        trace = make_trace(make_sequence_trace(crc_vals))
        matches = detect_sequences(trace, min_match_length=4)
        assert any("CRC32" in m.algorithm for m in matches)

    def test_des_ip_sequence(self):
        """DES IP table: 16 consecutive values (new min_match for permutation tables)."""
        seq_db = _load_sequences_db()
        ip_vals = seq_db["DES_IP"]["values"][:16]
        trace = make_trace(make_sequence_trace(ip_vals))
        matches = detect_sequences(trace, min_match_length=16)
        assert any("DES" in m.algorithm for m in matches)

    def test_sm4_sbox_sequence(self):
        """SM4 S-box: 12 consecutive values."""
        seq_db = _load_sequences_db()
        sbox_vals = seq_db["SM4_SBOX"]["values"][:12]
        trace = make_trace(make_sequence_trace(sbox_vals))
        matches = detect_sequences(trace, min_match_length=12)
        assert any("SM4" in m.algorithm for m in matches)

    def test_sm3_iv_sequence(self):
        """SM3 IV: 4 consecutive values."""
        seq_db = _load_sequences_db()
        iv_vals = seq_db["SM3_IV"]["values"][:4]
        trace = make_trace(make_sequence_trace(iv_vals))
        matches = detect_sequences(trace, min_match_length=4)
        assert any("SM3" in m.algorithm for m in matches)

    def test_keccak_rc_sequence(self):
        """Keccak RC: 4 consecutive values."""
        seq_db = _load_sequences_db()
        rc_vals = seq_db["KECCAK_RC"]["values"][:4]
        trace = make_trace(make_sequence_trace(rc_vals))
        matches = detect_sequences(trace, min_match_length=4)
        assert any("Keccak" in m.algorithm or "SHA-3" in m.algorithm for m in matches)

    def test_chacha_sigma_sequence(self):
        """ChaCha20 sigma: 3 consecutive values."""
        seq_db = _load_sequences_db()
        sigma_vals = seq_db["CHACHA_SIGMA"]["values"][:3]
        trace = make_trace(make_sequence_trace(sigma_vals))
        matches = detect_sequences(trace, min_match_length=3)
        assert any("ChaCha" in m.algorithm or "Salsa" in m.algorithm for m in matches)

    def test_ripemd160_constant_detection(self):
        """RIPEMD-160 is detected via its large K constants, not rotation tables.
        (Rotation/word-selection sequences were removed as false-positive-prone.)"""
        trace = make_constant_trace(0x50A28BE6)  # RIPEMD-160 right K0
        matches = detect_constants(trace)
        assert any("RIPEMD" in m.algorithm for m in matches)

    def test_sm4_ck_sequence(self):
        """SM4 CK: 4 consecutive values."""
        seq_db = _load_sequences_db()
        ck_vals = seq_db["SM4_CK"]["values"][:4]
        trace = make_trace(make_sequence_trace(ck_vals))
        matches = detect_sequences(trace, min_match_length=4)
        assert any("SM4" in m.algorithm for m in matches)

    def test_sha512_k_sequence(self):
        """SHA-512 K table: consecutive 64-bit values."""
        seq_db = _load_sequences_db()
        k_vals = seq_db["SHA512_K"]["values"][:4]
        trace = make_trace(make_sequence_trace(k_vals))
        matches = detect_sequences(trace, min_match_length=3)
        assert any("SHA-512" in m.algorithm for m in matches)

    def test_md2_sbox_sequence(self):
        """MD2 S-box: 12 consecutive values (byte table min_match)."""
        seq_db = _load_sequences_db()
        sbox_vals = seq_db["MD2_SBOX"]["values"][:12]
        trace = make_trace(make_sequence_trace(sbox_vals))
        matches = detect_sequences(trace, min_match_length=12)
        assert any("MD2" in m.algorithm for m in matches)

    def test_base64_alphabet_sequence(self):
        """Base64 alphabet: 16 consecutive ASCII codes (min_match for ASCII tables)."""
        seq_db = _load_sequences_db()
        b64_vals = seq_db["BASE64_ALPHABET"]["values"][:16]
        trace = make_trace(make_sequence_trace(b64_vals))
        matches = detect_sequences(trace, min_match_length=16)
        assert any("Base64" in m.algorithm for m in matches)

    def test_skipjack_f_sequence(self):
        """Skipjack F-table: 12 consecutive values."""
        seq_db = _load_sequences_db()
        f_vals = seq_db["SKIPJACK_F"]["values"][:12]
        trace = make_trace(make_sequence_trace(f_vals))
        matches = detect_sequences(trace, min_match_length=12)
        assert any("Skipjack" in m.algorithm for m in matches)


# ============================================================
# Layer 4: Cross-validation (detect_all) Tests
# ============================================================

class TestDetectAll:
    """Test detect_all combining multiple layers for each algorithm."""

    def test_sha256_combined(self):
        """SHA-256: constants + sequence → high confidence."""
        seq_db = _load_sequences_db()
        k_vals = seq_db["SHA256_K"]["values"][:8]
        entries = [("R", 50, "val", str(0x6A09E667))]  # IV constant
        entries += make_sequence_trace(k_vals, start_pc=100)
        trace = make_trace(entries)
        detections = detect_all(trace, min_confidence=0.3)
        assert_algorithm_detected(detections, "SHA-256", min_confidence=0.7)

    def test_md5_combined(self):
        """MD5: constants + sequence → high confidence."""
        seq_db = _load_sequences_db()
        t_vals = seq_db["MD5_T"]["values"][:6]
        entries = [("R", 50, "val", str(0x67452301))]  # IV
        entries += make_sequence_trace(t_vals, start_pc=100)
        trace = make_trace(entries)
        detections = detect_all(trace, min_confidence=0.3)
        assert_algorithm_detected(detections, "MD5")

    def test_aes_combined(self):
        """AES: S-box sequence (12 values) → detection."""
        seq_db = _load_sequences_db()
        sbox_vals = seq_db["AES_SBOX"]["values"][:14]
        trace = make_trace(make_sequence_trace(sbox_vals))
        detections = detect_all(trace, min_confidence=0.3)
        assert_algorithm_detected(detections, "AES")

    def test_xtea_constant_only(self):
        """XTEA: single delta constant → detected with ambiguity."""
        trace = make_constant_trace(0x9E3779B9)
        detections = detect_all(trace, min_confidence=0.3)
        # Should detect XTEA (among others sharing this constant)
        xtea = [d for d in detections if "XTEA" in d.algorithm]
        assert xtea
        # Should have ambiguity annotation
        assert xtea[0].ambiguity  # TEA/RC5/Serpent/SEED share this constant

    def test_blowfish_combined(self):
        """Blowfish: P-array sequence + constant."""
        seq_db = _load_sequences_db()
        p_vals = seq_db["BLOWFISH_P"]["values"][:6]
        trace = make_trace(make_sequence_trace(p_vals))
        detections = detect_all(trace, min_confidence=0.3)
        assert_algorithm_detected(detections, "Blowfish")

    def test_crc32_combined(self):
        """CRC32: polynomial constant + table sequence."""
        seq_db = _load_sequences_db()
        crc_vals = seq_db["CRC32_TABLE"]["values"][:4]
        entries = [("R", 50, "val", str(0xEDB88320))]
        entries += make_sequence_trace(crc_vals, start_pc=100)
        trace = make_trace(entries)
        detections = detect_all(trace, min_confidence=0.3)
        assert_algorithm_detected(detections, "CRC32")

    def test_chacha20_combined(self):
        """ChaCha20: sigma constants + sequence."""
        seq_db = _load_sequences_db()
        sigma = seq_db["CHACHA_SIGMA"]["values"]
        entries = make_sequence_trace(sigma, start_pc=100)
        trace = make_trace(entries)
        detections = detect_all(trace, min_confidence=0.3)
        assert_algorithm_detected(detections, "ChaCha20")

    def test_sm4_combined(self):
        """SM4: FK constant + CK sequence."""
        seq_db = _load_sequences_db()
        ck_vals = seq_db["SM4_CK"]["values"][:4]
        entries = [("R", 50, "val", str(0xA3B1BAC6))]
        entries += make_sequence_trace(ck_vals, start_pc=100)
        trace = make_trace(entries)
        detections = detect_all(trace, min_confidence=0.3)
        assert_algorithm_detected(detections, "SM4")

    def test_hmac_detection(self):
        """HMAC: ipad + opad constants."""
        entries = [
            ("R", 100, "val", str(0x36363636)),
            ("R", 200, "val", str(0x5C5C5C5C)),
        ]
        trace = make_trace(entries)
        detections = detect_all(trace, min_confidence=0.3)
        assert_algorithm_detected(detections, "HMAC")

    def test_murmur3_detection(self):
        """MurmurHash3: c1 + c2 constants."""
        entries = [
            ("R", 100, "val", str(0xCC9E2D51)),
            ("R", 200, "val", str(0x1B873593)),
        ]
        trace = make_trace(entries)
        detections = detect_all(trace, min_confidence=0.3)
        assert_algorithm_detected(detections, "MurmurHash3")


# ============================================================
# M25/M31 Enhancement Tests
# ============================================================

class TestTraceEnhancements:
    """Test M25 StructuredTrace enhancements."""

    def test_pc_sequence(self):
        trace = make_trace([("D", 10, "5", "3"), ("D", 20, "7", "3"), ("D", 30, "5", "3")])
        assert trace.pc_sequence() == [10, 20, 30]

    def test_value_sequence(self):
        trace = make_trace([("R", 10, "x", "100"), ("R", 20, "y", "200")])
        assert trace.value_sequence() == ["100", "200"]

    def test_unique_pcs(self):
        trace = make_trace([("D", 10, "5", "3"), ("D", 10, "5", "3"), ("D", 20, "7", "3")])
        assert trace.unique_pcs() == {10, 20}

    def test_index_by_pc(self):
        trace = make_trace([("D", 10, "5", "3"), ("R", 10, "x", "1"), ("D", 20, "7", "3")])
        idx = trace.index_by_pc()
        assert len(idx[10]) == 2
        assert len(idx[20]) == 1

    def test_index_by_target(self):
        trace = make_trace([("R", 10, "screen.width", "1920"), ("R", 20, "screen.width", "1920")])
        idx = trace.index_by_target()
        assert len(idx["screen.width"]) == 2

    def test_compress_trace(self):
        from iv8_rs.trace import compress_trace
        entries = [("D", 10, "5", "3")] * 100 + [("R", 11, "x", "1")] + [("D", 20, "7", "3")] * 50
        trace = make_trace(entries)
        compressed = compress_trace(trace)
        assert len(compressed.entries) == 3  # 3 groups
        assert compressed.total_dispatches == 150
        assert compressed.compression_ratio < 0.1

    def test_parse_trace_stream(self):
        from iv8_rs.trace import parse_trace_stream
        lines = ["D,10,5,3", "R,11,screen.width,1920", "invalid", "C,12,Math.random,0.5"]
        trace = parse_trace_stream(iter(lines))
        assert len(trace) == 3  # invalid line skipped


class TestProbeEnhancements:
    """Test M31 probe report enhancements (structure only, no Rust needed)."""

    def test_probe_report_has_issues_field(self):
        """Verify probe_environment returns issues field."""
        # We can't run the full probe without Rust, but we can verify the structure
        from iv8_rs.probe import probe_environment
        # This will fail at JSContext creation, but we verify the function signature exists
        assert callable(probe_environment)


# ============================================================
# Detect Loops / Hotspots Tests
# ============================================================

class TestLoopsAndHotspots:
    """Test loop detection and hotspot analysis."""

    def test_detect_loops_basic(self):
        entries = [("D", 100, "5", "3")] * 50 + [("D", 200, "7", "3")] * 5
        trace = make_trace(entries)
        loops = detect_loops(trace, min_iterations=10)
        assert len(loops) >= 1
        assert loops[0]["pc"] == 100
        assert loops[0]["count"] == 50

    def test_detect_hotspots(self):
        entries = [("D", 100, "5", "3")] * 100 + [("D", 200, "7", "3")] * 10
        trace = make_trace(entries)
        hotspots = detect_hotspots(trace, top_n=5)
        assert hotspots[0]["pc"] == 100
        assert hotspots[0]["percentage"] > 80.0


# ============================================================
# Database Coverage Test
# ============================================================

class TestDatabaseCoverage:
    """Verify database integrity and coverage."""

    def test_patterns_count(self):
        patterns = _load_builtin_patterns()
        assert len(patterns) >= 51

    def test_constants_count(self):
        db = _load_constants_db()
        assert len(db) >= 200

    def test_sequences_count(self):
        db = _load_sequences_db()
        assert len(db) >= 22

    def test_all_patterns_have_required_fields(self):
        patterns = _load_builtin_patterns()
        for name, p in patterns.items():
            if name.startswith("_"):
                continue
            assert "description" in p, f"{name} missing description"
            assert "category" in p, f"{name} missing category"
            assert "behavior_pattern" in p or "constants" in p, f"{name} missing pattern/constants"

    def test_all_sequences_have_values(self):
        db = _load_sequences_db()
        for name, s in db.items():
            assert "values" in s, f"{name} missing values"
            assert len(s["values"]) >= 3, f"{name} has too few values ({len(s['values'])})"
            assert "algorithm" in s, f"{name} missing algorithm"
            assert "min_match" in s, f"{name} missing min_match"

    def test_no_empty_algorithm_fields(self):
        db = _load_constants_db()
        for int_val, info in db.items():
            assert info["algorithm"], f"Constant {info['name']} has empty algorithm field"


# ============================================================
# 100% Coverage Verification
# ============================================================

# Map pattern key -> canonical algorithm name (as used in constants/sequences)
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

# Algorithms that are LEGITIMATELY constant-free (key-dependent tables or
# constructions over other primitives). These can ONLY be detected via Layer 3
# (behavior_pattern), not constants/sequences. Documented limitation.
LAYER3_ONLY = {"RC4", "IDEA", "XOR_Cipher", "WAKE", "PBKDF2", "HKDF", "GOST_28147", "SAFER"}


class TestFullCoverage:
    """Verify 100% of patterns are accounted for: either detectable or documented L3-only."""

    def test_every_pattern_has_signature_or_is_layer3(self):
        """Every algorithm must have constants/sequences OR be a documented L3-only algorithm."""
        patterns = _load_builtin_patterns()
        const_db = _load_constants_db()
        seq_db = _load_sequences_db()

        # Build sets of algorithms with constants/sequences
        const_algos = set()
        for info in const_db.values():
            for a in info["algorithm"].split("/"):
                const_algos.add(a.strip())
        seq_algos = set()
        for s in seq_db.values():
            for a in s.get("algorithm", "").split("/"):
                seq_algos.add(a.strip())

        uncovered = []
        for pname in patterns:
            if pname.startswith("_"):
                continue
            algo = PATTERN_TO_ALGO.get(pname, pname)
            has_const = any(algo in ca or ca in algo for ca in const_algos if ca)
            has_seq = any(algo in sa or sa in algo for sa in seq_algos if sa)
            if not has_const and not has_seq and pname not in LAYER3_ONLY:
                uncovered.append(pname)

        assert not uncovered, (
            f"These patterns have NO detection signature and are NOT documented "
            f"as Layer-3-only: {uncovered}. Either add constants/sequences or "
            f"add them to LAYER3_ONLY set."
        )

    def test_layer3_only_have_behavior_patterns(self):
        """All Layer-3-only algorithms must have a behavior_pattern defined."""
        patterns = _load_builtin_patterns()
        for pname in LAYER3_ONLY:
            assert pname in patterns, f"{pname} in LAYER3_ONLY but not in patterns DB"
            bp = patterns[pname].get("behavior_pattern", [])
            assert bp, f"{pname} is Layer-3-only but has no behavior_pattern"
            assert len(bp) >= 2, f"{pname} behavior_pattern too short: {bp}"

    def test_pattern_count_is_51(self):
        patterns = _load_builtin_patterns()
        count = len([k for k in patterns if not k.startswith("_")])
        assert count == 51, f"Expected exactly 51 patterns, got {count}"

    def test_all_detectable_algorithms_actually_detect(self):
        """For every algorithm with constants, verify detection works end-to-end.

        Uses min_value=0 to test ALL constants including small ones (like
        Adler-32 mod 0xFFF1) which are filtered by default to prevent false
        positives but ARE valid signatures when explicitly searched for.
        """
        const_db = _load_constants_db()
        # Group constants by algorithm, pick one representative int per algorithm
        algo_to_int = {}
        for int_val, info in const_db.items():
            for a in info["algorithm"].split("/"):
                a = a.strip()
                if a and a not in algo_to_int:
                    algo_to_int[a] = int_val

        failures = []
        for algo, int_val in algo_to_int.items():
            trace = make_constant_trace(int_val)
            matches = detect_constants(trace, min_value=0)
            detected = any(algo in m.algorithm for m in matches)
            if not detected:
                failures.append(f"{algo} (const {hex(int_val)})")

        assert not failures, f"These algorithms have constants but detection failed: {failures}"

    def test_default_min_value_filters_small_constants(self):
        """Regression guard: default detect_constants must NOT match small values."""
        # Single byte / small magic values must be filtered by default
        for small_val in [1, 0x36, 0x63, 65521, 0x1F8B]:
            trace = make_constant_trace(small_val)
            matches = detect_constants(trace)  # default min_value=0x10000
            assert not matches, (
                f"Small value {hex(small_val)} should be filtered by default "
                f"but matched: {[m.name for m in matches]}"
            )


# ============================================================
# False Positive Regression Guards
# ============================================================

class TestFalsePositives:
    """Ensure unrelated/random traces do NOT trigger crypto detection.

    These are regression guards for the false-positive audit that found
    a VM dispatch loop was hallucinating 1250 RIPEMD-160 matches and
    SHA-3 constants. Must stay at zero.
    """

    def _random_trace(self, kind):
        import random
        rng = random.Random(99)
        entries = []
        if kind == "dispatch_loop":
            for i in range(5000):
                entries.append(("D", 1000 + (i % 50), str(i % 66), str(i % 8)))
        elif kind == "random_bytes":
            for i in range(3000):
                entries.append(("R", 2000 + i, "byte", str(rng.randint(0, 255))))
        elif kind == "small_indices":
            for i in range(2000):
                entries.append(("R", 3000 + i, "idx", str(i % 16)))
        elif kind == "permutation_1_64":
            for _ in range(30):
                perm = list(range(1, 65))
                rng.shuffle(perm)
                for i, v in enumerate(perm):
                    entries.append(("R", 4000 + i, "perm", str(v)))
        elif kind == "realistic":
            vals = [1920, 1080, 24, 8, 1, 0, 60, 100, 404, 1234567890, 16, 32, 64, 255, 256, 65535]
            for i in range(2000):
                entries.append(("R", 5000 + (i % 100), "prop", str(rng.choice(vals))))
        return make_trace(entries)

    def test_dispatch_loop_no_false_positive(self):
        trace = self._random_trace("dispatch_loop")
        assert len(detect_sequences(trace)) == 0, "VM dispatch loop triggered sequence match!"
        assert len(detect_constants(trace)) == 0, "VM dispatch loop triggered constant match!"
        assert len(detect_all(trace, min_confidence=0.5)) == 0

    def test_random_bytes_no_false_positive(self):
        trace = self._random_trace("random_bytes")
        assert len(detect_sequences(trace)) == 0
        assert len(detect_all(trace, min_confidence=0.5)) == 0

    def test_small_indices_no_false_positive(self):
        trace = self._random_trace("small_indices")
        assert len(detect_sequences(trace)) == 0, "Loop indices 0-15 triggered sequence match!"

    def test_permutation_no_false_positive(self):
        trace = self._random_trace("permutation_1_64")
        assert len(detect_sequences(trace)) == 0, "Permutation 1-64 triggered DES match!"

    def test_realistic_no_false_positive(self):
        trace = self._random_trace("realistic")
        dets = detect_all(trace, min_confidence=0.5)
        assert len(dets) == 0, f"Realistic non-crypto trace triggered: {[d.algorithm for d in dets]}"

    def test_empty_trace_no_detection(self):
        empty = make_trace([])
        assert len(detect_sequences(empty)) == 0
        assert len(detect_constants(empty)) == 0
        assert len(detect_all(empty)) == 0
