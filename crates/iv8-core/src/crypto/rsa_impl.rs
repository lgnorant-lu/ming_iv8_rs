//! RSA-OAEP and RSA-PSS implementation using the `rsa` crate.
//!
//! Supports:
//! - RSA-OAEP: encrypt, decrypt, importKey (spki/pkcs8/jwk), generateKey, exportKey
//! - RSA-PSS: sign, verify

use rand::rngs::OsRng;
use rsa::pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey};
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey};
use rsa::{Oaep, RsaPrivateKey, RsaPublicKey};
use sha1::Sha1;
use sha2::{Sha256, Sha384, Sha512};

/// RSA key material stored in a CryptoKey.
#[derive(Clone)]
pub enum RsaKeyMaterial {
    Public(RsaPublicKey),
    Private(RsaPrivateKey),
    KeyPair {
        public: RsaPublicKey,
        private: RsaPrivateKey,
    },
}

/// RSA-OAEP encrypt.
pub fn rsa_oaep_encrypt(
    public_key: &RsaPublicKey,
    data: &[u8],
    hash: &str,
) -> Result<Vec<u8>, String> {
    let mut rng = OsRng;
    match hash.to_uppercase().replace("-", "").as_str() {
        "SHA1" => {
            let padding = Oaep::new::<Sha1>();
            public_key
                .encrypt(&mut rng, padding, data)
                .map_err(|e| e.to_string())
        }
        "SHA256" => {
            let padding = Oaep::new::<Sha256>();
            public_key
                .encrypt(&mut rng, padding, data)
                .map_err(|e| e.to_string())
        }
        "SHA384" => {
            let padding = Oaep::new::<Sha384>();
            public_key
                .encrypt(&mut rng, padding, data)
                .map_err(|e| e.to_string())
        }
        "SHA512" => {
            let padding = Oaep::new::<Sha512>();
            public_key
                .encrypt(&mut rng, padding, data)
                .map_err(|e| e.to_string())
        }
        _ => Err(format!("RSA-OAEP: unsupported hash '{}'", hash)),
    }
}

/// RSA-OAEP decrypt.
pub fn rsa_oaep_decrypt(
    private_key: &RsaPrivateKey,
    data: &[u8],
    hash: &str,
) -> Result<Vec<u8>, String> {
    match hash.to_uppercase().replace("-", "").as_str() {
        "SHA1" => {
            let padding = Oaep::new::<Sha1>();
            private_key
                .decrypt(padding, data)
                .map_err(|e| e.to_string())
        }
        "SHA256" => {
            let padding = Oaep::new::<Sha256>();
            private_key
                .decrypt(padding, data)
                .map_err(|e| e.to_string())
        }
        "SHA384" => {
            let padding = Oaep::new::<Sha384>();
            private_key
                .decrypt(padding, data)
                .map_err(|e| e.to_string())
        }
        "SHA512" => {
            let padding = Oaep::new::<Sha512>();
            private_key
                .decrypt(padding, data)
                .map_err(|e| e.to_string())
        }
        _ => Err(format!("RSA-OAEP: unsupported hash '{}'", hash)),
    }
}

/// RSA-PSS sign.
pub fn rsa_pss_sign(
    private_key: &RsaPrivateKey,
    data: &[u8],
    hash: &str,
    _salt_length: usize,
) -> Result<Vec<u8>, String> {
    use rsa::signature::RandomizedSigner;
    let mut rng = OsRng;

    match hash.to_uppercase().replace("-", "").as_str() {
        "SHA256" => {
            use rsa::pss::SigningKey;
            let signing_key = SigningKey::<Sha256>::new(private_key.clone());
            use rsa::signature::SignatureEncoding;
            let sig = signing_key.sign_with_rng(&mut rng, data);
            Ok(sig.to_bytes().to_vec())
        }
        "SHA384" => {
            use rsa::pss::SigningKey;
            let signing_key = SigningKey::<Sha384>::new(private_key.clone());
            use rsa::signature::SignatureEncoding;
            let sig = signing_key.sign_with_rng(&mut rng, data);
            Ok(sig.to_bytes().to_vec())
        }
        "SHA512" => {
            use rsa::pss::SigningKey;
            let signing_key = SigningKey::<Sha512>::new(private_key.clone());
            use rsa::signature::SignatureEncoding;
            let sig = signing_key.sign_with_rng(&mut rng, data);
            Ok(sig.to_bytes().to_vec())
        }
        _ => Err(format!("RSA-PSS: unsupported hash '{}'", hash)),
    }
}

