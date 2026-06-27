//! Structured clone serialization for Web Worker postMessage.
//!
//! Implements V8 ValueSerializer/ValueDeserializer delegates for structured
//! clone algorithm (W3C HTML spec §2.9.5). Used by Worker.postMessage to
//! transfer data between isolates via mpsc channels.

use v8::{ValueDeserializerImpl, ValueDeserializerHelper, ValueSerializerImpl, ValueSerializerHelper};

pub struct Iv8ValueSerializer;

impl ValueSerializerImpl for Iv8ValueSerializer {
    fn throw_data_clone_error<'s>(
        &self,
        scope: &mut v8::PinScope<'s, '_>,
        message: v8::Local<'s, v8::String>,
    ) {
        let exc = v8::Exception::type_error(scope, message);
        scope.throw_exception(exc);
    }
}

pub struct Iv8ValueDeserializer;

impl ValueDeserializerImpl for Iv8ValueDeserializer {}

pub fn serialize_value<'s>(
    scope: &v8::PinScope<'s, '_>,
    context: v8::Local<'s, v8::Context>,
    value: v8::Local<'s, v8::Value>,
) -> Result<Vec<u8>, String> {
    let mut ser = v8::ValueSerializer::new(scope, Box::new(Iv8ValueSerializer));
    ser.write_header();
    match ser.write_value(context, value) {
        Some(true) => Ok(ser.release()),
        _ => Err("structured clone serialization failed".into()),
    }
}

pub fn deserialize_value<'s>(
    scope: &v8::PinScope<'s, '_>,
    context: v8::Local<'s, v8::Context>,
    bytes: &[u8],
) -> Option<v8::Local<'s, v8::Value>> {
    if bytes.is_empty() {
        return None;
    }
    let mut de = v8::ValueDeserializer::new(scope, Box::new(Iv8ValueDeserializer), bytes);
    if de.read_header(context) != Some(true) {
        return None;
    }
    de.read_value(context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_deserialize_roundtrip_primitive() {
        crate::v8_init::ensure_v8_initialized();
        let mut isolate = v8::Isolate::new(v8::CreateParams::default());
        v8::scope!(hs, &mut isolate);
        let ctx = v8::Context::new(hs, Default::default());
        v8::scope_with_context!(scope, hs, ctx);

        let val = v8::Number::new(scope, 42.0);
        let bytes = serialize_value(scope, ctx, val.into()).unwrap();
        assert!(!bytes.is_empty());

        let restored = deserialize_value(scope, ctx, &bytes).unwrap();
        assert!(restored.is_number());
        assert_eq!(restored.number_value(scope), Some(42.0));
    }

    #[test]
    fn serialize_deserialize_roundtrip_string() {
        crate::v8_init::ensure_v8_initialized();
        let mut isolate = v8::Isolate::new(v8::CreateParams::default());
        v8::scope!(hs, &mut isolate);
        let ctx = v8::Context::new(hs, Default::default());
        v8::scope_with_context!(scope, hs, ctx);

        let val = crate::v8_utils::v8_string(scope, "hello worker");
        let bytes = serialize_value(scope, ctx, val.into()).unwrap();
        let restored = deserialize_value(scope, ctx, &bytes).unwrap();
        assert_eq!(restored.to_rust_string_lossy(scope), "hello worker");
    }

    #[test]
    fn serialize_deserialize_roundtrip_object() {
        crate::v8_init::ensure_v8_initialized();
        let mut isolate = v8::Isolate::new(v8::CreateParams::default());
        v8::scope!(hs, &mut isolate);
        let ctx = v8::Context::new(hs, Default::default());
        v8::scope_with_context!(scope, hs, ctx);

        let src = crate::v8_utils::v8_string(scope, "({a: 1, b: 'two', c: [3, 4]})");
        let script = v8::Script::compile(scope, src, None).unwrap();
        let val = script.run(scope).unwrap();

        let bytes = serialize_value(scope, ctx, val).unwrap();
        let restored = deserialize_value(scope, ctx, &bytes).unwrap();
        assert!(restored.is_object());

        let obj: v8::Local<v8::Object> = unsafe { v8::Local::cast_unchecked(restored) };
        let a_key = crate::v8_utils::v8_string(scope, "a");
        let a_val = obj.get(scope, a_key.into()).unwrap();
        assert_eq!(a_val.int32_value(scope), Some(1));
    }

    #[test]
    fn serialize_deserialize_roundtrip_array() {
        crate::v8_init::ensure_v8_initialized();
        let mut isolate = v8::Isolate::new(v8::CreateParams::default());
        v8::scope!(hs, &mut isolate);
        let ctx = v8::Context::new(hs, Default::default());
        v8::scope_with_context!(scope, hs, ctx);

        let src = crate::v8_utils::v8_string(scope, "[10, 20, 30]");
        let script = v8::Script::compile(scope, src, None).unwrap();
        let val = script.run(scope).unwrap();

        let bytes = serialize_value(scope, ctx, val).unwrap();
        let restored = deserialize_value(scope, ctx, &bytes).unwrap();
        assert!(restored.is_array());

        let arr: v8::Local<v8::Array> = unsafe { v8::Local::cast_unchecked(restored) };
        assert_eq!(arr.length(), 3);
    }
}
