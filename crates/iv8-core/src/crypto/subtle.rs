//! SubtleCrypto: complete Web Crypto API implementation.
// SAFETY: remaining expects are OOM-only or logic invariants
#![expect(clippy::expect_used, reason = "OOM or logic invariant")]
//!
//! Algorithms supported:
//! - digest: SHA-1, SHA-256, SHA-384, SHA-512
//! - HMAC: sign, verify, importKey, generateKey
//! - AES-GCM, AES-CBC: encrypt, decrypt, importKey, generateKey
//! - PBKDF2, HKDF: deriveBits, deriveKey
//! - RSA-OAEP: encrypt, decrypt, importKey (spki/pkcs8), generateKey, exportKey
//! - RSA-PSS: sign, verify, importKey (spki/pkcs8), generateKey, exportKey
//! - ECDSA (P-256, P-384): sign, verify, importKey (raw/spki/pkcs8), generateKey, exportKey
//! - ECDH (P-256, P-384): deriveBits, deriveKey, importKey (raw/spki/pkcs8), generateKey
//! - wrapKey, unwrapKey

use aes_gcm::aead::generic_array::GenericArray;
use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes128Gcm, Aes256Gcm};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha384, Sha512};

// AES-CBC
use aes::Aes128;
use cbc::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use ctr::Ctr32BE;
use ctr::cipher::StreamCipher;
use cbc::{Decryptor as CbcDecryptor, Encryptor as CbcEncryptor};

use crate::crypto::ec_impl::{EcCurve, EcKeyMaterial};
use crate::crypto::rsa_impl;

// ─── Key metadata ─────────────────────────────────────────────────────────────

/// Key metadata stored as JSON in __keyMeta__ field.
/// Allows reconstructing the key in any operation.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct KeyMeta {
    /// "secret" | "public" | "private"
    key_type: String,
    /// Algorithm name: "HMAC", "AES-GCM", "AES-CBC", "RSA-OAEP", "RSA-PSS", "ECDSA", "ECDH", "PBKDF2", "HKDF"
    algo: String,
    /// For HMAC/AES: hash algorithm
    hash: Option<String>,
    /// For EC: curve name "P-256" | "P-384"
    curve: Option<String>,
    /// For RSA: modulus length
    modulus_length: Option<usize>,
    /// Raw key bytes (base64-encoded)
    /// For symmetric: raw key bytes
    /// For RSA public: SPKI DER
    /// For RSA private: PKCS8 DER
    /// For EC public: SPKI DER
    /// For EC private: PKCS8 DER
    key_bytes_b64: String,
    /// Whether the key is extractable
    extractable: bool,
    /// Key usages
    usages: Vec<String>,
}

/// Simple base64 encode (no padding variant for internal use).
fn b64_encode(data: &[u8]) -> String {
    crate::canvas::canvas2d::base64_encode(data)
}

/// Simple base64 decode.
fn b64_decode(s: &str) -> Result<Vec<u8>, String> {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let s = s.trim_end_matches('=');
    let mut result = Vec::with_capacity(s.len() * 3 / 4);
    let mut buf = 0u32;
    let mut bits = 0u32;
    for ch in s.bytes() {
        let val = CHARS
            .iter()
            .position(|&c| c == ch)
            .ok_or_else(|| format!("invalid base64 char: {}", ch as char))?
            as u32;
        buf = (buf << 6) | val;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            result.push((buf >> bits) as u8);
        }
    }
    Ok(result)
}

/// Create a CryptoKey V8 object from KeyMeta.
fn make_crypto_key<'s>(scope: &v8::PinScope<'s, '_>, meta: &KeyMeta) -> v8::Local<'s, v8::Object> {
    let key_obj = v8::Object::new(scope);

    // type
    let type_key = crate::v8_utils::v8_string(scope, "type");
    let type_val = crate::v8_utils::v8_string(scope, &meta.key_type);
    key_obj.set(scope, type_key.into(), type_val.into());

    // algorithm
    let algo_key = crate::v8_utils::v8_string(scope, "algorithm");
    let algo_obj = v8::Object::new(scope);
    let name_key = crate::v8_utils::v8_string(scope, "name");
    let name_val = crate::v8_utils::v8_string(scope, &meta.algo);
    algo_obj.set(scope, name_key.into(), name_val.into());
    if let Some(ref hash) = meta.hash {
        let hash_key = crate::v8_utils::v8_string(scope, "hash");
        let hash_val = crate::v8_utils::v8_string(scope, hash);
        algo_obj.set(scope, hash_key.into(), hash_val.into());
    }
    if let Some(ref curve) = meta.curve {
        let curve_key = crate::v8_utils::v8_string(scope, "namedCurve");
        let curve_val = crate::v8_utils::v8_string(scope, curve);
        algo_obj.set(scope, curve_key.into(), curve_val.into());
    }
    if let Some(ml) = meta.modulus_length {
        let ml_key = crate::v8_utils::v8_string(scope, "modulusLength");
        algo_obj.set(
            scope,
            ml_key.into(),
            v8::Integer::new(scope, ml as i32).into(),
        );
    }
    // For symmetric keys (AES/HMAC), set length from key bytes
    if meta.key_type == "secret" {
        if let Ok(raw) = b64_decode(&meta.key_bytes_b64) {
            let len_bits = (raw.len() * 8) as i32;
            let len_key = crate::v8_utils::v8_string(scope, "length");
            algo_obj.set(
                scope,
                len_key.into(),
                v8::Integer::new(scope, len_bits).into(),
            );
        }
    }
    key_obj.set(scope, algo_key.into(), algo_obj.into());

    // extractable
    let ext_key = crate::v8_utils::v8_string(scope, "extractable");
    key_obj.set(
        scope,
        ext_key.into(),
        v8::Boolean::new(scope, meta.extractable).into(),
    );

    // usages
    let usages_key = crate::v8_utils::v8_string(scope, "usages");
    let usages_arr = v8::Array::new(scope, meta.usages.len() as i32);
    for (i, u) in meta.usages.iter().enumerate() {
        if let Some(s) = v8::String::new(scope, u) {
            usages_arr.set_index(scope, i as u32, s.into());
        }
    }
    key_obj.set(scope, usages_key.into(), usages_arr.into());

    // __keyMeta__ (hidden, stores JSON for reconstruction)
    let meta_json = serde_json::to_string(meta).unwrap_or_default();
    let meta_key = crate::v8_utils::v8_string(scope, "__keyMeta__");
    let meta_val = crate::v8_utils::v8_string(scope, &meta_json);
    key_obj.define_own_property(
        scope,
        meta_key.into(),
        meta_val.into(),
        v8::PropertyAttribute::DONT_ENUM,
    );

    // __rawKey__ (for backward compat with existing HMAC/AES code)
    if let Ok(raw) = b64_decode(&meta.key_bytes_b64) {
        let store = v8::ArrayBuffer::new_backing_store_from_vec(raw);
        let ab = v8::ArrayBuffer::with_backing_store(scope, &store.into());
        let raw_key = crate::v8_utils::v8_string(scope, "__rawKey__");
        key_obj.define_own_property(
            scope,
            raw_key.into(),
            ab.into(),
            v8::PropertyAttribute::DONT_ENUM,
        );
    }

    key_obj
}

/// Extract KeyMeta from a CryptoKey V8 object.
fn extract_key_meta(
    scope: &v8::PinScope<'_, '_>,
    key_arg: v8::Local<v8::Value>,
) -> Option<KeyMeta> {
    if !key_arg.is_object() {
        return None;
    }
    let key_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(key_arg) };
    let meta_key = v8::String::new(scope, "__keyMeta__")?;
    let meta_val = key_obj.get(scope, meta_key.into())?;
    if meta_val.is_string() {
        let json = meta_val.to_rust_string_lossy(scope);
        serde_json::from_str(&json).ok()
    } else {
        None
    }
}

/// Reconstruct RSA public key from KeyMeta.
fn meta_to_rsa_public(meta: &KeyMeta) -> Result<rsa::RsaPublicKey, String> {
    let der = b64_decode(&meta.key_bytes_b64)?;
    rsa_impl::import_rsa_public_key_spki(&der)
        .or_else(|_| rsa_impl::import_rsa_public_key_pkcs1(&der))
}

/// Reconstruct RSA private key from KeyMeta.
fn meta_to_rsa_private(meta: &KeyMeta) -> Result<rsa::RsaPrivateKey, String> {
    let der = b64_decode(&meta.key_bytes_b64)?;
    rsa_impl::import_rsa_private_key_pkcs8(&der)
        .or_else(|_| rsa_impl::import_rsa_private_key_pkcs1(&der))
}

/// Reconstruct EC key from KeyMeta.
fn meta_to_ec_key(meta: &KeyMeta) -> Result<EcKeyMaterial, String> {
    let der = b64_decode(&meta.key_bytes_b64)?;
    let curve = meta
        .curve
        .as_deref()
        .and_then(EcCurve::from_name)
        .ok_or("EC key missing curve")?;
    match meta.key_type.as_str() {
        "private" => crate::crypto::ec_impl::import_ec_private_key_pkcs8(&der, curve)
            .or_else(|_| crate::crypto::ec_impl::import_ec_key_raw(&der, curve, "private")),
        "public" => crate::crypto::ec_impl::import_ec_public_key_spki(&der, curve)
            .or_else(|_| crate::crypto::ec_impl::import_ec_key_raw(&der, curve, "public")),
        _ => Err(format!("Unknown key type: {}", meta.key_type)),
    }
}

/// Get hash algorithm from algorithm object.
fn get_hash_from_algo(scope: &v8::PinScope<'_, '_>, algo_arg: v8::Local<v8::Value>) -> String {
    if algo_arg.is_object() {
        let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
        let hash_key = crate::v8_utils::v8_string(scope, "hash");
        if let Some(hash_val) = obj.get(scope, hash_key.into()) {
            return get_algorithm_name(scope, hash_val);
        }
    }
    "SHA-256".to_string()
}

/// Get salt length from RSA-PSS algorithm object.
fn get_salt_length(scope: &v8::PinScope<'_, '_>, algo_arg: v8::Local<v8::Value>) -> usize {
    if algo_arg.is_object() {
        let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
        let key = crate::v8_utils::v8_string(scope, "saltLength");
        if let Some(val) = obj.get(scope, key.into()) {
            return val.number_value(scope).unwrap_or(32.0) as usize;
        }
    }
    32
}

