//! v0.8.51: Integration tests for AudioContext surface.
mod common;

#[test]
fn test_audio_context_constructor_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof AudioContext", "function");
}

#[test]
fn test_audio_context_create() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("var a = new AudioContext(); typeof a"));
    assert_eq!(val, "object");
}

#[test]
fn test_audio_context_sample_rate() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("new AudioContext().sampleRate"));
    assert!(val != "0" && val != "null", "sampleRate invalid: {}", val);
}

#[test]
fn test_audio_context_destination_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof (new AudioContext()).destination", "object");
}

#[test]
fn test_audio_context_create_oscillator() {
    let mut k = common::make_kernel();
    let val =
        common::to_str(&k.eval_to_rust_value("typeof (new AudioContext()).createOscillator()"));
    assert_eq!(val, "object");
}

#[test]
fn test_audio_context_create_gain() {
    let mut k = common::make_kernel();
    let val = common::to_str(&k.eval_to_rust_value("typeof (new AudioContext()).createGain()"));
    assert_eq!(val, "object");
}

#[test]
fn test_audio_context_close_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof (new AudioContext()).close", "function");
}

#[test]
fn test_audio_context_state() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof (new AudioContext()).state", "string");
}
