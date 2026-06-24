//! localStorage / sessionStorage stubs.
//!
//! localStorage can be backed by a shared `LocalStorageStore` for cross-kernel
//! persistence. When a backend is present, `window.__iv8LocalSeed` is injected
//! before this shim runs and the StorageStub initializes from it.
//!
//! sessionStorage is always session-scoped JS-only (cleared on page unload).

/// JS shim for localStorage and sessionStorage.
pub const STORAGE_JS: &str = r#"
(function() {
    // Use the codegen-generated Storage constructor if available;
    // otherwise fall back to a local constructor.
    var StorageCtor = (typeof Storage !== 'undefined') ? Storage : null;

    function makeStorage() {
        var obj;
        if (StorageCtor) {
            // Create instance from codegen Storage.prototype
            // Use Object.create to avoid calling the constructor
            // (codegen constructors may throw illegal_constructor)
            obj = Object.create(StorageCtor.prototype);
        } else {
            // Fallback: define a local Storage constructor
            function Storage() {}
            StorageCtor = Storage;
            Object.defineProperty(Storage.prototype, Symbol.toStringTag, {
                value: 'Storage', configurable: true
            });
            obj = new Storage();
        }

        // Use Object.defineProperty for _data and length to avoid
        // potential issues with FunctionTemplate prototype restrictions
        var seed = window.__iv8LocalSeed;
        var data = (seed && typeof seed === 'object') ? seed : {};
        var len = Object.keys(data).length;
        try {
            Object.defineProperty(obj, '_data', { value: data, writable: true, enumerable: false, configurable: true });
            Object.defineProperty(obj, 'length', { value: len, writable: true, enumerable: false, configurable: true });
        } catch(e) {
            // Fallback: direct assignment
            obj._data = data;
            obj.length = len;
        }
        if (seed) {
            try { delete window.__iv8LocalSeed; } catch(e) {}
        }
        return obj;
    }

    // Install methods on StorageCtor.prototype
    var p = StorageCtor.prototype;
    p.getItem = function(key) {
        return this._data.hasOwnProperty(key) ? this._data[key] : null;
    };
    p.setItem = function(key, value) {
        if (!this._data.hasOwnProperty(key)) this.length++;
        this._data[key] = String(value);
    };
    p.removeItem = function(key) {
        if (this._data.hasOwnProperty(key)) {
            delete this._data[key];
            this.length--;
        }
    };
    p.clear = function() {
        this._data = {};
        this.length = 0;
    };
    p.key = function(index) {
        var keys = Object.keys(this._data);
        return index < keys.length ? keys[index] : null;
    };

    if (typeof localStorage === 'undefined') {
        globalThis.localStorage = makeStorage();
    }
    if (typeof sessionStorage === 'undefined') {
        globalThis.sessionStorage = makeStorage();
    }

    // v0.8.72: explicit dump helper for Rust-side flush (dispose / drop).
    // Returns JSON string of all localStorage entries.
    window.__iv8DumpLocalStorage = function() {
        try {
            var store = localStorage;
            if (store && store._data) {
                return JSON.stringify(store._data);
            }
        } catch(e) {}
        return '{}';
    };
})();
"#;
