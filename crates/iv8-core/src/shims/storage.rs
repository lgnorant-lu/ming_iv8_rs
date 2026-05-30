//! localStorage / sessionStorage stubs.
//!
//! In-memory implementation (not persisted). Sufficient for anti-bot scripts
//! that check for storage existence or do simple get/set.

/// JS shim for localStorage and sessionStorage.
pub const STORAGE_JS: &str = r#"
(function() {
    function StorageStub() {
        this._data = {};
        this.length = 0;
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
