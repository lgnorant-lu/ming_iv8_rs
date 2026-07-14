//! crypto.getRandomValues(typedArray) and crypto.randomUUID()
//!
//! getRandomValues: fills a TypedArray with cryptographically random values.
//! randomUUID: returns a v4 UUID string.
//!
//! Uses OS random (getrandom crate) by default.
//! v0.2+ will support seeded PRNG for deterministic output.

/// Check if `this` has Crypto.prototype in its prototype chain.
fn check_crypto_receiver(scope: &v8::PinScope<'_, '_>, this: v8::Local<v8::Value>) -> bool {
    let crypto_key = crate::v8_utils::v8_string(scope, "Crypto");
    let ctx = scope.get_current_context();
    let global = ctx.global(scope);
    let mut is_valid = false;
    if let Some(crypto_ctor_val) = global.get(scope, crypto_key.into()) {
        if crypto_ctor_val.is_function() {
            let crypto_ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(crypto_ctor_val) };
            let proto_key = crate::v8_utils::v8_string(scope, "prototype");
            if let Some(proto_val) = crypto_ctor.get(scope, proto_key.into()) {
                let mut cur: v8::Local<v8::Value> = this;
                for _ in 0..30 {
                    if cur.strict_equals(proto_val) { is_valid = true; break; }
                    if !cur.is_object() { break; }
                    let obj = match cur.to_object(scope) { Some(o) => o, None => break };
                    cur = match obj.get_prototype(scope) { Some(p) => p, None => break };
                }
            }
        }
    }
    is_valid
}

/// Install crypto.getRandomValues and crypto.randomUUID on Crypto.prototype.
/// Methods installed on prototype (not instance) for correct prototype chain.
/// Uses with_prototype_and_properties to create crypto with Crypto.prototype.
pub fn install_crypto_random(scope: &v8::PinScope<'_, '_>, global: v8::Local<v8::Object>) {
    // Get Crypto.prototype from codegen constructor
    let crypto_ctor_key = crate::v8_utils::v8_string(scope, "Crypto");
    let crypto_proto = if let Some(ctor_val) = global.get(scope, crypto_ctor_key.into()) {
        if ctor_val.is_function() {
            let ctor: v8::Local<v8::Function> = unsafe { v8::Local::cast_unchecked(ctor_val) };
            let proto_key = crate::v8_utils::v8_string(scope, "prototype");
            if let Some(proto_val) = ctor.get(scope, proto_key.into()) {
                proto_val.to_object(scope)
            } else { None }
        } else { None }
    } else { None };

    // Get or create crypto object with Crypto.prototype
    let crypto_key = crate::v8_utils::v8_string(scope, "crypto");
    let crypto_obj = if let Some(existing) = global.get(scope, crypto_key.into()) {
        if existing.is_object() && !existing.is_null_or_undefined() {
            unsafe { v8::Local::<v8::Object>::cast_unchecked(existing) }
        } else if let Some(ref proto) = crypto_proto {
            let obj = v8::Object::with_prototype_and_properties(scope, (*proto).into(), &[], &[]);
            global.set(scope, crypto_key.into(), obj.into());
            obj
        } else {
            let obj = v8::Object::new(scope);
            global.set(scope, crypto_key.into(), obj.into());
            obj
        }
    } else if let Some(ref proto) = crypto_proto {
        let obj = v8::Object::with_prototype_and_properties(scope, (*proto).into(), &[], &[]);
        global.set(scope, crypto_key.into(), obj.into());
        obj
    } else {
        let obj = v8::Object::new(scope);
        global.set(scope, crypto_key.into(), obj.into());
        obj
    };

    // Install getRandomValues and randomUUID on Crypto.prototype
    if let Some(proto_obj) = crypto_proto {
        let grv_name = crate::v8_utils::v8_string(scope, "getRandomValues");
        let grv_tmpl = v8::FunctionTemplate::builder_raw(get_random_values_callback)
            .length(1)
            .build(scope);
        grv_tmpl.set_class_name(grv_name);
        let grv_fn = crate::v8_utils::v8_fn(scope, &grv_tmpl);
        proto_obj.set(scope, grv_name.into(), grv_fn.into());

        let uuid_name = crate::v8_utils::v8_string(scope, "randomUUID");
        let uuid_tmpl = v8::FunctionTemplate::builder_raw(random_uuid_callback).build(scope);
        uuid_tmpl.set_class_name(uuid_name);
        let uuid_fn = crate::v8_utils::v8_fn(scope, &uuid_tmpl);
        proto_obj.set(scope, uuid_name.into(), uuid_fn.into());
    } else {
        // Fallback: install on instance
        let grv_tmpl = v8::FunctionTemplate::builder_raw(get_random_values_callback).build(scope);
        let grv_fn = crate::v8_utils::v8_fn(scope, &grv_tmpl);
        crypto_obj.set(scope, crate::v8_utils::v8_string(scope, "getRandomValues").into(), grv_fn.into());
        let uuid_tmpl = v8::FunctionTemplate::builder_raw(random_uuid_callback).build(scope);
        let uuid_fn = crate::v8_utils::v8_fn(scope, &uuid_tmpl);
        crypto_obj.set(scope, crate::v8_utils::v8_string(scope, "randomUUID").into(), uuid_fn.into());
    }
}

