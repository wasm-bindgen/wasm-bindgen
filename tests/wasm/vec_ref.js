const assert = require('assert');

// `&Vec<T>` outgoing arguments must arrive on the JS side as plain
// `Array` instances (not typed arrays) for primitive element kinds.
// For non-primitive kinds (string / externref) the JS-visible type was
// already a plain `Array` under the existing wire format.

exports.js_take_vec_ref_u8 = (v) => {
    assert.ok(Array.isArray(v), `expected Array, got ${Object.prototype.toString.call(v)}`);
    assert.ok(!(v instanceof Uint8Array), 'should not be a Uint8Array');
    assert.deepStrictEqual(v, [1, 2, 3]);
};

exports.js_take_vec_ref_u16 = (v) => {
    assert.ok(Array.isArray(v));
    assert.ok(!(v instanceof Uint16Array));
    assert.deepStrictEqual(v, [10, 20, 30]);
};

exports.js_take_vec_ref_i32 = (v) => {
    assert.ok(Array.isArray(v));
    assert.ok(!(v instanceof Int32Array));
    assert.deepStrictEqual(v, [-1, 0, 1]);
};

exports.js_take_vec_ref_f64 = (v) => {
    assert.ok(Array.isArray(v));
    assert.ok(!(v instanceof Float64Array));
    assert.deepStrictEqual(v, [1.5, 2.5, 3.5]);
};

exports.js_take_vec_ref_string = (v) => {
    assert.ok(Array.isArray(v));
    assert.deepStrictEqual(v, ['hello', 'world']);
};

exports.js_take_vec_ref_optional_u16 = (v) => {
    if (v === undefined) {
        return;
    }
    assert.ok(Array.isArray(v));
    assert.ok(!(v instanceof Uint16Array));
    assert.deepStrictEqual(v, [5, 6, 7]);
};

// Verifies an exported Rust function declaring `Vec<u16>` accepts both
// a plain JS `Array<number>` and a `Uint16Array` from JS callers. The
// codegen path uses `TypedArray.prototype.set(arrayLike, offset)` which
// coerces array-likes element-by-element, so plain arrays work as
// input. The incoming-side ABI remains unchanged from existing
// `Vec<u16>` behavior; this is just a regression test.
exports.js_drive_vec_u16_from_array = () => {
    const wasm = require('wasm-bindgen-test.js');
    // Plain JS Array as input.
    assert.strictEqual(wasm.rust_consume_vec_u16([1, 2, 3]), 6);
    // Typed array as input (existing supported case).
    assert.strictEqual(wasm.rust_consume_vec_u16(new Uint16Array([4, 5, 6])), 15);
    // Empty plain array.
    assert.strictEqual(wasm.rust_consume_vec_u16([]), 0);
    // Iterable / array-like with length.
    assert.strictEqual(wasm.rust_consume_vec_u16(Array.of(10, 20, 30)), 60);
};
