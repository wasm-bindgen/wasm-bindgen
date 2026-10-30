//! Owned vectors whose elements own memory or JS handles, whose glue calls
//! into Wasm for each element. Each wrapper crosses as the type it wraps,
//! with its vectors encoded.

use wasm_bindgen::convert::{
    FromWasmAbi, IntoWasmAbi, VectorFromWasmAbi, VectorIntoWasmAbi, WasmSlice,
};
use wasm_bindgen::describe::{inform, WasmDescribe, WasmDescribeVector, VECTOR};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use core::sync::atomic::{AtomicU32, Ordering};

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
    MaybePoint(Option<StagedPoint>);
    NullablePoint(Nullable<StagedPoint>);
    MaybeNullablePoint(Option<Nullable<StagedPoint>>);
    MaybeByte(Option<u8>);
}

/// How many `StagedPoint`s are alive in Rust, to tell a point JS handed over
/// from one it leaked.
static LIVE_POINTS: AtomicU32 = AtomicU32::new(0);

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct StagedPoint(u32);

#[wasm_bindgen]
impl StagedPoint {
    #[wasm_bindgen(constructor)]
    pub fn new(value: u32) -> StagedPoint {
        LIVE_POINTS.fetch_add(1, Ordering::Relaxed);
        StagedPoint(value)
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> u32 {
        self.0
    }

    #[wasm_bindgen(js_name = liveCount)]
    pub fn live_count() -> u32 {
        LIVE_POINTS.load(Ordering::Relaxed)
    }
}

impl Drop for StagedPoint {
    fn drop(&mut self) {
        LIVE_POINTS.fetch_sub(1, Ordering::Relaxed);
    }
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
    #[wasm_bindgen(js_name = pointRoundTrips)]
    fn point_round_trips();
    #[wasm_bindgen(js_name = rejectedPoints)]
    fn rejected_points();
    #[wasm_bindgen(js_name = jsMaybePoints)]
    fn js_maybe_points(v: Vec<MaybePoint>) -> Vec<MaybePoint>;
    #[wasm_bindgen(js_name = jsNullablePoints)]
    fn js_nullable_points(v: Vec<NullablePoint>) -> Vec<NullablePoint>;
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
pub fn staged_maybe_points(v: Vec<MaybePoint>) -> Vec<MaybePoint> {
    v
}

#[wasm_bindgen]
pub fn staged_nullable_points(v: Vec<NullablePoint>) -> Vec<NullablePoint> {
    v
}

#[wasm_bindgen]
pub fn staged_maybe_nullable_points(v: Vec<MaybeNullablePoint>) -> Vec<MaybeNullablePoint> {
    v
}

#[wasm_bindgen]
pub fn staged_points_then_names(_a: Vec<MaybePoint>, _b: Vec<Name>) {}

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

#[wasm_bindgen_test]
fn point_exports() {
    point_round_trips();
}

#[wasm_bindgen_test]
fn rejected_point_elements() {
    rejected_points();
}

#[wasm_bindgen_test]
fn point_imports() {
    let live = StagedPoint::live_count();
    let points = js_maybe_points(vec![
        MaybePoint(Some(StagedPoint::new(1))),
        MaybePoint(None),
        MaybePoint(Some(StagedPoint::new(2))),
    ]);
    let values = |points: &[MaybePoint]| -> Vec<Option<u32>> {
        points.iter().map(|p| p.0.as_ref().map(|p| p.0)).collect()
    };
    assert_eq!(values(&points), [Some(2), None, Some(1), Some(3)]);
    // JS handed back the two points it was given and one it made.
    assert_eq!(StagedPoint::live_count(), live + 3);
    drop(points);
    assert_eq!(StagedPoint::live_count(), live);

    let points = js_nullable_points(vec![
        NullablePoint(Nullable::Value(StagedPoint::new(1))),
        NullablePoint(Nullable::Null),
    ]);
    let values: Vec<Option<u32>> = points
        .iter()
        .map(|p| p.0.as_ref().value().map(|p| p.0))
        .collect();
    assert_eq!(values, [None, Some(1)]);
    assert_eq!(StagedPoint::live_count(), live + 1);
    drop(points);
    assert_eq!(StagedPoint::live_count(), live);
}
