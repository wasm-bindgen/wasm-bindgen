const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

const debug = process.env.WASM_BINDGEN_NO_DEBUG == null;
const notUndefined = /expected undefined, found/;

const show = (v) =>
    v === undefined ? 'undefined'
    : Array.isArray(v) ? `[${v.map(show)}]`
    : JSON.stringify(v);

const showArguments = (...args) => `${args.length}:${args.map(show)}`;
exports.showArguments = showArguments;

exports.Shown = class {
    constructor(...args) {
        this.made = showArguments(...args);
    }
    shown(...args) {
        return `${this.made}|${showArguments(...args)}`;
    }
    static shownStatic(...args) {
        return showArguments(...args);
    }
    set unit(unit) {
        this.made = showArguments(unit);
    }
};

exports.callWithUnit = (f) => f(1, undefined, 2);

exports.callWithWrongUnit = (f) => {
    assert.strictEqual(f.length, 3);
    // An argument is checked by a debug build only.
    for (const unit of [null, 0, 'x']) {
        if (debug) {
            assert.throws(() => f(1, unit, 2), notUndefined);
        } else {
            assert.strictEqual(f(1, unit, 2), 12);
        }
    }
};

exports.callReturning = (f) => show(f());

exports.resolvesToANumber = async () => 7;

exports.unitArgumentExports = () => {
    assert.strictEqual(wasm.unit_flat.length, 3);
    assert.strictEqual(wasm.unit_flat(1, undefined, 2), 12);
    assert.strictEqual(wasm.unit_only(undefined), 1);
    assert.strictEqual(wasm.unit_only(), 1);
    assert.strictEqual(wasm.unit_between(['a', 'b'], undefined, [undefined], undefined), 'a,b:1');
    assert.strictEqual(wasm.unit_between(['a'], undefined, []), 'a:0');

    // A JS value keeps its place beside one.
    const [lent, given] = [{}, {}];
    const values = wasm.unit_beside_values(undefined, lent, given, undefined);
    assert.ok(values[0] === lent && values[1] === given);
    assert.strictEqual(wasm.unit_beside_values.length, 4);

    const taker = new wasm.UnitTaker(undefined, 3);
    assert.strictEqual(taker.add(undefined, 4), 7);
    taker.unit = undefined;
    assert.strictEqual(taker.add(undefined, 4), 8);

    const field = new wasm.UnitField();
    field.unit = undefined;
    assert.strictEqual(field.unit, undefined);
    field.value = 5;
    assert.strictEqual(field.value, 5);

    // Only a debug build checks an argument.
    for (const unit of [null, 0, false, '', {}]) {
        if (debug) {
            assert.throws(() => wasm.unit_flat(1, unit, 2), notUndefined);
            assert.throws(() => wasm.unit_only(unit), notUndefined);
            assert.throws(() => wasm.unit_between(['a'], undefined, [], unit), notUndefined);
            assert.throws(() => taker.add(unit, 4), notUndefined);
            assert.throws(() => new wasm.UnitTaker(unit, 3), notUndefined);
            assert.throws(() => { taker.unit = unit; }, notUndefined);
            assert.throws(() => { field.unit = unit; }, notUndefined);
        } else {
            assert.strictEqual(wasm.unit_flat(1, unit, 2), 12);
            assert.strictEqual(wasm.unit_only(unit), 1);
            assert.strictEqual(wasm.unit_between(['a'], undefined, [], unit), 'a:0');
            assert.strictEqual(taker.add(unit, 4), 8);
            field.unit = unit;
            assert.strictEqual(field.unit, undefined);
        }
    }
    taker.free();
    field.free();
};
