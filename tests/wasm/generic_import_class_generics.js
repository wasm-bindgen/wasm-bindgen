// A minimal stand-in for a `js-sys`-style generic container (`Array<T>`,
// `Iterator<T>`, ...): a single JS class whose "genericness" is purely a Rust
// concept, since JS itself has no type parameters. What's under test is
// whether the *Rust* binding correctly threads a class-level `T` through an
// `impl<T> Container<T> { .. }` block.
class Container {
  constructor(value) {
    this._value = value;
    this._items = [];
  }

  get value() {
    return this._value;
  }

  set value(v) {
    this._value = v;
  }

  push(item) {
    this._items.push(item);
    return this._items.length;
  }

  static of(value) {
    const c = new Container(value);
    c._kind = "static";
    return c;
  }
}

exports.Container = Container;

exports.containerValue = function (c) {
  return c._value;
};

exports.containerItems = function (c) {
  return c._items.join(",");
};
