// Valid dispatch expression for AST transform
var A = [function() { return 1; }, function() { return 2; }];
var Q = [0];
var U = 0;
var result = A[Q[U++]]();
