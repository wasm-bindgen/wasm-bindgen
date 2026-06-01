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
