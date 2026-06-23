//! v0.8.51: Integration tests for storage surface (localStorage, sessionStorage).
//! v0.8.72: + cross-kernel persistence tests (Track A).
mod common;

use iv8_core::dom::local_storage::LocalStorageStore;
use iv8_core::kernel::KernelConfig;

#[test]
fn test_local_storage_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof localStorage", "object");
}

#[test]
fn test_session_storage_exists() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof sessionStorage", "object");
}

#[test]
fn test_local_storage_set_get() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("localStorage.setItem('k', 'v')");
    let val = common::to_str(&k.eval_to_rust_value("localStorage.getItem('k')"));
    assert_eq!(val, "v");
}

#[test]
fn test_local_storage_remove() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("localStorage.setItem('x', '1')");
    k.eval_to_rust_value("localStorage.removeItem('x')");
    let val = common::to_str(&k.eval_to_rust_value("localStorage.getItem('x')"));
    assert_eq!(val, "null");
}

#[test]
fn test_local_storage_length() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof localStorage.length", "number");
}

#[test]
fn test_local_storage_clear() {
    let mut k = common::make_kernel();
    k.eval_to_rust_value("localStorage.setItem('a', '1')");
    k.eval_to_rust_value("localStorage.setItem('b', '2')");
    k.eval_to_rust_value("localStorage.clear()");
    let len = common::to_str(&k.eval_to_rust_value("localStorage.length"));
    assert_eq!(len, "0");
}

// ── v0.8.72 Track A: cross-kernel persistence ──

#[test]
fn test_local_storage_cross_kernel_persistence() {
    let store = LocalStorageStore::new();

    // Kernel 1: set a value and dispose
    {
        let mut cfg = KernelConfig::default();
        cfg.local_storage = Some(store.clone());
        let mut k1 = iv8_core::kernel::embedded_v8::EmbeddedV8Kernel::new(cfg).unwrap();
        k1.eval_to_rust_value("localStorage.setItem('persist_test', 'hello_world')");
        let v1 = common::to_str(
            &k1.eval_to_rust_value("localStorage.getItem('persist_test')"),
        );
        assert_eq!(v1, "hello_world");
        k1.dispose();
    }

    // Kernel 2 with same store: data survives
    {
        let mut cfg = KernelConfig::default();
        cfg.local_storage = Some(store.clone());
        let mut k2 = iv8_core::kernel::embedded_v8::EmbeddedV8Kernel::new(cfg).unwrap();
        let v2 = common::to_str(
            &k2.eval_to_rust_value("localStorage.getItem('persist_test')"),
        );
        assert_eq!(
            v2, "hello_world",
            "localStorage should survive across kernel instances"
        );
        k2.dispose();
    }
}

#[test]
fn test_local_storage_cross_kernel_multiple_keys() {
    let store = LocalStorageStore::new();

    {
        let mut cfg = KernelConfig::default();
        cfg.local_storage = Some(store.clone());
        let mut k1 = iv8_core::kernel::embedded_v8::EmbeddedV8Kernel::new(cfg).unwrap();
        k1.eval_to_rust_value("localStorage.setItem('a', '1')");
        k1.eval_to_rust_value("localStorage.setItem('b', '2')");
        k1.eval_to_rust_value("localStorage.setItem('c', '3')");
        let len1 = common::to_str(&k1.eval_to_rust_value("localStorage.length"));
        assert_eq!(len1, "3");
        k1.dispose();
    }

    {
        let mut cfg = KernelConfig::default();
        cfg.local_storage = Some(store.clone());
        let mut k2 = iv8_core::kernel::embedded_v8::EmbeddedV8Kernel::new(cfg).unwrap();
        let len2 = common::to_str(&k2.eval_to_rust_value("localStorage.length"));
        assert_eq!(len2, "3", "all three keys should survive");
        let a = common::to_str(&k2.eval_to_rust_value("localStorage.getItem('a')"));
        let b = common::to_str(&k2.eval_to_rust_value("localStorage.getItem('b')"));
        let c = common::to_str(&k2.eval_to_rust_value("localStorage.getItem('c')"));
        assert_eq!(a, "1");
        assert_eq!(b, "2");
        assert_eq!(c, "3");
        k2.dispose();
    }
}

#[test]
fn test_local_storage_no_store_uses_default() {
    let mut k = common::make_kernel();
    common::assert_js_str(&mut k, "typeof localStorage", "object");
    k.eval_to_rust_value("localStorage.setItem('no_store', 'still_works')");
    let v = common::to_str(
        &k.eval_to_rust_value("localStorage.getItem('no_store')"),
    );
    assert_eq!(v, "still_works");
}

#[test]
fn test_local_storage_empty_store_no_seed() {
    let store = LocalStorageStore::new();
    let mut cfg = KernelConfig::default();
    cfg.local_storage = Some(store);
    let mut k = iv8_core::kernel::embedded_v8::EmbeddedV8Kernel::new(cfg).unwrap();
    let len = common::to_str(&k.eval_to_rust_value("localStorage.length"));
    assert_eq!(len, "0");
}

#[test]
fn test_local_storage_persists_on_drop() {
    let store = LocalStorageStore::new();

    // Kernel 1: set value, let it drop (no explicit dispose)
    {
        let mut cfg = KernelConfig::default();
        cfg.local_storage = Some(store.clone());
        let mut k1 = iv8_core::kernel::embedded_v8::EmbeddedV8Kernel::new(cfg).unwrap();
        k1.eval_to_rust_value("localStorage.setItem('drop_test', 'survives_drop')");
        // no explicit dispose — RAII drop must flush
    }

    // Kernel 2: value must survive
    {
        let mut cfg = KernelConfig::default();
        cfg.local_storage = Some(store.clone());
        let mut k2 = iv8_core::kernel::embedded_v8::EmbeddedV8Kernel::new(cfg).unwrap();
        let v = common::to_str(
            &k2.eval_to_rust_value("localStorage.getItem('drop_test')"),
        );
        assert_eq!(
            v, "survives_drop",
            "Drop should flush localStorage: got '{}'",
            v
        );
        k2.dispose();
    }
}
