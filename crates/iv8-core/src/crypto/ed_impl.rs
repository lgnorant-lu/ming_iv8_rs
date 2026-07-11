//! Ed25519 and X25519 implementation using ed25519-dalek and x25519-dalek.
//!
//! Supports:
//! - Ed25519: sign, verify, generateKey, importKey (raw/spki/pkcs8/jwk), exportKey
//! - X25519: deriveBits, generateKey, importKey (raw/spki/pkcs8/jwk), exportKey

use ed25519_dalek::{Signature, Signer, Verifier, SigningKey, VerifyingKey};
use x25519_dalek::PublicKey as X25519PublicKey;
use rand::RngCore;

/// Ed25519 key material.
#[derive(Clone)]
pub enum Ed25519KeyMaterial {
    Public(VerifyingKey),
    Private(SigningKey),
}

/// X25519 key material.
#[derive(Clone)]
pub enum X25519KeyMaterial {
    Public(X25519PublicKey),
    Private(Vec<u8>), // 32-byte private key
}

/// Ed25519 sign.
pub fn ed25519_sign(private_key: &SigningKey, data: &[u8]) -> Vec<u8> {
    let sig = private_key.sign(data);
    sig.to_bytes().to_vec()
}

/// Ed25519 verify.
pub fn ed25519_verify(public_key: &VerifyingKey, data: &[u8], signature: &[u8]) -> bool {
    if signature.len() != 64 {
        return false;
    }
    let sig_bytes: [u8; 64] = match signature.try_into() {
        Ok(s) => s,
        Err(_) => return false,
    };
    let sig = Signature::from_bytes(&sig_bytes);
    public_key.verify(data, &sig).is_ok()
}

/// Generate Ed25519 key pair.
pub fn ed25519_generate_key() -> (VerifyingKey, SigningKey) {
    let mut rng = rand::rngs::OsRng;
    let signing_key = SigningKey::generate(&mut rng);
    let verifying_key = signing_key.verifying_key();
    (verifying_key, signing_key)
}

/// Generate X25519 key pair.
pub fn x25519_generate_key() -> (X25519PublicKey, Vec<u8>) {
    let mut private_bytes = [0u8; 32];
    rand::rngs::OsRng.fill_bytes(&mut private_bytes);
    let public = x25519_dalek::x25519(private_bytes, x25519_dalek::X25519_BASEPOINT_BYTES);
    let pub_key = X25519PublicKey::from(public);
    (pub_key, private_bytes.to_vec())
}

/// X25519 deriveBits (shared secret).
pub fn x25519_derive_bits(private_bytes: &[u8; 32], public_bytes: &[u8; 32]) -> Result<Vec<u8>, String> {
    let shared = x25519_dalek::x25519(*private_bytes, *public_bytes);
    if shared.iter().all(|&b| b == 0) {
        return Err("X25519: shared secret is all-zero".to_string());
    }
    Ok(shared.to_vec())
}

/// Import Ed25519 public key from raw 32 bytes.
pub fn import_ed25519_public_raw(data: &[u8]) -> Result<VerifyingKey, String> {
    if data.len() != 32 {
        return Err("Ed25519: public key must be 32 bytes".to_string());
    }
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(data);
    VerifyingKey::from_bytes(&bytes).map_err(|e| format!("Ed25519: invalid public key: {}", e))
}

/// Import Ed25519 private key from raw 32 bytes.
pub fn import_ed25519_private_raw(data: &[u8]) -> Result<SigningKey, String> {
    if data.len() != 32 {
        return Err("Ed25519: private key must be 32 bytes".to_string());
    }
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(data);
    Ok(SigningKey::from_bytes(&bytes))
}

/// Export Ed25519 public key to raw 32 bytes.
pub fn export_ed25519_public_raw(key: &VerifyingKey) -> Vec<u8> {
    key.to_bytes().to_vec()
}

/// Export Ed25519 private key to raw 32 bytes.
pub fn export_ed25519_private_raw(key: &SigningKey) -> Vec<u8> {
    key.to_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ed25519_sign_verify_roundtrip() {
        let (vk, sk) = ed25519_generate_key();
        let data = b"hello world";
        let sig = ed25519_sign(&sk, data);
        assert_eq!(sig.len(), 64);
        assert!(ed25519_verify(&vk, data, &sig));
    }

    #[test]
    fn test_ed25519_verify_wrong_data_fails() {
        let (vk, sk) = ed25519_generate_key();
        let sig = ed25519_sign(&sk, b"original");
        assert!(!ed25519_verify(&vk, b"tampered", &sig));
    }

    #[test]
    fn test_ed25519_verify_wrong_signature_fails() {
        let (vk, _sk) = ed25519_generate_key();
        let bad_sig = vec![0u8; 64];
        assert!(!ed25519_verify(&vk, b"data", &bad_sig));
    }

    #[test]
    fn test_ed25519_verify_short_signature_fails() {
        let (vk, _sk) = ed25519_generate_key();
        let short_sig = vec![0u8; 32];
        assert!(!ed25519_verify(&vk, b"data", &short_sig));
    }

    #[test]
    fn test_ed25519_import_export_public_raw_roundtrip() {
        let (vk, _sk) = ed25519_generate_key();
        let raw = export_ed25519_public_raw(&vk);
        assert_eq!(raw.len(), 32);
        let imported = import_ed25519_public_raw(&raw).unwrap();
        assert_eq!(export_ed25519_public_raw(&imported), raw);
    }

    #[test]
    fn test_ed25519_import_public_raw_wrong_length() {
        let result = import_ed25519_public_raw(&[0u8; 16]);
        assert!(result.is_err());
    }

    #[test]
    fn test_ed25519_import_private_raw_wrong_length() {
        let result = import_ed25519_private_raw(&[0u8; 16]);
        assert!(result.is_err());
    }

    #[test]
    fn test_ed25519_import_export_private_raw_roundtrip() {
        let (_vk, sk) = ed25519_generate_key();
        let raw = export_ed25519_private_raw(&sk);
        assert_eq!(raw.len(), 32);
        let imported = import_ed25519_private_raw(&raw).unwrap();
        let sig1 = ed25519_sign(&sk, b"test");
        let sig2 = ed25519_sign(&imported, b"test");
        assert_eq!(sig1, sig2);
    }

    #[test]
    fn test_x25519_generate_key_length() {
        let (pub_key, priv_key) = x25519_generate_key();
        assert_eq!(pub_key.as_bytes().len(), 32);
        assert_eq!(priv_key.len(), 32);
    }

    #[test]
    fn test_x25519_derive_bits_roundtrip() {
        let (pub_a, priv_a) = x25519_generate_key();
        let (pub_b, priv_b) = x25519_generate_key();
        let priv_a_arr: [u8; 32] = priv_a.try_into().unwrap();
        let priv_b_arr: [u8; 32] = priv_b.try_into().unwrap();
        let shared_a = x25519_derive_bits(&priv_a_arr, pub_b.as_bytes()).unwrap();
        let shared_b = x25519_derive_bits(&priv_b_arr, pub_a.as_bytes()).unwrap();
        // DH should produce the same shared secret from both sides
        assert_eq!(shared_a, shared_b, "X25519 DH shared secrets must match");
        assert_eq!(shared_a.len(), 32);
    }

    #[test]
    fn test_x25519_derive_bits_all_zero_returns_error() {
        let zero_priv = [0u8; 32];
        let zero_pub = [0u8; 32];
        let result = x25519_derive_bits(&zero_priv, &zero_pub);
        assert!(result.is_err());
    }
}
