let GENERIC_LOG = [];

exports.record_generic = function(x) {
  GENERIC_LOG.push(x);
};

exports.take_generic_log = function() {
  const v = GENERIC_LOG;
  GENERIC_LOG = [];
  return v;
};
