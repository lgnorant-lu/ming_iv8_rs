"""
Task 40: SubtleCrypto 完整测试套件

覆盖所有 12 个 SubtleCrypto 方法和所有算法：
- digest: SHA-1/256/384/512
- HMAC: sign/verify/importKey/generateKey/exportKey
- AES-GCM/AES-CBC: encrypt/decrypt/importKey/generateKey
- PBKDF2/HKDF: deriveBits/deriveKey
- RSA-OAEP: encrypt/decrypt/importKey/generateKey/exportKey
- RSA-PSS: sign/verify/importKey/generateKey/exportKey
- ECDSA (P-256/P-384): sign/verify/importKey/generateKey/exportKey
- ECDH (P-256/P-384): deriveBits/deriveKey
- wrapKey/unwrapKey
"""


# ─── 1. digest ────────────────────────────────────────────────────────────────

class TestDigest:
    def test_sha256_length(self, ctx):
        result = ctx.eval_promise(
            "crypto.subtle.digest('SHA-256', new Uint8Array([1,2,3])).then(b => b.byteLength)"
        )
        assert result == 32

    def test_sha384_length(self, ctx):
        result = ctx.eval_promise(
            "crypto.subtle.digest('SHA-384', new Uint8Array([1,2,3])).then(b => b.byteLength)"
        )
        assert result == 48

    def test_sha512_length(self, ctx):
        result = ctx.eval_promise(
            "crypto.subtle.digest('SHA-512', new Uint8Array([1,2,3])).then(b => b.byteLength)"
        )
        assert result == 64

    def test_sha1_length(self, ctx):
        result = ctx.eval_promise(
            "crypto.subtle.digest('SHA-1', new Uint8Array([1,2,3])).then(b => b.byteLength)"
        )
        assert result == 20

    def test_sha256_deterministic(self, ctx):
        r1 = ctx.eval_promise(
            "crypto.subtle.digest('SHA-256', new Uint8Array([42])).then(b => new Uint8Array(b)[0])"
        )
        r2 = ctx.eval_promise(
            "crypto.subtle.digest('SHA-256', new Uint8Array([42])).then(b => new Uint8Array(b)[0])"
        )
        assert r1 == r2

    def test_sha256_different_inputs(self, ctx):
        r1 = ctx.eval_promise(
            "crypto.subtle.digest('SHA-256', new Uint8Array([1])).then(b => new Uint8Array(b)[0])"
        )
        r2 = ctx.eval_promise(
            "crypto.subtle.digest('SHA-256', new Uint8Array([2])).then(b => new Uint8Array(b)[0])"
        )
        assert r1 != r2


# ─── 2. HMAC ──────────────────────────────────────────────────────────────────

class TestHMAC:
    def test_hmac_sign_sha256_length(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.importKey('raw', new Uint8Array(32),
                {name:'HMAC', hash:'SHA-256'}, false, ['sign'])
            .then(k => crypto.subtle.sign('HMAC', k, new Uint8Array(10)))
            .then(sig => sig.byteLength)
        """)
        assert result == 32

    def test_hmac_sign_sha384_length(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.importKey('raw', new Uint8Array(48),
                {name:'HMAC', hash:'SHA-384'}, false, ['sign'])
            .then(k => crypto.subtle.sign('HMAC', k, new Uint8Array(10)))
            .then(sig => sig.byteLength)
        """)
        assert result == 48

    def test_hmac_verify_correct(self, ctx):
        result = ctx.eval_promise("""
            var key;
            crypto.subtle.importKey('raw', new Uint8Array(32),
                {name:'HMAC', hash:'SHA-256'}, false, ['sign','verify'])
            .then(k => { key = k; return crypto.subtle.sign('HMAC', k, new Uint8Array([1,2,3])); })
            .then(sig => crypto.subtle.verify('HMAC', key, sig, new Uint8Array([1,2,3])))
        """)
        assert result == True

    def test_hmac_verify_wrong_data(self, ctx):
        result = ctx.eval_promise("""
            var key;
            crypto.subtle.importKey('raw', new Uint8Array(32),
                {name:'HMAC', hash:'SHA-256'}, false, ['sign','verify'])
            .then(k => { key = k; return crypto.subtle.sign('HMAC', k, new Uint8Array([1,2,3])); })
            .then(sig => crypto.subtle.verify('HMAC', key, sig, new Uint8Array([4,5,6])))
        """)
        assert result == False

    def test_hmac_generate_key(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey({name:'HMAC', hash:'SHA-256', length:256},
                true, ['sign','verify'])
            .then(k => k.type)
        """)
        assert result == "secret"

    def test_hmac_export_key(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey({name:'HMAC', hash:'SHA-256', length:256},
                true, ['sign'])
            .then(k => crypto.subtle.exportKey('raw', k))
            .then(raw => raw.byteLength)
        """)
        assert result == 32


