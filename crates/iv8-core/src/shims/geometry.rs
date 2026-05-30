//! getBoundingClientRect + offsetWidth/offsetHeight stubs.
//!
//! Returns configurable default values (from environment or sensible defaults).
//! No real layout engine — just static values that pass fingerprint checks.

/// JS shim for getBoundingClientRect and geometry properties.
pub const GEOMETRY_SHIM_JS: &str = r#"
(function() {
    // Default DOMRect values (configurable via environment in future)
    var defaultRect = {x: 0, y: 0, width: 0, height: 0, top: 0, right: 0, bottom: 0, left: 0};

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

    // Store original __addNavProps__ and extend it
    var _origAddNav = globalThis.__addNavProps__;
    globalThis.__addNavProps__ = function(node) {
        if (_origAddNav) node = _origAddNav(node);
        if (!node || typeof node !== 'object' || !node.__nodeId__) return node;
        if (node.__geomInstalled__) return node;

        // getBoundingClientRect
        node.getBoundingClientRect = function() {
            return new DOMRect(0, 0, 0, 0);
        };

        // Geometry properties (default 0, real values would need layout engine)
        Object.defineProperties(node, {
            offsetWidth: { get: function() { return 0; }, enumerable: true },
            offsetHeight: { get: function() { return 0; }, enumerable: true },
            offsetTop: { get: function() { return 0; }, enumerable: true },
            offsetLeft: { get: function() { return 0; }, enumerable: true },
            clientWidth: { get: function() { return 0; }, enumerable: true },
            clientHeight: { get: function() { return 0; }, enumerable: true },
            scrollWidth: { get: function() { return 0; }, enumerable: true },
            scrollHeight: { get: function() { return 0; }, enumerable: true },
            scrollTop: { value: 0, writable: true, enumerable: true },
            scrollLeft: { value: 0, writable: true, enumerable: true },
            __geomInstalled__: { value: true, enumerable: false },
        });

        return node;
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
            // Convert camelCase to kebab-case
            var kebab = prop.replace(/([A-Z])/g, '-$1').toLowerCase();
            return this[prop] || this[kebab] || '';
        };
        styles.length = Object.keys(styles).length - 1; // exclude getPropertyValue
        return styles;
    };
})();
"#;
