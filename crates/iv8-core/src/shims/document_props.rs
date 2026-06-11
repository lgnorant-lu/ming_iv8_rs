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
        // Augment existing history object with navigation methods if missing
        if (!window.history.pushState) window.history.pushState = function() {};
        if (!window.history.replaceState) window.history.replaceState = function() {};
        if (!window.history.go) window.history.go = function() {};
        if (!window.history.back) window.history.back = function() {};
        if (!window.history.forward) window.history.forward = function() {};
        if (window.history.state === undefined) window.history.state = null;
    }
    // performance.timing stub (PerformanceTiming-like object)
    if (typeof performance !== 'undefined' && !performance.timing) {
        var _navStart = Date.now() - Math.floor(performance.now());
        performance.timing = {
            navigationStart: _navStart,
            unloadEventStart: 0, unloadEventEnd: 0,
            redirectStart: 0, redirectEnd: 0,
            fetchStart: _navStart,
            domainLookupStart: _navStart, domainLookupEnd: _navStart,
            connectStart: _navStart, connectEnd: _navStart,
            secureConnectionStart: 0,
            requestStart: _navStart, responseStart: _navStart, responseEnd: _navStart,
            domLoading: _navStart, domInteractive: _navStart,
            domContentLoadedEventStart: _navStart, domContentLoadedEventEnd: _navStart,
            domComplete: _navStart,
            loadEventStart: _navStart, loadEventEnd: _navStart,
        };
    }
    if (!window.devicePixelRatio) {
        window.devicePixelRatio = 1;
    }
    if (!window.innerWidth) {
        window.innerWidth = 1920;
    }
    if (!window.innerHeight) {
        window.innerHeight = 1080;
    }
    if (!window.outerWidth) {
        window.outerWidth = 1920;
    }
    if (!window.outerHeight) {
        window.outerHeight = 1080;
    }
    if (!window.screenX) {
        window.screenX = 0;
    }
    if (!window.screenY) {
        window.screenY = 0;
    }
    if (window.pageXOffset === undefined) {
        window.pageXOffset = 0;
    }
    if (window.pageYOffset === undefined) {
        window.pageYOffset = 0;
    }
    if (window.scrollX === undefined) {
        window.scrollX = 0;
    }
    if (window.scrollY === undefined) {
        window.scrollY = 0;
    }

    // ── Canvas context factory ───────────────────────────────────────────────
    // __getCanvasContext__ is called by HTMLCanvasElement.prototype.getContext
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
            this.readyState = 0; // CONNECTING
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

    // ── structuredClone polyfill ─────────────────────────────────────────────
    if (typeof structuredClone === 'undefined') {
        globalThis.structuredClone = function structuredClone(val) {
            // Deep clone via JSON for plain objects/arrays/primitives
            // Handles: null, undefined, boolean, number, string, Date, Array, Object
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
            // Plain object
            var obj = {};
            var keys = Object.keys(val);
            for (var k = 0; k < keys.length; k++) {
                obj[keys[k]] = structuredClone(val[keys[k]]);
            }
            return obj;
        };
    }

    // ── MutationObserver stub ────────────────────────────────────────────────
    if (typeof MutationObserver === 'undefined') {
        globalThis.MutationObserver = function MutationObserver(callback) {
            this._callback = callback;
            this._targets = [];
        };
        MutationObserver.prototype.observe = function(target, options) {
            this._targets.push({ target: target, options: options });
        };
        MutationObserver.prototype.disconnect = function() {
            this._targets = [];
        };
        MutationObserver.prototype.takeRecords = function() { return []; };
    }

    // ── IntersectionObserver stub ────────────────────────────────────────────
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

    // ── ResizeObserver stub ──────────────────────────────────────────────────
    if (typeof ResizeObserver === 'undefined') {
        globalThis.ResizeObserver = function ResizeObserver(callback) {
            this._callback = callback;
        };
        ResizeObserver.prototype.observe = function(target, options) {};
        ResizeObserver.prototype.unobserve = function(target) {};
        ResizeObserver.prototype.disconnect = function() {};
    }

    // ── Blob stub ────────────────────────────────────────────────────────────
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

    // ── URL.createObjectURL / revokeObjectURL ────────────────────────────────
    if (typeof URL !== 'undefined' && !URL.createObjectURL) {
        var _blobCounter = 0;
        URL.createObjectURL = function createObjectURL(obj) {
            return 'blob:null/' + (++_blobCounter) + '-' + Math.random().toString(36).slice(2);
        };
        URL.revokeObjectURL = function revokeObjectURL(url) {};
    }

    // ── requestIdleCallback / cancelIdleCallback ─────────────────────────────
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

    // ── AudioContext / OfflineAudioContext stubs ─────────────────────────────
    // FingerprintJS uses AudioContext for audio fingerprinting.
    // We provide a realistic stub that supports the fingerprint collection pattern.
    (function() {
        // AudioParam stub
        function AudioParam(value) {
            this.value = value !== undefined ? value : 0;
            this.defaultValue = this.value;
            this.minValue = -3.4028234663852886e+38;
            this.maxValue = 3.4028234663852886e+38;
            this.automationRate = 'a-rate';
        }
        AudioParam.prototype.setValueAtTime = function(v, t) { this.value = v; return this; };
        AudioParam.prototype.linearRampToValueAtTime = function(v, t) { return this; };
        AudioParam.prototype.exponentialRampToValueAtTime = function(v, t) { return this; };
        AudioParam.prototype.setTargetAtTime = function(v, t, tc) { return this; };
        AudioParam.prototype.setValueCurveAtTime = function(vs, t, d) { return this; };
        AudioParam.prototype.cancelScheduledValues = function(t) { return this; };
        AudioParam.prototype.cancelAndHoldAtTime = function(t) { return this; };

        // AudioNode base
        function AudioNode(ctx) {
            this.context = ctx;
            this.numberOfInputs = 0;
            this.numberOfOutputs = 1;
            this.channelCount = 2;
            this.channelCountMode = 'max';
            this.channelInterpretation = 'speakers';
        }
        AudioNode.prototype.connect = function(dest) { return dest; };
        AudioNode.prototype.disconnect = function() {};
        AudioNode.prototype.addEventListener = function() {};
        AudioNode.prototype.removeEventListener = function() {};
        AudioNode.prototype.dispatchEvent = function() { return true; };

        // OscillatorNode
        function OscillatorNode(ctx, options) {
            AudioNode.call(this, ctx);
            this.type = (options && options.type) || 'sine';
            this.frequency = new AudioParam((options && options.frequency) || 440);
            this.detune = new AudioParam(0);
            this.onended = null;
        }
        OscillatorNode.prototype = Object.create(AudioNode.prototype);
        OscillatorNode.prototype.start = function(when) {};
        OscillatorNode.prototype.stop = function(when) {};

        // DynamicsCompressorNode
        function DynamicsCompressorNode(ctx, options) {
            AudioNode.call(this, ctx);
            this.threshold = new AudioParam((options && options.threshold !== undefined) ? options.threshold : -24);
            this.knee = new AudioParam((options && options.knee !== undefined) ? options.knee : 30);
            this.ratio = new AudioParam((options && options.ratio !== undefined) ? options.ratio : 12);
            this.attack = new AudioParam((options && options.attack !== undefined) ? options.attack : 0.003);
            this.release = new AudioParam((options && options.release !== undefined) ? options.release : 0.25);
            this.reduction = 0;
        }
        DynamicsCompressorNode.prototype = Object.create(AudioNode.prototype);

        // AnalyserNode
        function AnalyserNode(ctx, options) {
            AudioNode.call(this, ctx);
            this.fftSize = (options && options.fftSize) || 2048;
            this.frequencyBinCount = this.fftSize / 2;
            this.minDecibels = -100;
            this.maxDecibels = -30;
            this.smoothingTimeConstant = 0.8;
        }
        AnalyserNode.prototype = Object.create(AudioNode.prototype);
        AnalyserNode.prototype.getFloatFrequencyData = function(arr) {};
        AnalyserNode.prototype.getByteFrequencyData = function(arr) {};
        AnalyserNode.prototype.getFloatTimeDomainData = function(arr) {};
        AnalyserNode.prototype.getByteTimeDomainData = function(arr) {};

        // GainNode
        function GainNode(ctx, options) {
            AudioNode.call(this, ctx);
            this.gain = new AudioParam((options && options.gain !== undefined) ? options.gain : 1);
        }
        GainNode.prototype = Object.create(AudioNode.prototype);

        // AudioDestinationNode
        function AudioDestinationNode(ctx) {
            AudioNode.call(this, ctx);
            this.maxChannelCount = 2;
            this.numberOfInputs = 1;
            this.numberOfOutputs = 0;
        }
        AudioDestinationNode.prototype = Object.create(AudioNode.prototype);

        // AudioBuffer stub
        function AudioBuffer(options) {
            this.sampleRate = (options && options.sampleRate) || 44100;
            this.length = (options && options.length) || 0;
            this.duration = this.length / this.sampleRate;
            this.numberOfChannels = (options && options.numberOfChannels) || 1;
            this._data = new Float32Array(this.length);
        }
        AudioBuffer.prototype.getChannelData = function(channel) {
            // Return slightly varied data for fingerprinting (non-zero, non-uniform)
            var data = new Float32Array(this.length);
            // Simulate audio processing output with small values
            for (var i = 0; i < Math.min(data.length, 100); i++) {
                data[i] = Math.sin(i * 0.1) * 0.0001;
            }
            return data;
        };
        AudioBuffer.prototype.copyFromChannel = function(dest, channel, offset) {};
        AudioBuffer.prototype.copyToChannel = function(src, channel, offset) {};

        // BaseAudioContext
        function BaseAudioContext(sampleRate) {
            this.sampleRate = sampleRate || 44100;
            this.currentTime = 0;
            this.destination = new AudioDestinationNode(this);
            this.listener = {};
            this.state = 'suspended';
            this.onstatechange = null;
        }
        BaseAudioContext.prototype.createOscillator = function(options) {
            return new OscillatorNode(this, options);
        };
        BaseAudioContext.prototype.createDynamicsCompressor = function(options) {
            return new DynamicsCompressorNode(this, options);
        };
        BaseAudioContext.prototype.createAnalyser = function(options) {
            return new AnalyserNode(this, options);
        };
        BaseAudioContext.prototype.createGain = function(options) {
            return new GainNode(this, options);
        };
        BaseAudioContext.prototype.createBuffer = function(channels, length, sampleRate) {
            return new AudioBuffer({ numberOfChannels: channels, length: length, sampleRate: sampleRate });
        };
        BaseAudioContext.prototype.createBufferSource = function() {
            var node = new AudioNode(this);
            node.buffer = null;
            node.loop = false;
            node.loopStart = 0;
            node.loopEnd = 0;
            node.playbackRate = new AudioParam(1);
            node.detune = new AudioParam(0);
            node.onended = null;
            node.start = function() {};
            node.stop = function() {};
            return node;
        };
        BaseAudioContext.prototype.createScriptProcessor = function(bufferSize, inputChannels, outputChannels) {
            var node = new AudioNode(this);
            node.bufferSize = bufferSize || 4096;
            node.onaudioprocess = null;
            return node;
        };
        BaseAudioContext.prototype.createChannelSplitter = function(n) {
            var node = new AudioNode(this);
            node.numberOfOutputs = n || 6;
            return node;
        };
        BaseAudioContext.prototype.createChannelMerger = function(n) {
            var node = new AudioNode(this);
            node.numberOfInputs = n || 6;
            return node;
        };
        BaseAudioContext.prototype.createConvolver = function() {
            var node = new AudioNode(this);
            node.buffer = null;
            node.normalize = true;
            return node;
        };
        BaseAudioContext.prototype.createDelay = function(maxDelay) {
            var node = new AudioNode(this);
            node.delayTime = new AudioParam(0);
            return node;
        };
        BaseAudioContext.prototype.createBiquadFilter = function() {
            var node = new AudioNode(this);
            node.type = 'lowpass';
            node.frequency = new AudioParam(350);
            node.detune = new AudioParam(0);
            node.Q = new AudioParam(1);
            node.gain = new AudioParam(0);
            node.getFrequencyResponse = function() {};
            return node;
        };
        BaseAudioContext.prototype.createWaveShaper = function() {
            var node = new AudioNode(this);
            node.curve = null;
            node.oversample = 'none';
            return node;
        };
        BaseAudioContext.prototype.createStereoPanner = function() {
            var node = new AudioNode(this);
            node.pan = new AudioParam(0);
            return node;
        };
        BaseAudioContext.prototype.createPanner = function() {
            var node = new AudioNode(this);
            node.panningModel = 'equalpower';
            node.distanceModel = 'inverse';
            node.positionX = new AudioParam(0);
            node.positionY = new AudioParam(0);
            node.positionZ = new AudioParam(0);
            node.orientationX = new AudioParam(1);
            node.orientationY = new AudioParam(0);
            node.orientationZ = new AudioParam(0);
            node.refDistance = 1;
            node.maxDistance = 10000;
            node.rolloffFactor = 1;
            node.coneInnerAngle = 360;
            node.coneOuterAngle = 0;
            node.coneOuterGain = 0;
            return node;
        };
        BaseAudioContext.prototype.decodeAudioData = function(buffer, successCb, errorCb) {
            var ab = new AudioBuffer({ length: 1, sampleRate: this.sampleRate });
            if (successCb) { setTimeout(function() { successCb(ab); }, 0); return; }
            return Promise.resolve(ab);
        };
        BaseAudioContext.prototype.resume = function() { this.state = 'running'; return Promise.resolve(); };
        BaseAudioContext.prototype.suspend = function() { this.state = 'suspended'; return Promise.resolve(); };
        BaseAudioContext.prototype.close = function() { this.state = 'closed'; return Promise.resolve(); };
        BaseAudioContext.prototype.addEventListener = function() {};
        BaseAudioContext.prototype.removeEventListener = function() {};
        BaseAudioContext.prototype.dispatchEvent = function() { return true; };

        // AudioContext
        function AudioContext(options) {
            BaseAudioContext.call(this, options && options.sampleRate);
            this.baseLatency = 0.005;
            this.outputLatency = 0.01;
        }
        AudioContext.prototype = Object.create(BaseAudioContext.prototype);
        AudioContext.prototype.constructor = AudioContext;
        AudioContext.prototype.getOutputTimestamp = function() {
            return { contextTime: this.currentTime, performanceTime: performance.now() };
        };
        AudioContext.prototype.createMediaStreamSource = function(stream) { return new AudioNode(this); };
        AudioContext.prototype.createMediaStreamDestination = function() {
            var node = new AudioNode(this);
            node.stream = { getTracks: function() { return []; }, getAudioTracks: function() { return []; } };
            return node;
        };
        AudioContext.prototype.createMediaElementSource = function(el) { return new AudioNode(this); };

        // OfflineAudioContext
        function OfflineAudioContext(numberOfChannels, length, sampleRate) {
            // Support both (channels, length, sampleRate) and ({numberOfChannels, length, sampleRate})
            if (typeof numberOfChannels === 'object') {
                var opts = numberOfChannels;
                numberOfChannels = opts.numberOfChannels || 1;
                length = opts.length || 44100;
                sampleRate = opts.sampleRate || 44100;
            }
            BaseAudioContext.call(this, sampleRate);
            this.length = length;
            this.numberOfChannels = numberOfChannels;
            this._buffer = new AudioBuffer({ numberOfChannels: numberOfChannels, length: length, sampleRate: sampleRate });
        }
        OfflineAudioContext.prototype = Object.create(BaseAudioContext.prototype);
        OfflineAudioContext.prototype.constructor = OfflineAudioContext;
        OfflineAudioContext.prototype.startRendering = function() {
            var self = this;
            return Promise.resolve(self._buffer);
        };
        OfflineAudioContext.prototype.suspend = function(suspendTime) { return Promise.resolve(); };
        OfflineAudioContext.prototype.resume = function() { return Promise.resolve(); };

        // Install on globalThis (always override — tier1_stubs may have installed empty stubs)
        globalThis.AudioContext = AudioContext;
        globalThis.webkitAudioContext = AudioContext;
        globalThis.OfflineAudioContext = OfflineAudioContext;
        globalThis.AudioBuffer = AudioBuffer;
        globalThis.AudioNode = AudioNode;
        globalThis.AudioParam = AudioParam;
        globalThis.GainNode = GainNode;
        globalThis.OscillatorNode = OscillatorNode;
        globalThis.AnalyserNode = AnalyserNode;
        globalThis.DynamicsCompressorNode = DynamicsCompressorNode;
        globalThis.BaseAudioContext = BaseAudioContext;
    })();

})();
"#;
