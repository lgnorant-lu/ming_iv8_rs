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

    // getComputedStyle stub
    globalThis.getComputedStyle = function(element, pseudoElt) {
        var styles = {
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
            padding: '0px',
            border: '0px none rgb(0, 0, 0)',
            overflow: 'visible',
            opacity: '1',
            zIndex: 'auto',
            transform: 'none',
            transition: 'all 0s ease 0s',
        };
        styles.getPropertyValue = function(prop) {
            var kebab = prop.replace(/([A-Z])/g, '-$1').toLowerCase();
            return this[prop] || this[kebab] || '';
        };
        styles.length = Object.keys(styles).length - 1;
        return styles;
    };
})();
"#;