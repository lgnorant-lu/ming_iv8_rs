// Multi-arg handler array: A[Q[U++]](a, b, c)
var handlers = [function(a, b, c) { return a + b + c; }, function(x) { return x * 2; }];
var indexArr = [0];
var pc = 0;
var stack = [];
stack.push(handlers[indexArr[pc++]](1, 2, 3));
