const assert = require('assert');

// Each handler asserts the incoming value is a plain `Array` (not a
// typed array), then returns a transformed plain `Array` which Rust
// receives as `Vec<T>`. The return-side coercion uses
// `TypedArray.prototype.set`, which accepts plain arrays for primitive
// element kinds, exercising both directions in a single call.

exports.js_roundtrip_vec_ref_u8 = (v) => {
    assert.ok(Array.isArray(v), `expected Array, got ${Object.prototype.toString.call(v)}`);
    assert.ok(!(v instanceof Uint8Array), 'should not be a Uint8Array');
    assert.deepStrictEqual(v, [1, 2, 3]);
    return v.map(x => x * 2);
};

exports.js_roundtrip_vec_ref_u16 = (v) => {
    assert.ok(Array.isArray(v));
    assert.ok(!(v instanceof Uint16Array));
    assert.deepStrictEqual(v, [10, 20, 30]);
    return v.map(x => x * 2);
};

exports.js_roundtrip_vec_ref_i32 = (v) => {
    assert.ok(Array.isArray(v));
    assert.ok(!(v instanceof Int32Array));
    assert.deepStrictEqual(v, [-1, 0, 1]);
    return v.map(x => x * 2);
};

exports.js_roundtrip_vec_ref_f64 = (v) => {
    assert.ok(Array.isArray(v));
    assert.ok(!(v instanceof Float64Array));
    assert.deepStrictEqual(v, [1.5, 2.5, 3.5]);
    return v.map(x => x * 2);
};

exports.js_roundtrip_vec_ref_string = (v) => {
    assert.ok(Array.isArray(v));
    assert.deepStrictEqual(v, ['hello', 'world']);
    return v.map(s => s + '!');
};

exports.js_roundtrip_vec_ref_optional_u16 = (v) => {
    if (v === undefined) {
        return undefined;
    }
    assert.ok(Array.isArray(v));
    assert.ok(!(v instanceof Uint16Array));
    assert.deepStrictEqual(v, [5, 6, 7]);
    return v.map(x => x * 2);
};
