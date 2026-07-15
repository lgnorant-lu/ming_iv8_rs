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

        this._protocol = match[1] || '';
        this._hostname = match[2] || '';
        this._port = match[3] || '';
        this._pathname = match[4] || '/';
        this._search = match[5] || '';
        this._hash = match[6] || '';
        this._username = '';
        this._password = '';
        this._searchParams = new URLSearchParams(this._search);
        this._rebuild();
    }

    function _urlThis(self) {
        if (self == null || typeof self !== 'object' || !(self instanceof URL)) {
            throw new TypeError('Illegal invocation');
        }
        return self;
    }

    URL.prototype._rebuild = function _rebuild() {
        var host = this._hostname + (this._port ? (':' + this._port) : '');
        this._host = host;
        this._origin = this._protocol + '//' + host;
        this._href = this._origin + this._pathname + this._search + this._hash;
        if (this._searchParams) {
            // keep searchParams in sync when search string changes externally
            try {
                var sp = this._searchParams.toString();
                var want = this._search ? this._search.replace(/^\?/, '') : '';
                if (sp !== want) {
                    this._searchParams = new URLSearchParams(this._search);
                }
            } catch (e) {}
        }
    };

    function _defUrlAcc(name, getKey, setFn) {
        Object.defineProperty(URL.prototype, name, {
            get: function() {
                var s = _urlThis(this);
                return s[getKey];
            },
            set: setFn ? function(v) {
                var s = _urlThis(this);
                setFn.call(s, v);
                s._rebuild();
            } : undefined,
            enumerable: true, configurable: true
        });
    }

    _defUrlAcc('protocol', '_protocol', function(v) {
        var p = String(v);
        if (p.charAt(p.length - 1) !== ':') p += ':';
        this._protocol = p;
    });
    _defUrlAcc('hostname', '_hostname', function(v) {
        this._hostname = String(v);
    });
    _defUrlAcc('port', '_port', function(v) {
        this._port = String(v);
    });
    _defUrlAcc('pathname', '_pathname', function(v) {
        var p = String(v);
        this._pathname = p.charAt(0) === '/' ? p : ('/' + p);
    });
    _defUrlAcc('search', '_search', function(v) {
        var s = String(v);
        this._search = !s ? '' : (s.charAt(0) === '?' ? s : ('?' + s));
        this._searchParams = new URLSearchParams(this._search);
    });
    _defUrlAcc('hash', '_hash', function(v) {
        var h = String(v);
        this._hash = !h ? '' : (h.charAt(0) === '#' ? h : ('#' + h));
    });
    Object.defineProperty(URL.prototype, 'host', {
        get: function() {
            var s = _urlThis(this);
            return s._hostname + (s._port ? (':' + s._port) : '');
        },
        set: function(v) {
            var s = _urlThis(this);
            var str = String(v);
            var coli = str.lastIndexOf(':');
            if (coli > 0) {
                s._hostname = str.slice(0, coli);
                s._port = str.slice(coli + 1);
            } else {
                s._hostname = str;
                s._port = '';
            }
            s._rebuild();
        },
        enumerable: true, configurable: true
    });
    Object.defineProperty(URL.prototype, 'href', {
        get: function() { return _urlThis(this)._href; },
        set: function(v) {
            var s = _urlThis(this);
            var fullUrl = String(v);
            var match = fullUrl.match(/^(https?:)\/\/([^:\/\?#]+)(?::(\d+))?(\/[^\?#]*)?(\?[^#]*)?(#.*)?$/);
            if (!match) {
                match = fullUrl.match(/^([a-z]+:)\/\/([^:\/\?#]+)(?::(\d+))?(\/[^\?#]*)?(\?[^#]*)?(#.*)?$/);
            }
            if (!match) {
                throw new TypeError("Failed to set 'href' on 'URL': Invalid URL");
            }
            s._protocol = match[1] || '';
            s._hostname = match[2] || '';
            s._port = match[3] || '';
            s._pathname = match[4] || '/';
            s._search = match[5] || '';
            s._hash = match[6] || '';
            s._searchParams = new URLSearchParams(s._search);
            s._rebuild();
        },
        enumerable: true, configurable: true
    });
    Object.defineProperty(URL.prototype, 'origin', {
        get: function() { return _urlThis(this)._origin; },
        enumerable: true, configurable: true
    });
    Object.defineProperty(URL.prototype, 'searchParams', {
        get: function() { return _urlThis(this)._searchParams; },
        enumerable: true, configurable: true
    });
    Object.defineProperty(URL.prototype, 'username', {
        get: function() { return _urlThis(this)._username; },
        set: function(v) { _urlThis(this)._username = String(v); },
        enumerable: true, configurable: true
    });
    Object.defineProperty(URL.prototype, 'password', {
        get: function() { return _urlThis(this)._password; },
        set: function(v) { _urlThis(this)._password = String(v); },
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
