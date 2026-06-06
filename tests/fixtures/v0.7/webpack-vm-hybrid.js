// Webpack + VM dispatch hybrid
var __webpack_require__ = {};
__webpack_require__.m = { 0: function() { return "entry"; } };
__webpack_require__.c = {};
var handlers = [function(a) { return a + 1; }, function(b) { return b * 2; }];
var pc = 0;
var opcodes = [0, 1, 0, 1];
var stack = [];
for (var i = 0; i < opcodes.length; i++) {
    stack.push(handlers[opcodes[i]](pc++));
}