/// RSA-PSS verify.
pub fn rsa_pss_verify(
    public_key: &RsaPublicKey,
    data: &[u8],
    signature: &[u8],
    hash: &str,
) -> bool {
    use rsa::signature::Verifier;

    match hash.to_uppercase().replace("-", "").as_str() {
        "SHA256" => {
            use rsa::pss::{Signature, VerifyingKey};
            let verifying_key = VerifyingKey::<Sha256>::new(public_key.clone());
            if let Ok(sig) = Signature::try_from(signature) {
                verifying_key.verify(data, &sig).is_ok()
            } else {
                false
            }
        }
        "SHA384" => {
            use rsa::pss::{Signature, VerifyingKey};
            let verifying_key = VerifyingKey::<Sha384>::new(public_key.clone());
            if let Ok(sig) = Signature::try_from(signature) {
                verifying_key.verify(data, &sig).is_ok()
            } else {
                false
            }
        }
        "SHA512" => {
            use rsa::pss::{Signature, VerifyingKey};
            let verifying_key = VerifyingKey::<Sha512>::new(public_key.clone());
            if let Ok(sig) = Signature::try_from(signature) {
                verifying_key.verify(data, &sig).is_ok()
            } else {
                false
            }
        }
        _ => false,
    }
}

/// Generate RSA key pair.
pub fn rsa_generate_key(modulus_length: usize) -> Result<(RsaPublicKey, RsaPrivateKey), String> {
    let mut rng = OsRng;
    let private_key = RsaPrivateKey::new(&mut rng, modulus_length).map_err(|e| e.to_string())?;
    let public_key = RsaPublicKey::from(&private_key);
    Ok((public_key, private_key))
}

/// Import RSA public key from SPKI DER bytes.
pub fn import_rsa_public_key_spki(der: &[u8]) -> Result<RsaPublicKey, String> {
    RsaPublicKey::from_public_key_der(der).map_err(|e| e.to_string())
}

/// Import RSA private key from PKCS8 DER bytes.
pub fn import_rsa_private_key_pkcs8(der: &[u8]) -> Result<RsaPrivateKey, String> {
    RsaPrivateKey::from_pkcs8_der(der).map_err(|e| e.to_string())
}

/// Export RSA public key to SPKI DER bytes.
pub fn export_rsa_public_key_spki(key: &RsaPublicKey) -> Result<Vec<u8>, String> {
    key.to_public_key_der()
        .map(|d| d.to_vec())
        .map_err(|e| e.to_string())
}

/// Export RSA private key to PKCS8 DER bytes.
pub fn export_rsa_private_key_pkcs8(key: &RsaPrivateKey) -> Result<Vec<u8>, String> {
    key.to_pkcs8_der()
        .map(|d| d.as_bytes().to_vec())
        .map_err(|e| e.to_string())
}

/// Import RSA public key from PKCS1 DER bytes.
pub fn import_rsa_public_key_pkcs1(der: &[u8]) -> Result<RsaPublicKey, String> {
    RsaPublicKey::from_pkcs1_der(der).map_err(|e| e.to_string())
}

/// Import RSA private key from PKCS1 DER bytes.
pub fn import_rsa_private_key_pkcs1(der: &[u8]) -> Result<RsaPrivateKey, String> {
    RsaPrivateKey::from_pkcs1_der(der).map_err(|e| e.to_string())
}

// ─── RSA-PKCS1-v1.5 sign/verify ────────────────────────────────────

/// RSA-PKCS1-v1.5 sign.
pub fn rsa_pkcs1_sign(
    private_key: &RsaPrivateKey,
    data: &[u8],
    hash: &str,
) -> Result<Vec<u8>, String> {
    use rsa::pkcs1v15::SigningKey;
    use rsa::signature::{SignatureEncoding, Signer};

    match hash.to_uppercase().replace("-", "").as_str() {
        "SHA1" => {
            let signing_key = SigningKey::<Sha1>::new(private_key.clone());
            let sig = signing_key.sign(data);
            Ok(sig.to_bytes().to_vec())
        }
        "SHA256" => {
            let signing_key = SigningKey::<Sha256>::new(private_key.clone());
            let sig = signing_key.sign(data);
            Ok(sig.to_bytes().to_vec())
        }
        "SHA384" => {
            let signing_key = SigningKey::<Sha384>::new(private_key.clone());
            let sig = signing_key.sign(data);
            Ok(sig.to_bytes().to_vec())
        }
        "SHA512" => {
            let signing_key = SigningKey::<Sha512>::new(private_key.clone());
            let sig = signing_key.sign(data);
            Ok(sig.to_bytes().to_vec())
        }
        _ => Err(format!("RSA-PKCS1-v1.5: unsupported hash '{}'", hash)),
    }
}