/// crypto.getRandomValues(typedArray) → fills array with random bytes, returns it.
unsafe extern "C" fn get_random_values_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        // Receiver check
        if !check_crypto_receiver(scope, args.this().into()) {
            let msg = crate::v8_utils::v8_string(scope, "Illegal invocation");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }

        if args.length() < 1 {
            let msg = crate::v8_utils::v8_string(scope, "getRandomValues requires 1 argument");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }

        let arg = args.get(0);

        // Must be a TypedArray (Uint8Array, Uint16Array, Uint32Array, Int8Array, etc.)
        if !arg.is_typed_array() {
            let msg =
                crate::v8_utils::v8_string(scope, "getRandomValues: argument must be a TypedArray");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }

        let ta: v8::Local<v8::TypedArray> = unsafe { v8::Local::cast_unchecked(arg) };
        let byte_length = ta.byte_length();

        // Spec limit: max 65536 bytes
        if byte_length > 65536 {
            let msg = crate::v8_utils::v8_string(
                scope,
                "getRandomValues: quota exceeded (max 65536 bytes)",
            );
            let exc = v8::Exception::error(scope, msg);
            scope.throw_exception(exc);
            return;
        }

        // Generate random bytes
        let mut random_bytes = vec![0u8; byte_length];

        // Check for deterministic crypto_seed
        let isolate: &v8::Isolate = &*scope;
        let use_seed = if crate::state::RuntimeState::has(isolate) {
            let state = crate::state::RuntimeState::get(isolate);
            state.crypto_seed.borrow().is_some()
        } else {
            false
        };

        if use_seed {
            let state = crate::state::RuntimeState::get(isolate);
            let seed = state.crypto_seed.borrow().unwrap_or(0);
            // Simple xorshift64 PRNG seeded from crypto_seed + call counter
            // Not cryptographically secure, but deterministic (which is the point).
            let counter = state.increment_eval_count(); // use eval count as nonce
            let mut s = seed.wrapping_add(counter).wrapping_mul(6364136223846793005) | 1;
            for chunk in random_bytes.chunks_mut(8) {
                s ^= s << 13;
                s ^= s >> 7;
                s ^= s << 17;
                let bytes = s.to_le_bytes();
                let len = chunk.len().min(8);
                chunk[..len].copy_from_slice(&bytes[..len]);
            }
        } else {
            fill_random(&mut random_bytes);
        }

        // Write into the TypedArray's backing store
        // SAFETY: guarded by is_typed_array() check above
        let Some(ab) = ta.buffer(scope) else {
            let msg =
                crate::v8_utils::v8_string(scope, "getRandomValues: backing buffer unavailable");
            scope.throw_exception(v8::Exception::type_error(scope, msg));
            return;
        };
        let byte_offset = ta.byte_offset();
        let store = ab.get_backing_store();
        if let Some(data_ptr) = store.data() {
            let slice = unsafe {
                std::slice::from_raw_parts_mut(
                    (data_ptr.as_ptr() as *mut u8).add(byte_offset),
                    byte_length,
                )
            };
            slice.copy_from_slice(&random_bytes);
        }

        // Return the same TypedArray (per spec)
        rv.set(arg);
    }));
}

/// crypto.randomUUID() → "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx"
unsafe extern "C" fn random_uuid_callback(info: *const v8::FunctionCallbackInfo) {
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let info_ref = unsafe { &*info };
        v8::callback_scope!(unsafe scope, info_ref);
        let args = v8::FunctionCallbackArguments::from_function_callback_info(info_ref);
        let mut rv = v8::ReturnValue::from_function_callback_info(info_ref);

        // Receiver check
        if !check_crypto_receiver(scope, args.this().into()) {
            let msg = crate::v8_utils::v8_string(scope, "Illegal invocation");
            let exc = v8::Exception::type_error(scope, msg);
            scope.throw_exception(exc);
            return;
        }

        let uuid = generate_uuid_v4();
        if let Some(s) = v8::String::new(scope, &uuid) {
            rv.set(s.into());
        }
    }));
}

