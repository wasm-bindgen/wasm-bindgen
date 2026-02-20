const strictEqual = require('assert').strictEqual;

exports.noop = function () { };

global.TestArrays = class {
  strings(x) {
    strictEqual(x, 'y');
    return 'x';
  }
  byteStrings(x) {
    strictEqual(x, 'yz');
    return 'xx';
  }
  usvStrings(x) {
    strictEqual(x, 'abc');
    return 'efg';
  }
  f32(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Float32Array([3, 4, 5]);
  }
  f64(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Float64Array([3, 4, 5]);
  }
  i8(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Int8Array([3, 4, 5]);
  }
  i16(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Int16Array([3, 4, 5]);
  }
  i32(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Int32Array([3, 4, 5]);
  }
  u8(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Uint8Array([3, 4, 5]);
  }
  u8Clamped(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Uint8ClampedArray([3, 4, 5]);
  }
  u16(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Uint16Array([3, 4, 5]);
  }
  u32(x) {
    strictEqual(x.length, 2);
    strictEqual(x[0], 1);
    strictEqual(x[1], 2);
    return new Uint32Array([3, 4, 5]);
  }
  get octetArray() {
    return new Uint8Array([3, 4, 5]);
  }

  get octetSequence() {
    return new Uint8Array([3, 4, 5]);
  }
  sumShorts(values) {
    return values.reduce((a, b) => a + b, 0);
  }
  async sumPromises(promises) {
    const results = await Promise.all(promises);
    return results.reduce((a, b) => a + b, 0);
  }
  getShorts() {
    return [1, 2, 3];
  }
  getPromises() {
    return [Promise.resolve(10), Promise.resolve(20), Promise.resolve(30)];
  }
  async getAsyncShorts() {
    return [4, 5, 6];
  }
  async getAsyncPromises() {
    return [Promise.resolve(100), Promise.resolve(200)];
  }
  async processShorts(input) {
    return input.map(x => x * 2);
  }
  async processPromises(input) {
    const results = await Promise.all(input);
    return results.map(x => Promise.resolve(x * 2));
  }
};

global.ArrayBufferTest = class {
  getBuffer() {
    return new ArrayBuffer(3);
  }
  setBuffer(x) {
    // ...
  }
  getDataView() {
    // taken from MDN
    const buffer = new ArrayBuffer(16);
    const view1 = new DataView(buffer);
    const view2 = new DataView(buffer, 12, 4);
    view1.setInt8(12, 42);

    return view2;
  }
};

global.TakeCallbackInterface = class {
  a() { }
  b() { }
};

global.assert_dict_c = function (c) {
  strictEqual(c.a, 1);
  strictEqual(c.b, 2);
  strictEqual(c.c, 3);
  strictEqual(c.d, 4);
  strictEqual(c.e, 5);
  strictEqual(c.f, 6);
  strictEqual(c.g, 7);
  strictEqual(c.h, 8);
};

global.mk_dict_a = function () {
  return {};
};

global.assert_dict_required = function (c) {
  strictEqual(c.a, 3);
  strictEqual(c.b, "a");
  strictEqual(c.c, 4);
};

global.assert_camel_case = function (dict) {
  strictEqual(dict.weird_fieldName, 1);
}

global.Shape = class Shape {
  constructor(kind) {
    this.kind = kind;
  }

  static triangle() {
    return new Shape('triangle');
  }

  isSquare() {
    return this.kind === 'square';
  }

  isCircle() {
    return this.kind === 'circle';
  }

  getShape() {
    return this.kind;
  }

  get shapeTypeNone() {
    return null;
  }

  get shapeTypeSome() {
    return this.kind;
  }
};

const map = {
  global_no_args: () => 3,
  global_with_args: (a, b) => a + b,
  global_attribute: 'x',
};

global.get_global = () => map;

global.TestReadOnlyMapLike = class {
  constructor() {
    this.map = new Map();
    this.map.set('a', 1);
    this.map.set('b', 2);
    this.map.set('c', 3);
  }

  entries() {
    return this.map.entries();
  }

  forEach(callback, thisArg) {
    return this.map.forEach(callback, thisArg);
  }

  get(key) {
    return this.map.get(key);
  }

  has(key) {
    return this.map.has(key);
  }

  keys() {
    return this.map.keys();
  }

  values() {
    return this.map.values();
  }

  get size() {
    return this.map.size;
  }
};

global.TestReadWriteMapLike = class extends global.TestReadOnlyMapLike {
  constructor() {
    super();
  }

  set(key, value) {
    this.map.set(key, value);
    return this;
  }

  delete(key) {
    return this.map.delete(key);
  }

  clear() {
    return this.map.clear();
  }
};