/// RSA-PKCS1-v1.5 verify.
pub fn rsa_pkcs1_verify(
    public_key: &RsaPublicKey,
    data: &[u8],
    signature: &[u8],
    hash: &str,
) -> bool {
    use rsa::signature::Verifier;
    use rsa::pkcs1v15::{Signature, VerifyingKey};

    match hash.to_uppercase().replace("-", "").as_str() {
        "SHA1" => {
            let verifying_key = VerifyingKey::<Sha1>::new(public_key.clone());
            if let Ok(sig) = Signature::try_from(signature) {
                verifying_key.verify(data, &sig).is_ok()
            } else {
                false
            }
        }
        "SHA256" => {
            let verifying_key = VerifyingKey::<Sha256>::new(public_key.clone());
            if let Ok(sig) = Signature::try_from(signature) {
                verifying_key.verify(data, &sig).is_ok()
            } else {
                false
            }
        }
        "SHA384" => {
            let verifying_key = VerifyingKey::<Sha384>::new(public_key.clone());
            if let Ok(sig) = Signature::try_from(signature) {
                verifying_key.verify(data, &sig).is_ok()
            } else {
                false
            }
        }
        "SHA512" => {
            let verifying_key = VerifyingKey::<Sha512>::new(public_key.clone());
            if let Ok(sig) = Signature::try_from(signature) {
                verifying_key.verify(data, &sig).is_ok()
            } else {
                false
            }
        }
        _ => false,
    }
}

// ─── AES-KW (Key Wrap) ──────────────────────────────────────────────

/// AES-KW wrap (RFC 3394).
pub fn aes_kw_wrap(kek: &[u8], key_data: &[u8]) -> Result<Vec<u8>, String> {
    if key_data.len() % 8 != 0 || key_data.is_empty() {
        return Err("AES-KW: key data must be non-empty multiple of 8 bytes".to_string());
    }
    match kek.len() {
        16 => wrap_impl::<aes::Aes128>(kek, key_data),
        24 => wrap_impl::<aes::Aes192>(kek, key_data),
        32 => wrap_impl::<aes::Aes256>(kek, key_data),
        _ => Err("AES-KW: invalid KEK size (must be 16/24/32 bytes)".to_string()),
    }
}

/// AES-KW unwrap (RFC 3394).
pub fn aes_kw_unwrap(kek: &[u8], wrapped: &[u8]) -> Result<Vec<u8>, String> {
    if wrapped.len() % 8 != 0 || wrapped.len() < 16 {
        return Err("AES-KW: wrapped data must be at least 16 bytes and multiple of 8".to_string());
    }
    match kek.len() {
        16 => unwrap_impl::<aes::Aes128>(kek, wrapped),
        24 => unwrap_impl::<aes::Aes192>(kek, wrapped),
        32 => unwrap_impl::<aes::Aes256>(kek, wrapped),
        _ => Err("AES-KW: invalid KEK size (must be 16/24/32 bytes)".to_string()),
    }
}

fn wrap_impl<C: aes::cipher::BlockCipher + aes::cipher::KeyInit + aes::cipher::BlockEncrypt>(kek: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    let cipher = C::new_from_slice(kek).map_err(|_| "AES-KW: cipher init failed")?;
    let n = data.len() / 8;
    let mut a = [0xA6u8; 8];
    let mut r: Vec<[u8; 8]> = data.chunks(8).map(|c| c.try_into().unwrap()).collect();

    for j in 0..6u64 {
        for i in 0..n {
            let mut block = [0u8; 16];
            block[..8].copy_from_slice(&a);
            block[8..].copy_from_slice(&r[i]);
            let mut ga = aes::cipher::generic_array::GenericArray::from_mut_slice(&mut block);
            cipher.encrypt_block(&mut ga);
            a.copy_from_slice(&block[..8]);
            a = u64::from_be_bytes(a)
                .wrapping_add(n as u64 * j + i as u64 + 1)
                .to_be_bytes();
            r[i].copy_from_slice(&block[8..]);
        }
    }

    let mut result = Vec::with_capacity(8 * (n + 1));
    result.extend_from_slice(&a);
    for chunk in &r {
        result.extend_from_slice(chunk);
    }
    Ok(result)
}

