// L1 synthetic: page entry that depends on vendor modules (ids 50/51)
// Expect: after runtime+vendor preload/merge, require(51).via === "vendor-lib"
(function () {
  var r =
    typeof __webpack_require__ === "function"
      ? __webpack_require__
      : globalThis.__webpack_require__;
  if (typeof r !== "function") {
    globalThis.__iv8_page_err = "no-require";
    return;
  }
  try {
    var page = r(51);
    globalThis.__iv8_page = {
      via: page.via,
      sum: page.sum,
      boot: r(0).boot,
    };
  } catch (e) {
    globalThis.__iv8_page_err = String(e && e.message ? e.message : e);
  }
})();
