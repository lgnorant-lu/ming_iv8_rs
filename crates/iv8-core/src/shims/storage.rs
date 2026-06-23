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
    function StorageStub() {
        if (window.__iv8LocalSeed) {
            this._data = window.__iv8LocalSeed;
            this.length = Object.keys(window.__iv8LocalSeed).length;
            delete window.__iv8LocalSeed;
        } else {
            this._data = {};
            this.length = 0;
        }
    }
    StorageStub.prototype.getItem = function(key) {
        return this._data.hasOwnProperty(key) ? this._data[key] : null;
    };
    StorageStub.prototype.setItem = function(key, value) {
        if (!this._data.hasOwnProperty(key)) this.length++;
        this._data[key] = String(value);
    };
    StorageStub.prototype.removeItem = function(key) {
        if (this._data.hasOwnProperty(key)) {
            delete this._data[key];
            this.length--;
        }
    };
    StorageStub.prototype.clear = function() {
        this._data = {};
        this.length = 0;
    };
    StorageStub.prototype.key = function(index) {
        var keys = Object.keys(this._data);
        return index < keys.length ? keys[index] : null;
    };

    if (typeof localStorage === 'undefined') {
        globalThis.localStorage = new StorageStub();
    }
    if (typeof sessionStorage === 'undefined') {
        globalThis.sessionStorage = new StorageStub();
    }
})();
"#;