/// Install crypto.subtle on the global crypto object.
pub fn install_subtle_crypto(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    // Get or create crypto object
    let crypto_key = crate::v8_utils::v8_string(scope, "crypto");
    let crypto_obj = if let Some(existing) = global.get(scope, crypto_key.into()) {
        if existing.is_object() && !existing.is_null_or_undefined() {
            unsafe { v8::Local::<v8::Object>::cast_unchecked(existing) }
        } else {
            let obj = v8::Object::new(scope);
            global.set(scope, crypto_key.into(), obj.into());
            obj
        }
    } else {
        let obj = v8::Object::new(scope);
        global.set(scope, crypto_key.into(), obj.into());
        obj
    };

    // Create subtle object
    // Get SubtleCrypto.prototype from codegen constructor
    let subtle_ctor_key = crate::v8_utils::v8_string(scope, "SubtleCrypto");
    let proto_obj = if let Some(ctor_val) = global.get(scope, subtle_ctor_key.into()) {
        if ctor_val.is_function() {
            let ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(ctor_val) };
            let proto_key = crate::v8_utils::v8_string(scope, "prototype");
            if let Some(proto_val) = ctor.get(scope, proto_key.into()) {
                proto_val.to_object(scope)
            } else { None }
        } else { None }
    } else { None };

    // Create subtle object WITH SubtleCrypto.prototype as its prototype
    let subtle_obj = if let Some(ref proto) = proto_obj {
        v8::Object::with_prototype_and_properties(scope, (*proto).into(), &[], &[])
    } else {
        v8::Object::new(scope)
    };

    // Install methods on SubtleCrypto.prototype (or fallback to instance)
    let target = proto_obj.unwrap_or(subtle_obj);
    install_method(scope, target, "digest", 2, subtle_digest);
    install_method(scope, target, "importKey", 5, subtle_import_key);
    install_method(scope, target, "sign", 3, subtle_sign);
    install_method(scope, target, "verify", 4, subtle_verify);
    install_method(scope, target, "encrypt", 3, subtle_encrypt);
    install_method(scope, target, "decrypt", 3, subtle_decrypt);
    install_method(scope, target, "deriveBits", 3, subtle_derive_bits);
    install_method(scope, target, "deriveKey", 5, subtle_derive_key);
    install_method(scope, target, "generateKey", 3, subtle_generate_key);
    install_method(scope, target, "exportKey", 2, subtle_export_key);
    install_method(scope, target, "wrapKey", 4, subtle_wrap_key);
    install_method(scope, target, "unwrapKey", 7, subtle_unwrap_key);

    // Install crypto.subtle on Crypto.prototype as data property
    // (overwrites codegen's readonly accessor via create_data_property)
    let crypto_ctor_key2 = crate::v8_utils::v8_string(scope, "Crypto");
    if let Some(ctor_val) = global.get(scope, crypto_ctor_key2.into()) {
        if ctor_val.is_function() {
            let ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(ctor_val) };
            let proto_key = crate::v8_utils::v8_string(scope, "prototype");
            if let Some(proto_val) = ctor.get(scope, proto_key.into()) {
                if let Some(crypto_proto) = proto_val.to_object(scope) {
                    let subtle_key2 = crate::v8_utils::v8_string(scope, "subtle");
                    let _ = crypto_proto.create_data_property(scope, subtle_key2.into(), subtle_obj.into());
                }
            }
        }
    }
}

fn install_method(
    scope: &v8::PinScope<'_, '_>,
    obj: v8::Local<v8::Object>,
    name: &str,
    length: i32,
    callback: unsafe extern "C" fn(*const v8::FunctionCallbackInfo),
) {
    let name_str = crate::v8_utils::v8_string(scope, name);
    let tmpl = v8::FunctionTemplate::builder_raw(callback)
        .length(length)
        .build(scope);
    tmpl.set_class_name(name_str);
    let func = crate::v8_utils::v8_fn(scope, &tmpl);
    obj.set(scope, name_str.into(), func.into());
}

/// Extract bytes from a V8 value (ArrayBuffer, TypedArray, or DataView).
fn extract_bytes(_scope: &v8::PinScope<'_, '_>, value: v8::Local<v8::Value>) -> Option<Vec<u8>> {
    if value.is_array_buffer() {
        let ab: v8::Local<v8::ArrayBuffer> = unsafe { v8::Local::cast_unchecked(value) };
        let len = ab.byte_length();
        let mut buf = vec![0u8; len];
        if len > 0 {
            let store = ab.get_backing_store();
            if let Some(data_ptr) = store.data() {
                let slice =
                    unsafe { std::slice::from_raw_parts(data_ptr.as_ptr() as *const u8, len) };
                buf.copy_from_slice(slice);
            }
        }
        Some(buf)
    } else if value.is_typed_array() {
        let ta: v8::Local<v8::TypedArray> = unsafe { v8::Local::cast_unchecked(value) };
        let len = ta.byte_length();
        let mut buf = vec![0u8; len];
        if len > 0 {
            ta.copy_contents(&mut buf);
        }
        Some(buf)
    } else {
        None
    }
}

/// Get algorithm name from a V8 value (string or {name: string}).
fn get_algorithm_name(scope: &v8::PinScope<'_, '_>, value: v8::Local<v8::Value>) -> String {
    if value.is_string() {
        return value.to_rust_string_lossy(scope);
    }
    if value.is_object() {
        let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(value) };
        let name_key = crate::v8_utils::v8_string(scope, "name");
        if let Some(name_val) = obj.get(scope, name_key.into()) {
            return name_val.to_rust_string_lossy(scope);
        }
    }
    String::new()
}

/// Create an ArrayBuffer from bytes and wrap in a resolved Promise.
fn resolve_with_array_buffer(
    scope: &v8::PinScope<'_, '_>,
    resolver: v8::Local<v8::PromiseResolver>,
    data: &[u8],
) {
    let store = v8::ArrayBuffer::new_backing_store_from_vec(data.to_vec());
    let ab = v8::ArrayBuffer::with_backing_store(scope, &store.into());
    resolver.resolve(scope, ab.into());
}

/// crypto.subtle.digest(algorithm, data) → Promise<ArrayBuffer>
unsafe extern "C" fn subtle_digest(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 2 {
            let msg = crate::v8_utils::v8_string(scope, "digest requires 2 arguments");
            resolver.reject(scope, v8::Exception::type_error(scope, msg));
            return;
        }

        let algo = get_algorithm_name(scope, args.get(0));
        let data = match extract_bytes(scope, args.get(1)) {
            Some(d) => d,
            None => {
                let msg = crate::v8_utils::v8_string(scope, "digest: data must be BufferSource");
                resolver.reject(scope, v8::Exception::type_error(scope, msg));
                return;
            }
        };

        let result = match algo.to_uppercase().replace("-", "").as_str() {
            "SHA1" => {
                let mut hasher = Sha1::new();
                hasher.update(&data);
                hasher.finalize().to_vec()
            }
            "SHA256" => {
                let mut hasher = Sha256::new();
                hasher.update(&data);
                hasher.finalize().to_vec()
            }
            "SHA384" => {
                let mut hasher = Sha384::new();
                hasher.update(&data);
                hasher.finalize().to_vec()
            }
            "SHA512" => {
                let mut hasher = Sha512::new();
                hasher.update(&data);
                hasher.finalize().to_vec()
            }
            _ => {
                let msg = crate::v8_utils::v8_string(
                    scope,
                    &format!("digest: unsupported algorithm '{}'", algo),
                );
                resolver.reject(scope, v8::Exception::error(scope, msg));
                return;
            }
        };

        resolve_with_array_buffer(scope, resolver, &result);
    }));
}

/// crypto.subtle.importKey(format, keyData, algorithm, extractable, usages) → Promise<CryptoKey>
/// Returns a simple object with {type, algorithm, __rawKey__} for use in sign/verify.
unsafe extern "C" fn subtle_import_key(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 3 {
            let msg = crate::v8_utils::v8_string(scope, "importKey requires at least 3 arguments");
            resolver.reject(scope, v8::Exception::type_error(scope, msg));
            return;
        }

        let format = args.get(0).to_rust_string_lossy(scope); // "raw", "spki", "pkcs8", "jwk"
        let algo_arg = args.get(2);
        let algo = get_algorithm_name(scope, algo_arg);
        let extractable = if args.length() >= 4 {
            args.get(3).boolean_value(scope)
        } else {
            false
        };
        let usages: Vec<String> = if args.length() >= 5 && args.get(4).is_array() {
            let arr: v8::Local<v8::Array> = unsafe { v8::Local::cast_unchecked(args.get(4)) };
            (0..arr.length())
                .filter_map(|i| {
                    arr.get_index(scope, i)
                        .map(|v| v.to_rust_string_lossy(scope))
                })
                .collect()
        } else {
            vec![]
        };

        let hash = get_hash_from_algo(scope, algo_arg);

        // Get curve for EC algorithms
        let curve_name = if algo_arg.is_object() {
            let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
            let curve_key = crate::v8_utils::v8_string(scope, "namedCurve");
            obj.get(scope, curve_key.into())
                .map(|v| v.to_rust_string_lossy(scope))
        } else {
            None
        };

        let algo_upper = algo.to_uppercase().replace("-", "");

        // Handle JWK format specially
        if format == "jwk" {
            import_key_jwk(
                scope,
                resolver,
                args.get(1),
                &algo,
                &hash,
                curve_name.as_deref(),
                extractable,
                usages,
            );
            return;
        }

        // Get key bytes for non-JWK formats
        let key_data = match extract_bytes(scope, args.get(1)) {
            Some(d) => d,
            None => {
                let msg =
                    crate::v8_utils::v8_string(scope, "importKey: keyData must be BufferSource");
                resolver.reject(scope, v8::Exception::type_error(scope, msg));
                return;
            }
        };

        match algo_upper.as_str() {
            // Symmetric algorithms: raw format only
            "HMAC" | "AESGCM" | "AESCBC" | "AESCTR" | "PBKDF2" | "HKDF" => {
                let meta = KeyMeta {
                    key_type: "secret".to_string(),
                    algo: algo.clone(),
                    hash: Some(hash),
                    curve: None,
                    modulus_length: None,
                    key_bytes_b64: b64_encode(&key_data),
                    extractable,
                    usages,
                };
                let key_obj = make_crypto_key(scope, &meta);
                resolver.resolve(scope, key_obj.into());
            }

            // RSA algorithms
            "RSAOAEP" | "RSAPSS" | "RSASSAPKCS1V15" => {
                let (key_type, key_bytes) = match format.as_str() {
                    "spki" => match rsa_impl::import_rsa_public_key_spki(&key_data) {
                        Ok(pub_key) => match rsa_impl::export_rsa_public_key_spki(&pub_key) {
                            Ok(der) => ("public".to_string(), der),
                            Err(e) => {
                                let msg = crate::v8_utils::v8_string(
                                    scope,
                                    &format!("importKey RSA spki: {}", e),
                                );
                                resolver.reject(scope, v8::Exception::error(scope, msg));
                                return;
                            }
                        },
                        Err(e) => {
                            let msg = crate::v8_utils::v8_string(
                                scope,
                                &format!("importKey RSA spki: {}", e),
                            );
                            resolver.reject(scope, v8::Exception::error(scope, msg));
                            return;
                        }
                    },
                    "pkcs8" => match rsa_impl::import_rsa_private_key_pkcs8(&key_data) {
                        Ok(priv_key) => match rsa_impl::export_rsa_private_key_pkcs8(&priv_key) {
                            Ok(der) => ("private".to_string(), der),
                            Err(e) => {
                                let msg = crate::v8_utils::v8_string(
                                    scope,
                                    &format!("importKey RSA pkcs8: {}", e),
                                );
                                resolver.reject(scope, v8::Exception::error(scope, msg));
                                return;
                            }
                        },
                        Err(e) => {
                            let msg = crate::v8_utils::v8_string(
                                scope,
                                &format!("importKey RSA pkcs8: {}", e),
                            );
                            resolver.reject(scope, v8::Exception::error(scope, msg));
                            return;
                        }
                    },
                    _ => {
                        let msg = crate::v8_utils::v8_string(
                            scope,
                            &format!("importKey RSA: unsupported format '{}'", format),
                        );
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };

                let meta = KeyMeta {
                    key_type,
                    algo: algo.clone(),
                    hash: Some(hash),
                    curve: None,
                    modulus_length: None,
                    key_bytes_b64: b64_encode(&key_bytes),
                    extractable,
                    usages,
                };
                let key_obj = make_crypto_key(scope, &meta);
                resolver.resolve(scope, key_obj.into());
            }

            // EC algorithms
            "ECDSA" | "ECDH" => {
                let curve = match curve_name.as_deref().and_then(EcCurve::from_name) {
                    Some(c) => c,
                    None => {
                        let msg = crate::v8_utils::v8_string(
                            scope,
                            "importKey EC: missing or unsupported namedCurve",
                        );
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };

                let (key_type, key_bytes) = match format.as_str() {
                    "raw" => {
                        // Raw public key (uncompressed SEC1 point)
                        match crate::crypto::ec_impl::import_ec_key_raw(&key_data, curve, "public")
                        {
                            Ok(k) => ("public".to_string(), k.to_raw_bytes()),
                            Err(e) => {
                                let msg = crate::v8_utils::v8_string(
                                    scope,
                                    &format!("importKey EC raw: {}", e),
                                );
                                resolver.reject(scope, v8::Exception::error(scope, msg));
                                return;
                            }
                        }
                    }
                    "spki" => {
                        match crate::crypto::ec_impl::import_ec_public_key_spki(&key_data, curve) {
                            Ok(k) => match k.to_spki_der() {
                                Ok(der) => ("public".to_string(), der),
                                Err(e) => {
                                    let msg = crate::v8_utils::v8_string(
                                        scope,
                                        &format!("importKey EC spki export: {}", e),
                                    );
                                    resolver.reject(scope, v8::Exception::error(scope, msg));
                                    return;
                                }
                            },
                            Err(e) => {
                                let msg = crate::v8_utils::v8_string(
                                    scope,
                                    &format!("importKey EC spki: {}", e),
                                );
                                resolver.reject(scope, v8::Exception::error(scope, msg));
                                return;
                            }
                        }
                    }
                    "pkcs8" => {
                        match crate::crypto::ec_impl::import_ec_private_key_pkcs8(&key_data, curve)
                        {
                            Ok(k) => match k.to_pkcs8_der() {
                                Ok(der) => ("private".to_string(), der),
                                Err(e) => {
                                    let msg = crate::v8_utils::v8_string(
                                        scope,
                                        &format!("importKey EC pkcs8 export: {}", e),
                                    );
                                    resolver.reject(scope, v8::Exception::error(scope, msg));
                                    return;
                                }
                            },
                            Err(e) => {
                                let msg = crate::v8_utils::v8_string(
                                    scope,
                                    &format!("importKey EC pkcs8: {}", e),
                                );
                                resolver.reject(scope, v8::Exception::error(scope, msg));
                                return;
                            }
                        }
                    }
                    _ => {
                        let msg = crate::v8_utils::v8_string(
                            scope,
                            &format!("importKey EC: unsupported format '{}'", format),
                        );
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };

                let meta = KeyMeta {
                    key_type,
                    algo: algo.clone(),
                    hash: Some(hash),
                    curve: curve_name,
                    modulus_length: None,
                    key_bytes_b64: b64_encode(&key_bytes),
                    extractable,
                    usages,
                };
                let key_obj = make_crypto_key(scope, &meta);
                resolver.resolve(scope, key_obj.into());
            }

            _ => {
                // Fallback: treat as raw symmetric key
                let meta = KeyMeta {
                    key_type: "secret".to_string(),
                    algo: algo.clone(),
                    hash: Some(hash),
                    curve: None,
                    modulus_length: None,
                    key_bytes_b64: b64_encode(&key_data),
                    extractable,
                    usages,
                };
                let key_obj = make_crypto_key(scope, &meta);
                resolver.resolve(scope, key_obj.into());
            }
        }
    }));
}

