//! URL and URLSearchParams global classes.
//!
//! Implements the WHATWG URL API subset needed for anti-bot scripts.

/// JS shim for URL and URLSearchParams.
pub const URL_SHIM_JS: &str = r#"
(function() {
    // ─── URLSearchParams ────────────────────────────────────────────────────
    function URLSearchParams(init) {
        this._params = [];
        if (typeof init === 'string') {
            var s = init.charAt(0) === '?' ? init.slice(1) : init;
            if (s) {
                var pairs = s.split('&');
                for (var i = 0; i < pairs.length; i++) {
                    var eq = pairs[i].indexOf('=');
                    if (eq === -1) {
                        this._params.push([decodeURIComponent(pairs[i]), '']);
                    } else {
                        this._params.push([
                            decodeURIComponent(pairs[i].slice(0, eq)),
                            decodeURIComponent(pairs[i].slice(eq + 1))
                        ]);
                    }
                }
            }
        } else if (init && typeof init === 'object' && !(init instanceof URLSearchParams)) {
            var keys = Object.keys(init);
            for (var i = 0; i < keys.length; i++) {
                this._params.push([keys[i], String(init[keys[i]])]);
            }
        } else if (init instanceof URLSearchParams) {
            this._params = init._params.slice();
        }
    }

    function _uspThis(self) {
        if (self == null || typeof self !== 'object' || !Array.isArray(self._params)) {
            throw new TypeError('Illegal invocation');
        }
        return self;
    }

    URLSearchParams.prototype.get = function get(name) {
        var self = _uspThis(this);
        for (var i = 0; i < self._params.length; i++) {
            if (self._params[i][0] === name) return self._params[i][1];
        }
        return null;
    };

    URLSearchParams.prototype.getAll = function getAll(name) {
        var self = _uspThis(this);
        var result = [];
        for (var i = 0; i < self._params.length; i++) {
            if (self._params[i][0] === name) result.push(self._params[i][1]);
        }
        return result;
    };

    URLSearchParams.prototype.has = function has(name) {
        var self = _uspThis(this);
        for (var i = 0; i < self._params.length; i++) {
            if (self._params[i][0] === name) return true;
        }
        return false;
    };

    URLSearchParams.prototype.set = function set(name, value) {
        var self = _uspThis(this);
        var found = false;
        for (var i = self._params.length - 1; i >= 0; i--) {
            if (self._params[i][0] === name) {
                if (!found) { self._params[i][1] = String(value); found = true; }
                else { self._params.splice(i, 1); }
            }
        }
        if (!found) self._params.push([name, String(value)]);
    };

    URLSearchParams.prototype.append = function append(name, value) {
        _uspThis(this)._params.push([name, String(value)]);
    };

    URLSearchParams.prototype['delete'] = function(name) {
        var self = _uspThis(this);
        self._params = self._params.filter(function(p) { return p[0] !== name; });
    };

    URLSearchParams.prototype.toString = function toString() {
        return _uspThis(this)._params.map(function(p) {
            return encodeURIComponent(p[0]) + '=' + encodeURIComponent(p[1]);
        }).join('&');
    };

    URLSearchParams.prototype.sort = function sort() {
        _uspThis(this)._params.sort(function(a, b) { return a[0] < b[0] ? -1 : a[0] > b[0] ? 1 : 0; });
    };

    Object.defineProperty(URLSearchParams.prototype, 'size', {
        get: function() { return _uspThis(this)._params.length; },
        enumerable: true, configurable: true
    });

    URLSearchParams.prototype.forEach = function forEach(callback, thisArg) {
        var self = _uspThis(this);
        for (var i = 0; i < self._params.length; i++) {
            callback.call(thisArg, self._params[i][1], self._params[i][0], self);
        }
    };

    URLSearchParams.prototype.entries = function entries() {
        var p = _uspThis(this)._params;
        return p[Symbol.iterator] ? p[Symbol.iterator]() : p;
    };

    URLSearchParams.prototype.keys = function keys() {
        return _uspThis(this)._params.map(function(p) { return p[0]; });
    };

    URLSearchParams.prototype.values = function values() {
        return _uspThis(this)._params.map(function(p) { return p[1]; });
    };

    Object.defineProperty(URLSearchParams.prototype, Symbol.toStringTag, {
        value: 'URLSearchParams', writable: true, configurable: true, enumerable: false
    });

    globalThis.URLSearchParams = URLSearchParams;

    // ─── URL ────────────────────────────────────────────────────────────────
    function URL(url, base) {
        if (!(this instanceof URL)) {
            throw new TypeError("Failed to construct 'URL': Please use the 'new' operator");
        }
        var fullUrl = url;
        if (base) {
            // Simple base resolution
            if (url.indexOf('://') === -1 && url.charAt(0) !== '/') {
                fullUrl = base.replace(/\/[^\/]*$/, '/') + url;
            } else if (url.charAt(0) === '/') {
                var m = base.match(/^(https?:\/\/[^\/]+)/);
                fullUrl = (m ? m[1] : '') + url;
            }
        }

        // Parse URL
        var match = fullUrl.match(/^(https?:)\/\/([^:\/\?#]+)(?::(\d+))?(\/[^\?#]*)?(\?[^#]*)?(#.*)?$/);
        if (!match) {
            // Try simpler patterns
            match = fullUrl.match(/^([a-z]+:)\/\/([^:\/\?#]+)(?::(\d+))?(\/[^\?#]*)?(\?[^#]*)?(#.*)?$/);
        }
        if (!match) {
            throw new TypeError("Failed to construct 'URL': Invalid URL");
        }

        this.protocol = match[1] || '';
        this.hostname = match[2] || '';
        this.port = match[3] || '';
        this.pathname = match[4] || '/';
        this.search = match[5] || '';
        this.hash = match[6] || '';
        this.host = this.hostname + (this.port ? ':' + this.port : '');
        this._origin = this.protocol + '//' + this.host;
        this.href = this._origin + this.pathname + this.search + this.hash;
        this._searchParams = new URLSearchParams(this.search);
        this.username = '';
        this.password = '';
    }

    function _urlThis(self) {
        if (self == null || typeof self !== 'object' || !('href' in self)) {
            throw new TypeError('Illegal invocation');
        }
        return self;
    }
    Object.defineProperty(URL.prototype, 'origin', {
        get: function() { return _urlThis(this)._origin; },
        enumerable: true, configurable: true
    });
    Object.defineProperty(URL.prototype, 'searchParams', {
        get: function() { return _urlThis(this)._searchParams; },
        enumerable: true, configurable: true
    });

    URL.prototype.toString = function toString() { return _urlThis(this).href; };
    URL.prototype.toJSON = function toJSON() { return _urlThis(this).href; };

    Object.defineProperty(URL.prototype, Symbol.toStringTag, {
        value: 'URL', writable: true, configurable: true, enumerable: false
    });

    // URL.length should be 1 (url is required, base is optional)
    Object.defineProperty(URL, 'length', { value: 1, writable: false, enumerable: false, configurable: true });
    // URLSearchParams.length should be 0 (init is optional)
    Object.defineProperty(URLSearchParams, 'length', { value: 0, writable: false, enumerable: false, configurable: true });

    // Force-own global constructors (overwrite empty codegen skeletons if present).
    try {
        Object.defineProperty(globalThis, 'URLSearchParams', {
            value: URLSearchParams, writable: true, enumerable: false, configurable: true
        });
    } catch (e) {
        globalThis.URLSearchParams = URLSearchParams;
    }
    try {
        Object.defineProperty(globalThis, 'URL', {
            value: URL, writable: true, enumerable: false, configurable: true
        });
    } catch (e) {
        globalThis.URL = URL;
    }
    try {
        Object.defineProperty(globalThis, 'webkitURL', {
            value: URL, writable: true, enumerable: false, configurable: true
        });
    } catch (e) {
        globalThis.webkitURL = URL;
    }
})();
"#;
