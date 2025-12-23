export class TestValue {
  constructor(value) {
    this._value = value;
  }

  get value() {
    return this._value;
  }

  transform(suffix) {
    return new TestValue(this._value + suffix);
  }
}

export class TestResult {
  constructor(success, data) {
    this._success = success;
    this._data = data;
  }

  get success() {
    return this._success;
  }

  get data() {
    return this._data;
  }
}

export function createTestValuePromise(value) {
  return Promise.resolve(new TestValue(value));
}

export function createTestResultPromise(success, value) {
  return Promise.resolve(new TestResult(success, new TestValue(value)));
}

export function processTestValuePromise(promise) {
  return promise.then((val) => val.transform("_processed"));
}

export function chainTestValuePromises(promise1, promise2) {
  return Promise.all([promise1, promise2]).then(
    ([v1, v2]) => new TestValue(v1.value + "+" + v2.value)
  );
}

export async function checkTestValuePromise(promise) {
  const val = await promise;
  if (!(val instanceof TestValue)) {
    throw new Error("Expected TestValue instance");
  }
}

// Test functions for exported Rust functions with standard types
export async function testRustNumberPromise(rustModule) {
  const promise = rustModule.rust_create_number_promise(123.45);
  if (!(promise instanceof Promise)) {
    throw new Error("Expected Promise instance");
  }
  const result = await promise;
  if (typeof result !== 'number') {
    throw new Error(`Expected number, got ${typeof result}`);
  }
  if (result !== 123.45) {
    throw new Error(`Expected 123.45, got ${result}`);
  }
  return true;
}

export async function testRustStringPromise(rustModule) {
  const promise = rustModule.rust_create_string_promise("test_string");
  if (!(promise instanceof Promise)) {
    throw new Error("Expected Promise instance");
  }
  const result = await promise;
  if (typeof result !== 'string') {
    throw new Error(`Expected string, got ${typeof result}`);
  }
  if (result !== "test_string") {
    throw new Error(`Expected 'test_string', got '${result}'`);
  }
  return true;
}

export async function testRustDoubleNumberPromise(rustModule) {
  const inputPromise = Promise.resolve(50);
  const outputPromise = rustModule.rust_double_number_promise(inputPromise);
  if (!(outputPromise instanceof Promise)) {
    throw new Error("Expected Promise instance");
  }
  const result = await outputPromise;
  if (typeof result !== 'number') {
    throw new Error(`Expected number, got ${typeof result}`);
  }
  if (result !== 100) {
    throw new Error(`Expected 100, got ${result}`);
  }
  return true;
}
