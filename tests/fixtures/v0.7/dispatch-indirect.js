// Indirect handler map — dispatch through a mapping table
var handlerMap = { add: function(a, b) { return a + b; }, mul: function(x) { return x * 2; } };
var opcodes = ["add", "mul"];
var pc = 0;
var result = handlerMap[opcodes[pc++]](3, 4);
