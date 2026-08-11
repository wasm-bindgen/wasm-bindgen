class Widget {
  constructor(value) {
    this._value = value;
    // `kind` is read through a `final` getter, which captures the property
    // descriptor from `Widget.prototype` once rather than reading the property
    // off the receiver on every call. Derive it from the constructor argument so
    // a binding that passed the wrong receiver would produce a visibly wrong
    // string rather than a plausible one.
    this._kind = "widget:" + typeof value;
    this.tag = null;
    this.attached = null;
    this.received = [];
  }

  // Instance method. Records every call so the test can assert both the value
  // and the receiver, which is what distinguishes a correct method binding from
  // one that passed the receiver as the first argument.
  set(value) {
    this.received.push(value);
  }

  // Takes another `Widget` handle, so a wrong handle index surfaces as the
  // wrong object rather than as a type error.
  attach(other) {
    this.attached = other;
  }

  static of(value) {
    const w = new Widget(value);
    w._kind = "static:" + typeof value;
    return w;
  }

  get value() {
    return this._value;
  }

  set value(v) {
    this._value = v;
  }

  // Real accessor on the prototype: required for the `final` getter path, which
  // emits `GetOwnOrInheritedPropertyDescriptor(Widget.prototype, 'kind').get.call(obj)`.
  get kind() {
    return this._kind;
  }
}

exports.Widget = Widget;

// One JS class serving every `TypedCell<T>` instantiation: the class parameter
// is phantom on the JS side, while method arguments and returns cross at each
// monomorphisation's concrete type.
class TypedCell {
  constructor(value) {
    this._value = value;
  }

  get() {
    return this._value;
  }

  set(value) {
    this._value = value;
  }

  describeWith(label) {
    return label + ":" + typeof this._value + ":" + this._value;
  }
}

exports.TypedCell = TypedCell;

// Inspection helpers. These are deliberately plain (non-generic) imports so
// that a bug in the generic per-monomorphisation path cannot also corrupt the
// assertions themselves.
exports.widgetValue = function (w) {
  return w._value;
};

exports.widgetTag = function (w) {
  return w.tag;
};

exports.widgetReceived = function (w) {
  return w.received.join(",");
};

exports.widgetAttachedValue = function (w) {
  return w.attached === null ? "none" : String(w.attached._value);
};

exports.widgetHasProp = function (w, prop) {
  return Object.prototype.hasOwnProperty.call(w, prop);
};

exports.widgetSetProp = function (w, prop, value) {
  w[prop] = value;
};
