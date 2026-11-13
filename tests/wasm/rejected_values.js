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

const live = () => wasm.Kept.liveCount();
const wrongType = /array contains a value of the wrong type/;

exports.rejectedArguments = (memory) => {
    const before = live();
    const a = new wasm.Kept(1);
    const b = new wasm.Kept(2);
    const kept = () => {
        assert.strictEqual(a.value, 1);
        assert.strictEqual(b.value, 2);
        assert.strictEqual(live(), before + 2);
    };

    // An element that is rejected leaves the ones before it with JS.
    assert.throws(() => wasm.rejected_points([a, b, {}]), wrongType);
    kept();
    assert.throws(() => wasm.rejected_maybe_points([a, undefined, b, 'x']), /expected instance of Kept/);
    kept();
    assert.throws(() => wasm.rejected_nested_points([[a], [], [b, 1]]), wrongType);
    kept();

    // As does an argument, or an element of one, after an argument.
    assert.throws(() => wasm.rejected_point_then_point(a, {}), /expected instance of Kept/);
    kept();
    assert.throws(() => wasm.rejected_point_then_maybe_point(a, 1), /expected instance of Kept/);
    kept();
    assert.throws(() => wasm.rejected_point_then_names(a, ['x', Symbol()]), wrongType);
    kept();
    assert.throws(() => wasm.rejected_point_then_nested(a, [['x'], ['y', 1]]), wrongType);
    kept();
    assert.throws(() => a.consumeWith(['x', 1]), wrongType);
    kept();

    // A sequence held in a sequence is one.
    assert.throws(() => wasm.rejected_point_then_nested(a, [['x'], 5]), wrongType);
    assert.throws(() => wasm.rejected_point_then_nested(a, [['x'], 'yz']), wrongType);
    assert.throws(() => wasm.rejected_point_then_nested(a, [undefined]), wrongType);
    kept();
    assert.throws(() => wasm.rejected_maybe_nested([['x'], {}]), wrongType);
    assert.strictEqual(wasm.rejected_maybe_nested([['x'], undefined, null]), 3);
    assert.throws(() => wasm.rejected_nested_numbers([[1], 2]), wrongType);
    assert.strictEqual(wasm.rejected_nested_numbers([[1], new Uint32Array(2), []]), 3);

    // Nothing was allocated for the values ahead of the one rejected.
    assertFlat(memory, () =>
        assert.throws(() => wasm.rejected_names_then_point([big, big], {})));
    assertFlat(memory, () =>
        assert.throws(() => wasm.rejected_point_then_nested(a, [[big, big], [big, 1]])));
    kept();

    a.free();
    b.free();
    assert.strictEqual(live(), before);
};

let returned = [];

exports.jsBadPoints = () => {
    returned = [new wasm.Kept(1), new wasm.Kept(2)];
    return [...returned, {}];
};

exports.jsBadNestedPoints = () => {
    returned = [new wasm.Kept(1), new wasm.Kept(2)];
    return [[returned[0]], [returned[1], 1]];
};

exports.jsFreeKept = () => {
    let sum = 0;
    for (const point of returned) {
        sum += point.value;
        point.free();
    }
    returned = [];
    return sum;
};
