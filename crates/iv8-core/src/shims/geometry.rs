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
    globalThis.__iv8SetElementRect = function(element, rect) {
        if (!element || typeof element !== 'object') return;
        element.__iv8Rect__ = {
            x: ('x' in rect) ? Number(rect.x) : 0,
            y: ('y' in rect) ? Number(rect.y) : 0,
            width: ('width' in rect) ? Number(rect.width) : 0,
            height: ('height' in rect) ? Number(rect.height) : 0,
        };
    };

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
    // Chrome desktop defaults: light scheme, fine pointer, hover, sRGB, etc.
    globalThis.matchMedia = function(query) {
        query = String(query || '');
        var q = query.toLowerCase().replace(/\s+/g, '');

        // Chrome desktop defaults
        var matches = false;
        if (q.indexOf('prefers-color-scheme:light') !== -1) matches = true;
        else if (q.indexOf('prefers-color-scheme:dark') !== -1) matches = false;
        else if (q.indexOf('prefers-reduced-motion:no-preference') !== -1) matches = true;
        else if (q.indexOf('prefers-reduced-motion:reduce') !== -1) matches = false;
        else if (q.indexOf('prefers-contrast:no-preference') !== -1) matches = true;
        else if (q.indexOf('prefers-reduced-data:no-preference') !== -1) matches = true;
        else if (q.indexOf('forced-colors:none') !== -1) matches = true;
        else if (q.indexOf('color-gamut:srgb') !== -1) matches = true;
        else if (q.indexOf('color-gamut:p3') !== -1) matches = false;
        else if (q.indexOf('scripting:enabled') !== -1) matches = true;
        else if (q.indexOf('update:fast') !== -1) matches = true;
        else if (q.indexOf('pointer:fine') !== -1) matches = true;
        else if (q.indexOf('pointer:coarse') !== -1) matches = false;
        else if (q.indexOf('hover:hover') !== -1) matches = true;
        else if (q.indexOf('hover:none') !== -1) matches = false;
        else if (q.indexOf('any-pointer:fine') !== -1) matches = true;
        else if (q.indexOf('any-hover:hover') !== -1) matches = true;
        else if (q.indexOf('display-mode:browser') !== -1) matches = true;
        else if (q.indexOf('inverted-colors:none') !== -1) matches = true;
        else if (q.indexOf('min-width:') !== -1 || q.indexOf('max-width:') !== -1) {
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