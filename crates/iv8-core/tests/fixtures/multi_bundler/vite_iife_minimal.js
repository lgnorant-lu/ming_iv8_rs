// Vite IIFE minimal fixture — Vite output in IIFE format
// Simulates Vite build with build.rollupOptions.output.format = 'iife'
const __vitePreload = function preload(url, dep, asType) {
    return Promise.resolve();
};
(function() {
    var hello = "Hello from Vite IIFE";
    if (typeof globalThis !== 'undefined') {
        globalThis.__vite_result = hello;
    }
})();
