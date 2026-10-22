const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.nullable_type_describe = v =>
    v === null ? 'null' : v === undefined ? 'undefined' : 'value';

exports.nullable_type_js_null = () => null;
exports.nullable_type_js_undefined = () => undefined;
exports.nullable_type_js_number = () => 7;
exports.nullable_type_js_float = () => 2.5;
exports.nullable_type_js_bigint = () => 7n;
exports.nullable_type_js_true = () => true;
exports.nullable_type_js_string = () => 'hi';
exports.nullable_type_js_bytes = () => new Uint8Array([1, 2, 3]);
exports.nullable_type_js_bools = () => [true, false];
exports.nullable_type_js_values = () => [1, 'a'];
exports.nullable_type_js_beta = () => 'beta';
exports.nullable_type_js_thing = () => new wasm.NullableThing(5);
exports.nullable_type_js_object = () => ({});

// Each export is the identity on a `Nullable<T>`: the three states must come
// back as themselves, `null` and `undefined` checked strictly.
const roundtrip = (f, value, eq = assert.strictEqual) => {
    assert.strictEqual(f(null), null);
    assert.strictEqual(f(undefined), undefined);
    assert.strictEqual(f(), undefined);
    eq(f(value), value);
};

exports.nullable_type_js_exports = () => {
    for (const f of [
        wasm.nullable_type_u8, wasm.nullable_type_i32, wasm.nullable_type_u32,
        wasm.nullable_type_usize, wasm.nullable_type_nonnull,
    ]) {
        roundtrip(f, 3);
    }
    roundtrip(wasm.nullable_type_f32, 1.5);
    roundtrip(wasm.nullable_type_f64, 1.5);
    roundtrip(wasm.nullable_type_i64, -3n);
    roundtrip(wasm.nullable_type_u128, 3n);
    roundtrip(wasm.nullable_type_bool, true);
    roundtrip(wasm.nullable_type_bool, false);
    roundtrip(wasm.nullable_type_char, 'x');
    roundtrip(wasm.nullable_type_string, 'hi');
    roundtrip(wasm.nullable_type_string, '');
    roundtrip(wasm.nullable_type_vec_u8, new Uint8Array([1, 2]), assert.deepStrictEqual);
    roundtrip(wasm.nullable_type_vec_bool, [true, false], assert.deepStrictEqual);
    roundtrip(wasm.nullable_type_vec_jsvalue, [1, 'a'], assert.deepStrictEqual);
    roundtrip(wasm.nullable_type_enum, wasm.NullableColor.Blue);
    roundtrip(wasm.nullable_type_string_enum, 'beta');
    roundtrip(wasm.nullable_type_jsvalue, {}, (a, b) => assert.strictEqual(a, b));
    roundtrip(wasm.nullable_type_js_type, {}, (a, b) => assert.strictEqual(a, b));

    assert.strictEqual(wasm.nullable_type_struct(null), null);
    assert.strictEqual(wasm.nullable_type_struct(undefined), undefined);
    assert.strictEqual(wasm.nullable_type_struct(new wasm.NullableThing(4)).n, 4);

    assert.strictEqual(wasm.nullable_type_string_enum_name(null), 'null');
    assert.strictEqual(wasm.nullable_type_string_enum_name(undefined), 'undefined');
    assert.strictEqual(wasm.nullable_type_string_enum_name('alpha'), 'alpha');
    assert.strictEqual(wasm.nullable_type_string_enum_name('gamma'), 'invalid');

    const fields = new wasm.NullableFields();
    assert.strictEqual(fields.n, null);
    assert.strictEqual(fields.s, null);
    fields.n = 2;
    fields.s = 'x';
    assert.strictEqual(fields.n, 2);
    assert.strictEqual(fields.s, 'x');
    fields.n = undefined;
    fields.s = undefined;
    assert.strictEqual(fields.n, undefined);
    assert.strictEqual(fields.s, undefined);
    fields.n = null;
    fields.s = null;
    assert.strictEqual(fields.n, null);
    assert.strictEqual(fields.s, null);
};

exports.nullable_type_closure_driver = f => {
    assert.strictEqual(f(null, 'a'), null);
    assert.strictEqual(f(undefined, 'a'), undefined);
    assert.strictEqual(f(1, null), '1 Null');
    assert.strictEqual(f(1, undefined), '1 Undefined');
    assert.strictEqual(f(1, 'a'), '1 Value("a")');
};
