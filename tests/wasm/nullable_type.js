const wasm = require('wasm-bindgen-test.js');
const assert = require('assert');

exports.nullable_type_describe = v =>
    v === null ? 'null' : v === undefined ? 'undefined' : 'value';

exports.nullable_type_js_null = () => null;
exports.nullable_type_js_undefined = () => undefined;
exports.nullable_type_js_number = () => 7;
exports.nullable_type_js_float = () => 2.5;
exports.nullable_type_js_bigint = () => 7n;
exports.nullable_type_js_true = () => true;
exports.nullable_type_js_string = () => 'hi';
exports.nullable_type_js_bytes = () => new Uint8Array([1, 2, 3]);
exports.nullable_type_js_bools = () => [true, false];
exports.nullable_type_js_values = () => [1, 'a'];
exports.nullable_type_js_beta = () => 'beta';
exports.nullable_type_js_thing = () => new wasm.NullableThing(5);
exports.nullable_type_js_object = () => ({});

// Debug glue asserts argument types, release glue lets JS coerce them.
const debug = () => {
    try {
        wasm.nullable_type_u8('x');
        return false;
    } catch {
        return true;
    }
};
exports.nullable_type_js_debug = debug;

// `undefined` into a plain `Nullable<T>` goes to `T`'s conversion, as `null`
// does into a strict `Option<T>`: debug glue throws its type assertion;
// release glue throws `throws` or hands Rust `value`.
const checkUndefined = (f, { throws, value }) => {
    if (debug()) {
        assert.throws(() => f(undefined), Error);
    } else if (throws) {
        assert.throws(() => f(undefined), throws);
    } else {
        assert.strictEqual(f(undefined), value);
    }
};

// Each export is the identity: the states come back as themselves, `null`
// and `undefined` checked strictly.
const roundtrip = (f, value, eq = assert.strictEqual) => {
    assert.strictEqual(f(null), null);
    eq(f(value), value);
};
const roundtripOpt = (f, value, eq = assert.strictEqual) => {
    roundtrip(f, value, eq);
    assert.strictEqual(f(undefined), undefined);
    assert.strictEqual(f(), undefined);
};
const roundtripBoth = (f, g, value, eq) => {
    roundtrip(f, value, eq);
    roundtripOpt(g, value, eq);
};

