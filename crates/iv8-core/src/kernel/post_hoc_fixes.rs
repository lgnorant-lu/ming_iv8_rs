//! Post-hoc JS fixes applied after all surface/shim installation.
//!
//! These fixes run in `freeze_all_prototypes` (non-worker) or
//! `install_browser_surface_init` (worker path). They modify the V8 object
//! graph after all FunctionTemplates and shims are instantiated.
//!
//! ## Why JS instead of Rust?
//!
//! Most fixes here need to **inspect** the V8 object graph after installation
//! (e.g., "if navigator.plugins is not instanceof PluginArray, wrap it").
//! This requires V8 API calls that are verbose and error-prone in Rust but
//! trivial in JS. Moving them to Rust would require equivalent
//! `ObjectTemplate::get`/`set`/`delete` chains with manual scope management.
//!
//! ## Organization
//!
//! Each blob is a `pub const &str` with a doc comment explaining:
//! - What it fixes
//! - Why it can't be done in codegen/shim/native
//! - Dependencies on other blobs (ordering)

/// P0 boundary fix: delete navigator.webdriver from Navigator.prototype.
///
/// Real Chrome: `Object.getOwnPropertyDescriptor(Navigator.prototype, 'webdriver') === undefined`.
/// IV8 codegen installs it as a getter returning false (it's in the WebIDL).
/// JS fix matches Chrome by deleting the property post-install.
///
/// **Why not codegen?** The WebIDL includes `webdriver` as a readonly attribute.
/// Codegen faithfully installs it. Chrome removes it at runtime. A codegen
/// annotation like `[ChromeOmit]` would work but is a bigger architectural change.
pub const WEBDRIVER_FIX_JS: &str = r#"
    (function() {
        try { delete Navigator.prototype.webdriver; } catch(e) {}
    })();
"#;

/// P0 boundary fix: patch document.createElement toString to return [native code].
///
/// Shim exposes JS source in toString(). Real Chrome returns
/// `function createElement() { [native code] }`.
///
/// **Why not codegen?** createElement is a DOM template operation (dom/template.rs).
/// The DOM template installs a callback that doesn't set toString. A native
/// fix would require `v8::Function::new` with a name, but DOM template uses
/// `ObjectTemplate::set` which creates internal slots.
pub const CREATE_ELEMENT_FIX_JS: &str = r#"
    (function() {
        if (typeof document !== 'undefined' && document.createElement) {
            var orig = document.createElement;
            var origStr = orig.toString();
            if (origStr.indexOf('[native code]') < 0) {
                var patched = function createElement(tagName) { return orig.call(document, tagName); };
                patched.toString = function() { return 'function createElement() { [native code] }'; };
                patched.toString.toString = function() { return 'function toString() { [native code] }'; };
                try { Object.defineProperty(document, 'createElement', { value: patched, writable: true, configurable: true, enumerable: true }); } catch(e) {}
            }
        }
    })();
"#;

/// P0 boundary fix: navigator.plugins/mimeTypes instanceof check.
///
/// Shim replaces plugins/mimeTypes with plain objects. Real Chrome returns
/// PluginArray/MimeTypeArray instances. This wraps the shim output with
/// the correct prototype.
///
/// **Why not codegen?** navigator.plugins is installed by navigator_extras.rs
/// shim (not codegen) because it needs runtime data (plugin list). The shim
/// creates plain objects for simplicity. This fix wraps them post-hoc.
pub const PLUGINS_FIX_JS: &str = r#"
    (function() {
        if (typeof PluginArray === 'undefined' || typeof MimeTypeArray === 'undefined') return;
        if (typeof navigator === 'undefined' || !navigator.plugins) return;
        if (!(navigator.plugins instanceof PluginArray)) {
            var realPlugins = navigator.plugins;
            var pa = Object.create(PluginArray.prototype);
            for (var i = 0; i < realPlugins.length; i++) {
                pa[i] = realPlugins[i];
            }
            pa.length = realPlugins.length;
            pa.item = function(i) { return realPlugins[i]; };
            pa.namedItem = function(n) { return realPlugins[n]; };
            pa[Symbol.toStringTag] = 'PluginArray';
            try { Object.defineProperty(navigator, 'plugins', { value: pa, writable: true, configurable: true, enumerable: true }); } catch(e) {}
        }
        if (!(navigator.mimeTypes instanceof MimeTypeArray)) {
            var realMT = navigator.mimeTypes;
            var ma = Object.create(MimeTypeArray.prototype);
            for (var i = 0; i < realMT.length; i++) {
                ma[i] = realMT[i];
            }
            ma.length = realMT.length;
            ma.item = function(i) { return realMT[i]; };
            ma.namedItem = function(n) { return realMT[n]; };
            ma[Symbol.toStringTag] = 'MimeTypeArray';
            try { Object.defineProperty(navigator, 'mimeTypes', { value: ma, writable: true, configurable: true, enumerable: true }); } catch(e) {}
        }
    })();
