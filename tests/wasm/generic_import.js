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

exports.async_echo = function(x) {
  return Promise.resolve(x);
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

exports.record_ref_mut = function(x) {
  GENERIC_LOG.push(x);
};

exports.record_many = function(...args) {
  for (const a of args) GENERIC_LOG.push(a);
};

exports.record_vec_opt = function(xs) {
  GENERIC_LOG.push(xs);
};

exports.call_pair = function(f) {
  f(1, "a");
  f(2, "b");
};

exports.Stats = class Stats {
  static combine(a, b) {
    return a + b;
  }
};

exports.Boxed = class Boxed {
  constructor(v) {
    this.v = v;
  }
  unwrap() {
    return this.v;
  }
};

exports.Recorder = class Recorder {
  constructor() {
    this.items = [];
    this._tag = null;
  }
  pushVal(x) {
    this.items.push(x);
  }
  get last() {
    return this.items[this.items.length - 1];
  }
  set tag(x) {
    this._tag = x;
  }
  get tag() {
    return this._tag;
  }
};
