(function(target) {
    const _nativeToString = Function.prototype.toString;
    const _wrappedFunctions = new WeakSet();
    const _wrappedNames = new WeakMap();

    // Override Function.prototype.toString
    Function.prototype.toString = function() {
        if (_wrappedFunctions.has(this)) {
            const name = _wrappedNames.get(this) || '';
            return 'function ' + name + '() { [native code] }';
        }
        return _nativeToString.call(this);
    };
    // Make toString itself look native
    _wrappedFunctions.add(Function.prototype.toString);
    _wrappedNames.set(Function.prototype.toString, 'toString');

    target.wrapNative = function wrapNative(fn, name) {
        if (typeof fn !== 'function') {
            throw new TypeError('wrapNative: first argument must be a function');
        }
        name = name || fn.name || '';

        // Create wrapper that delegates to original
        const wrapper = function() {
            return fn.apply(this, arguments);
        };

        // Set name and length
        Object.defineProperty(wrapper, 'name', { value: name, configurable: true });
        Object.defineProperty(wrapper, 'length', { value: fn.length, configurable: true });

        // Mark as native
        _wrappedFunctions.add(wrapper);
        _wrappedNames.set(wrapper, name);

        return wrapper;
    };

    // Make wrapNative itself look native
    _wrappedFunctions.add(target.wrapNative);
    _wrappedNames.set(target.wrapNative, 'wrapNative');
})