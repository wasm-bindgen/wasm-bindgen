const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.js_works = async () => {
    assert.deepStrictEqual(await wasm.async_jsvalue_vec(), [1, "hi", new Float64Array(), null]);
    assert.deepStrictEqual(await wasm.async_import_vec(), [/hi|bye/, /hello w[a-z]rld/]);
    assert.deepStrictEqual(await wasm.async_string_vec(), ["a", "b", "c"]);
    assert.strictEqual((await wasm.async_struct_vec()).length, 2);
    assert.deepStrictEqual(await wasm.async_enum_vec(), [wasm.AnotherEnum.C, wasm.AnotherEnum.A, wasm.AnotherEnum.B]);

    const numberVec = await wasm.async_number_vec();
    assert.deepStrictEqual(numberVec, new Int32Array([1, -3, 7, 12]));
    // Make sure `numberVec` is a fresh `Int32Array`, not a view into Wasm memory,
    // so that it can be GC'd without the whole Wasm module having to be GC'd as
    // well.
    assert.strictEqual(numberVec.byteLength, numberVec.buffer.byteLength);
};
