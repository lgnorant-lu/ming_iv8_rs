// Browserify minimal fixture — browser-pack prelude with two modules
// Module 1: entry point, requires module 2
// Module 2: exports a function
(function() {
    var modules = {};
    var instances = {};
    function outer() {
        function newRequire(id) {
            if (instances[id]) return instances[id].exports;
            var mod = instances[id] = { id: id, exports: {} };
            modules[id][0].call(mod.exports, newRequire, mod, mod.exports, modules[id][1]);
            return mod.exports;
        }
        return newRequire;
    }
    return outer;
})()({
    1: [function(require,module,exports) {
        var dep = require(2);
        module.exports = dep(10);
    }, {"./dep": 2}],
    2: [function(require,module,exports) {
        module.exports = function(n) { return n * 2; };
    }, {}]
},{},[1]);
