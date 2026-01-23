const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.pass_struct_vec = () => {
    const el1 = new wasm.ArrayElement();
    const el2 = new wasm.ArrayElement();
    const ret = wasm.consume_struct_vec([el1, el2]);
    assert.strictEqual(ret.length, 3);

    const ret2 = wasm.consume_optional_struct_vec(ret);
    assert.strictEqual(ret2.length, 4);

    assert.strictEqual(wasm.consume_optional_struct_vec(undefined), undefined);
};

exports.pass_invalid_struct_vec = () => {
    try {
        wasm.consume_struct_vec(['not a struct']);
    } catch (e) {
        assert.match(e.message, /array contains a value of the wrong type/)
        assert.match(e.stack, /consume_struct_vec/)
    }
};