/// Import key from JWK format.
#[expect(
    clippy::too_many_arguments,
    reason = "JWK import needs explicit algorithm context"
)]
fn import_key_jwk(
    scope: &v8::PinScope<'_, '_>,
    resolver: v8::Local<v8::PromiseResolver>,
    jwk_val: v8::Local<v8::Value>,
    algo: &str,
    hash: &str,
    curve_name: Option<&str>,
    extractable: bool,
    usages: Vec<String>,
) {
    if !jwk_val.is_object() {
        let msg = crate::v8_utils::v8_string(scope, "importKey JWK: keyData must be an object");
        resolver.reject(scope, v8::Exception::type_error(scope, msg));
        return;
    }
    let jwk_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(jwk_val) };

    // Extract 'k' field for symmetric keys (base64url-encoded key bytes)
    let k_key = crate::v8_utils::v8_string(scope, "k");
    if let Some(k_val) = jwk_obj.get(scope, k_key.into()) {
        if k_val.is_string() {
            let k_b64 = k_val.to_rust_string_lossy(scope);
            // base64url → base64
            let k_b64_std = k_b64.replace('-', "+").replace('_', "/");
            if let Ok(key_bytes) = b64_decode(&k_b64_std) {
                let meta = KeyMeta {
                    key_type: "secret".to_string(),
                    algo: algo.to_string(),
                    hash: Some(hash.to_string()),
                    curve: None,
                    modulus_length: None,
                    key_bytes_b64: b64_encode(&key_bytes),
                    extractable,
                    usages,
                };
                let key_obj = make_crypto_key(scope, &meta);
                resolver.resolve(scope, key_obj.into());
                return;
            }
        }
    }

    // For RSA JWK: extract n, e (public) or n, e, d, p, q (private)
    // For EC JWK: extract x, y (public) or x, y, d (private)
    // Simplified: just create a placeholder key with the JWK JSON stored
    let json_str = {
        let json_key = crate::v8_utils::v8_string(scope, "JSON");
        let global = scope.get_current_context().global(scope);
        if let Some(json_obj) = global.get(scope, json_key.into()) {
            if json_obj.is_object() {
                let json_obj: v8::Local<v8::Object> =
                    unsafe { v8::Local::cast_unchecked(json_obj) };
                let stringify_key = crate::v8_utils::v8_string(scope, "stringify");
                if let Some(stringify_fn) = json_obj.get(scope, stringify_key.into()) {
                    if stringify_fn.is_function() {
                        let func: v8::Local<v8::Function> =
                            unsafe { v8::Local::cast_unchecked(stringify_fn) };
                        let undefined = v8::undefined(scope);
                        if let Some(result) = func.call(scope, undefined.into(), &[jwk_val]) {
                            result.to_rust_string_lossy(scope)
                        } else {
                            "{}".to_string()
                        }
                    } else {
                        "{}".to_string()
                    }
                } else {
                    "{}".to_string()
                }
            } else {
                "{}".to_string()
            }
        } else {
            "{}".to_string()
        }
    };

    // Store JWK JSON as key bytes (base64-encoded)
    let meta = KeyMeta {
        key_type: "secret".to_string(),
        algo: algo.to_string(),
        hash: Some(hash.to_string()),
        curve: curve_name.map(|s| s.to_string()),
        modulus_length: None,
        key_bytes_b64: b64_encode(json_str.as_bytes()),
        extractable,
        usages,
    };
    let key_obj = make_crypto_key(scope, &meta);
    resolver.resolve(scope, key_obj.into());
}

/// crypto.subtle.sign(algorithm, key, data) → Promise<ArrayBuffer>
unsafe extern "C" fn subtle_sign(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 3 {
            let msg = crate::v8_utils::v8_string(scope, "sign requires 3 arguments");
            resolver.reject(scope, v8::Exception::type_error(scope, msg));
            return;
        }

        let algo_arg = args.get(0);
        let algo = get_algorithm_name(scope, algo_arg);
        let hash_algo_from_arg = get_hash_from_algo(scope, algo_arg);
        let salt_length = get_salt_length(scope, algo_arg);

        let data = match extract_bytes(scope, args.get(2)) {
            Some(d) => d,
            None => {
                let msg = crate::v8_utils::v8_string(scope, "sign: data must be BufferSource");
                resolver.reject(scope, v8::Exception::type_error(scope, msg));
                return;
            }
        };

        let algo_upper = algo.to_uppercase().replace("-", "");

        // Try to get KeyMeta first (new path)
        if let Some(meta) = extract_key_meta(scope, args.get(1)) {
            // Use hash from key metadata if algo arg doesn't specify one explicitly
            let hash_algo = {
                // Check if algo_arg has an explicit hash field
                let has_explicit_hash = if algo_arg.is_object() {
                    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
                    let hash_key = crate::v8_utils::v8_string(scope, "hash");
                    obj.get(scope, hash_key.into())
                        .map(|v| !v.is_null_or_undefined())
                        .unwrap_or(false)
                } else {
                    false
                };

                if has_explicit_hash {
                    hash_algo_from_arg.clone()
                } else {
                    // Fall back to key's hash
                    meta.hash
                        .as_deref()
                        .unwrap_or(&hash_algo_from_arg)
                        .to_string()
                }
            };

            let result = match algo_upper.as_str() {
                "HMAC" => {
                    if let Ok(key_bytes) = b64_decode(&meta.key_bytes_b64) {
                        hmac_sign(&hash_algo, &key_bytes, &data)
                    } else {
                        None
                    }
                }
                "RSAPSS" => match meta_to_rsa_private(&meta) {
                    Ok(priv_key) => {
                        match rsa_impl::rsa_pss_sign(&priv_key, &data, &hash_algo, salt_length) {
                            Ok(sig) => Some(sig),
                            Err(e) => {
                                let msg = crate::v8_utils::v8_string(
                                    scope,
                                    &format!("RSA-PSS sign failed: {}", e),
                                );
                                resolver.reject(scope, v8::Exception::error(scope, msg));
                                return;
                            }
                        }
                    }
                    Err(e) => {
                        let msg = crate::v8_utils::v8_string(
                            scope,
                            &format!("RSA-PSS: key import failed: {}", e),
                        );
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                },
                "ECDSA" => match meta_to_ec_key(&meta) {
                    Ok(ec_key) => {
                        crate::crypto::ec_impl::ecdsa_sign(&ec_key, &data, &hash_algo).ok()
                    }
                    Err(_) => None,
                },
                _ => {
                    // Fallback to raw key bytes for HMAC
                    if let Ok(key_bytes) = b64_decode(&meta.key_bytes_b64) {
                        hmac_sign(&hash_algo, &key_bytes, &data)
                    } else {
                        None
                    }
                }
            };
            match result {
                Some(sig) => resolve_with_array_buffer(scope, resolver, &sig),
                None => {
                    let msg = crate::v8_utils::v8_string(
                        scope,
                        &format!("sign: operation failed for '{}'", algo),
                    );
                    resolver.reject(scope, v8::Exception::error(scope, msg));
                }
            }
            return;
        }

        // Legacy path: extract raw key bytes from __rawKey__
        let key_arg = args.get(1);
        let key_bytes = if key_arg.is_object() {
            let key_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(key_arg) };
            let raw_key = crate::v8_utils::v8_string(scope, "__rawKey__");
            key_obj
                .get(scope, raw_key.into())
                .and_then(|v| extract_bytes(scope, v))
        } else {
            None
        };

        let key_bytes = match key_bytes {
            Some(k) => k,
            None => {
                let msg = crate::v8_utils::v8_string(scope, "sign: invalid key");
                resolver.reject(scope, v8::Exception::error(scope, msg));
                return;
            }
        };

        let result = match algo_upper.as_str() {
            "HMAC" => hmac_sign(&hash_algo_from_arg, &key_bytes, &data),
            _ => {
                let msg = crate::v8_utils::v8_string(
                    scope,
                    &format!("sign: unsupported algorithm '{}'", algo),
                );
                resolver.reject(scope, v8::Exception::error(scope, msg));
                return;
            }
        };

        match result {
            Some(sig) => resolve_with_array_buffer(scope, resolver, &sig),
            None => {
                let msg = crate::v8_utils::v8_string(scope, "sign: operation failed");
                resolver.reject(scope, v8::Exception::error(scope, msg));
            }
        }
    }));
}

