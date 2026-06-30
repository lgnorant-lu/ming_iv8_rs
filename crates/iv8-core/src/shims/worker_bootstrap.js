(function(profile) {
    'use strict';

    // idlharness checks self instanceof DedicatedWorkerGlobalScope.
    // Setting __proto__ triggers V8 OOM from shape transitions.
    // Symbol.hasInstance override also causes issues.
    // Current approach: accept the "Unexpected global object" FAIL
    // and focus on interface property tests that don't require
    // the global object check to pass.

    var _profile = profile;

    Object.defineProperty(self, 'self', {
        value: self,
        enumerable: true,
        configurable: true,
        writable: true
    });

    var _navigator = Object.create(null);
    Object.defineProperty(_navigator, 'userAgent', {
        get: function() { return _profile.userAgent; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(_navigator, 'platform', {
        get: function() { return _profile.platform; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(_navigator, 'language', {
        get: function() { return _profile.language; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(_navigator, 'languages', {
        get: function() { return _profile.languages; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(_navigator, 'hardwareConcurrency', {
        get: function() { return _profile.hardwareConcurrency; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(_navigator, 'deviceMemory', {
        get: function() { return _profile.deviceMemory; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(_navigator, 'onLine', {
        get: function() { return _profile.onLine; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(_navigator, 'vendor', {
        get: function() { return _profile.vendor; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(_navigator, 'product', {
        get: function() { return _profile.product; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(_navigator, 'appName', {
        get: function() { return _profile.appName; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(_navigator, 'appVersion', {
        get: function() { return _profile.appVersion; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(_navigator, 'appCodeName', {
        get: function() { return _profile.appCodeName; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(_navigator, 'cookieEnabled', {
        get: function() { return _profile.cookieEnabled; },
        enumerable: true,
        configurable: true
    });
    Object.defineProperty(_navigator, 'pdfViewerEnabled', {
        get: function() { return _profile.pdfViewerEnabled; },
        enumerable: true,
        configurable: true
    });
    if (_profile.userAgentData) {
        Object.defineProperty(_navigator, 'userAgentData', {
            get: function() { return _profile.userAgentData; },
            enumerable: true,
            configurable: true
        });
    }
    _navigator.toString = function() { return '[object WorkerNavigator]'; };
    Object.defineProperty(self, 'navigator', {
        value: _navigator,
        enumerable: true,
        configurable: true
    });

    var _location = Object.create(null);
    Object.defineProperty(_location, 'href', {
        get: function() { return _profile.workerUrl; },
        enumerable: true, configurable: true
    });
    Object.defineProperty(_location, 'origin', {
        get: function() { return _profile.workerOrigin; },
        enumerable: true, configurable: true
    });
    Object.defineProperty(_location, 'protocol', {
        get: function() { return _profile.workerProtocol; },
        enumerable: true, configurable: true
    });
    Object.defineProperty(_location, 'host', {
        get: function() { return _profile.workerHost; },
        enumerable: true, configurable: true
    });
    Object.defineProperty(_location, 'hostname', {
        get: function() { return _profile.workerHostname; },
        enumerable: true, configurable: true
    });
    Object.defineProperty(_location, 'port', {
        get: function() { return _profile.workerPort; },
        enumerable: true, configurable: true
    });
    Object.defineProperty(_location, 'pathname', {
        get: function() { return _profile.workerPathname; },
        enumerable: true, configurable: true
    });
    Object.defineProperty(_location, 'search', {
        get: function() { return ''; },
        enumerable: true, configurable: true
    });
    Object.defineProperty(_location, 'hash', {
        get: function() { return ''; },
        enumerable: true, configurable: true
    });
    _location.toString = function() { return _profile.workerUrl; };
    Object.defineProperty(self, 'location', {
        value: _location,
        enumerable: true,
        configurable: true
    });

    var _onmessage = null;
    Object.defineProperty(self, 'onmessage', {
        get: function() { return _onmessage; },
        set: function(fn) {
            if (typeof fn === 'function' || fn === null) {
                _onmessage = fn;
            }
        },
        enumerable: true,
        configurable: true
    });
    var _onmessageerror = null;
    Object.defineProperty(self, 'onmessageerror', {
        get: function() { return _onmessageerror; },
        set: function(fn) {
            if (typeof fn === 'function' || fn === null) {
                _onmessageerror = fn;
            }
        },
        enumerable: true,
        configurable: true
    });
    var _onerror = null;
    Object.defineProperty(self, 'onerror', {
        get: function() { return _onerror; },
        set: function(fn) {
            if (typeof fn === 'function' || fn === null) {
                _onerror = fn;
            }
        },
        enumerable: true,
        configurable: true
    });

    self.postMessage = function(message, transfer) {
        if (typeof __iv8WorkerPostMessage !== 'function') {
            throw new TypeError('__iv8WorkerPostMessage is not available');
        }
        __iv8WorkerPostMessage(message);
    };

    self.close = function() {
        if (typeof __iv8WorkerClose === 'function') {
            __iv8WorkerClose();
        }
    };

    self.importScripts = function() {
        for (var i = 0; i < arguments.length; i++) {
            var url = arguments[i];
            if (typeof __iv8ImportScript === 'function') {
                var source = __iv8ImportScript(url);
                if (source) {
                    try {
                        (0, eval)(source);
                    } catch (e) {
                        throw e;
                    }
                }
            }
        }
    };

    self.console = {
        log: function() {},
        warn: function() {},
        error: function() {},
        info: function() {},
        debug: function() {},
        trace: function() {},
        dir: function() {},
        table: function() {},
        group: function() {},
        groupEnd: function() {},
        time: function() {},
        timeEnd: function() {},
        assert: function() {},
        count: function() {},
        clear: function() {}
    };

    var _timerId = 0;
    var _timers = Object.create(null);
    self.setTimeout = function(fn, delay) {
        var id = ++_timerId;
        if (typeof fn === 'function') {
            try { fn(); } catch (e) {}
        }
        return id;
    };
    self.clearTimeout = function(id) { delete _timers[id]; };
    self.setInterval = function(fn, delay) {
        var id = ++_timerId;
        if (typeof fn === 'function') {
            try { fn(); } catch (e) {}
        }
        return id;
    };
    self.clearInterval = function(id) { delete _timers[id]; };
    self.queueMicrotask = function(fn) {
        if (typeof fn === 'function') {
            Promise.resolve().then(fn);
        }
    };

    self.fetch = function(url, options) {
        return Promise.reject(new TypeError('fetch is not available in worker'));
    };

    self.crypto = {
        getRandomValues: function(arr) {
            for (var i = 0; i < arr.length; i++) {
                arr[i] = Math.floor(Math.random() * 256);
            }
            return arr;
        },
        randomUUID: function() {
            return '00000000-0000-4000-8000-000000000000'.replace(/0/g, function() {
                return Math.floor(Math.random() * 16).toString(16);
            });
        },
        subtle: {
            digest: function(algo, data) {
                return Promise.reject(new TypeError('crypto.subtle not available'));
            }
        }
    };

    self.dispatchEvent = function(event) { return true; };
    self.addEventListener = function(type, listener) {};
    self.removeEventListener = function(type, listener) {};

    self.name = _profile.name || '';

    try {
        Object.defineProperty(self, 'isWorker', {
            value: true,
            enumerable: false,
            configurable: false,
            writable: false
        });
    } catch (e) {}

    if (typeof window !== 'undefined') {
        try { delete self.window; } catch (e) {}
        try { Object.defineProperty(self, 'window', { get: function() { return undefined; }, configurable: true }); } catch (e) {}
    }
    if (typeof document !== 'undefined') {
        try { delete self.document; } catch (e) {}
        try { Object.defineProperty(self, 'document', { get: function() { return undefined; }, configurable: true }); } catch (e) {}
    }
})(__iv8WorkerProfile);
