const assert = require('assert');

// Imported class used by the `&[ImportedType]` test.
exports.Tagged = class Tagged {
    constructor(tag) {
        this._tag = tag;
    }
    get tag() {
        return this._tag;
    }
};

// Each handler asserts the incoming value is a plain `Array` (not a
// typed array) and returns a transformed plain `Array` which Rust
// receives as `Vec<T>`. The return-side coercion uses
// `TypedArray.prototype.set`, which accepts plain arrays for primitive
// element kinds, exercising both directions in a single call.

exports.js_st2a_u8 = (v) => {
    assert.ok(Array.isArray(v), `expected Array, got ${Object.prototype.toString.call(v)}`);
    assert.ok(!(v instanceof Uint8Array), 'should not be a Uint8Array');
    assert.deepStrictEqual(v, [1, 2, 3]);
    return v.map(x => x * 2);
};

exports.js_st2a_u16 = (v) => {
    assert.ok(Array.isArray(v));
    assert.ok(!(v instanceof Uint16Array));
    assert.deepStrictEqual(v, [10, 20, 30]);
    return v.map(x => x * 2);
};

exports.js_st2a_i32 = (v) => {
    assert.ok(Array.isArray(v));
    assert.ok(!(v instanceof Int32Array));
    assert.deepStrictEqual(v, [-1, 0, 1]);
    return v.map(x => x * 2);
};

exports.js_st2a_f64 = (v) => {
    assert.ok(Array.isArray(v));
    assert.ok(!(v instanceof Float64Array));
    assert.deepStrictEqual(v, [1.5, 2.5, 3.5]);
    return v.map(x => x * 2);
};

exports.js_st2a_string = (v) => {
    assert.ok(Array.isArray(v));
    assert.deepStrictEqual(v, ['hello', 'world']);
    return v.map(s => s + '!');
};

exports.js_st2a_imported = (v) => {
    assert.ok(Array.isArray(v));
    return v.map(t => t.tag);
};

exports.js_st2a_optional_u16 = (v) => {
    if (v === undefined) {
        return undefined;
    }
    assert.ok(Array.isArray(v));
    assert.ok(!(v instanceof Uint16Array));
    assert.deepStrictEqual(v, [5, 6, 7]);
    return v.map(x => x * 2);
};

// Block-level `slice_to_array` test handlers.
exports.js_st2a_block_a = (v) => {
    assert.ok(Array.isArray(v));
    assert.ok(!(v instanceof Uint16Array));
    assert.deepStrictEqual(v, [1, 2, 3]);
    return v.map(x => x * 2);
};

exports.js_st2a_block_b = (v) => {
    assert.ok(Array.isArray(v));
    assert.deepStrictEqual(v, ['x', 'y']);
    return v.map(s => s + '!');
};
