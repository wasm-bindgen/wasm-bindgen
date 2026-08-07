const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.return_null = () => null;

exports.return_undefined = () => undefined;

exports.return_number = () => 42;

exports.return_string = () => "hello";

exports.take_nullable_null = (val) => {
    assert.strictEqual(val, undefined, `expected undefined, got ${val}`);
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

exports.take_js_nullable_null = (val) => {
    assert.strictEqual(val, null, `expected null, got ${val}`);
};

exports.take_js_nullable_value = (val) => {
    assert.strictEqual(val, 321);
};

exports.test_js_nullable_exports = () => {
    // Rust JsNullable empty produces canonical `null`.
    const nullVal = wasm.rust_return_js_nullable_null();
    assert.strictEqual(nullVal, null,
        `expected null from rust_return_js_nullable_null, got ${nullVal}`);

    const numVal = wasm.rust_return_js_nullable_value();
    assert.strictEqual(numVal, 654);

    // Both null and undefined decode as empty.
    wasm.rust_take_js_nullable_empty(null);
    wasm.rust_take_js_nullable_empty(undefined);
    wasm.rust_take_js_nullable_value(987);
};

exports.test_nullable_exports = () => {
    // Test rust functions that return JsOption — strict: empty == undefined only.
    const nullVal = wasm.rust_return_nullable_null();
    assert.strictEqual(nullVal, undefined,
        `expected undefined from rust_return_nullable_null, got ${nullVal}`);

    const numVal = wasm.rust_return_nullable_value();
    assert.ok(numVal !== null && numVal !== undefined,
        `expected a value from rust_return_nullable_value, got ${numVal}`);
    assert.strictEqual(numVal, 456);

    // Test rust functions that take JsOption
    wasm.rust_take_nullable_null(undefined);
    wasm.rust_take_nullable_value(789);
};

exports.call_with_null_undefined_and_value = (f) => {
    f(null);
    f(undefined);
    f(321);
};
