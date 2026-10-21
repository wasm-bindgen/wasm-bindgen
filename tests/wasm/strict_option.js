const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

const strict = () => wasm.strict_option_enabled();

// Debug glue asserts argument types, release glue lets JS coerce them.
const debug = () => {
    try {
        wasm.strict_option_u8('x');
        return false;
    } catch {
        return true;
    }
};
exports.strict_option_js_debug = debug;

// `null` under the cfg reaches `T`'s conversion: debug glue throws its type
// assertion; release glue throws `throws` or hands Rust `value`. Without the
// cfg it is `None`, which `f` returns as `none`.
const checkNull = (f, { throws, value, none }) => {
    if (!strict()) {
        assert.strictEqual(f(null), none);
    } else if (debug()) {
        assert.throws(() => f(null), Error);
    } else if (throws) {
        assert.throws(() => f(null), throws);
    } else {
        assert.strictEqual(f(null), value);
    }
};

exports.strict_option_js_exports = () => {
    for (const f of [
        wasm.strict_option_i32, wasm.strict_option_u8, wasm.strict_option_usize,
        wasm.strict_option_f32, wasm.strict_option_f64,
    ]) {
        assert.strictEqual(f(undefined), undefined);
        assert.strictEqual(f(), undefined);
        assert.strictEqual(f(3), 3);
        checkNull(f, { value: 0 });
    }

    for (const f of [wasm.strict_option_i64, wasm.strict_option_u128]) {
        assert.strictEqual(f(undefined), undefined);
        assert.strictEqual(f(3n), 3n);
        checkNull(f, { throws: TypeError });
    }

    // The null pointer is `None` for `Option<NonNull<T>>`, so coerced `null`
    // is too.
    assert.strictEqual(wasm.strict_option_nonnull(undefined), false);
    assert.strictEqual(wasm.strict_option_nonnull(0), false);
    assert.strictEqual(wasm.strict_option_nonnull(8), true);
    checkNull(wasm.strict_option_nonnull, { value: false, none: false });

    assert.strictEqual(wasm.strict_option_bool(undefined), undefined);
    assert.strictEqual(wasm.strict_option_bool(true), true);
    checkNull(wasm.strict_option_bool, { value: false });

    assert.strictEqual(wasm.strict_option_char(undefined), undefined);
    assert.strictEqual(wasm.strict_option_char('x'), 'x');
    checkNull(wasm.strict_option_char, { throws: TypeError });

    assert.strictEqual(wasm.strict_option_string(undefined), undefined);
    assert.strictEqual(wasm.strict_option_string('hi'), 'hi');
    checkNull(wasm.strict_option_string, { throws: TypeError });

    assert.strictEqual(wasm.strict_option_vec_u8(undefined), undefined);
    assert.strictEqual(wasm.strict_option_vec_u8(new Uint8Array([1, 2])), 2);
    checkNull(wasm.strict_option_vec_u8, { throws: TypeError });
    assert.strictEqual(wasm.strict_option_vec_bool(undefined), undefined);
    assert.strictEqual(wasm.strict_option_vec_bool([true, false]), 2);
    checkNull(wasm.strict_option_vec_bool, { throws: TypeError });
    assert.strictEqual(wasm.strict_option_vec_jsvalue(undefined), undefined);
    assert.strictEqual(wasm.strict_option_vec_jsvalue([1, 'a']), 2);
    checkNull(wasm.strict_option_vec_jsvalue, { throws: TypeError });

    assert.strictEqual(wasm.strict_option_enum(undefined), undefined);
    assert.strictEqual(wasm.strict_option_enum(wasm.StrictColor.Blue), wasm.StrictColor.Blue);
    checkNull(wasm.strict_option_enum, { value: wasm.StrictColor.Red });

    // An unknown string, `null` included under the cfg, is the invalid variant.
    assert.strictEqual(wasm.strict_option_string_enum(undefined), 'none');
    assert.strictEqual(wasm.strict_option_string_enum('beta'), 'beta');
    assert.strictEqual(wasm.strict_option_string_enum('gamma'), 'invalid');
    assert.strictEqual(wasm.strict_option_string_enum(null), strict() ? 'invalid' : 'none');

    assert.strictEqual(wasm.strict_option_struct(undefined), undefined);
    assert.strictEqual(wasm.strict_option_struct(new wasm.StrictThing(4)), 4);
    checkNull(wasm.strict_option_struct, { throws: Error });

    const fields = new wasm.StrictFields();
    assert.strictEqual(fields.n, undefined);
    fields.n = 2;
    assert.strictEqual(fields.n, 2);
    fields.n = undefined;
    assert.strictEqual(fields.n, undefined);
    fields.s = 'x';
    assert.strictEqual(fields.s, 'x');
    checkNull(v => { fields.n = v; return fields.n; }, { value: 0 });
    checkNull(v => { fields.s = v; return fields.s; }, { throws: TypeError });

    for (const f of [wasm.strict_option_jsvalue, wasm.strict_option_js_type]) {
        assert.strictEqual(f(undefined), 'none');
        assert.strictEqual(f(null), strict() ? 'null' : 'none');
        assert.strictEqual(f({}), 'some');
    }
};

exports.strict_option_closure_driver = f => {
    assert.strictEqual(f(undefined, undefined), 'None None');
    assert.strictEqual(f(1, 'a'), 'Some(1) Some("a")');
    checkNull(v => f(v, undefined), { value: 'Some(0) None', none: 'None None' });
    checkNull(v => f(undefined, v), { throws: TypeError, none: 'None None' });
};

exports.strict_option_js_undefined = () => undefined;
exports.strict_option_js_null = () => null;
exports.strict_option_js_value = () => 7;
exports.strict_option_js_true = () => true;
exports.strict_option_js_string = () => 'hi';
exports.strict_option_js_bytes = () => new Uint8Array([1, 2, 3]);
exports.strict_option_js_thing = () => new wasm.StrictThing(5);
