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

// Each element is checked as a single enum argument would be; only debug
// glue checks the type before the value.
exports.pass_invalid_enum_vec = () => {
    const debug = process.env.WASM_BINDGEN_NO_DEBUG == null;
    for (const [vec, message] of [
        [['not an enum value'], debug ? /expected a number argument/ : /invalid enum value passed/],
        [[1.5], /invalid enum value passed/],
        [[NaN], /invalid enum value passed/],
    ]) {
        let threw = false;
        try {
            wasm.consume_enum_vec(vec);
        } catch (e) {
            threw = true;
            assert.match(e.message, message);
            assert.match(e.stack, /consume_enum_vec/);
        }
        assert.ok(threw, `expected consume_enum_vec to throw on ${vec}`);
    }
};
