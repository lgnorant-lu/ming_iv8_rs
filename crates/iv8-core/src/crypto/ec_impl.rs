//! ECDSA and ECDH implementation using p256/p384 crates.
//!
//! Supports:
//! - ECDSA (P-256, P-384): sign, verify, importKey, generateKey, exportKey
//! - ECDH (P-256, P-384): deriveBits

use p256::ecdsa::{SigningKey as P256SigningKey, VerifyingKey as P256VerifyingKey, Signature as P256Signature};
use p384::ecdsa::{SigningKey as P384SigningKey, VerifyingKey as P384VerifyingKey, Signature as P384Signature};
#[allow(unused_imports)]
use elliptic_curve::sec1::{ToEncodedPoint, FromEncodedPoint};
use rand::rngs::OsRng;

/// EC curve identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EcCurve {
    P256,
    P384,
}

impl EcCurve {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_uppercase().replace("-", "").as_str() {
            "P256" => Some(EcCurve::P256),
            "P384" => Some(EcCurve::P384),
            _ => None,
        }
    }
}

/// EC key material.
#[derive(Clone)]
pub enum EcKeyMaterial {
    P256Private(P256SigningKey),
    P256Public(P256VerifyingKey),
    P384Private(P384SigningKey),
    P384Public(P384VerifyingKey),
}

impl EcKeyMaterial {
    pub fn curve(&self) -> EcCurve {
        match self {
            EcKeyMaterial::P256Private(_) | EcKeyMaterial::P256Public(_) => EcCurve::P256,
            EcKeyMaterial::P384Private(_) | EcKeyMaterial::P384Public(_) => EcCurve::P384,
        }
    }

    pub fn key_type(&self) -> &'static str {
        match self {
            EcKeyMaterial::P256Private(_) | EcKeyMaterial::P384Private(_) => "private",
            EcKeyMaterial::P256Public(_) | EcKeyMaterial::P384Public(_) => "public",
        }
    }

    /// Export as raw bytes (uncompressed SEC1 point for public, raw scalar for private).
    pub fn to_raw_bytes(&self) -> Vec<u8> {
        match self {
            EcKeyMaterial::P256Private(k) => k.to_bytes().to_vec(),
            EcKeyMaterial::P256Public(k) => k.to_encoded_point(false).as_bytes().to_vec(),
            EcKeyMaterial::P384Private(k) => k.to_bytes().to_vec(),
            EcKeyMaterial::P384Public(k) => k.to_encoded_point(false).as_bytes().to_vec(),
        }
    }

    /// Export as PKCS8 DER (private key).
    pub fn to_pkcs8_der(&self) -> Result<Vec<u8>, String> {
        use pkcs8::EncodePrivateKey;
        match self {
            EcKeyMaterial::P256Private(k) => {
                k.to_pkcs8_der().map(|d| d.as_bytes().to_vec()).map_err(|e| e.to_string())
            }
            EcKeyMaterial::P384Private(k) => {
                k.to_pkcs8_der().map(|d| d.as_bytes().to_vec()).map_err(|e| e.to_string())
            }
            _ => Err("Cannot export public key as PKCS8".to_string()),
        }
    }

    /// Export as SPKI DER (public key).
    pub fn to_spki_der(&self) -> Result<Vec<u8>, String> {
        use spki::EncodePublicKey;
        match self {
            EcKeyMaterial::P256Public(k) => {
                k.to_public_key_der().map(|d| d.to_vec()).map_err(|e| e.to_string())
            }
            EcKeyMaterial::P384Public(k) => {
                k.to_public_key_der().map(|d| d.to_vec()).map_err(|e| e.to_string())
            }
            _ => Err("Cannot export private key as SPKI".to_string()),
        }
    }
}

/// Generate ECDSA key pair.
pub fn ecdsa_generate_key(curve: EcCurve) -> Result<(EcKeyMaterial, EcKeyMaterial), String> {
    let mut rng = OsRng;
    match curve {
        EcCurve::P256 => {
            let signing_key = P256SigningKey::random(&mut rng);
            let verifying_key = P256VerifyingKey::from(&signing_key);
            Ok((EcKeyMaterial::P256Private(signing_key), EcKeyMaterial::P256Public(verifying_key)))
        }
        EcCurve::P384 => {
            let signing_key = P384SigningKey::random(&mut rng);
            let verifying_key = P384VerifyingKey::from(&signing_key);
            Ok((EcKeyMaterial::P384Private(signing_key), EcKeyMaterial::P384Public(verifying_key)))
        }
    }
}

/// ECDSA sign.
pub fn ecdsa_sign(key: &EcKeyMaterial, data: &[u8], _hash: &str) -> Result<Vec<u8>, String> {
    use p256::ecdsa::signature::Signer;

    match key {
        EcKeyMaterial::P256Private(k) => {
            let sig: P256Signature = k.sign(data);
            Ok(sig.to_bytes().to_vec())
        }
        EcKeyMaterial::P384Private(k) => {
            let sig: P384Signature = k.sign(data);
            Ok(sig.to_bytes().to_vec())
        }
        _ => Err("ECDSA sign requires private key".to_string()),
    }
}

