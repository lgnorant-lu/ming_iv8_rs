//! document.cookie, document.referrer, document.hidden, document.visibilityState
//!
//! document.cookie (v0.8.72 Track B): enhanced with attribute parsing
//! (Path, Secure, SameSite, expires, max-age, domain, httpOnly) and
//! visibility filtering (Path prefix match, Secure context). See
//! `crates/iv8-core/src/dom/cookie_jar.rs` for the canonical Rust model.

/// JS shim for document properties.
pub const DOCUMENT_PROPS_JS: &str = r#"
(function() {
    if (typeof document === 'undefined') return;

    // IV8 logical mode treats all contexts as secure.
    if (window.__iv8IsSecureContext === undefined) {
        window.__iv8IsSecureContext = true;
    }

    // document.cookie (v0.8.72 Track B: attribute parsing + filtering)
    var _cookies = window._iv8CookieStore || (window._iv8CookieStore = {});

    function _cookieValue(rec) {
        if (typeof rec === 'string') return rec;
        if (rec && typeof rec === 'object' && rec.v !== undefined) return rec.v;
        return '';
    }

    function _cookieVisible(rec) {
        if (typeof rec === 'string') return true;    // legacy: no attributes
        if (!rec || typeof rec !== 'object') return true;
        // Path filtering (prefix match)
        if (rec.path && rec.path !== '/') {
            var docPath = '/';
            try { docPath = document.location ? document.location.pathname : '/'; } catch(e) {}
            if (docPath.indexOf(rec.path) !== 0) return false;
        }
        // Secure filtering
        if (rec.secure) {
            var isSecure = window.__iv8IsSecureContext;
            if (isSecure !== true) return false;
        }
        return true;
    }

    Object.defineProperty(document, 'cookie', {
        get: function() {
            var parts = [];
            for (var k in _cookies) {
                if (!_cookies.hasOwnProperty(k)) continue;
                var rec = _cookies[k];
                if (!_cookieVisible(rec)) continue;
                parts.push(k + '=' + _cookieValue(rec));
            }
            return parts.join('; ');
        },
        set: function(val) {
            var str = String(val);
            var parts = str.split(';');
            var kv = parts[0].split('=');
            if (kv.length < 2) return;
            var name = kv[0].trim();
            var value = kv.slice(1).join('=').trim();

            // Parse attributes from remaining segments
            var attrs = {};
            var hasAttrs = false;
            for (var i = 1; i < parts.length; i++) {
                var attr = parts[i].trim();
                var lower = attr.toLowerCase();
                if (lower === 'secure')        { attrs.secure = true; hasAttrs = true; }
                else if (lower === 'httponly') { attrs.httpOnly = true; hasAttrs = true; }
                else if (lower.indexOf('path=') === 0) {
                    attrs.path = attr.substring(5); hasAttrs = true;
                }
                else if (lower.indexOf('domain=') === 0) {
                    attrs.domain = attr.substring(7); hasAttrs = true;
                }
                else if (lower.indexOf('samesite=') === 0) {
                    attrs.sameSite = attr.substring(9); hasAttrs = true;
                }
                else if (lower.indexOf('expires=') === 0) {
                    attrs.expires = attr.substring(8); hasAttrs = true;
                }
                else if (lower.indexOf('max-age=') === 0) {
                    var ma = parseInt(attr.substring(8), 10);
                    if (!isNaN(ma)) {
                        if (ma <= 0) { delete _cookies[name]; return; }
                        attrs.maxAge = ma; hasAttrs = true;
                    }
                }
            }

            if (hasAttrs) {
                attrs.v = value;
                _cookies[name] = attrs;
            } else {
                _cookies[name] = value;
            }
        },
        enumerable: true,
        configurable: true,
    });

    // document.referrer
    if (!('referrer' in document)) {
        Object.defineProperty(document, 'referrer', {
            value: '',
            writable: true,
            enumerable: true,
            configurable: true,
        });
    }

    // document.hidden
    Object.defineProperty(document, 'hidden', {
        value: false,
        writable: true,
        enumerable: true,
        configurable: true,
    });

    // document.visibilityState
    Object.defineProperty(document, 'visibilityState', {
        value: 'visible',
        writable: true,
        enumerable: true,
        configurable: true,
    });

    // document.readyState
    Object.defineProperty(document, 'readyState', {
        value: 'complete',
        writable: true,
        enumerable: true,
        configurable: true,
    });

    // document.domain
    if (!('domain' in document)) {
        Object.defineProperty(document, 'domain', {
            value: location.hostname || '',
            writable: true,
            enumerable: true,
            configurable: true,
        });
    }

    // document.URL
    if (!('URL' in document) || document.URL === undefined) {
        Object.defineProperty(document, 'URL', {
            get: function() { return location.href; },
            enumerable: true,
            configurable: true,
        });
    }

    // document.title (reads/writes <title> element text)
    Object.defineProperty(document, 'title', {
        get: function() {
            var titleEl = document.querySelector ? document.querySelector('title') : null;
            return titleEl ? (titleEl.textContent || '') : '';
        },
        set: function(val) {
            var titleEl = document.querySelector ? document.querySelector('title') : null;
            if (titleEl) { titleEl.textContent = String(val); }
        },
        enumerable: true,
        configurable: true,
    });

    // document.documentURI (alias for URL)
    if (!('documentURI' in document) || document.documentURI === undefined) {
        Object.defineProperty(document, 'documentURI', {
            get: function() { return location.href; },
            enumerable: true,
            configurable: true,
        });
    }

    // document DOM methods (stubs for anti-bot compatibility)
    if (!document.createEvent) {
        document.createEvent = function(type) {
            var e = {};
            e.type = '';
            e.bubbles = false;
            e.cancelable = false;
            e.initEvent = function(t, b, c) { e.type = t; e.bubbles = b; e.cancelable = c; };
            e.initMouseEvent = function(t) { e.type = t; };
            e.initCustomEvent = function(t, b, c, d) { e.type = t; e.bubbles = b; e.cancelable = c; e.detail = d; };
            return e;
        };
    }
    if (!document.dispatchEvent) {
        document.dispatchEvent = function(event) { return true; };
    }
    if (!document.addEventListener) {
        document.addEventListener = function(type, listener, options) {};
    }
    if (!document.removeEventListener) {
        document.removeEventListener = function(type, listener, options) {};
    }
    if (!document.createTextNode) {
        document.createTextNode = function(data) {
            return { nodeType: 3, textContent: data, data: data, nodeName: '#text' };
        };
    }
    if (!document.createComment) {
        document.createComment = function(data) {
            return { nodeType: 8, textContent: data, data: data, nodeName: '#comment' };
        };
    }
    if (!document.createDocumentFragment) {
        document.createDocumentFragment = function() {
            return { nodeType: 11, nodeName: '#document-fragment', childNodes: [], appendChild: function(n) { this.childNodes.push(n); return n; }, children: [] };
        };
    }
    if (!document.importNode) {
        document.importNode = function(node, deep) { return node; };
    }
    if (!document.adoptNode) {
        document.adoptNode = function(node) { return node; };
    }
    if (!document.createNodeIterator) {
        document.createNodeIterator = function() { return { nextNode: function() { return null; }, detach: function() {} }; };
    }
    if (!document.execCommand) {
        document.execCommand = function() { return false; };
    }
    if (!document.queryCommandEnabled) {
        document.queryCommandEnabled = function() { return false; };
    }
    if (!document.queryCommandState) {
        document.queryCommandState = function() { return false; };
    }
    if (!document.queryCommandValue) {
        document.queryCommandValue = function() { return ''; };
    }
    if (!document.getSelection) {
        document.getSelection = function() { return null; };
    }
    if (!document.exitFullscreen) {
        document.exitFullscreen = function() { return Promise.resolve(); };
    }
    if (!document.exitPointerLock) {
        document.exitPointerLock = function() {};
    }
    if (!document.fonts) {
        document.fonts = { ready: Promise.resolve(), check: function() { return true; }, load: function() { return Promise.resolve([]); }, forEach: function() {} };
    }
    if (!document.timeline) {
        document.timeline = { currentTime: performance.now() };
    }
    if (!document.scrollingElement) {
        Object.defineProperty(document, 'scrollingElement', { get: function() { return document.body || null; }, configurable: true });
    }
    if (!document.currentScript) {
        Object.defineProperty(document, 'currentScript', { get: function() { return null; }, configurable: true });
    }
    if (!document.implementation) {
        document.implementation = { createHTMLDocument: function(t) { return document; }, hasFeature: function() { return true; } };
    }
    if (!document.defaultView) {
        Object.defineProperty(document, 'defaultView', { get: function() { return window; }, configurable: true });
    }
    if (!document.ownerDocument) {
        Object.defineProperty(document, 'ownerDocument', { get: function() { return null; }, configurable: true });
    }
    if (!document.baseURI) {
        Object.defineProperty(document, 'baseURI', { get: function() { return location.href; }, configurable: true });
    }
    if (!document.characterSet) {
        Object.defineProperty(document, 'characterSet', { get: function() { return 'UTF-8'; }, configurable: true });
    }
    if (!document.contentType) {
        Object.defineProperty(document, 'contentType', { get: function() { return 'text/html'; }, configurable: true });
    }
    if (!document.compatMode) {
        Object.defineProperty(document, 'compatMode', { get: function() { return 'CSS1Compat'; }, configurable: true });
    }
    if (!document.lastModified) {
        Object.defineProperty(document, 'lastModified', { get: function() { return new Date().toLocaleString(); }, configurable: true });
    }
    if (!document.fullscreenEnabled) {
        Object.defineProperty(document, 'fullscreenEnabled', { get: function() { return false; }, configurable: true });
    }
    if (!document.pictureInPictureEnabled) {
        Object.defineProperty(document, 'pictureInPictureEnabled', { get: function() { return false; }, configurable: true });
    }

    // MimeTypeArray / PluginArray prototype tags
    // Real Chrome: Object.prototype.toString.call(mimeTypes) === '[object MimeTypeArray]'
    if (typeof navigator !== 'undefined' && navigator.mimeTypes) {
        try {
            var mta = navigator.mimeTypes;
            Object.defineProperty(mta, Symbol.toStringTag, {value: 'MimeTypeArray', configurable: true});
            if (!mta.item) mta.item = function(i) { return this[i] || null; };
            if (!mta.namedItem) mta.namedItem = function(n) { for (var i=0;i<this.length;i++) if (this[i].type===n) return this[i]; return null; };
        } catch(e) {}
    }
    if (typeof navigator !== 'undefined' && navigator.plugins) {
        try {
            var pa = navigator.plugins;
            Object.defineProperty(pa, Symbol.toStringTag, {value: 'PluginArray', configurable: true});
            if (!pa.item) pa.item = function(i) { return this[i] || null; };
            if (!pa.namedItem) pa.namedItem = function(n) { for (var i=0;i<this.length;i++) if (this[i].name===n) return this[i]; return null; };
            if (!pa.refresh) pa.refresh = function() {};
            for (var i = 0; i < pa.length; i++) {
                if (pa[i] && typeof pa[i] === 'object') {
                    try {
                        Object.defineProperty(pa[i], Symbol.toStringTag, {value: 'Plugin', configurable: true});
                        if (!pa[i].item) pa[i].item = function(j) { return this[j] || null; };
                    } catch(e) {}
                }
            }
        } catch(e) {}
    }

    // Inject PDF Plugin items if plugins/mimeTypes exist but are empty
    if (typeof navigator !== 'undefined' && navigator.plugins && navigator.plugins.length === 0) {
        try {
            var _m1 = { type: 'application/pdf', suffixes: 'pdf', description: 'Portable Document Format', enabledPlugin: null };
            Object.defineProperty(_m1, Symbol.toStringTag, { value: 'MimeType', configurable: true });
            var _m2 = { type: 'text/pdf', suffixes: 'pdf', description: 'Portable Document Format', enabledPlugin: null };
            Object.defineProperty(_m2, Symbol.toStringTag, { value: 'MimeType', configurable: true });
            if (navigator.mimeTypes && navigator.mimeTypes.length === 0) {
                navigator.mimeTypes[0] = _m1; navigator.mimeTypes[1] = _m2;
                Object.defineProperty(navigator.mimeTypes, 'length', { value: 2, writable: true, configurable: true });
            }
            var _pls = [
                { name: 'PDF Viewer', filename: 'internal-pdf-viewer', description: 'Portable Document Format', 0: _m1, 1: _m2, length: 2 },
                { name: 'Chrome PDF Viewer', filename: 'internal-pdf-viewer', description: 'Portable Document Format', 0: _m1, 1: _m2, length: 2 },
                { name: 'Chromium PDF Viewer', filename: 'internal-pdf-viewer', description: 'Portable Document Format', 0: _m1, 1: _m2, length: 2 },
                { name: 'Microsoft Edge PDF Viewer', filename: 'internal-pdf-viewer', description: 'Portable Document Format', 0: _m1, 1: _m2, length: 2 },
                { name: 'WebKit built-in PDF', filename: 'internal-pdf-viewer', description: 'Portable Document Format', 0: _m1, 1: _m2, length: 2 },
            ];
            for (var i = 0; i < _pls.length; i++) {
                navigator.plugins[i] = _pls[i];
                Object.defineProperty(_pls[i], Symbol.toStringTag, { value: 'Plugin', configurable: true });
            }
            Object.defineProperty(navigator.plugins, 'length', { value: 5, writable: true, configurable: true });
        } catch(e) {}
    }

    // video.canPlayType / audio.canPlayType: return "probably" for H.264/AAC
    // Must override on all prototypes that shadow HTMLMediaElement.prototype
    // (codegen creates HTMLAudioElement/HTMLVideoElement with own canPlayType)
    try {
        var _mediaCanPlay = HTMLMediaElement.prototype.canPlayType;
        var _canPlayOverride = function(type) {
            if (/avc1|mp4a|aac|h\.264|h264/i.test(type)) return 'probably';
            return _mediaCanPlay.call(this, type);
        };
        HTMLMediaElement.prototype.canPlayType = _canPlayOverride;
        if (typeof HTMLAudioElement !== 'undefined' && HTMLAudioElement.prototype.canPlayType !== _canPlayOverride) {
            HTMLAudioElement.prototype.canPlayType = _canPlayOverride;
        }
        if (typeof HTMLVideoElement !== 'undefined' && HTMLVideoElement.prototype.canPlayType !== _canPlayOverride) {
            HTMLVideoElement.prototype.canPlayType = _canPlayOverride;
        }
    } catch(e) {}

    // window.Image constructor (standard DOM API)
    if (typeof Image === 'undefined') {
        window.Image = function Image(width, height) {
            var img = document.createElement('img');
            if (width !== undefined) img.width = width;
            if (height !== undefined) img.height = height;
            return img;
        };
    }

    // requestIdleCallback / cancelIdleCallback
    if (typeof requestIdleCallback === 'undefined') {
        globalThis.requestIdleCallback = function requestIdleCallback(callback, options) {
            var timeout = (options && options.timeout) || 50;
            return setTimeout(function() {
                var deadline = {
                    timeRemaining: function() { return Math.max(0, 50 - (Date.now() % 50)); },
                    didTimeout: false,
                };
                callback(deadline);
            }, 1);
        };
        globalThis.cancelIdleCallback = function cancelIdleCallback(id) {
            clearTimeout(id);
        };
    }

})();
"#;