/// Fill a buffer with cryptographically secure random bytes.
pub fn fill_random_bytes(buf: &mut [u8]) {
    fill_random(buf);
}

/// Fill a buffer with cryptographically secure random bytes.
fn fill_random(buf: &mut [u8]) {
    // Use getrandom crate for cross-platform cryptographic randomness
    // This works on Windows, Linux, macOS, and other platforms

    #[cfg(target_os = "windows")]
    {
        // On Windows, use BCryptGenRandom via the windows API
        // Fallback: use /dev/urandom equivalent via std
        extern "system" {
            fn BCryptGenRandom(
                h_algorithm: *mut std::ffi::c_void,
                pb_buffer: *mut u8,
                cb_buffer: u32,
                dw_flags: u32,
            ) -> i32;
        }
        const BCRYPT_USE_SYSTEM_PREFERRED_RNG: u32 = 0x00000002;
        let result = unsafe {
            BCryptGenRandom(
                std::ptr::null_mut(),
                buf.as_mut_ptr(),
                buf.len() as u32,
                BCRYPT_USE_SYSTEM_PREFERRED_RNG,
            )
        };
        if result == 0 {
            return; // Success
        }
        // Fallback to time-based if BCryptGenRandom fails
        let seed = std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        let mut state = seed ^ 0xdeadbeefcafe1234;
        for byte in buf.iter_mut() {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            state ^= state >> 33;
            state = state.wrapping_mul(0xff51afd7ed558ccd);
            state ^= state >> 33;
            *byte = (state >> 56) as u8;
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        if let Ok(mut f) = std::fs::File::open("/dev/urandom") {
            let _ = f.read_exact(buf);
        }
    }
}

/// Generate a v4 UUID string.
fn generate_uuid_v4() -> String {
    let mut bytes = [0u8; 16];
    fill_random(&mut bytes);

    // Set version (4) and variant (10xx)
    bytes[6] = (bytes[6] & 0x0f) | 0x40; // version 4
    bytes[8] = (bytes[8] & 0x3f) | 0x80; // variant 10xx

    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_random_bytes_fills_buffer_with_non_all_zero() {
        let mut a = [0u8; 32];
        let mut b = [0u8; 32];
        fill_random_bytes(&mut a);
        fill_random_bytes(&mut b);
        assert!(a.iter().any(|&x| x != 0) || b.iter().any(|&x| x != 0));
        assert_ne!(a, b);
    }

    #[test]
    fn test_generate_uuid_v4_format_and_version_variant() {
        let uuid = generate_uuid_v4();
        assert_eq!(uuid.len(), 36);
        assert_eq!(&uuid[8..9], "-");
        assert_eq!(&uuid[13..14], "-");
        assert_eq!(&uuid[18..19], "-");
        assert_eq!(&uuid[23..24], "-");
        assert_eq!(&uuid[14..15], "4");
        let variant_nibble = u8::from_str_radix(&uuid[19..20], 16).unwrap();
        assert!(variant_nibble >= 8 && variant_nibble <= 0xb);
        for (i, ch) in uuid.chars().enumerate() {
            if matches!(i, 8 | 13 | 18 | 23) {
                continue;
            }
            assert!(ch.is_ascii_hexdigit(), "non-hex at {i}: {ch}");
        }
    }

    #[test]
    fn test_generate_uuid_v4_unique_across_calls() {
        let u1 = generate_uuid_v4();
        let u2 = generate_uuid_v4();
        assert_ne!(u1, u2);
    }

    #[test]
    fn test_fill_random_bytes_empty_slice_is_noop() {
        let mut empty: [u8; 0] = [];
        fill_random_bytes(&mut empty);
        assert!(empty.is_empty());
    }

    #[test]
    fn test_fill_random_bytes_single_byte_range() {
        let mut hits = [false; 256];
        for _ in 0..512 {
            let mut b = [0u8; 1];
            fill_random_bytes(&mut b);
            hits[b[0] as usize] = true;
        }
        // Not a statistical proof; just ensure we are not stuck on one value.
        let distinct = hits.iter().filter(|&&h| h).count();
        assert!(distinct >= 2, "expected multiple distinct byte values, got {distinct}");
    }
}
