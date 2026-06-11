//! document.cookie, document.referrer, document.hidden, document.visibilityState
//!
//! These are standard document properties that anti-bot scripts check.

/// JS shim for document properties.
pub const DOCUMENT_PROPS_JS: &str = r#"
(function() {
    if (typeof document === 'undefined') return;

    // document.cookie (read/write string)
    var _cookie = '';
    Object.defineProperty(document, 'cookie', {
        get: function() { return _cookie; },
        set: function(val) {
            // Simple cookie append (real impl would parse name=value;expires=...)
            if (_cookie) _cookie += '; ';
            var parts = String(val).split(';');
            _cookie += parts[0]; // Only keep name=value part
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

    // navigator.sendBeacon stub
    if (typeof navigator !== 'undefined' && !navigator.sendBeacon) {
        try {
            Object.defineProperty(navigator, 'sendBeacon', {
                value: function sendBeacon(url, data) { return true; },
                writable: true, configurable: true, enumerable: true,
            });
        } catch(e) {}
    }

    // navigator.getBattery stub
    if (typeof navigator !== 'undefined' && !navigator.getBattery) {
        try {
            Object.defineProperty(navigator, 'getBattery', {
                value: function getBattery() {
                    return Promise.resolve({
                        charging: true, chargingTime: 0,
                        dischargingTime: Infinity, level: 1.0,
                        addEventListener: function() {}, removeEventListener: function() {},
                    });
                },
                writable: true, configurable: true, enumerable: true,
            });
        } catch(e) {}
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
