//! WindowProperties interface shim.
//!
//! WindowProperties is not in webref IDL, so we create it manually.
//! Window.prototype.__proto__ must be WindowProperties.prototype.

/// JS shim for the WindowProperties interface.
pub const WINDOW_PROPERTIES_SHIM_JS: &str = r#"
(function() {
    try {
        var winCtor = globalThis.Window;
        if (winCtor && winCtor.prototype) {
            var wpCtor = function WindowProperties() { throw new TypeError('Illegal constructor'); };
            Object.defineProperty(wpCtor.prototype, Symbol.toStringTag, {
                value: 'WindowProperties', writable: true, configurable: true, enumerable: false
            });
            Object.setPrototypeOf(wpCtor, Function.prototype);
            Object.setPrototypeOf(wpCtor.prototype, EventTarget.prototype || Object.prototype);
            Object.defineProperty(globalThis, 'WindowProperties', {
                value: wpCtor, writable: true, configurable: true, enumerable: false
            });
            Object.setPrototypeOf(winCtor.prototype, wpCtor.prototype);
        }
    } catch(e) {}
})();
"#;