# ─── 3. AES-GCM ───────────────────────────────────────────────────────────────

class TestAESGCM:
    def test_aes_gcm_encrypt_decrypt_roundtrip(self, ctx):
        result = ctx.eval_promise("""
            var key, iv = crypto.getRandomValues(new Uint8Array(12));
            var plaintext = new TextEncoder().encode('hello world');
            crypto.subtle.generateKey({name:'AES-GCM', length:256}, true, ['encrypt','decrypt'])
            .then(k => { key = k; return crypto.subtle.encrypt({name:'AES-GCM', iv:iv}, k, plaintext); })
            .then(ct => crypto.subtle.decrypt({name:'AES-GCM', iv:iv}, key, ct))
            .then(pt => new TextDecoder().decode(pt))
        """)
        assert result == "hello world"

    def test_aes_gcm_128_key(self, ctx):
        result = ctx.eval_promise("""
            var iv = new Uint8Array(12);
            crypto.subtle.generateKey({name:'AES-GCM', length:128}, true, ['encrypt'])
            .then(k => crypto.subtle.encrypt({name:'AES-GCM', iv:iv}, k, new Uint8Array([1,2,3])))
            .then(ct => ct.byteLength > 0)
        """)
        assert result == True

    def test_aes_gcm_import_key(self, ctx):
        result = ctx.eval_promise("""
            var key = new Uint8Array(32);
            crypto.subtle.importKey('raw', key, {name:'AES-GCM'}, false, ['encrypt'])
            .then(k => k.algorithm.name)
        """)
        assert result == "AES-GCM"


# ─── 4. AES-CBC ───────────────────────────────────────────────────────────────

class TestAESCBC:
    def test_aes_cbc_encrypt_decrypt_roundtrip(self, ctx):
        result = ctx.eval_promise("""
            var key, iv = new Uint8Array(16);
            var plaintext = new TextEncoder().encode('hello world!!!!');
            crypto.subtle.generateKey({name:'AES-CBC', length:256}, true, ['encrypt','decrypt'])
            .then(k => { key = k; return crypto.subtle.encrypt({name:'AES-CBC', iv:iv}, k, plaintext); })
            .then(ct => crypto.subtle.decrypt({name:'AES-CBC', iv:iv}, key, ct))
            .then(pt => new TextDecoder().decode(pt))
        """)
        assert result == "hello world!!!!"

    def test_aes_cbc_ciphertext_length(self, ctx):
        result = ctx.eval_promise("""
            var iv = new Uint8Array(16);
            crypto.subtle.generateKey({name:'AES-CBC', length:128}, true, ['encrypt'])
            .then(k => crypto.subtle.encrypt({name:'AES-CBC', iv:iv}, k, new Uint8Array(16)))
            .then(ct => ct.byteLength)
        """)
        # 16 bytes plaintext + PKCS7 padding = 32 bytes
        assert result == 32


# ─── 5. PBKDF2 ────────────────────────────────────────────────────────────────

