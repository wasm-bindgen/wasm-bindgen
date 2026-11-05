const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.i64_js_identity = a => a;
exports.u64_js_identity = a => a;

exports.js_works = () => {
    assert.strictEqual(wasm.zero(), BigInt('0'));
    assert.strictEqual(wasm.one(), BigInt('1'));
    assert.strictEqual(wasm.neg_one(), BigInt('-1'));
    assert.strictEqual(wasm.i32_min(), BigInt('-2147483648'));
    assert.strictEqual(wasm.u32_max(), BigInt('4294967295'));
    assert.strictEqual(wasm.i64_min(), BigInt('-9223372036854775808'));
    assert.strictEqual(wasm.u64_max(), BigInt('18446744073709551615'));

    const i64_min = BigInt('-9223372036854775808');
    const u64_max = BigInt('18446744073709551615');

    const identityTestI64Values = [
        BigInt('0'),
        BigInt('1'),
        BigInt('-1'),
        i64_min,
    ];
    for (const value of identityTestI64Values) {
        assert.strictEqual(wasm.i64_rust_identity(value), value);
        assert.strictEqual(wasm.i64_jsvalue_identity(value), value);
    }

    const identityTestU64Values = [
        BigInt('0'),
        BigInt('1'),
        u64_max,
    ];
    for (const value of identityTestU64Values) {
        assert.strictEqual(wasm.u64_rust_identity(value), value);
        assert.strictEqual(wasm.u64_jsvalue_identity(value), value);
    }

    assert.strictEqual(wasm.u64_rust_identity(BigInt('1') << BigInt('64')), BigInt('0'));
    assert.strictEqual(wasm.i128_min_jsvalue(), BigInt('-170141183460469231731687303715884105728'));
    assert.strictEqual(wasm.u128_max_jsvalue(), BigInt('340282366920938463463374607431768211455'));

    assert.deepStrictEqual(wasm.u64_slice([]), new BigUint64Array());
    assert.deepStrictEqual(wasm.i64_slice([]), new BigInt64Array());
    const arr1 = new BigUint64Array([BigInt('1'), BigInt('2')]);
    assert.deepStrictEqual(wasm.u64_slice([BigInt('1'), BigInt('2')]), arr1);
    const arr2 = new BigInt64Array([BigInt('1'), BigInt('2')]);
    assert.deepStrictEqual(wasm.i64_slice([BigInt('1'), BigInt('2')]), arr2);

    assert.deepStrictEqual(wasm.i64_slice([i64_min]), new BigInt64Array([i64_min]));
    assert.deepStrictEqual(wasm.u64_slice([u64_max]), new BigUint64Array([u64_max]));
};

exports.safe_u64_js_identity = a => {
    assert.strictEqual(typeof a, 'number');
    return a;
};
exports.safe_i64_js_identity = a => {
    assert.strictEqual(typeof a, 'number');
    return a;
};
exports.safe_opt_js_identity = a => a;
exports.safe_vec_js_identity = a => {
    assert.ok(a instanceof Float64Array);
    return a;
};
exports.safe_is_number = a => a === Number.MAX_SAFE_INTEGER;
exports.safe_catch = ok => {
    if (!ok) throw new Error('not ok');
    return Number.MAX_SAFE_INTEGER;
};
exports.safe_call = f => f(-21);
exports.safe_returns = n => n;
exports.safe_call_bad = f => {
    try {
        f(1.5);
        return 'no throw';
    } catch (e) {
        return e.message;
    }
};

exports.safe_js_rejects = () => {
    const unsigned = /expected a non-negative safe integer/;
    const signed = /expected a safe integer/;
    for (const bad of [2 ** 53, 1.5, -1, NaN, Infinity]) {
        assert.throws(() => wasm.safe_u64_rust_identity(bad), unsigned);
        assert.throws(() => wasm.safe_returns_rust(bad), unsigned);
    }
    assert.strictEqual(wasm.safe_u64_rust_identity(Number.MAX_SAFE_INTEGER), Number.MAX_SAFE_INTEGER);
    assert.throws(() => wasm.safe_i64_rust_identity(-(2 ** 53)), signed);
    assert.throws(() => wasm.safe_opt_rust_identity(0.5), signed);
    assert.throws(() => wasm.safe_slice_doubled([1, 1.5]), unsigned);
    // The instance is still usable after each rejection.
    assert.deepStrictEqual(wasm.safe_slice_doubled([1, 2]), new Float64Array([2, 4]));
};

exports.safe_js_works = () => {
    for (const value of [0, 1, Number.MAX_SAFE_INTEGER]) {
        assert.strictEqual(wasm.safe_u64_rust_identity(value), value);
    }
    for (const value of [0, -1, Number.MIN_SAFE_INTEGER, Number.MAX_SAFE_INTEGER]) {
        assert.strictEqual(wasm.safe_i64_rust_identity(value), value);
    }
    assert.strictEqual(wasm.safe_opt_rust_identity(Number.MAX_SAFE_INTEGER), Number.MAX_SAFE_INTEGER);
    assert.strictEqual(wasm.safe_opt_rust_identity(-7), -7);
    assert.strictEqual(wasm.safe_opt_rust_identity(undefined), undefined);
    assert.deepStrictEqual(wasm.safe_slice_doubled([1, 2, 2 ** 51]), new Float64Array([2, 4, 2 ** 52]));
    assert.deepStrictEqual(wasm.safe_slice_doubled([]), new Float64Array());
    assert.strictEqual(wasm.safe_catch_export(true), Number.MAX_SAFE_INTEGER);
    assert.throws(() => wasm.safe_catch_export(false), /not ok/);
};
