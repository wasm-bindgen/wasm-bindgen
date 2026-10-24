const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

// `Kind:[values]`, where `Kind` is `Array` or the typed array's constructor.
const describe = (v) => {
    if (v === undefined) return 'undefined';
    const kind = Array.isArray(v) ? 'Array' : v.constructor.name;
    return `${kind}:${JSON.stringify(Array.from(v))}`;
};

exports.describe = describe;
exports.echo = (v) => v;
exports.callNullable = (f) => {
    f(null);
    f('a');
};
exports.nullableArray = () => ['a', null];
exports.nullableMap = () => new Map([['a', 'b'], ['c', null]]);

exports.test_exports = () => {
    assert.deepStrictEqual(wasm.export_echo_vec(['a', null]), ['a', null]);
    assert.deepStrictEqual(wasm.export_echo_opt_vec(['a', undefined]), ['a', undefined]);
    assert.strictEqual(wasm.export_null_count(['a', null, null]), 2);
};
