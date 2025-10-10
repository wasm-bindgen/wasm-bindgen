const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.pass_enum_vec = () => {
    const el1 = wasm.EnumArrayElement.Unit;
    const el2 = wasm.EnumArrayElement.Unit;
    const ret = wasm.consume_enum_vec([el1, el2]);
    assert.strictEqual(ret.length, 3);

    const ret2 = wasm.consume_optional_enum_vec(ret);
    assert.strictEqual(ret2.length, 4);

    assert.strictEqual(wasm.consume_optional_enum_vec(undefined), undefined);
};

exports.pass_invalid_enum_vec = () => {
    let threw = false;
    try {
        wasm.consume_enum_vec(['not an enum value']);
    } catch (e) {
        threw = true;
        assert.match(e.message, /array contains a value of the wrong type/);
        assert.match(e.stack, /consume_enum_vec/);
    }
    assert.ok(threw, 'expected consume_enum_vec to throw on a non-numeric element');
};