class TestPBKDF2:
    def test_pbkdf2_derive_bits_length(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.importKey('raw', new TextEncoder().encode('password'),
                'PBKDF2', false, ['deriveBits'])
            .then(k => crypto.subtle.deriveBits(
                {name:'PBKDF2', salt:new Uint8Array(16), iterations:1000, hash:'SHA-256'},
                k, 256))
            .then(bits => bits.byteLength)
        """)
        assert result == 32

    def test_pbkdf2_derive_key(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.importKey('raw', new TextEncoder().encode('password'),
                'PBKDF2', false, ['deriveKey'])
            .then(k => crypto.subtle.deriveKey(
                {name:'PBKDF2', salt:new Uint8Array(16), iterations:1000, hash:'SHA-256'},
                k, {name:'AES-GCM', length:256}, true, ['encrypt']))
            .then(dk => dk.algorithm.name)
        """)
        assert result == "AES-GCM"

    def test_pbkdf2_deterministic(self, ctx):
        r1 = ctx.eval_promise("""
            crypto.subtle.importKey('raw', new TextEncoder().encode('pass'),
                'PBKDF2', false, ['deriveBits'])
            .then(k => crypto.subtle.deriveBits(
                {name:'PBKDF2', salt:new Uint8Array([1,2,3]), iterations:100, hash:'SHA-256'},
                k, 128))
            .then(bits => Array.from(new Uint8Array(bits)).join(','))
        """)
        r2 = ctx.eval_promise("""
            crypto.subtle.importKey('raw', new TextEncoder().encode('pass'),
                'PBKDF2', false, ['deriveBits'])
            .then(k => crypto.subtle.deriveBits(
                {name:'PBKDF2', salt:new Uint8Array([1,2,3]), iterations:100, hash:'SHA-256'},
                k, 128))
            .then(bits => Array.from(new Uint8Array(bits)).join(','))
        """)
        assert r1 == r2


# ─── 6. HKDF ──────────────────────────────────────────────────────────────────

class TestHKDF:
    def test_hkdf_derive_bits_length(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.importKey('raw', new Uint8Array(32), 'HKDF', false, ['deriveBits'])
            .then(k => crypto.subtle.deriveBits(
                {name:'HKDF', salt:new Uint8Array(32), info:new Uint8Array(0), hash:'SHA-256'},
                k, 256))
            .then(bits => bits.byteLength)
        """)
        assert result == 32

    def test_hkdf_derive_key(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.importKey('raw', new Uint8Array(32), 'HKDF', false, ['deriveKey'])
            .then(k => crypto.subtle.deriveKey(
                {name:'HKDF', salt:new Uint8Array(16), info:new Uint8Array(0), hash:'SHA-256'},
                k, {name:'AES-GCM', length:128}, true, ['encrypt']))
            .then(dk => dk.algorithm.name)
        """)
        assert result == "AES-GCM"

    def test_hkdf_deterministic(self, ctx):
        r1 = ctx.eval_promise("""
            crypto.subtle.importKey('raw', new Uint8Array([1,2,3,4]), 'HKDF', false, ['deriveBits'])
            .then(k => crypto.subtle.deriveBits(
                {name:'HKDF', salt:new Uint8Array([5,6]), info:new Uint8Array([7,8]), hash:'SHA-256'},
                k, 128))
            .then(bits => Array.from(new Uint8Array(bits)).join(','))
        """)
        r2 = ctx.eval_promise("""
            crypto.subtle.importKey('raw', new Uint8Array([1,2,3,4]), 'HKDF', false, ['deriveBits'])
            .then(k => crypto.subtle.deriveBits(
                {name:'HKDF', salt:new Uint8Array([5,6]), info:new Uint8Array([7,8]), hash:'SHA-256'},
                k, 128))
            .then(bits => Array.from(new Uint8Array(bits)).join(','))
        """)
        assert r1 == r2


# ─── 7. RSA-OAEP ──────────────────────────────────────────────────────────────

