//! v0.8.55: Integration tests for Screen surface.
mod common;

#[test]
fn test_screen_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof screen", "object");
}

#[test]
fn test_screen_width() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof screen.width", "number");
    let w = common::to_str(&k.eval_to_rust_value("screen.width"));
    let w_val: i64 = w.parse().unwrap();
    assert!(w_val >= 800, "screen.width too small: {}", w);
}

#[test]
fn test_screen_height() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof screen.height", "number");
    let h = common::to_str(&k.eval_to_rust_value("screen.height"));
    let h_val: i64 = h.parse().unwrap();
    assert!(h_val >= 600, "screen.height too small: {}", h);
}

#[test]
fn test_screen_avail_dimensions_not_exceed_full() {
    let mut k = common::make_kernel();
    let w = k.eval_to_rust_value("screen.width");
    let aw = k.eval_to_rust_value("screen.availWidth");
    let h = k.eval_to_rust_value("screen.height");
    let ah = k.eval_to_rust_value("screen.availHeight");
    let w_val: i64 = common::to_str(&w).parse().unwrap();
    let aw_val: i64 = common::to_str(&aw).parse().unwrap();
    let h_val: i64 = common::to_str(&h).parse().unwrap();
    let ah_val: i64 = common::to_str(&ah).parse().unwrap();
    assert!(aw_val <= w_val, "availWidth > width: {} > {}", aw_val, w_val);
    assert!(ah_val <= h_val, "availHeight > height: {} > {}", ah_val, h_val);
}

#[test]
fn test_screen_color_depth() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof screen.colorDepth", "number");
    common::assert_js_str(&mut k, "typeof screen.pixelDepth", "number");
}

#[test]
fn test_screen_avail_left_top() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof screen.availLeft", "number");
    common::assert_js_str(&mut k, "typeof screen.availTop", "number");
}

#[test]
fn test_screen_custom_profile() {
    use iv8_core::shims::browser_profile::{BrowserProfile, DEFAULT_PROFILE};
    let profile = BrowserProfile {
        screen_width: 1280.0,
        screen_height: 720.0,
        screen_avail_width: 1280.0,
        screen_avail_height: 680.0,
        screen_color_depth: 30.0,
        screen_pixel_depth: 30.0,
        ..DEFAULT_PROFILE
    };
    let mut k = common::make_kernel_with_profile(profile);
    common::assert_js_str(&mut k, "screen.width", "1280");
    common::assert_js_str(&mut k, "screen.height", "720");
    common::assert_js_str(&mut k, "screen.availWidth", "1280");
    common::assert_js_str(&mut k, "screen.availHeight", "680");
    common::assert_js_str(&mut k, "screen.colorDepth", "30");
    common::assert_js_str(&mut k, "screen.pixelDepth", "30");
}

// v0.8.61: generated Screen template unification.
// Generated skeleton properties (orientation, isExtended, onchange)
// now visible via Screen template inherit().

#[test]
fn test_screen_generated_skeleton_visible() {
    let mut k = common::make_kernel();
    // Generated skeleton properties visible
    common::assert_js_str(&mut k, "typeof screen.orientation", "object");
    // isExtended is generated as boolean (codegen maps correctly)
    common::assert_js_str(&mut k, "typeof screen.isExtended", "boolean");
    // Native getters still override generated skeletons
    common::assert_js_str(&mut k, "typeof screen.width", "number");
    common::assert_js_str(&mut k, "typeof screen.colorDepth", "number");
}

// v0.8.61: Screen constructor still throws TypeError
#[test]
fn test_screen_constructor_throws() {
    let mut k = common::make_kernel();
    common::assert_js_error(&mut k, "new Screen()");
    // Verify constructor exists and is callable with new
    common::assert_js_str(&mut k, "typeof Screen", "function");
}
