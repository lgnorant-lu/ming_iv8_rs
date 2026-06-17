// Rollup IIFE minimal fixture — self-executing bundle with PURE annotation
// Simulates Rollup's IIFE output with a pure function annotation
var _interopRequireDefault = function(obj) { return obj && obj.__esModule ? obj : { default: obj }; };
var myLib = /*#__PURE__*/function() {
    function add(a, b) { return a + b; }
    return { add: add };
}();
if (typeof globalThis !== 'undefined') { globalThis.__rollup_result = myLib; }
