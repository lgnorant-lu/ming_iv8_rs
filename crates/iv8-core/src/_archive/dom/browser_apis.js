// Browser API stubs for fingerprint bitmask detection (P0/P1 fixes).
// These make 'X' in navigator/window checks return true.
// Each stub is minimal but sufficient to pass existence checks.
(function() {
  // --- Navigator API stubs ---
  var navStubs = {
    requestMIDIAccess: function() { return Promise.resolve({ inputs: new Map(), outputs: new Map() }); },
    bluetooth: { getAvailability: function() { return Promise.resolve(false); } },
    usb: { getDevices: function() { return Promise.resolve([]); } },
    credentials: { get: function() { return Promise.resolve(null); }, create: function() { return Promise.resolve(null); } },
    clipboard: { readText: function() { return Promise.resolve(''); }, writeText: function() { return Promise.resolve(); } },
    storage: { estimate: function() { return Promise.resolve({ quota: 0, usage: 0 }); }, persist: function() { return Promise.resolve(false); } },
    wakeLock: { request: function() { return Promise.resolve({ released: false, release: function() { return Promise.resolve(); } }); } },
    locks: { request: function() { return Promise.resolve(); }, query: function() { return Promise.resolve({ held: [], pending: [] }); } },
    share: function() { return Promise.reject(new Error('not supported')); },
    getBattery: function() { return Promise.resolve({ charging: true, chargingTime: 0, dischargingTime: Infinity, level: 1 }); },
    getGamepads: function() { return [null, null, null, null]; },
    vibrate: function() { return true; },
    sendBeacon: function() { return true; },
    connection: { effectiveType: '4g', downlink: 10, rtt: 50, saveData: false, type: 'wifi' },
  };
  for (var key in navStubs) {
    if (!(key in navigator)) {
      try {
        Object.defineProperty(navigator, key, {
          value: navStubs[key], writable: true, configurable: true, enumerable: true
        });
      } catch(e) {}
    }
  }

  // --- Window API stubs ---
  if (!('customElements' in window) || !window.customElements) {
    window.customElements = {
      define: function() {},
      get: function() { return undefined; },
      whenDefined: function() { return Promise.resolve(); },
      upgrade: function() {},
    };
  }

  if (typeof window.matchMedia === 'undefined') {
    window.matchMedia = function(query) {
      return {
        matches: false,
        media: query,
        onchange: null,
        addListener: function() {},
        removeListener: function() {},
        addEventListener: function() {},
        removeEventListener: function() {},
        dispatchEvent: function() { return true; },
      };
    };
  }

  if (typeof window.getComputedStyle === 'undefined') {
    window.getComputedStyle = function(el) {
      return new Proxy({}, {
        get: function(t, p) {
          if (p === 'getPropertyValue') return function() { return ''; };
          if (p === 'length') return 0;
          return '';
        }
      });
    };
  }

  // CSS.supports
  if (typeof CSS === 'undefined') {
    window.CSS = { supports: function() { return true; }, escape: function(s) { return s; } };
  } else if (!CSS.supports) {
    CSS.supports = function() { return true; };
  }

  // --- performance.now() ---
  // Note: performance.now() returning 0 in time_freeze mode is a known issue.
  // Fix requires Rust-layer change (performance_base parameter). Not patched here
  // to avoid breaking logical time mode tests.

  // --- navigator.webdriver property descriptor fix ---
  // Real Chrome: navigator.webdriver is a data property (not accessor).
  // getOwnPropertyDescriptor should NOT return a getter function.
  // iv8-rs uses a native getter (accessor) which exposes typeof .get === 'function'.
  // Fix: redefine as data property.
  try {
    var wd = navigator.webdriver;
    Object.defineProperty(navigator, 'webdriver', {
      value: wd === undefined ? false : wd,
      writable: true, configurable: true, enumerable: true
    });
  } catch(e) {}

  // --- DOM element prototype stubs (captureStream) ---
  // Bitmask bit 1: video.captureStream / canvas.captureStream existence check
  // Note: iv8-rs DOM elements are created from Rust ObjectTemplate, so
  // HTMLVideoElement.prototype patching doesn't work. Instead, we patch
  // the createElement return value via a wrapper.
  if (typeof document !== 'undefined' && document.createElement) {
    var _origCreate = document.createElement;
    document.createElement = function(tag) {
      var el = _origCreate.call(document, tag);
      if (el) {
        var t = (tag || '').toLowerCase();
        if (t === 'video' || t === 'audio') {
          if (!el.captureStream) el.captureStream = function() { return {}; };
          if (!el.mozCaptureStream) el.mozCaptureStream = function() { return {}; };
          if (!el.webkitCaptureStream) el.webkitCaptureStream = function() { return {}; };
        }
        if (t === 'canvas') {
          if (!el.captureStream) el.captureStream = function() { return {}; };
          if (!el.webkitCaptureStream) el.webkitCaptureStream = function() { return {}; };
        }
      }
      return el;
    };
    // Preserve native code appearance
    if ('__iv8__' in globalThis && globalThis.__iv8__ && globalThis.__iv8__.wrapNative) {
      document.createElement = globalThis.__iv8__.wrapNative(document.createElement, 'createElement');
    }
  }

  // --- navigator.plugins + navigator.mimeTypes ---
  // Real Chrome has 5 PDF-related plugins. Many fingerprinters check plugins.length > 0.
  try {
    var pluginList = [
      {name: 'PDF Viewer', filename: 'internal-pdf-viewer', description: 'Portable Document Format', length: 2},
      {name: 'Chrome PDF Viewer', filename: 'internal-pdf-viewer', description: 'Portable Document Format', length: 2},
      {name: 'Chromium PDF Viewer', filename: 'internal-pdf-viewer', description: 'Portable Document Format', length: 2},
      {name: 'Microsoft Edge PDF Viewer', filename: 'internal-pdf-viewer', description: 'Portable Document Format', length: 2},
      {name: 'WebKit built-in PDF', filename: 'internal-pdf-viewer', description: 'Portable Document Format', length: 2},
    ];
    var mimeList = [
      {type: 'application/pdf', suffixes: 'pdf', description: 'Portable Document Format'},
      {type: 'text/pdf', suffixes: 'pdf', description: 'Portable Document Format'},
    ];
    // Only override if plugins is empty or has length 0
    if (!navigator.plugins || navigator.plugins.length === 0) {
      Object.defineProperty(navigator, 'plugins', {
        value: Object.assign(pluginList, {
          length: pluginList.length,
          item: function(i) { return pluginList[i] || null; },
          namedItem: function(n) { return pluginList.find(function(p){return p.name===n;}) || null; },
          refresh: function() {},
        }),
        writable: true, configurable: true, enumerable: true
      });
    }
    if (!navigator.mimeTypes || navigator.mimeTypes.length === 0) {
      Object.defineProperty(navigator, 'mimeTypes', {
        value: Object.assign(mimeList, {
          length: mimeList.length,
          item: function(i) { return mimeList[i] || null; },
          namedItem: function(t) { return mimeList.find(function(m){return m.type===t;}) || null; },
        }),
        writable: true, configurable: true, enumerable: true
      });
    }
  } catch(e) {}

  // Also webkitGetUserMedia (some checks look for this)
  if (navigator.mediaDevices && !navigator.mediaDevices.getUserMedia) {
    navigator.mediaDevices.getUserMedia = function() { return Promise.reject(new Error('not supported')); };
  }
})();
