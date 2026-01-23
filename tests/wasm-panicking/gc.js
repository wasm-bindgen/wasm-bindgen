const wasm = require("wasm-bindgen-test.js");
const assert = require("assert");

async function gc() {
  if ("gc" in global) {
    global.gc();
    // Give the `FinalizationRegistry` callbacks a chance to run.
    await new Promise((resolve) => setTimeout(resolve, 0));
  } else {
    console.warn("test runner doesn't expose GC function");
  }
}

let dropCount = 0;
exports.drop_callback = () => dropCount += 1;

exports.owned_methods = async () => {
  dropCount = 0;
  new wasm.OwnedValue(1).add(new wasm.OwnedValue(2)).n();

  // The `OwnedValue`s should have been dropped as a result of `add` and `n`
  // taking ownership of `self`...
  assert.strictEqual(dropCount, 3);
  await gc();
  // ... and GCing shouldn't double-free them.
  assert.strictEqual(dropCount, 3);
};

// Make sure that objects created via. builders get GC'd properly.
exports.gc_builder = async () => {
  dropCount = 0;
  wasm.OwnedValue.build(1);

  await gc();
  assert.strictEqual(dropCount, 1);
};

// Make sure that objects created via. constructors get GC'd properly.
exports.gc_constructor = async () => {
  dropCount = 0;
  new wasm.OwnedValue(1);

  await gc();
  assert.strictEqual(dropCount, 1);
};

// Make sure that exported Rust types don't get GC'd while they're still in use
// by an async function.
exports.no_gc_fn_argument = async () => {
  dropCount = 0;
  let resolve;

  // It seems like we have to create the `OwnedValue` inside another function in
  // order for the GC to see it as unused.
  const createPromise = () =>
    wasm.borrow_and_wait(
      new wasm.OwnedValue(1),
      new Promise((x) => resolve = x),
    );
  const promise = createPromise();

  await gc();
  assert.strictEqual(dropCount, 0);

  resolve();
  await promise;
  await gc();
  assert.strictEqual(dropCount, 1);
};

exports.no_gc_method_receiver = async () => {
  dropCount = 0;
  let resolve;

  const createPromise = () =>
    new wasm.OwnedValue(1).borrow_and_wait(
      new Promise((x) => resolve = x),
    );
  const promise = createPromise();

  await gc();
  assert.strictEqual(dropCount, 0);

  resolve();
  await promise;
  await gc();
  assert.strictEqual(dropCount, 1);
};
