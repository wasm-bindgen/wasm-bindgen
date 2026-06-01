const assert = require('assert');

let next = null;

exports.assert_next_undefined = function() {
  next = undefined;
};

exports.assert_next_ten = function() {
  next = 10;
};

exports.foo = function(a) {
  console.log(a, next);
  assert.strictEqual(a, next);
  next = null;
};

let GENERIC_CROSS = [];

exports.generic_record = function(x) {
  GENERIC_CROSS.push(x);
};

exports.take_generic_cross = function() {
  const v = GENERIC_CROSS;
  GENERIC_CROSS = [];
  return v;
};
