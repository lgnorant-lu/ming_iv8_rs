// BDMS negative: plain CommonJS without webpack module table
function add(a, b) {
  return a + b;
}
module.exports = { add: add, name: "plain-cjs" };