"#;

/// P0-BT-5 fix: iframe contentWindow.navigator missing.
///
/// Root cause: contentWindow getter returns bare Object or null (looks for
/// nonexistent "WindowProxy" global). Fix: wrap contentWindow to create a
/// Window-like proxy with navigator.
///
/// **Why not codegen?** contentWindow is a DOM template getter. The DOM
/// template returns a stored value (may be null). This fix wraps the getter
/// to install navigator/document/parent on the returned object.
pub const IFRAME_FIX_JS: &str = r#"
    (function() {
        if (typeof HTMLIFrameElement === 'undefined') return;
        var proto = HTMLIFrameElement.prototype;
        var origGetter = Object.getOwnPropertyDescriptor(proto, 'contentWindow');
        if (!origGetter || !origGetter.get) return;
        var origGet = origGetter.get;
        Object.defineProperty(proto, 'contentWindow', {
            get: function contentWindow() {
                var cw = origGet.call(this);
                if (!cw || typeof cw !== 'object') {
                    cw = {};
                }
                if (!cw.navigator) {
                    try {
                        Object.defineProperty(cw, 'navigator', {
                            get: function() { return navigator; },
                            enumerable: true,
                            configurable: true,
                        });
                    } catch(e) {}
                }
                if (!cw.document) {
                    try {
                        Object.defineProperty(cw, 'document', {
                            get: function() { return this._contentDocument || document; },
                            enumerable: true,
                            configurable: true,
                        });
                    } catch(e) {}
                }
                if (!('parent' in cw)) {
                    try {
                        Object.defineProperty(cw, 'parent', { value: window, enumerable: true, configurable: true });
                        Object.defineProperty(cw, 'top', { value: window, enumerable: true, configurable: true });
                        Object.defineProperty(cw, 'self', { value: cw, enumerable: true, configurable: true });
                        Object.defineProperty(cw, 'window', { value: cw, enumerable: true, configurable: true });
                    } catch(e) {}
                }
                return cw;
            },
            set: undefined,
            enumerable: true,
            configurable: true,
        });
    })();
"#;

/// Element.prototype.shadowRoot returns null + attachShadow stub.
///
/// Root cause: DOM template installs shadowRoot as a getter returning {}
/// (empty object). Real Chrome returns null when no shadow root is attached.
/// VMP checks this API and takes wrong branch.
///
/// **Why not codegen?** shadowRoot is a DOM template getter. The DOM template
/// callback returns a default value. This fix wraps the getter to return
/// the stored __iv8_shadowRoot or null.
pub const SHADOW_ROOT_FIX_JS: &str = r#"
    (function() {
        if (typeof Element === 'undefined' || typeof Element.prototype === 'undefined') {
            return;
        }
        try {
            var oldDesc = Object.getOwnPropertyDescriptor(Element.prototype, 'shadowRoot');
            if (!oldDesc) return;
            Object.defineProperty(Element.prototype, 'shadowRoot', {
                get: function() {
                    if (!this || typeof this !== 'object') return null;
                    return this.__iv8_shadowRoot || null;
                },
                enumerable: true, configurable: true
            });
        } catch(e) { }
        if (typeof Element.prototype.attachShadow === 'function') {
            Element.prototype.attachShadow = function(init) {
                var root = {};
                try { root = Object.create(ShadowRoot.prototype); } catch(ex) {}
                this.__iv8_shadowRoot = root;
                return root;
            };
        }
    })();
"#;

