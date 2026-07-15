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
    // F-30b: RFC 6265 expires/max-age expiry + domain matching
    var _cookies = window._iv8CookieStore || (window._iv8CookieStore = {});

    function _cookieValue(rec) {
        if (typeof rec === 'string') return rec;
        if (rec && typeof rec === 'object' && rec.v !== undefined) return rec.v;
        return '';
    }

    function _cookieExpired(rec) {
        if (typeof rec === 'string' || !rec || typeof rec !== 'object') return false;
        // Check expires timestamp (epoch ms)
        if (rec.expiresTs && typeof rec.expiresTs === 'number') {
            if (Date.now() >= rec.expiresTs) return true;
        }
        return false;
    }

    function _domainMatches(cookieDomain, hostName) {
        if (!cookieDomain) return true;
        var d = cookieDomain.toLowerCase().replace(/^\./, '');
        var h = (hostName || '').toLowerCase();
        if (h === d) return true;
        // Subdomain match: cookie domain ".example.com" matches "www.example.com"
        return h.length > d.length && h.charAt(h.length - d.length - 1) === '.' && h.slice(-d.length) === d;
    }

    function _cookieVisible(rec) {
        if (typeof rec === 'string') return true;    // legacy: no attributes
        if (!rec || typeof rec !== 'object') return true;
        // httpOnly cookies are NOT visible to document.cookie (RFC 6265 §5.3)
        if (rec.httpOnly) return false;
        // Expired cookies are not visible
        if (_cookieExpired(rec)) return false;
        // Path filtering (RFC 6265 prefix match)
        if (rec.path && rec.path !== '/') {
            var docPath = '/';
            try { docPath = document.location ? document.location.pathname : '/'; } catch(e) {}
            if (!_pathMatches(docPath, rec.path)) return false;
        }
        // Domain filtering
        if (rec.domain) {
            var host = '';
            try { host = location.hostname || ''; } catch(e) {}
            if (!_domainMatches(rec.domain, host)) return false;
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
                    var dateStr = attr.substring(8);
                    attrs.expires = dateStr; hasAttrs = true;
                    // Parse to epoch ms for expiry checking
                    var parsed = Date.parse(dateStr);
                    if (!isNaN(parsed)) {
                        attrs.expiresTs = parsed;
                    }
                }
                else if (lower.indexOf('max-age=') === 0) {
                    var ma = parseInt(attr.substring(8), 10);
                    if (!isNaN(ma)) {
                        if (ma <= 0) { delete _cookies[name]; return; }
                        attrs.maxAge = ma; hasAttrs = true;
                        // Convert max-age to expires timestamp (seconds → ms)
                        attrs.expiresTs = Date.now() + (ma * 1000);
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
    // P1-DESC: Install as accessor (not data property) on instance so
    // getOwnPropertyDescriptor returns accessor descriptor, matching Chrome.
    // Prototype accessor (line ~848) is the spec source; instance accessor
    // delegates to the same value but with correct descriptor type.
    Object.defineProperty(document, 'referrer', {
        get: function() { return ''; },
        set: undefined,
        enumerable: true,
        configurable: true,
    });

    // document.hidden / visibilityState — controllable for timer throttle (Q082 residual).
    var _hidden = false;
    var _visibilityState = 'visible';
    function _setVisibility(hidden) {
        var h = !!hidden;
        var vs = h ? 'hidden' : 'visible';
        if (h === _hidden && vs === _visibilityState) return;
        _hidden = h;
        _visibilityState = vs;
        try {
            if (h) {
                var t = (typeof performance !== 'undefined' && performance.now)
                    ? performance.now() : Date.now();
                globalThis.__iv8HiddenSinceMs = t;
            } else {
                try { delete globalThis.__iv8HiddenSinceMs; } catch (e0) {
                    globalThis.__iv8HiddenSinceMs = undefined;
                }
            }
        } catch (e1) {}
        try {
            var ev = new Event('visibilitychange');
            document.dispatchEvent(ev);
        } catch (e) {}
    }
    Object.defineProperty(document, 'hidden', {
        get: function() { return _hidden; },
        set: function(v) { _setVisibility(v); },
        enumerable: true,
        configurable: true,
    });
    Object.defineProperty(document, 'visibilityState', {
        get: function() { return _visibilityState; },
        set: function(v) {
            var s = String(v);
            if (s === 'hidden' || s === 'visible') _setVisibility(s === 'hidden');
        },
        enumerable: true,
        configurable: true,
    });

    // document.readyState — mutable for page_load lifecycle (Q080).
    // Initial blank context is complete; page_load sets loading→interactive→complete.
    var _readyState = 'complete';
    var _onreadystatechange = null;
    Object.defineProperty(document, 'readyState', {
        get: function() { return _readyState; },
        set: function(v) {
            var nv = String(v);
            if (nv !== 'loading' && nv !== 'interactive' && nv !== 'complete') return;
            if (nv === _readyState) return;
            _readyState = nv;
            try {
                var ev = new Event('readystatechange');
                if (typeof _onreadystatechange === 'function') {
                    try { _onreadystatechange.call(document, ev); } catch (e0) {}
                }
                document.dispatchEvent(ev);
            } catch (e) {}
        },
        enumerable: true,
        configurable: true,
    });
    Object.defineProperty(document, 'onreadystatechange', {
        get: function() { return _onreadystatechange; },
        set: function(fn) {
            _onreadystatechange = (typeof fn === 'function' || fn === null) ? fn : null;
        },
        enumerable: true,
        configurable: true,
    });

    // document.domain — store value for reflection (sandbox does not change origin)
    var _domain = (typeof location !== 'undefined' && location.hostname) ? location.hostname : '';
    Object.defineProperty(document, 'domain', {
        get: function() { return _domain; },
        set: function(v) { _domain = String(v); },
        enumerable: true,
        configurable: true,
    });

    // URL / title / documentURI: native Document.prototype (binding.rs).
    // Do not define own instance shadows (RD-11).

    // createTextNode / createComment / createDocumentFragment / createElement:
    // native Document.prototype only (RD-16/17).
    // EventTarget methods: native binding on Document.prototype (RD-11).
    // MF-4: do NOT install own importNode/adoptNode/createEvent/execCommand*
    // identity stubs on the document instance — they dual-own Document.prototype
    // natives (createEvent was then overwritten again below). Policy-only surface
    // remains cookie/referrer/visibility/fonts/plugins.
    if (!Object.getOwnPropertyDescriptor(document, 'fonts') || !document.fonts) {
        var _fontPrefs = (typeof globalThis.__iv8FontPrefs === 'object' && globalThis.__iv8FontPrefs) ? globalThis.__iv8FontPrefs : {};
        var _fontFamilies = _fontPrefs.families || [];
        var _fontFaceObjects = _fontFamilies.map(function(f) {
            return {
                family: f, status: 'loaded', weight: 'normal', style: 'normal',
                stretch: 'normal', display: 'auto', unicodeRange: 'U+0-10FFFF',
                featureSettings: 'normal', variationSettings: 'normal',
            };
        });
        var _fontSet = {
            ready: null, // set after object creation (see below)
            status: 'loaded',
            onloading: null,
            onloadingdone: null,
            onloadingerror: null,
            get size() { return _fontFaceObjects.length; },
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
            load: function(font, text) { return Promise.resolve(_fontFaceObjects.slice()); },
            forEach: function(cb) {
                _fontFaceObjects.forEach(function(ff, i) {
                    cb(ff, i, _fontSet);
                });
            },
            values: function() { return _fontFaceObjects.slice().values(); },
            entries: function() { return _fontFaceObjects.map(function(ff, i) { return [i, ff]; }).entries(); },
            keys: function() { return _fontFaceObjects.keys(); },
            add: function(fontFace) {
                if (fontFace && typeof fontFace === 'object') {
                    _fontFaceObjects.push(fontFace);
                }
            },
            delete: function(fontFace) {
                var idx = _fontFaceObjects.indexOf(fontFace);
                if (idx !== -1) _fontFaceObjects.splice(idx, 1);
            },
            clear: function() { _fontFaceObjects.length = 0; },
        };
        // Per FontFaceSet spec: ready resolves to the FontFaceSet itself.
        _fontSet.ready = Promise.resolve(_fontSet);
        Object.defineProperty(document, 'fonts', {
            get: function() { return _fontSet; },
            set: function() {},
            enumerable: true,
            configurable: true,
        });
    }
    if (!Object.getOwnPropertyDescriptor(document, 'timeline') || !document.timeline) {
        var _timeline = { currentTime: performance.now() };
        Object.defineProperty(document, 'timeline', {
            get: function() { return _timeline; },
            set: function() {},
            enumerable: true,
            configurable: true,
        });
    }

    // FontFace constructor — new FontFace(family, source)
    // Returns a FontFace object with family, status='unloaded', load()→Promise.
    // If a codegen FontFace constructor already exists, enhance its prototype
    // with the load() method and status property if missing.
    if (typeof FontFace === 'undefined') {
        function FontFace(family, source, descriptors) {
            if (!(this instanceof FontFace)) {
                throw new TypeError("Failed to construct 'FontFace': Please use the 'new' operator, this DOM object constructor cannot be called as a function.");
            }
            this.family = String(family || '');
            this._source = String(source || '');
            this._descriptors = descriptors || {};
            this.status = 'unloaded';
            this.display = this._descriptors.display || 'auto';
            this.weight = this._descriptors.weight || 'normal';
            this.style = this._descriptors.style || 'normal';
            this.stretch = this._descriptors.stretch || 'normal';
            this.unicodeRange = this._descriptors.unicodeRange || 'U+0-10FFFF';
            this.featureSettings = this._descriptors.featureSettings || 'normal';
            this.variationSettings = this._descriptors.variationSettings || 'normal';
            this.ascentOverride = this._descriptors.ascentOverride || 'normal';
            this.descentOverride = this._descriptors.descentOverride || 'normal';
            this.lineGapOverride = this._descriptors.lineGapOverride || 'normal';
        }
        FontFace.prototype.load = function() {
            if (this == null || typeof this !== 'object') {
                return Promise.reject(new TypeError('Illegal invocation'));
            }
            var self = this;
            return new Promise(function(resolve, reject) {
                self.status = 'loading';
                setTimeout(function() {
                    self.status = 'loaded';
                    resolve(self);
                }, 0);
            });
        };
        if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {
            Object.defineProperty(FontFace.prototype, Symbol.toStringTag, {
                value: 'FontFace', writable: false, configurable: true, enumerable: false,
            });
        }
        try { Object.defineProperty(FontFace, 'length', { value: 2, writable: false, enumerable: false, configurable: true }); } catch(e) {}
        globalThis.FontFace = FontFace;
    } else {
        // Codegen FontFace exists — ensure load() and status are present.
        // Always install load with receiver check (overrides codegen skeleton).
        FontFace.prototype.load = function() {
            if (this == null || typeof this !== 'object' ||
                (typeof FontFace !== 'undefined' && !(this instanceof FontFace))) {
                return Promise.reject(new TypeError('Illegal invocation'));
            }
            var self = this;
            return new Promise(function(resolve) {
                if (self.status === undefined) self.status = 'unloaded';
                self.status = 'loading';
                setTimeout(function() {
                    self.status = 'loaded';
                    resolve(self);
                }, 0);
            });
        };
    }
    // Properties below are in EXCLUDED_ATTRIBUTES — codegen no longer
    // installs them, so guards are dead code. Always install to ensure
    // the shim is the single source of truth.
    Object.defineProperty(document, 'scrollingElement', { get: function() { return document.body || null; }, configurable: true });
    // currentScript is set while classic scripts run (DOCUMENT_WRITE_SHIM / appendChild).
    var _currentScript = null;
    Object.defineProperty(document, 'currentScript', {
        get: function() { return _currentScript; },
        set: function(v) { _currentScript = v || null; },
        configurable: true,
        enumerable: true,
    });
    // document.scripts — live collection of script elements (codegen stub may be empty).
    try {
        Object.defineProperty(document, 'scripts', {
            get: function() { return document.getElementsByTagName('script'); },
            configurable: true,
            enumerable: true,
        });
    } catch (e) {}
    // Override Document.prototype.createRange/createEvent (not document own)
    // so instance and prototype share one Function (MF-4 dual-path cleanup).
    if (typeof Document !== 'undefined' && Document.prototype) {
        if (typeof Range !== 'undefined') {
            Document.prototype.createRange = function createRange() {
                return Object.create(Range.prototype);
            };
        }
        if (typeof Event !== 'undefined') {
            Document.prototype.createEvent = function createEvent(type) {
                var ctorMap = { Event: Event, CustomEvent: CustomEvent, MouseEvent: MouseEvent, UIEvent: UIEvent, KeyboardEvent: KeyboardEvent, AnimationEvent: AnimationEvent, TransitionEvent: TransitionEvent, MessageEvent: MessageEvent, DragEvent: DragEvent, BeforeUnloadEvent: BeforeUnloadEvent, HashChangeEvent: HashChangeEvent, PageTransitionEvent: PageTransitionEvent, PopStateEvent: PopStateEvent, StorageEvent: StorageEvent, SubmitEvent: SubmitEvent, ToggleEvent: ToggleEvent, CloseWatcher: CloseWatcher, PromiseRejectionEvent: PromiseRejectionEvent, ErrorEvent: ErrorEvent, FormDataEvent: FormDataEvent };
                var Ctor = ctorMap[type];
                var ev;
                if (Ctor) {
                    try { ev = new Ctor(type); } catch(e) {}
                }
                if (!ev) { ev = new Event(type); }
                try { Object.defineProperty(ev, 'isTrusted', { value: false, writable: false, enumerable: true, configurable: true }); } catch(e) { ev.isTrusted = false; }
                return ev;
            };
        }
    }
    if (!document.implementation) {
        var implProto = (typeof DOMImplementation !== 'undefined') ? DOMImplementation.prototype : Object.prototype;
        var impl = Object.create(implProto);
        Object.defineProperty(impl, 'createHTMLDocument', { value: function createHTMLDocument(t) { return document; }, writable: true, configurable: true, enumerable: true });
        Object.defineProperty(impl, 'hasFeature', { value: function() { return true; }, writable: true, configurable: true, enumerable: true });
        // createDocument: return an XMLDocument-like object with Document prototype
        // codegen callback returns Object::new() without proper prototype
        Object.defineProperty(impl, 'createDocument', { value: function createDocument(ns, name, doctype) {
            if (arguments.length < 2) throw new TypeError('2 argument(s) required, but only ' + arguments.length + ' present.');
            var docProto = (typeof XMLDocument !== 'undefined') ? XMLDocument.prototype : (typeof Document !== 'undefined' ? Document.prototype : Object.prototype);
            var doc = Object.create(docProto);
            Object.defineProperty(doc, Symbol.toStringTag, { value: 'XMLDocument', writable: true, configurable: true, enumerable: false });
            // Override createElementNS to return Element with correct prototype
            // codegen callback returns Object::new() without Element.prototype
            // Use document.createElement internally to get a real Element instance
            doc.createElementNS = function createElementNS(ns, qname) {
                if (arguments.length < 2) throw new TypeError('2 argument(s) required, but only ' + arguments.length + ' present.');
                return document.createElement(qname || 'div');
            };
            doc.createElement = function createElement(tag) {
                if (arguments.length < 1) throw new TypeError('1 argument required, but only 0 present.');
                return document.createElement(tag || 'div');
            };
            doc.createProcessingInstruction = function createProcessingInstruction(target, data) {
                if (arguments.length < 2) throw new TypeError('2 argument(s) required, but only ' + arguments.length + ' present.');
                var piProto = (typeof ProcessingInstruction !== 'undefined') ? ProcessingInstruction.prototype : Object.prototype;
                return Object.create(piProto);
            };
            doc.createAttribute = function createAttribute(name) {
                if (arguments.length < 1) throw new TypeError('1 argument required, but only 0 present.');
                var attrProto = (typeof Attr !== 'undefined') ? Attr.prototype : Object.prototype;
                return Object.create(attrProto);
            };
            doc.createCDATASection = function createCDATASection(data) {
                if (arguments.length < 1) throw new TypeError('1 argument required, but only 0 present.');
                var textProto = (typeof Text !== 'undefined') ? Text.prototype : Object.prototype;
                var node = Object.create(textProto);
                Object.defineProperty(node, 'nodeType', { value: 4, writable: false, enumerable: true, configurable: false });
                Object.defineProperty(node, 'nodeName', { value: '#cdata-section', writable: false, enumerable: true, configurable: false });
                Object.defineProperty(node, 'nodeValue', { value: String(data), writable: true, enumerable: true, configurable: true });
                Object.defineProperty(node, 'textContent', { value: String(data), writable: true, enumerable: true, configurable: true });
                Object.defineProperty(node, 'data', { value: String(data), writable: true, enumerable: true, configurable: true });
                return node;
            };
            // createTextNode/createComment: use Document.prototype native only
            return doc;
        }, writable: true, configurable: true, enumerable: true });
        // createDocumentType: return a DocumentType-like object
        Object.defineProperty(impl, 'createDocumentType', { value: function createDocumentType(qname, publicId, systemId) {
            if (arguments.length < 3) throw new TypeError('3 argument(s) required, but only ' + arguments.length + ' present.');
            var dtProto = (typeof DocumentType !== 'undefined') ? DocumentType.prototype : Object.prototype;
            var dt = Object.create(dtProto);
            return dt;
        }, writable: true, configurable: true, enumerable: true });
        // Readonly: no-op setter so assignment does not shadow (H05b Category C).
        Object.defineProperty(document, 'implementation', {
            get: function() { return impl; },
            set: function() {},
            enumerable: true,
            configurable: true,
        });
    }
    Object.defineProperty(document, 'defaultView', { get: function() { return window; }, set: function() {}, configurable: true });
    Object.defineProperty(document, 'ownerDocument', { get: function() { return null; }, set: function() {}, configurable: true });
    Object.defineProperty(document, 'baseURI', { get: function() { return location.href; }, set: function() {}, configurable: true });
    Object.defineProperty(document, 'characterSet', { get: function() { return 'UTF-8'; }, set: function() {}, configurable: true });
    Object.defineProperty(document, 'charset', { get: function() { return 'UTF-8'; }, set: function() {}, configurable: true });
    Object.defineProperty(document, 'inputEncoding', { get: function() { return 'UTF-8'; }, set: function() {}, configurable: true });
    var _designMode = 'off';
    Object.defineProperty(document, 'designMode', {
        get: function() { return _designMode; },
        set: function(v) { _designMode = String(v); },
        enumerable: true,
        configurable: true,
    });
    Object.defineProperty(document, 'contentType', { get: function() { return 'text/html'; }, set: function() {}, configurable: true });
    Object.defineProperty(document, 'compatMode', { get: function() { return 'CSS1Compat'; }, set: function() {}, configurable: true });
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

    // PDF plugins: owned by native_env::nav_plugins (SameObject cache).
    // Do not dual-inject here (INIT-2 / MF-4 ownership).

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
                if (arguments.length < 1) throw new TypeError("1 argument required, but only 0 present");
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

    // window.Image / window.Audio named constructors
    // codegen installs native named constructors, but they return elements
    // without Symbol.toStringTag. Wrap them to set toStringTag.
    try {
        var _origImage = globalThis.Image;
        if (_origImage && typeof _origImage === 'function') {
            var _wrappedImage = function Image(width, height) {
                if (!(this instanceof Image)) throw new TypeError("Failed to construct 'Image': Please use the 'new' operator, this DOM object constructor cannot be called as a function.");
                var img = document.createElement('img');
                if (width !== undefined) img.width = width;
                if (height !== undefined) img.height = height;
                if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {
                    try { Object.defineProperty(img, Symbol.toStringTag, {
                        value: 'HTMLImageElement', writable: false,
                        enumerable: false, configurable: true
                    }); } catch(e) {}
                }
                return img;
            };
            try { Object.defineProperty(_wrappedImage, 'name', { value: 'Image' }); } catch(e) {}
            // WebIDL Image(optional unsigned long w, optional unsigned long h):
            // minOverloadLength is 0 (all args optional) — matches Chrome.
            try { Object.defineProperty(_wrappedImage, 'length', { value: 0, writable: false, enumerable: false, configurable: true }); } catch(e) {}
            Object.defineProperty(globalThis, 'Image', {
                value: _wrappedImage, writable: true, configurable: true, enumerable: false
            });
        }
    } catch(e) {}

    try {
        var _origAudio = globalThis.Audio;
        if (_origAudio && typeof _origAudio === 'function') {
            var _wrappedAudio = function Audio(url) {
                if (!(this instanceof Audio)) throw new TypeError("Failed to construct 'Audio': Please use the 'new' operator, this DOM object constructor cannot be called as a function.");
                var aud = document.createElement('audio');
                if (url !== undefined) aud.src = url;
                if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {
                    try { Object.defineProperty(aud, Symbol.toStringTag, {
                        value: 'HTMLAudioElement', writable: false,
                        enumerable: false, configurable: true
                    }); } catch(e) {}
                }
                return aud;
            };
            try { Object.defineProperty(_wrappedAudio, 'name', { value: 'Audio' }); } catch(e) {}
            try { Object.defineProperty(_wrappedAudio, 'length', { value: 0, writable: false, enumerable: false, configurable: true }); } catch(e) {}
            Object.defineProperty(globalThis, 'Audio', {
                value: _wrappedAudio, writable: true, configurable: true, enumerable: false
            });
        }
    } catch(e) {}

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
            // Geometry stubs only. innerText is owned by Element FT (template.rs)
            // and must not be overwritten with empty _innerText slots (RD-11).
            var _heProps = {
                offsetTop: 0, offsetLeft: 0,
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
            // Geometry is owned by Element FT (template.rs). Do not install
            // HTMLElement.prototype stubs that shadow native layout getters.
            Object.keys(_heProps).forEach(function(prop) {
                if (prop in Element.prototype || prop in HTMLElement.prototype) return;
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

    // createElement ownership: native binding + create_node_object /
    // tag_to_interface_name (RD-16/17). Do not wrap document.createElement here.

    // HTMLIFrameElement readonly attrs: ensure no-op setters so assignment
    // does not create shadowing data properties (H05b Category C).
    try {
        if (typeof HTMLIFrameElement !== 'undefined' && HTMLIFrameElement.prototype) {
            var _iframeRo = ['contentDocument', 'contentWindow', 'permissionsPolicy', 'sandbox', 'featurePolicy'];
            _iframeRo.forEach(function(prop) {
                var cur = Object.getOwnPropertyDescriptor(HTMLIFrameElement.prototype, prop);
                var getter = (cur && cur.get) ? cur.get : function() { return null; };
                Object.defineProperty(HTMLIFrameElement.prototype, prop, {
                    get: getter,
                    set: function() {},
                    enumerable: true,
                    configurable: true,
                });
            });
        }
    } catch (e) {}

    // window.close — installed by window_extras.rs _winOps with receiver check

    // window.external — legacy IE API. Must be instanceof External.
    // Real Chrome has window.external as an External instance.
    if (typeof External === 'undefined') {
        function External() {}
        if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {
            Object.defineProperty(External.prototype, Symbol.toStringTag, {
                value: 'External', writable: false, configurable: true, enumerable: false
            });
        }
        globalThis.External = External;
    }
    try {
        var _extProto = (typeof External !== 'undefined' && External.prototype)
            ? External.prototype : Object.prototype;
        var _extInstance = Object.create(_extProto);
        Object.defineProperty(globalThis, 'external', {
            value: _extInstance, writable: true, configurable: true, enumerable: true
        });
    } catch(e) {}
    // External prototype operations — AddSearchProvider / IsSearchProviderInstalled
    // must be own properties of External.prototype (not the instance) for idlharness.
    try {
        if (typeof External !== 'undefined' && External.prototype) {
            if (!External.prototype.AddSearchProvider) {
                External.prototype.AddSearchProvider = function AddSearchProvider() { if (this == null) throw new TypeError('Illegal invocation'); };
                try { Object.defineProperty(External.prototype.AddSearchProvider, 'name', { value: 'AddSearchProvider' }); } catch(e) {}
                try { Object.defineProperty(External.prototype.AddSearchProvider, 'length', { value: 0, writable: false, enumerable: false, configurable: true }); } catch(e) {}
            }
            if (!External.prototype.IsSearchProviderInstalled) {
                External.prototype.IsSearchProviderInstalled = function IsSearchProviderInstalled() { if (this == null) throw new TypeError('Illegal invocation'); return 0; };
                try { Object.defineProperty(External.prototype.IsSearchProviderInstalled, 'name', { value: 'IsSearchProviderInstalled' }); } catch(e) {}
                try { Object.defineProperty(External.prototype.IsSearchProviderInstalled, 'length', { value: 0, writable: false, enumerable: false, configurable: true }); } catch(e) {}
            }
        }
    } catch(e) {}
    // Delete AddSearchProvider / IsSearchProviderInstalled from the external
    // instance if they are own properties, so they resolve via the prototype.
    try {
        if (globalThis.external && globalThis.external.hasOwnProperty('AddSearchProvider')) {
            try { delete globalThis.external.AddSearchProvider; } catch(e) {}
        }
        if (globalThis.external && globalThis.external.hasOwnProperty('IsSearchProviderInstalled')) {
            try { delete globalThis.external.IsSearchProviderInstalled; } catch(e) {}
        }
    } catch(e) {}

    // Stringifier for HTMLAnchorElement/HTMLAreaElement — toString returns href
    if (typeof HTMLAnchorElement !== 'undefined' && HTMLAnchorElement.prototype) {
        Object.defineProperty(HTMLAnchorElement.prototype, 'toString', {
            value: function toString() { return this.href || ''; },
            writable: true, enumerable: false, configurable: true
        });
    }
    if (typeof HTMLAreaElement !== 'undefined' && HTMLAreaElement.prototype) {
        Object.defineProperty(HTMLAreaElement.prototype, 'toString', {
            value: function toString() { return this.href || ''; },
            writable: true, enumerable: false, configurable: true
        });
    }

    // Location interface properties
    // [LegacyUnforgeable] attributes are own properties of the location
    // instance, not Location.prototype. idlharness uses
    // assert_own_property(window.location, name) for these.
    // W2: own getters must brand-check (Illegal invocation) and writable
    // fields must update href — never install set:undefined for protocol/host.
    try {
        var _locObj = (typeof location !== 'undefined') ? location : null;
        var _locTarget = _locObj;
        if (_locTarget) {
            function _locBrand() {
                if (typeof Location === 'undefined' || !(this instanceof Location)) {
                    throw new TypeError('Illegal invocation');
                }
            }
            function _locHref() {
                try { return String(this.href || ''); } catch (e) { return ''; }
            }
            function _locSetPart(part, value) {
                _locBrand.call(this);
                try {
                    // Manual parse — IV8 URL component setters are not fully live.
                    var h = _locHref.call(this) || 'about:blank';
                    var hash = '';
                    var search = '';
                    var hi = h.indexOf('#');
                    if (hi >= 0) { hash = h.slice(hi); h = h.slice(0, hi); }
                    var si = h.indexOf('?');
                    if (si >= 0) { search = h.slice(si); h = h.slice(0, si); }
                    var protocol = 'https:';
                    var rest = h;
                    var ci = h.indexOf(':');
                    if (ci > 0) {
                        protocol = h.slice(0, ci + 1);
                        rest = h.slice(ci + 1);
                    }
                    if (rest.indexOf('//') === 0) rest = rest.slice(2);
                    else if (rest.charAt(0) === '/') rest = rest.slice(1);
                    var path = '';
                    var hostport = rest;
                    var pi = rest.indexOf('/');
                    if (pi >= 0) {
                        hostport = rest.slice(0, pi);
                        path = rest.slice(pi);
                    }
                    if (!path) path = '/';
                    var hostname = hostport;
                    var port = '';
                    var coli = hostport.lastIndexOf(':');
                    if (coli > 0) {
                        hostname = hostport.slice(0, coli);
                        port = hostport.slice(coli + 1);
                    }
                    if (!hostname || /^about$/i.test(protocol.replace(':', ''))) {
                        hostname = 'localhost';
                        if (protocol === 'about:') protocol = 'https:';
                    }
                    var v = String(value);
                    if (part === 'protocol') {
                        protocol = v.charAt(v.length - 1) === ':' ? v : (v + ':');
                    } else if (part === 'hostname') {
                        hostname = v;
                    } else if (part === 'host') {
                        hostport = v;
                        coli = hostport.lastIndexOf(':');
                        if (coli > 0) {
                            hostname = hostport.slice(0, coli);
                            port = hostport.slice(coli + 1);
                        } else {
                            hostname = hostport;
                            port = '';
                        }
                    } else if (part === 'port') {
                        port = v;
                    } else if (part === 'pathname') {
                        path = v.charAt(0) === '/' ? v : ('/' + v);
                    } else if (part === 'search') {
                        search = !v ? '' : (v.charAt(0) === '?' ? v : ('?' + v));
                    } else if (part === 'hash') {
                        hash = !v ? '' : (v.charAt(0) === '#' ? v : ('#' + v));
                    }
                    var auth = hostname + (port ? (':' + port) : '');
                    this.href = protocol + '//' + auth + path + search + hash;
                } catch (e) {
                    // keep prior value on parse failure
                }
            }
            var _locDefs = {
                origin: {
                    get: function() {
                        _locBrand.call(this);
                        var h = _locHref.call(this);
                        return h.split('/').slice(0, 3).join('/');
                    }
                },
                protocol: {
                    get: function() {
                        _locBrand.call(this);
                        var h = _locHref.call(this);
                        return h.split(':')[0] + ':';
                    },
                    set: function(v) { _locSetPart.call(this, 'protocol', v); }
                },
                host: {
                    get: function() {
                        _locBrand.call(this);
                        var h = _locHref.call(this);
                        return h.split('/')[2] || '';
                    },
                    set: function(v) { _locSetPart.call(this, 'host', v); }
                },
                hostname: {
                    get: function() {
                        _locBrand.call(this);
                        var h = _locHref.call(this);
                        return (h.split('/')[2] || '').split(':')[0];
                    },
                    set: function(v) { _locSetPart.call(this, 'hostname', v); }
                },
                port: {
                    get: function() {
                        _locBrand.call(this);
                        var h = _locHref.call(this);
                        var p = (h.split('/')[2] || '').split(':')[1];
                        return p || '';
                    },
                    set: function(v) { _locSetPart.call(this, 'port', v); }
                },
                pathname: {
                    get: function() {
                        _locBrand.call(this);
                        var h = _locHref.call(this);
                        var p = h.split('?')[0].split('#')[0];
                        return p.split('/').slice(3).join('/') ? '/' + p.split('/').slice(3).join('/') : '/';
                    },
                    set: function(v) { _locSetPart.call(this, 'pathname', v); }
                },
                search: {
                    get: function() {
                        _locBrand.call(this);
                        var h = _locHref.call(this);
                        var q = h.split('?')[1];
                        return q ? '?' + q.split('#')[0] : '';
                    },
                    set: function(v) { _locSetPart.call(this, 'search', v); }
                },
                hash: {
                    get: function() {
                        _locBrand.call(this);
                        var h = _locHref.call(this);
                        var f = h.split('#')[1];
                        return f ? '#' + f : '';
                    },
                    set: function(v) { _locSetPart.call(this, 'hash', v); }
                }
            };
            for (var _lp in _locDefs) {
                if (!Object.prototype.hasOwnProperty.call(_locDefs, _lp)) continue;
                // Always reinstall: prior own getter-only shadows broke protocol/host sets.
                (function(prop, def) {
                    Object.defineProperty(_locTarget, prop, {
                        get: def.get,
                        set: def.set,
                        enumerable: true,
                        configurable: true
                    });
                })(_lp, _locDefs[_lp]);
            }
        }
    } catch(e) {}

    // Location stringifier — Location.prototype.toString returns href
    try {
        if (typeof Location !== 'undefined' && Location.prototype) {
            Object.defineProperty(Location.prototype, 'toString', {
                value: function toString() { return this.href || ''; },
                writable: true, enumerable: false, configurable: true
            });
        }
    } catch(e) {}

    // postMessage: argument count validation + structured clone of message
    try {
        var _origPostMessage = globalThis.postMessage;
        if (_origPostMessage && typeof _origPostMessage === 'function') {
            var _wrappedPostMessage = function postMessage(message, targetOrigin, transfer) {
                if (this !== globalThis && this !== window) throw new TypeError('Illegal invocation');
                if (arguments.length < 2) throw new TypeError('2 argument(s) required, but only ' + arguments.length + ' present.');
                // Structured clone the message per HTML spec §2.9.5.
                // structuredClone is provided by window_extras.rs polyfill.
                // Fall back to JSON round-trip if unavailable or if it throws
                // (e.g. functions, Symbols, circular refs).
                var cloned;
                try {
                    cloned = (typeof structuredClone === 'function')
                        ? structuredClone(message)
                        : JSON.parse(JSON.stringify(message));
                } catch(e) {
                    cloned = JSON.parse(JSON.stringify(message));
                }
                // No actual dispatch (stub) — just accept the message.
            };
            try { Object.defineProperty(_wrappedPostMessage, 'length', { value: 2, writable: false, enumerable: false, configurable: true }); } catch(e) {}
            Object.defineProperty(globalThis, 'postMessage', { value: _wrappedPostMessage, writable: true, configurable: true, enumerable: true });
        }
    } catch(e) {}

    // Window scroll operations (no-op in headless context)
    ['scroll', 'scrollTo', 'scrollBy'].forEach(function(name) {
        if (typeof globalThis[name] === 'undefined') {
            globalThis[name] = new Function('return function ' + name + '() {}')();
            try { Object.defineProperty(globalThis[name], 'length', { value: 0, writable: false, enumerable: false, configurable: true }); } catch(e) {}
        }
    });
    // Also add to Window.prototype if it exists
    if (typeof Window !== 'undefined' && Window.prototype) {
        ['scroll', 'scrollTo', 'scrollBy'].forEach(function(name) {
            if (!Window.prototype[name]) {
                Window.prototype[name] = new Function('return function ' + name + '() {}')();
                try { Object.defineProperty(Window.prototype[name], 'length', { value: 0, writable: false, enumerable: false, configurable: true }); } catch(e) {}
            }
        });
    }

    // INHERITS_other fix: delete own properties that shadow prototype.
    // idlharness assert_inherits requires properties on the prototype chain,
    // not own properties on the instance. After codegen installs prototypes
    // (capture_codegen_prototypes), remove shadowing own props so lookups
    // resolve via the prototype chain.

    // History — set __proto__ to History.prototype.
    // Don't delete own properties — codegen prototype stubs may not work.
    try {
        if (typeof History !== 'undefined' && History.prototype && globalThis.history) {
            try { Object.setPrototypeOf(globalThis.history, History.prototype); } catch(e) {}
        }
    } catch(e) {}

    // Storage — set __proto__ to Storage.prototype.
    try {
        if (typeof Storage !== 'undefined' && Storage.prototype) {
            ['localStorage', 'sessionStorage'].forEach(function(name) {
                var obj = globalThis[name];
                if (obj && typeof obj === 'object') {
                    try { Object.setPrototypeOf(obj, Storage.prototype); } catch(e) {}
                }
            });
        }
    } catch(e) {}

    // DOMImplementation — set __proto__ to DOMImplementation.prototype
    // so codegen properties are in the prototype chain.
    // Don't delete own properties — codegen prototype methods are stubs
    // that don't actually work; the real implementations are own props.
    try {
        if (typeof DOMImplementation !== 'undefined' && DOMImplementation.prototype && document.implementation) {
            try { Object.setPrototypeOf(document.implementation, DOMImplementation.prototype); } catch(e) {}
        }
    } catch(e) {}

    // HTMLCollection — define item/namedItem on HTMLCollection.prototype
    // if missing, so assert_inherits passes for document.body.children etc.
    try {
        if (typeof HTMLCollection !== 'undefined' && HTMLCollection.prototype) {
            if (!HTMLCollection.prototype.item) {
                HTMLCollection.prototype.item = function item(index) {
                    if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present.');
                    return this[index] || null;
                };
            }
            if (!HTMLCollection.prototype.namedItem) {
                HTMLCollection.prototype.namedItem = function namedItem(name) {
                    if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present.');
                    for (var i = 0; i < this.length; i++) {
                        if (this[i].id === name || this[i].name === name) return this[i];
                    }
                    return null;
                };
            }
        }
    } catch(e) {}

    // External — set __proto__ to External.prototype
    try {
        if (typeof External !== 'undefined' && External.prototype && globalThis.external) {
            try { Object.setPrototypeOf(globalThis.external, External.prototype); } catch(e) {}
        }
    } catch(e) {}

    // MediaQueryList — wrap matchMedia so returned objects get the correct
    // prototype chain. Don't delete own properties — codegen stubs may not work.
    try {
        if (typeof MediaQueryList !== 'undefined' && MediaQueryList.prototype) {
            var origMM = globalThis.matchMedia;
            if (origMM && !origMM.__iv8MqlInheritsPatched) {
                var _mqlInheritsWrapper = function matchMedia(query) {
                    var mql = origMM.call(this, query);
                    if (mql && typeof MediaQueryList !== 'undefined' && MediaQueryList.prototype) {
                        try { Object.setPrototypeOf(mql, MediaQueryList.prototype); } catch(e) {}
                    }
                    return mql;
                };
                Object.defineProperty(_mqlInheritsWrapper, '__iv8MqlInheritsPatched', {
                    value: true, writable: true, configurable: true, enumerable: false,
                });
                globalThis.matchMedia = _mqlInheritsWrapper;
            }
        }
    } catch(e) {}

    // BarProp — window.locationbar/menubar/etc must be BarProp instances.
    // idlharness checks `window.locationbar instanceof BarProp`.
    // If codegen did not create a BarProp constructor, create one here.
    if (typeof BarProp === 'undefined') {
        function BarProp() {}
        if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {
            Object.defineProperty(BarProp.prototype, Symbol.toStringTag, {
                value: 'BarProp', writable: false, configurable: true, enumerable: false
            });
        }
        globalThis.BarProp = BarProp;
    }
    try {
        var _barProto = (typeof BarProp !== 'undefined' && BarProp.prototype)
            ? BarProp.prototype : Object.prototype;
        var _barPropInstance = Object.create(_barProto);
        Object.defineProperty(_barPropInstance, 'visible', {
            get: function() { return true; },
            enumerable: true, configurable: true
        });
        // Set all bar props to use this instance. Reinstall unconditionally
        // so existing plain objects become proper BarProp instances.
        ['locationbar', 'menubar', 'personalbar', 'scrollbars', 'statusbar', 'toolbar'].forEach(function(name) {
            var existing = globalThis[name];
            if (existing instanceof BarProp) return;
            try {
                Object.defineProperty(globalThis, name, {
                    value: _barPropInstance, writable: true,
                    configurable: true, enumerable: true
                });
            } catch(e) {}
        });
    } catch(e) {}

    // document.all — [[IsHTMLDDA]] exotic object. typeof must return
    // "undefined". This internal slot cannot be set from JS.
    // Making document.all actually undefined breaks DOM queries that
    // use document.all internally. Leave as-is for now (7 FAIL accepted).
    // try {
    //     Object.defineProperty(document, 'all', {
    //         value: undefined,
    //         writable: false,
    //         enumerable: false,
    //         configurable: true
    //     });
    // } catch(e) {}

    // Document.location: native binding installs on Document.prototype (RD-11).
    // Only fill if still missing after binding (should not happen post RD-16).
    try {
        if (typeof Document !== 'undefined' && Document.prototype) {
            if (!Object.getOwnPropertyDescriptor(Document.prototype, 'location')) {
                Object.defineProperty(Document.prototype, 'location', {
                    get: function() { return globalThis.location; },
                    enumerable: true,
                    configurable: true,
                });
            }
            // Wrap DOMImplementation.createDocument so XMLDocuments get location
            if (typeof document !== 'undefined' && document.implementation
                && document.implementation.createDocument
                && !document.implementation.createDocument.__iv8LocPatched) {
                var _origImplCreateDoc = document.implementation.createDocument;
                var _implWrapper = function createDocument(ns, name, doctype) {
                    var doc = _origImplCreateDoc.call(this, ns, name, doctype);
                    if (doc) {
                        try {
                            if (!Object.getOwnPropertyDescriptor(doc, 'location')) {
                                Object.defineProperty(doc, 'location', {
                                    value: globalThis.location,
                                    writable: false,
                                    enumerable: true,
                                    configurable: false,
                                });
                            }
                        } catch(e) {}
                    }
                    return doc;
                };
                Object.defineProperty(_implWrapper, '__iv8LocPatched', {
                    value: true, writable: true, configurable: true, enumerable: false,
                });
                document.implementation.createDocument = _implWrapper;
            }
        }
    } catch(e) {}

    // XPathResult — set __proto__ on document.evaluate() result
    try {
        var _origEvaluate = document.evaluate;
        if (_origEvaluate && typeof XPathResult !== 'undefined' && XPathResult.prototype) {
            document.evaluate = function evaluate(expr, context) {
                var result = _origEvaluate.apply(this, arguments);
                if (result && typeof result === 'object') {
                    try { Object.setPrototypeOf(result, XPathResult.prototype); } catch(e) {}
                }
                return result;
            };
        }
    } catch(e) {}

    // createTreeWalker / createNodeIterator / createExpression:
    // Do NOT wrap on document instance — wrappers zero Function.length and
    // dual-install with Document.prototype (codegen). Native/codegen path only.

    // TextTrack.cues — return empty TextTrackCueList-like object
    try {
        if (typeof HTMLMediaElement !== 'undefined' && HTMLMediaElement.prototype) {
            var origAddTT = HTMLMediaElement.prototype.addTextTrack;
            if (origAddTT && typeof origAddTT === 'function' && !origAddTT.__iv8TTpatched) {
                var wrappedAddTT = function addTextTrack(kind, label, language) {
                    var track = origAddTT.call(this, kind, label, language);
                    if (track && !track.cues) {
                        var cueList = {};
                        cueList.length = 0;
                        cueList.getCueById = function() { return null; };
                        if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {
                            Object.defineProperty(cueList, Symbol.toStringTag, {
                                value: 'TextTrackCueList', writable: false, configurable: true, enumerable: false
                            });
                        }
                        Object.defineProperty(track, 'cues', {
                            value: cueList, writable: true, configurable: true, enumerable: true
                        });
                    }
                    return track;
                };
                try { Object.defineProperty(wrappedAddTT, 'name', { value: 'addTextTrack' }); } catch(e) {}
                try { Object.defineProperty(wrappedAddTT, 'length', { value: 1, writable: false, enumerable: false, configurable: true }); } catch(e) {}
                Object.defineProperty(wrappedAddTT, '__iv8TTpatched', { value: true, writable: true, configurable: true, enumerable: false });
                Object.defineProperty(HTMLMediaElement.prototype, 'addTextTrack', { value: wrappedAddTT, writable: true, configurable: true, enumerable: true });
            }
        }
    } catch(e) {}

    // DOMStringMap — set Symbol.toStringTag on dataset
    // K-008: V8 set_accessor_property getter cannot be called via .call().
    // dataset is readonly, so fix_accessor_properties doesn't reinstall it.
    // Create DOMStringMap directly from element attributes instead.
    try {
        if (typeof HTMLElement !== 'undefined' && HTMLElement.prototype && !HTMLElement.prototype.__iv8DatasetPatched) {
            var wrappedDatasetGet = function dataset() {
                // Receiver check: must be HTMLElement instance
                if (this !== globalThis) {
                    var cur = Object.getPrototypeOf(this);
                    var valid = false;
                    for (var k = 0; k < 30; k++) {
                        if (cur === HTMLElement.prototype) { valid = true; break; }
                        if (!cur) break;
                        cur = Object.getPrototypeOf(cur);
                    }
                    if (!valid) throw new TypeError('Illegal invocation');
                }
                // Create DOMStringMap with correct prototype
                var ds = Object.create(typeof DOMStringMap !== 'undefined' ? DOMStringMap.prototype : Object.prototype);
                if (this && this.attributes) {
                    for (var i = 0; i < this.attributes.length; i++) {
                        var attr = this.attributes[i];
                        if (attr.name && attr.name.indexOf('data-') === 0) {
                            ds[attr.name.slice(5)] = attr.value;
                        }
                    }
                }
                if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {
                    try { Object.defineProperty(ds, Symbol.toStringTag, {
                        value: 'DOMStringMap', writable: false, configurable: true, enumerable: false
                    }); } catch(e) {}
                }
                return ds;
            };
            try { Object.defineProperty(wrappedDatasetGet, 'name', { value: 'get dataset' }); } catch(e) {}
            try { Object.defineProperty(wrappedDatasetGet, '__iv8_wrapped', { value: true, writable: false, enumerable: false, configurable: false }); } catch(e) {}
            Object.defineProperty(HTMLElement.prototype, 'dataset', {
                get: wrappedDatasetGet, set: undefined, enumerable: true, configurable: true
            });
            Object.defineProperty(HTMLElement.prototype, '__iv8DatasetPatched', {
                value: true, writable: true, configurable: true, enumerable: false
            });
        }
    } catch(e) {}

    if (typeof EventTarget !== 'undefined' && EventTarget.prototype && typeof Symbol !== 'undefined' && Symbol.toStringTag) {
        try {
            if (!EventTarget.prototype[Symbol.toStringTag]) {
                Object.defineProperty(EventTarget.prototype, Symbol.toStringTag, {
                    value: 'EventTarget', writable: false, configurable: true, enumerable: false
                });
            }
        } catch(e) {}
    }

    // window.chrome — configurable via globalThis.__iv8ChromePrefs
    if (typeof window !== 'undefined' && !window.chrome) {
        window.chrome = globalThis.__iv8ChromePrefs || {
            runtime: { OnInstalledReason: { CHROME_UPDATE: 'chrome_update', INSTALL: 'install', SHARED_MODULE_UPDATE: 'shared_module_update', UPDATE: 'update' }, PlatformArch: { ARM: 'arm', ARM64: 'arm64', MIPS: 'mips', MIPS64: 'mips64', X86_32: 'x86-32', X86_64: 'x86-64' } }
        };
    }

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
    function _cookieExpired(rec) {
        if (typeof rec === 'string' || !rec || typeof rec !== 'object') return false;
        if (rec.expiresTs && typeof rec.expiresTs === 'number') {
            if (Date.now() >= rec.expiresTs) return true;
        }
        return false;
    }
    function _domainMatches(cookieDomain, hostName) {
        if (!cookieDomain) return true;
        var d = cookieDomain.toLowerCase().replace(/^\./, '');
        var h = (hostName || '').toLowerCase();
        if (h === d) return true;
        return h.length > d.length && h.charAt(h.length - d.length - 1) === '.' && h.slice(-d.length) === d;
    }
    function _cookieVisible(rec) {
        if (typeof rec === 'string') return true;
        if (!rec || typeof rec !== 'object') return true;
        if (rec.httpOnly) return false;
        if (_cookieExpired(rec)) return false;
        if (rec.path && rec.path !== '/') {
            var docPath = '/';
            try { docPath = document.location ? document.location.pathname : '/'; } catch(e) {}
            if (docPath !== rec.path && docPath.indexOf(rec.path) !== 0) return false;
        }
        if (rec.domain) {
            var host = '';
            try { host = location.hostname || ''; } catch(e) {}
            if (!_domainMatches(rec.domain, host)) return false;
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
                else if (lower.indexOf('expires=') === 0) {
                    var dateStr = attr.substring(8);
                    attrs.expires = dateStr; hasAttrs = true;
                    var parsed = Date.parse(dateStr);
                    if (!isNaN(parsed)) { attrs.expiresTs = parsed; }
                }
                else if (lower.indexOf('max-age=') === 0) {
                    var ma = parseInt(attr.substring(8), 10);
                    if (!isNaN(ma)) {
                        if (ma <= 0) { delete _cookies[name]; return; }
                        attrs.maxAge = ma; hasAttrs = true;
                        attrs.expiresTs = Date.now() + (ma * 1000);
                    }
                }
            }
            if (hasAttrs) { attrs.v = value; _cookies[name] = attrs; }
            else { _cookies[name] = value; }
        },
        enumerable: true,
        configurable: true,
    });
    } catch(e) {}

// SharedWorker stub
if (typeof SharedWorker === 'undefined') {
    function SharedWorker(url, options) {
        this.port = { postMessage: function() {}, start: function() {}, close: function() {}, onmessage: null };
        this.onerror = null;
    }
    SharedWorker.prototype = Object.create(EventTarget.prototype || Object.prototype);
    try { Object.defineProperty(SharedWorker, 'length', { value: 1, writable: false, enumerable: false, configurable: true }); } catch(e) {}
    globalThis.SharedWorker = SharedWorker;
}

// ServiceWorkerContainer stub (navigator.serviceWorker already exists as stub)
if (typeof navigator !== 'undefined' && navigator.serviceWorker) {
    // Already has register stub, ensure ready returns a Promise
    if (!navigator.serviceWorker.ready) {
        Object.defineProperty(navigator.serviceWorker, 'ready', {
            get: function() { return Promise.resolve({ active: null, installing: null, waiting: null }); },
            enumerable: true, configurable: true
        });
    }
    if (!navigator.serviceWorker.controller) {
        Object.defineProperty(navigator.serviceWorker, 'controller', {
            get: function() { return null; },
            enumerable: true, configurable: true
        });
    }
}

// WorkletGlobalScope stubs — always install audioWorklet getter
if (typeof AudioContext !== 'undefined' && AudioContext.prototype) {
    try {
        Object.defineProperty(AudioContext.prototype, 'audioWorklet', {
            get: function() {
                return { addModule: function() { return Promise.resolve(); } };
            },
            enumerable: true, configurable: true
        });
    } catch(e) {}
}
})();
"#;
