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
