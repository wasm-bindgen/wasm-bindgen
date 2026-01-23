const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.pass_string_vec = () => {
    assert.deepStrictEqual(
        wasm.consume_string_vec(["hello", "world"]),
        ["hello", "world", "Hello from Rust!"],
    );
    assert.deepStrictEqual(
        wasm.consume_optional_string_vec(["hello", "world"]),
        ["hello", "world", "Hello from Rust!"],
    );
    assert.strictEqual(wasm.consume_optional_string_vec(undefined), undefined);
};

exports.pass_invalid_string_vec = () => {
    try {
        wasm.consume_string_vec([42]);
    } catch (e) {
        assert.match(e.message, /array contains a value of the wrong type/)
        assert.match(e.stack, /consume_string_vec/)
    }
};
