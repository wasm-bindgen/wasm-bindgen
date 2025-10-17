const wasm = require("wasm-bindgen-test.js");
const assert = require("assert");

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
