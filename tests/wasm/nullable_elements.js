const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

// `Kind:[values]`, where `Kind` is `Array` or the typed array's constructor.
// An `undefined` element is spelled out, since `JSON.stringify` would write
// it as `null`.
const describe = (v) => {
    if (v === undefined) return 'undefined';
    const kind = Array.isArray(v) ? 'Array' : v.constructor.name;
    const json = JSON.stringify(Array.from(v), (_, x) => x === undefined ? 'undefined' : x);
    return `${kind}:${json}`;
};

exports.describe = describe;
exports.echo = (v) => v;
exports.callNullable = (f) => {
    f(null);
    f('a');
};
exports.nullableArray = () => ['a', null];
// `Opaque` has no JS class, so there is nothing to test `instanceof` against.
exports.opaqueOpt = () => [{ a: 1 }, null, undefined];
exports.opaqueNullable = () => [{ a: 1 }, null];
exports.opaqueOptNullable = () => [{ a: 1 }, null, undefined];
exports.nullableMap = () => new Map([['a', 'b'], ['c', null]]);

exports.test_exports = () => {
    assert.deepStrictEqual(wasm.export_echo_vec(['a', null]), ['a', null]);
    assert.deepStrictEqual(wasm.export_echo_opt_vec(['a', undefined]), ['a', undefined]);
    assert.deepStrictEqual(wasm.export_echo_opt_js_vec(['a', undefined]), ['a', undefined]);
    // An `Option<T>` element reads `null` as `None`, as a bare one does.
    assert.deepStrictEqual(wasm.export_echo_opt_vec(['a', null, undefined]), ['a', undefined, undefined]);
    assert.deepStrictEqual(wasm.export_echo_opt_js_vec(['a', null, undefined]), ['a', undefined, undefined]);
    assert.strictEqual(wasm.export_null_count(['a', null, null]), 2);
};
