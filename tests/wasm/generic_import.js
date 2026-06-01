let GENERIC_LOG = [];

exports.record_generic = function(x) {
  GENERIC_LOG.push(x);
};

exports.take_generic_log = function() {
  const v = GENERIC_LOG;
  GENERIC_LOG = [];
  return v;
};

exports.record_mixed = function(a, b, c) {
  GENERIC_LOG.push(a);
  GENERIC_LOG.push(b);
  GENERIC_LOG.push(c);
};

exports.record_two = function(a, b) {
  GENERIC_LOG.push(a);
  GENERIC_LOG.push(b);
};

exports.groundtrip = function(x) {
  return x;
};

exports.gget = function() {
  return 7;
};

exports.record_opt = function(x) {
  GENERIC_LOG.push(x);
};

exports.record_ref = function(x) {
  GENERIC_LOG.push(x);
};

exports.try_maybe = function(x) {
  if (x === 13) throw new Error("unlucky");
  return x;
};

exports.record_vec = function(xs) {
  GENERIC_LOG.push(xs);
};

exports.call_each = function(f) {
  f(1);
  f(2);
  f(3);
};

exports.call_each_option = function(f) {
  f(5);
  f(undefined);
  f(7);
};

exports.call_each_return = function(f) {
  const a = f();
  const b = f();
  return a + b;
};

exports.Recorder = class Recorder {
  constructor() {
    this.items = [];
  }
  pushVal(x) {
    this.items.push(x);
  }
  get last() {
    return this.items[this.items.length - 1];
  }
};
