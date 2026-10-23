const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.js_identity = a => a;

exports.js_works = () => {
    assert.strictEqual(wasm.letter(), 'a');
    assert.strictEqual(wasm.face(), '😀');
    assert.strictEqual(wasm.rust_identity(''), '\u0000');
    assert.strictEqual(wasm.rust_identity('Ղ'), 'Ղ');
    assert.strictEqual(wasm.rust_identity('ҝ'), 'ҝ');
    assert.strictEqual(wasm.rust_identity('Δ'), 'Δ');
    assert.strictEqual(wasm.rust_identity('䉨'), '䉨');
    assert.strictEqual(wasm.rust_js_identity('a'), 'a');
    assert.strictEqual(wasm.rust_js_identity('㊻'), '㊻');
    wasm.rust_letter('a');
    wasm.rust_face('😀');

    assert.strictEqual(wasm.rust_option_identity(undefined), undefined);
    if (!wasm.strict_option_enabled()) {
        assert.strictEqual(wasm.rust_option_identity(null), undefined);
    }
    assert.strictEqual(wasm.rust_option_identity(''), '\u0000');
    assert.strictEqual(wasm.rust_option_identity('\u0000'), '\u0000');

    assert.throws(() => wasm.rust_identity(55357), /c.codePointAt is not a function/);
    assert.throws(() => wasm.rust_identity('\uD83D'), /expected a valid Unicode scalar value, found 55357/);
    assert.throws(() => wasm.rust_option_identity('\uD83D'), /expected a valid Unicode scalar value, found 55357/);
};
