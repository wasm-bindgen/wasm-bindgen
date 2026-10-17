const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

const big = 'a'.repeat(200 * 1024);
const MAX_GROWTH = 4 * 1024 * 1024;

// Leaked, 100 calls each owning 200 KB would grow memory by 20 MB.
function assertFlat(memory, f) {
    f();
    const before = memory.buffer.byteLength;
    for (let i = 0; i < 100; i++) f();
    assert.ok(memory.buffer.byteLength - before < MAX_GROWTH);
}

exports.rejectedAfterOwned = (memory) => {
    for (const [f, bad, message] of [
        [wasm.arg_leaks_enum, 1.5, /invalid enum value passed/],
        [wasm.arg_leaks_option_enum, 1.5, /invalid enum value passed/],
        [wasm.arg_leaks_enum_vec, [0, 1.5], /invalid enum value passed/],
        [wasm.arg_leaks_union, {}, /invalid dynamic union value/],
    ]) {
        assertFlat(memory, () => assert.throws(() => f(big, bad), message));
    }
    // A rejected argument doesn't stop later ones from being taken and freed.
    assertFlat(memory, () =>
        assert.throws(() => wasm.arg_leaks_enum_first(1.5, big), /invalid enum value passed/));
};

exports.rejectedOwnedObject = (memory) => {
    const bytes = new Uint8Array(200 * 1024);
    const drops = wasm.arg_leaks_drops();
    assertFlat(memory, () => {
        const o = new wasm.ArgLeakObject();
        o.borrowed(() => {
            assert.throws(() => wasm.arg_leaks_object(bytes, o), /attempted to take ownership/);
        });
    });
    // JS gave up each object to the rejected call, so it is freed once its
    // borrow ends.
    assert.strictEqual(wasm.arg_leaks_drops() - drops, 101);
};

exports.rejectedReceiver = () => {
    const o = new wasm.ArgLeakObject();
    assert.throws(() => o.set(1.5), /invalid enum value passed/);
    o.set(1);
    assert.strictEqual(o.value(), 1);
    assert.throws(() => wasm.arg_leaks_object_ref(o, 1.5), /invalid enum value passed/);
    o.set(0);
    assert.strictEqual(o.value(), 0);
    const drops = wasm.arg_leaks_drops();
    o.free();
    assert.strictEqual(wasm.arg_leaks_drops() - drops, 1);
};

exports.rejectedSetter = (memory) => {
    const o = new wasm.ArgLeakObject();
    o.borrowed(() => {
        assertFlat(memory, () =>
            assert.throws(() => { o.name = big; }, /recursive use of an object/));
    });
    o.name = 'set';
    assert.strictEqual(o.name, 'set');
    o.free();
};

exports.rejectedFree = () => {
    const drops = wasm.arg_leaks_drops();
    const o = new wasm.ArgLeakObject();
    o.borrowed(() => {
        assert.throws(() => o.free(), /attempted to take ownership/);
        assert.strictEqual(wasm.arg_leaks_drops(), drops);
    });
    assert.strictEqual(wasm.arg_leaks_drops() - drops, 1);
};

exports.rejectedClosureArg = (f, memory) => {
    assertFlat(memory, () => assert.throws(() => f(big, 1.5), /invalid enum value passed/));
};

exports.rejectedClosureCall = (f, memory) => {
    assertFlat(memory, () =>
        assert.throws(() => f(big), /closure invoked recursively or after being dropped/));
};

exports.rejectedOnceCall = (f, memory) => {
    f('first');
    assertFlat(memory, () => assert.throws(() => f(big), /FnOnce called more than once/));
};

exports.rejectedAsync = async (memory) => {
    const call = () => assert.rejects(wasm.arg_leaks_async(big, 1.5), /invalid enum value passed/);
    await call();
    const before = memory.buffer.byteLength;
    for (let i = 0; i < 100; i++) await call();
    assert.ok(memory.buffer.byteLength - before < MAX_GROWTH);
};

// The receiver is borrowed when the method is called, not when its future is
// first polled, so the object can't be freed in between.
exports.asyncBorrowsAtCall = async () => {
    const drops = wasm.arg_leaks_drops();
    const o = new wasm.ArgLeakObject();
    o.set(1);
    const value = o.value_later();
    assert.throws(() => o.free(), /attempted to take ownership/);
    assert.strictEqual(wasm.arg_leaks_drops(), drops);
    assert.strictEqual(await value, 1);
    assert.strictEqual(wasm.arg_leaks_drops() - drops, 1);
};

// The same, rejected by JS or by Wasm converting a later argument, before
// Rust is called.
exports.rejectedInJs = (memory) => {
    for (const [f, bad, message] of [
        [wasm.arg_leaks_u64, 1, /bigint/i],
        [wasm.arg_leaks_class_ref, {}, /expected instance of/],
        [wasm.arg_leaks_bytes, null, Error],
        [wasm.arg_leaks_option_u32, 1n, /bigint/i],
    ]) {
        assertFlat(memory, () => assert.throws(() => f(big, bad), message));
    }
    const values = Array.from({ length: 1000 }, (_, i) => ({ i }));
    const live = wasm.arg_leaks_live_count();
    for (let i = 0; i < 10; i++) {
        assert.throws(() => wasm.arg_leaks_values(values, {}), /expected instance of/);
    }
    assert.strictEqual(wasm.arg_leaks_live_count(), live);
};

exports.rejectedObjectInJs = () => {
    const drops = wasm.arg_leaks_drops();
    // Rejected before JS gives up the object, which stays usable.
    const o = new wasm.ArgLeakObject();
    assert.throws(() => wasm.arg_leaks_object_u64(o, 1), /bigint/i);
    assert.throws(() => o.consume(1), /bigint/i);
    o.set(1);
    assert.strictEqual(o.value(), 1);
    o.free();
    assert.strictEqual(wasm.arg_leaks_drops() - drops, 1);
    // Rejected after, the object is freed, as when Rust rejects the call.
    assert.throws(() => wasm.arg_leaks_owned_and_ref(new wasm.ArgLeakObject(), {}), /expected instance of/);
    assert.strictEqual(wasm.arg_leaks_drops() - drops, 2);
    // Taken by value first, the object is moved by the time it is borrowed,
    // which debug glue rejects in JS and Rust rejects as null.
    const p = new wasm.ArgLeakObject();
    assert.throws(
        () => wasm.arg_leaks_owned_and_ref(p, p),
        /Attempt to use a moved value|null pointer passed to rust/,
    );
    assert.strictEqual(wasm.arg_leaks_drops() - drops, 3);
};
