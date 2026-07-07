//! Event / CustomEvent / MouseEvent constructors.
//!
//! Installed as global classes via JS shim.

pub const EVENT_CONSTRUCTORS_JS: &str = r#"
(function() {
    function _defAccessor(proto, name, defaultVal) {
        var slot = '_' + name;
        var getter = function() {
            if (this === null || this === undefined) throw new TypeError('Illegal invocation');
            if (this === proto) throw new TypeError('Illegal invocation');
            var cur = Object.getPrototypeOf(this);
            var found = false;
            while (cur) { if (cur === proto) { found = true; break; } cur = Object.getPrototypeOf(cur); }
            if (!found) throw new TypeError('Illegal invocation');
            var v = this[slot];
            return v !== undefined ? v : defaultVal;
        };
        var setter = function(v) {
            if (this === null || this === undefined) throw new TypeError('Illegal invocation');
            if (this === proto) throw new TypeError('Illegal invocation');
            this[slot] = v;
        };
        try { Object.defineProperty(getter, 'name', { value: 'get ' + name, writable: false, enumerable: false, configurable: true }); } catch(e) {}
        try { Object.defineProperty(getter, 'length', { value: 0, writable: false, enumerable: false, configurable: true }); } catch(e) {}
        try { Object.defineProperty(setter, 'name', { value: 'set ' + name, writable: false, enumerable: false, configurable: true }); } catch(e) {}
        try { Object.defineProperty(setter, 'length', { value: 1, writable: false, enumerable: false, configurable: true }); } catch(e) {}
        Object.defineProperty(proto, name, {
            get: getter,
            set: setter,
            enumerable: true,
            configurable: true
        });
    }
    function _defReadOnly(proto, name, defaultVal) {
        var slot = '_' + name;
        var getter = function() {
            if (this === null || this === undefined) throw new TypeError('Illegal invocation');
            if (this === proto) throw new TypeError('Illegal invocation');
            var cur = Object.getPrototypeOf(this);
            var found = false;
            while (cur) { if (cur === proto) { found = true; break; } cur = Object.getPrototypeOf(cur); }
            if (!found) throw new TypeError('Illegal invocation');
            var v = this[slot];
            return v !== undefined ? v : defaultVal;
        };
        try { Object.defineProperty(getter, 'name', { value: 'get ' + name, writable: false, enumerable: false, configurable: true }); } catch(e) {}
        try { Object.defineProperty(getter, 'length', { value: 0, writable: false, enumerable: false, configurable: true }); } catch(e) {}
        Object.defineProperty(proto, name, {
            get: getter,
            set: undefined,
            enumerable: true,
            configurable: true
        });
    }

    function Event(type, options) {
        if (!(this instanceof Event)) {
            throw new TypeError("Failed to construct 'Event': Please use the 'new' operator");
        }
        options = options || {};
        this._type = type || '';
        this._bubbles = !!options.bubbles;
        this._cancelable = options.cancelable !== undefined ? !!options.cancelable : false;
        this._composed = !!options.composed;
        this._defaultPrevented = false;
        this._target = null;
        this._currentTarget = null;
        this._srcElement = null;
        this._eventPhase = 0;
        this._timeStamp = Date.now();
        this._isTrusted = false;
        this._returnValue = true;
        this._cancelBubble = false;
        this._stopPropagation = false;
        this._stopImmediatePropagation = false;
        this.isTrusted = false;
    }

    _defAccessor(Event.prototype, 'type', '');
    _defReadOnly(Event.prototype, 'bubbles', false);
    _defReadOnly(Event.prototype, 'cancelable', false);
    _defReadOnly(Event.prototype, 'composed', false);
    _defReadOnly(Event.prototype, 'defaultPrevented', false);
    _defReadOnly(Event.prototype, 'target', null);
    _defReadOnly(Event.prototype, 'srcElement', null);
    _defReadOnly(Event.prototype, 'currentTarget', null);
    _defReadOnly(Event.prototype, 'eventPhase', 0);
    _defReadOnly(Event.prototype, 'timeStamp', 0);
    _defReadOnly(Event.prototype, 'isTrusted', false);
    _defAccessor(Event.prototype, 'returnValue', true);
    _defAccessor(Event.prototype, 'cancelBubble', false);

    Event.prototype.preventDefault = function preventDefault() {
        if (this._cancelable) {
            this._defaultPrevented = true;
        }
    };

    Event.prototype.stopPropagation = function stopPropagation() {
        this._stopPropagation = true;
    };

    Event.prototype.stopImmediatePropagation = function stopImmediatePropagation() {
        this._stopPropagation = true;
        this._stopImmediatePropagation = true;
    };

    Event.prototype.composedPath = function composedPath() {
        return [];
    };

    Event.prototype.initEvent = function initEvent(eventType, bubbles, cancelable) {
        if (arguments.length < 1) throw new TypeError("1 argument(s) required, but only 0 present.");
        this.type = eventType;
        this._bubbles = bubbles !== undefined ? !!bubbles : false;
        this._cancelable = cancelable !== undefined ? !!cancelable : false;
    };

    if (typeof Symbol !== 'undefined' && Symbol.toStringTag) {
        Object.defineProperty(Event.prototype, Symbol.toStringTag, {
            value: 'Event', writable: false, enumerable: false, configurable: true
        });
    }

    Event.NONE = 0;
    Event.CAPTURING_PHASE = 1;
    Event.AT_TARGET = 2;
    Event.BUBBLING_PHASE = 3;
    Object.defineProperty(Event.prototype, 'NONE', {value: 0, writable: false, enumerable: true, configurable: false});
    Object.defineProperty(Event.prototype, 'CAPTURING_PHASE', {value: 1, writable: false, enumerable: true, configurable: false});
    Object.defineProperty(Event.prototype, 'AT_TARGET', {value: 2, writable: false, enumerable: true, configurable: false});
    Object.defineProperty(Event.prototype, 'BUBBLING_PHASE', {value: 3, writable: false, enumerable: true, configurable: false});

    Object.defineProperty(Event, 'prototype', {writable: false, enumerable: false, configurable: false});
    Object.defineProperty(Event, 'length', {value: 1, writable: false, enumerable: false, configurable: true});
    ['NONE', 'CAPTURING_PHASE', 'AT_TARGET', 'BUBBLING_PHASE'].forEach(function(k) {
        Object.defineProperty(Event, k, {writable: false, enumerable: true, configurable: false});
    });

    globalThis.Event = Event;

    function CustomEvent(type, options) {
        Event.call(this, type, options);
        options = options || {};
        this._detail = options.detail !== undefined ? options.detail : null;
        this.isTrusted = false;
    }
    CustomEvent.prototype = Object.create(Event.prototype);
    Object.defineProperty(CustomEvent.prototype, 'constructor', {value: CustomEvent, writable: true, enumerable: false, configurable: true});
    _defReadOnly(CustomEvent.prototype, 'detail', null);

    CustomEvent.prototype.initCustomEvent = function initCustomEvent(type, bubbles, cancelable, detail) {
        if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present.');
        this._type = type;
        this._bubbles = bubbles !== undefined ? !!bubbles : false;
        this._cancelable = cancelable !== undefined ? !!cancelable : false;
        this._detail = detail;
    };
    try { Object.defineProperty(CustomEvent.prototype.initCustomEvent, 'name', { value: 'initCustomEvent' }); } catch(e) {}
    try { Object.defineProperty(CustomEvent.prototype.initCustomEvent, 'length', { value: 1, writable: false, enumerable: false, configurable: true }); } catch(e) {}

    Object.defineProperty(CustomEvent, 'prototype', {writable: false, enumerable: false, configurable: false});
    Object.defineProperty(CustomEvent, 'length', {value: 1, writable: false, enumerable: false, configurable: true});

    globalThis.CustomEvent = CustomEvent;

    function MouseEvent(type, options) {
        Event.call(this, type, options);
        options = options || {};
        this._clientX = options.clientX || 0;
        this._clientY = options.clientY || 0;
        this._screenX = options.screenX || 0;
        this._screenY = options.screenY || 0;
        this._pageX = options.pageX || 0;
        this._pageY = options.pageY || 0;
        this._offsetX = options.offsetX || 0;
        this._offsetY = options.offsetY || 0;
        this._x = options.clientX || 0;
        this._y = options.clientY || 0;
        this._button = options.button || 0;
        this._buttons = options.buttons || 0;
        this._ctrlKey = !!options.ctrlKey;
        this._shiftKey = !!options.shiftKey;
        this._altKey = !!options.altKey;
        this._metaKey = !!options.metaKey;
        this._relatedTarget = options.relatedTarget || null;
    }
    MouseEvent.prototype = Object.create(Event.prototype);
    Object.defineProperty(MouseEvent.prototype, 'constructor', {value: MouseEvent, writable: true, enumerable: false, configurable: true});
    ['clientX','clientY','screenX','screenY','pageX','pageY','offsetX','offsetY','x','y','button','buttons'].forEach(function(prop) {
        _defReadOnly(MouseEvent.prototype, prop, 0);
    });
    ['ctrlKey','shiftKey','altKey','metaKey'].forEach(function(prop) {
        _defReadOnly(MouseEvent.prototype, prop, false);
    });
    _defReadOnly(MouseEvent.prototype, 'relatedTarget', null);
    Object.defineProperty(MouseEvent, 'prototype', {writable: false, enumerable: false, configurable: false});

    globalThis.MouseEvent = MouseEvent;

    function KeyboardEvent(type, options) {
        Event.call(this, type, options);
        options = options || {};
        this._key = options.key || '';
        this._code = options.code || '';
        this._keyCode = options.keyCode || 0;
        this._charCode = options.charCode || 0;
        this._which = options.which || options.keyCode || 0;
        this._ctrlKey = !!options.ctrlKey;
        this._shiftKey = !!options.shiftKey;
        this._altKey = !!options.altKey;
        this._metaKey = !!options.metaKey;
        this._repeat = !!options.repeat;
        this._location = options.location || 0;
    }
    KeyboardEvent.prototype = Object.create(Event.prototype);
    Object.defineProperty(KeyboardEvent.prototype, 'constructor', {value: KeyboardEvent, writable: true, enumerable: false, configurable: true});
    ['key','code'].forEach(function(prop) {
        _defReadOnly(KeyboardEvent.prototype, prop, '');
    });
    ['keyCode','charCode','which','location'].forEach(function(prop) {
        _defReadOnly(KeyboardEvent.prototype, prop, 0);
    });
    ['ctrlKey','shiftKey','altKey','metaKey','repeat'].forEach(function(prop) {
        _defReadOnly(KeyboardEvent.prototype, prop, false);
    });
    Object.defineProperty(KeyboardEvent, 'prototype', {writable: false, enumerable: false, configurable: false});

    globalThis.KeyboardEvent = KeyboardEvent;

    function PointerEvent(type, options) {
        MouseEvent.call(this, type, options);
        options = options || {};
        this._pointerId = options.pointerId || 0;
        this._width = options.width || 1;
        this._height = options.height || 1;
        this._pressure = options.pressure || 0;
        this._pointerType = options.pointerType || 'mouse';
        this._isPrimary = options.isPrimary !== undefined ? !!options.isPrimary : true;
    }
    PointerEvent.prototype = Object.create(MouseEvent.prototype);
    Object.defineProperty(PointerEvent.prototype, 'constructor', {value: PointerEvent, writable: true, enumerable: false, configurable: true});
    ['pointerId','width','height','pressure'].forEach(function(prop) {
        _defReadOnly(PointerEvent.prototype, prop, 0);
    });
    _defReadOnly(PointerEvent.prototype, 'pointerType', 'mouse');
    _defReadOnly(PointerEvent.prototype, 'isPrimary', true);
    Object.defineProperty(PointerEvent, 'prototype', {writable: false, enumerable: false, configurable: false});

    globalThis.PointerEvent = PointerEvent;
})();
"#;
