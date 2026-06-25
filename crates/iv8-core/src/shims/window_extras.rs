//! Window properties, global constructors, structuredClone, Blob, performance timing.
//!
//! Extracted from document_props.rs for code organization.
//! Dependencies: performance.now() (events/page_api.rs), setTimeout (events/timers.rs)

pub const WINDOW_EXTRAS_JS: &str = r#"
(function() {
    if (typeof window === 'undefined') return;

    // window.addEventListener/removeEventListener/dispatchEvent
    if (!window.addEventListener) {
        window.addEventListener = function(type, listener, options) {};
    }
    if (!window.removeEventListener) {
        window.removeEventListener = function(type, listener, options) {};
    }
    if (!window.dispatchEvent) {
        window.dispatchEvent = function(event) { return true; };
    }
    if (!window.postMessage) {
        window.postMessage = function(msg, origin) {};
    }
    if (!window.history) {
        window.history = { pushState: function() {}, replaceState: function() {}, go: function() {}, back: function() {}, forward: function() {}, length: 1, state: null };
    } else {
        if (!window.history.pushState) window.history.pushState = function() {};
        if (!window.history.replaceState) window.history.replaceState = function() {};
        if (!window.history.go) window.history.go = function() {};
        if (!window.history.back) window.history.back = function() {};
        if (!window.history.forward) window.history.forward = function() {};
        if (window.history.state === undefined) window.history.state = null;
    }

    // Ensure window.top/self/parent/frames identity (may be overwritten by codegen)
    try {
        Object.defineProperty(window, 'top', { get: function() { return window; }, configurable: true });
        Object.defineProperty(window, 'self', { get: function() { return window; }, configurable: true });
        Object.defineProperty(window, 'parent', { get: function() { return window; }, configurable: true });
        Object.defineProperty(window, 'frames', { get: function() { return window; }, configurable: true });
    } catch(e) {}

    // HTMLDocument constructor (RS VMP checks document instanceof HTMLDocument)
    if (typeof HTMLDocument === 'undefined' && typeof document !== 'undefined') {
        function HTMLDocument() {}
        HTMLDocument.prototype = Object.getPrototypeOf(document) || {};
        Object.defineProperty(document, 'constructor', { value: HTMLDocument, writable: true, configurable: true });
        globalThis.HTMLDocument = HTMLDocument;
    }

    // performance.timing stub
    if (typeof performance !== 'undefined' && !performance.timing) {
        var _navStart = Date.now() - Math.floor(performance.now());
        performance.timing = {
            navigationStart: _navStart,
            unloadEventStart: 0, unloadEventEnd: 0,
            redirectStart: 0, redirectEnd: 0,
            fetchStart: _navStart + 2,
            domainLookupStart: _navStart + 5,
            domainLookupEnd: _navStart + 10,
            connectStart: _navStart + 12,
            connectEnd: _navStart + 20,
            secureConnectionStart: 0,
            requestStart: _navStart + 22,
            responseStart: _navStart + 50,
            responseEnd: _navStart + 150,
            domLoading: _navStart + 160,
            domInteractive: _navStart + 300,
            domContentLoadedEventStart: _navStart + 500,
            domContentLoadedEventEnd: _navStart + 510,
            domComplete: _navStart + 600,
            loadEventStart: _navStart + 700,
            loadEventEnd: _navStart + 710,
        };
    }
    if (!window.screenX) { window.screenX = 0; }
    if (!window.screenY) { window.screenY = 0; }
    if (window.pageXOffset === undefined) { window.pageXOffset = 0; }
    if (window.pageYOffset === undefined) { window.pageYOffset = 0; }
    if (window.scrollX === undefined) { window.scrollX = 0; }
    if (window.scrollY === undefined) { window.scrollY = 0; }

    // Canvas context factory
    if (!window.__getCanvasContext__) {
        window.__getCanvasContext__ = function(type) {
            if (type === '2d') {
                return {
                    canvas: null,
                    fillStyle: '#000000', strokeStyle: '#000000',
                    lineWidth: 1, font: '10px sans-serif',
                    textAlign: 'start', textBaseline: 'alphabetic',
                    globalAlpha: 1, globalCompositeOperation: 'source-over',
                    shadowBlur: 0, shadowColor: 'rgba(0,0,0,0)',
                    shadowOffsetX: 0, shadowOffsetY: 0,
                    lineCap: 'butt', lineJoin: 'miter', miterLimit: 10,
                    imageSmoothingEnabled: true,
                    fillRect: function() {}, strokeRect: function() {},
                    clearRect: function() {}, fillText: function() {},
                    strokeText: function() {},
                    measureText: function(text) {
                        var font = this.font || '10px sans-serif';
                        var sizeMatch = font.match(/(\d+(?:\.\d+)?)(px|pt|em)/);
                        var fontSize = sizeMatch ? parseFloat(sizeMatch[1]) : 10;
                        var isMonospace = /monospace|courier|mono/i.test(font);
                        var isSerif = /serif/i.test(font) && !/sans-serif/i.test(font);
                        var charWidth = isMonospace ? fontSize * 0.6 : isSerif ? fontSize * 0.55 : fontSize * 0.5;
                        var width = (text || '').length * charWidth;
                        return { width: width, actualBoundingBoxAscent: fontSize * 0.8,
                                 actualBoundingBoxDescent: fontSize * 0.2,
                                 actualBoundingBoxLeft: 0, actualBoundingBoxRight: width,
                                 fontBoundingBoxAscent: fontSize, fontBoundingBoxDescent: fontSize * 0.25 };
                    },
                    beginPath: function() {}, closePath: function() {},
                    moveTo: function() {}, lineTo: function() {},
                    arc: function() {}, arcTo: function() {},
                    bezierCurveTo: function() {}, quadraticCurveTo: function() {},
                    rect: function() {}, fill: function() {}, stroke: function() {},
                    clip: function() {}, save: function() {}, restore: function() {},
                    scale: function() {}, rotate: function() {}, translate: function() {},
                    transform: function() {}, setTransform: function() {}, resetTransform: function() {},
                    createLinearGradient: function() { return {addColorStop: function(){}}; },
                    createRadialGradient: function() { return {addColorStop: function(){}}; },
                    createPattern: function() { return null; },
                    drawImage: function() {},
                    createImageData: function(w, h) {
                        return {width: w, height: h, data: new Uint8ClampedArray(w*h*4)};
                    },
                    getImageData: function(x, y, w, h) {
                        return {width: w, height: h, data: new Uint8ClampedArray(w*h*4)};
                    },
                    putImageData: function() {},
                    getLineDash: function() { return []; },
                    setLineDash: function() {},
                    isPointInPath: function() { return false; },
                    isPointInStroke: function() { return false; },
                };
            }
            if (type === 'webgl' || type === 'experimental-webgl' ||
                type === 'webgl2' || type === 'experimental-webgl2') {
                return window.__webglContext__ || null;
            }
            return null;
        };
    }

    // WebSocket stub
    if (!window.WebSocket) {
        window.WebSocket = function WebSocket(url, protocols) {
            this.url = url;
            this.readyState = 0;
            this.CONNECTING = 0; this.OPEN = 1; this.CLOSING = 2; this.CLOSED = 3;
            this.onopen = null; this.onclose = null; this.onmessage = null; this.onerror = null;
            this.send = function() {};
            this.close = function() { this.readyState = 3; };
            this.addEventListener = function() {};
            this.removeEventListener = function() {};
        };
        window.WebSocket.CONNECTING = 0; window.WebSocket.OPEN = 1;
        window.WebSocket.CLOSING = 2; window.WebSocket.CLOSED = 3;
    }

    // indexedDB stub
    if (!window.indexedDB) {
        window.indexedDB = {
            open: function(name, version) {
                var req = { result: null, error: null, onsuccess: null, onerror: null, onupgradeneeded: null };
                setTimeout(function() { if (req.onerror) req.onerror({}); }, 0);
                return req;
            },
            deleteDatabase: function() { return {}; },
            databases: function() { return Promise.resolve([]); },
        };
    }

    // structuredClone polyfill
    if (typeof structuredClone === 'undefined') {
        globalThis.structuredClone = function structuredClone(val) {
            if (val === null || val === undefined) return val;
            if (typeof val !== 'object' && typeof val !== 'function') return val;
            if (val instanceof Date) return new Date(val.getTime());
            if (val instanceof RegExp) return new RegExp(val.source, val.flags);
            if (Array.isArray(val)) {
                var arr = [];
                for (var i = 0; i < val.length; i++) arr.push(structuredClone(val[i]));
                return arr;
            }
            if (val instanceof ArrayBuffer) {
                var buf = new ArrayBuffer(val.byteLength);
                new Uint8Array(buf).set(new Uint8Array(val));
                return buf;
            }
            if (ArrayBuffer.isView(val)) {
                var Ctor = val.constructor;
                return new Ctor(structuredClone(val.buffer), val.byteOffset, val.length !== undefined ? val.length : undefined);
            }
            var obj = {};
            var keys = Object.keys(val);
            for (var k = 0; k < keys.length; k++) {
                obj[keys[k]] = structuredClone(val[keys[k]]);
            }
            return obj;
        };
    }

    // MutationObserver stub
    if (typeof MutationObserver === 'undefined') {
        globalThis.MutationObserver = function MutationObserver(callback) {
            this._callback = callback;
            this._targets = [];
        };
        MutationObserver.prototype.observe = function(target, options) {
            this._targets.push({ target: target, options: options });
        };
        MutationObserver.prototype.disconnect = function() { this._targets = []; };
        MutationObserver.prototype.takeRecords = function() { return []; };
    }

    // IntersectionObserver stub
    if (typeof IntersectionObserver === 'undefined') {
        globalThis.IntersectionObserver = function IntersectionObserver(callback, options) {
            this._callback = callback;
            this._options = options || {};
            this.root = null;
            this.rootMargin = '0px';
            this.thresholds = [0];
        };
        IntersectionObserver.prototype.observe = function(target) {};
        IntersectionObserver.prototype.unobserve = function(target) {};
        IntersectionObserver.prototype.disconnect = function() {};
        IntersectionObserver.prototype.takeRecords = function() { return []; };
    }

    // ResizeObserver stub
    if (typeof ResizeObserver === 'undefined') {
        globalThis.ResizeObserver = function ResizeObserver(callback) {
            this._callback = callback;
        };
        ResizeObserver.prototype.observe = function(target, options) {};
        ResizeObserver.prototype.unobserve = function(target) {};
        ResizeObserver.prototype.disconnect = function() {};
        ResizeObserver.prototype.takeRecords = function() { return []; };
    }

    // ReportingObserver stub
    if (typeof ReportingObserver === 'undefined') {
        globalThis.ReportingObserver = function ReportingObserver(callback, options) {
            this._callback = callback;
        };
        ReportingObserver.prototype.observe = function() {};
        ReportingObserver.prototype.disconnect = function() {};
        ReportingObserver.prototype.takeRecords = function() { return []; };
    }

    // Blob stub
    if (typeof Blob === 'undefined') {
        globalThis.Blob = function Blob(parts, options) {
            this._parts = parts || [];
            this._options = options || {};
            this.type = (options && options.type) || '';
            var size = 0;
            if (parts) {
                for (var i = 0; i < parts.length; i++) {
                    var p = parts[i];
                    if (typeof p === 'string') size += p.length;
                    else if (p && p.byteLength !== undefined) size += p.byteLength;
                    else if (p && p.size !== undefined) size += p.size;
                }
            }
            this.size = size;
        };
        Blob.prototype.text = function() {
            var parts = this._parts;
            var text = '';
            for (var i = 0; i < parts.length; i++) {
                if (typeof parts[i] === 'string') text += parts[i];
            }
            return Promise.resolve(text);
        };
        Blob.prototype.arrayBuffer = function() {
            return Promise.resolve(new ArrayBuffer(this.size));
        };
        Blob.prototype.slice = function(start, end, type) {
            return new Blob([], { type: type || this.type });
        };
        Blob.prototype.stream = function() {
            return { getReader: function() { return { read: function() { return Promise.resolve({done:true,value:undefined}); }, cancel: function() {} }; } };
        };
    }

    // URL.createObjectURL / revokeObjectURL
    if (typeof URL !== 'undefined' && !URL.createObjectURL) {
        var _blobCounter = 0;
        URL.createObjectURL = function createObjectURL(obj) {
            return 'blob:null/' + (++_blobCounter) + '-' + Math.random().toString(36).slice(2);
        };
        URL.revokeObjectURL = function revokeObjectURL(url) {};
    }

    // speechSynthesis stub (SpeechSynthesis API)
    if (typeof speechSynthesis === 'undefined') {
        globalThis.speechSynthesis = {
            pending: false,
            speaking: false,
            paused: false,
            onvoiceschanged: null,
            onstart: null,
            onend: null,
            onerror: null,
            onpause: null,
            onresume: null,
            onmark: null,
            onboundary: null,
            getVoices: function() { return []; },
            speak: function() {},
            cancel: function() {},
            pause: function() {},
            resume: function() {},
            addEventListener: function() {},
            removeEventListener: function() {},
            dispatchEvent: function() { return true; },
        };
    }

    // navigator.speechSynthesis alias
    if (typeof navigator !== 'undefined' && !navigator.speechSynthesis) {
        Object.defineProperty(navigator, 'speechSynthesis', {
            get: function() { return globalThis.speechSynthesis; },
            configurable: true,
        });
    }

    // PerformanceObserver stub
    if (typeof PerformanceObserver === 'undefined') {
        globalThis.PerformanceObserver = function PerformanceObserver(callback) {
            this._callback = callback;
        };
        PerformanceObserver.prototype.observe = function(options) {};
        PerformanceObserver.prototype.disconnect = function() {};
        PerformanceObserver.prototype.takeRecords = function() { return []; };
        PerformanceObserver.supportedEntryTypes = [
            'element', 'event', 'first-input', 'largest-contentful-paint',
            'layout-shift', 'longtask', 'mark', 'measure', 'navigation',
            'paint', 'resource'
        ];
    }

    // PerformanceEntry stub (base class)
    if (typeof PerformanceEntry === 'undefined') {
        globalThis.PerformanceEntry = function PerformanceEntry() {};
        PerformanceEntry.prototype.name = '';
        PerformanceEntry.prototype.entryType = '';
        PerformanceEntry.prototype.startTime = 0;
        PerformanceEntry.prototype.duration = 0;
        PerformanceEntry.prototype.toJSON = function() {
            return { name: this.name, entryType: this.entryType,
                     startTime: this.startTime, duration: this.duration };
        };
    }

    // performance.getEntries / getEntriesByName / getEntriesByType stubs
    if (typeof performance !== 'undefined') {
        if (!performance.getEntries) {
            performance.getEntries = function() { return []; };
        }
        if (!performance.getEntriesByName) {
            performance.getEntriesByName = function() { return []; };
        }
        if (!performance.getEntriesByType) {
            performance.getEntriesByType = function() { return []; };
        }
        if (!performance.mark) {
            performance.mark = function(name) {};
        }
        if (!performance.measure) {
            performance.measure = function(name, startMark, endMark) {};
        }
        if (!performance.clearMarks) {
            performance.clearMarks = function() {};
        }
        if (!performance.clearMeasures) {
            performance.clearMeasures = function() {};
        }
        if (!performance.now) {
            performance.now = function() { return Date.now() - (performance.timeOrigin || 0); };
        }
    }

    // CookieStore API stub (modern cookie access)
    if (typeof CookieStore === 'undefined') {
        globalThis.CookieStore = function CookieStore() {};
        CookieStore.prototype.get = function(name) { return Promise.resolve(null); };
        CookieStore.prototype.set = function(name, value) { return Promise.resolve(); };
        CookieStore.prototype.delete = function(name) { return Promise.resolve(); };
        CookieStore.prototype.getAll = function() { return Promise.resolve([]); };
        if (typeof window !== 'undefined' && !window.cookieStore) {
            window.cookieStore = new CookieStore();
        }
    }

    // CookieStore onchange event stub
    if (typeof window !== 'undefined' && window.cookieStore && !window.cookieStore.addEventListener) {
        window.cookieStore.addEventListener = function() {};
        window.cookieStore.removeEventListener = function() {};
    }

    // RTCPeerConnection ICE stub (WebRTC fingerprint)
    // codegen provides constructor skeleton; we add behavioral stubs
    if (typeof RTCPeerConnection !== 'undefined' && !RTCPeerConnection.prototype.createOffer) {
        RTCPeerConnection.prototype.createOffer = function(options) {
            return Promise.resolve({ type: 'offer', sdp: '' });
        };
        RTCPeerConnection.prototype.createAnswer = function(options) {
            return Promise.resolve({ type: 'answer', sdp: '' });
        };
        RTCPeerConnection.prototype.setLocalDescription = function(desc) {
            this.localDescription = desc;
            return Promise.resolve();
        };
        RTCPeerConnection.prototype.setRemoteDescription = function(desc) {
            this.remoteDescription = desc;
            return Promise.resolve();
        };
        RTCPeerConnection.prototype.addIceCandidate = function(candidate) {
            return Promise.resolve();
        };
        RTCPeerConnection.prototype.close = function() { this.connectionState = 'closed'; };
        RTCPeerConnection.prototype.getStats = function() {
            return Promise.resolve(new Map());
        };
        RTCPeerConnection.prototype.addTransceiver = function(track) { return {}; };
        RTCPeerConnection.prototype.addTrack = function(track) { return {}; };
    }

    // Chromium Feature Flags stub (navigator.userAgentData feature detection)
    // Chrome exposes these via Object.defineProperty on navigator or window
    if (typeof window !== 'undefined' && !window.__iv8FeatureFlags) {
        var _flags = {
            FencedFrames: true,
            FencedFramesAPIChanges: false,
            FencedFramesDefaultMode: false,
            FencedFramesLocalUnpartitionedDataAccess: false,
            SharedArrayBufferEnabled: false,
            ModelExecutionAPI: true,
            TrustedTypeBeforePolicyCreationEvent: false,
            AdInterestGroupAPI: true,
            Fledge: true,
            AllowURNsInIframes: true,
            AllowURNsInIframe: false,
            FledgeNegativeTargeting: true,
            FledgeClearOriginJoinedAdInterestGroups: true,
            FledgeFeatureDetection: true,
            EnforceAnonymityExposure: true,
            InstalledApp: true,
            CookieDeprecationFacilitatedTesting: false,
            AttributionReportingInterface: true,
            SharedStorageAPIM118: true,
            NavigationId: false,
            CrossFramePerformanceTimeline: false,
            CSSKeyframesRuleLength: true,
            ManagedConfiguration: true,
            DeviceAttributes: false,
            Focusgroup: true,
            FetchLaterAPI: false,
            UACHOverrideBlank: false,
            HTMLElementScrollParent: false,
            LateWindowProperties: false,
            WebGPUExperimentalFeatures: false,
            WebGPUDeveloperFeatures: false,
        };
        // Merge profile-driven overrides from globalThis.__iv8FeatureFlagPrefs
        var _flagPrefs = (typeof globalThis.__iv8FeatureFlagPrefs === 'object' && globalThis.__iv8FeatureFlagPrefs) ? globalThis.__iv8FeatureFlagPrefs : {};
        for (var _fk in _flagPrefs) {
            if (_fk in _flags) _flags[_fk] = _flagPrefs[_fk];
        }
        Object.defineProperty(window, '__iv8FeatureFlags', {
            value: _flags, writable: false, configurable: false, enumerable: false,
        });
    }

    // Ad Block Detection resistance: offsetParent returns non-null for bait elements.
    // Real ad blockers set display:none on bait elements, making offsetParent null.
    // We ensure bait-like elements return normal offsetParent (document.body).
    // This is a passive defense — no active intervention needed beyond
    // ensuring getBoundingClientRect/offsetParent return normal values.

    // Extension Detection resistance: chrome-extension:// URLs return 404 (not found).
    // Real browsers without the extension return net::ERR_FAILED.
    // We intercept fetch/XHR to chrome-extension:// to return rejection.
    if (typeof window !== 'undefined' && !window.__iv8ExtDetectionGuard) {
        var _origFetch = typeof fetch !== 'undefined' ? fetch : null;
        if (_origFetch) {
            var _wrappedFetch = function(input, init) {
                var url = typeof input === 'string' ? input : (input && input.url) || '';
                if (url.indexOf('chrome-extension://') === 0) {
                    return Promise.reject(new TypeError('Failed to fetch'));
                }
                return _origFetch.call(this, input, init);
            };
            // Preserve native code toString for anti-detection
            _wrappedFetch.toString = function() { return _origFetch.toString(); };
            fetch = _wrappedFetch;
        }
        Object.defineProperty(window, '__iv8ExtDetectionGuard', {
            value: true, writable: false, configurable: false, enumerable: false,
        });
    }

    // Custom Elements (customElements) stub
    if (typeof customElements !== 'undefined' && !customElements.define) {
        customElements._registry = {};
        customElements.define = function(name, constructor, options) {
            customElements._registry[name] = { constructor: constructor, options: options };
        };
        customElements.get = function(name) {
            var entry = customElements._registry[name];
            return entry ? entry.constructor : undefined;
        };
        customElements.getName = function(constructor) {
            var keys = Object.keys(customElements._registry);
            for (var i = 0; i < keys.length; i++) {
                if (customElements._registry[keys[i]].constructor === constructor) return keys[i];
            }
            return null;
        };
        customElements.upgrade = function(root) {};
        customElements.whenDefined = function(name) {
            return Promise.resolve(customElements._registry[name] ? constructor : undefined);
        };
    }

    // queryLocalFonts API stub (Font Detection)
    if (typeof queryLocalFonts === 'undefined') {
        globalThis.queryLocalFonts = function queryLocalFonts(options) {
            return Promise.resolve([]);
        };
    }

    // Managed Device config (navigator.managed) stub
    if (typeof navigator !== 'undefined' && !navigator.managed) {
        var _managed = {
            deviceId: '',
            organizationName: '',
            annotatedAssetId: '',
            annotatedLocation: '',
            directoryId: '',
            hostname: '',
            serialNumber: '',
        };
        Object.defineProperty(navigator, 'managed', {
            get: function() { return _managed; },
            configurable: true,
        });
    }

    // Range stub (DOM Range API)
    if (typeof Range === 'undefined') {
        globalThis.Range = function Range() {
            this.startContainer = null;
            this.startOffset = 0;
            this.endContainer = null;
            this.endOffset = 0;
            this.collapsed = true;
            this.commonAncestorContainer = null;
        };
        Range.prototype.setStart = function(node, offset) {
            this.startContainer = node; this.startOffset = offset;
        };
        Range.prototype.setEnd = function(node, offset) {
            this.endContainer = node; this.endOffset = offset;
        };
        Range.prototype.setStartBefore = function(node) { this.startContainer = node; };
        Range.prototype.setStartAfter = function(node) { this.startContainer = node; };
        Range.prototype.setEndBefore = function(node) { this.endContainer = node; };
        Range.prototype.setEndAfter = function(node) { this.endContainer = node; };
        Range.prototype.selectNode = function(node) {
            this.startContainer = node; this.endContainer = node;
        };
        Range.prototype.selectNodeContents = function(node) {
            this.startContainer = node; this.endContainer = node;
        };
        Range.prototype.collapse = function(toStart) { this.collapsed = true; };
        Range.prototype.cloneRange = function() {
            var r = new Range();
            r.startContainer = this.startContainer; r.startOffset = this.startOffset;
            r.endContainer = this.endContainer; r.endOffset = this.endOffset;
            return r;
        };
        Range.prototype.detach = function() {};
        Range.prototype.toString = function() { return ''; };
        Range.prototype.getBoundingClientRect = function() {
            return { x: 0, y: 0, width: 0, height: 0, top: 0, left: 0, bottom: 0, right: 0 };
        };
        Range.prototype.getClientRects = function() { return []; };
        Range.prototype.insertNode = function(node) {};
        Range.prototype.deleteContents = function() {};
        Range.prototype.extractContents = function() { return document.createDocumentFragment(); };
        Range.prototype.surroundContents = function(newParent) {};
        Range.prototype.cloneContents = function() { return document.createDocumentFragment(); };
        Range.START_TO_START = 0; Range.START_TO_END = 1;
        Range.END_TO_END = 2; Range.END_TO_START = 3;
    }

    // P1: Notification.requestPermission returns Promise
    if (typeof Notification !== 'undefined' && Notification.requestPermission) {
        Notification.requestPermission = function() {
            return Promise.resolve(Notification.permission || 'default');
        };
    }

    // P1: BroadcastChannel name echo
    if (typeof BroadcastChannel !== 'undefined') {
        var _origBCProto = BroadcastChannel.prototype;
        globalThis.BroadcastChannel = function BroadcastChannel(name) {
            this.name = name || '';
            this.onmessage = null;
            this.onmessageerror = null;
        };
        globalThis.BroadcastChannel.prototype = _origBCProto;
        if (!_origBCProto.postMessage) _origBCProto.postMessage = function(data) {};
        if (!_origBCProto.close) _origBCProto.close = function() {};
    }

    // P1: navigator.getInstalledRelatedApps returns Promise
    if (typeof navigator !== 'undefined' && typeof navigator.getInstalledRelatedApps === 'function') {
        navigator.getInstalledRelatedApps = function() { return Promise.resolve([]); };
    }

    // P1: navigator.wakeLock.request returns Promise
    if (typeof navigator !== 'undefined' && navigator.wakeLock && typeof navigator.wakeLock.request === 'undefined') {
        navigator.wakeLock.request = function(type) {
            return Promise.resolve({
                type: type || 'screen', released: false,
                release: function() { this.released = true; return Promise.resolve(); },
                addEventListener: function() {}, removeEventListener: function() {},
                dispatchEvent: function() { return true; }
            });
        };
    }

    // P1: PaymentRequest show/canMakePayment return Promises
    if (typeof PaymentRequest !== 'undefined') {
        PaymentRequest.prototype.show = function() { return Promise.reject(new DOMException('AbortError')); };
        PaymentRequest.prototype.canMakePayment = function() { return Promise.resolve(false); };
        PaymentRequest.prototype.abort = function() { return Promise.reject(new DOMException('InvalidStateError')); };
    }

    // P1: desktop-only APIs should be undefined
    if (typeof navigator !== 'undefined') {
        try {
            if (navigator.contacts && Object.keys(navigator.contacts).length === 0) {
                Object.defineProperty(navigator, 'contacts', { value: undefined, writable: true, configurable: true });
            }
            if (navigator.virtualKeyboard && Object.keys(navigator.virtualKeyboard).length === 0) {
                Object.defineProperty(navigator, 'virtualKeyboard', { value: undefined, writable: true, configurable: true });
            }
        } catch(e) {}
    }
})();
"#;
