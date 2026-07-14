// L1 synthetic: vendor chunk (webpack 5 webpackChunk push)
(self["webpackChunk"] = self["webpackChunk"] || []).push([
  ["vendors"],
  {
    50: function (module, exports, require) {
      exports.lib = "vendor-lib";
      exports.n = 7;
    },
    51: function (module, exports, require) {
      var v = require(50);
      exports.via = v.lib;
      exports.sum = v.n + 1;
    },
  },
]);
