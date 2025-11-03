const assert = require('assert');

exports.PI = 3.14159;

exports.add = function add(a, b) {
    return a + b;
};

exports.multiply = function multiply(a, b) {
    return a * b;
};

exports.test_reexports_exist = function test_reexports_exist(wasm) {
    assert.strictEqual(typeof wasm.customAdd, 'function');
    assert.strictEqual(typeof wasm.multiply, 'function');
    assert.strictEqual(wasm.PI_VALUE, PI);
    assert.strictEqual(wasm.customAdd(10, 20), 30);
    assert.strictEqual(wasm.multiply(3, 7), 21);
};
