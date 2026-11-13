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

exports.rejectedInstances = () => {
    const before = live();
    // An instance that was freed isn't one any more, which leaves the rest
    // with JS.
    const a = new wasm.Kept(1);
    const c = new wasm.Kept(3);
    a.free();
    assert.throws(() => wasm.rejected_points([a, c]), wrongType);
    assert.throws(() => wasm.rejected_maybe_points([c, a]), wrongType);
    assert.throws(() => wasm.rejected_nested_points([[c], [a]]), wrongType);
    assert.strictEqual(c.value, 3);

    // The same instance twice is only seen where it's taken a second time,
    // which frees the ones taken and leaves the rest with JS.
    const b = new wasm.Kept(2);
    assert.throws(() => wasm.rejected_points([b, b, c]), wrongType);
    assert.strictEqual(b.__wbg_ptr, 0);
    assert.strictEqual(c.value, 3);
    assert.strictEqual(live(), before + 1);
    const d = new wasm.Kept(4);
    assert.throws(() => wasm.rejected_nested_points([[d], [c, d]]), wrongType);
    assert.strictEqual(d.__wbg_ptr, 0);
    assert.strictEqual(c.__wbg_ptr, 0);
    assert.strictEqual(live(), before);
};

exports.rejectedByRust = () => {
    const before = live();
    // Only Rust knows an instance it has borrowed, and it's then handed
    // every element, dropping them all.
    for (const rejected of [
        (a, c) => wasm.rejected_points([a, c]),
        (a, c) => wasm.rejected_maybe_points([a, undefined, c]),
        (a, c) => wasm.rejected_nested_points([[a], [c]]),
    ]) {
        const a = new wasm.Kept(1);
        const c = new wasm.Kept(3);
        assert.throws(
            () => a.whileBorrowed(() => rejected(a, c)),
            /while it was borrowed/,
        );
        assert.strictEqual(a.__wbg_ptr, 0);
        assert.strictEqual(c.__wbg_ptr, 0);
        assert.strictEqual(live(), before);
    }
};

// A function that returns through the stack takes its space there, which a
// rejected value mustn't leave taken or give back untaken.
exports.rejectedKeepsTheStack = () => {
    const before = live();
    const a = new wasm.Kept(1);
    const b = new wasm.Kept(2);
    const freed = new wasm.Kept(3);
    freed.free();
    const stack = wasm.rejected_stack();
    for (let i = 0; i < 8; i++) {
        assert.throws(() => a.merge({}));
        assert.throws(() => a.mergeAll([b, {}], ['x']));
        assert.throws(() => a.mergeAll([b], ['x', 1]));
        assert.throws(() => a.mergeMaybe(1));
        assert.throws(() => a.label({}));
        assert.throws(() => wasm.Kept.join(a, ['x', 1]));
        assert.throws(() => wasm.rejected_named(a, [1]));
        assert.throws(() => wasm.rejected_named({}, ['x']));
        assert.throws(() => wasm.Kept.prototype.merge.call({}, b));
        // Only a debug build's glue knows the receiver was freed. Rust
        // otherwise throws from its own frames, which keep their stack.
        if (process.env.WASM_BINDGEN_NO_DEBUG == null) {
            assert.throws(() => freed.label(a));
            assert.throws(() => freed.merge(b));
        }
    }
    assert.strictEqual(wasm.rejected_stack(), stack);
    assert.strictEqual(a.value, 1);
    assert.strictEqual(b.value, 2);
    a.free();
    b.free();
    assert.strictEqual(live(), before);
};

// What holds the values is read again to take them, and may answer
// differently then: a value is checked where it's taken too.
exports.rejectedChangedValues = (memory) => {
    const before = live();
    const b = new wasm.Kept(2);
    const changed = (values, index, then) => {
        let reads = 0;
        return new Proxy(values, {
            get: (target, key) =>
                key === String(index) && reads++ > 0 ? then : target[key],
        });
    };
    for (const then of [42, ['x'], undefined, {}]) {
        assertFlat(memory, () => assert.throws(
            () => wasm.rejected_names_then_point(changed([big, big, 'c'], 2, then), b),
            wrongType,
        ));
        assertFlat(memory, () => assert.throws(
            () => wasm.rejected_nested_names_then_point(
                [[big], changed([big, 'c'], 1, then)], b),
            wrongType,
        ));
    }
    for (const then of [42, 'x', undefined]) {
        assertFlat(memory, () => assert.throws(
            () => wasm.rejected_nested_names_then_point(changed([[big], ['c']], 1, then), b),
            wrongType,
        ));
    }
    // An instance taken before the one that changed is Rust's by then, and
    // is freed.
    const a = new wasm.Kept(1);
    assert.throws(() => wasm.rejected_points_then_point(changed([a, b], 1, {}), b), wrongType);
    assert.strictEqual(a.__wbg_ptr, 0);
    assert.strictEqual(b.value, 2);
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

// Runs `f` with `object[name]` throwing on its `nth` call.
function failing(object, name, nth, f) {
    const original = object[name];
    let calls = 0;
    object[name] = function (...args) {
        if (++calls === nth) throw new Error('failed decode');
        return original.apply(this, args);
    };
    try {
        return f(() => { calls = 0; });
    } finally {
        object[name] = original;
    }
}

exports.failedDecodes = (memory) => {
    // A string that fails to decode releases the rest, and their buffer.
    failing(TextDecoder.prototype, 'decode', 2, reset => {
        assertFlat(memory, () => {
            reset();
            assert.throws(() => wasm.given_names(3), /failed decode/);
        });
    });
    failing(TextDecoder.prototype, 'decode', 4, reset => {
        assertFlat(memory, () => {
            reset();
            assert.throws(() => wasm.given_nested_names(3), /failed decode/);
        });
    });

    // A point that fails to wrap frees the ones not yet wrapped, and leaves
    // the wrapped ones to their wrappers.
    for (const given of [wasm.given_points, wasm.given_maybe_points]) {
        const before = live();
        const wrapped = [];
        const wrap = wasm.Kept.__wrap;
        failing(wasm.Kept, '__wrap', 3, () => {
            const failingWrap = wasm.Kept.__wrap;
            wasm.Kept.__wrap = function (ptr) {
                const point = failingWrap.call(this, ptr);
                wrapped.push(point);
                return point;
            };
            assert.throws(() => given(5), /failed decode/);
        });
        assert.strictEqual(wasm.Kept.__wrap, wrap);
        assert.deepStrictEqual(wrapped.map(point => point.value), [0, 1]);
        assert.strictEqual(live(), before + 2);
        wrapped.forEach(point => point.free());
        assert.strictEqual(live(), before);
    }

    // A handle that fails to load drops the rest. Without reference types
    // a handle is read off the JS heap, which can't fail.
    for (const given of [wasm.given_values, wasm.given_maybe_values]) {
        assert.deepStrictEqual(given(3), ['v0', 'v1', 'v2']);
        const before = wasm.rejected_handles();
        let failed = true;
        failing(WebAssembly.Table.prototype, 'get', 2, () => {
            try {
                given(3);
                failed = false;
            } catch (e) {
                assert.match(e.message, /failed decode/);
            }
        });
        assert.strictEqual(wasm.rejected_handles(), before);
        if (!failed) continue;
    }
};
