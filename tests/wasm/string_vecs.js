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
        // The glue rejects it itself under `wasm_bindgen_unstable_generic_vec`.
        assert.match(e.message, /array contains a value of the wrong type|expected a string argument/)
        assert.match(e.stack, /consume_string_vec/)
    }
};
