//! Event / CustomEvent / MouseEvent / KeyboardEvent / PointerEvent constructors.
//!
//! North Star Phase 1 (v0.8.90): shim preserves codegen prototype.
//! Instead of creating new constructors with `Object.create()`, the shim
//! wraps the codegen constructors and installs JS accessors on the codegen
//! prototype. This keeps Symbol.toStringTag, instanceof, and prototype chain
//! intact without post-hoc fixes (TO_STRING_TAG_FIX_JS, fix_proto_js Event
//! getter wrapping).

pub const EVENT_CONSTRUCTORS_JS: &str = r#"
(function() {
    if (globalThis.__iv8EventShimInstalled) return;
    globalThis.__iv8EventShimInstalled = true;
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

    // --- Phase 1: preserve codegen prototype ---
    // Grab codegen constructors (installed by install_all.rs before shims)
    var CodegenEvent = globalThis.Event;
    var CodegenCustomEvent = globalThis.CustomEvent;
    var CodegenMouseEvent = globalThis.MouseEvent;
    var CodegenKeyboardEvent = globalThis.KeyboardEvent;
    var CodegenPointerEvent = globalThis.PointerEvent;

    // Install JS accessors on codegen prototypes (override native getters)
    var EventProto = CodegenEvent.prototype;

    function _initEventSlots(inst, type, options) {
        options = options || {};
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, 'isTrusted', { value: false, writable: false, enumerable: true, configurable: true });
    }

    _defAccessor(EventProto, 'type', '');
    _defReadOnly(EventProto, 'bubbles', false);
    _defReadOnly(EventProto, 'cancelable', false);
    _defReadOnly(EventProto, 'composed', false);
    _defReadOnly(EventProto, 'defaultPrevented', false);
    _defReadOnly(EventProto, 'target', null);
    _defReadOnly(EventProto, 'srcElement', null);
    _defReadOnly(EventProto, 'currentTarget', null);
    _defReadOnly(EventProto, 'eventPhase', 0);
    _defReadOnly(EventProto, 'timeStamp', 0);
    _defReadOnly(EventProto, 'isTrusted', false);
    _defAccessor(EventProto, 'returnValue', true);
    _defAccessor(EventProto, 'cancelBubble', false);

    EventProto.preventDefault = function preventDefault() {
        if (this._cancelable) {
            this._defaultPrevented = true;
        }
    };

    EventProto.stopPropagation = function stopPropagation() {
        this._stopPropagation = true;
    };

    EventProto.stopImmediatePropagation = function stopImmediatePropagation() {
        this._stopPropagation = true;
        this._stopImmediatePropagation = true;
    };

    EventProto.composedPath = function composedPath() {
        return [];
    };

    EventProto.initEvent = function initEvent(eventType, bubbles, cancelable) {
        if (arguments.length < 1) throw new TypeError("1 argument(s) required, but only 0 present.");
        this._type = eventType;
        this._bubbles = bubbles !== undefined ? !!bubbles : false;
        this._cancelable = cancelable !== undefined ? !!cancelable : false;
    };
    try { Object.defineProperty(EventProto.initEvent, 'name', { value: 'initEvent' }); } catch(e) {}
    try { Object.defineProperty(EventProto.initEvent, 'length', { value: 1, writable: false, enumerable: false, configurable: true }); } catch(e) {}

    // Wrap codegen Event constructor: call codegen constructor (for V8 internal
    // setup), then initialize JS slots.
    function Event(type, options) {
        if (!(this instanceof Event)) {
            throw new TypeError("Failed to construct 'Event': Please use the 'new' operator");
        }
        var inst = Reflect.construct(CodegenEvent, [], new.target || Event);
        _initEventSlots(inst, type, options);
        return inst;
    }
    // Preserve codegen prototype (do NOT replace with Object.create)
    Event.prototype = EventProto;
    Object.defineProperty(Event.prototype, 'constructor', {value: Event, writable: true, enumerable: false, configurable: true});
    Object.defineProperty(Event, 'prototype', {writable: false, enumerable: false, configurable: false});
    Object.defineProperty(Event, 'length', {value: 1, writable: false, enumerable: false, configurable: true});

    // Copy static constants from codegen (they may already be there)
    if (CodegenEvent.NONE !== undefined) {
        Event.NONE = CodegenEvent.NONE;
        Event.CAPTURING_PHASE = CodegenEvent.CAPTURING_PHASE;
        Event.AT_TARGET = CodegenEvent.AT_TARGET;
        Event.BUBBLING_PHASE = CodegenEvent.BUBBLING_PHASE;
    } else {
        Event.NONE = 0;
        Event.CAPTURING_PHASE = 1;
        Event.AT_TARGET = 2;
        Event.BUBBLING_PHASE = 3;
    }
    ['NONE', 'CAPTURING_PHASE', 'AT_TARGET', 'BUBBLING_PHASE'].forEach(function(k) {
        Object.defineProperty(Event, k, {writable: false, enumerable: true, configurable: false});
    });

    globalThis.Event = Event;

    // --- CustomEvent ---
    var CEProto = CodegenCustomEvent.prototype;
    // Ensure prototype chain: CustomEvent.prototype → Event.prototype
    Object.setPrototypeOf(CEProto, EventProto);
    Object.defineProperty(CEProto, 'constructor', {value: CodegenCustomEvent, writable: true, enumerable: false, configurable: true});

    _defReadOnly(CEProto, 'detail', null);

    CEProto.initCustomEvent = function initCustomEvent(type, bubbles, cancelable, detail) {
        if (arguments.length < 1) throw new TypeError('1 argument(s) required, but only 0 present.');
        this._type = type;
        this._bubbles = bubbles !== undefined ? !!bubbles : false;
        this._cancelable = cancelable !== undefined ? !!cancelable : false;
        this._detail = detail;
    };
    try { Object.defineProperty(CEProto.initCustomEvent, 'name', { value: 'initCustomEvent' }); } catch(e) {}
    try { Object.defineProperty(CEProto.initCustomEvent, 'length', { value: 1, writable: false, enumerable: false, configurable: true }); } catch(e) {}

    function CustomEvent(type, options) {
        if (!(this instanceof CustomEvent)) {
            throw new TypeError("Failed to construct 'CustomEvent': Please use the 'new' operator");
        }
        var inst = Reflect.construct(CodegenCustomEvent, [], new.target || CustomEvent);
        _initEventSlots(inst, type, options);
        options = options || {};
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        return inst;
    }
    CustomEvent.prototype = CEProto;
    Object.defineProperty(CustomEvent, 'prototype', {writable: false, enumerable: false, configurable: false});
    Object.defineProperty(CustomEvent, 'length', {value: 1, writable: false, enumerable: false, configurable: true});

    globalThis.CustomEvent = CustomEvent;

    // --- MouseEvent ---
    var MEProto = CodegenMouseEvent.prototype;
    Object.setPrototypeOf(MEProto, EventProto);
    Object.defineProperty(MEProto, 'constructor', {value: CodegenMouseEvent, writable: true, enumerable: false, configurable: true});

    ['clientX','clientY','screenX','screenY','pageX','pageY','offsetX','offsetY','x','y','button','buttons','layerX','layerY','movementX','movementY'].forEach(function(prop) {
        _defReadOnly(MEProto, prop, 0);
    });
    ['ctrlKey','shiftKey','altKey','metaKey'].forEach(function(prop) {
        _defReadOnly(MEProto, prop, false);
    });
    _defReadOnly(MEProto, 'relatedTarget', null);

    function MouseEvent(type, options) {
        if (!(this instanceof MouseEvent)) {
            throw new TypeError("Failed to construct 'MouseEvent': Please use the 'new' operator");
        }
        var inst = Reflect.construct(CodegenMouseEvent, [], new.target || MouseEvent);
        _initEventSlots(inst, type, options);
        options = options || {};
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        return inst;
    }
    MouseEvent.prototype = MEProto;
    Object.defineProperty(MouseEvent, 'prototype', {writable: false, enumerable: false, configurable: false});

    globalThis.MouseEvent = MouseEvent;

    // --- KeyboardEvent ---
    var KEProto = CodegenKeyboardEvent.prototype;
    Object.setPrototypeOf(KEProto, EventProto);
    Object.defineProperty(KEProto, 'constructor', {value: CodegenKeyboardEvent, writable: true, enumerable: false, configurable: true});

    ['key','code'].forEach(function(prop) {
        _defReadOnly(KEProto, prop, '');
    });
    ['keyCode','charCode','which','location'].forEach(function(prop) {
        _defReadOnly(KEProto, prop, 0);
    });
    ['ctrlKey','shiftKey','altKey','metaKey','repeat','isComposing'].forEach(function(prop) {
        _defReadOnly(KEProto, prop, false);
    });

    function KeyboardEvent(type, options) {
        if (!(this instanceof KeyboardEvent)) {
            throw new TypeError("Failed to construct 'KeyboardEvent': Please use the 'new' operator");
        }
        var inst = Reflect.construct(CodegenKeyboardEvent, [], new.target || KeyboardEvent);
        _initEventSlots(inst, type, options);
        options = options || {};
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        return inst;
    }
    KeyboardEvent.prototype = KEProto;
    Object.defineProperty(KeyboardEvent, 'prototype', {writable: false, enumerable: false, configurable: false});

    globalThis.KeyboardEvent = KeyboardEvent;

    // --- PointerEvent ---
    var PEProto = CodegenPointerEvent.prototype;
    Object.setPrototypeOf(PEProto, MEProto);
    Object.defineProperty(PEProto, 'constructor', {value: CodegenPointerEvent, writable: true, enumerable: false, configurable: true});

    ['pointerId','width','height','pressure','tiltX','tiltY','twist','tangentialPressure','altitudeAngle','azimuthAngle','persistentDeviceId'].forEach(function(prop) {
        _defReadOnly(PEProto, prop, 0);
    });
    _defReadOnly(PEProto, 'pointerType', 'mouse');
    _defReadOnly(PEProto, 'isPrimary', true);

    function PointerEvent(type, options) {
        if (!(this instanceof PointerEvent)) {
            throw new TypeError("Failed to construct 'PointerEvent': Please use the 'new' operator");
        }
        var inst = Reflect.construct(CodegenPointerEvent, [], new.target || PointerEvent);
        _initEventSlots(inst, type, options);
        options = options || {};
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        Object.defineProperty(inst, '', {value: , writable: true, enumerable: false, configurable: true});
        return inst;
    }
    PointerEvent.prototype = PEProto;
    Object.defineProperty(PointerEvent, 'prototype', {writable: false, enumerable: false, configurable: false});

    globalThis.PointerEvent = PointerEvent;
})();
"#;
