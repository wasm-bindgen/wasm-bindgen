exports.MyDurableObject = class MyDurableObject {
  constructor() {}
};

exports.MyDurableObjectStub = class MyDurableObjectStub {
  constructor() {}
};

exports.DurableObjectNamespace = class DurableObjectNamespace {
  constructor() {}

  getByName(name) {
    return new exports.MyDurableObjectStub();
  }
};

// Simple value-holding class used to demonstrate the non-identity
// `IntoJsGeneric` / `FromJsGeneric` path in `tests/wasm/generics.rs`.
// The methods are deliberately unaware of the Rust type — they just
// stash whatever JS value Rust passes in and hand it back on get.
exports.Cell = class Cell {
  constructor() {
    this._v = null;
  }
  getValue() {
    return this._v;
  }
  setValue(v) {
    this._v = v;
  }
};
