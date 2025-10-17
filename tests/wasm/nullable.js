const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.return_null = () => null;

exports.return_undefined = () => undefined;

exports.return_number = () => 42;

exports.return_string = () => "hello";

exports.take_nullable_null = (val) => {
    assert.ok(val === null || val === undefined, 
        `expected null or undefined, got ${val}`);
};

exports.take_nullable_value = (val) => {
    assert.ok(val !== null && val !== undefined,
        `expected a value, got ${val}`);
    assert.strictEqual(val, 123);
};

exports.take_nullable_number = (val) => {
    assert.ok(val !== null && val !== undefined,
        `expected a number, got ${val}`);
    assert.strictEqual(typeof val, 'number');
};

exports.take_nullable_string = (val) => {
    assert.ok(val !== null && val !== undefined,
        `expected a string, got ${val}`);
    assert.strictEqual(typeof val, 'string');
};

exports.test_nullable_exports = () => {
    // Test rust functions that return JsOption
    const nullVal = wasm.rust_return_nullable_null();
    assert.ok(nullVal === null || nullVal === undefined,
        `expected null or undefined from rust_return_nullable_null, got ${nullVal}`);

    const numVal = wasm.rust_return_nullable_value();
    assert.ok(numVal !== null && numVal !== undefined,
        `expected a value from rust_return_nullable_value, got ${numVal}`);
    assert.strictEqual(numVal, 456);

    // Test rust functions that take JsOption
    wasm.rust_take_nullable_null(null);
    wasm.rust_take_nullable_null(undefined);
    wasm.rust_take_nullable_value(789);
};