global.TestReadOnlySetLike = class {
  constructor() {
    this.set = new Set();
    this.set.add('a');
    this.set.add('b');
    this.set.add('c');
  }

  entries() {
    return this.set.entries();
  }

  forEach(callback, thisArg) {
    return this.set.forEach(callback, thisArg);
  }

  has(value) {
    return this.set.has(value);
  }

  keys() {
    return this.set.keys();
  }

  values() {
    return this.set.values();
  }

  get size() {
    return this.set.size;
  }
};

global.TestReadWriteSetLike = class extends global.TestReadOnlySetLike {
  constructor() {
    super();
  }

  add(value) {
    this.set.add(value);
    return this;
  }

  delete(value) {
    return this.set.delete(value);
  }

  clear() {
    return this.set.clear();
  }
};

global.math_test = {
  pow(base, exp) {
    return Math.pow(base, exp);
  },

  add_one(val) {
    return val + 1;
  },

  get PI() {
    return Math.PI;
  },
};

global.GetNoInterfaceObject = class {
  static get() {
    return {
      number: 3,
      foo: () => { },
    }
  }
};

global.Method = class Method {
  constructor(value) {
    this.value = value;
  }
  myCmp(other) {
    return this.value === other.value;
  }
};

global.Property = class Property {
  constructor(value) {
    this._value = value;
  }

  get value() {
    return this._value;
  }

  set value(value) {
    this._value = value;
  }
};

global.NamedConstructorParent = class NamedConstructor {
  constructor() {
    this._value = 0;
  }

  get value() {
    return this._value;
  }
};

global.NamedConstructor = class NamedConstructor extends NamedConstructorParent {
  constructor(_value) {
    super();
    this._value = _value;
  }
};

global.NamedConstructorBar = class NamedConstructorBar extends NamedConstructor {
  constructor(_value) {
    super();
    this._value = _value;
  }
};

global.StaticMethod = class StaticMethod {
  static swap(value) {
    const res = StaticMethod.value;
    StaticMethod.value = value;
    return res;
  }
};

StaticMethod.value = 0;

global.StaticProperty = class StaticProperty {
  static get value() {
    return StaticProperty._value;
  }

  static set value(value) {
    StaticProperty._value = value;
  }
};

StaticProperty._value = 0;

global.UndefinedMethod = class UndefinedMethod {
  constructor() { }
  ok_method() {
    return true;
  }
};

global.JsOptionMethod = class JsOptionMethod {
  constructor() { }
  opt(a) {
    if (a == undefined) {
      return undefined;
    } else {
      return a + 1;
    }
  }
};

global.Unforgeable = class Unforgeable {
  constructor() {
    this.uno = 1;
  }
  get dos() {
    return 2;
  }
};

global.GlobalMethod = class {
  m() { return 123; }
};

global.get_global_method = () => new GlobalMethod();

global.Indexing = function () {
  return new Proxy({}, {
    get(obj, prop) {
      return obj.hasOwnProperty(prop) ? obj[prop] : -1;
    },
    set(obj, prop, value) {
      obj[prop] = value;
    },
    deleteProperty(obj, prop) {
      delete obj[prop];
    },
  });
};

global.OptionalAndUnionArguments = class OptionalAndUnionArguments {
  constructor() { }
  m(a, b = true, c = 123, d = 456) {
    return [typeof a, a, typeof b, b, typeof c, c, typeof d, d].join(', ');
  }
};

global.Variadic = class Variadic {
  constructor() { }
  sum(...values) {
    return values.reduce((a, b) => a + b, 0);
  }
  countObjects(...items) {
    return items.length;
  }
};

global.PartialInterface = class PartialInterface {
  get un() {
    return 1;
  }
  deux() {
    return 2;
  }
  get trois() {
    return 3;
  }
  quatre() {
    return 4;
  }
};

global.MixinFoo = class MixinFoo {
  constructor(bar) {
    this._bar = bar | MixinFoo.defaultBar;
  }
  static get defaultBar() {
    return MixinFoo._defaultBar;
  }
  static set defaultBar(defaultBar) {
    MixinFoo._defaultBar = defaultBar;
  }
  get bar() {
    return this._bar;
  }
  addToBar(other) {
    this._bar += other;
  }
};

global.Overloads = class {
  foo() { }
};

global.InvokeCallback = class {
  invoke(f) { f(); }
  callAdd(f) {
    return f(1, 2);
  }
  callRepeat(f) {
    return f('ab', 4);
  }
};