/// crypto.subtle.verify(algorithm, key, signature, data) → Promise<boolean>
unsafe extern "C" fn subtle_verify(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 4 {
            let msg = crate::v8_utils::v8_string(scope, "verify requires 4 arguments");
            resolver.reject(scope, v8::Exception::type_error(scope, msg));
            return;
        }

        let algo_arg = args.get(0);
        let algo = get_algorithm_name(scope, algo_arg);
        let hash_algo_from_arg = get_hash_from_algo(scope, algo_arg);
        let signature = extract_bytes(scope, args.get(2)).unwrap_or_default();
        let data = extract_bytes(scope, args.get(3)).unwrap_or_default();
        let algo_upper = algo.to_uppercase().replace("-", "");

        // Try KeyMeta path first
        if let Some(meta) = extract_key_meta(scope, args.get(1)) {
            // Use hash from key metadata if algo arg doesn't specify one explicitly
            let hash_algo = {
                let has_explicit_hash = if algo_arg.is_object() {
                    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
                    let hash_key = crate::v8_utils::v8_string(scope, "hash");
                    obj.get(scope, hash_key.into())
                        .map(|v| !v.is_null_or_undefined())
                        .unwrap_or(false)
                } else {
                    false
                };
                if has_explicit_hash {
                    hash_algo_from_arg.clone()
                } else {
                    meta.hash
                        .as_deref()
                        .unwrap_or(&hash_algo_from_arg)
                        .to_string()
                }
            };
            let valid = match algo_upper.as_str() {
                "HMAC" => {
                    if let Ok(key_bytes) = b64_decode(&meta.key_bytes_b64) {
                        hmac_verify(&hash_algo, &key_bytes, &data, &signature)
                    } else {
                        false
                    }
                }
                "RSAPSS" => match meta_to_rsa_public(&meta) {
                    Ok(pub_key) => {
                        rsa_impl::rsa_pss_verify(&pub_key, &data, &signature, &hash_algo)
                    }
                    Err(_) => false,
                },
                "ECDSA" => match meta_to_ec_key(&meta) {
                    Ok(ec_key) => crate::crypto::ec_impl::ecdsa_verify(&ec_key, &data, &signature),
                    Err(_) => false,
                },
                _ => {
                    if let Ok(key_bytes) = b64_decode(&meta.key_bytes_b64) {
                        hmac_verify(&hash_algo, &key_bytes, &data, &signature)
                    } else {
                        false
                    }
                }
            };
            resolver.resolve(scope, v8::Boolean::new(scope, valid).into());
            return;
        }

        // Legacy path
        let key_arg = args.get(1);
        let key_bytes = if key_arg.is_object() {
            let key_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(key_arg) };
            let raw_key = crate::v8_utils::v8_string(scope, "__rawKey__");
            key_obj
                .get(scope, raw_key.into())
                .and_then(|v| extract_bytes(scope, v))
        } else {
            None
        };

        let valid = match key_bytes {
            Some(k) => match algo_upper.as_str() {
                "HMAC" => hmac_verify(&hash_algo_from_arg, &k, &data, &signature),
                _ => false,
            },
            None => false,
        };
        resolver.resolve(scope, v8::Boolean::new(scope, valid).into());
    }));
}

/// HMAC sign with the specified hash algorithm.
fn hmac_sign(hash_algo: &str, key: &[u8], data: &[u8]) -> Option<Vec<u8>> {
    match hash_algo.to_uppercase().replace("-", "").as_str() {
        "SHA1" => {
            let mut mac = <Hmac<Sha1> as Mac>::new_from_slice(key).ok()?;
            mac.update(data);
            Some(mac.finalize().into_bytes().to_vec())
        }
        "SHA256" => {
            let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(key).ok()?;
            mac.update(data);
            Some(mac.finalize().into_bytes().to_vec())
        }
        "SHA384" => {
            let mut mac = <Hmac<Sha384> as Mac>::new_from_slice(key).ok()?;
            mac.update(data);
            Some(mac.finalize().into_bytes().to_vec())
        }
        "SHA512" => {
            let mut mac = <Hmac<Sha512> as Mac>::new_from_slice(key).ok()?;
            mac.update(data);
            Some(mac.finalize().into_bytes().to_vec())
        }
        _ => None,
    }
}

/// HMAC verify.
fn hmac_verify(hash_algo: &str, key: &[u8], data: &[u8], signature: &[u8]) -> bool {
    match hmac_sign(hash_algo, key, data) {
        Some(expected) => expected == signature,
        None => false,
    }
}

/// Get IV from algorithm object: {name: 'AES-GCM', iv: Uint8Array}
fn get_iv(scope: &v8::PinScope<'_, '_>, algo_arg: v8::Local<v8::Value>) -> Option<Vec<u8>> {
    if !algo_arg.is_object() {
        return None;
    }
    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
    let iv_key = v8::String::new(scope, "iv")?;
    let iv_val = obj.get(scope, iv_key.into())?;
    extract_bytes(scope, iv_val)
}

