// Zero-arg handler array: A[Q[U++]]()
var handlers = [function() { return 1; }, function() { return 2; }];
var indexArr = [0];
var pc = 0;
var stack = [];
stack.push(handlers[indexArr[pc++]]());
