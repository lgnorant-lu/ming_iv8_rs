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
        Object.defineProperty(window, '__iv8IsSecureContext', {
            value: true, writable: true, configurable: true, enumerable: false,
        });
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
        // Path filtering (RFC 6265 prefix match)
        if (rec.path && rec.path !== '/') {
            var docPath = '/';
            try { docPath = document.location ? document.location.pathname : '/'; } catch(e) {}
            if (!_pathMatches(docPath, rec.path)) return false;
        }
        // Secure filtering
        if (rec.secure) {
            var isSecure = window.__iv8IsSecureContext;
            if (isSecure !== true) return false;
        }
        return true;
    }

    function _pathMatches(docPath, cookiePath) {
        if (docPath === cookiePath) return true;
        if (docPath.indexOf(cookiePath) !== 0) return false;
        var next = docPath.charAt(cookiePath.length);
        return next === '/' || next === '';
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
    Object.defineProperty(document, 'referrer', {
        value: '',
        writable: true,
        enumerable: true,
        configurable: true,
    });

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
    Object.defineProperty(document, 'domain', {
        value: location.hostname || '',
        writable: true,
        enumerable: true,
        configurable: true,
    });

    // document.URL
    Object.defineProperty(document, 'URL', {
        get: function() { return location.href; },
        enumerable: true,
        configurable: true,
    });

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
    Object.defineProperty(document, 'documentURI', {
        get: function() { return location.href; },
        enumerable: true,
        configurable: true,
    });

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
            if (typeof Text !== 'undefined' && Text.prototype) {
                return Object.create(Text.prototype);
            }
            return { nodeType: 3, textContent: data, data: data, nodeName: '#text' };
        };
    }
    if (!document.createComment) {
        document.createComment = function(data) {
            if (typeof Comment !== 'undefined' && Comment.prototype) {
                return Object.create(Comment.prototype);
            }
            return { nodeType: 8, textContent: data, data: data, nodeName: '#comment' };
        };
    }
    if (!document.createDocumentFragment) {
        document.createDocumentFragment = function() {
            // Use codegen DocumentFragment constructor's prototype if available.
            // This ensures the fragment has the correct prototype chain:
            // fragment → DocumentFragment.prototype → Node.prototype → EventTarget.prototype
            if (typeof DocumentFragment !== 'undefined' && DocumentFragment.prototype) {
                var frag = Object.create(DocumentFragment.prototype);
                return frag;
            }
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
        var _fontPrefs = (typeof globalThis.__iv8FontPrefs === 'object' && globalThis.__iv8FontPrefs) ? globalThis.__iv8FontPrefs : {};
        var _fontFamilies = _fontPrefs.families || [];
        var _fontSet = {
            ready: Promise.resolve(),
            status: 'loaded',
            onloading: null,
            onloadingdone: null,
            onloadingerror: null,
            size: _fontFamilies.length,
            check: function(font, text) {
                // Return true if the font is in the profile's family list
                if (!font) return true;
                var m = font.match(/["']?([^"']+)["']?/);
                var family = m ? m[1].toLowerCase() : font.toLowerCase();
                if (family.indexOf('sans-serif') !== -1 || family.indexOf('serif') !== -1 ||
                    family.indexOf('monospace') !== -1 || family.indexOf('cursive') !== -1 ||
                    family.indexOf('fantasy') !== -1) return true;
                return _fontFamilies.some(function(f) { return f.toLowerCase() === family; });
            },
            load: function(font, text) { return Promise.resolve([]); },
            forEach: function(cb) {
                _fontFamilies.forEach(function(f, i) {
                    cb({ family: f, status: 'loaded', weight: 'normal', style: 'normal', display: 'auto' }, i, _fontSet);
                });
            },
            values: function() { return _fontFamilies.map(function(f) { return { family: f, status: 'loaded' }; }).values(); },
            entries: function() { return _fontFamilies.map(function(f, i) { return [i, { family: f, status: 'loaded' }]; }).entries(); },
            keys: function() { return _fontFamilies.keys(); },
            add: function(fontFace) {},
            delete: function(fontFace) {},
            clear: function() {},
        };
        document.fonts = _fontSet;
    }
    if (!document.timeline) {
        document.timeline = { currentTime: performance.now() };
    }
    // Properties below are in EXCLUDED_ATTRIBUTES — codegen no longer
    // installs them, so guards are dead code. Always install to ensure
    // the shim is the single source of truth.
    Object.defineProperty(document, 'scrollingElement', { get: function() { return document.body || null; }, configurable: true });
    Object.defineProperty(document, 'currentScript', { get: function() { return null; }, configurable: true });
    // Override document.createRange to return object with correct prototype
    // codegen callback returns Object::new() with Object.prototype, not Range.prototype
    if (typeof Range !== 'undefined') {
        document.createRange = function createRange() {
            return Object.create(Range.prototype);
        };
    }
    // Override document.createEvent to return objects with correct prototypes
    if (typeof Event !== 'undefined') {
        var origCreateEvent = document.createEvent;
        document.createEvent = function createEvent(type) {
            var ctorMap = { Event: Event, CustomEvent: CustomEvent, MouseEvent: MouseEvent, UIEvent: UIEvent, KeyboardEvent: KeyboardEvent, AnimationEvent: AnimationEvent, TransitionEvent: TransitionEvent, MessageEvent: MessageEvent, DragEvent: DragEvent, BeforeUnloadEvent: BeforeUnloadEvent, HashChangeEvent: HashChangeEvent, PageTransitionEvent: PageTransitionEvent, PopStateEvent: PopStateEvent, StorageEvent: StorageEvent, SubmitEvent: SubmitEvent, ToggleEvent: ToggleEvent, CloseWatcher: CloseWatcher, PromiseRejectionEvent: PromiseRejectionEvent, ErrorEvent: ErrorEvent, FormDataEvent: FormDataEvent, DragEvent: DragEvent };
            var Ctor = ctorMap[type];
            if (Ctor) {
                try { return new Ctor(type); } catch(e) {}
            }
            return new Event(type);
        };
    }
    if (!document.implementation) {
        var implProto = (typeof DOMImplementation !== 'undefined') ? DOMImplementation.prototype : Object.prototype;
        var impl = Object.create(implProto);
        Object.defineProperty(impl, 'createHTMLDocument', { value: function(t) { return document; }, writable: true, configurable: true, enumerable: true });
        Object.defineProperty(impl, 'hasFeature', { value: function() { return true; }, writable: true, configurable: true, enumerable: true });
        // createDocument: return an XMLDocument-like object with Document prototype
        // codegen callback returns Object::new() without proper prototype
        Object.defineProperty(impl, 'createDocument', { value: function createDocument(ns, name, doctype) {
            var docProto = (typeof XMLDocument !== 'undefined') ? XMLDocument.prototype : (typeof Document !== 'undefined' ? Document.prototype : Object.prototype);
            var doc = Object.create(docProto);
            Object.defineProperty(doc, Symbol.toStringTag, { value: 'XMLDocument', writable: true, configurable: true, enumerable: false });
            // Override createElementNS to return Element with correct prototype
            // codegen callback returns Object::new() without Element.prototype
            // Use document.createElement internally to get a real Element instance
            doc.createElementNS = function createElementNS(ns, qname) {
                return document.createElement(qname || 'div');
            };
            doc.createElement = function createElement(tag) {
                return document.createElement(tag || 'div');
            };
            doc.createProcessingInstruction = function createProcessingInstruction(target, data) {
                var piProto = (typeof ProcessingInstruction !== 'undefined') ? ProcessingInstruction.prototype : Object.prototype;
                return Object.create(piProto);
            };
            doc.createAttribute = function createAttribute(name) {
                var attrProto = (typeof Attr !== 'undefined') ? Attr.prototype : Object.prototype;
                return Object.create(attrProto);
            };
            doc.createCDATASection = function createCDATASection(data) {
                var textProto = (typeof Text !== 'undefined') ? Text.prototype : Object.prototype;
                return Object.create(textProto);
            };
            doc.createTextNode = function createTextNode(data) {
                var textProto = (typeof Text !== 'undefined') ? Text.prototype : Object.prototype;
                return Object.create(textProto);
            };
            doc.createComment = function createComment(data) {
                var commentProto = (typeof Comment !== 'undefined') ? Comment.prototype : Object.prototype;
                return Object.create(commentProto);
            };
            return doc;
        }, writable: true, configurable: true, enumerable: true });
        // createDocumentType: return a DocumentType-like object
        Object.defineProperty(impl, 'createDocumentType', { value: function createDocumentType(qname, publicId, systemId) {
            var dtProto = (typeof DocumentType !== 'undefined') ? DocumentType.prototype : Object.prototype;
            var dt = Object.create(dtProto);
            return dt;
        }, writable: true, configurable: true, enumerable: true });
        document.implementation = impl;
    }
    Object.defineProperty(document, 'defaultView', { get: function() { return window; }, configurable: true });
    Object.defineProperty(document, 'ownerDocument', { get: function() { return null; }, configurable: true });
    Object.defineProperty(document, 'baseURI', { get: function() { return location.href; }, configurable: true });
    Object.defineProperty(document, 'characterSet', { get: function() { return 'UTF-8'; }, configurable: true });
    Object.defineProperty(document, 'charset', { get: function() { return 'UTF-8'; }, configurable: true });
    Object.defineProperty(document, 'inputEncoding', { get: function() { return 'UTF-8'; }, configurable: true });
    Object.defineProperty(document, 'designMode', { get: function() { return 'off'; }, set: function() {}, configurable: true });
    Object.defineProperty(document, 'contentType', { get: function() { return 'text/html'; }, configurable: true });
    Object.defineProperty(document, 'compatMode', { get: function() { return 'CSS1Compat'; }, configurable: true });
    // Cache lastModified at install time using a non-Intl date format to
    // avoid Intl.DateTimeFormat re-entrancy OOM. Real Chrome formats this
    // as "MM/DD/YYYY HH:MM:SS" from the document's last-modified header.
    var _now = new Date();
    var _iv8LastModified = (_now.getMonth()+1) + '/' + _now.getDate() + '/' +
        _now.getFullYear() + ' ' + _now.getHours() + ':' +
        ('0'+_now.getMinutes()).slice(-2) + ':' + ('0'+_now.getSeconds()).slice(-2);
    Object.defineProperty(document, 'lastModified', { get: function() { return _iv8LastModified; }, configurable: true });
    Object.defineProperty(document, 'fullscreenEnabled', { get: function() { return false; }, configurable: true });
    Object.defineProperty(document, 'pictureInPictureEnabled', { get: function() { return false; }, configurable: true });

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
    // Guard against re-wrap accumulation (DOCUMENT_PROPS_JS may run multiple times).
    try {
        if (HTMLMediaElement.prototype.__iv8CanPlayPatched) {
            // Already patched — skip to avoid wrapper chain buildup.
        } else {
            var _mediaCanPlay = HTMLMediaElement.prototype.canPlayType;
            var _canPlayOverride = function canPlayType(type) {
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
            Object.defineProperty(HTMLMediaElement.prototype, '__iv8CanPlayPatched', {
                value: true, writable: true, configurable: true, enumerable: false,
            });
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

    // Symbol.toStringTag for global objects — anti-detection fidelity.
    // Real Chrome: Object.prototype.toString.call(window) === '[object Window]'
    // etc. IV8 creates plain V8 objects for these, so they default to
    // '[object Object]'. Set the correct tags and prototype chains.
    try {
        Object.defineProperty(window, Symbol.toStringTag, { value: 'Window', configurable: true });
    } catch(e) {}
    try {
        Object.defineProperty(document, Symbol.toStringTag, { value: 'HTMLDocument', configurable: true });
    } catch(e) {}
    try {
        Object.defineProperty(location, Symbol.toStringTag, { value: 'Location', configurable: true });
    } catch(e) {}
    try {
        if (typeof history !== 'undefined') {
            Object.defineProperty(history, Symbol.toStringTag, { value: 'History', configurable: true });
        }
    } catch(e) {}

    // Prototype chain fixes — IV8 creates plain V8 objects for document,
    // window, location, history, crypto. Real browsers have these as
    // instances of their respective interfaces. Set __proto__ (not
    // setPrototypeOf) to preserve any existing properties.
    // Note: location prototype is NOT set because the Location codegen
    // template has its own href getter that would shadow the injected
    // profile values. This is a known limitation — location instanceof
    // Location remains false until codegen template inheritance is fixed.
    try {
        if (typeof Document !== 'undefined' && Document.prototype) {
            document.__proto__ = Document.prototype;
        }
    } catch(e) {}
    try {
        if (typeof History !== 'undefined' && History.prototype && typeof history !== 'undefined') {
            history.__proto__ = History.prototype;
        }
    } catch(e) {}
    try {
        if (typeof Crypto !== 'undefined' && Crypto.prototype && typeof crypto !== 'undefined') {
            crypto.__proto__ = Crypto.prototype;
        }
    } catch(e) {}

    // crypto.subtle prototype — should be SubtleCrypto instance
    try {
        if (typeof SubtleCrypto !== 'undefined' && SubtleCrypto.prototype && typeof crypto !== 'undefined' && crypto.subtle) {
            crypto.subtle.__proto__ = SubtleCrypto.prototype;
        }
    } catch(e) {}

    // Notification.permission default — real Chrome returns 'default'
    try {
        if (typeof Notification !== 'undefined' && Notification.permission === undefined) {
            Object.defineProperty(Notification, 'permission', { value: 'default', writable: true, configurable: true });
        }
    } catch(e) {}

    // localStorage/sessionStorage constructor name fix
    // JS shim creates them as "StorageStub" — rename to match browser
    try {
        if (typeof localStorage !== 'undefined' && localStorage.constructor && localStorage.constructor.name === 'StorageStub') {
            Object.defineProperty(localStorage.constructor, 'name', { value: 'Storage', configurable: true });
        }
    } catch(e) {}
    try {
        if (typeof sessionStorage !== 'undefined' && sessionStorage.constructor && sessionStorage.constructor.name === 'StorageStub') {
            Object.defineProperty(sessionStorage.constructor, 'name', { value: 'Storage', configurable: true });
        }
    } catch(e) {}

    // localStorage/sessionStorage toString tag
    try {
        if (typeof localStorage !== 'undefined') {
            Object.defineProperty(localStorage, Symbol.toStringTag, { value: 'Storage', configurable: true });
        }
    } catch(e) {}
    try {
        if (typeof sessionStorage !== 'undefined') {
            Object.defineProperty(sessionStorage, Symbol.toStringTag, { value: 'Storage', configurable: true });
        }
    } catch(e) {}

    // XMLHttpRequest toString tag
    try {
        if (typeof XMLHttpRequest !== 'undefined' && XMLHttpRequest.prototype) {
            Object.defineProperty(XMLHttpRequest.prototype, Symbol.toStringTag, { value: 'XMLHttpRequest', configurable: true });
        }
    } catch(e) {}

    // Prototype inheritance chain fixes.
    // Codegen creates FunctionTemplates for each interface but does not
    // set up inheritance (child.prototype.__proto__ = parent.prototype).
    // This causes instanceof checks to fail across the chain.
    //
    // IMPORTANT: Only Document→Node is safe to fix here because Document
    // is a plain V8 object (not from FunctionTemplate). Navigator and
    // other codegen-created prototypes lose their installed accessors
    // when __proto__ is modified — must be fixed in codegen instead.
    try {
        if (typeof Document !== 'undefined' && typeof Node !== 'undefined'
            && Document.prototype && Node.prototype) {
            Document.prototype.__proto__ = Node.prototype;
        }
    } catch(e) {}

    // Symbol.unscopables on DOM prototypes — WPT idlharness checks that
    // these prototypes have @@unscopables. Without it, accessing
    // prototype[Symbol.unscopables] returns undefined, and
    // Object.getOwnPropertyDescriptor(undefined, name) throws
    // "Cannot convert undefined or null to object".
    function _installUnscopables(proto, names) {
        if (!proto) return;
        var obj = Object.create(null);
        for (var i = 0; i < names.length; i++) obj[names[i]] = true;
        try {
            Object.defineProperty(proto, Symbol.unscopables, {
                value: obj, writable: false, configurable: true, enumerable: false,
            });
        } catch(e) {}
    }
    try {
        if (typeof Document !== 'undefined' && Document.prototype) {
            _installUnscopables(Document.prototype, ['prepend', 'append', 'replaceChildren', 'fullscreen']);
        }
    } catch(e) {}
    try {
        if (typeof DocumentFragment !== 'undefined' && DocumentFragment.prototype) {
            _installUnscopables(DocumentFragment.prototype, ['prepend', 'append', 'replaceChildren']);
        }
    } catch(e) {}
    try {
        if (typeof Element !== 'undefined' && Element.prototype) {
            _installUnscopables(Element.prototype, ['prepend', 'append', 'replaceChildren', 'before', 'after', 'replaceWith', 'remove', 'slot']);
        }
    } catch(e) {}
    try {
        if (typeof DocumentType !== 'undefined' && DocumentType.prototype) {
            _installUnscopables(DocumentType.prototype, ['before', 'after', 'replaceWith', 'remove']);
        }
    } catch(e) {}
    try {
        if (typeof CharacterData !== 'undefined' && CharacterData.prototype) {
            _installUnscopables(CharacterData.prototype, ['before', 'after', 'replaceWith', 'remove']);
        }
    } catch(e) {}

    // WindowProperties intermediate prototype — cannot be inserted from JS
    // because Window.prototype has set_immutable_proto() (codegen).
    // Requires codegen change to add WindowProperties template between
    // Window and EventTarget. See TODO.

    // MediaQueryList: wrap matchMedia so returned objects get the correct
    // toStringTag and prototype chain. The window_extras.rs shim returns a
    // plain object. We set __proto__ to MediaQueryList.prototype so that
    // instanceof works, and keep own properties (matches, media, etc.) so
    // they shadow codegen prototype getters.
    try {
        if (typeof matchMedia !== 'undefined' && !matchMedia.__iv8MqlPatched && typeof MediaQueryList !== 'undefined') {
            var _origMatchMedia = matchMedia;
            var _mqlWrapper = function matchMedia(query) {
                var mql = _origMatchMedia.call(this, query);
                if (mql && typeof mql === 'object' && typeof MediaQueryList !== 'undefined') {
                    try {
                        Object.setPrototypeOf(mql, MediaQueryList.prototype);
                    } catch(e) {}
                }
                return mql;
            };
            Object.defineProperty(_mqlWrapper, '__iv8MqlPatched', {
                value: true, writable: true, configurable: true, enumerable: false,
            });
            globalThis.matchMedia = _mqlWrapper;
        }
    } catch(e) {}

    try {
        if (typeof Document !== 'undefined' && Document.prototype) {
            var _docProps = {
                domain: '', referrer: '', cookie: '', lastModified: '',
                readyState: 'complete', title: '', currentScript: null,
                defaultView: null, hidden: false, visibilityState: 'visible',
                URL: '', documentURI: '', compatMode: 'CSS1Compat',
                characterSet: 'UTF-8', charset: 'UTF-8', contentType: 'text/html',
                fullscreenEnabled: false, fullscreenElement: null,
                scrollingElement: null
            };
            // readonly properties — no setter (setter must be undefined per WebIDL)
            var _docReadonly = {
                referrer: true, lastModified: true, readyState: true,
                currentScript: true, defaultView: true, hidden: true,
                visibilityState: true, URL: true, documentURI: true,
                compatMode: true, characterSet: true, charset: true,
                contentType: true, fullscreenEnabled: true, fullscreenElement: true,
                scrollingElement: true
            };
            Object.keys(_docProps).forEach(function(prop) {
                if (prop in Document.prototype) return;
                var dv = _docProps[prop];
                var docProto = Document.prototype;
                var isReadonly = _docReadonly[prop];
                var desc = {
                    get: function() {
                        if (this === null || this === undefined) throw new TypeError('Illegal invocation');
                        if (this === docProto) return dv;
                        var cur = Object.getPrototypeOf(this);
                        var found = false;
                        while (cur) { if (cur === docProto) { found = true; break; } cur = Object.getPrototypeOf(cur); }
                        if (!found) throw new TypeError('Illegal invocation');
                        var ownDesc = Object.getOwnPropertyDescriptor(this, prop);
                        if (ownDesc && 'value' in ownDesc) return ownDesc.value;
                        if (this['_' + prop] !== undefined) return this['_' + prop];
                        if (prop === 'defaultView') return typeof window !== 'undefined' ? window : null;
                        if (prop === 'URL' || prop === 'documentURI') return typeof location !== 'undefined' ? location.href : '';
                        return dv;
                    },
                    enumerable: true,
                    configurable: true
                };
                if (!isReadonly) {
                    desc.set = function(v) {
                        if (this === null || this === undefined) throw new TypeError('Illegal invocation');
                        this['_' + prop] = v;
                    };
                }
                Object.defineProperty(docProto, prop, desc);
            });
        }
    } catch(e) {}

    try {
        if (typeof HTMLElement !== 'undefined' && HTMLElement.prototype) {
            var _heProps = {
                innerText: '', offsetTop: 0, offsetLeft: 0,
                offsetWidth: 0, offsetHeight: 0,
                clientTop: 0, clientLeft: 0,
                clientWidth: 0, clientHeight: 0,
                scrollTop: 0, scrollLeft: 0,
                scrollWidth: 0, scrollHeight: 0
            };
            var _heReadonly = {
                offsetTop: true, offsetLeft: true, offsetWidth: true, offsetHeight: true,
                clientTop: true, clientLeft: true, clientWidth: true, clientHeight: true,
                scrollWidth: true, scrollHeight: true
            };
            Object.keys(_heProps).forEach(function(prop) {
                var dv = _heProps[prop];
                var proto = HTMLElement.prototype;
                var isReadonly = _heReadonly[prop];
                var desc = {
                    get: function() {
                        if (this === null || this === undefined) throw new TypeError('Illegal invocation');
                        if (this === proto) return dv;
                        var cur = Object.getPrototypeOf(this);
                        var found = false;
                        while (cur) { if (cur === proto) { found = true; break; } cur = Object.getPrototypeOf(cur); }
                        if (!found) throw new TypeError('Illegal invocation');
                        var s = this['_' + prop];
                        return s !== undefined ? s : dv;
                    },
                    enumerable: true,
                    configurable: true
                };
                if (!isReadonly) {
                    desc.set = function(v) {
                        if (this === null || this === undefined) throw new TypeError('Illegal invocation');
                        this['_' + prop] = v;
                    };
                }
                Object.defineProperty(proto, prop, desc);
            });
        }
    } catch(e) {}

    // CaretPosition: wrap caretPositionFromPoint so returned objects get
    // the correct toStringTag. Only set toStringTag, not prototype chain
    // (same rationale as matchMedia above).
    try {
        if (typeof document !== 'undefined' && document.caretPositionFromPoint
            && !document.caretPositionFromPoint.__iv8CaretPatched && typeof CaretPosition !== 'undefined') {
            var _origCaret = document.caretPositionFromPoint;
            var _caretWrapper = function caretPositionFromPoint(x, y) {
                var cp = _origCaret.call(this, x, y);
                if (cp && typeof cp === 'object' && typeof CaretPosition !== 'undefined') {
                    try {
                        Object.setPrototypeOf(cp, CaretPosition.prototype);
                    } catch(e) {}
                }
                return cp;
            };
            Object.defineProperty(_caretWrapper, '__iv8CaretPatched', {
                value: true, writable: true, configurable: true, enumerable: false,
            });
            document.caretPositionFromPoint = _caretWrapper;
        }
    } catch(e) {}

    // VisualViewport: the codegen getter returns a plain object. Set
    // toStringTag only (not prototype chain) for the same rationale as above.
    try {
        if (typeof window !== 'undefined' && window.visualViewport && typeof VisualViewport !== 'undefined') {
            try {
                var vv = window.visualViewport;
                var VVProxy = Object.create(VisualViewport.prototype);
                var vnames = Object.getOwnPropertyNames(vv);
                for (var vi = 0; vi < vnames.length; vi++) {
                    var vn = vnames[vi];
                    try {
                        var vd = Object.getOwnPropertyDescriptor(vv, vn);
                        if (vd && vd.configurable) {
                            Object.defineProperty(VVProxy, vn, vd);
                        }
                    } catch(e) {}
                }
                Object.defineProperty(window, 'visualViewport', {value: VVProxy, writable: true, configurable: true, enumerable: true});
            } catch(e) {}
        }
    } catch(e) {}

    // DOMException constructor shim — real Chrome has DOMException for
    // Promise rejections (EME, MIDI, mediaDevices). V8 only has TypeError.
    if (typeof DOMException === 'undefined') {
        var _domExCodes = {
            IndexSizeError: 1, DOMStringSizeError: 2,
            HierarchyRequestError: 3, WrongDocumentError: 4,
            InvalidCharacterError: 5, NoDataAllowedError: 6,
            NoModificationAllowedError: 7, NotFoundError: 8,
            NotSupportedError: 9, InUseAttributeError: 10,
            InvalidStateError: 11, SyntaxError: 12,
            InvalidModificationError: 13, NamespaceError: 14,
            InvalidAccessError: 15, ValidationError: 16,
            TypeMismatchError: 17, SecurityError: 18,
            NetworkError: 19, AbortError: 20,
            URLMismatchError: 21, QuotaExceededError: 22,
            TimeoutError: 23, InvalidNodeTypeError: 24,
            DataCloneError: 25
        };
        function DOMException(message, name) {
            this.message = message || '';
            this.name = name || 'Error';
            this.code = _domExCodes[name] || 0;
        }
        Object.defineProperty(DOMException.prototype, Symbol.toStringTag, {
            value: 'DOMException', configurable: true
        });
        globalThis.DOMException = DOMException;
    }

    // createElement prototype fix — template_for_tag maps many tags to
    // generic HTMLElement. For tags with a specific codegen constructor,
    // set the correct prototype so instanceof checks pass.
    try {
        var _tagToClass = {
            iframe: 'HTMLIFrameElement', object: 'HTMLObjectElement',
            embed: 'HTMLEmbedElement', progress: 'HTMLProgressElement',
            meter: 'HTMLMeterElement', label: 'HTMLLabelElement',
            fieldset: 'HTMLFieldSetElement', legend: 'HTMLLegendElement',
            optgroup: 'HTMLOptGroupElement', option: 'HTMLOptionElement',
            template: 'HTMLTemplateElement', slot: 'HTMLSlotElement',
            data: 'HTMLDataElement', time: 'HTMLTimeElement',
            output: 'HTMLOutputElement', picture: 'HTMLPictureElement',
            source: 'HTMLSourceElement', details: 'HTMLDetailsElement',
            dialog: 'HTMLDialogElement', datalist: 'HTMLDataListElement',
            track: 'HTMLTrackElement', video: 'HTMLVideoElement',
            audio: 'HTMLAudioElement', map: 'HTMLMapElement',
            area: 'HTMLAreaElement', base: 'HTMLBaseElement',
            head: 'HTMLHeadElement', body: 'HTMLBodyElement',
            html: 'HTMLHtmlElement', link: 'HTMLLinkElement',
            meta: 'HTMLMetaElement', title: 'HTMLTitleElement',
            style: 'HTMLStyleElement', script: 'HTMLScriptElement',
            img: 'HTMLImageElement', canvas: 'HTMLCanvasElement',
            select: 'HTMLSelectElement', textarea: 'HTMLTextAreaElement',
            pre: 'HTMLPreElement', br: 'HTMLBRElement', hr: 'HTMLHRElement',
            blockquote: 'HTMLQuoteElement', q: 'HTMLQuoteElement',
            ins: 'HTMLModElement', del: 'HTMLModElement',
            ul: 'HTMLUListElement', ol: 'HTMLOListElement', li: 'HTMLLIElement',
            table: 'HTMLTableElement', td: 'HTMLTableCellElement',
            th: 'HTMLTableCellElement', tr: 'HTMLTableRowElement',
            thead: 'HTMLTableSectionElement', tbody: 'HTMLTableSectionElement',
            tfoot: 'HTMLTableSectionElement', col: 'HTMLTableColElement',
            colgroup: 'HTMLTableColElement', caption: 'HTMLTableCaptionElement',
            frameset: 'HTMLFrameSetElement', frame: 'HTMLFrameElement',
            marquee: 'HTMLMarqueeElement', param: 'HTMLParamElement',
            font: 'HTMLFontElement', dir: 'HTMLDirectoryElement',
            listing: 'HTMLPreElement', xmp: 'HTMLPreElement',
            menu: 'HTMLMenuElement', noscript: 'HTMLElement',
            nobr: 'HTMLElement', center: 'HTMLElement',
            div: 'HTMLDivElement', p: 'HTMLParagraphElement',
            a: 'HTMLAnchorElement', span: 'HTMLSpanElement',
            h1: 'HTMLHeadingElement', h2: 'HTMLHeadingElement',
            h3: 'HTMLHeadingElement', h4: 'HTMLHeadingElement',
            h5: 'HTMLHeadingElement', h6: 'HTMLHeadingElement',
            input: 'HTMLInputElement', button: 'HTMLButtonElement',
            form: 'HTMLFormElement',
            code: 'HTMLElement', small: 'HTMLElement',
            strong: 'HTMLElement', em: 'HTMLElement',
            b: 'HTMLElement', i: 'HTMLElement', u: 'HTMLElement',
            s: 'HTMLElement', sub: 'HTMLElement', sup: 'HTMLElement',
        };
        var _origCreate = document.createElement;
        if (_origCreate && !_origCreate.__iv8ElemPatched) {
            var _wrapper = function(tagName) {
                var el = _origCreate.call(document, tagName);
                if (el && typeof tagName === 'string') {
                    var cls = _tagToClass[tagName.toLowerCase()];
                    if (cls && typeof globalThis[cls] !== 'undefined'
                        && globalThis[cls].prototype
                        && el.__proto__ !== globalThis[cls].prototype) {
                        try { el.__proto__ = globalThis[cls].prototype; } catch(e) {}
                    }
                    // Set Symbol.toStringTag on the instance for correct class string
                    if (cls && typeof Symbol !== 'undefined' && Symbol.toStringTag) {
                        try {
                            Object.defineProperty(el, Symbol.toStringTag, {
                                value: cls, writable: false,
                                enumerable: false, configurable: true
                            });
                        } catch(e) {}
                    }
                }
                return el;
            };
            Object.defineProperty(_wrapper, '__iv8ElemPatched', {
                value: true, writable: true, configurable: true, enumerable: false,
            });
            document.createElement = _wrapper;
        }
    } catch(e) {}

})();
"#;

/// Cookie-only re-install JS. Used at page_load step 9b to restore the
/// cookie accessor after inline scripts may have overridden it.
///
/// Must NOT use the full DOCUMENT_PROPS_JS because the `lastModified`
/// getter's `new Date().toLocaleString()` triggers Intl re-entrancy → OOM
/// when run a second time in the same context.
pub const COOKIE_REINSTALL_JS: &str = r#"
(function() {
    if (typeof document === 'undefined') return;
    var _cookies = window._iv8CookieStore || (window._iv8CookieStore = {});
    function _cookieValue(rec) {
        if (typeof rec === 'string') return rec;
        if (rec && typeof rec === 'object' && rec.v !== undefined) return rec.v;
        return '';
    }
    function _cookieVisible(rec) {
        if (typeof rec === 'string') return true;
        if (!rec || typeof rec !== 'object') return true;
        if (rec.path && rec.path !== '/') {
            var docPath = '/';
            try { docPath = document.location ? document.location.pathname : '/'; } catch(e) {}
            if (docPath !== rec.path && docPath.indexOf(rec.path) !== 0) return false;
        }
        if (rec.secure && window.__iv8IsSecureContext !== true) return false;
        return true;
    }
    try {
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
            var attrs = {};
            var hasAttrs = false;
            for (var i = 1; i < parts.length; i++) {
                var attr = parts[i].trim();
                var lower = attr.toLowerCase();
                if (lower === 'secure') { attrs.secure = true; hasAttrs = true; }
                else if (lower === 'httponly') { attrs.httpOnly = true; hasAttrs = true; }
                else if (lower.indexOf('path=') === 0) { attrs.path = attr.substring(5); hasAttrs = true; }
                else if (lower.indexOf('domain=') === 0) { attrs.domain = attr.substring(7); hasAttrs = true; }
                else if (lower.indexOf('samesite=') === 0) { attrs.sameSite = attr.substring(9); hasAttrs = true; }
                else if (lower.indexOf('expires=') === 0) { attrs.expires = attr.substring(8); hasAttrs = true; }
                else if (lower.indexOf('max-age=') === 0) {
                    var ma = parseInt(attr.substring(8), 10);
                    if (!isNaN(ma)) { if (ma <= 0) { delete _cookies[name]; return; } attrs.maxAge = ma; hasAttrs = true; }
                }
            }
            if (hasAttrs) { attrs.v = value; _cookies[name] = attrs; }
            else { _cookies[name] = value; }
        },
        enumerable: true,
        configurable: true,
    });
    } catch(e) {}
})();
"#;