/// Request constructor shim.
///
/// Codegen creates empty object for Request. This shim stores url/method/headers.
///
/// **Why not codegen?** Request constructor needs to parse input (string or
/// Request object) and init dict. Codegen constructors are empty templates.
/// A proper fix would be a hand-implemented constructor in hand_implemented/.
/// TODO: move to hand_implemented/fetch.rs (v0.8.90).
pub const REQUEST_FIX_JS: &str = r#"
    (function() {
        if (typeof Request === 'undefined') return;
        var origCtor = Request;
        function RequestShim(input, init) {
            var url = '';
            if (typeof input === 'string') {
                url = input;
            } else if (input && typeof input === 'object' && input.url) {
                url = input.url;
            }
            var method = (init && init.method) || 'GET';
            Object.defineProperty(this, 'url', { value: url, writable: true, configurable: true, enumerable: true });
            Object.defineProperty(this, 'method', { value: method, writable: true, configurable: true, enumerable: true });
            Object.defineProperty(this, 'headers', { value: (init && init.headers) || {}, writable: true, configurable: true, enumerable: true });
            Object.defineProperty(this, 'body', { value: (init && init.body) || null, writable: true, configurable: true, enumerable: true });
            Object.defineProperty(this, 'cache', { value: 'default', writable: true, configurable: true, enumerable: true });
            Object.defineProperty(this, 'credentials', { value: 'same-origin', writable: true, configurable: true, enumerable: true });
            Object.defineProperty(this, 'destination', { value: '', writable: true, configurable: true, enumerable: true });
            Object.defineProperty(this, 'integrity', { value: '', writable: true, configurable: true, enumerable: true });
            Object.defineProperty(this, 'keepalive', { value: false, writable: true, configurable: true, enumerable: true });
            Object.defineProperty(this, 'mode', { value: 'cors', writable: true, configurable: true, enumerable: true });
            Object.defineProperty(this, 'redirect', { value: 'follow', writable: true, configurable: true, enumerable: true });
            Object.defineProperty(this, 'referrer', { value: 'about:client', writable: true, configurable: true, enumerable: true });
            Object.defineProperty(this, 'referrerPolicy', { value: '', writable: true, configurable: true, enumerable: true });
            Object.defineProperty(this, 'signal', { value: null, writable: true, configurable: true, enumerable: true });
        }
        RequestShim.prototype = origCtor.prototype;
        try { Object.defineProperty(globalThis, 'Request', {
            value: RequestShim, writable: true, configurable: true, enumerable: true
        }); } catch(e) {}
    })();
"#;

/// Fix readonly attribute setters: idlharness expects setter=undefined
/// for readonly attributes. Some accessor wrappers install a JS setter.
///
/// **Why not codegen?** fix_accessor_properties (codegen) already handles
/// readonly, but shim-installed accessors (event_constructors.rs) don't
/// check readonly. This fix runs after shims to enforce readonly.
pub const READONLY_FIX_JS: &str = r#"
    (function() {
        var readonlyAttrs = {
            'Event': ['type','target','currentTarget','srcElement','eventPhase',
                      'bubbles','cancelable','timeStamp','defaultPrevented','composed'],
            'MouseEvent': ['screenX','screenY','clientX','clientY','ctrlKey','shiftKey',
                           'altKey','metaKey','button','buttons','relatedTarget','region'],
            'CustomEvent': ['detail'],
        };
        for (var iface in readonlyAttrs) {
            var ctor = globalThis[iface];
            if (!ctor || !ctor.prototype) continue;
            var attrs = readonlyAttrs[iface];
            for (var i = 0; i < attrs.length; i++) {
                var desc = Object.getOwnPropertyDescriptor(ctor.prototype, attrs[i]);
                if (desc && desc.get && desc.set) {
                    try {
                        Object.defineProperty(ctor.prototype, attrs[i], {
                            get: desc.get, set: undefined,
                            enumerable: desc.enumerable, configurable: true
                        });
                    } catch(e) {}
                }
            }
        }
    })();
"#;

