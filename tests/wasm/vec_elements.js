const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.echo = (v) => v;

exports.describe = (v) =>
    v === undefined ? 'undefined' : `${Array.isArray(v) ? 'Array' : v.constructor.name}:${JSON.stringify(v)}`;

exports.modesWithUnknown = () => ['read', 'nope'];

exports.callExports = () => {
    assert.deepStrictEqual(wasm.vec_elements_not([true, false, true]), [false, true, false]);
    assert.deepStrictEqual(wasm.vec_elements_not([]), []);
    assert.deepStrictEqual(wasm.vec_elements_next_chars(['a', 'ü', '🌍']), ['b', 'ý', '🌎']);
    assert.deepStrictEqual(wasm.vec_elements_flip_modes(['read', 'write']), ['write', 'read']);
    assert.deepStrictEqual(wasm.vec_elements_flip_modes([]), []);
    assert.strictEqual(wasm.vec_elements_flip_modes(undefined), undefined);
    assert.deepStrictEqual(wasm.vec_elements_strs(), ['a', '', 'ü🌍']);

    const many = Array.from({ length: 100000 }, (_, i) => i % 3 === 0);
    assert.deepStrictEqual(
        wasm.vec_elements_not(many),
        many.map((b) => !b),
    );
};
