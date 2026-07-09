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
    let public = X25519PublicKey::from(private_bytes);
    (public, private_bytes.to_vec())
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
