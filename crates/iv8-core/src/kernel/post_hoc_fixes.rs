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
        Object.defineProperty(RequestShim.prototype, 'constructor', {value: RequestShim, writable: true, configurable: true, enumerable: false});
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
            'HTMLIFrameElement': ['sandbox'],
            'Document': ['implementation','timeline','fonts'],
            'HTMLElement': ['style'],
            'HTMLLinkElement': ['relList','sizes','blocking'],
            'HTMLAnchorElement': ['relList'],
            'HTMLFormElement': ['relList'],
            'HTMLScriptElement': ['blocking'],
        };
        for (var iface in readonlyAttrs) {
            var ctor = globalThis[iface];
            if (!ctor || !ctor.prototype) continue;
            var attrs = readonlyAttrs[iface];
            for (var i = 0; i < attrs.length; i++) {
                var desc = Object.getOwnPropertyDescriptor(ctor.prototype, attrs[i]);
                if (desc && desc.get) {
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

        // AnimationEvent.animationName/pseudoElement — codegen returns object
        // instead of string. Default empty string per spec.
        if (typeof AnimationEvent !== 'undefined' && AnimationEvent.prototype) {
            _installGetter(AnimationEvent.prototype, 'animationName', function() {
                return this.__iv8AnimationName || '';
            });
            _installGetter(AnimationEvent.prototype, 'pseudoElement', function() {
                return this.__iv8PseudoElement || '';
            });
        }

        // TransitionEvent.propertyName/pseudoElement — codegen returns object
        if (typeof TransitionEvent !== 'undefined' && TransitionEvent.prototype) {
            _installGetter(TransitionEvent.prototype, 'propertyName', function() {
                return this.__iv8PropertyName || '';
            });
            _installGetter(TransitionEvent.prototype, 'pseudoElement', function() {
                return this.__iv8PseudoElement || '';
            });
        }

        // HTMLDListElement.compact — codegen returns undefined instead of boolean
        if (typeof HTMLDListElement !== 'undefined' && HTMLDListElement.prototype) {
            _installGetter(HTMLDListElement.prototype, 'compact', function() {
                return this.__iv8Compact || false;
            });
        }

        // BeforeUnloadEvent.returnValue — codegen throws Illegal invocation
        if (typeof BeforeUnloadEvent !== 'undefined' && BeforeUnloadEvent.prototype) {
            _installGetter(BeforeUnloadEvent.prototype, 'returnValue', function() {
                return this.__iv8ReturnValue !== undefined ? this.__iv8ReturnValue : true;
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

/// Comprehensive [Global] attribute accessor fix.
///
/// Wraps all [Global] attributes on globalThis with receiver-checked
/// JS accessors. Uses GLOBAL_ATTR_METADATA from codegen to determine
/// which attrs are readonly/Replaceable.
///
/// **Why not codegen?** Codegen's `fix_global_accessor_properties` installs
/// native FunctionTemplate getters, but V8 native getters throw "Illegal
/// invocation" when called via `.call(globalThis)` (K-008). This JS wrapper
/// calls `origGet.call(globalThis)` to bypass the native receiver check.
///
/// **Phase 2 target (v0.8.91)**: Eliminate by making codegen native getters
/// include receiver check internally, or by using `Object.defineProperty`
/// with JS wrappers that have `[native code]` toString.
pub fn global_accessor_fix_js(attr_meta: &[(&str, bool, bool)]) -> String {
    let meta_js = attr_meta
        .iter()
        .map(|(n, ro, rep)| format!("[\"{}\",{},{}]", n, ro, rep))
        .collect::<Vec<_>>()
        .join(",");
    format!(r#"
    (function() {{
        var meta = [{meta}];
        var windowCtor = globalThis.Window;
        var windowProto = windowCtor && windowCtor.prototype;
        for (var i = 0; i < meta.length; i++) {{
            (function(name, isReadonly, hasReplaceable) {{
                var needsSetter = !isReadonly || hasReplaceable;
                try {{
                    var desc = Object.getOwnPropertyDescriptor(globalThis, name);
                    if (!desc) {{
                        var getter = function() {{ return undefined; }};
                        try {{ Object.defineProperty(getter, 'name', {{ value: 'get ' + name }}); }} catch(e) {{}}
                        try {{ Object.defineProperty(getter, 'length', {{ value: 0, writable: false, enumerable: false, configurable: true }}); }} catch(e) {{}}
                        var setter = needsSetter ? (function(nm, wp) {{
                            return function(v) {{
                                if (wp && this !== globalThis && this !== wp) {{
                                    var cur = Object.getPrototypeOf(this); var found = false;
                                    for (var k = 0; k < 30; k++) {{ if (cur === wp) {{ found = true; break; }} if (!cur) break; cur = Object.getPrototypeOf(cur); }}
                                    if (!found) throw new TypeError('Illegal invocation');
                                }}
                                Object.defineProperty(globalThis, nm, {{ value: v, writable: true, enumerable: true, configurable: true }});
                            }};
                        }})(name, windowProto) : undefined;
                        if (setter) {{
                            try {{ Object.defineProperty(setter, 'name', {{ value: 'set ' + name }}); }} catch(e) {{}}
                            try {{ Object.defineProperty(setter, 'length', {{ value: 1, writable: false, enumerable: false, configurable: true }}); }} catch(e) {{}}
                        }}
                        Object.defineProperty(globalThis, name, {{ get: getter, set: setter, enumerable: true, configurable: true }});
                        return;
                    }}
                    if (desc.get || desc.set) {{
                        // Phase 2 (v0.8.91): Preserve native [Global] getters.
                        // Codegen installs native FunctionTemplate getters that already
                        // have receiver check in the callback (web_apis.rs window_get_N).
                        // Replacing them with JS wrappers loses [native code] toString,
                        // which is detectable. Only wrap non-native (shim-installed) getters.
                        if (desc.get && typeof desc.get === 'function') {{
                            var fnStr = '';
                            try {{ fnStr = desc.get.toString(); }} catch(e) {{}}
                            if (fnStr.indexOf('[native code]') !== -1) {{
                                // Native getter — preserve as-is, only fix setter if needed
                                if (needsSetter && (!desc.set || typeof desc.set !== 'function')) {{
                                    var newSetter = (function(nm, wp) {{
                                        return function(v) {{
                                            if (wp && this !== globalThis && this !== wp) throw new TypeError('Illegal invocation');
                                            Object.defineProperty(globalThis, nm, {{ value: v, writable: true, enumerable: true, configurable: true }});
                                        }};
                                    }})(name, windowProto);
                                    try {{ Object.defineProperty(newSetter, 'name', {{ value: 'set ' + name }}); }} catch(e) {{}}
                                    Object.defineProperty(globalThis, name, {{ get: desc.get, set: newSetter, enumerable: desc.enumerable !== false, configurable: true }});
                                }}
                                return;
                            }}
                            // JS getter (shim-installed) — wrap with receiver check
                            var origGet = desc.get;
                            var wrappedGet = function() {{
                                if (windowProto && this !== globalThis && this !== windowProto) {{
                                    var cur = Object.getPrototypeOf(this); var found = false;
                                    for (var k = 0; k < 30; k++) {{ if (cur === windowProto) {{ found = true; break; }} if (!cur) break; cur = Object.getPrototypeOf(cur); }}
                                    if (!found) throw new TypeError('Illegal invocation');
                                }}
                                if (name === 'self' || name === 'window' || name === 'top' || name === 'parent' || name === 'frames') return globalThis;
                                return origGet.call(globalThis);
                            }};
                            try {{ Object.defineProperty(wrappedGet, 'name', {{ value: 'get ' + name }}); }} catch(e) {{}}
                            wrappedGet.__iv8_native = true;
                            var newSetter;
                            if (needsSetter) {{
                                if (desc.set && typeof desc.set === 'function') {{ newSetter = desc.set; }}
                                else {{
                                    newSetter = (function(nm, wp) {{
                                        return function(v) {{
                                            if (wp && this !== globalThis && this !== wp) throw new TypeError('Illegal invocation');
                                            Object.defineProperty(globalThis, nm, {{ value: v, writable: true, enumerable: true, configurable: true }});
                                        }};
                                    }})(name, windowProto);
                                    try {{ Object.defineProperty(newSetter, 'name', {{ value: 'set ' + name }}); }} catch(e) {{}}
                                }}
                            }}
                            Object.defineProperty(globalThis, name, {{ get: wrappedGet, set: newSetter, enumerable: desc.enumerable !== false, configurable: true }});
                        }}
                        return;
                    }}
                    if (!desc.configurable) return;
                    var value = desc.value;
                    var getter = (function(v, wp) {{
                        return function() {{
                            if (wp && this !== globalThis && this !== wp) {{
                                var cur = Object.getPrototypeOf(this); var found = false;
                                for (var k = 0; k < 30; k++) {{ if (cur === wp) {{ found = true; break; }} if (!cur) break; cur = Object.getPrototypeOf(cur); }}
                                if (!found) throw new TypeError('Illegal invocation');
                            }}
                            return v;
                        }};
                    }})(value, windowProto);
                    try {{ Object.defineProperty(getter, 'name', {{ value: 'get ' + name }}); }} catch(e) {{}}
                    try {{ Object.defineProperty(getter, 'length', {{ value: 0, writable: false, enumerable: false, configurable: true }}); }} catch(e) {{}}
                    var setter = needsSetter ? (function(nm, wp) {{
                        return function(v) {{
                            if (wp && this !== globalThis && this !== wp) {{
                                var cur = Object.getPrototypeOf(this); var found = false;
                                for (var k = 0; k < 30; k++) {{ if (cur === wp) {{ found = true; break; }} if (!cur) break; cur = Object.getPrototypeOf(cur); }}
                                if (!found) throw new TypeError('Illegal invocation');
                            }}
                            Object.defineProperty(globalThis, nm, {{ value: v, writable: true, enumerable: true, configurable: true }});
                        }};
                    }})(name, windowProto) : undefined;
                    if (setter) {{
                        try {{ Object.defineProperty(setter, 'name', {{ value: 'set ' + name }}); }} catch(e) {{}}
                        try {{ Object.defineProperty(setter, 'length', {{ value: 1, writable: false, enumerable: false, configurable: true }}); }} catch(e) {{}}
                    }}
                    Object.defineProperty(globalThis, name, {{ get: getter, set: setter, enumerable: desc.enumerable !== false, configurable: true }});
                }} catch(e) {{}}
            }})(meta[i][0], meta[i][1], meta[i][2]);
        }}
    }})();
"#, meta = meta_js)
}

/// Prototype chain + constructor + receiver check fix.
///
/// This is the largest post-hoc fix (~360 lines of JS). It handles:
/// 1. Prototype chain: setPrototypeOf for 60+ interfaces (child→parent)
/// 2. Constructor pointer: prototype.constructor = ctor for 20 interfaces
/// 3. Constructor __proto__: setPrototypeOf(ctor, Function.prototype) for 4 interfaces
/// 4. Operation receiver check: wrap shim JS methods with TypeError on wrong receiver
/// 5. Getter/setter receiver check: wrap shim JS accessors with TypeError on wrong receiver
///
/// **Why not codegen?** Codegen's `fix_accessor_properties` installs native
/// FunctionTemplate getters that already have R3 receiver check. But shim-
/// installed JS functions (event_constructors.rs, document_props.rs, etc.)
/// don't have receiver check. This fix wraps them.
///
/// **Phase 2 target**: Migrate receiver check wrapping to codegen layer
/// (add receiver check to shim installation code). Phase 4 (v0.8.93) will
/// decompose this into per-concern fixes.
pub const FIX_PROTO_JS: &str = r#"
                (function() {
                    var shimEvent = globalThis.Event;
                    var shimMouseEvent = globalThis.MouseEvent;
                    var fixes = [
                        ['TrackEvent','Event'], ['SubmitEvent','Event'], ['FormDataEvent','Event'],
                        ['ToggleEvent','Event'], ['CommandEvent','Event'],
                        ['DragEvent','MouseEvent'],
                        ['AudioTrackList','EventTarget'], ['VideoTrackList','EventTarget'],
                        ['TextTrackList','EventTarget'], ['TextTrack','EventTarget'],
                        ['TextTrackCue','EventTarget'], ['OffscreenCanvas','EventTarget'],
                        ['CloseWatcher','EventTarget'], ['Navigation','EventTarget'],
                        ['NavigationHistoryEntry','EventTarget'],
                        ['NavigateEvent','Event'], ['NavigationCurrentEntryChangeEvent','Event'],
                        ['PopStateEvent','Event'], ['HashChangeEvent','Event'],
                        ['PageSwapEvent','Event'], ['PageRevealEvent','Event'],
                        ['PageTransitionEvent','Event'], ['BeforeUnloadEvent','Event'],
                        ['ErrorEvent','Event'], ['PromiseRejectionEvent','Event'],
                        ['MessageEvent','Event'], ['StorageEvent','Event'],
                        ['EventSource','EventTarget'], ['MessagePort','EventTarget'],
                        ['BroadcastChannel','EventTarget'], ['Worker','EventTarget'],
                        ['SharedWorker','EventTarget'], ['Storage','EventTarget'],
                        ['RadioNodeList','NodeList'],
                        ['CustomEvent','Event'],
                        ['AbortSignal','EventTarget'],
                        ['XMLDocument','Document'],
                        ['DocumentType','Node'],
                        ['DocumentFragment','Node'],
                        ['Attr','Node'],
                        ['Navigator','EventTarget'],
                        ['EventTarget','Object'],
                        ['MediaQueryList','EventTarget'],
                        ['MediaQueryListEvent','Event'],
                        ['CharacterData','Node'],
                        ['Text','CharacterData'],
                        ['CDATASection','Text'],
                        ['Comment','CharacterData'],
                        ['ProcessingInstruction','CharacterData'],
                        ['Node','EventTarget'],
                        ['Element','Node'],
                        ['HTMLElement','Element'],
                        ['Screen','Object'],
                        ['VisualViewport','EventTarget'],
                        ['Location','Object'],
                        ['IDBRequest','EventTarget'], ['IDBDatabase','EventTarget'],
                        ['IDBTransaction','EventTarget'], ['IDBVersionChangeEvent','Event'],
                        ['IDBOpenDBRequest','IDBRequest'],
                        ['Performance','EventTarget'],
                        ['ScreenOrientation','EventTarget'],
                        ['PerformanceEntry','Object'],
                        ['PerformanceResourceTiming','PerformanceEntry'],
                        ['PerformanceNavigationTiming','PerformanceResourceTiming'],
                        ['PerformanceObserver','EventTarget'],
                        ['XMLHttpRequestEventTarget','EventTarget'],
                        ['XMLHttpRequest','XMLHttpRequestEventTarget'],
                        ['XMLHttpRequestUpload','XMLHttpRequestEventTarget'],
                        ['WebSocket','EventTarget'],
                        ['Animation','EventTarget'],
                        ['FileReader','EventTarget'],
                    ];
                    for (var i = 0; i < fixes.length; i++) {
                        var child = fixes[i][0], parent = fixes[i][1];
                        try {
                            var childCtor = globalThis[child];
                            var parentCtor = globalThis[parent];
                            if (childCtor && parentCtor) {
                                Object.setPrototypeOf(childCtor, parentCtor);
                                var childProto = childCtor.prototype;
                                var parentProto = parentCtor.prototype;
                                if (childProto && parentProto) {
                                    Object.setPrototypeOf(childProto, parentProto);
                                }
                            }
                        } catch(e) {}
                    }
                    var ctorFixes = [
                        'Location', 'Navigator', 'BroadcastChannel', 'MessagePort',
                        'Worker', 'SharedWorker', 'Storage', 'Screen',
                        'EventTarget', 'Node', 'Document', 'Element', 'HTMLElement',
                        'CharacterData', 'Text', 'Comment', 'Event', 'CustomEvent',
                        'MouseEvent', 'VisualViewport', 'MediaQueryList',
                    ];
                    for (var i = 0; i < ctorFixes.length; i++) {
                        try {
                            var ctor = globalThis[ctorFixes[i]];
                            if (ctor && ctor.prototype) {
                                Object.defineProperty(ctor.prototype, 'constructor', {
                                    value: ctor, writable: true, configurable: true, enumerable: false
                                });
                            }
                        } catch(e) {}
                    }
                    var functionProto = Function.prototype;
                    var protoFixes = [
                        'Location', 'Navigator', 'Storage', 'Screen',
                    ];
                    for (var i = 0; i < protoFixes.length; i++) {
                        try {
                            var ctor = globalThis[protoFixes[i]];
                            if (ctor && typeof ctor === 'function') {
                                Object.setPrototypeOf(ctor, functionProto);
                            }
                        } catch(e) {}
                    }
                    var etCtor = globalThis.EventTarget;
                    var etInheritors = [
                        'MessagePort', 'BroadcastChannel', 'Worker', 'SharedWorker',
                        'EventSource', 'AbortSignal', 'Navigation',
                    ];
                    for (var i = 0; i < etInheritors.length; i++) {
                        try {
                            var ctor = globalThis[etInheritors[i]];
                            if (ctor && typeof ctor === 'function' && etCtor) {
                                Object.setPrototypeOf(ctor, etCtor);
                            }
                        } catch(e) {}
                    }
                    try {
                        var storageCtor = globalThis.Storage;
                        if (storageCtor && storageCtor.prototype) {
                            Object.setPrototypeOf(storageCtor.prototype, Object.prototype);
                        }
                    } catch(e) {}
                    var shimOpInterfaces = [
                        'Event', 'CustomEvent', 'MouseEvent',
                        'MessagePort', 'BroadcastChannel', 'Worker', 'SharedWorker',
                        'Storage', 'Navigator',
                        'NodeList', 'MutationObserver', 'DOMTokenList',
                    ];
                    for (let i = 0; i < shimOpInterfaces.length; i++) {
                        try {
                            var ctor = globalThis[shimOpInterfaces[i]];
                            if (!ctor || !ctor.prototype) continue;
                            var proto = ctor.prototype;
                            var names = Object.getOwnPropertyNames(proto);
                            for (let j = 0; j < names.length; j++) {
                                let pname = names[j];
                                if (pname === 'constructor') continue;
                                try {
                                    var desc = Object.getOwnPropertyDescriptor(proto, pname);
                                    if (!desc || typeof desc.value !== 'function') continue;
                                    if (desc.value.__iv8_op_wrapped) continue;
                                    var fnStr = '';
                                    try { fnStr = desc.value.toString(); } catch(e) { continue; }
                                    var isNative = fnStr.indexOf('[native code]') !== -1;
                                    var alreadyThrows = false;
                                    if (isNative) {
                                        try {
                                            desc.value.call({});
                                        } catch(e) {
                                            alreadyThrows = true;
                                        }
                                    }
                                    if (alreadyThrows) continue;
                                    let origFn = desc.value;
                                    let expectedTag = shimOpInterfaces[i];
                                    let origName = origFn.name || pname;
                                    let origLen = origFn.length;
                                    let wrappedFn = function() {
                                        var thisTag = '';
                                        try { thisTag = this[Symbol.toStringTag]; } catch(e) {}
                                        if (thisTag !== expectedTag && this !== globalThis[shimOpInterfaces[i]].prototype) {
                                            var isValid = false;
                                            try {
                                                var cur = Object.getPrototypeOf(this);
                                                var expectedProto = globalThis[expectedTag].prototype;
                                                for (var k = 0; k < 30; k++) {
                                                    if (cur === expectedProto) { isValid = true; break; }
                                                    if (!cur) break;
                                                    cur = Object.getPrototypeOf(cur);
                                                }
                                            } catch(e) {}
                                            if (!isValid) {
                                                throw new TypeError('Illegal invocation');
                                            }
                                        }
                                        return origFn.apply(this, arguments);
                                    };
                                    wrappedFn.__iv8_op_wrapped = true;
                                    try { Object.defineProperty(wrappedFn, 'name', { value: origName }); } catch(e) {}
                                    try { Object.defineProperty(wrappedFn, 'length', { value: origLen }); } catch(e) {}
                                    Object.defineProperty(proto, pname, {
                                        value: wrappedFn,
                                        writable: desc.writable,
                                        enumerable: desc.enumerable,
                                        configurable: true
                                    });
                                } catch(e) {}
                            }
                        } catch(e) {}
                    }
                    var receiverCheckInterfaces = [
                        'Document', 'CustomEvent', 'MouseEvent',
                        'HTMLElement', 'Element', 'Node', 'Window',
                        'NavigationTransition', 'ShadowRoot',
                    ];
                    for (let i = 0; i < receiverCheckInterfaces.length; i++) {
                        let ifaceName = receiverCheckInterfaces[i];
                        try {
                            var ctor = globalThis[ifaceName];
                            if (!ctor || !ctor.prototype) continue;
                            var proto = ctor.prototype;
                            var names = Object.getOwnPropertyNames(proto);
                            for (let j = 0; j < names.length; j++) {
                                let pname = names[j];
                                if (pname === 'constructor') continue;
                                if (pname === 'attributes') continue;
                                if (pname.startsWith('on')) continue;
                                try {
                                    var desc = Object.getOwnPropertyDescriptor(proto, pname);
                                    if (!desc || !desc.get) continue;
                                    let origGet = desc.get;
                                    let origSet = desc.set;
                                    var alreadyWrapped = desc.get && desc.get.__iv8_wrapped;
                                    if (alreadyWrapped && (!desc.set || desc.set.__iv8_set_wrapped)) continue;
                                    if (origGet.toString().indexOf('[native code]') !== -1) continue;
                                    let thisIfaceName = ifaceName;
                                    var wrappedGet;
                                    if (alreadyWrapped) {
                                        wrappedGet = origGet;
                                    } else {
                                        wrappedGet = function() {
                                            var thisCtor = globalThis[thisIfaceName];
                                            if (thisCtor && thisCtor.prototype) {
                                                if (this === thisCtor.prototype) {
                                                    throw new TypeError('Illegal invocation');
                                                }
                                                var isValid = false;
                                                var cur = Object.getPrototypeOf(this);
                                                for (var k = 0; k < 30; k++) {
                                                    if (cur === thisCtor.prototype) { isValid = true; break; }
                                                    if (!cur) break;
                                                    cur = Object.getPrototypeOf(cur);
                                                }
                                                if (!isValid) {
                                                    throw new TypeError('Illegal invocation');
                                                }
                                            }
                                            if (pname.indexOf('on') === 0 && pname.length > 2) {
                                                var hv = this['__iv8_' + pname];
                                                if (hv !== undefined) return hv;
                                                return null;
                                            }
                                            return origGet.call(this);
                                        };
                                        wrappedGet.__iv8_wrapped = true;
                                        try { Object.defineProperty(wrappedGet, 'name', { value: 'get ' + pname }); } catch(e) {}
                                    }
                                    var wrappedSet = origSet;
                                    if (typeof origSet === 'function' && origSet.toString().indexOf('[native code]') === -1) {
                                        if (pname.indexOf('on') === 0 && pname.length > 2) {
                                            wrappedSet = function(v) {
                                                var thisCtor2 = globalThis[thisIfaceName];
                                                if (thisCtor2 && thisCtor2.prototype) {
                                                    if (this === thisCtor2.prototype) {
                                                        throw new TypeError('Illegal invocation');
                                                    }
                                                    var isValid2 = false;
                                                    var cur2 = Object.getPrototypeOf(this);
                                                    for (var k2 = 0; k2 < 30; k2++) {
                                                        if (cur2 === thisCtor2.prototype) { isValid2 = true; break; }
                                                        if (!cur2) break;
                                                        cur2 = Object.getPrototypeOf(cur2);
                                                    }
                                                    if (!isValid2) {
                                                        throw new TypeError('Illegal invocation');
                                                    }
                                                }
                                                Object.defineProperty(this, '__iv8_' + pname, { value: v, writable: true, enumerable: false, configurable: true });
                                            };
                                        } else {
                                            wrappedSet = function(v) {
                                                var thisCtor2 = globalThis[thisIfaceName];
                                                if (thisCtor2 && thisCtor2.prototype) {
                                                    if (this === thisCtor2.prototype) {
                                                        throw new TypeError('Illegal invocation');
                                                    }
                                                    var isValid2 = false;
                                                    var cur2 = Object.getPrototypeOf(this);
                                                    for (var k2 = 0; k2 < 30; k2++) {
                                                        if (cur2 === thisCtor2.prototype) { isValid2 = true; break; }
                                                        if (!cur2) break;
                                                        cur2 = Object.getPrototypeOf(cur2);
                                                    }
                                                    if (!isValid2) {
                                                        throw new TypeError('Illegal invocation');
                                                    }
                                                }
                                                Object.defineProperty(this, pname, { value: v, writable: true, enumerable: true, configurable: true });
                                            };
                                        }
                                        try { Object.defineProperty(wrappedSet, 'name', { value: 'set ' + pname }); } catch(e) {}
                                        wrappedSet.__iv8_set_wrapped = true;
                                    }
                                    Object.defineProperty(proto, pname, {
                                        get: wrappedGet,
                                        set: wrappedSet,
                                        enumerable: desc.enumerable,
                                        configurable: true
                                    });
                                } catch(e) {}
                            }
                        } catch(e) {}
                    }
                })();
"#;

/// Function.prototype.toString camouflage.
///
/// Chrome returns `function name() { [native code] }` for all built-in
/// functions. IV8's JS wrapper functions (global_accessor_fix, fix_proto_js,
/// shim getters) return JS source code, which is detectable.
///
/// This fix overrides Function.prototype.toString to return
/// `function <name>() { [native code] }` for functions that are either:
/// 1. Already returning [native code] (V8 built-in / codegen FunctionTemplate)
/// 2. Marked with __iv8_native flag (shim wrappers)
///
/// Functions without __iv8_native flag and without [native code] in their
/// original toString are left unchanged (user JS functions).
///
/// This is the A1 codegen API: instead of creating a separate replace_getter
/// mechanism, we make ALL wrapper functions appear native via toString.
pub const FUNCTION_TO_STRING_FIX_JS: &str = r#"
    (function() {
        // Mark all accessor getters/setters on key prototypes as __iv8_native
        var ifaces = Object.getOwnPropertyNames(globalThis);
        for (var i = 0; i < ifaces.length; i++) {
            try {
                var ctor = globalThis[ifaces[i]];
                if (ctor && typeof ctor === 'function' && ctor.prototype) {
                    var proto = ctor.prototype;
                    var pnames = Object.getOwnPropertyNames(proto);
                    for (var j = 0; j < pnames.length; j++) {
                        try {
                            var desc = Object.getOwnPropertyDescriptor(proto, pnames[j]);
                            if (desc) {
                                if (desc.get && typeof desc.get === 'function') {
                                    try { desc.get.__iv8_native = true; } catch(e) {}
                                }
                                if (desc.set && typeof desc.set === 'function') {
                                    try { desc.set.__iv8_native = true; } catch(e) {}
                                }
                                if (typeof desc.value === 'function') {
                                    try { desc.value.__iv8_native = true; } catch(e) {}
                                }
                            }
                        } catch(e) {}
                    }
                }
            } catch(e) {}
        }
        // Mark globalThis accessor getters/setters
        var gnames = Object.getOwnPropertyNames(globalThis);
        for (var i = 0; i < gnames.length; i++) {
            try {
                var desc = Object.getOwnPropertyDescriptor(globalThis, gnames[i]);
                if (desc) {
                    if (desc.get && typeof desc.get === 'function') {
                        try { desc.get.__iv8_native = true; } catch(e) {}
                    }
                    if (desc.set && typeof desc.set === 'function') {
                        try { desc.set.__iv8_native = true; } catch(e) {}
                    }
                }
            } catch(e) {}
        }
        // Override Function.prototype.toString
        var origToString = Function.prototype.toString;
        var nativePattern = /\[native code\]/;
        Function.prototype.toString = function toString() {
            var s = '';
            try { s = origToString.call(this); } catch(e) { return 'function () { [native code] }'; }
            if (nativePattern.test(s)) return s;
            if (this.__iv8_native) {
                var name = '';
                try { name = this.name || ''; } catch(e) {}
                if (name) return 'function ' + name + '() { [native code] }';
                return 'function () { [native code] }';
            }
            return s;
        };
        try { Object.defineProperty(Function.prototype.toString, 'name', { value: 'toString' }); } catch(e) {}
        try { Function.prototype.toString.__iv8_native = true; } catch(e) {}
    })();
"#;