fn unwrap_impl<C: aes::cipher::BlockCipher + aes::cipher::KeyInit + aes::cipher::BlockDecrypt>(kek: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    let cipher = C::new_from_slice(kek).map_err(|_| "AES-KW: cipher init failed")?;
    let n = data.len() / 8 - 1;
    let mut a: [u8; 8] = data[..8].try_into().map_err(|_| "AES-KW: bad data")?;
    let mut r: Vec<[u8; 8]> = data[8..]
        .chunks(8)
        .map(|c| c.try_into().unwrap())
        .collect();

    for j in (0..6u64).rev() {
        for i in (0..n).rev() {
            let t = n as u64 * j + i as u64 + 1;
            a = u64::from_be_bytes(a).wrapping_sub(t).to_be_bytes();
            let mut block = [0u8; 16];
            block[..8].copy_from_slice(&a);
            block[8..].copy_from_slice(&r[i]);
            let mut ga = aes::cipher::generic_array::GenericArray::from_mut_slice(&mut block);
            cipher.decrypt_block(&mut ga);
            a.copy_from_slice(&block[..8]);
            r[i].copy_from_slice(&block[8..]);
        }
    }

    if a != [0xA6u8; 8] {
        return Err("AES-KW: integrity check failed".to_string());
    }

    let mut result = Vec::with_capacity(8 * n);
    for chunk in &r {
        result.extend_from_slice(chunk);
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa_generate_key_2048() {
        let (pub_key, priv_key) = rsa_generate_key(2048).unwrap();
        let pub_spki = export_rsa_public_key_spki(&pub_key).unwrap();
        assert!(pub_spki.len() > 100);
        let priv_pkcs8 = export_rsa_private_key_pkcs8(&priv_key).unwrap();
        assert!(priv_pkcs8.len() > 100);
    }

    #[test]
    fn test_rsa_oaep_encrypt_decrypt_roundtrip() {
        let (pub_key, priv_key) = rsa_generate_key(2048).unwrap();
        let data = b"secret message";
        let encrypted = rsa_oaep_encrypt(&pub_key, data, "SHA-256").unwrap();
        assert_ne!(encrypted, data);
        let decrypted = rsa_oaep_decrypt(&priv_key, &encrypted, "SHA-256").unwrap();
        assert_eq!(decrypted, data);
    }

    #[test]
    fn test_rsa_pss_sign_verify_roundtrip() {
        let (pub_key, priv_key) = rsa_generate_key(2048).unwrap();
        let data = b"message to sign";
        let sig = rsa_pss_sign(&priv_key, data, "SHA-256", 32).unwrap();
        assert!(rsa_pss_verify(&pub_key, data, &sig, "SHA-256"));
    }

    #[test]
    fn test_rsa_pss_verify_wrong_data_fails() {
        let (pub_key, priv_key) = rsa_generate_key(2048).unwrap();
        let sig = rsa_pss_sign(&priv_key, b"original", "SHA-256", 32).unwrap();
        assert!(!rsa_pss_verify(&pub_key, b"tampered", &sig, "SHA-256"));
    }

    #[test]
    fn test_rsa_pkcs1_sign_verify_roundtrip() {
        let (pub_key, priv_key) = rsa_generate_key(2048).unwrap();
        let data = b"pkcs1 test";
        let sig = rsa_pkcs1_sign(&priv_key, data, "SHA-256").unwrap();
        assert!(rsa_pkcs1_verify(&pub_key, data, &sig, "SHA-256"));
    }

    #[test]
    fn test_rsa_export_import_public_key_spki_roundtrip() {
        let (pub_key, _priv_key) = rsa_generate_key(2048).unwrap();
        let spki = export_rsa_public_key_spki(&pub_key).unwrap();
        let imported = import_rsa_public_key_spki(&spki).unwrap();
        let spki2 = export_rsa_public_key_spki(&imported).unwrap();
        assert_eq!(spki, spki2);
    }

    #[test]
    fn test_rsa_export_import_private_key_pkcs8_roundtrip() {
        let (_pub_key, priv_key) = rsa_generate_key(2048).unwrap();
        let pkcs8 = export_rsa_private_key_pkcs8(&priv_key).unwrap();
        let imported = import_rsa_private_key_pkcs8(&pkcs8).unwrap();
        let pkcs8_2 = export_rsa_private_key_pkcs8(&imported).unwrap();
        assert_eq!(pkcs8, pkcs8_2);
    }

    #[test]
    fn test_aes_kw_wrap_unwrap_roundtrip() {
        let kek = [0u8; 16]; // AES-128 key
        let key_data = [0x42u8; 16]; // 128-bit key to wrap
        let wrapped = aes_kw_wrap(&kek, &key_data).unwrap();
        assert_ne!(wrapped, key_data);
        let unwrapped = aes_kw_unwrap(&kek, &wrapped).unwrap();
        assert_eq!(unwrapped, key_data);
    }

    #[test]
    fn test_aes_kw_unwrap_bad_data_fails() {
        let kek = [0u8; 16];
        let bad_wrapped = vec![0xFFu8; 24];
        let result = aes_kw_unwrap(&kek, &bad_wrapped);
        assert!(result.is_err());
    }

    #[test]
    fn test_import_rsa_public_key_spki_invalid_der() {
        let bad_der = vec![0x00, 0x01, 0x02];
        assert!(import_rsa_public_key_spki(&bad_der).is_err());
    }

    #[test]
    fn test_import_rsa_private_key_pkcs8_invalid_der() {
        let bad_der = vec![0x00, 0x01, 0x02];
        assert!(import_rsa_private_key_pkcs8(&bad_der).is_err());
    }
}
