exports.takeRefPrimitive = function (x) {
  return x + 1;
};

exports.doubleRef = function (x) {
  return x * 2;
};

exports.concatStr = function (s, suffix) {
  return s + ":" + suffix;
};

exports.joinStrs = function (a, b, suffix) {
  return a + "|" + b + ":" + suffix;
};

// Used by the method-receiver cases below. `scale` multiplies by the instance's
// own factor, so a binding that passed the wrong receiver (or passed the
// receiver as the first argument) produces a visibly wrong number rather than a
// plausible one.
class Scaler {
  constructor(factor) {
    this.factor = factor;
  }

  scale(x) {
    return x * this.factor;
  }

  scaleRef(x) {
    return x * this.factor;
  }
}

exports.Scaler = Scaler;

// Class with a Rust-side lifetime parameter (`type Tagged<'a>`): invisible
// here, it is purely a Rust borrow-tracking concern.
class Tagged {
  constructor(tag) {
    this._tag = tag;
  }

  tagWith(x) {
    return this._tag + ":" + x;
  }
}

exports.Tagged = Tagged;
