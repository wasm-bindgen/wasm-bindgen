const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.echo = (v) => v;
exports.second = (first, v) => {
    first.free();
    return v;
};
exports.count = (...args) => args.length;
exports.fail = () => {
    throw new Error('failed');
};
exports.sum = (v) => (v ? Array.from(v).reduce((a, b) => a + b, 0) : 0);
exports.fill = (v) => v.fill(7);
exports.join = (v) => v.map(String).join(',');
exports.call = (f, v) => f(v, ['a', 'b'], new Uint8Array([1, 2, 3]));

exports.exports = () => {
    const values = [1, 'two', null, { a: 1 }];
    assert.deepStrictEqual(wasm.high_values(values), values);
    assert.deepStrictEqual(wasm.high_opt_values(values), values);
    assert.strictEqual(wasm.high_opt_values(undefined), undefined);

    const names = ['a', '', 'cde'];
    assert.deepStrictEqual(wasm.high_strings(names), names);
    assert.deepStrictEqual(wasm.high_opt_strings(names), names);
    assert.strictEqual(wasm.high_opt_strings(undefined), undefined);

    const nested = [new Uint8Array([1, 2, 3]), new Uint8Array(), new Uint8Array([4])];
    assert.deepStrictEqual(wasm.high_nested(nested), nested);
    const optional = [names, undefined, []];
    assert.deepStrictEqual(wasm.high_nested_strings(optional), optional);

    const items = wasm.high_items([new wasm.HighItem(1), new wasm.HighItem(2)]);
    assert.deepStrictEqual(items.map((item) => item.id), [1, 2]);
    items.forEach((item) => item.free());

    assert.deepStrictEqual(wasm.high_bytes(new Uint8Array([1, 2, 3])), new Uint8Array([1, 2, 3]));
    const bytes = new Uint8Array(4);
    wasm.high_fill(bytes);
    assert.deepStrictEqual(bytes, new Uint8Array(4).fill(7));
    assert.strictEqual(wasm.high_text('ab', 'cd'), 'abcd');

    const record = new wasm.HighRecord();
    record.names = names;
    record.values = values;
    record.bytes = bytes;
    assert.deepStrictEqual(record.names, names);
    assert.deepStrictEqual(record.values, values);
    assert.deepStrictEqual(record.bytes, bytes);
    record.free();

    const item = new wasm.HighItem(3);
    assert.throws(() => wasm.high_rejects(names, [values, values], {}));
    assert.throws(() => wasm.high_rejects(names, [values, 1], item));
    assert.throws(() => wasm.high_rejects([1], [values], item));
    item.free();
};