global.Thang = class Thang {
  constructor(value) {
    if (value % 2 == 0) {
      throw new Error("only odd allowed");
    }
    this.value = value;
  }

  get ok_attr() { return this.value; }
  set ok_attr(x) { }

  get err_attr() { throw new Error("bad"); }
  set err_attr(x) { throw new Error("bad"); }

  ok_method() { return this.value + 1; }
  err_method() { throw new Error("bad"); }

  static ok_static_method() { return 1; }
  static err_static_method() { throw new Error("bad"); }

  static get ok_static_attr() { return 1; }
  static set ok_static_attr(x) { }

  static get err_static_attr() { throw new Error("bad"); }
  static set err_static_attr(x) { throw new Error("bad"); }
};

global.GetUnstableInterface = class {
  static get() {
    return {
      enum_value(dict) {
        if (!dict) {
          return 0;
        }

        return dict.unstableEnum === "a" ? 1 : 2;
      }
    }
  }
}

global.TestPromises = class {
  stringPromise() {
    return new Promise(r => r("abc"));
  }
  anyPromise() {
    return new Promise(r => r({ foo: "bar", num: 42 }));
  }
};

global.SignatureStability = class {
  process(options) {
    if (options === undefined) {
      return "stable";
    } else {
      return options.mode || "safe";
    }
  }
};

// Mixin override test classes
global.ImageDataLike = class ImageDataLike {
  constructor() {
    this.data = [];
  }
};

global.CanvasLike = class CanvasLike {
  constructor() {}
  
  // putImageData accepts both f64 and i32 for dx/dy in JavaScript
  putImageData(imageData, dx, dy) {
    // Return the type of dx to verify which binding was used
    return typeof dx === 'number' ? (Number.isInteger(dx) ? 'integer' : 'float') : 'unknown';
  }
};

global.Position = class Position {
  constructor() {}
};

global.GeolocationLike = class GeolocationLike {
  constructor() {}
  
  getCurrentPosition(successCallback) {
    // Just a stub
  }
};

global.TextureLike = class TextureLike {
  constructor() {}
};

global.UnstableFrame = class UnstableFrame {
  constructor() {}
};

global.WebGLLike = class WebGLLike {
  constructor() {}

  texUpload(texture, arg1, arg2) {
    if (typeof arg1 === 'number' && Number.isInteger(arg1)) return 'long';
    if (typeof arg1 === 'number') return 'double';
    if (typeof arg1 === 'string') return 'string';
    return 'frame';
  }
};

global.TestOptionalUnstableArg = class TestOptionalUnstableArg {
  constructor() {}
  
  read(options) {
    return Promise.resolve("data");
  }
};

global.TestSingleIterable = class {
  constructor() {
    this.items = ['item0', 'item1', 'item2'];
  }

  get length() {
    return this.items.length;
  }

  // Indexed property getter
  [Symbol.for('wasm-bindgen-indexed-getter')](index) {
    return this.items[index];
  }

  entries() {
    return this.items.values();
  }

  keys() {
    return this.items.keys();
  }

  values() {
    return this.items.values();
  }

  forEach(callback, thisArg) {
    return this.items.forEach(callback, thisArg);
  }
};

global.TestDoubleIterable = class {
  constructor() {
    this.data = new Map([
      ['key1', 10],
      ['key2', 20],
      ['key3', 30]
    ]);
  }

  entries() {
    return this.data.entries();
  }

  keys() {
    return this.data.keys();
  }

  values() {
    return this.data.values();
  }

  forEach(callback, thisArg) {
    return this.data.forEach(callback, thisArg);
  }
};

global.TestRecord = class {
  constructor() {
    this.lastSetRecord = null;
  }

  getNumberRecord() {
    return {
      a: 1,
      b: 2,
      c: 3
    };
  }

  getStringRecord() {
    return {
      x: 'hello',
      y: 'world',
      z: 'test'
    };
  }

  setRecord(data) {
    this.lastSetRecord = data;
  }
};

global.TestCallbacks = class {
  constructor() { }

  invokeVoidCallback(callback) {
    callback();
  }

  invokeNumberCallback(callback, value) {
    callback(value);
  }

  invokeStringTransformer(callback, input) {
    return callback(input);
  }

  invokeBinaryOp(callback, a, b) {
    return callback(a, b);
  }

  invokeObjectCallback(callback, data) {
    callback(data);
  }

  invokeSequenceCallback(callback, input) {
    return callback(input);
  }
};

// Upcast test classes
global.BaseType = class BaseType {
  constructor() {
    this.value = "";
  }
};

global.ChildType = class ChildType extends global.BaseType {
  constructor() {
    super();
    this.childValue = 0;
  }
};

global.GrandChildType = class GrandChildType extends global.ChildType {
  constructor() {
    super();
    this.grandChildValue = false;
  }
};

global.UpcastTest = class UpcastTest {
  static processBase(obj) {
    return obj.value;
  }

  static processChild(obj) {
    return obj.value;
  }
};
