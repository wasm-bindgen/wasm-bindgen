const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

const big = 'a'.repeat(200 * 1024);
const MAX_GROWTH = 4 * 1024 * 1024;

function assertFlat(memory, f) {
    f();
    const before = memory.buffer.byteLength;
    for (let i = 0; i < 100; i++) f();
    assert.ok(memory.buffer.byteLength - before < MAX_GROWTH);
}

exports.roundTrips = (memory) => {
    const names = ['', 'a', 'é'.repeat(1000), big];
    assert.deepStrictEqual(wasm.staged_names(names), names);
    assert.deepStrictEqual(wasm.staged_names([]), []);
    assert.deepStrictEqual(
        wasm.staged_maybe_names(['a', undefined, null, 'b']),
        ['a', undefined, undefined, 'b'],
    );
    assert.strictEqual(wasm.staged_maybe_names(undefined), undefined);
    const o = {};
    const objs = wasm.staged_maybe_objs([o, undefined, 1, 'x']);
    assert.strictEqual(objs[0], o);
    assert.deepStrictEqual(objs.slice(1), [undefined, 1, 'x']);
    const nullable = wasm.staged_nullable_objs([o, null, undefined]);
    assert.strictEqual(nullable[0], o);
    assert.deepStrictEqual(nullable.slice(1), [null, undefined]);
    assert.deepStrictEqual(
        wasm.staged_maybe_bytes([1, undefined, 255, null]),
        [1, undefined, 255, undefined],
    );
    // Every handle that crossed was taken back.
    const live = wasm.staged_live_count();
    for (let i = 0; i < 100; i++) {
        wasm.staged_maybe_objs([{}, {}, undefined]);
        wasm.staged_nullable_objs([{}, null]);
    }
    assert.strictEqual(wasm.staged_live_count(), live);
    assertFlat(memory, () => wasm.staged_names([big, big]));
};

exports.rejectedElements = (memory) => {
    // A later element that throws releases the strings encoded before it.
    assertFlat(memory, () => assert.throws(() => wasm.staged_names([big, big, Symbol()])));
    assertFlat(memory, () =>
        assert.throws(() => wasm.staged_maybe_names([big, undefined, Symbol()])));
    // A later argument that throws releases the handles encoded before it.
    const live = wasm.staged_live_count();
    for (let i = 0; i < 100; i++) {
        assert.throws(() => wasm.staged_objs_then_names([{}, {}], [Symbol()]));
    }
    assert.strictEqual(wasm.staged_live_count(), live);
};

exports.jsNames = (v) => {
    assert.deepStrictEqual(v, ['a', 'é'.repeat(100)]);
    return [v[1], v[0], 'js'];
};

exports.jsMaybeObjs = (v) => {
    assert.deepStrictEqual(v, [1, undefined, null]);
    return [undefined, 1, null, 'js'];
};
