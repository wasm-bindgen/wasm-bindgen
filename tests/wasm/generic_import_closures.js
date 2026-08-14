exports.concreteCallback = function (times, f) {
  for (let i = 0; i < times; i++) {
    f(i);
  }
};

exports.forEachOwned = function (xs, f) {
  for (const x of xs) {
    f(x);
  }
};

exports.buildValue = function (f) {
  return f(1);
};

exports.transformValue = function (x, f) {
  return f(x);
};

exports.foldValues = function (xs, init, mapper, reducer) {
  let acc = init;
  for (const x of xs) {
    acc = reducer(acc, mapper(x));
  }
  return acc;
};

exports.mapValues = function (xs, mapper) {
  return xs.map(mapper);
};

class Bucket {
  constructor(items) {
    this.items = items;
  }
}

Bucket.prototype.forEach = function (f) {
  for (let i = 0; i < this.items.length; i++) {
    f(this.items[i], i, this);
  }
};

Bucket.prototype.every = function (predicate) {
  for (let i = 0; i < this.items.length; i++) {
    if (!predicate(this.items[i], i, this)) {
      return false;
    }
  }
  return true;
};

exports.Bucket = Bucket;

exports.bucketLen = function (b) {
  return b.items.length;
};
