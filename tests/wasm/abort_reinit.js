const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.catch_panic = () => {
    // Normal operation works
    assert.strictEqual(wasm.simple_add(1, 2), 3);
    assert.strictEqual(wasm.maybe_panic(false), 42);

    // Trigger panic - should throw
    let panicked = false;
    try {
        wasm.maybe_panic(true);
    } catch (e) {
        panicked = true;
        // Verify it's a RuntimeError from Wasm
        assert(e instanceof WebAssembly.RuntimeError || e.message.includes('unreachable'), 
               `Expected RuntimeError, got: ${e}`);
    }
    assert(panicked, 'Expected panic to throw');

    // After panic, next call should work (module was re-initialized)
    assert.strictEqual(wasm.simple_add(1, 2), 3, 'Module should work after re-init');
    assert.strictEqual(wasm.maybe_panic(false), 42, 'Module should work after re-init');
};

exports.expect_reinit = () => {
    let counter = new wasm.models.Counter(0);
    counter.increment();
    // Increment counter a few times
    assert.strictEqual(wasm.increment_and_get(), 0);
    assert.strictEqual(wasm.increment_and_get(), 1);
    assert.strictEqual(wasm.increment_and_get(), 2);
    // Trigger panic
    try {
        wasm.trigger_panic();
    } catch (e) {
        // Expected
    }

    // After re-init, counter should be back to 0
    assert.strictEqual(wasm.increment_and_get(), 0, 'Counter should be reset after re-init');
    assert.strictEqual(wasm.increment_and_get(), 1);

}
