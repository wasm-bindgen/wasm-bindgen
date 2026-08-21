// Backing JS for `generic_import_class_generics.rs`: real classes with real
// mutable state, so the Rust-side assertions check observable JS behaviour
// rather than merely "it didn't panic".

class Holder {
  constructor(value) {
    this._value = value;
    this._fromStatic = false;
    this._combined = undefined;
  }

  get() {
    return this._value;
  }

  get value() {
    return this._value;
  }

  static of(value) {
    const h = new Holder(value);
    h._fromStatic = true;
    return h;
  }

  // Non-hoisted `U` alongside the hoisted class parameter `T`: records
  // whatever the caller passed so the test can assert the *value*, not just
  // that the call didn't throw.
  combine(other) {
    this._combined = other;
  }
}

exports.Holder = Holder;

class ErasedHolder {
  constructor(value) {
    this._value = value;
  }

  get() {
    return this._value;
  }
}

exports.ErasedHolder = ErasedHolder;

// Deliberately plain (non-generic) inspection helpers, so a bug in the
// generic per-monomorphisation path cannot also corrupt the assertions.
exports.holderValue = function (h) {
  return h._value;
};
exports.holderIsFromStatic = function (h) {
  return h._fromStatic;
};
exports.holderCombined = function (h) {
  return h._combined;
};

class LifetimeHolder {
  constructor(value) {
    this._value = value;
  }

  get() {
    return this._value;
  }
}

exports.LifetimeHolder = LifetimeHolder;

class LtHolder {
  constructor(value) {
    this._value = value;
  }

  get() {
    return this._value;
  }

  static of(value) {
    return new LtHolder(value);
  }
}

exports.LtHolder = LtHolder;

class Pair {
  constructor(k, v) {
    this._k = k;
    this._v = v;
  }

  get() {
    return this._v;
  }

  key() {
    return this._k;
  }

  // `pair_swap` on the Rust side declares its own generics in reversed order
  // (`<V, K>` against `&Pair<V, K>`), so its return type `K` names the same
  // *positional* slot (`_v`) as `get`'s `V` does. It's still the same
  // underlying field: the point is that a binding which let the reversed
  // declaration order desync `class_generic_params` from
  // `class_generic_exprs` would instead marshal `_k`'s value at the type the
  // caller expects for `_v`, producing a visibly wrong (or outright
  // unmarshallable) result.
  swap() {
    return this._v;
  }

  both() {
    return this._v;
  }
}

exports.Pair = Pair;

class Boxed {
  constructor(tag, value) {
    this._tag = tag;
    this._value = value;
  }

  tag() {
    return this._tag;
  }

  // The composed-argument (`Boxed<Option<T>>`) case: still just reads the tag,
  // proving the hoist through `Option<T>` compiles and runs, without also
  // depending on exactly how `Option<T>` itself is marshalled (that's covered
  // elsewhere, e.g. `tests/wasm/option.rs`).
  nestedGet() {
    return this._tag;
  }

  whereBound() {
    return this._tag;
  }

  // Reports the length of whatever it's handed, so a caller supplying a
  // projected `T::Item` can check the value round-tripped rather than just
  // that some string arrived.
  first(v) {
    return String(v).length;
  }
}

exports.Boxed = Boxed;

class Fallible {
  constructor(value, fail) {
    if (fail) {
      throw new Error("boom");
    }
    this._value = value;
  }

  get() {
    return this._value;
  }
}

exports.Fallible = Fallible;
