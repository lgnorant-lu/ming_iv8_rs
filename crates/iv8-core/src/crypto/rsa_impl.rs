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