/// crypto.subtle.encrypt(algorithm, key, data) → Promise<ArrayBuffer>
unsafe extern "C" fn subtle_encrypt(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 3 {
            let msg = crate::v8_utils::v8_string(scope, "encrypt requires 3 arguments");
            resolver.reject(scope, v8::Exception::type_error(scope, msg));
            return;
        }

        let algo_arg = args.get(0);
        let algo = get_algorithm_name(scope, algo_arg);
        let iv = get_iv(scope, algo_arg);
        let algo_upper = algo.to_uppercase().replace("-", "");

        // Extract key bytes (for symmetric algorithms)
        let key_arg = args.get(1);
        let key_bytes_opt = if key_arg.is_object() {
            let key_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(key_arg) };
            let raw_key = crate::v8_utils::v8_string(scope, "__rawKey__");
            key_obj
                .get(scope, raw_key.into())
                .and_then(|v| extract_bytes(scope, v))
        } else {
            None
        };

        let plaintext = match extract_bytes(scope, args.get(2)) {
            Some(d) => d,
            None => {
                let msg = crate::v8_utils::v8_string(scope, "encrypt: data must be BufferSource");
                resolver.reject(scope, v8::Exception::type_error(scope, msg));
                return;
            }
        };

        let result = match algo_upper.as_str() {
            "AESGCM" => {
                let key_bytes = match key_bytes_opt {
                    Some(k) => k,
                    None => {
                        let msg = crate::v8_utils::v8_string(scope, "encrypt: invalid key");
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };
                let iv = match iv {
                    Some(iv) => iv,
                    None => {
                        let msg = crate::v8_utils::v8_string(scope, "encrypt: AES-GCM requires iv");
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };
                aes_gcm_encrypt(&key_bytes, &iv, &plaintext)
            }
            "AESCBC" => {
                let key_bytes = match key_bytes_opt {
                    Some(k) => k,
                    None => {
                        let msg = crate::v8_utils::v8_string(scope, "encrypt: invalid key");
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };
                let iv = match iv {
                    Some(iv) => iv,
                    None => {
                        let msg = crate::v8_utils::v8_string(scope, "encrypt: AES-CBC requires iv");
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };
                aes_cbc_encrypt(&key_bytes, &iv, &plaintext)
            }
            "AESCTR" => {
                let key_bytes = match key_bytes_opt {
                    Some(k) => k,
                    None => {
                        let msg = crate::v8_utils::v8_string(scope, "encrypt: invalid key");
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };
                let iv = match iv {
                    Some(iv) => iv,
                    None => {
                        let msg = crate::v8_utils::v8_string(scope, "encrypt: AES-CTR requires counter");
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };
                aes_ctr_encrypt(&key_bytes, &iv, &plaintext)
            }
            "RSAOAEP" => {
                if let Some(meta) = extract_key_meta(scope, args.get(1)) {
                    match meta_to_rsa_public(&meta) {
                        Ok(pub_key) => {
                            let hash = meta.hash.as_deref().unwrap_or("SHA-256");
                            rsa_impl::rsa_oaep_encrypt(&pub_key, &plaintext, hash)
                        }
                        Err(e) => Err(e),
                    }
                } else {
                    Err("RSA-OAEP encrypt: invalid key (missing __keyMeta__)".to_string())
                }
            }
            _ => {
                let msg = crate::v8_utils::v8_string(
                    scope,
                    &format!("encrypt: unsupported algorithm '{}'", algo),
                );
                resolver.reject(scope, v8::Exception::error(scope, msg));
                return;
            }
        };

        match result {
            Ok(ciphertext) => resolve_with_array_buffer(scope, resolver, &ciphertext),
            Err(e) => {
                let msg = crate::v8_utils::v8_string(scope, &format!("encrypt failed: {}", e));
                resolver.reject(scope, v8::Exception::error(scope, msg));
            }
        }
    }));
}

/// crypto.subtle.decrypt(algorithm, key, data) → Promise<ArrayBuffer>
unsafe extern "C" fn subtle_decrypt(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 3 {
            let msg = crate::v8_utils::v8_string(scope, "decrypt requires 3 arguments");
            resolver.reject(scope, v8::Exception::type_error(scope, msg));
            return;
        }

        let algo_arg = args.get(0);
        let algo = get_algorithm_name(scope, algo_arg);
        let iv = get_iv(scope, algo_arg);
        let algo_upper = algo.to_uppercase().replace("-", "");

        let key_arg = args.get(1);
        let key_bytes_opt = if key_arg.is_object() {
            let key_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(key_arg) };
            let raw_key = crate::v8_utils::v8_string(scope, "__rawKey__");
            key_obj
                .get(scope, raw_key.into())
                .and_then(|v| extract_bytes(scope, v))
        } else {
            None
        };

        let ciphertext = match extract_bytes(scope, args.get(2)) {
            Some(d) => d,
            None => {
                let msg = crate::v8_utils::v8_string(scope, "decrypt: data must be BufferSource");
                resolver.reject(scope, v8::Exception::type_error(scope, msg));
                return;
            }
        };

        let result = match algo_upper.as_str() {
            "AESGCM" => {
                let key_bytes = match key_bytes_opt {
                    Some(k) => k,
                    None => {
                        let msg = crate::v8_utils::v8_string(scope, "decrypt: invalid key");
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };
                let iv = match iv {
                    Some(iv) => iv,
                    None => {
                        let msg = crate::v8_utils::v8_string(scope, "decrypt: AES-GCM requires iv");
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };
                aes_gcm_decrypt(&key_bytes, &iv, &ciphertext)
            }
            "AESCBC" => {
                let key_bytes = match key_bytes_opt {
                    Some(k) => k,
                    None => {
                        let msg = crate::v8_utils::v8_string(scope, "decrypt: invalid key");
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };
                let iv = match iv {
                    Some(iv) => iv,
                    None => {
                        let msg = crate::v8_utils::v8_string(scope, "decrypt: AES-CBC requires iv");
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };
                aes_cbc_decrypt(&key_bytes, &iv, &ciphertext)
            }
            "AESCTR" => {
                let key_bytes = match key_bytes_opt {
                    Some(k) => k,
                    None => {
                        let msg = crate::v8_utils::v8_string(scope, "decrypt: invalid key");
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };
                let iv = match iv {
                    Some(iv) => iv,
                    None => {
                        let msg = crate::v8_utils::v8_string(scope, "decrypt: AES-CTR requires counter");
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };
                aes_ctr_decrypt(&key_bytes, &iv, &ciphertext)
            }
            "RSAOAEP" => {
                if let Some(meta) = extract_key_meta(scope, args.get(1)) {
                    match meta_to_rsa_private(&meta) {
                        Ok(priv_key) => {
                            let hash = meta.hash.as_deref().unwrap_or("SHA-256");
                            rsa_impl::rsa_oaep_decrypt(&priv_key, &ciphertext, hash)
                        }
                        Err(e) => Err(e),
                    }
                } else {
                    Err("RSA-OAEP decrypt: invalid key (missing __keyMeta__)".to_string())
                }
            }
            _ => {
                let msg = crate::v8_utils::v8_string(
                    scope,
                    &format!("decrypt: unsupported algorithm '{}'", algo),
                );
                resolver.reject(scope, v8::Exception::error(scope, msg));
                return;
            }
        };

        match result {
            Ok(plaintext) => resolve_with_array_buffer(scope, resolver, &plaintext),
            Err(e) => {
                let msg = crate::v8_utils::v8_string(scope, &format!("decrypt failed: {}", e));
                resolver.reject(scope, v8::Exception::error(scope, msg));
            }
        }
    }));
}

/// AES-GCM encrypt.
fn aes_gcm_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, String> {
    let nonce = GenericArray::from_slice(iv);
    match key.len() {
        16 => {
            let cipher = Aes128Gcm::new(GenericArray::from_slice(key));
            cipher
                .encrypt(nonce, plaintext)
                .map_err(|e| format!("{}", e))
        }
        32 => {
            let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
            cipher
                .encrypt(nonce, plaintext)
                .map_err(|e| format!("{}", e))
        }
        _ => Err(format!(
            "AES-GCM: unsupported key length {} (need 16 or 32)",
            key.len()
        )),
    }
}

/// AES-GCM decrypt.
fn aes_gcm_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    let nonce = GenericArray::from_slice(iv);
    match key.len() {
        16 => {
            let cipher = Aes128Gcm::new(GenericArray::from_slice(key));
            cipher
                .decrypt(nonce, ciphertext)
                .map_err(|e| format!("{}", e))
        }
        32 => {
            let cipher = Aes256Gcm::new(GenericArray::from_slice(key));
            cipher
                .decrypt(nonce, ciphertext)
                .map_err(|e| format!("{}", e))
        }
        _ => Err(format!(
            "AES-GCM: unsupported key length {} (need 16 or 32)",
            key.len()
        )),
    }
}

/// crypto.subtle.deriveBits(algorithm, baseKey, length) → Promise<ArrayBuffer>
/// Supports PBKDF2, HKDF, ECDH.
unsafe extern "C" fn subtle_derive_bits(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 3 {
            let msg = crate::v8_utils::v8_string(scope, "deriveBits requires 3 arguments");
            resolver.reject(scope, v8::Exception::type_error(scope, msg));
            return;
        }

        let algo_arg = args.get(0);
        let algo = get_algorithm_name(scope, algo_arg);
        let length_bits = args.get(2).number_value(scope).unwrap_or(256.0) as usize;
        let length_bytes = length_bits / 8;
        let algo_upper = algo.to_uppercase().replace("-", "");

        let result = match algo_upper.as_str() {
            "PBKDF2" => {
                // Extract PBKDF2 parameters
                let (salt, iterations, hash) = if algo_arg.is_object() {
                    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
                    let salt_key = crate::v8_utils::v8_string(scope, "salt");
                    let salt = obj
                        .get(scope, salt_key.into())
                        .and_then(|v| extract_bytes(scope, v))
                        .unwrap_or_default();
                    let iter_key = crate::v8_utils::v8_string(scope, "iterations");
                    let iterations = obj
                        .get(scope, iter_key.into())
                        .and_then(|v| v.number_value(scope))
                        .unwrap_or(1000.0) as u32;
                    let hash_key = crate::v8_utils::v8_string(scope, "hash");
                    let hash = obj
                        .get(scope, hash_key.into())
                        .map(|v| get_algorithm_name(scope, v))
                        .unwrap_or_else(|| "SHA-256".to_string());
                    (salt, iterations, hash)
                } else {
                    (vec![], 1000, "SHA-256".to_string())
                };

                let key_bytes = if let Some(meta) = extract_key_meta(scope, args.get(1)) {
                    b64_decode(&meta.key_bytes_b64).unwrap_or_default()
                } else {
                    let key_arg = args.get(1);
                    if key_arg.is_object() {
                        let key_obj: v8::Local<v8::Object> =
                            unsafe { v8::Local::cast_unchecked(key_arg) };
                        let raw_key = crate::v8_utils::v8_string(scope, "__rawKey__");
                        key_obj
                            .get(scope, raw_key.into())
                            .and_then(|v| extract_bytes(scope, v))
                            .unwrap_or_default()
                    } else {
                        vec![]
                    }
                };

                pbkdf2_derive(&key_bytes, &salt, iterations, &hash, length_bytes)
            }
            "HKDF" => {
                let (salt, info_bytes, hash) = if algo_arg.is_object() {
                    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
                    let salt_key = crate::v8_utils::v8_string(scope, "salt");
                    let salt = obj
                        .get(scope, salt_key.into())
                        .and_then(|v| extract_bytes(scope, v))
                        .unwrap_or_default();
                    let info_key = crate::v8_utils::v8_string(scope, "info");
                    let info_bytes = obj
                        .get(scope, info_key.into())
                        .and_then(|v| extract_bytes(scope, v))
                        .unwrap_or_default();
                    let hash_key = crate::v8_utils::v8_string(scope, "hash");
                    let hash = obj
                        .get(scope, hash_key.into())
                        .map(|v| get_algorithm_name(scope, v))
                        .unwrap_or_else(|| "SHA-256".to_string());
                    (salt, info_bytes, hash)
                } else {
                    (vec![], vec![], "SHA-256".to_string())
                };

                let key_bytes = if let Some(meta) = extract_key_meta(scope, args.get(1)) {
                    b64_decode(&meta.key_bytes_b64).unwrap_or_default()
                } else {
                    let key_arg = args.get(1);
                    if key_arg.is_object() {
                        let key_obj: v8::Local<v8::Object> =
                            unsafe { v8::Local::cast_unchecked(key_arg) };
                        let raw_key = crate::v8_utils::v8_string(scope, "__rawKey__");
                        key_obj
                            .get(scope, raw_key.into())
                            .and_then(|v| extract_bytes(scope, v))
                            .unwrap_or_default()
                    } else {
                        vec![]
                    }
                };

                hkdf_derive(&key_bytes, &salt, &info_bytes, &hash, length_bytes)
            }
            "ECDH" => {
                // ECDH: private key is args.get(1), public key is in algo_arg.public
                let priv_meta = extract_key_meta(scope, args.get(1));
                let pub_key_arg = if algo_arg.is_object() {
                    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
                    let pub_key_key = crate::v8_utils::v8_string(scope, "public");
                    obj.get(scope, pub_key_key.into())
                } else {
                    None
                };

                match (priv_meta, pub_key_arg) {
                    (Some(priv_m), Some(pub_arg)) => {
                        let pub_meta = extract_key_meta(scope, pub_arg);
                        match (
                            meta_to_ec_key(&priv_m),
                            pub_meta.as_ref().and_then(|m| meta_to_ec_key(m).ok()),
                        ) {
                            (Ok(priv_key), Some(pub_key)) => {
                                crate::crypto::ec_impl::ecdh_derive_bits(
                                    &priv_key,
                                    &pub_key,
                                    length_bits,
                                )
                            }
                            _ => Err("ECDH deriveBits: invalid keys".to_string()),
                        }
                    }
                    _ => Err("ECDH deriveBits: missing keys".to_string()),
                }
            }
            _ => {
                let msg = crate::v8_utils::v8_string(
                    scope,
                    &format!("deriveBits: unsupported algorithm '{}'", algo),
                );
                resolver.reject(scope, v8::Exception::error(scope, msg));
                return;
            }
        };

        match result {
            Ok(derived) => resolve_with_array_buffer(scope, resolver, &derived),
            Err(e) => {
                let msg = crate::v8_utils::v8_string(scope, &format!("deriveBits failed: {}", e));
                resolver.reject(scope, v8::Exception::error(scope, msg));
            }
        }
    }));
}

/// PBKDF2 key derivation.
fn pbkdf2_derive(
    password: &[u8],
    salt: &[u8],
    iterations: u32,
    hash: &str,
    length: usize,
) -> Result<Vec<u8>, String> {
    let mut output = vec![0u8; length];

    match hash.to_uppercase().replace("-", "").as_str() {
        "SHA256" => pbkdf2_sha256(password, salt, iterations, &mut output),
        "SHA1" => pbkdf2_sha1(password, salt, iterations, &mut output),
        _ => return Err(format!("PBKDF2: unsupported hash '{}'", hash)),
    }

    Ok(output)
}

fn pbkdf2_sha256(password: &[u8], salt: &[u8], iterations: u32, output: &mut [u8]) {
    let hlen = 32; // SHA-256 output size
    for (block_num, chunk) in (1_u32..).zip(output.chunks_mut(hlen)) {
        let mut salt_block = salt.to_vec();
        salt_block.extend_from_slice(&block_num.to_be_bytes());
        let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(password).expect("key");
        mac.update(&salt_block);
        let mut u = mac.finalize().into_bytes().to_vec();
        let mut result = u.clone();
        for _ in 1..iterations {
            let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(password).expect("key");
            mac.update(&u);
            u = mac.finalize().into_bytes().to_vec();
            for (r, b) in result.iter_mut().zip(u.iter()) {
                *r ^= *b;
            }
        }
        let copy_len = chunk.len().min(result.len());
        chunk[..copy_len].copy_from_slice(&result[..copy_len]);
    }
}

fn pbkdf2_sha1(password: &[u8], salt: &[u8], iterations: u32, output: &mut [u8]) {
    let hlen = 20; // SHA-1 output size
    for (block_num, chunk) in (1_u32..).zip(output.chunks_mut(hlen)) {
        let mut salt_block = salt.to_vec();
        salt_block.extend_from_slice(&block_num.to_be_bytes());
        let mut mac = <Hmac<Sha1> as Mac>::new_from_slice(password).expect("key");
        mac.update(&salt_block);
        let mut u = mac.finalize().into_bytes().to_vec();
        let mut result = u.clone();
        for _ in 1..iterations {
            let mut mac = <Hmac<Sha1> as Mac>::new_from_slice(password).expect("key");
            mac.update(&u);
            u = mac.finalize().into_bytes().to_vec();
            for (r, b) in result.iter_mut().zip(u.iter()) {
                *r ^= *b;
            }
        }
        let copy_len = chunk.len().min(result.len());
        chunk[..copy_len].copy_from_slice(&result[..copy_len]);
    }
}