/// Fix operation .name and .length on key prototypes.
///
/// Codegen sets .name via set_class_name but V8 may not persist it on the
/// function object. .length may be wrong for overloaded ops.
///
/// **Why not codegen?** V8's ObjectTemplate::set call handler creates
/// internal function objects. set_class_name sets the display name but
/// not Function.name. The fix uses Object.defineProperty to set it post-hoc.
/// TODO: investigate if rusty_v8 exposes Function.name setter (v0.8.90).
pub const NAME_LENGTH_FIX_JS: &str = r#"
    (function() {
        var fixes = {
            'Window': { 'postMessage': { name: 'postMessage', length: 1 } },
            'HTMLCanvasElement': {
                'getContext': { name: 'getContext', length: 1 },
                'toDataURL': { name: 'toDataURL', length: 0 }
            },
            'CanvasRenderingContext2D': {
                'setTransform': { name: 'setTransform', length: 0 }
            },
            'OffscreenCanvasRenderingContext2D': {
                'setTransform': { name: 'setTransform', length: 0 },
                'createLinearGradient': { name: 'createLinearGradient', length: 4 },
                'createRadialGradient': { name: 'createRadialGradient', length: 6 },
                'createConicGradient': { name: 'createConicGradient', length: 1 },
                'drawImage': { name: 'drawImage', length: 3 },
                'fillText': { name: 'fillText', length: 3 },
                'strokeText': { name: 'strokeText', length: 3 },
                'putImageData': { name: 'putImageData', length: 3 },
            },
        };
        for (var ifaceName in fixes) {
            try {
                var ctor = globalThis[ifaceName];
                if (!ctor || !ctor.prototype) continue;
                var proto = ctor.prototype;
                var ifaceFixes = fixes[ifaceName];
                for (var opName in ifaceFixes) {
                    try {
                        var fn = proto[opName];
                        if (!fn || typeof fn !== 'function') continue;
                        var fix = ifaceFixes[opName];
                        if (fn.name !== fix.name) {
                            try { Object.defineProperty(fn, 'name', {
                                value: fix.name, writable: false,
                                enumerable: false, configurable: true
                            }); } catch(e) {}
                        }
                        if (fn.length !== fix.length) {
                            try { Object.defineProperty(fn, 'length', {
                                value: fix.length, writable: false,
                                enumerable: false, configurable: true
                            }); } catch(e) {}
                        }
                    } catch(e) {}
                }
            } catch(e) {}
        }
        try {
            var pm = globalThis.postMessage;
            if (pm && typeof pm === 'function') {
                if (pm.name !== 'postMessage') {
                    try { Object.defineProperty(pm, 'name', {
                        value: 'postMessage', writable: false,
                        enumerable: false, configurable: true
                    }); } catch(e) {}
                }
                if (pm.length !== 1) {
                    try { Object.defineProperty(pm, 'length', {
                        value: 1, writable: false,
                        enumerable: false, configurable: true
                    }); } catch(e) {}
                }
            }
        } catch(e) {}

        try {
            if (typeof Event !== 'undefined' && Event.prototype) {
                var ie = Event.prototype.initEvent;
                if (ie && typeof ie === 'function' && ie.length !== 1) {
                    try { Object.defineProperty(ie, 'length', {
                        value: 1, writable: false,
                        enumerable: false, configurable: true
                    }); } catch(e) {}
                }
            }
        } catch(e) {}

        try {
            if (typeof CanvasRenderingContext2D !== 'undefined' && CanvasRenderingContext2D.prototype) {
                var cid = CanvasRenderingContext2D.prototype.createImageData;
                if (cid && typeof cid === 'function' && cid.length !== 1) {
                    try { Object.defineProperty(cid, 'length', {
                        value: 1, writable: false,
                        enumerable: false, configurable: true
                    }); } catch(e) {}
                }
            }
        } catch(e) {}

        try {
            if (typeof OffscreenCanvasRenderingContext2D !== 'undefined' && OffscreenCanvasRenderingContext2D.prototype) {
                var oproto = OffscreenCanvasRenderingContext2D.prototype;
                var ocid = oproto.createImageData;
                if (ocid && typeof ocid === 'function' && ocid.length !== 1) {
                    try { Object.defineProperty(ocid, 'length', {
                        value: 1, writable: false,
                        enumerable: false, configurable: true
                    }); } catch(e) {}
                }
                var occg = oproto.createConicGradient;
                if (occg && typeof occg === 'function' && occg.length !== 3) {
                    try { Object.defineProperty(occg, 'length', {
                        value: 3, writable: false,
                        enumerable: false, configurable: true
                    }); } catch(e) {}
                }
            }
        } catch(e) {}

        try {
            if (typeof Navigator !== 'undefined' && Navigator.prototype) {
                var rph = Navigator.prototype.registerProtocolHandler;
                if (rph && typeof rph === 'function' && rph.length !== 2) {
                    try { Object.defineProperty(rph, 'length', {
                        value: 2, writable: false,
                        enumerable: false, configurable: true
                    }); } catch(e) {}
                }
                var uph = Navigator.prototype.unregisterProtocolHandler;
                if (uph && typeof uph === 'function' && uph.length !== 2) {
                    try { Object.defineProperty(uph, 'length', {
                        value: 2, writable: false,
                        enumerable: false, configurable: true
                    }); } catch(e) {}
                }
            }
        } catch(e) {}

        try {
            if (typeof HTMLMediaElement !== 'undefined' && HTMLMediaElement.prototype) {
                var origCPT = HTMLMediaElement.prototype.canPlayType;
                if (origCPT && typeof origCPT === 'function') {
                    var wCPT = function canPlayType(type) {
                        if (arguments.length < 1) throw new TypeError("1 argument(s) required, but only 0 present.");
                        return origCPT.call(this, type);
                    };
                    try { Object.defineProperty(wCPT, 'length', { value: 1, writable: false, enumerable: false, configurable: true }); } catch(e) {}
                    try { Object.defineProperty(wCPT, 'name', { value: 'canPlayType', writable: false, enumerable: false, configurable: true }); } catch(e) {}
                    Object.defineProperty(HTMLMediaElement.prototype, 'canPlayType', { value: wCPT, writable: true, configurable: true, enumerable: true });
                }
            }
        } catch(e) {}

        try {
            if (typeof HTMLCanvasElement !== 'undefined' && HTMLCanvasElement.prototype) {
                var origGC = HTMLCanvasElement.prototype.getContext;
                if (origGC && typeof origGC === 'function') {
                    var wGC = function getContext(contextId, options) {
                        if (arguments.length < 1) throw new TypeError("1 argument(s) required, but only 0 present.");
                        return origGC.call(this, contextId, options);
                    };
                    try { Object.defineProperty(wGC, 'length', { value: 1, writable: false, enumerable: false, configurable: true }); } catch(e) {}
                    try { Object.defineProperty(wGC, 'name', { value: 'getContext', writable: false, enumerable: false, configurable: true }); } catch(e) {}
                    Object.defineProperty(HTMLCanvasElement.prototype, 'getContext', { value: wGC, writable: true, configurable: true, enumerable: true });
                }
            }
        } catch(e) {}
    })();
