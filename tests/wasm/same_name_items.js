const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.same_name_a_import = (mode, level, union) => {
    assert.strictEqual(mode, 'slow');
    assert.strictEqual(level, 2);
    assert.strictEqual(union, 'fast');
    return 'slow';
};

exports.same_name_b_import = (mode, level, union) => {
    assert.strictEqual(mode, 'off');
    assert.strictEqual(level, 20);
    assert.strictEqual(union, 'on');
    return 2.5;
};

exports.same_name_call_exports = () => {
    assert.strictEqual(wasm.same_name_a_mode('fast'), 'fast');
    assert.strictEqual(wasm.same_name_a_mode('slow'), 'slow');
    assert.strictEqual(wasm.same_name_b_mode('on'), 'on');
    assert.strictEqual(wasm.same_name_b_mode('off'), 'off');

    assert.strictEqual(wasm.same_name_a_level(2), 2);
    assert.strictEqual(wasm.same_name_b_level(20), 20);
    assert.throws(() => wasm.same_name_a_level(20), /invalid enum value/);

    assert.strictEqual(wasm.same_name_a_union('slow'), 'slow');
    assert.strictEqual(wasm.same_name_b_union('off'), 'off');
    assert.strictEqual(wasm.same_name_b_union(3), 3);
    assert.throws(() => wasm.same_name_a_union('on'), /invalid dynamic union value/);
};
