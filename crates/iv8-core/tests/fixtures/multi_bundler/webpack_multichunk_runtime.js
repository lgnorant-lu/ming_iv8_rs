// L1 synthetic: webpack-like runtime (preloaded require table)
var __webpack_require__ = function (id) {
  if (__webpack_require__.c[id]) return __webpack_require__.c[id].exports;
  var m = { exports: {}, id: id };
  __webpack_require__.c[id] = m;
  var f = __webpack_require__.m[id];
  if (typeof f === "function") f.call(m.exports, m, m.exports, __webpack_require__);
  return m.exports;
};
__webpack_require__.m = {
  0: function (module, exports, require) {
    exports.boot = true;
  },
};
__webpack_require__.c = {};
__webpack_require__.e = function () {
  return Promise.resolve();
};
