let log = [];

exports.asyncEcho = async function(x) {
  await null;
  return x;
};

exports.asyncLen = async function(x) {
  await null;
  return String(x).length;
};

exports.asyncTryEcho = async function(x, fail) {
  await null;
  if (fail) throw new Error('boom');
  return x;
};

exports.asyncRecord = async function(x) {
  await null;
  log.push(x);
};

exports.takeLog = function() {
  const s = log.join(',');
  log = [];
  return s;
};

// Reports the JS-visible type of the first argument: `Array` when
// `slice_to_array` is in effect, the typed-array name otherwise.
exports.kindOf = function(v) {
  return v?.constructor?.name ?? typeof v;
};

// Reports the element contents *and* the JS-visible type, so that a regression
// which handed over a view, or dropped/duplicated elements, is observable.
exports.joinOf = function(v) {
  if (v === undefined) return 'undefined';
  return `${v.constructor.name}:${v.join('|')}`;
};
