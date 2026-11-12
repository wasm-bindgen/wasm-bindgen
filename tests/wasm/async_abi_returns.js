const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.js_async_abi_returns = async () => {
    assert.strictEqual(await wasm.async_char(), 'é');
    assert.strictEqual(await wasm.async_option_char(true), 'x');
    assert.strictEqual(await wasm.async_option_char(false), undefined);
    assert.strictEqual(await wasm.async_nullable(true), 7);
    assert.strictEqual(await wasm.async_nullable(false), null);
    assert.strictEqual(await wasm.async_result_char(true), 'y');
    await assert.rejects(wasm.async_result_char(false), /no char/);
    assert.strictEqual(await wasm.async_into_only_error(true), 3);
    await assert.rejects(wasm.async_into_only_error(false), e => e === 'into only');
    assert.strictEqual(await wasm.async_unit_error(true), undefined);
    await assert.rejects(wasm.async_unit_error(false), e => e === 'unit');
};
