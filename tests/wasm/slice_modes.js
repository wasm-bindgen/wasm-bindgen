const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

// `:fresh` means the value owns its entire backing buffer (a copy), `:view`
// means it aliases a larger buffer (wasm linear memory).
const describe = (v) => {
    if (v === undefined) return 'undefined';
    if (Array.isArray(v)) return 'Array';
    return `${v.constructor.name}:${v.buffer.byteLength === v.byteLength ? 'fresh' : 'view'}`;
};

exports.kindOf = (v) => v === undefined ? 'undefined' : v.constructor.name;

exports.kindsOf = (...args) => args.map(describe).join(',');

exports.isWasmMemoryView = (v) => v.buffer.byteLength > v.byteLength;

exports.joined = (v) => v === undefined ? 'undefined' : `${describe(v)}:${Array.from(v).join(',')}`;

exports.js_copy_u8 = (v) => {
    assert.strictEqual(describe(v), 'Uint8Array:fresh');
    assert.deepStrictEqual(Array.from(v), [1, 2, 3]);
    return v.map(x => x * 2);
};

exports.js_copy_f64 = (v) => {
    assert.strictEqual(describe(v), 'Float64Array:fresh');
    assert.deepStrictEqual(Array.from(v), [1.5, 2.5]);
    return v.map(x => x * 2);
};

exports.js_copy_i64 = (v) => {
    assert.strictEqual(describe(v), 'BigInt64Array:fresh');
    assert.deepStrictEqual(Array.from(v), [-1n, 0n, 1n]);
    return Array.from(v, x => x * 2n);
};

exports.js_copy_optional_u8 = (v) => {
    if (v === undefined) return undefined;
    assert.strictEqual(describe(v), 'Uint8Array:fresh');
    return v.map(x => x * 2);
};

// Freshness: the received typed array itself is retained — not a defensive
// copy of it — so a view of wasm memory would observe later mutations (or
// detach on memory growth) where the owned copy must not.
let retained = null;
exports.js_retain_u8 = (v) => {
    assert.strictEqual(describe(v), 'Uint8Array:fresh');
    retained = v;
};
exports.js_retained_u8 = () => retained;

exports.Sink = class Sink {
    kindOf(v) {
        return describe(v);
    }
};

const isView = (v) => v.buffer.byteLength > v.byteLength;

// What Rust returns is JS's, so nothing in it is a view of what Rust lent.
exports.js_returned_bytes_are_copies = () => {
    const bytes = wasm.slice_modes_lent_bytes();
    assert.deepStrictEqual(bytes.map(b => Array.from(b)), [[1, 2], [3]]);
    assert.ok(bytes.every(b => b instanceof Uint8Array && !isView(b)));
    const maybe = wasm.slice_modes_maybe_lent_bytes();
    assert.deepStrictEqual(Array.from(maybe[0]), [1, 2]);
    assert.strictEqual(maybe[1], undefined);
    assert.ok(!isView(maybe[0]));
    const nested = wasm.slice_modes_nested_lent_floats();
    assert.deepStrictEqual(nested.map(v => v.map(f => Array.from(f))), [[[1.5]], []]);
    assert.ok(nested[0][0] instanceof Float64Array && !isView(nested[0][0]));
};

// While an argument's elements are lent as the argument itself would be.
exports.js_lent_bytes_are_views = (bytes) =>
    bytes.length === 2 && bytes.every(b => b instanceof Uint8Array && isView(b));
