// Generic member call — unsupported join point
var obj = { method: function() { return 1; } };
var key = "method";
var r = obj[key]();
