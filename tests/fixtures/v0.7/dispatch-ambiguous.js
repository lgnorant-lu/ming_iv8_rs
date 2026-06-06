// Ambiguous computed member call — not VM dispatch
var obj = { method: function() { return 42; } };
var key = "method";
var result = obj[key]();
