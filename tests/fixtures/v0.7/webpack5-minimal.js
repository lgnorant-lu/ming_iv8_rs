function __webpack_require__(moduleId) {
    var module = __webpack_require__.c[moduleId] = { exports: {} };
    return module.exports;
}
__webpack_require__.m = { 0: function() { return "entry"; } };
__webpack_require__.c = {};
__webpack_require__.d = function(exports, name, getter) {
    Object.defineProperty(exports, name, { get: getter });
};
__webpack_require__.e = function(chunkId) {
    return Promise.resolve();
};
__webpack_require__.f = {};
__webpack_require__.r = function(exports) {
    Object.defineProperty(exports, "__esModule", { value: true });
};
// Expose globally
globalThis.__webpack_require__ = __webpack_require__;
globalThis.__webpack_require__.c = {};
self.webpackChunk = self.webpackChunk || [];
self.webpackChunk.push(["main", { 0: function() {} }]);
