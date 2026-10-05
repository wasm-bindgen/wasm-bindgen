use js_sys::{BigInt, Boolean, JsString, Number, Symbol};
use wasm_bindgen_test::*;

macro_rules! assert_types_implement {
    ($trait:path: $($ty:ty),+ $(,)?) => {{
        fn assert_one<T: $trait>() {}
        $(assert_one::<$ty>();)+
    }};
}

macro_rules! assert_bigint_members {
    ($trait:path) => {
        assert_types_implement!($trait: i64, u64, i128, u128, BigInt, &'static BigInt)
    };
}

macro_rules! assert_boolean_members {
    ($trait:path) => {
        assert_types_implement!($trait: bool, Boolean, &'static Boolean)
    };
}

macro_rules! assert_number_members {
    ($trait:path) => {
        assert_types_implement!(
            $trait: i8,
            u8,
            i16,
            u16,
            i32,
            u32,
            isize,
            usize,
            f32,
            f64,
            Number,
            &'static Number,
        )
    };
}

macro_rules! assert_string_members {
    ($trait:path) => {
        assert_types_implement!(
            $trait: String,
            &'static str,
            JsString,
            &'static JsString,
        )
    };
}

macro_rules! assert_symbol_members {
    ($trait:path) => {
        assert_types_implement!($trait: Symbol, &'static Symbol)
    };
}

macro_rules! assert_widens {
    ($narrow:path, $witness:ty => $($wider:path),+ $(,)?) => {{
        fn assert_one<T: $narrow>() {
            $({
                fn accepts_wider<U: $wider>() {}
                accepts_wider::<T>();
            })+
        }
        assert_one::<$witness>();
    }};
}

#[wasm_bindgen_test]
fn every_primitive_union_accepts_its_member_categories() {
    macro_rules! assert_union {
        ($trait:path: $($category:ident),+ $(,)?) => {
            $(assert_union!(@category $trait, $category);)+
        };
        (@category $trait:path, bigint) => { assert_bigint_members!($trait) };
        (@category $trait:path, boolean) => { assert_boolean_members!($trait) };
        (@category $trait:path, number) => { assert_number_members!($trait) };
        (@category $trait:path, string) => { assert_string_members!($trait) };
        (@category $trait:path, symbol) => { assert_symbol_members!($trait) };
    }

    assert_union!(js_sys::JsBigIntOrBooleanLike: bigint, boolean);
    assert_union!(js_sys::JsBigIntOrNumberLike: bigint, number);
    assert_union!(js_sys::JsBigIntOrStringLike: bigint, string);
    assert_union!(js_sys::JsBooleanOrNumberLike: boolean, number);
    assert_union!(js_sys::JsBooleanOrStringLike: boolean, string);
    assert_union!(js_sys::JsNumberOrStringLike: number, string);

    assert_union!(js_sys::JsBigIntOrBooleanOrNumberLike: bigint, boolean, number);
    assert_union!(js_sys::JsBigIntOrBooleanOrStringLike: bigint, boolean, string);
    assert_union!(js_sys::JsBigIntOrNumberOrStringLike: bigint, number, string);
    assert_union!(js_sys::JsBooleanOrNumberOrStringLike: boolean, number, string);

    assert_union!(
        js_sys::JsBigIntOrBooleanOrNumberOrStringLike: bigint,
        boolean,
        number,
        string,
    );

    assert_union!(js_sys::PropertyKey: number, string, symbol);
}

#[wasm_bindgen_test]
fn narrower_primitive_unions_widen_to_each_immediate_superset() {
    assert_widens!(js_sys::JsBigIntOrBooleanLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberLike,
        js_sys::JsBigIntOrBooleanOrStringLike,
    );
    assert_widens!(js_sys::JsBigIntOrNumberLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberLike,
        js_sys::JsBigIntOrNumberOrStringLike,
    );
    assert_widens!(js_sys::JsBigIntOrStringLike, i64 =>
        js_sys::JsBigIntOrBooleanOrStringLike,
        js_sys::JsBigIntOrNumberOrStringLike,
    );
    assert_widens!(js_sys::JsBooleanOrNumberLike, bool =>
        js_sys::JsBigIntOrBooleanOrNumberLike,
        js_sys::JsBooleanOrNumberOrStringLike,
    );
    assert_widens!(js_sys::JsBooleanOrStringLike, bool =>
        js_sys::JsBigIntOrBooleanOrStringLike,
        js_sys::JsBooleanOrNumberOrStringLike,
    );
    assert_widens!(js_sys::JsNumberOrStringLike, f64 =>
        js_sys::JsBigIntOrNumberOrStringLike,
        js_sys::JsBooleanOrNumberOrStringLike,
        js_sys::PropertyKey,
    );

    assert_widens!(js_sys::JsBigIntOrBooleanOrNumberLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberOrStringLike,
    );
    assert_widens!(js_sys::JsBigIntOrBooleanOrStringLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberOrStringLike,
    );
    assert_widens!(js_sys::JsBigIntOrNumberOrStringLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberOrStringLike,
    );
    assert_widens!(js_sys::JsBooleanOrNumberOrStringLike, bool =>
        js_sys::JsBigIntOrBooleanOrNumberOrStringLike,
    );
}
