const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

// The same object per id, so a round trip compares by identity.
const things = new Map();
exports.thing = id => {
    if (!things.has(id)) things.set(id, { thing: id });
    return things.get(id);
};

const show = v =>
    v === undefined ? 'undefined'
    : typeof v === 'bigint' ? `${v}n`
    : ArrayBuffer.isView(v) ? `${v.constructor.name}[${Array.from(v, e => show(e))}]`
    : Array.isArray(v) ? `[${v.map(e => show(e))}]`
    : v instanceof wasm.MatrixItem ? `Item(${v.id})`
    : JSON.stringify(v);

const empty = view => `${view.slice(0, view.indexOf('[') + 1)}]`;

exports.show = show;
exports.reversed = v => v === undefined ? undefined : v.slice().reverse();

exports.exportRet = (row, view) => {
    const give = wasm[`em_${row}_give`];
    assert.strictEqual(show(give(0)), view);
    assert.strictEqual(show(give(2)), empty(view));
};

exports.exportArg = row => {
    const give = wasm[`em_${row}_give`];
    const take = wasm[`em_${row}_take`];
    take(give(0), 0);
    take(give(0).reverse(), 1);
    take(give(2), 2);
    take([], 2);
};

// A value no row takes, after values the row does: the call throws, and
// what came before the rejected value is still the caller's to pass.
exports.exportBad = row => {
    const give = wasm[`em_${row}_give`];
    const take = wasm[`em_${row}_take`];
    const good = give(0);
    assert.throws(() => take([...good, Symbol()], 0));
    take(good, 0);
};

exports.exportOption = (row, view) => {
    const opt = wasm[`em_${row}_opt`];
    const given = opt(undefined, 0);
    assert.strictEqual(show(given), view);
    assert.strictEqual(opt(given, 0), undefined);
    assert.strictEqual(show(opt(undefined, 2)), empty(view));
    assert.strictEqual(opt([], 2), undefined);
    assert.strictEqual(opt(undefined, 3), undefined);
};