/// HKDF key derivation.
fn hkdf_derive(
    ikm: &[u8],
    salt: &[u8],
    info: &[u8],
    hash: &str,
    length: usize,
) -> Result<Vec<u8>, String> {
    use hkdf::Hkdf;
    match hash.to_uppercase().replace("-", "").as_str() {
        "SHA256" => {
            let hk = Hkdf::<Sha256>::new(if salt.is_empty() { None } else { Some(salt) }, ikm);
            let mut okm = vec![0u8; length];
            hk.expand(info, &mut okm)
                .map_err(|e| format!("HKDF: {}", e))?;
            Ok(okm)
        }
        "SHA384" => {
            let hk = Hkdf::<Sha384>::new(if salt.is_empty() { None } else { Some(salt) }, ikm);
            let mut okm = vec![0u8; length];
            hk.expand(info, &mut okm)
                .map_err(|e| format!("HKDF: {}", e))?;
            Ok(okm)
        }
        "SHA512" => {
            let hk = Hkdf::<Sha512>::new(if salt.is_empty() { None } else { Some(salt) }, ikm);
            let mut okm = vec![0u8; length];
            hk.expand(info, &mut okm)
                .map_err(|e| format!("HKDF: {}", e))?;
            Ok(okm)
        }
        "SHA1" => {
            let hk = Hkdf::<Sha1>::new(if salt.is_empty() { None } else { Some(salt) }, ikm);
            let mut okm = vec![0u8; length];
            hk.expand(info, &mut okm)
                .map_err(|e| format!("HKDF: {}", e))?;
            Ok(okm)
        }
        _ => Err(format!("HKDF: unsupported hash '{}'", hash)),
    }
}

/// AES-CBC encrypt with PKCS7 padding.
fn aes_cbc_encrypt(key: &[u8], iv: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 16 && key.len() != 32 {
        return Err(format!(
            "AES-CBC: unsupported key length {} (need 16 or 32)",
            key.len()
        ));
    }
    if iv.len() != 16 {
        return Err("AES-CBC: iv must be 16 bytes".to_string());
    }

    // PKCS7 padding
    let block_size = 16;
    let pad_len = block_size - (plaintext.len() % block_size);
    let mut padded = plaintext.to_vec();
    padded.extend(std::iter::repeat(pad_len as u8).take(pad_len));

    if key.len() == 16 {
        type Aes128CbcEnc = CbcEncryptor<Aes128>;
        let cipher = Aes128CbcEnc::new_from_slices(key, iv).map_err(|e| format!("{}", e))?;
        let ciphertext =
            cipher.encrypt_padded_vec_mut::<cbc::cipher::block_padding::NoPadding>(&padded);
        Ok(ciphertext)
    } else {
        // 256-bit key
        type Aes256CbcEnc = CbcEncryptor<aes::Aes256>;
        let cipher = Aes256CbcEnc::new_from_slices(key, iv).map_err(|e| format!("{}", e))?;
        let ciphertext =
            cipher.encrypt_padded_vec_mut::<cbc::cipher::block_padding::NoPadding>(&padded);
        Ok(ciphertext)
    }
}

/// AES-CBC decrypt with PKCS7 unpadding.
fn aes_cbc_decrypt(key: &[u8], iv: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 16 && key.len() != 32 {
        return Err(format!(
            "AES-CBC: unsupported key length {} (need 16 or 32)",
            key.len()
        ));
    }
    if iv.len() != 16 {
        return Err("AES-CBC: iv must be 16 bytes".to_string());
    }
    if ciphertext.is_empty() || ciphertext.len() % 16 != 0 {
        return Err("AES-CBC: ciphertext length must be multiple of 16".to_string());
    }

    let plaintext = if key.len() == 16 {
        type Aes128CbcDec = CbcDecryptor<Aes128>;
        let cipher = Aes128CbcDec::new_from_slices(key, iv).map_err(|e| format!("{}", e))?;
        cipher
            .decrypt_padded_vec_mut::<cbc::cipher::block_padding::NoPadding>(ciphertext)
            .map_err(|e| format!("{}", e))?
    } else {
        type Aes256CbcDec = CbcDecryptor<aes::Aes256>;
        let cipher = Aes256CbcDec::new_from_slices(key, iv).map_err(|e| format!("{}", e))?;
        cipher
            .decrypt_padded_vec_mut::<cbc::cipher::block_padding::NoPadding>(ciphertext)
            .map_err(|e| format!("{}", e))?
    };

    // Remove PKCS7 padding
    if let Some(&pad_byte) = plaintext.last() {
        let pad_len = pad_byte as usize;
        if pad_len > 0 && pad_len <= 16 && plaintext.len() >= pad_len {
            let unpadded = &plaintext[..plaintext.len() - pad_len];
            return Ok(unpadded.to_vec());
        }
    }
    Ok(plaintext)
}

/// AES-CTR encrypt (stream cipher, no padding needed).
fn aes_ctr_encrypt(key: &[u8], counter: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, String> {
    if key.len() != 16 && key.len() != 32 {
        return Err(format!(
            "AES-CTR: unsupported key length {} (need 16 or 32)",
            key.len()
        ));
    }
    if counter.len() != 16 {
        return Err("AES-CTR: counter must be 16 bytes".to_string());
    }
    let mut buf = plaintext.to_vec();
    match key.len() {
        16 => {
            let mut cipher = Ctr32BE::<Aes128>::new(key.into(), counter.into());
            cipher.apply_keystream(&mut buf);
        }
        32 => {
            let mut cipher = Ctr32BE::<aes::Aes256>::new(key.into(), counter.into());
            cipher.apply_keystream(&mut buf);
        }
        _ => unreachable!(),
    }
    Ok(buf)
}

/// AES-CTR decrypt (same as encrypt for stream cipher).
fn aes_ctr_decrypt(key: &[u8], counter: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>, String> {
    aes_ctr_encrypt(key, counter, ciphertext)
}

/// crypto.subtle.generateKey(algorithm, extractable, usages) → Promise<CryptoKey | CryptoKeyPair>
unsafe extern "C" fn subtle_generate_key(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 1 {
            let msg = crate::v8_utils::v8_string(scope, "generateKey requires at least 1 argument");
            resolver.reject(scope, v8::Exception::type_error(scope, msg));
            return;
        }

        let algo_arg = args.get(0);
        let algo = get_algorithm_name(scope, algo_arg);
        let extractable = if args.length() >= 2 {
            args.get(1).boolean_value(scope)
        } else {
            false
        };
        let usages: Vec<String> = if args.length() >= 3 && args.get(2).is_array() {
            let arr: v8::Local<v8::Array> = unsafe { v8::Local::cast_unchecked(args.get(2)) };
            (0..arr.length())
                .filter_map(|i| {
                    arr.get_index(scope, i)
                        .map(|v| v.to_rust_string_lossy(scope))
                })
                .collect()
        } else {
            vec![]
        };

        let hash = get_hash_from_algo(scope, algo_arg);
        let algo_upper = algo.to_uppercase().replace("-", "");

        match algo_upper.as_str() {
            // Symmetric key generation
            "HMAC" | "AESGCM" | "AESCBC" | "AESCTR" => {
                let key_length = if algo_arg.is_object() {
                    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
                    let len_key = crate::v8_utils::v8_string(scope, "length");
                    obj.get(scope, len_key.into())
                        .and_then(|v| v.number_value(scope))
                        .unwrap_or(256.0) as usize
                } else {
                    256
                };

                let byte_len = key_length / 8;
                let mut key_data = vec![0u8; byte_len];
                crate::crypto::random::fill_random_bytes(&mut key_data);

                let meta = KeyMeta {
                    key_type: "secret".to_string(),
                    algo: algo.clone(),
                    hash: Some(hash),
                    curve: None,
                    modulus_length: None,
                    key_bytes_b64: b64_encode(&key_data),
                    extractable,
                    usages,
                };
                let key_obj = make_crypto_key(scope, &meta);
                resolver.resolve(scope, key_obj.into());
            }

            // RSA key pair generation
            "RSAOAEP" | "RSAPSS" | "RSASSAPKCS1V15" => {
                let modulus_length = if algo_arg.is_object() {
                    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
                    let ml_key = crate::v8_utils::v8_string(scope, "modulusLength");
                    obj.get(scope, ml_key.into())
                        .and_then(|v| v.number_value(scope))
                        .unwrap_or(2048.0) as usize
                } else {
                    2048
                };

                match rsa_impl::rsa_generate_key(modulus_length) {
                    Ok((pub_key, priv_key)) => {
                        let pub_der = match rsa_impl::export_rsa_public_key_spki(&pub_key) {
                            Ok(d) => d,
                            Err(e) => {
                                let msg = crate::v8_utils::v8_string(
                                    scope,
                                    &format!("generateKey RSA: {}", e),
                                );
                                resolver.reject(scope, v8::Exception::error(scope, msg));
                                return;
                            }
                        };
                        let priv_der = match rsa_impl::export_rsa_private_key_pkcs8(&priv_key) {
                            Ok(d) => d,
                            Err(e) => {
                                let msg = crate::v8_utils::v8_string(
                                    scope,
                                    &format!("generateKey RSA: {}", e),
                                );
                                resolver.reject(scope, v8::Exception::error(scope, msg));
                                return;
                            }
                        };

                        let pub_meta = KeyMeta {
                            key_type: "public".to_string(),
                            algo: algo.clone(),
                            hash: Some(hash.clone()),
                            curve: None,
                            modulus_length: Some(modulus_length),
                            key_bytes_b64: b64_encode(&pub_der),
                            extractable,
                            usages: usages
                                .iter()
                                .filter(|u| matches!(u.as_str(), "encrypt" | "verify" | "wrapKey"))
                                .cloned()
                                .collect(),
                        };
                        let priv_meta = KeyMeta {
                            key_type: "private".to_string(),
                            algo: algo.clone(),
                            hash: Some(hash),
                            curve: None,
                            modulus_length: Some(modulus_length),
                            key_bytes_b64: b64_encode(&priv_der),
                            extractable,
                            usages: usages
                                .iter()
                                .filter(|u| matches!(u.as_str(), "decrypt" | "sign" | "unwrapKey"))
                                .cloned()
                                .collect(),
                        };

                        let pub_obj = make_crypto_key(scope, &pub_meta);
                        let priv_obj = make_crypto_key(scope, &priv_meta);

                        // Return CryptoKeyPair {publicKey, privateKey}
                        let pair_obj = v8::Object::new(scope);
                        let pub_key_str = crate::v8_utils::v8_string(scope, "publicKey");
                        let priv_key_str = crate::v8_utils::v8_string(scope, "privateKey");
                        pair_obj.set(scope, pub_key_str.into(), pub_obj.into());
                        pair_obj.set(scope, priv_key_str.into(), priv_obj.into());
                        resolver.resolve(scope, pair_obj.into());
                    }
                    Err(e) => {
                        let msg = crate::v8_utils::v8_string(
                            scope,
                            &format!("generateKey RSA failed: {}", e),
                        );
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                    }
                }
            }

            // EC key pair generation
            "ECDSA" | "ECDH" => {
                let curve_name = if algo_arg.is_object() {
                    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
                    let curve_key = crate::v8_utils::v8_string(scope, "namedCurve");
                    obj.get(scope, curve_key.into())
                        .map(|v| v.to_rust_string_lossy(scope))
                } else {
                    None
                };

                let curve = match curve_name.as_deref().and_then(EcCurve::from_name) {
                    Some(c) => c,
                    None => {
                        let msg = crate::v8_utils::v8_string(
                            scope,
                            "generateKey EC: missing or unsupported namedCurve",
                        );
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                        return;
                    }
                };

                match crate::crypto::ec_impl::ecdsa_generate_key(curve) {
                    Ok((priv_key, pub_key)) => {
                        let priv_der = match priv_key.to_pkcs8_der() {
                            Ok(d) => d,
                            Err(e) => {
                                let msg = crate::v8_utils::v8_string(
                                    scope,
                                    &format!("generateKey EC: {}", e),
                                );
                                resolver.reject(scope, v8::Exception::error(scope, msg));
                                return;
                            }
                        };
                        let pub_der = match pub_key.to_spki_der() {
                            Ok(d) => d,
                            Err(e) => {
                                let msg = crate::v8_utils::v8_string(
                                    scope,
                                    &format!("generateKey EC: {}", e),
                                );
                                resolver.reject(scope, v8::Exception::error(scope, msg));
                                return;
                            }
                        };

                        let pub_meta = KeyMeta {
                            key_type: "public".to_string(),
                            algo: algo.clone(),
                            hash: Some(hash.clone()),
                            curve: curve_name.clone(),
                            modulus_length: None,
                            key_bytes_b64: b64_encode(&pub_der),
                            extractable,
                            usages: usages
                                .iter()
                                .filter(|u| matches!(u.as_str(), "verify"))
                                .cloned()
                                .collect(),
                        };
                        let priv_meta = KeyMeta {
                            key_type: "private".to_string(),
                            algo: algo.clone(),
                            hash: Some(hash),
                            curve: curve_name,
                            modulus_length: None,
                            key_bytes_b64: b64_encode(&priv_der),
                            extractable,
                            usages: usages
                                .iter()
                                .filter(|u| {
                                    matches!(u.as_str(), "sign" | "deriveKey" | "deriveBits")
                                })
                                .cloned()
                                .collect(),
                        };

                        let pub_obj = make_crypto_key(scope, &pub_meta);
                        let priv_obj = make_crypto_key(scope, &priv_meta);

                        let pair_obj = v8::Object::new(scope);
                        let pub_key_str = crate::v8_utils::v8_string(scope, "publicKey");
                        let priv_key_str = crate::v8_utils::v8_string(scope, "privateKey");
                        pair_obj.set(scope, pub_key_str.into(), pub_obj.into());
                        pair_obj.set(scope, priv_key_str.into(), priv_obj.into());
                        resolver.resolve(scope, pair_obj.into());
                    }
                    Err(e) => {
                        let msg = crate::v8_utils::v8_string(
                            scope,
                            &format!("generateKey EC failed: {}", e),
                        );
                        resolver.reject(scope, v8::Exception::error(scope, msg));
                    }
                }
            }

            _ => {
                // Fallback: generate random symmetric key
                let key_length = 256usize;
                let mut key_data = vec![0u8; key_length / 8];
                crate::crypto::random::fill_random_bytes(&mut key_data);
                let meta = KeyMeta {
                    key_type: "secret".to_string(),
                    algo: algo.clone(),
                    hash: Some(hash),
                    curve: None,
                    modulus_length: None,
                    key_bytes_b64: b64_encode(&key_data),
                    extractable,
                    usages,
                };
                let key_obj = make_crypto_key(scope, &meta);
                resolver.resolve(scope, key_obj.into());
            }
        }
    }));
}

