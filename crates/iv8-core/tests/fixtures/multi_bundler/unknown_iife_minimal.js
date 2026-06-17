// Unknown IIFE minimal fixture — generic self-executing wrapper
// No bundler-specific markers, used for classification as UnknownIife
(function() {
    var value = 42;
    if (typeof globalThis !== 'undefined') {
        globalThis.__unknown_result = value;
    }
})();
