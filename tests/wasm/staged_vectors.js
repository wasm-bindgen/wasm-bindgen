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


const live = () => wasm.StagedPoint.liveCount();

exports.pointRoundTrips = () => {
    const before = live();
    const a = new wasm.StagedPoint(1);
    const b = new wasm.StagedPoint(2);
    const points = wasm.staged_maybe_points([a, undefined, null, b]);
    // Rust took ownership of what was passed, and handed it back wrapped anew.
    assert.strictEqual(a.__wbg_ptr, 0);
    assert.strictEqual(b.__wbg_ptr, 0);
    assert.strictEqual(points.length, 4);
    assert.ok(points[0] instanceof wasm.StagedPoint);
    assert.strictEqual(points[0].value, 1);
    assert.deepStrictEqual(points.slice(1, 3), [undefined, undefined]);
    assert.strictEqual(points[3].value, 2);
    assert.strictEqual(live(), before + 2);

    const nullable = wasm.staged_nullable_points([points[0], null, points[3]]);
    assert.strictEqual(points[0].__wbg_ptr, 0);
    assert.deepStrictEqual(nullable.map(p => p && p.value), [1, null, 2]);
    assert.strictEqual(nullable[1], null);

    const both = wasm.staged_maybe_nullable_points([nullable[0], null, undefined, nullable[2]]);
    assert.strictEqual(nullable[2].__wbg_ptr, 0);
    assert.strictEqual(both.length, 4);
    assert.strictEqual(both[0].value, 1);
    assert.strictEqual(both[1], null);
    assert.strictEqual(both[2], undefined);
    assert.strictEqual(both[3].value, 2);
    assert.strictEqual(live(), before + 2);
    both[0].free();
    both[3].free();
    assert.strictEqual(live(), before);
};

exports.rejectedPoints = () => {
    const before = live();
    // A later element that throws frees the points encoded before it: Rust
    // was handed them, so they are no longer the caller's.
    const a = new wasm.StagedPoint(1);
    const b = new wasm.StagedPoint(2);
    assert.throws(() => wasm.staged_maybe_points([a, b, {}]), /expected instance of StagedPoint/);
    assert.strictEqual(a.__wbg_ptr, 0);
    assert.strictEqual(b.__wbg_ptr, 0);
    assert.strictEqual(live(), before);
    const c = new wasm.StagedPoint(3);
    assert.throws(() => wasm.staged_nullable_points([c, null, 1]));
    assert.strictEqual(c.__wbg_ptr, 0);
    assert.strictEqual(live(), before);
    const d = new wasm.StagedPoint(4);
    assert.throws(() => wasm.staged_maybe_nullable_points([d, undefined, 'x']));
    assert.strictEqual(d.__wbg_ptr, 0);
    assert.strictEqual(live(), before);
    // As does a later argument that throws.
    const e = new wasm.StagedPoint(5);
    assert.throws(() => wasm.staged_points_then_names([e, undefined], [Symbol()]));
    assert.strictEqual(e.__wbg_ptr, 0);
    assert.strictEqual(live(), before);
    // An element rejected before any point was taken leaves the caller's.
    const f = new wasm.StagedPoint(6);
    assert.throws(() => wasm.staged_maybe_points([{}, f]));
    assert.strictEqual(f.value, 6);
    f.free();
    assert.strictEqual(live(), before);
};

exports.jsMaybePoints = (v) => {
    assert.strictEqual(v.length, 3);
    assert.ok(v[0] instanceof wasm.StagedPoint);
    assert.strictEqual(v[0].value, 1);
    assert.strictEqual(v[1], undefined);
    assert.strictEqual(v[2].value, 2);
    return [v[2], undefined, v[0], new wasm.StagedPoint(3)];
};

exports.jsNullablePoints = (v) => {
    assert.strictEqual(v.length, 2);
    assert.strictEqual(v[0].value, 1);
    assert.strictEqual(v[1], null);
    return [null, v[0]];
};
