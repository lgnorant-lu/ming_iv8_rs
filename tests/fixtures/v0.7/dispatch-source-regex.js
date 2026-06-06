var require = function() { return function() {}; };
var ctx = {}, arg = {};
var req = require("./module");
var result = req.call(ctx, arg);