/// crypto.subtle.exportKey(format, key) → Promise<ArrayBuffer | JsonWebKey>
unsafe extern "C" fn subtle_export_key(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 2 {
            let msg = crate::v8_utils::v8_string(scope, "exportKey requires 2 arguments");
            resolver.reject(scope, v8::Exception::type_error(scope, msg));
            return;
        }

        let format = args.get(0).to_rust_string_lossy(scope);

        // Try KeyMeta path
        if let Some(meta) = extract_key_meta(scope, args.get(1)) {
            if !meta.extractable {
                let msg = crate::v8_utils::v8_string(scope, "exportKey: key is not extractable");
                resolver.reject(scope, v8::Exception::error(scope, msg));
                return;
            }

            let algo_upper = meta.algo.to_uppercase().replace("-", "");

            match format.as_str() {
                "raw" => {
                    // For symmetric keys: return raw bytes
                    // For EC public keys: return uncompressed point
                    match algo_upper.as_str() {
                        "ECDSA" | "ECDH" if meta.key_type == "public" => {
                            match meta_to_ec_key(&meta) {
                                Ok(ec_key) => resolve_with_array_buffer(
                                    scope,
                                    resolver,
                                    &ec_key.to_raw_bytes(),
                                ),
                                Err(e) => {
                                    let msg = crate::v8_utils::v8_string(
                                        scope,
                                        &format!("exportKey EC raw: {}", e),
                                    );
                                    resolver.reject(scope, v8::Exception::error(scope, msg));
                                }
                            }
                        }
                        _ => {
                            // Symmetric: return raw key bytes
                            match b64_decode(&meta.key_bytes_b64) {
                                Ok(bytes) => resolve_with_array_buffer(scope, resolver, &bytes),
                                Err(e) => {
                                    let msg = crate::v8_utils::v8_string(
                                        scope,
                                        &format!("exportKey raw: {}", e),
                                    );
                                    resolver.reject(scope, v8::Exception::error(scope, msg));
                                }
                            }
                        }
                    }
                }
                "spki" => {
                    // Public key in SPKI DER format
                    let result = match algo_upper.as_str() {
                        "RSAOAEP" | "RSAPSS" | "RSASSAPKCS1V15" => meta_to_rsa_public(&meta)
                            .and_then(|k| rsa_impl::export_rsa_public_key_spki(&k)),
                        "ECDSA" | "ECDH" => meta_to_ec_key(&meta).and_then(|k| k.to_spki_der()),
                        _ => Err(format!(
                            "exportKey spki: unsupported algorithm '{}'",
                            meta.algo
                        )),
                    };
                    match result {
                        Ok(der) => resolve_with_array_buffer(scope, resolver, &der),
                        Err(e) => {
                            let msg = crate::v8_utils::v8_string(
                                scope,
                                &format!("exportKey spki: {}", e),
                            );
                            resolver.reject(scope, v8::Exception::error(scope, msg));
                        }
                    }
                }
                "pkcs8" => {
                    // Private key in PKCS8 DER format
                    let result = match algo_upper.as_str() {
                        "RSAOAEP" | "RSAPSS" | "RSASSAPKCS1V15" => meta_to_rsa_private(&meta)
                            .and_then(|k| rsa_impl::export_rsa_private_key_pkcs8(&k)),
                        "ECDSA" | "ECDH" => meta_to_ec_key(&meta).and_then(|k| k.to_pkcs8_der()),
                        _ => Err(format!(
                            "exportKey pkcs8: unsupported algorithm '{}'",
                            meta.algo
                        )),
                    };
                    match result {
                        Ok(der) => resolve_with_array_buffer(scope, resolver, &der),
                        Err(e) => {
                            let msg = crate::v8_utils::v8_string(
                                scope,
                                &format!("exportKey pkcs8: {}", e),
                            );
                            resolver.reject(scope, v8::Exception::error(scope, msg));
                        }
                    }
                }
                "jwk" => {
                    // For symmetric keys: return {kty: "oct", k: base64url(key_bytes)}
                    match b64_decode(&meta.key_bytes_b64) {
                        Ok(bytes) => {
                            // base64url encode
                            let k_b64url = b64_encode(&bytes)
                                .replace('+', "-")
                                .replace('/', "_")
                                .trim_end_matches('=')
                                .to_string();
                            let jwk_json = format!(
                                r#"{{"kty":"oct","k":"{}","alg":"{}","ext":{}}}"#,
                                k_b64url, meta.algo, meta.extractable
                            );
                            if let Some(s) = v8::String::new(scope, &jwk_json) {
                                // Parse as JSON object
                                let global = scope.get_current_context().global(scope);
                                let json_key = crate::v8_utils::v8_string(scope, "JSON");
                                if let Some(json_obj) = global.get(scope, json_key.into()) {
                                    if json_obj.is_object() {
                                        let json_obj: v8::Local<v8::Object> =
                                            unsafe { v8::Local::cast_unchecked(json_obj) };
                                        let parse_key = crate::v8_utils::v8_string(scope, "parse");
                                        if let Some(parse_fn) =
                                            json_obj.get(scope, parse_key.into())
                                        {
                                            if parse_fn.is_function() {
                                                let func: v8::Local<v8::Function> =
                                                    unsafe { v8::Local::cast_unchecked(parse_fn) };
                                                let undefined = v8::undefined(scope);
                                                if let Some(result) =
                                                    func.call(scope, undefined.into(), &[s.into()])
                                                {
                                                    resolver.resolve(scope, result);
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                                // Fallback: return as string
                                resolver.resolve(scope, s.into());
                            }
                        }
                        Err(e) => {
                            let msg =
                                crate::v8_utils::v8_string(scope, &format!("exportKey jwk: {}", e));
                            resolver.reject(scope, v8::Exception::error(scope, msg));
                        }
                    }
                }
                _ => {
                    let msg = crate::v8_utils::v8_string(
                        scope,
                        &format!("exportKey: unsupported format '{}'", format),
                    );
                    resolver.reject(scope, v8::Exception::error(scope, msg));
                }
            }
            return;
        }

        // Legacy path: return raw bytes from __rawKey__
        let key_arg = args.get(1);
        let key_bytes = if key_arg.is_object() {
            let key_obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(key_arg) };
            let raw_key = crate::v8_utils::v8_string(scope, "__rawKey__");
            key_obj
                .get(scope, raw_key.into())
                .and_then(|v| extract_bytes(scope, v))
        } else {
            None
        };

        match key_bytes {
            Some(bytes) => resolve_with_array_buffer(scope, resolver, &bytes),
            None => {
                let msg = crate::v8_utils::v8_string(scope, "exportKey: invalid key");
                resolver.reject(scope, v8::Exception::error(scope, msg));
            }
        }
    }));
}

/// crypto.subtle.deriveKey(algorithm, baseKey, derivedKeyAlgorithm, extractable, usages) → Promise<CryptoKey>
unsafe extern "C" fn subtle_derive_key(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 5 {
            let msg = crate::v8_utils::v8_string(scope, "deriveKey requires 5 arguments");
            resolver.reject(scope, v8::Exception::type_error(scope, msg));
            return;
        }

        let algo_arg = args.get(0);
        let algo = get_algorithm_name(scope, algo_arg);
        let derived_key_algo_arg = args.get(2);
        let derived_key_algo = get_algorithm_name(scope, derived_key_algo_arg);
        let extractable = args.get(3).boolean_value(scope);
        let usages: Vec<String> = if args.get(4).is_array() {
            let arr: v8::Local<v8::Array> = unsafe { v8::Local::cast_unchecked(args.get(4)) };
            (0..arr.length())
                .filter_map(|i| {
                    arr.get_index(scope, i)
                        .map(|v| v.to_rust_string_lossy(scope))
                })
                .collect()
        } else {
            vec![]
        };

        // Get derived key length
        let derived_key_length = if derived_key_algo_arg.is_object() {
            let obj: v8::Local<v8::Object> =
                unsafe { v8::Local::cast_unchecked(derived_key_algo_arg) };
            let len_key = crate::v8_utils::v8_string(scope, "length");
            obj.get(scope, len_key.into())
                .and_then(|v| v.number_value(scope))
                .unwrap_or(256.0) as usize
        } else {
            256
        };

        let algo_upper = algo.to_uppercase().replace("-", "");

        // Derive bits first, then wrap as key
        let derived_bits = match algo_upper.as_str() {
            "PBKDF2" => {
                // Extract parameters
                let (salt, iterations, hash) = if algo_arg.is_object() {
                    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
                    let salt_key = crate::v8_utils::v8_string(scope, "salt");
                    let salt = obj
                        .get(scope, salt_key.into())
                        .and_then(|v| extract_bytes(scope, v))
                        .unwrap_or_default();
                    let iter_key = crate::v8_utils::v8_string(scope, "iterations");
                    let iterations = obj
                        .get(scope, iter_key.into())
                        .and_then(|v| v.number_value(scope))
                        .unwrap_or(1000.0) as u32;
                    let hash_key = crate::v8_utils::v8_string(scope, "hash");
                    let hash = obj
                        .get(scope, hash_key.into())
                        .map(|v| get_algorithm_name(scope, v))
                        .unwrap_or_else(|| "SHA-256".to_string());
                    (salt, iterations, hash)
                } else {
                    (vec![], 1000, "SHA-256".to_string())
                };

                let key_bytes = if let Some(meta) = extract_key_meta(scope, args.get(1)) {
                    b64_decode(&meta.key_bytes_b64).unwrap_or_default()
                } else {
                    let key_arg = args.get(1);
                    if key_arg.is_object() {
                        let key_obj: v8::Local<v8::Object> =
                            unsafe { v8::Local::cast_unchecked(key_arg) };
                        let raw_key = crate::v8_utils::v8_string(scope, "__rawKey__");
                        key_obj
                            .get(scope, raw_key.into())
                            .and_then(|v| extract_bytes(scope, v))
                            .unwrap_or_default()
                    } else {
                        vec![]
                    }
                };

                pbkdf2_derive(&key_bytes, &salt, iterations, &hash, derived_key_length / 8)
            }
            "HKDF" => {
                let (salt, info_bytes, hash) = if algo_arg.is_object() {
                    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
                    let salt_key = crate::v8_utils::v8_string(scope, "salt");
                    let salt = obj
                        .get(scope, salt_key.into())
                        .and_then(|v| extract_bytes(scope, v))
                        .unwrap_or_default();
                    let info_key = crate::v8_utils::v8_string(scope, "info");
                    let info_bytes = obj
                        .get(scope, info_key.into())
                        .and_then(|v| extract_bytes(scope, v))
                        .unwrap_or_default();
                    let hash_key = crate::v8_utils::v8_string(scope, "hash");
                    let hash = obj
                        .get(scope, hash_key.into())
                        .map(|v| get_algorithm_name(scope, v))
                        .unwrap_or_else(|| "SHA-256".to_string());
                    (salt, info_bytes, hash)
                } else {
                    (vec![], vec![], "SHA-256".to_string())
                };

                let key_bytes = if let Some(meta) = extract_key_meta(scope, args.get(1)) {
                    b64_decode(&meta.key_bytes_b64).unwrap_or_default()
                } else {
                    let key_arg = args.get(1);
                    if key_arg.is_object() {
                        let key_obj: v8::Local<v8::Object> =
                            unsafe { v8::Local::cast_unchecked(key_arg) };
                        let raw_key = crate::v8_utils::v8_string(scope, "__rawKey__");
                        key_obj
                            .get(scope, raw_key.into())
                            .and_then(|v| extract_bytes(scope, v))
                            .unwrap_or_default()
                    } else {
                        vec![]
                    }
                };

                hkdf_derive(
                    &key_bytes,
                    &salt,
                    &info_bytes,
                    &hash,
                    derived_key_length / 8,
                )
            }
            "ECDH" => {
                // ECDH key agreement
                let priv_meta = extract_key_meta(scope, args.get(1));
                let pub_key_arg = if algo_arg.is_object() {
                    let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(algo_arg) };
                    let pub_key_key = crate::v8_utils::v8_string(scope, "public");
                    obj.get(scope, pub_key_key.into())
                } else {
                    None
                };

                match (priv_meta, pub_key_arg) {
                    (Some(priv_m), Some(pub_arg)) => {
                        let pub_meta = extract_key_meta(scope, pub_arg);
                        match (
                            meta_to_ec_key(&priv_m),
                            pub_meta.as_ref().and_then(|m| meta_to_ec_key(m).ok()),
                        ) {
                            (Ok(priv_key), Some(pub_key)) => {
                                crate::crypto::ec_impl::ecdh_derive_bits(
                                    &priv_key,
                                    &pub_key,
                                    derived_key_length,
                                )
                            }
                            _ => Err("ECDH deriveKey: invalid keys".to_string()),
                        }
                    }
                    _ => Err("ECDH deriveKey: missing keys".to_string()),
                }
            }
            _ => Err(format!("deriveKey: unsupported algorithm '{}'", algo)),
        };

        match derived_bits {
            Ok(bits) => {
                let meta = KeyMeta {
                    key_type: "secret".to_string(),
                    algo: derived_key_algo,
                    hash: None,
                    curve: None,
                    modulus_length: None,
                    key_bytes_b64: b64_encode(&bits),
                    extractable,
                    usages,
                };
                let key_obj = make_crypto_key(scope, &meta);
                resolver.resolve(scope, key_obj.into());
            }
            Err(e) => {
                let msg = crate::v8_utils::v8_string(scope, &format!("deriveKey failed: {}", e));
                resolver.reject(scope, v8::Exception::error(scope, msg));
            }
        }
    }));
}

