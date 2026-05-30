//! navigator.mimeTypes, navigator.plugins, navigator.connection stubs.

/// JS shim for navigator extras.
pub const NAVIGATOR_EXTRAS_JS: &str = r#"
(function() {
    // navigator.mimeTypes (empty PluginArray-like)
    if (typeof navigator.mimeTypes === 'undefined') {
        Object.defineProperty(navigator, 'mimeTypes', {
            value: Object.assign([], {length: 0, item: function() { return null; }, namedItem: function() { return null; }}),
            writable: true,
            enumerable: true,
            configurable: true,
        });
    }

    // navigator.plugins (empty PluginArray-like)
    if (typeof navigator.plugins === 'undefined') {
        Object.defineProperty(navigator, 'plugins', {
            value: Object.assign([], {length: 0, item: function() { return null; }, namedItem: function() { return null; }, refresh: function() {}}),
            writable: true,
            enumerable: true,
            configurable: true,
        });
    }

    // navigator.connection stub
    if (typeof navigator.connection === 'undefined') {
        Object.defineProperty(navigator, 'connection', {
            value: {
                effectiveType: '4g',
                downlink: 10,
                rtt: 50,
                saveData: false,
                type: 'wifi',
                addEventListener: function() {},
                removeEventListener: function() {},
            },
            writable: true,
            enumerable: true,
            configurable: true,
        });
    }

    // window.history stub
    if (typeof history === 'undefined') {
        globalThis.history = {
            length: 1,
            state: null,
            pushState: function() {},
            replaceState: function() {},
            go: function() {},
            back: function() {},
            forward: function() {},
        };
    }
})();
"#;