/// ECDSA verify.
pub fn ecdsa_verify(key: &EcKeyMaterial, data: &[u8], signature: &[u8]) -> bool {
    use p256::ecdsa::signature::Verifier;

    match key {
        EcKeyMaterial::P256Public(k) => {
            if let Ok(sig) = P256Signature::from_slice(signature) {
                k.verify(data, &sig).is_ok()
            } else { false }
        }
        EcKeyMaterial::P384Public(k) => {
            if let Ok(sig) = P384Signature::from_slice(signature) {
                k.verify(data, &sig).is_ok()
            } else { false }
        }
        _ => false,
    }
}

/// Import EC key from raw bytes.
pub fn import_ec_key_raw(bytes: &[u8], curve: EcCurve, key_type: &str) -> Result<EcKeyMaterial, String> {
    match (curve, key_type) {
        (EcCurve::P256, "private") => {
            P256SigningKey::from_slice(bytes)
                .map(EcKeyMaterial::P256Private)
                .map_err(|e| e.to_string())
        }
        (EcCurve::P256, "public") => {
            let point = p256::EncodedPoint::from_bytes(bytes).map_err(|e| e.to_string())?;
            P256VerifyingKey::from_encoded_point(&point)
                .map(EcKeyMaterial::P256Public)
                .map_err(|e| e.to_string())
        }
        (EcCurve::P384, "private") => {
            P384SigningKey::from_slice(bytes)
                .map(EcKeyMaterial::P384Private)
                .map_err(|e| e.to_string())
        }
        (EcCurve::P384, "public") => {
            let point = p384::EncodedPoint::from_bytes(bytes).map_err(|e| e.to_string())?;
            P384VerifyingKey::from_encoded_point(&point)
                .map(EcKeyMaterial::P384Public)
                .map_err(|e| e.to_string())
        }
        _ => Err(format!("Unsupported curve/key_type combination: {:?}/{}", curve, key_type)),
    }
}

/// Import EC private key from PKCS8 DER.
pub fn import_ec_private_key_pkcs8(der: &[u8], curve: EcCurve) -> Result<EcKeyMaterial, String> {
    use pkcs8::DecodePrivateKey;
    match curve {
        EcCurve::P256 => {
            P256SigningKey::from_pkcs8_der(der)
                .map(EcKeyMaterial::P256Private)
                .map_err(|e| e.to_string())
        }
        EcCurve::P384 => {
            P384SigningKey::from_pkcs8_der(der)
                .map(EcKeyMaterial::P384Private)
                .map_err(|e| e.to_string())
        }
    }
}

/// Import EC public key from SPKI DER.
pub fn import_ec_public_key_spki(der: &[u8], curve: EcCurve) -> Result<EcKeyMaterial, String> {
    use spki::DecodePublicKey;
    match curve {
        EcCurve::P256 => {
            P256VerifyingKey::from_public_key_der(der)
                .map(EcKeyMaterial::P256Public)
                .map_err(|e| e.to_string())
        }
        EcCurve::P384 => {
            P384VerifyingKey::from_public_key_der(der)
                .map(EcKeyMaterial::P384Public)
                .map_err(|e| e.to_string())
        }
    }
}

/// ECDH derive shared secret (raw bytes).
/// Returns the x-coordinate of the shared point.
pub fn ecdh_derive_bits(
    private_key: &EcKeyMaterial,
    public_key: &EcKeyMaterial,
    length_bits: usize,
) -> Result<Vec<u8>, String> {
    let length_bytes = length_bits.div_ceil(8);

    match (private_key, public_key) {
        (EcKeyMaterial::P256Private(priv_k), EcKeyMaterial::P256Public(pub_k)) => {
            // Use diffie-hellman via p256
            let shared = p256::ecdh::diffie_hellman(
                priv_k.as_nonzero_scalar(),
                pub_k.as_affine(),
            );
            let raw = shared.raw_secret_bytes();
            let bytes = raw.as_slice();
            Ok(bytes[..length_bytes.min(bytes.len())].to_vec())
        }
        (EcKeyMaterial::P384Private(priv_k), EcKeyMaterial::P384Public(pub_k)) => {
            let shared = p384::ecdh::diffie_hellman(
                priv_k.as_nonzero_scalar(),
                pub_k.as_affine(),
            );
            let raw = shared.raw_secret_bytes();
            let bytes = raw.as_slice();
            Ok(bytes[..length_bytes.min(bytes.len())].to_vec())
        }
        _ => Err("ECDH: key curve mismatch or wrong key type".to_string()),
    }
}
