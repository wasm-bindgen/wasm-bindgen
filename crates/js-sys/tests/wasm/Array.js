const wasm = require("wasm-bindgen-test.js");
const assert = require("assert");

// Used for `Array.rs` tests
exports.populate_array = function(arr, start, len) {
  var isBigInt = typeof(arr[0]) === "bigint";
  for (i = 0; i < len; i++) {
    arr[i] = isBigInt ? BigInt(start + i) : start + i;
  }
};

// Create an async iterable for testing Array.fromAsync
exports.createAsyncIterable = function(values) {
  return {
    async *[Symbol.asyncIterator]() {
      for (const value of values) {
        yield value;
      }
    }
  };
};

class TestItem {
  constructor(id, name) {
    this._id = id;
    this._name = name;
  }

  get id() {
    return this._id;
  }

  get name() {
    return this._name;
  }

  with_prefix(prefix) {
    return new TestItem(this._id, prefix + this._name);
  }
}

exports.TestItem = TestItem;

exports.createTestItemArray = function () {
  return [
    new TestItem(1, "first"),
    new TestItem(2, "second"),
    new TestItem(3, "third"),
  ];
};

exports.processTestItemArray = function (arr) {
  return arr.reduce((sum, item) => sum + item.id, 0);
};

exports.checkArrayType = function (arr) {
  return Array.isArray(arr) && arr.every((item) => item instanceof TestItem);
};

exports.createThrowingIterable = function () {
  return {
    [Symbol.iterator]: function* () {
      yield 1;
      yield 2;
      throw new Error("iterator error");
    },
  };
};