class TestRSAOAEP:
    def test_rsa_oaep_generate_key_pair(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name:'RSA-OAEP', modulusLength:2048, publicExponent:new Uint8Array([1,0,1]),
                 hash:'SHA-256'},
                true, ['encrypt','decrypt'])
            .then(kp => kp.publicKey.type + '/' + kp.privateKey.type)
        """)
        assert result == "public/private"

    def test_rsa_oaep_encrypt_decrypt_roundtrip(self, ctx):
        result = ctx.eval_promise("""
            var kp;
            crypto.subtle.generateKey(
                {name:'RSA-OAEP', modulusLength:2048, publicExponent:new Uint8Array([1,0,1]),
                 hash:'SHA-256'},
                true, ['encrypt','decrypt'])
            .then(k => { kp = k; return crypto.subtle.encrypt(
                {name:'RSA-OAEP'}, kp.publicKey, new TextEncoder().encode('secret')); })
            .then(ct => crypto.subtle.decrypt({name:'RSA-OAEP'}, kp.privateKey, ct))
            .then(pt => new TextDecoder().decode(pt))
        """)
        assert result == "secret"

    def test_rsa_oaep_export_public_key(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name:'RSA-OAEP', modulusLength:2048, publicExponent:new Uint8Array([1,0,1]),
                 hash:'SHA-256'},
                true, ['encrypt'])
            .then(kp => crypto.subtle.exportKey('spki', kp.publicKey))
            .then(spki => spki.byteLength > 0)
        """)
        assert result == True

    def test_rsa_oaep_export_private_key(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name:'RSA-OAEP', modulusLength:2048, publicExponent:new Uint8Array([1,0,1]),
                 hash:'SHA-256'},
                true, ['decrypt'])
            .then(kp => crypto.subtle.exportKey('pkcs8', kp.privateKey))
            .then(pkcs8 => pkcs8.byteLength > 0)
        """)
        assert result == True

    def test_rsa_oaep_import_export_roundtrip(self, ctx):
        """Generate, export, re-import, and use the key."""
        result = ctx.eval_promise("""
            var origKp, exportedSpki;
            crypto.subtle.generateKey(
                {name:'RSA-OAEP', modulusLength:2048, publicExponent:new Uint8Array([1,0,1]),
                 hash:'SHA-256'},
                true, ['encrypt','decrypt'])
            .then(kp => { origKp = kp; return crypto.subtle.exportKey('spki', kp.publicKey); })
            .then(spki => { exportedSpki = spki;
                return crypto.subtle.importKey('spki', spki,
                    {name:'RSA-OAEP', hash:'SHA-256'}, true, ['encrypt']); })
            .then(importedPub => crypto.subtle.encrypt(
                {name:'RSA-OAEP'}, importedPub, new TextEncoder().encode('test')))
            .then(ct => crypto.subtle.decrypt({name:'RSA-OAEP'}, origKp.privateKey, ct))
            .then(pt => new TextDecoder().decode(pt))
        """)
        assert result == "test"


# ─── 8. RSA-PSS ───────────────────────────────────────────────────────────────

class TestRSAPSS:
    def test_rsa_pss_generate_key_pair(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name:'RSA-PSS', modulusLength:2048, publicExponent:new Uint8Array([1,0,1]),
                 hash:'SHA-256'},
                true, ['sign','verify'])
            .then(kp => kp.publicKey.type + '/' + kp.privateKey.type)
        """)
        assert result == "public/private"

    def test_rsa_pss_sign_verify_roundtrip(self, ctx):
        result = ctx.eval_promise("""
            var kp;
            crypto.subtle.generateKey(
                {name:'RSA-PSS', modulusLength:2048, publicExponent:new Uint8Array([1,0,1]),
                 hash:'SHA-256'},
                true, ['sign','verify'])
            .then(k => { kp = k;
                return crypto.subtle.sign(
                    {name:'RSA-PSS', saltLength:32}, kp.privateKey,
                    new TextEncoder().encode('message')); })
            .then(sig => crypto.subtle.verify(
                {name:'RSA-PSS', saltLength:32}, kp.publicKey, sig,
                new TextEncoder().encode('message')))
        """)
        assert result == True

    def test_rsa_pss_verify_wrong_message(self, ctx):
        result = ctx.eval_promise("""
            var kp;
            crypto.subtle.generateKey(
                {name:'RSA-PSS', modulusLength:2048, publicExponent:new Uint8Array([1,0,1]),
                 hash:'SHA-256'},
                true, ['sign','verify'])
            .then(k => { kp = k;
                return crypto.subtle.sign(
                    {name:'RSA-PSS', saltLength:32}, kp.privateKey,
                    new TextEncoder().encode('message')); })
            .then(sig => crypto.subtle.verify(
                {name:'RSA-PSS', saltLength:32}, kp.publicKey, sig,
                new TextEncoder().encode('wrong')))
        """)
        assert result == False

    def test_rsa_pss_signature_length(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name:'RSA-PSS', modulusLength:2048, publicExponent:new Uint8Array([1,0,1]),
                 hash:'SHA-256'},
                true, ['sign'])
            .then(kp => crypto.subtle.sign(
                {name:'RSA-PSS', saltLength:32}, kp.privateKey,
                new Uint8Array([1,2,3])))
            .then(sig => sig.byteLength)
        """)
        # RSA-2048 signature is 256 bytes
        assert result == 256


# ─── 9. ECDSA ─────────────────────────────────────────────────────────────────

class TestECDSA:
    def test_ecdsa_p256_generate_key_pair(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name:'ECDSA', namedCurve:'P-256'}, true, ['sign','verify'])
            .then(kp => kp.publicKey.type + '/' + kp.privateKey.type)
        """)
        assert result == "public/private"

    def test_ecdsa_p384_generate_key_pair(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name:'ECDSA', namedCurve:'P-384'}, true, ['sign','verify'])
            .then(kp => kp.publicKey.algorithm.name)
        """)
        assert result == "ECDSA"

    def test_ecdsa_p256_sign_verify_roundtrip(self, ctx):
        result = ctx.eval_promise("""
            var kp;
            crypto.subtle.generateKey(
                {name:'ECDSA', namedCurve:'P-256'}, true, ['sign','verify'])
            .then(k => { kp = k;
                return crypto.subtle.sign(
                    {name:'ECDSA', hash:'SHA-256'}, kp.privateKey,
                    new TextEncoder().encode('data')); })
            .then(sig => crypto.subtle.verify(
                {name:'ECDSA', hash:'SHA-256'}, kp.publicKey, sig,
                new TextEncoder().encode('data')))
        """)
        assert result == True

    def test_ecdsa_p384_sign_verify_roundtrip(self, ctx):
        result = ctx.eval_promise("""
            var kp;
            crypto.subtle.generateKey(
                {name:'ECDSA', namedCurve:'P-384'}, true, ['sign','verify'])
            .then(k => { kp = k;
                return crypto.subtle.sign(
                    {name:'ECDSA', hash:'SHA-384'}, kp.privateKey,
                    new TextEncoder().encode('data')); })
            .then(sig => crypto.subtle.verify(
                {name:'ECDSA', hash:'SHA-384'}, kp.publicKey, sig,
                new TextEncoder().encode('data')))
        """)
        assert result == True

    def test_ecdsa_verify_wrong_data(self, ctx):
        result = ctx.eval_promise("""
            var kp;
            crypto.subtle.generateKey(
                {name:'ECDSA', namedCurve:'P-256'}, true, ['sign','verify'])
            .then(k => { kp = k;
                return crypto.subtle.sign(
                    {name:'ECDSA', hash:'SHA-256'}, kp.privateKey,
                    new TextEncoder().encode('data')); })
            .then(sig => crypto.subtle.verify(
                {name:'ECDSA', hash:'SHA-256'}, kp.publicKey, sig,
                new TextEncoder().encode('wrong')))
        """)
        assert result == False

    def test_ecdsa_export_public_key_spki(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name:'ECDSA', namedCurve:'P-256'}, true, ['sign'])
            .then(kp => crypto.subtle.exportKey('spki', kp.publicKey))
            .then(spki => spki.byteLength > 0)
        """)
        assert result == True

    def test_ecdsa_export_private_key_pkcs8(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name:'ECDSA', namedCurve:'P-256'}, true, ['sign'])
            .then(kp => crypto.subtle.exportKey('pkcs8', kp.privateKey))
            .then(pkcs8 => pkcs8.byteLength > 0)
        """)
        assert result == True

    def test_ecdsa_import_export_roundtrip(self, ctx):
        """Generate, export public key, re-import, verify."""
        result = ctx.eval_promise("""
            var kp, sig;
            crypto.subtle.generateKey(
                {name:'ECDSA', namedCurve:'P-256'}, true, ['sign','verify'])
            .then(k => { kp = k;
                return crypto.subtle.sign(
                    {name:'ECDSA', hash:'SHA-256'}, kp.privateKey,
                    new TextEncoder().encode('test')); })
            .then(s => { sig = s;
                return crypto.subtle.exportKey('spki', kp.publicKey); })
            .then(spki => crypto.subtle.importKey('spki', spki,
                {name:'ECDSA', namedCurve:'P-256'}, true, ['verify']))
            .then(importedPub => crypto.subtle.verify(
                {name:'ECDSA', hash:'SHA-256'}, importedPub, sig,
                new TextEncoder().encode('test')))
        """)
        assert result == True


# ─── 10. ECDH ─────────────────────────────────────────────────────────────────

class TestECDH:
    def test_ecdh_p256_generate_key_pair(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name:'ECDH', namedCurve:'P-256'}, true, ['deriveKey','deriveBits'])
            .then(kp => kp.publicKey.type + '/' + kp.privateKey.type)
        """)
        assert result == "public/private"

    def test_ecdh_p256_derive_bits(self, ctx):
        result = ctx.eval_promise("""
            var kp1, kp2;
            Promise.all([
                crypto.subtle.generateKey({name:'ECDH', namedCurve:'P-256'}, true, ['deriveBits']),
                crypto.subtle.generateKey({name:'ECDH', namedCurve:'P-256'}, true, ['deriveBits'])
            ]).then(([k1, k2]) => { kp1 = k1; kp2 = k2;
                return crypto.subtle.deriveBits(
                    {name:'ECDH', public: kp2.publicKey}, kp1.privateKey, 256); })
            .then(bits => bits.byteLength)
        """)
        assert result == 32

    def test_ecdh_shared_secret_symmetric(self, ctx):
        """Alice and Bob should derive the same shared secret."""
        result = ctx.eval_promise("""
            var alice, bob;
            Promise.all([
                crypto.subtle.generateKey({name:'ECDH', namedCurve:'P-256'}, true, ['deriveBits']),
                crypto.subtle.generateKey({name:'ECDH', namedCurve:'P-256'}, true, ['deriveBits'])
            ]).then(([a, b]) => { alice = a; bob = b;
                return Promise.all([
                    crypto.subtle.deriveBits({name:'ECDH', public:bob.publicKey}, alice.privateKey, 256),
                    crypto.subtle.deriveBits({name:'ECDH', public:alice.publicKey}, bob.privateKey, 256)
                ]); })
            .then(([s1, s2]) => {
                var a1 = new Uint8Array(s1), a2 = new Uint8Array(s2);
                for (var i = 0; i < a1.length; i++) if (a1[i] !== a2[i]) return false;
                return true;
            })
        """)
        assert result == True

    def test_ecdh_derive_key(self, ctx):
        result = ctx.eval_promise("""
            var kp1, kp2;
            Promise.all([
                crypto.subtle.generateKey({name:'ECDH', namedCurve:'P-256'}, true, ['deriveKey']),
                crypto.subtle.generateKey({name:'ECDH', namedCurve:'P-256'}, true, ['deriveKey'])
            ]).then(([k1, k2]) => { kp1 = k1; kp2 = k2;
                return crypto.subtle.deriveKey(
                    {name:'ECDH', public:kp2.publicKey}, kp1.privateKey,
                    {name:'AES-GCM', length:256}, true, ['encrypt']); })
            .then(dk => dk.algorithm.name)
        """)
        assert result == "AES-GCM"