"#;

/// CDP diff fix: window.chrome should have runtime:{}.
///
/// Note: document.all [[IsHTMLDDA]] cannot be fixed from JS
/// (see document_props.rs:1403 comment).
pub const CHROME_FIX_JS: &str = r#"
    (function() {
        try {
            if (typeof window.chrome === 'object' && window.chrome) {
                if (!window.chrome.runtime) {
                    try { Object.defineProperty(window.chrome, 'runtime', {
                        value: {}, writable: true, enumerable: true, configurable: true
                    }); } catch(e) {}
                }
            }
        } catch(e) {}
    })();
"#;

/// Delete Worker-only globals in Window mode.
///
/// WorkerGlobalScope, DedicatedWorkerGlobalScope, etc. should not be
/// visible in Window context. Codegen installs them because they're in
/// the WebIDL, but real Chrome doesn't expose them on window.
pub const WORKER_ONLY_DELETE_JS: &str = r#"
    (function() {
        var workerOnly = ['WorkerGlobalScope','DedicatedWorkerGlobalScope',
            'SharedWorkerGlobalScope','ServiceWorkerGlobalScope',
            'WorkerNavigator','WorkerLocation','WorkletGlobalScope',
            'AnimationWorkletGlobalScope','AudioWorkletGlobalScope',
            'LayoutWorkletGlobalScope','PaintWorkletGlobalScope',
            'RTCIdentityProviderGlobalScope'];
        for (var i = 0; i < workerOnly.length; i++) {
            try { delete globalThis[workerOnly[i]]; } catch(e) {}
        }
    })();
"#;

/// Freeze all constructor prototypes (non-writable, non-configurable).
///
/// idlharness checks that X.prototype is not writable and
/// Object.setPrototypeOf(X.prototype, {}) throws TypeError.
/// Codegen interfaces already use read_only_prototype(), but
/// JS shim constructors (Event, MessageChannel, etc.) do not.
pub const FREEZE_SHIM_PROTOTYPES_JS: &str = r#"
    (function() {
        var names = ['Event','CustomEvent','MouseEvent','KeyboardEvent','PointerEvent',
            'MessageChannel','MessagePort','BroadcastChannel','Worker',
            'Location','Navigator','Screen','DOMRect','DOMException',
            'AudioContext','OfflineAudioContext','AudioBuffer','AudioNode','AudioParam'];
        for (var i = 0; i < names.length; i++) {
            var name = names[i];
            try {
                var ctor = globalThis[name];
                if (ctor && typeof ctor === 'function') {
                    Object.defineProperty(ctor, 'prototype', {writable: false, enumerable: false, configurable: false});
                }
            } catch(e) {}
        }
    })();
