// Rollup UMD minimal fixture — three-branch AMD/CJS/global dispatch
(function(root, factory) {
    if (typeof define === 'function' && define.amd) {
        define([], factory);
    } else if (typeof module === 'object' && typeof module.exports === 'object') {
        module.exports = factory();
    } else {
        root.MyLib = factory();
    }
})(typeof globalThis !== 'undefined' ? globalThis : this, function() {
    return { version: '1.0.0', hello: function() { return 'world'; } };
});
