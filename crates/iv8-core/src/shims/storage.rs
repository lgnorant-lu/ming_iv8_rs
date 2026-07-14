//! localStorage / sessionStorage stubs.
//!
//! localStorage can be backed by a shared `LocalStorageStore` for cross-kernel
//! persistence. When a backend is present, `window.__iv8LocalSeed` is injected
//! before this shim runs and the Storage instance initializes from it.
//!
//! sessionStorage is always session-scoped JS-only (cleared on page unload).
//!
//! v0.8.97: Window codegen installs `localStorage`/`sessionStorage` as
//! **accessors**. A plain data assignment is ignored; we install singleton
//! getters. Methods are own properties so native empty `setItem` cannot shadow.

/// JS shim for localStorage and sessionStorage.
pub const STORAGE_JS: &str = r#"
(function() {
    var StorageCtor = (typeof Storage !== 'undefined') ? Storage : null;

    function bindMethods(obj) {
        obj.getItem = function getItem(key) {
            if (arguments.length < 1) throw new TypeError('1 argument required, but only 0 present.');
            return this._data.hasOwnProperty(key) ? this._data[key] : null;
        };
        obj.setItem = function setItem(key, value) {
            if (arguments.length < 2) throw new TypeError('2 argument(s) required, but only ' + arguments.length + ' present.');
            var next = String(value);
            var prev = this._data.hasOwnProperty(key) ? String(this._data[key]) : null;
            var delta = next.length - (prev === null ? 0 : prev.length);
            var used = 0;
            var keys = Object.keys(this._data);
            for (var i = 0; i < keys.length; i++) {
                used += String(this._data[keys[i]]).length;
            }
            if (prev === null) used += String(key).length;
            var quota = 5 * 1024 * 1024;
            if (used + delta > quota) {
                var err = new Error(
                    "Failed to execute 'setItem' on 'Storage': Setting the value of '" + key + "' exceeded the quota."
                );
                err.name = 'QuotaExceededError';
                throw err;
            }
            if (prev === null) this.length++;
            this._data[key] = next;
        };
        obj.removeItem = function removeItem(key) {
            if (arguments.length < 1) throw new TypeError('1 argument required, but only 0 present.');
            if (this._data.hasOwnProperty(key)) {
                delete this._data[key];
                this.length--;
            }
        };
        obj.clear = function clear() {
            this._data = {};
            this.length = 0;
        };
        obj.key = function key(index) {
            if (arguments.length < 1) throw new TypeError('1 argument required, but only 0 present.');
            var keys = Object.keys(this._data);
            return index < keys.length ? keys[index] : null;
        };
    }

    function makeStorage(useSeed) {
        var obj;
        if (StorageCtor) {
            obj = Object.create(StorageCtor.prototype);
        } else {
            function Storage() {}
            StorageCtor = Storage;
            Object.defineProperty(Storage.prototype, Symbol.toStringTag, {
                value: 'Storage', configurable: true
            });
            obj = new Storage();
        }

        var seed = useSeed ? window.__iv8LocalSeed : null;
        var data = (seed && typeof seed === 'object') ? seed : {};
        var len = Object.keys(data).length;
        try {
            Object.defineProperty(obj, '_data', { value: data, writable: true, enumerable: false, configurable: true });
            Object.defineProperty(obj, 'length', { value: len, writable: true, enumerable: false, configurable: true });
        } catch (e) {
            obj._data = data;
            obj.length = len;
        }
        if (useSeed && seed) {
            try { delete window.__iv8LocalSeed; } catch (e) {}
        }
        bindMethods(obj);
        return obj;
    }

    // Codegen Window exposes localStorage/sessionStorage as accessors that may
    // return empty native Storage stubs. Replace with singleton getters.
    var _localStorage = makeStorage(true);
    var _sessionStorage = makeStorage(false);

    function installSingleton(name, instance) {
        try {
            Object.defineProperty(globalThis, name, {
                get: function() { return instance; },
                set: function() {},
                configurable: true,
                enumerable: true
            });
        } catch (e) {
            try {
                delete globalThis[name];
            } catch (e2) {}
            try {
                Object.defineProperty(globalThis, name, {
                    value: instance,
                    writable: true,
                    configurable: true,
                    enumerable: true
                });
            } catch (e3) {
                try { globalThis[name] = instance; } catch (e4) {}
            }
        }
    }

    installSingleton('localStorage', _localStorage);
    installSingleton('sessionStorage', _sessionStorage);

    // v0.8.72: explicit dump helper for Rust-side flush (dispose / drop).
    window.__iv8DumpLocalStorage = function() {
        try {
            if (_localStorage && _localStorage._data) {
                return JSON.stringify(_localStorage._data);
            }
        } catch (e) {}
        return '{}';
    };
})();
"#;
