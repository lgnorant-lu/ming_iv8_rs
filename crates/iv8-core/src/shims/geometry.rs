//! getBoundingClientRect + offsetWidth/offsetHeight stubs.
//!
//! Default values are zero. Fixtures can configure per-element rectangles via
//! `__iv8SetElementRect(element, {x, y, width, height})`.
//! The native Rust callback reads the stored rect from `this.__iv8Rect__`.
//! No real layout engine.

/// JS shim for geometry properties (fixture hooks + getComputedStyle).
/// getBoundingClientRect itself is a Rust native callback on the prototype.
pub const GEOMETRY_SHIM_JS: &str = r#"
(function() {
    // DOMRect constructor
    function DOMRect(x, y, width, height) {
        this.x = x || 0;
        this.y = y || 0;
        this.width = width || 0;
        this.height = height || 0;
        this.top = this.y;
        this.left = this.x;
        this.bottom = this.y + this.height;
        this.right = this.x + this.width;
    }
    DOMRect.prototype.toJSON = function() {
        return {x: this.x, y: this.y, width: this.width, height: this.height,
                top: this.top, right: this.right, bottom: this.bottom, left: this.left};
    };
    globalThis.DOMRect = DOMRect;

    // Fixture hook: store a rect directly on the element object.
    // The native getBoundingClientRect callback reads this.__iv8Rect__.
    Object.defineProperty(globalThis, '__iv8SetElementRect', {
        value: function(element, rect) {
            if (!element || typeof element !== 'object') return;
            element.__iv8Rect__ = {
                x: ('x' in rect) ? Number(rect.x) : 0,
                y: ('y' in rect) ? Number(rect.y) : 0,
                width: ('width' in rect) ? Number(rect.width) : 0,
                height: ('height' in rect) ? Number(rect.height) : 0,
            };
        },
        writable: true, configurable: true, enumerable: false,
    });

    // getComputedStyle stub — returns a CSSStyleDeclaration-like object
    // with Chrome-default computed values for common properties.
    // Falls back to element.style values when available.
    globalThis.getComputedStyle = function(element, pseudoElt) {
        var defaults = {
            display: 'block',
            visibility: 'visible',
            position: 'static',
            fontSize: '16px',
            fontFamily: 'Arial, sans-serif',
            color: 'rgb(0, 0, 0)',
            backgroundColor: 'rgba(0, 0, 0, 0)',
            width: 'auto',
            height: 'auto',
            margin: '0px',
            marginTop: '0px',
            marginRight: '0px',
            marginBottom: '0px',
            marginLeft: '0px',
            padding: '0px',
            paddingTop: '0px',
            paddingRight: '0px',
            paddingBottom: '0px',
            paddingLeft: '0px',
            border: '0px none rgb(0, 0, 0)',
            borderTopWidth: '0px',
            borderRightWidth: '0px',
            borderBottomWidth: '0px',
            borderLeftWidth: '0px',
            boxSizing: 'content-box',
            overflow: 'visible',
            opacity: '1',
            zIndex: 'auto',
            transform: 'none',
            transition: 'all 0s ease 0s',
            pointerEvents: 'auto',
            cursor: 'auto',
            lineHeight: 'normal',
            fontWeight: '400',
            fontStyle: 'normal',
            textTransform: 'none',
            textAlign: 'start',
            textDecoration: 'none solid rgb(0, 0, 0)',
            whiteSpace: 'normal',
            float: 'none',
            clear: 'none',
        };
        // Merge element.style overrides if available
        var styles = {};
        var keys = Object.keys(defaults);
        for (var i = 0; i < keys.length; i++) {
            var k = keys[i];
            if (element && element.style && element.style[k] !== undefined && element.style[k] !== '') {
                styles[k] = element.style[k];
            } else {
                styles[k] = defaults[k];
            }
        }
        styles.getPropertyValue = function(prop) {
            var kebab = prop.replace(/([A-Z])/g, '-$1').toLowerCase();
            return this[prop] || this[kebab] || '';
        };
        styles.length = keys.length;
        return styles;
    };

    // matchMedia — returns MediaQueryList-like object
    // Reads from globalThis.__iv8MediaPrefs (injected from profile env map).
    // Falls back to Chrome desktop defaults if not set.
    globalThis.matchMedia = function matchMedia(query) {
        query = String(query || '');
        var q = query.toLowerCase().replace(/\s+/g, '');

        var prefs = (typeof globalThis.__iv8MediaPrefs === 'object' && globalThis.__iv8MediaPrefs) ? globalThis.__iv8MediaPrefs : {};
        function prefVal(name, fallback) {
            return prefs[name] || fallback;
        }

        // Helper: check if a media feature query matches the configured value.
        function mediaMatches(feature, value) {
            var configured = prefVal(feature, '');
            if (!configured) return false;
            // Handle scripting specially: Chrome uses "enabled"/"none",
            // CSS spec uses "initial"/"none".
            if (feature === 'scripting' && value === 'enabled') {
                return configured === 'enabled' || configured === 'yes' || configured === 'initial';
            }
            return configured === value;
        }

        var matches = false;
        // Parse query for feature:value patterns
        if (q.indexOf('prefers-color-scheme:') !== -1) {
            matches = mediaMatches('prefers-color-scheme',
                q.indexOf('prefers-color-scheme:light') !== -1 ? 'light' :
                q.indexOf('prefers-color-scheme:dark') !== -1 ? 'dark' : '');
        } else if (q.indexOf('prefers-reduced-motion:') !== -1) {
            matches = mediaMatches('prefers-reduced-motion',
                q.indexOf('prefers-reduced-motion:reduce') !== -1 ? 'reduce' : 'no-preference');
        } else if (q.indexOf('prefers-contrast:') !== -1) {
            matches = mediaMatches('prefers-contrast',
                q.indexOf('prefers-contrast:more') !== -1 ? 'more' :
                q.indexOf('prefers-contrast:less') !== -1 ? 'less' : 'no-preference');
        } else if (q.indexOf('prefers-reduced-data:') !== -1) {
            matches = mediaMatches('prefers-reduced-data',
                q.indexOf('prefers-reduced-data:reduce') !== -1 ? 'reduce' : 'no-preference');
        } else if (q.indexOf('prefers-reduced-transparency:') !== -1) {
            matches = mediaMatches('prefers-reduced-transparency',
                q.indexOf('prefers-reduced-transparency:reduce') !== -1 ? 'reduce' : 'no-preference');
        } else if (q.indexOf('forced-colors:') !== -1) {
            matches = mediaMatches('forced-colors',
                q.indexOf('forced-colors:none') !== -1 ? 'none' : 'active');
        } else if (q.indexOf('color-gamut:') !== -1) {
            matches = mediaMatches('color-gamut',
                q.indexOf('color-gamut:p3') !== -1 ? 'p3' :
                q.indexOf('color-gamut:rec2020') !== -1 ? 'rec2020' : 'srgb');
        } else if (q.indexOf('dynamic-range:') !== -1) {
            matches = mediaMatches('dynamic-range',
                q.indexOf('dynamic-range:high') !== -1 ? 'high' : 'standard');
        } else if (q.indexOf('scripting:') !== -1) {
            matches = mediaMatches('scripting',
                q.indexOf('scripting:enabled') !== -1 ? 'enabled' :
                q.indexOf('scripting:none') !== -1 ? 'none' : 'initial');
        } else if (q.indexOf('update:') !== -1) {
            matches = mediaMatches('update',
                q.indexOf('update:slow') !== -1 ? 'slow' :
                q.indexOf('update:none') !== -1 ? 'none' : 'fast');
        } else if (q.indexOf('any-pointer:') !== -1) {
            matches = mediaMatches('any-pointer',
                q.indexOf('any-pointer:fine') !== -1 ? 'fine' : 'coarse');
        } else if (q.indexOf('any-hover:') !== -1) {
            matches = mediaMatches('any-hover',
                q.indexOf('any-hover:hover') !== -1 ? 'hover' : 'none');
        } else if (q.indexOf('pointer:') !== -1) {
            matches = mediaMatches('pointer',
                q.indexOf('pointer:fine') !== -1 ? 'fine' : 'coarse');
        } else if (q.indexOf('hover:') !== -1) {
            matches = mediaMatches('hover',
                q.indexOf('hover:hover') !== -1 ? 'hover' : 'none');
        } else if (q.indexOf('display-mode:') !== -1) {
            matches = mediaMatches('display-mode',
                q.indexOf('display-mode:fullscreen') !== -1 ? 'fullscreen' :
                q.indexOf('display-mode:standalone') !== -1 ? 'standalone' :
                q.indexOf('display-mode:minimal-ui') !== -1 ? 'minimal-ui' : 'browser');
        } else if (q.indexOf('inverted-colors:') !== -1) {
            matches = mediaMatches('inverted-colors',
                q.indexOf('inverted-colors:inverted') !== -1 ? 'inverted' : 'none');
        } else if (q.indexOf('min-width:') !== -1 || q.indexOf('max-width:') !== -1) {
            // Screen-based queries — use window.innerWidth
            var w = (typeof window !== 'undefined' && window.innerWidth) ? window.innerWidth : 1920;
            var minMatch = q.match(/min-width:\s*(\d+)px/);
            var maxMatch = q.match(/max-width:\s*(\d+)px/);
            matches = true;
            if (minMatch && w < parseInt(minMatch[1])) matches = false;
            if (maxMatch && w > parseInt(maxMatch[1])) matches = false;
        } else if (q.indexOf('min-height:') !== -1 || q.indexOf('max-height:') !== -1) {
            var h = (typeof window !== 'undefined' && window.innerHeight) ? window.innerHeight : 969;
            var minH = q.match(/min-height:\s*(\d+)px/);
            var maxH = q.match(/max-height:\s*(\d+)px/);
            matches = true;
            if (minH && h < parseInt(minH[1])) matches = false;
            if (maxH && h > parseInt(maxH[1])) matches = false;
        } else if (q.indexOf('orientation:landscape') !== -1) {
            var sw = (typeof screen !== 'undefined' && screen.width) ? screen.width : 1920;
            var sh = (typeof screen !== 'undefined' && screen.height) ? screen.height : 1080;
            matches = sw >= sh;
        } else if (q.indexOf('orientation:portrait') !== -1) {
            var sw2 = (typeof screen !== 'undefined' && screen.width) ? screen.width : 1920;
            var sh2 = (typeof screen !== 'undefined' && screen.height) ? screen.height : 1080;
            matches = sw2 < sh2;
        }

        var mql = {
            matches: matches,
            media: query,
            onchange: null,
        };
        mql.addEventListener = function(type, listener) {};
        mql.removeEventListener = function(type, listener) {};
        mql.addListener = function(listener) {};
        mql.removeListener = function(listener) {};
        mql.dispatchEvent = function(event) { return true; };
        return mql;
    };
})();
"#;