/// crypto.subtle.wrapKey(format, key, wrappingKey, wrapAlgorithm) → Promise<ArrayBuffer>
unsafe extern "C" fn subtle_wrap_key(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 4 {
            let msg = crate::v8_utils::v8_string(scope, "wrapKey requires 4 arguments");
            resolver.reject(scope, v8::Exception::type_error(scope, msg));
            return;
        }

        // Export the key to raw bytes, then encrypt with wrapping key
        let key_meta = extract_key_meta(scope, args.get(1));
        let key_bytes = match key_meta {
            Some(ref meta) => b64_decode(&meta.key_bytes_b64).unwrap_or_default(),
            None => {
                let key_arg = args.get(1);
                if key_arg.is_object() {
                    let key_obj: v8::Local<v8::Object> =
                        unsafe { v8::Local::cast_unchecked(key_arg) };
                    let raw_key = crate::v8_utils::v8_string(scope, "__rawKey__");
                    key_obj
                        .get(scope, raw_key.into())
                        .and_then(|v| extract_bytes(scope, v))
                        .unwrap_or_default()
                } else {
                    vec![]
                }
            }
        };

        let wrap_algo_arg = args.get(3);
        let wrap_algo = get_algorithm_name(scope, wrap_algo_arg);
        let iv = get_iv(scope, wrap_algo_arg);
        let wrap_key_meta = extract_key_meta(scope, args.get(2));

        let wrap_key_bytes = match wrap_key_meta {
            Some(ref meta) => b64_decode(&meta.key_bytes_b64).unwrap_or_default(),
            None => {
                let wk_arg = args.get(2);
                if wk_arg.is_object() {
                    let wk_obj: v8::Local<v8::Object> =
                        unsafe { v8::Local::cast_unchecked(wk_arg) };
                    let raw_key = crate::v8_utils::v8_string(scope, "__rawKey__");
                    wk_obj
                        .get(scope, raw_key.into())
                        .and_then(|v| extract_bytes(scope, v))
                        .unwrap_or_default()
                } else {
                    vec![]
                }
            }
        };

        let result = match wrap_algo.to_uppercase().replace("-", "").as_str() {
            "AESGCM" => {
                let iv = iv.unwrap_or_else(|| vec![0u8; 12]);
                aes_gcm_encrypt(&wrap_key_bytes, &iv, &key_bytes)
            }
            "AESCBC" => {
                let iv = iv.unwrap_or_else(|| vec![0u8; 16]);
                aes_cbc_encrypt(&wrap_key_bytes, &iv, &key_bytes)
            }
            _ => Err(format!(
                "wrapKey: unsupported wrap algorithm '{}'",
                wrap_algo
            )),
        };

        match result {
            Ok(wrapped) => resolve_with_array_buffer(scope, resolver, &wrapped),
            Err(e) => {
                let msg = crate::v8_utils::v8_string(scope, &format!("wrapKey failed: {}", e));
                resolver.reject(scope, v8::Exception::error(scope, msg));
            }
        }
    }));
}

/// crypto.subtle.unwrapKey(format, wrappedKey, unwrappingKey, unwrapAlgorithm, unwrappedKeyAlgorithm, extractable, usages) → Promise<CryptoKey>
unsafe extern "C" fn subtle_unwrap_key(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        let resolver = crate::v8_utils::v8_resolver(scope);
        let promise = resolver.get_promise(scope);
        rv.set(promise.into());

        if args.length() < 7 {
            let msg = crate::v8_utils::v8_string(scope, "unwrapKey requires 7 arguments");
            resolver.reject(scope, v8::Exception::type_error(scope, msg));
            return;
        }

        let wrapped_key_bytes = match extract_bytes(scope, args.get(1)) {
            Some(b) => b,
            None => {
                let msg =
                    crate::v8_utils::v8_string(scope, "unwrapKey: wrappedKey must be BufferSource");
                resolver.reject(scope, v8::Exception::type_error(scope, msg));
                return;
            }
        };

        let unwrap_algo_arg = args.get(3);
        let unwrap_algo = get_algorithm_name(scope, unwrap_algo_arg);
        let iv = get_iv(scope, unwrap_algo_arg);
        let unwrap_key_meta = extract_key_meta(scope, args.get(2));

        let unwrap_key_bytes = match unwrap_key_meta {
            Some(ref meta) => b64_decode(&meta.key_bytes_b64).unwrap_or_default(),
            None => {
                let uk_arg = args.get(2);
                if uk_arg.is_object() {
                    let uk_obj: v8::Local<v8::Object> =
                        unsafe { v8::Local::cast_unchecked(uk_arg) };
                    let raw_key = crate::v8_utils::v8_string(scope, "__rawKey__");
                    uk_obj
                        .get(scope, raw_key.into())
                        .and_then(|v| extract_bytes(scope, v))
                        .unwrap_or_default()
                } else {
                    vec![]
                }
            }
        };

        let decrypted = match unwrap_algo.to_uppercase().replace("-", "").as_str() {
            "AESGCM" => {
                let iv = iv.unwrap_or_else(|| vec![0u8; 12]);
                aes_gcm_decrypt(&unwrap_key_bytes, &iv, &wrapped_key_bytes)
            }
            "AESCBC" => {
                let iv = iv.unwrap_or_else(|| vec![0u8; 16]);
                aes_cbc_decrypt(&unwrap_key_bytes, &iv, &wrapped_key_bytes)
            }
            _ => Err(format!(
                "unwrapKey: unsupported algorithm '{}'",
                unwrap_algo
            )),
        };

        match decrypted {
            Ok(key_bytes) => {
                let unwrapped_algo_arg = args.get(4);
                let unwrapped_algo = get_algorithm_name(scope, unwrapped_algo_arg);
                let extractable = args.get(5).boolean_value(scope);
                let usages: Vec<String> = if args.get(6).is_array() {
                    let arr: v8::Local<v8::Array> =
                        unsafe { v8::Local::cast_unchecked(args.get(6)) };
                    (0..arr.length())
                        .filter_map(|i| {
                            arr.get_index(scope, i)
                                .map(|v| v.to_rust_string_lossy(scope))
                        })
                        .collect()
                } else {
                    vec![]
                };

                let meta = KeyMeta {
                    key_type: "secret".to_string(),
                    algo: unwrapped_algo,
                    hash: None,
                    curve: None,
                    modulus_length: None,
                    key_bytes_b64: b64_encode(&key_bytes),
                    extractable,
                    usages,
                };
                let key_obj = make_crypto_key(scope, &meta);
                resolver.resolve(scope, key_obj.into());
            }
            Err(e) => {
                let msg = crate::v8_utils::v8_string(scope, &format!("unwrapKey failed: {}", e));
                resolver.reject(scope, v8::Exception::error(scope, msg));
            }
        }
    }));
}