"#;

/// Freeze ALL prototypes on globalThis + Window.crypto/performance accessors +
/// FileReader chain + ScreenOrientation setup.
///
/// This is the final freeze pass. It:
/// 1. Makes all constructor.prototype non-writable, non-configurable
/// 2. Prevents extensions on all prototypes
/// 3. Installs Window.crypto/performance accessors
/// 4. Sets FileReader → EventTarget prototype chain
/// 5. Moves ScreenOrientation methods to prototype
pub const FREEZE_ALL_JS: &str = r#"
    (function() {
        var names = Object.getOwnPropertyNames(globalThis);
        for (var i = 0; i < names.length; i++) {
            try {
                var ctor = globalThis[names[i]];
                if (ctor && typeof ctor === 'function' && ctor.prototype) {
                    Object.defineProperty(ctor, 'prototype', {
                        writable: false, enumerable: false, configurable: false
                    });
                    Object.preventExtensions(ctor.prototype);
                }
            } catch(e) {}
        }
    })();
    (function() {
        if (typeof Window !== 'undefined' && Window.prototype && Object.isExtensible(Window.prototype) && typeof crypto !== 'undefined') {
            var _cryptoVal = crypto;
            Object.defineProperty(Window.prototype, 'crypto', {
                get: function() { return _cryptoVal; },
                enumerable: true, configurable: true
            });
        }
    })();
    (function() {
        if (typeof FileReader !== 'undefined' && typeof EventTarget !== 'undefined') {
            Object.setPrototypeOf(FileReader, EventTarget);
        }
        try { delete globalThis.FileReaderSync; } catch(e) {}
    })();
    (function() {
        if (typeof screen === 'undefined' || typeof ScreenOrientation === 'undefined') return;
        var so = screen.orientation;
        if (!so) return;
        var soProto = ScreenOrientation.prototype;
        if (!soProto) return;
        var soNames = Object.getOwnPropertyNames(so);
        for (var i = 0; i < soNames.length; i++) {
            var prop = soNames[i];
            if (typeof so[prop] === 'function' && !soProto[prop]) {
                soProto[prop] = so[prop];
                delete so[prop];
            }
        }
        Object.setPrototypeOf(so, soProto);
        if (typeof EventTarget !== 'undefined') {
            Object.setPrototypeOf(ScreenOrientation, EventTarget);
        }
        if (typeof Screen !== 'undefined' && Screen.prototype && Object.isExtensible(Screen.prototype)) {
            var _soVal = so;
            Object.defineProperty(Screen.prototype, 'orientation', {
                get: function() { return _soVal; },
                enumerable: true, configurable: true
            });
        }
    })();
    (function() {
        if (typeof Performance !== 'undefined' && typeof EventTarget !== 'undefined') {
            Object.setPrototypeOf(Performance, EventTarget);
        }
        if (typeof Window !== 'undefined' && Window.prototype && Object.isExtensible(Window.prototype)) {
            var _perfVal = performance;
            Object.defineProperty(Window.prototype, 'performance', {
                get: function() { return _perfVal; },
                enumerable: true, configurable: true
            });
        }
        if (typeof Window !== 'undefined' && Window.prototype && Object.isExtensible(Window.prototype) && typeof crypto !== 'undefined') {
            var _cryptoVal = crypto;
            Object.defineProperty(Window.prototype, 'crypto', {
                get: function() { return _cryptoVal; },
                enumerable: true, configurable: true
            });
        }
    })();
"#;