exports.nullable_type_js_exports = () => {
    for (const [f, g] of [
        [wasm.nullable_type_u8, wasm.nullable_type_opt_u8],
        [wasm.nullable_type_i32, wasm.nullable_type_opt_i32],
        [wasm.nullable_type_u32, wasm.nullable_type_opt_u32],
        [wasm.nullable_type_usize, wasm.nullable_type_opt_usize],
    ]) {
        roundtripBoth(f, g, 3);
        checkUndefined(f, { value: 0 });
    }

    roundtripBoth(wasm.nullable_type_nonnull, wasm.nullable_type_opt_nonnull, 8);
    // The null pointer is `Null`: a `Nullable<NonNull<T>>` has no other empty
    // state for it.
    assert.strictEqual(wasm.nullable_type_nonnull(0), null);
    assert.strictEqual(wasm.nullable_type_opt_nonnull(0), null);
    checkUndefined(wasm.nullable_type_nonnull, { value: null });
    roundtripBoth(wasm.nullable_type_ptr, wasm.nullable_type_opt_ptr, 8);
    assert.strictEqual(wasm.nullable_type_ptr(0), 0);
    checkUndefined(wasm.nullable_type_ptr, { value: 0 });

    for (const [f, g] of [
        [wasm.nullable_type_f32, wasm.nullable_type_opt_f32],
        [wasm.nullable_type_f64, wasm.nullable_type_opt_f64],
    ]) {
        roundtripBoth(f, g, 1.5);
        checkUndefined(f, { value: NaN });
    }
    roundtripBoth(wasm.nullable_type_i64, wasm.nullable_type_opt_i64, -3n);
    roundtripBoth(wasm.nullable_type_u128, wasm.nullable_type_opt_u128, 3n);
    checkUndefined(wasm.nullable_type_i64, { throws: TypeError });
    checkUndefined(wasm.nullable_type_u128, { throws: TypeError });

    roundtripBoth(wasm.nullable_type_bool, wasm.nullable_type_opt_bool, true);
    roundtripBoth(wasm.nullable_type_bool, wasm.nullable_type_opt_bool, false);
    checkUndefined(wasm.nullable_type_bool, { value: false });
    roundtripBoth(wasm.nullable_type_char, wasm.nullable_type_opt_char, 'x');
    checkUndefined(wasm.nullable_type_char, { throws: TypeError });
    roundtripBoth(wasm.nullable_type_string, wasm.nullable_type_opt_string, 'hi');
    roundtripBoth(wasm.nullable_type_string, wasm.nullable_type_opt_string, '');
    checkUndefined(wasm.nullable_type_string, { throws: TypeError });

    roundtripBoth(
        wasm.nullable_type_vec_u8, wasm.nullable_type_opt_vec_u8,
        new Uint8Array([1, 2]), assert.deepStrictEqual,
    );
    checkUndefined(wasm.nullable_type_vec_u8, { throws: TypeError });
    roundtripBoth(
        wasm.nullable_type_vec_bool, wasm.nullable_type_opt_vec_bool,
        [true, false], assert.deepStrictEqual,
    );
    checkUndefined(wasm.nullable_type_vec_bool, { throws: TypeError });
    roundtripBoth(
        wasm.nullable_type_vec_jsvalue, wasm.nullable_type_opt_vec_jsvalue,
        [1, 'a'], assert.deepStrictEqual,
    );
    checkUndefined(wasm.nullable_type_vec_jsvalue, { throws: TypeError });

    roundtripBoth(wasm.nullable_type_enum, wasm.nullable_type_opt_enum, wasm.NullableColor.Blue);
    // Coerced to `NaN`, an invalid discriminant, which throws either way.
    assert.throws(() => wasm.nullable_type_enum(undefined));

    roundtripBoth(wasm.nullable_type_string_enum, wasm.nullable_type_opt_string_enum, 'beta');
    // An unknown string, `undefined` included, is the invalid variant.
    assert.strictEqual(wasm.nullable_type_string_enum_name(null), 'null');
    assert.strictEqual(wasm.nullable_type_string_enum_name('alpha'), 'alpha');
    assert.strictEqual(wasm.nullable_type_string_enum_name('gamma'), 'invalid');
    assert.strictEqual(wasm.nullable_type_string_enum_name(undefined), 'invalid');
    assert.strictEqual(wasm.nullable_type_opt_string_enum_name(undefined), 'undefined');
    assert.strictEqual(wasm.nullable_type_opt_string_enum_name(), 'undefined');
    assert.strictEqual(wasm.nullable_type_opt_string_enum_name(null), 'null');
    assert.strictEqual(wasm.nullable_type_opt_string_enum_name('beta'), 'beta');
    assert.strictEqual(wasm.nullable_type_opt_string_enum_name('gamma'), 'invalid');

    const object = {};
    roundtripBoth(wasm.nullable_type_jsvalue, wasm.nullable_type_opt_jsvalue, object);
    roundtripBoth(wasm.nullable_type_js_type, wasm.nullable_type_opt_js_type, object);
    // `undefined` is a JS value like any other for a plain `Nullable<JsValue>`.
    assert.strictEqual(wasm.nullable_type_jsvalue(undefined), undefined);
    assert.strictEqual(wasm.nullable_type_js_type(undefined), undefined);

    // `null` into a JS-typed nullable is the fixed handle, not a fresh slot.
    const table = Object.values(wasm.__wasm).find(v => v instanceof WebAssembly.Table);
    if (table) {
        wasm.nullable_type_jsvalue(null);
        const before = table.length;
        for (let i = 0; i < 2000; i++) {
            wasm.nullable_type_jsvalue(null);
            wasm.nullable_type_opt_jsvalue(null);
            wasm.nullable_type_js_type(null);
            wasm.nullable_type_opt_js_type(null);
        }
        assert.strictEqual(table.length, before);
    }

    assert.strictEqual(wasm.nullable_type_struct(null), null);
    assert.strictEqual(wasm.nullable_type_struct(new wasm.NullableThing(4)).n, 4);
    checkUndefined(wasm.nullable_type_struct, { throws: Error });
    assert.strictEqual(wasm.nullable_type_opt_struct(null), null);
    assert.strictEqual(wasm.nullable_type_opt_struct(undefined), undefined);
    assert.strictEqual(wasm.nullable_type_opt_struct(), undefined);
    assert.strictEqual(wasm.nullable_type_opt_struct(new wasm.NullableThing(4)).n, 4);

    // `Result` returns peel the `Result` first.
    const results = (f, g, value, eq = assert.strictEqual) => {
        assert.strictEqual(f(null, false), null);
        eq(f(value, false), value);
        assert.throws(() => f(value, true), /failed/);
        assert.strictEqual(g(null, false), null);
        assert.strictEqual(g(undefined, false), undefined);
        eq(g(value, false), value);
        assert.throws(() => g(undefined, true), /failed/);
    };
    results(wasm.nullable_type_result_u32, wasm.nullable_type_result_opt_u32, 3);
    results(wasm.nullable_type_result_i64, wasm.nullable_type_result_opt_i64, 3n);
    results(wasm.nullable_type_result_string, wasm.nullable_type_result_opt_string, 'hi');
    results(wasm.nullable_type_result_jsvalue, wasm.nullable_type_result_opt_jsvalue, object);
    assert.strictEqual(wasm.nullable_type_result_struct(null, false), null);
    assert.strictEqual(wasm.nullable_type_result_struct(new wasm.NullableThing(4), false).n, 4);
    assert.throws(() => wasm.nullable_type_result_struct(null, true), /failed/);
    assert.strictEqual(wasm.nullable_type_result_opt_struct(null, false), null);
    assert.strictEqual(wasm.nullable_type_result_opt_struct(undefined, false), undefined);
    assert.strictEqual(wasm.nullable_type_result_opt_struct(new wasm.NullableThing(4), false).n, 4);

    const fields = new wasm.NullableFields();
    assert.strictEqual(fields.n, null);
    assert.strictEqual(fields.o, undefined);
    assert.strictEqual(fields.s, null);
    assert.strictEqual(fields.t, undefined);
    fields.n = 2;
    assert.strictEqual(fields.n, 2);
    fields.n = null;
    assert.strictEqual(fields.n, null);
    checkUndefined(v => { fields.n = v; return fields.n; }, { value: 0 });
    for (const v of [2, null, undefined]) {
        fields.o = v;
        assert.strictEqual(fields.o, v);
    }
    fields.s = 'x';
    assert.strictEqual(fields.s, 'x');
    fields.s = null;
    assert.strictEqual(fields.s, null);
    checkUndefined(v => { fields.s = v; return fields.s; }, { throws: TypeError });
    for (const v of ['x', null, undefined]) {
        fields.t = v;
        assert.strictEqual(fields.t, v);
    }
};

exports.nullable_type_closure_driver = f => {
    assert.strictEqual(f(null, undefined), 'Null None');
    assert.strictEqual(f(1, null), 'Value(1) Some(Null)');
    assert.strictEqual(f(1, 'a'), 'Value(1) Some(Value("a"))');
    checkUndefined(v => f(v, undefined), { value: 'Value(0) None' });
};

exports.nullable_type_nullable_closure_driver = f => {
    assert.strictEqual(f(null), null);
    assert.strictEqual(f('a'), 'a');
    checkUndefined(f, { throws: TypeError });
};

exports.nullable_type_optional_closure_driver = f => {
    assert.strictEqual(f(null), null);
    assert.strictEqual(f(undefined), undefined);
    assert.strictEqual(f(), undefined);
    assert.strictEqual(f(3), 3);
};
