//! Owned vectors whose elements own memory or JS handles, whose glue calls
//! into Wasm for each element. Each wrapper crosses as the type it wraps,
//! with its vectors encoded.

use wasm_bindgen::convert::{
    FromWasmAbi, IntoWasmAbi, VectorFromWasmAbi, VectorIntoWasmAbi, WasmSlice,
};
use wasm_bindgen::describe::{inform, WasmDescribe, WasmDescribeVector, VECTOR};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

macro_rules! encoded {
    ($($name:ident($t:ty);)*) => ($(
        #[derive(Debug, PartialEq)]
        pub struct $name(pub $t);

        impl WasmDescribe for $name {
            fn describe() {
                <$t>::describe()
            }
        }

        impl IntoWasmAbi for $name {
            type Abi = <$t as IntoWasmAbi>::Abi;

            fn into_abi(self) -> Self::Abi {
                self.0.into_abi()
            }
        }

        impl FromWasmAbi for $name {
            type Abi = <$t as FromWasmAbi>::Abi;

            unsafe fn from_abi(js: Self::Abi) -> Self {
                $name(<$t>::from_abi(js))
            }
        }

        impl WasmDescribeVector for $name {
            fn describe_vector() {
                inform(VECTOR);
                <$t>::describe();
            }
        }

        impl VectorIntoWasmAbi for $name {
            type Abi = WasmSlice;

            fn vector_into_abi(vector: Box<[Self]>) -> WasmSlice {
                WasmSlice::encode_owned(vector, IntoWasmAbi::into_abi)
            }
        }

        impl VectorFromWasmAbi for $name {
            type Abi = WasmSlice;

            unsafe fn vector_from_abi(js: WasmSlice) -> Box<[Self]> {
                js.try_decode().unwrap_or_else(|msg| wasm_bindgen::throw_str(msg))
            }
        }
    )*)
}

encoded! {
    Name(String);
    MaybeName(Option<String>);
    MaybeObj(Option<JsValue>);
    NullableObj(Nullable<JsValue>);
    MaybeByte(Option<u8>);
}

#[wasm_bindgen(module = "tests/wasm/staged_vectors.js")]
extern "C" {
    #[wasm_bindgen(js_name = roundTrips)]
    fn round_trips(memory: JsValue);
    #[wasm_bindgen(js_name = rejectedElements)]
    fn rejected_elements(memory: JsValue);
    #[wasm_bindgen(js_name = jsNames)]
    fn js_names(v: Vec<Name>) -> Vec<Name>;
    #[wasm_bindgen(js_name = jsMaybeObjs)]
    fn js_maybe_objs(v: Vec<MaybeObj>) -> Vec<MaybeObj>;
}

#[wasm_bindgen]
pub fn staged_names(v: Vec<Name>) -> Vec<Name> {
    v
}

#[wasm_bindgen]
pub fn staged_maybe_names(v: Option<Vec<MaybeName>>) -> Option<Vec<MaybeName>> {
    v
}

#[wasm_bindgen]
pub fn staged_maybe_objs(v: Vec<MaybeObj>) -> Vec<MaybeObj> {
    v
}

#[wasm_bindgen]
pub fn staged_nullable_objs(v: Vec<NullableObj>) -> Vec<NullableObj> {
    v
}

#[wasm_bindgen]
pub fn staged_maybe_bytes(v: Vec<MaybeByte>) -> Vec<MaybeByte> {
    v
}

#[wasm_bindgen]
pub fn staged_objs_then_names(_a: Vec<MaybeObj>, _b: Vec<Name>) {}

#[wasm_bindgen]
pub fn staged_live_count() -> u32 {
    wasm_bindgen::externref_heap_live_count()
}

#[wasm_bindgen_test]
fn exports() {
    round_trips(wasm_bindgen::memory());
}

#[wasm_bindgen_test]
fn rejected() {
    rejected_elements(wasm_bindgen::memory());
}

#[wasm_bindgen_test]
fn imports() {
    let names = js_names(vec![Name("a".into()), Name("é".repeat(100))]);
    assert_eq!(
        names,
        [Name("é".repeat(100)), Name("a".into()), Name("js".into())]
    );
    let objs = js_maybe_objs(vec![
        MaybeObj(Some(JsValue::from(1))),
        MaybeObj(None),
        MaybeObj(Some(JsValue::NULL)),
    ]);
    assert_eq!(objs.len(), 4);
    assert_eq!(objs[0].0, None);
    assert_eq!(objs[1].0.as_ref().and_then(JsValue::as_f64), Some(1.0));
    // `null` is `None` for an `Option<JsValue>`.
    assert_eq!(objs[2].0, None);
    assert_eq!(
        objs[3].0.as_ref().and_then(JsValue::as_string),
        Some("js".into())
    );
}