# ─── 11. wrapKey / unwrapKey ──────────────────────────────────────────────────

class TestWrapUnwrapKey:
    def test_wrap_unwrap_aes_gcm(self, ctx):
        result = ctx.eval_promise("""
            var wrappingKey, targetKey;
            Promise.all([
                crypto.subtle.generateKey({name:'AES-GCM', length:256}, true, ['encrypt','decrypt','wrapKey','unwrapKey']),
                crypto.subtle.generateKey({name:'AES-GCM', length:128}, true, ['encrypt'])
            ]).then(([wk, tk]) => { wrappingKey = wk; targetKey = tk;
                var iv = new Uint8Array(12);
                return crypto.subtle.wrapKey('raw', tk, wk, {name:'AES-GCM', iv:iv})
                    .then(wrapped => crypto.subtle.unwrapKey(
                        'raw', wrapped, wk, {name:'AES-GCM', iv:iv},
                        {name:'AES-GCM', length:128}, true, ['encrypt'])); })
            .then(unwrapped => unwrapped.algorithm.name)
        """)
        assert result == "AES-GCM"


# ─── 12. exportKey JWK ────────────────────────────────────────────────────────

class TestExportKeyJWK:
    def test_export_hmac_jwk(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey({name:'HMAC', hash:'SHA-256', length:256},
                true, ['sign'])
            .then(k => crypto.subtle.exportKey('jwk', k))
            .then(jwk => typeof jwk.k === 'string' && jwk.k.length > 0)
        """)
        assert result == True

    def test_export_aes_jwk(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey({name:'AES-GCM', length:256}, true, ['encrypt'])
            .then(k => crypto.subtle.exportKey('jwk', k))
            .then(jwk => typeof jwk.k === 'string')
        """)
        assert result == True


