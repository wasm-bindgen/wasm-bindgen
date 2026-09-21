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
    assert_union!(js_sys::JsBigIntOrSymbolLike: bigint, symbol);
    assert_union!(js_sys::JsBooleanOrNumberLike: boolean, number);
    assert_union!(js_sys::JsBooleanOrStringLike: boolean, string);
    assert_union!(js_sys::JsBooleanOrSymbolLike: boolean, symbol);
    assert_union!(js_sys::JsNumberOrStringLike: number, string);
    assert_union!(js_sys::JsNumberOrSymbolLike: number, symbol);
    assert_union!(js_sys::JsStringOrSymbolLike: string, symbol);

    assert_union!(js_sys::JsBigIntOrBooleanOrNumberLike: bigint, boolean, number);
    assert_union!(js_sys::JsBigIntOrBooleanOrStringLike: bigint, boolean, string);
    assert_union!(js_sys::JsBigIntOrBooleanOrSymbolLike: bigint, boolean, symbol);
    assert_union!(js_sys::JsBigIntOrNumberOrStringLike: bigint, number, string);
    assert_union!(js_sys::JsBigIntOrNumberOrSymbolLike: bigint, number, symbol);
    assert_union!(js_sys::JsBigIntOrStringOrSymbolLike: bigint, string, symbol);
    assert_union!(js_sys::JsBooleanOrNumberOrStringLike: boolean, number, string);
    assert_union!(js_sys::JsBooleanOrNumberOrSymbolLike: boolean, number, symbol);
    assert_union!(js_sys::JsBooleanOrStringOrSymbolLike: boolean, string, symbol);
    assert_union!(js_sys::JsNumberOrStringOrSymbolLike: number, string, symbol);

    assert_union!(
        js_sys::JsBigIntOrBooleanOrNumberOrStringLike: bigint,
        boolean,
        number,
        string,
    );
    assert_union!(
        js_sys::JsBigIntOrBooleanOrNumberOrSymbolLike: bigint,
        boolean,
        number,
        symbol,
    );
    assert_union!(
        js_sys::JsBigIntOrBooleanOrStringOrSymbolLike: bigint,
        boolean,
        string,
        symbol,
    );
    assert_union!(
        js_sys::JsBigIntOrNumberOrStringOrSymbolLike: bigint,
        number,
        string,
        symbol,
    );
    assert_union!(
        js_sys::JsBooleanOrNumberOrStringOrSymbolLike: boolean,
        number,
        string,
        symbol,
    );

    assert_union!(
        js_sys::JsBigIntOrBooleanOrNumberOrStringOrSymbolLike: bigint,
        boolean,
        number,
        string,
        symbol,
    );
}

#[wasm_bindgen_test]
fn narrower_primitive_unions_widen_to_each_immediate_superset() {
    assert_widens!(js_sys::JsBigIntOrBooleanLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberLike,
        js_sys::JsBigIntOrBooleanOrStringLike,
        js_sys::JsBigIntOrBooleanOrSymbolLike,
    );
    assert_widens!(js_sys::JsBigIntOrNumberLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberLike,
        js_sys::JsBigIntOrNumberOrStringLike,
        js_sys::JsBigIntOrNumberOrSymbolLike,
    );
    assert_widens!(js_sys::JsBigIntOrStringLike, i64 =>
        js_sys::JsBigIntOrBooleanOrStringLike,
        js_sys::JsBigIntOrNumberOrStringLike,
        js_sys::JsBigIntOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBigIntOrSymbolLike, i64 =>
        js_sys::JsBigIntOrBooleanOrSymbolLike,
        js_sys::JsBigIntOrNumberOrSymbolLike,
        js_sys::JsBigIntOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBooleanOrNumberLike, bool =>
        js_sys::JsBigIntOrBooleanOrNumberLike,
        js_sys::JsBooleanOrNumberOrStringLike,
        js_sys::JsBooleanOrNumberOrSymbolLike,
    );
    assert_widens!(js_sys::JsBooleanOrStringLike, bool =>
        js_sys::JsBigIntOrBooleanOrStringLike,
        js_sys::JsBooleanOrNumberOrStringLike,
        js_sys::JsBooleanOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBooleanOrSymbolLike, bool =>
        js_sys::JsBigIntOrBooleanOrSymbolLike,
        js_sys::JsBooleanOrNumberOrSymbolLike,
        js_sys::JsBooleanOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsNumberOrStringLike, f64 =>
        js_sys::JsBigIntOrNumberOrStringLike,
        js_sys::JsBooleanOrNumberOrStringLike,
        js_sys::JsNumberOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsNumberOrSymbolLike, f64 =>
        js_sys::JsBigIntOrNumberOrSymbolLike,
        js_sys::JsBooleanOrNumberOrSymbolLike,
        js_sys::JsNumberOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsStringOrSymbolLike, String =>
        js_sys::JsBigIntOrStringOrSymbolLike,
        js_sys::JsBooleanOrStringOrSymbolLike,
        js_sys::JsNumberOrStringOrSymbolLike,
    );

    assert_widens!(js_sys::JsBigIntOrBooleanOrNumberLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberOrStringLike,
        js_sys::JsBigIntOrBooleanOrNumberOrSymbolLike,
    );
    assert_widens!(js_sys::JsBigIntOrBooleanOrStringLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberOrStringLike,
        js_sys::JsBigIntOrBooleanOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBigIntOrBooleanOrSymbolLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberOrSymbolLike,
        js_sys::JsBigIntOrBooleanOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBigIntOrNumberOrStringLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberOrStringLike,
        js_sys::JsBigIntOrNumberOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBigIntOrNumberOrSymbolLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberOrSymbolLike,
        js_sys::JsBigIntOrNumberOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBigIntOrStringOrSymbolLike, i64 =>
        js_sys::JsBigIntOrBooleanOrStringOrSymbolLike,
        js_sys::JsBigIntOrNumberOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBooleanOrNumberOrStringLike, bool =>
        js_sys::JsBigIntOrBooleanOrNumberOrStringLike,
        js_sys::JsBooleanOrNumberOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBooleanOrNumberOrSymbolLike, bool =>
        js_sys::JsBigIntOrBooleanOrNumberOrSymbolLike,
        js_sys::JsBooleanOrNumberOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBooleanOrStringOrSymbolLike, bool =>
        js_sys::JsBigIntOrBooleanOrStringOrSymbolLike,
        js_sys::JsBooleanOrNumberOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsNumberOrStringOrSymbolLike, f64 =>
        js_sys::JsBigIntOrNumberOrStringOrSymbolLike,
        js_sys::JsBooleanOrNumberOrStringOrSymbolLike,
    );

    assert_widens!(js_sys::JsBigIntOrBooleanOrNumberOrStringLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBigIntOrBooleanOrNumberOrSymbolLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBigIntOrBooleanOrStringOrSymbolLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBigIntOrNumberOrStringOrSymbolLike, i64 =>
        js_sys::JsBigIntOrBooleanOrNumberOrStringOrSymbolLike,
    );
    assert_widens!(js_sys::JsBooleanOrNumberOrStringOrSymbolLike, bool =>
        js_sys::JsBigIntOrBooleanOrNumberOrStringOrSymbolLike,
    );
}