/// Fix all getter .name properties: codegen uses set_class_name which
/// doesn't set Function.name. Iterate all prototypes and set
/// getter.name = "get " + attrName for accessor getters.
/// Skip [native code] getters (V8 FunctionTemplate internals are not configurable).
pub const GETTER_NAME_FIX_JS: &str = r#"
    (function() {
        var ctors = Object.getOwnPropertyNames(globalThis);
        for (var i = 0; i < ctors.length; i++) {
            try {
                var c = globalThis[ctors[i]];
                if (!c || !c.prototype) continue;
                var proto = c.prototype;
                var names = Object.getOwnPropertyNames(proto);
                for (var j = 0; j < names.length; j++) {
                    var pn = names[j];
                    if (pn === 'constructor') continue;
                    try {
                        var desc = Object.getOwnPropertyDescriptor(proto, pn);
                        if (!desc || !desc.get) continue;
                        var g = desc.get;
                        if (typeof g !== 'function') continue;
                        var gStr = '';
                        try { gStr = g.toString(); } catch(e) {}
                        if (gStr.indexOf('[native code]') !== -1) continue;
                        if (g.name !== 'get ' + pn) {
                            try { Object.defineProperty(g, 'name', {
                                value: 'get ' + pn, writable: false,
                                enumerable: false, configurable: true
                            }); } catch(e) {}
                        }
                        if (g.length !== 0) {
                            try { Object.defineProperty(g, 'length', {
                                value: 0, writable: false,
                                enumerable: false, configurable: true
                            }); } catch(e) {}
                        }
                        if (desc.set && typeof desc.set === 'function') {
                            var sStr = '';
                            try { sStr = desc.set.toString(); } catch(e) {}
                            if (sStr.indexOf('[native code]') === -1) {
                                var s = desc.set;
                                if (s.name !== 'set ' + pn) {
                                    try { Object.defineProperty(s, 'name', {
                                        value: 'set ' + pn, writable: false,
                                        enumerable: false, configurable: true
                                    }); } catch(e) {}
                                }
                            }
                        }
                    } catch(e) {}
                }
            } catch(e) {}
        }
    })();
"#;

/// R10-5: Fix descriptor issues.
///
/// - LegacyUnforgeable: configurable=false for window/document/location/top
/// - Event.isTrusted: should be accessor not data property
/// - stringifier enumerable=true
/// - Worker interface objects: enumerable=false (commented out — conflicts with worker_shim)
pub const DESCRIPTOR_FIX_JS: &str = r#"
    (function() {
        var unforgeable = ['window', 'document', 'top'];
        for (var i = 0; i < unforgeable.length; i++) {
            try {
                var desc = Object.getOwnPropertyDescriptor(globalThis, unforgeable[i]);
                if (desc && desc.configurable) {
                    var newDesc = { configurable: false };
                    if (desc.get) { newDesc.get = desc.get; newDesc.set = desc.set; newDesc.enumerable = desc.enumerable !== false; }
                    else { newDesc.value = desc.value; newDesc.writable = desc.writable; newDesc.enumerable = desc.enumerable !== false; }
                    try { Object.defineProperty(globalThis, unforgeable[i], newDesc); } catch(e) {}
                }
            } catch(e) {}
        }

        try {
            var fd = Object.getOwnPropertyDescriptor(globalThis, 'frames');
            if (fd && fd.enumerable === false) {
                try { Object.defineProperty(globalThis, 'frames', { enumerable: true, configurable: true }); } catch(e) {}
            }
        } catch(e) {}

        try {
            if (typeof Event !== 'undefined' && Event.prototype) {
                var itd = Object.getOwnPropertyDescriptor(Event.prototype, 'isTrusted');
                if (itd && 'value' in itd) {
                    var val = itd.value;
                    Object.defineProperty(Event.prototype, 'isTrusted', {
                        get: function() { return val; },
                        set: undefined,
                        enumerable: true, configurable: true
                    });
                }
            }
        } catch(e) {}

        try {
            if (typeof Location !== 'undefined' && Location.prototype) {
                var locAttrs = ['href', 'search'];
                for (var j = 0; j < locAttrs.length; j++) {
                    var ld = Object.getOwnPropertyDescriptor(Location.prototype, locAttrs[j]);
                    if (ld && 'value' in ld) {
                        (function(attr, desc) {
                            var v = desc.value;
                            Object.defineProperty(Location.prototype, attr, {
                                get: function() { return v; },
                                set: undefined,
                                enumerable: desc.enumerable !== false, configurable: true
                            });
                        })(locAttrs[j], ld);
                    }
                }
            }
        } catch(e) {}

        try {
            if (typeof HTMLAnchorElement !== 'undefined' && HTMLAnchorElement.prototype) {
                var sd = Object.getOwnPropertyDescriptor(HTMLAnchorElement.prototype, 'toString');
                if (sd && sd.enumerable === false) {
                    try { Object.defineProperty(HTMLAnchorElement.prototype, 'toString', { enumerable: true, configurable: true }); } catch(e) {}
                }
            }
        } catch(e) {}
        try {
            if (typeof HTMLAreaElement !== 'undefined' && HTMLAreaElement.prototype) {
                var sd2 = Object.getOwnPropertyDescriptor(HTMLAreaElement.prototype, 'toString');
                if (sd2 && sd2.enumerable === false) {
                    try { Object.defineProperty(HTMLAreaElement.prototype, 'toString', { enumerable: true, configurable: true }); } catch(e) {}
                }
            }
        } catch(e) {}

        try { delete globalThis.external; } catch(e) {}
    })();