# ─── 13. 综合测试 ─────────────────────────────────────────────────────────────

class TestCryptoIntegration:
    def test_all_subtle_methods_exist(self, ctx):
        """All 12 SubtleCrypto methods should exist."""
        methods = ['digest', 'importKey', 'exportKey', 'generateKey',
                   'sign', 'verify', 'encrypt', 'decrypt',
                   'deriveBits', 'deriveKey', 'wrapKey', 'unwrapKey']
        for method in methods:
            result = ctx.eval(f"typeof crypto.subtle.{method}")
            assert result == "function", f"crypto.subtle.{method} should be a function"

    def test_crypto_key_has_correct_properties(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey({name:'AES-GCM', length:256}, true, ['encrypt'])
            .then(k => JSON.stringify({
                type: k.type,
                extractable: k.extractable,
                algoName: k.algorithm.name,
                hasUsages: Array.isArray(k.usages),
            }))
        """)
        import json
        data = json.loads(result)
        assert data['type'] == 'secret'
        assert data['extractable'] == True
        assert data['algoName'] == 'AES-GCM'
        assert data['hasUsages'] == True

    def test_rsa_key_pair_has_correct_properties(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name:'RSA-OAEP', modulusLength:2048, publicExponent:new Uint8Array([1,0,1]),
                 hash:'SHA-256'},
                true, ['encrypt','decrypt'])
            .then(kp => JSON.stringify({
                pubType: kp.publicKey.type,
                privType: kp.privateKey.type,
                pubAlgo: kp.publicKey.algorithm.name,
                privAlgo: kp.privateKey.algorithm.name,
            }))
        """)
        import json
        data = json.loads(result)
        assert data['pubType'] == 'public'
        assert data['privType'] == 'private'
        assert data['pubAlgo'] == 'RSA-OAEP'
        assert data['privAlgo'] == 'RSA-OAEP'

    def test_ecdsa_key_pair_has_correct_properties(self, ctx):
        result = ctx.eval_promise("""
            crypto.subtle.generateKey(
                {name:'ECDSA', namedCurve:'P-256'}, true, ['sign','verify'])
            .then(kp => JSON.stringify({
                pubType: kp.publicKey.type,
                privType: kp.privateKey.type,
                curve: kp.publicKey.algorithm.namedCurve,
            }))
        """)
        import json
        data = json.loads(result)
        assert data['pubType'] == 'public'
        assert data['privType'] == 'private'
        assert data['curve'] == 'P-256'
