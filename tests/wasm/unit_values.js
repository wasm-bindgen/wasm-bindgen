const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

const wrongType = /array contains a value of the wrong type/;

// An element that was never written shows as a hole, which `undefined` isn't.
const show = (v) =>
    v === undefined ? 'undefined'
    : Array.isArray(v) ? `[${Array.from(v, (e, i) => (i in v ? show(e) : 'hole'))}]`
    : JSON.stringify(v);

// A list of `length` values that are each exactly `undefined`.
const assertUnits = (v, length) => {
    assert.ok(Array.isArray(v));
    assert.strictEqual(v.length, length);
    for (let i = 0; i < length; i++) {
        assert.ok(i in v);
        assert.strictEqual(v[i], undefined);
    }
};

exports.echo = (v) => v;
exports.show = show;

exports.unitsOf = (kind) => ({
    units: [undefined, undefined],
    empty: [],
    holes: new Array(3),
    null: [undefined, null],
    number: [0],
    nested: [[undefined], []],
    'nested null': [[undefined], [null]],
}[kind]);

exports.callWithUnits = (f) => show(f([undefined, undefined], 7));

exports.callWithWrongUnits = (f) => {
    assertUnits(f([], 7), 0);
    assert.throws(() => f([undefined, null], 7), wrongType);
    assert.throws(() => f([1], 7), wrongType);
};

exports.unitExports = () => {
    assertUnits(wasm.unit_vec([undefined, undefined]), 2);
    assertUnits(wasm.unit_vec([]), 0);
    // A hole reads as `undefined`, and is given back as one.
    assertUnits(wasm.unit_vec(new Array(3)), 3);
    assertUnits(wasm.unit_vec([, undefined, ,]), 3);
    assertUnits(wasm.unit_boxed([undefined]), 1);
    assertUnits(wasm.unit_give(5), 5);
    assertUnits(wasm.unit_give(0), 0);
    assertUnits(wasm.unit_value(4), 4);
    assertUnits(wasm.unit_value(0), 0);
    assertUnits(wasm.unit_give(100000), 100000);
    assert.strictEqual(wasm.unit_len(new Array(1 << 20)), 1 << 20);
    assert.strictEqual(wasm.unit_len(wasm.unit_give(1 << 16)), 1 << 16);

    // An empty list is a list, and only an absent one is none.
    assertUnits(wasm.unit_opt([undefined]), 1);
    assertUnits(wasm.unit_opt([]), 0);
    assert.strictEqual(wasm.unit_opt(undefined), undefined);
    assert.strictEqual(wasm.unit_opt(null), undefined);
    assert.strictEqual(wasm.unit_opt_len([undefined, undefined]), 2);
    assert.strictEqual(wasm.unit_opt_len([]), 0);
    assert.strictEqual(wasm.unit_opt_len(undefined), -1);
    assert.strictEqual(wasm.unit_opt_len(null), -1);
    assertUnits(wasm.unit_nullable([undefined]), 1);
    assertUnits(wasm.unit_nullable([]), 0);
    assert.strictEqual(wasm.unit_nullable(undefined), undefined);
    assert.strictEqual(wasm.unit_nullable(null), null);

    const nested = wasm.unit_nested([[undefined, undefined], [], [undefined]]);
    assert.strictEqual(nested.length, 3);
    assertUnits(nested[0], 2);
    assertUnits(nested[1], 0);
    assertUnits(nested[2], 1);
    assert.deepStrictEqual(wasm.unit_nested([]), []);

    assert.strictEqual(wasm.unit_beside(['a', 'b'], [undefined], []), 'a,b:1:0');
    assert.throws(() => wasm.unit_beside(['a'], [], [null]), wrongType);

    const record = new wasm.UnitRecord([undefined, undefined]);
    assertUnits(record.units, 2);
    assertUnits(record.nested, 0);
    assert.strictEqual(record.maybe, undefined);
    record.units = [undefined];
    record.nested = [[undefined], []];
    record.maybe = [];
    assertUnits(record.units, 1);
    assertUnits(record.nested[0], 1);
    assertUnits(record.nested[1], 0);
    assertUnits(record.maybe, 0);
    assert.throws(() => { record.units = [null]; }, wrongType);
    assert.throws(() => { record.nested = [[undefined], [0]]; }, wrongType);
    assertUnits(record.units, 1);
    assert.strictEqual(record.count([undefined, undefined]), 3);
    assert.throws(() => record.count([null]), wrongType);
    assert.throws(() => new wasm.UnitRecord([0]), wrongType);
    record.free();
};

exports.unitAsyncExports = async () => {
    assertUnits(await wasm.unit_async_give(3), 3);
    assertUnits(await wasm.unit_async_give(0), 0);
    assert.strictEqual(await wasm.unit_async_take([undefined, undefined], 7), 27);
    assert.throws(() => wasm.unit_async_take([undefined, null], 7), wrongType);
};