"#;

/// Fix codegen native getters that throw "Illegal invocation" on DOM template
/// instances (K-013). Codegen FunctionTemplate callbacks require V8 internal
/// slots that DOM template instances (created via Object.create) don't have.
///
/// **Affected**: CharacterData.length, Text.wholeText, Element.regionOverset
///
/// **Why not codegen?** The codegen getter has a receiver check that validates
/// V8 internal slots. DOM template instances bypass this. A JS shim getter
/// reads from a hidden property instead.
///
/// **Why not DOM template?** DOM template instances are created via
/// `Object.create(Interface.prototype)` which doesn't have V8 slots.
/// Making them use FunctionTemplate would require deep architecture change.
pub const DOM_GETTER_FIX_JS: &str = r#"
    (function() {
        function _installGetter(proto, name, getter) {
            try {
                Object.defineProperty(proto, name, {
                    get: getter,
                    set: undefined,
                    enumerable: true,
                    configurable: true
                });
            } catch(e) {}
        }

        // CharacterData.length — number of characters in the text node
        if (typeof CharacterData !== 'undefined' && CharacterData.prototype) {
            _installGetter(CharacterData.prototype, 'length', function() {
                return (this._data || this.data || '').length;
            });
        }

        // Text.wholeText — concatenation of all sibling text nodes
        if (typeof Text !== 'undefined' && Text.prototype) {
            _installGetter(Text.prototype, 'wholeText', function() {
                return this._data || this.data || '';
            });
        }

        // Element.regionOverset — CSSOMString enum, default "overset" or "unset"
        // Codegen returns {} (object) instead of string. K-014.
        if (typeof Element !== 'undefined' && Element.prototype) {
            _installGetter(Element.prototype, 'regionOverset', function() {
                return 'unset';
            });
        }
    })();
"#;

/// Fix missing Symbol.toStringTag on codegen interfaces.
///
/// Codegen installs toStringTag via `proto.set(tag_sym, tag_val)` on the
/// FunctionTemplate's prototype template. However, after V8 instantiates the
/// template, the property may not survive subsequent Object.defineProperty
/// calls (fix_accessor_properties redefines prototype properties).
///
/// This fix iterates all globalThis constructors and installs
/// `Symbol.toStringTag = constructorName` if missing.
///
/// **Why not codegen?** Codegen does install it, but it gets lost during
/// fix_accessor_properties. Fixing codegen requires regenerating 197K lines.
/// This JS fix is simpler and more maintainable.
pub const TO_STRING_TAG_FIX_JS: &str = r#"
    (function() {
        // Legacy aliases: key = alias name, value = canonical name
        // The alias should NOT override the canonical constructor's toStringTag.
        var aliases = {
            'webkitAudioContext': 'AudioContext',
            'Option': 'HTMLOptionElement',
            'webkitOfflineAudioContext': 'OfflineAudioContext',
        };
        var names = Object.getOwnPropertyNames(globalThis);
        var seenCtors = [];
        var canonicalTags = {};
        for (var i = 0; i < names.length; i++) {
            var name = names[i];
            try {
                var ctor = globalThis[name];
                if (!ctor || typeof ctor !== 'function' || !ctor.prototype) continue;
                // Skip legacy aliases — they share prototype with canonical
                if (aliases[name]) continue;
                var proto = ctor.prototype;
                var existingDesc = Object.getOwnPropertyDescriptor(proto, Symbol.toStringTag);
                if (!existingDesc || existingDesc.value !== name) {
                    try {
                        Object.defineProperty(proto, Symbol.toStringTag, {
                            value: name,
                            writable: false,
                            enumerable: false,
                            configurable: true
                        });
                    } catch(e) {}
                }
                // Fix proto.toString() that throws "Illegal invocation"
                if (typeof proto.toString === 'function' && proto.toString !== Object.prototype.toString) {
                    try {
                        var origToString = proto.toString;
                        try { origToString.call(proto); } catch(e) {
                            proto.toString = function toString() {
                                return '[object ' + name + ']';
                            };
                        }
                    } catch(e) {}
                }
            } catch(e) {}
        }
    })();
"#;
