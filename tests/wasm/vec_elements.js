const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.echo = (v) => v;

exports.describe = (v) =>
    v === undefined ? 'undefined' : `${Array.isArray(v) ? 'Array' : v.constructor.name}:${JSON.stringify(v)}`;

exports.modesWithUnknown = () => ['read', 'nope'];

exports.throwingElementFreesBuffer = (memory) => {
    const chars = Array.from({ length: 100000 }, () => 'a');
    chars.push('🌍'.split('')[0]);
    const before = memory.buffer.byteLength;
    for (let i = 0; i < 100; i++) {
        assert.throws(() => wasm.vec_elements_next_chars(chars), /Unicode scalar value/);
    }
    // Each attempt allocates a 400 KB buffer: leaked, memory would grow by
    // 40 MB.
    assert.ok(memory.buffer.byteLength - before < 4 * 1024 * 1024);
};

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
