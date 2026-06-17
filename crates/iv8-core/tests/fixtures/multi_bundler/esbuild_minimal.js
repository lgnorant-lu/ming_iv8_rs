// esbuild minimal fixture — arrow IIFE, no specific markers
// Simulates esbuild output (compact, arrow-based IIFE)
(() => {
    var result = { sum: function(a, b) { return a + b; } };
    if (typeof globalThis !== 'undefined') {
        globalThis.__esbuild_result = result;
    }
})();
