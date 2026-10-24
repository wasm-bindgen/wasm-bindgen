const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

// Both JS empties are `None`; `None` comes back as `undefined`.
const checkEmpties = (f, none = undefined) => {
    assert.strictEqual(f(undefined), none);
    assert.strictEqual(f(null), none);
    assert.strictEqual(f(), none);
};

exports.option_null_js_exports = () => {
    for (const f of [wasm.option_null_i32, wasm.option_null_u8, wasm.option_null_f64]) {
        checkEmpties(f);
        assert.strictEqual(f(3), 3);
    }

    for (const f of [wasm.option_null_i64, wasm.option_null_u128]) {
        checkEmpties(f);
        assert.strictEqual(f(3n), 3n);
    }

    checkEmpties(wasm.option_null_nonnull, false);
    assert.strictEqual(wasm.option_null_nonnull(8), true);

    checkEmpties(wasm.option_null_bool);
    assert.strictEqual(wasm.option_null_bool(true), true);

    checkEmpties(wasm.option_null_char);
    assert.strictEqual(wasm.option_null_char('x'), 'x');

    checkEmpties(wasm.option_null_string);
    assert.strictEqual(wasm.option_null_string('hi'), 'hi');

    checkEmpties(wasm.option_null_vec_u8);
    assert.strictEqual(wasm.option_null_vec_u8(new Uint8Array([1, 2])), 2);
    checkEmpties(wasm.option_null_vec_jsvalue);
    assert.strictEqual(wasm.option_null_vec_jsvalue([1, 'a']), 2);

    checkEmpties(wasm.option_null_enum);
    assert.strictEqual(wasm.option_null_enum(wasm.NullColor.Blue), wasm.NullColor.Blue);

    checkEmpties(wasm.option_null_string_enum, 'none');
    assert.strictEqual(wasm.option_null_string_enum('beta'), 'beta');
    assert.strictEqual(wasm.option_null_string_enum('gamma'), 'invalid');

    checkEmpties(wasm.option_null_struct);
    assert.strictEqual(wasm.option_null_struct(new wasm.NullThing(4)), 4);

    const fields = new wasm.NullFields();
    assert.strictEqual(fields.n, undefined);
    fields.n = 2;
    assert.strictEqual(fields.n, 2);
    fields.n = null;
    assert.strictEqual(fields.n, undefined);
    fields.s = 'x';
    assert.strictEqual(fields.s, 'x');
    fields.s = null;
    assert.strictEqual(fields.s, undefined);

    for (const f of [wasm.option_null_jsvalue, wasm.option_null_js_type]) {
        checkEmpties(f, 'none');
        assert.strictEqual(f({}), 'some');
    }
};

exports.option_null_closure_driver = f => {
    assert.strictEqual(f(undefined, undefined), 'None None');
    assert.strictEqual(f(null, null), 'None None');
    assert.strictEqual(f(1, null), 'Some(1) None');
    assert.strictEqual(f(null, 'a'), 'None Some("a")');
};

exports.option_null_js_null = () => null;
exports.option_null_js_value = () => 7;
exports.option_null_js_string = () => 'hi';
exports.option_null_js_thing = () => new wasm.NullThing(5);
