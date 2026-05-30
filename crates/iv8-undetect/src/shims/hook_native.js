(function(target) {
    // hookNative: intercept a native API by dot-path.
    //
    // Usage:
    //   __iv8__.hookNative('Math.random', function(orig, args) { return orig.apply(this, args); })
    //   __iv8__.hookNative('Navigator.prototype.userAgent', function(orig, args) { return 'spoofed'; })
    //   __iv8__.hookNative('document.getElementById', function(orig, args) { return orig.apply(this, args); })
    //
    // The hook intercepts the function at the given path. The callback receives:
    //   orig  — the original function
    //   args  — the arguments array
    //   this  — the receiver object
    //
    // Returns the hook ID (for future unhook support).

    const _hookRegistry = new Map();
    var _hookIdCounter = 0;

    function resolvePath(path) {
        // Split path into object path + property name
        // Examples:
        //   'Math.random'                → obj=Math, prop='random'
        //   'Navigator.prototype.userAgent' → obj=Navigator.prototype, prop='userAgent'
        //   'document.getElementById'   → obj=document, prop='getElementById'
        //   'window.fetch'              → obj=window, prop='fetch'
        //   'fetch'                     → obj=globalThis, prop='fetch'
        var parts = path.split('.');
        var prop = parts[parts.length - 1];

        if (parts.length === 1) {
            // Single-level: property directly on globalThis
            return { obj: globalThis, prop: prop };
        }

        var objPath = parts.slice(0, -1);

        // Resolve the object
        var obj = globalThis;
        for (var i = 0; i < objPath.length; i++) {
            if (obj == null || typeof obj !== 'object' && typeof obj !== 'function') return null;
            obj = obj[objPath[i]];
        }
        if (obj == null) return null;

        return { obj: obj, prop: prop };
    }

    target.hookNative = function hookNative(path, hookFn) {
        if (arguments.length < 1) {
            throw new Error('hookNative requires at least 1 argument');
        }
        if (typeof path !== 'string') {
            throw new TypeError('hookNative: arg 0 must be a string (path like Math.random)');
        }
        if (path.length === 0) {
            throw new Error('hookNative: api name is empty');
        }
        if (arguments.length >= 2 && typeof hookFn !== 'function') {
            throw new TypeError('hookNative: arg 1 must be a function');
        }

        var hookId = ++_hookIdCounter;

        if (!hookFn) {
            // Just register without applying
            _hookRegistry.set(hookId, { path: path, fn: null });
            return hookId;
        }

        // Resolve the path to an object + property
        var resolved = resolvePath(path);
        if (!resolved) {
            // Path not resolvable — store for later
            _hookRegistry.set(hookId, { path: path, fn: hookFn, applied: false });
            return hookId;
        }

        var obj = resolved.obj;
        var prop = resolved.prop;
        var orig = obj[prop];

        if (typeof orig !== 'function') {
            // Not a function — store but don't apply
            _hookRegistry.set(hookId, { path: path, fn: hookFn, applied: false });
            return hookId;
        }

        // Create the hooked function
        var hooked = function() {
            var args = Array.prototype.slice.call(arguments);
            return hookFn.call(this, orig, args);
        };

        // Make the hooked function look native if wrapNative is available
        if (target.wrapNative) {
            hooked = target.wrapNative(hooked, orig.name || prop);
        }

        // Apply the hook
        try {
            // Use direct assignment first (more reliable for native functions)
            obj[prop] = hooked;
        } catch(e) {
            // Fallback: Object.defineProperty
            try {
                Object.defineProperty(obj, prop, {
                    value: hooked,
                    writable: true,
                    configurable: true,
                    enumerable: Object.getOwnPropertyDescriptor(obj, prop) ?
                        (Object.getOwnPropertyDescriptor(obj, prop).enumerable || false) : false,
                });
            } catch(e2) {}
        }

        _hookRegistry.set(hookId, { path: path, fn: hookFn, orig: orig, obj: obj, prop: prop, applied: true });
        return hookId;
    };

    // unhookNative: restore original function
    target.unhookNative = function unhookNative(hookId) {
        var entry = _hookRegistry.get(hookId);
        if (!entry || !entry.applied) return false;
        try {
            Object.defineProperty(entry.obj, entry.prop, {
                value: entry.orig,
                writable: true,
                configurable: true,
            });
        } catch(e) {
            try { entry.obj[entry.prop] = entry.orig; } catch(e2) {}
        }
        _hookRegistry.delete(hookId);
        return true;
    };

    // Make hookNative/unhookNative look native
    if (target.wrapNative) {
        target.hookNative = target.wrapNative(target.hookNative, 'hookNative');
        target.unhookNative = target.wrapNative(target.unhookNative, 'unhookNative');
    }
})
