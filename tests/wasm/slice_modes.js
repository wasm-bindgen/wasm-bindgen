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

// A sequence Rust gives holds copies of the numeric slices lent to it.
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

exports.js_async_returned_bytes_are_copies = async () => {
    const bytes = await wasm.slice_modes_async_lent_bytes();
    assert.deepStrictEqual(bytes.map(b => Array.from(b)), [[1, 2], [3]]);
    assert.ok(bytes.every(b => b instanceof Uint8Array && !isView(b)));
};

exports.js_lent_bytes_are_copies = (bytes) =>
    bytes.length === 2 && bytes.every(b => b instanceof Uint8Array && !isView(b));

const typedArrays = (v) =>
    Array.isArray(v) ? v.flatMap(typedArrays) : ArrayBuffer.isView(v) ? [v] : [];

exports.js_holds_no_view = (v) => {
    const all = typedArrays(v);
    return all.length > 0 && !all.some(isView);
};

exports.js_fill = (v, byte) => typedArrays(v).forEach(a => a.fill(byte));

let kept;
exports.js_keep = (v) => { kept = v; };
exports.js_kept = () => kept;
exports.js_kept_is = (value) => {
    const all = typedArrays(kept);
    return all.length > 0 && all.every(a => a.length > 0 && a.every(x => x === value));
};

let keptValue;
exports.keep = (v) => { keptValue = v; };
exports.kept = () => {
    const v = keptValue;
    if (v === undefined || v === null) return 'none';
    const bytes = v instanceof ArrayBuffer ? new Uint8Array(v) : new Uint8Array(v.buffer, v.byteOffset, v.byteLength);
    return `${v.constructor.name}:${Array.from(bytes)}`;
};
