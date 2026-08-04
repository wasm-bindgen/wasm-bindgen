exports.fillSlice = function (xs, base) {
  // Write through the mutable typed-array view. If the binding handed us a
  // copy instead of a live view into wasm memory, the caller's slice will be
  // unchanged and the Rust side will notice.
  for (let i = 0; i < xs.length; i++) {
    xs[i] = base + i;
  }
};

exports.withCallback = function (f, times) {
  for (let i = 0; i < times; i++) {
    f(i);
  }
};

exports.tryGet = function (key) {
  if (key === 0) {
    throw new Error("boom");
  }
  return key * 2;
};

exports.tryGetString = function (key) {
  if (key === 0) {
    throw new Error("boom");
  }
  return "v" + key;
};

exports.optEcho = function (x) {
  return x;
};

exports.optDescribe = function (x) {
  return x === undefined || x === null ? "none" : "some:" + x;
};

exports.variadicJoin = function (first, ...rest) {
  // If the final argument were passed as a single array rather than spread,
  // `rest` would be `[[1,2,3]]` and this would produce "0:1,2,3" with length 1.
  return first + ":" + rest.length + ":" + rest.join("|");
};
