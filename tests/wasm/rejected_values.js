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

    // An instance without a pointer isn't one.
    const empty = Object.create(wasm.Kept.prototype);
    assert.throws(() => wasm.rejected_points([a, empty, b]), wrongType);
    assert.throws(() => wasm.rejected_maybe_points([a, empty, b]), wrongType);
    assert.throws(() => wasm.rejected_nested_points([[a], [empty, b]]), wrongType);
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

    // A character is checked ahead of the points being taken.
    for (const c of ['\uD800', 5, {}]) {
        assert.throws(() => wasm.rejected_points_then_chars([a], b, ['x', c]));
        assert.throws(() => wasm.rejected_points_then_maybe_chars([a], [undefined, c]));
        kept();
    }
    assert.throws(() => wasm.rejected_points_then_chars([a], b, [undefined]));
    kept();

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

// A debug build's glue knows an instance that an argument ahead of it has
// taken, where it comes to take it again.
exports.rejectedMoved = () => {
    const before = live();
    const moved = /Attempt to use a moved value/;
    let a = new wasm.Kept(1);
    let b = new wasm.Kept(2);
    assert.strictEqual(a.eat(b, new wasm.Kept(3)), 3);
    a = new wasm.Kept(1);
    b = new wasm.Kept(2);
    if (process.env.WASM_BINDGEN_NO_DEBUG != null) {
        // Rust is otherwise given it as no instance.
        assert.strictEqual(a.eat(b, b), 2);
        assert.strictEqual(live(), before);
        return;
    }
    assert.throws(() => a.eat(b, b), moved);
    assert.strictEqual(a.__wbg_ptr, 0);
    assert.strictEqual(b.__wbg_ptr, 0);
    assert.strictEqual(live(), before);

    a = new wasm.Kept(1);
    assert.throws(() => a.eat(a, null), moved);
    assert.strictEqual(live(), before);
    a = new wasm.Kept(1);
    assert.throws(() => a.mergeMaybe(a), moved);
    assert.strictEqual(live(), before);
    a = new wasm.Kept(1);
    assert.throws(() => wasm.rejected_point_then_point(a, a), moved);
    assert.throws(() => wasm.rejected_point_then_maybe_point(a, a), moved);
    assert.strictEqual(live(), before);
    a = new wasm.Kept(1);
    assert.throws(() => wasm.rejected_points_then_point([a], a), moved);
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

// A debug build's glue rejects what isn't an array, where a string would
// otherwise be taken as its characters and a number as no values.
exports.rejectedNotArrays = () => {
    if (process.env.WASM_BINDGEN_NO_DEBUG != null) return;
    const notArray = /expected an array, found/;
    for (const v of ['ab', 5, {}, true]) {
        assert.throws(() => wasm.rejected_names(v), notArray);
        assert.throws(() => wasm.rejected_boxed_names(v), notArray);
        assert.throws(() => wasm.rejected_maybe_names(v), notArray);
        assert.throws(() => wasm.rejected_points(v), notArray);
    }
    assert.throws(() => wasm.rejected_names(null), notArray);
    assert.strictEqual(wasm.rejected_maybe_names(undefined), 0);
    assert.strictEqual(wasm.rejected_maybe_names(null), 0);
    assert.strictEqual(wasm.rejected_maybe_names(['a']), 1);
    assert.strictEqual(wasm.rejected_returned_names(), true);
};

// A sequence of `char`, `bool` or an enum has never asserted an array: it
// takes whatever has a `length` as its elements, and anything else as none.
exports.takenNotArrays = () => {
    assert.deepStrictEqual(wasm.rejected_chars('ab'), ['a', 'b']);
    assert.deepStrictEqual(wasm.rejected_maybe_chars('ab'), ['a', 'b']);
    assert.deepStrictEqual(wasm.rejected_chars({ length: 2, 0: 'a', 1: 'b' }), ['a', 'b']);
    assert.deepStrictEqual(wasm.rejected_chars(new Set(['a'])), []);
    assert.deepStrictEqual(wasm.rejected_bools({ length: 2, 0: true, 1: false }), [true, false]);
    assert.deepStrictEqual(wasm.rejected_bools(new Set([true])), []);
    assert.deepStrictEqual(wasm.rejected_levels({ length: 2, 0: 1, 1: 2 }), [1, 2]);
    assert.deepStrictEqual(wasm.rejected_levels(new Set([1])), []);
    assert.deepStrictEqual(wasm.rejected_level_names('ab'), ['a', 'b']);
    assert.deepStrictEqual(wasm.rejected_level_names(new Set(['a'])), []);
};

// A sequence whose cells come to more bytes than can be allocated is
// rejected before anything is taken, where the size would otherwise wrap to
// one that can.
exports.rejectedSizes = () => {
    const tooBig = /invalid malloc request/;
    const a = new wasm.Kept(1);
    for (const length of [2 ** 28, 2 ** 29, 2 ** 32 - 1]) {
        const huge = new Array(length);
        assert.throws(() => wasm.rejected_maybe_strings(huge), tooBig);
        assert.throws(() => wasm.rejected_maybe_maybe_strings(huge), tooBig);
        assert.throws(() => wasm.rejected_nested_maybe_strings([['a'], huge]), tooBig);
        assert.throws(() => wasm.rejected_named(a, huge), tooBig);
        assert.strictEqual(a.value, 1);
    }
    a.free();
    assert.strictEqual(wasm.rejected_maybe_strings(['a', undefined]), 2);
    assert.strictEqual(wasm.rejected_maybe_maybe_strings(['a', undefined]), 2);
    assert.strictEqual(wasm.rejected_nested_maybe_strings([['a'], [undefined]]), 2);
};

// `values`, answering `then` for their length once each was read and the
// reading stopped at the length they have.
function grown(values, then) {
    let read = 0;
    return new Proxy(values, {
        get(target, key) {
            if (key === 'length') {
                if (read > 1) return then;
                if (read) read++;
                return target.length;
            }
            if (key === String(target.length - 1)) read = read || 1;
            if (Number(key) >= target.length) throw new Error('read past the values');
            return target[key];
        },
    });
}

// The length is read again where the values are taken, and what it is then
// is what is allocated for.
exports.rejectedChangedLengths = (memory) => {
    const tooBig = /invalid malloc request/;
    const before = live();
    for (const then of [2 ** 28, 2 ** 29, 2 ** 32 - 1]) {
        assertFlat(memory, () => assert.throws(
            () => wasm.rejected_maybe_strings(grown([big, undefined], then)),
            tooBig,
        ));
        assertFlat(memory, () => assert.throws(
            () => wasm.rejected_maybe_maybe_strings(grown([big, undefined], then)),
            tooBig,
        ));
        assertFlat(memory, () => assert.throws(
            () => wasm.rejected_nested_maybe_strings([[big], grown([big, undefined], then)]),
            tooBig,
        ));
        // What was taken ahead of them is Rust's by then, and is freed.
        const a = new wasm.Kept(1);
        assert.throws(() => wasm.rejected_named(a, grown(['x'], then)), tooBig);
        assert.strictEqual(a.__wbg_ptr, 0);
        assert.strictEqual(live(), before);
    }
    // An array an earlier argument's getter lengthens, the second time it's
    // read.
    assertFlat(memory, () => {
        const numbers = [1n, undefined];
        const names = [big];
        let reads = 0;
        Object.defineProperty(names, 0, {
            get() {
                if (++reads === 2) numbers.length = 2 ** 28;
                return big;
            },
        });
        assert.throws(() => wasm.rejected_names_then_maybe_numbers(names, numbers), tooBig);
    });
    assert.strictEqual(wasm.rejected_returned_long_names(), true);
    // A length that isn't a whole number takes the values below it.
    assert.strictEqual(wasm.rejected_maybe_strings({ length: 1.5, 0: 'a', 1: 'b' }), 1);
    assert.throws(() => wasm.rejected_maybe_strings({ length: -1, 0: 'a' }), tooBig);
};

exports.jsLongNames = () => grown(['a', undefined], 2 ** 29);

function seeded(a) {
    return () => {
        a = (a + 0x6D2B79F5) | 0;
        let t = Math.imul(a ^ (a >>> 15), 1 | a);
        t = (t + Math.imul(t ^ (t >>> 7), 61 | t)) ^ t;
        return ((t ^ (t >>> 14)) >>> 0) / 2 ** 32;
    };
}

// Each call's sequences answer through a `Proxy`, and on one read of one of
// them something of the call changes: an element, a length, an instance's
// pointer or its class. However far the call then got, every instance is
// either still with JS or freed, and nothing else is left allocated.
exports.rejectedChanging = (memory) => {
    const calls = [
        ['rejected_names', () => [[big, 'b', 'c']]],
        ['rejected_boxed_names', () => [[big, 'b']]],
        ['rejected_maybe_names', () => [[big, 'b']]],
        ['rejected_maybe_strings', () => [[big, undefined, 'c']]],
        ['rejected_maybe_maybe_strings', () => [[big, undefined]]],
        ['rejected_nested_maybe_strings', () => [[[big], [undefined, 'c']]]],
        ['rejected_maybe_nested', () => [[[big], undefined, ['c']]]],
        ['rejected_points', k => [[k(), k(), k()]]],
        ['rejected_maybe_points', k => [[k(), undefined, k()]]],
        ['rejected_nested_points', k => [[[k()], [k(), k()]]]],
        ['rejected_chars', () => [['a', 'b']]],
        ['rejected_maybe_chars', () => [['a', 'b']]],
        ['rejected_bools', () => [[true, false]]],
        ['rejected_levels', () => [[1, 2]]],
        ['rejected_level_names', () => [['a', 'b']]],
        ['rejected_names_then_maybe_numbers', () => [[big], [1n, undefined]]],
        ['rejected_names_then_point', (k, flat) => [[big, 'b'], flat()]],
        ['rejected_nested_names_then_point', (k, flat) => [[[big], ['b']], flat()]],
        ['rejected_points_then_point', (k, flat) => [[k(), k()], flat()]],
        ['rejected_point_then_names', (k, flat) => [flat(), [big, 'b']]],
        ['rejected_point_then_nested', (k, flat) => [flat(), [[big], ['b']]]],
        ['rejected_points_then_chars', (k, flat) => [[k()], flat(), ['a', 'b']]],
        ['rejected_points_then_maybe_chars', k => [[k(), k()], [undefined, 'a']]],
        ['rejected_named', (k, flat) => [flat(), [big]]],
    ];
    const random = seeded(1);
    const pick = values => values[Math.floor(random() * values.length)];
    const before = live();
    let settled;
    for (let i = 0; i < 3000; i++) {
        if (i === 300) settled = memory.buffer.byteLength;
        // The instances of sequences, and every instance of the call.
        const stored = [];
        const all = [];
        const instance = () => {
            const k = new wasm.Kept(all.length);
            all.push(k);
            return k;
        };
        const sequences = [];
        const wrap = value => {
            if (!Array.isArray(value)) return value;
            const sequence = { target: value.map(wrap), reads: new Map(), answers: new Map() };
            sequences.push(sequence);
            return new Proxy(sequence.target, {
                get(target, key) {
                    if (typeof key !== 'string') return target[key];
                    const nth = (sequence.reads.get(key) || 0) + 1;
                    sequence.reads.set(key, nth);
                    if (sequence === change.on && key === change.key && nth === change.nth) {
                        change.make();
                    }
                    if (sequence.answers.has(key)) return sequence.answers.get(key);
                    // A length that never ends is only read so far.
                    if (Number(key) >= target.length + 8) throw new Error('read past the values');
                    return target[key];
                },
            });
        };
        const [name, make] = pick(calls);
        const args = make(
            () => {
                const k = instance();
                stored.push(k);
                return k;
            },
            instance,
        ).map(wrap);

        const on = pick(sequences);
        const of = pick(sequences);
        const length = of.target.length;
        const index = String(Math.floor(random() * (length + 1)));
        const changes = {
            element: () => of.answers.set(index, pick([
                42, 'x', 1n, true, undefined, null, {}, ['x'], Symbol(), big,
                { __wbg_ptr: 1 }, Object.create(wasm.Kept.prototype),
                stored.length ? pick(stored) : instance(),
            ])),
            length: () => of.answers.set('length', pick([
                0, length - 1, length + 1, length + 3, length + 0.5, 1.5, -1,
                2 ** 31, 2 ** 32 - 1,
            ])),
            shorter: () => { of.target.length = Math.floor(random() * length); },
            pointer: () => {
                const k = stored.length ? pick(stored) : instance();
                if (k.__wbg_ptr) wasm.Kept.prototype.free.call(k);
            },
            class: () => Object.setPrototypeOf(pick(all.length ? all : [instance()]), Object.prototype),
        };
        const kind = pick(Object.keys(changes));
        const change = {
            on,
            key: random() < 0.3 ? 'length' : String(Math.floor(random() * on.target.length)),
            nth: 1 + Math.floor(random() * 3),
            make: changes[kind],
        };
        const what = `${i}: ${name}, ${kind} on read ${change.nth} of ${change.key}`;

        try {
            wasm[name](...args);
        } catch (e) {
            // Neither a trap nor a write outside the memory.
            assert.ok(!(e instanceof WebAssembly.RuntimeError || e instanceof RangeError), `${what}: ${e}`);
        }
        const held = all.filter(k => k.__wbg_ptr !== 0);
        assert.strictEqual(live(), before + held.length, what);
        for (const k of held) {
            Object.setPrototypeOf(k, wasm.Kept.prototype);
            assert.strictEqual(typeof k.value, 'number', what);
            k.free();
        }
        assert.strictEqual(live(), before, what);
    }
    assert.ok(memory.buffer.byteLength - settled < MAX_GROWTH);
};

exports.jsNotNames = () => 'notarray';

exports.jsTakesMixed = (points) => points.forEach(point => point.free());

let returned = [];

exports.jsTakesOwned = (a, b) => {
    a.free();
    b.free();
};

exports.jsBadNames = () => ['a', 5];

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

    // An argument that fails to decode releases the ones after it too.
    assert.strictEqual(wasm.given_mixed(), false);
    failing(wasm.Kept, '__wrap', 1, reset => {
        const before = live();
        const handles = wasm.rejected_handles();
        assertFlat(memory, () => {
            reset();
            assert.strictEqual(wasm.given_mixed(), true);
        });
        assert.strictEqual(live(), before);
        assert.strictEqual(wasm.rejected_handles(), handles);
    });

    // So are the flat arguments beside a sequence that fails to decode,
    // which nothing has wrapped or taken by then.
    failing(TextDecoder.prototype, 'decode', 1, reset => {
        const before = live();
        const handles = wasm.rejected_handles();
        assertFlat(memory, () => {
            reset();
            assert.strictEqual(wasm.given_owned(), true);
        });
        assert.strictEqual(live(), before);
        assert.strictEqual(wasm.rejected_handles(), handles);
    });

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
