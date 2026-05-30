//! Event / CustomEvent / MouseEvent constructors.
//!
//! Installed as global classes via JS shim.

/// JS shim that installs Event, CustomEvent, MouseEvent constructors.
pub const EVENT_CONSTRUCTORS_JS: &str = r#"
(function() {
    // ─── Event ──────────────────────────────────────────────────────────────
    function Event(type, options) {
        if (!(this instanceof Event)) {
            throw new TypeError("Failed to construct 'Event': Please use the 'new' operator");
        }
        options = options || {};
        this.type = type || '';
        this.bubbles = !!options.bubbles;
        this.cancelable = options.cancelable !== undefined ? !!options.cancelable : false;
        this.composed = !!options.composed;
        this.defaultPrevented = false;
        this.target = null;
        this.currentTarget = null;
        this.eventPhase = 0;
        this.timeStamp = Date.now();
        this.isTrusted = false;
        this._stopPropagation = false;
        this._stopImmediatePropagation = false;
    }

    Event.prototype.preventDefault = function() {
        if (this.cancelable) {
            this.defaultPrevented = true;
        }
    };

    Event.prototype.stopPropagation = function() {
        this._stopPropagation = true;
    };

    Event.prototype.stopImmediatePropagation = function() {
        this._stopPropagation = true;
        this._stopImmediatePropagation = true;
    };

    Event.prototype.composedPath = function() {
        return [];
    };

    Event.NONE = 0;
    Event.CAPTURING_PHASE = 1;
    Event.AT_TARGET = 2;
    Event.BUBBLING_PHASE = 3;

    globalThis.Event = Event;

    // ─── CustomEvent ────────────────────────────────────────────────────────
    function CustomEvent(type, options) {
        Event.call(this, type, options);
        options = options || {};
        this.detail = options.detail !== undefined ? options.detail : null;
    }
    CustomEvent.prototype = Object.create(Event.prototype);
    CustomEvent.prototype.constructor = CustomEvent;

    globalThis.CustomEvent = CustomEvent;

    // ─── MouseEvent ─────────────────────────────────────────────────────────
    function MouseEvent(type, options) {
        Event.call(this, type, options);
        options = options || {};
        this.clientX = options.clientX || 0;
        this.clientY = options.clientY || 0;
        this.screenX = options.screenX || 0;
        this.screenY = options.screenY || 0;
        this.pageX = options.pageX || 0;
        this.pageY = options.pageY || 0;
        this.offsetX = options.offsetX || 0;
        this.offsetY = options.offsetY || 0;
        this.button = options.button || 0;
        this.buttons = options.buttons || 0;
        this.ctrlKey = !!options.ctrlKey;
        this.shiftKey = !!options.shiftKey;
        this.altKey = !!options.altKey;
        this.metaKey = !!options.metaKey;
        this.relatedTarget = options.relatedTarget || null;
    }
    MouseEvent.prototype = Object.create(Event.prototype);
    MouseEvent.prototype.constructor = MouseEvent;

    globalThis.MouseEvent = MouseEvent;

    // ─── KeyboardEvent ──────────────────────────────────────────────────────
    function KeyboardEvent(type, options) {
        Event.call(this, type, options);
        options = options || {};
        this.key = options.key || '';
        this.code = options.code || '';
        this.keyCode = options.keyCode || 0;
        this.charCode = options.charCode || 0;
        this.which = options.which || options.keyCode || 0;
        this.ctrlKey = !!options.ctrlKey;
        this.shiftKey = !!options.shiftKey;
        this.altKey = !!options.altKey;
        this.metaKey = !!options.metaKey;
        this.repeat = !!options.repeat;
        this.location = options.location || 0;
    }
    KeyboardEvent.prototype = Object.create(Event.prototype);
    KeyboardEvent.prototype.constructor = KeyboardEvent;

    globalThis.KeyboardEvent = KeyboardEvent;

    // ─── PointerEvent ───────────────────────────────────────────────────────
    function PointerEvent(type, options) {
        MouseEvent.call(this, type, options);
        options = options || {};
        this.pointerId = options.pointerId || 0;
        this.width = options.width || 1;
        this.height = options.height || 1;
        this.pressure = options.pressure || 0;
        this.pointerType = options.pointerType || 'mouse';
        this.isPrimary = options.isPrimary !== undefined ? !!options.isPrimary : true;
    }
    PointerEvent.prototype = Object.create(MouseEvent.prototype);
    PointerEvent.prototype.constructor = PointerEvent;

    globalThis.PointerEvent = PointerEvent;
})();
"#;
