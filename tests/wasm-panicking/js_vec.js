const assert = require('assert');
const wasm = require('wasm-bindgen-test');

// Test if passing large arrays which cause allocation in Wasm are properly handled.
exports.pass_array_with_allocation = () => {
    const values = new Array(10_000).fill(1)
    assert.strictEqual(wasm.test_sum(values), 10_000);
};
