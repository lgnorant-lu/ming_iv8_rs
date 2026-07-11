//! atob/btoa: Base64 encoding/decoding global functions.
//!
//! btoa(string) → base64-encoded string (only Latin-1 input)
//! atob(base64) → decoded string

/// Install atob and btoa as global functions.
pub fn install_atob_btoa(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    let btoa_tmpl = v8::FunctionTemplate::builder_raw(btoa_callback).build(scope);
    let btoa_fn = crate::v8_utils::v8_fn(scope, &btoa_tmpl);
    let btoa_key = crate::v8_utils::v8_string(scope, "btoa");
    btoa_fn.set_name(btoa_key);
    global.set(scope, btoa_key.into(), btoa_fn.into());

    let atob_tmpl = v8::FunctionTemplate::builder_raw(atob_callback).build(scope);
    let atob_fn = crate::v8_utils::v8_fn(scope, &atob_tmpl);
    let atob_key = crate::v8_utils::v8_string(scope, "atob");
    atob_fn.set_name(atob_key);
    global.set(scope, atob_key.into(), atob_fn.into());
}

/// btoa(string) → base64
unsafe extern "C" fn btoa_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            let msg = crate::v8_utils::v8_string(scope, "btoa requires 1 argument");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }

        let input = args.get(0).to_rust_string_lossy(scope);

        // Check Latin-1 range (each char must be <= 255)
        for ch in input.chars() {
            if ch as u32 > 255 {
                let msg = crate::v8_utils::v8_string(
                    scope,
                    "InvalidCharacterError: The string to be encoded contains characters outside of the Latin1 range.",
                );
                let exc = v8::Exception::error(scope, msg);
                scope.throw_exception(exc);
                return;
            }
        }

        // Encode: treat each char as a byte
        let bytes: Vec<u8> = input.chars().map(|c| c as u8).collect();
        let encoded = base64_encode(&bytes);

        if let Some(s) = v8::String::new(scope, &encoded) {
            rv.set(s.into());
        }
    }));
}

/// atob(base64) → string
unsafe extern "C" fn atob_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        if args.length() < 1 {
            let msg = crate::v8_utils::v8_string(scope, "atob requires 1 argument");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }

        let input = args.get(0).to_rust_string_lossy(scope);

        // Remove whitespace (per spec)
        let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();

        match base64_decode(&cleaned) {
            Ok(bytes) => {
                // Convert bytes back to string (Latin-1)
                let decoded: String = bytes.iter().map(|&b| b as char).collect();
                if let Some(s) = v8::String::new(scope, &decoded) {
                    rv.set(s.into());
                }
            }
            Err(_) => {
                let msg = crate::v8_utils::v8_string(
                    scope,
                    "InvalidCharacterError: The string to be decoded is not correctly encoded.",
                );
                let exc = v8::Exception::error(scope, msg);
                scope.throw_exception(exc);
            }
        }
    }));
}

// ─── Base64 implementation (no external dependency) ─────────────────────────

const BASE64_CHARS: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

fn base64_encode(input: &[u8]) -> String {
    let mut result = String::with_capacity(input.len().div_ceil(3) * 4);
    let chunks = input.chunks(3);

    for chunk in chunks {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };

        let triple = (b0 << 16) | (b1 << 8) | b2;

        result.push(BASE64_CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(BASE64_CHARS[((triple >> 12) & 0x3F) as usize] as char);

        if chunk.len() > 1 {
            result.push(BASE64_CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }

        if chunk.len() > 2 {
            result.push(BASE64_CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }

    result
}

fn base64_decode(input: &str) -> Result<Vec<u8>, &'static str> {
    if input.is_empty() {
        return Ok(Vec::new());
    }

    // Validate and build lookup
    let mut buf = Vec::with_capacity(input.len() * 3 / 4);
    let mut acc: u32 = 0;
    let mut bits: u32 = 0;

    for ch in input.chars() {
        if ch == '=' {
            break;
        }
        let val = match ch {
            'A'..='Z' => ch as u32 - 'A' as u32,
            'a'..='z' => ch as u32 - 'a' as u32 + 26,
            '0'..='9' => ch as u32 - '0' as u32 + 52,
            '+' => 62,
            '/' => 63,
            _ => return Err("invalid character"),
        };

        acc = (acc << 6) | val;
        bits += 6;

        if bits >= 8 {
            bits -= 8;
            buf.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }

    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_encode_empty() {
        assert_eq!(base64_encode(b""), "");
    }

    #[test]
    fn test_base64_encode_one_byte() {
        assert_eq!(base64_encode(b"f"), "Zg==");
    }

    #[test]
    fn test_base64_encode_two_bytes() {
        assert_eq!(base64_encode(b"fo"), "Zm8=");
    }

    #[test]
    fn test_base64_encode_three_bytes() {
        assert_eq!(base64_encode(b"foo"), "Zm9v");
    }

    #[test]
    fn test_base64_encode_roundtrip() {
        let data = vec![0u8, 1, 2, 3, 255, 254, 253, 128, 64, 32];
        let encoded = base64_encode(&data);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_base64_decode_empty() {
        assert_eq!(base64_decode("").unwrap(), Vec::<u8>::new());
    }

    #[test]
    fn test_base64_decode_known_values() {
        assert_eq!(base64_decode("Zg==").unwrap(), b"f");
        assert_eq!(base64_decode("Zm8=").unwrap(), b"fo");
        assert_eq!(base64_decode("Zm9v").unwrap(), b"foo");
    }

    #[test]
    fn test_base64_decode_invalid_char() {
        assert!(base64_decode("!!!invalid!!!").is_err());
    }

    #[test]
    fn test_base64_decode_padding() {
        let result = base64_decode("Zm9vYg==").unwrap();
        assert_eq!(result, b"foob");
    }

    #[test]
    fn test_base64_encode_all_bytes() {
        let data: Vec<u8> = (0..=255).collect();
        let encoded = base64_encode(&data);
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_base64_encode_binary_data() {
        let data = vec![0xFFu8; 16];
        let encoded = base64_encode(&data);
        assert_eq!(encoded.len(), 24); // 16 bytes -> 24 base64 chars (no padding, 16%3!=0)
        let decoded = base64_decode(&encoded).unwrap();
        assert_eq!(decoded, data);
    }
}
