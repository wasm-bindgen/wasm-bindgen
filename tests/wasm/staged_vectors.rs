//! Vectors whose elements own memory or JS handles, or are sequences
//! themselves, whose glue calls into Wasm for each element. Each wrapper
//! crosses as the type it wraps.

use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi, WasmSlice};
use wasm_bindgen::describe::WasmDescribe;
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
    )*)
}

encoded! {
    Name(String);
    MaybeName(Option<String>);
    MaybeObj(Option<JsValue>);
    NullableObj(Nullable<JsValue>);
    MaybePoint(Option<StagedPoint>);
    NullablePoint(Nullable<StagedPoint>);
    MaybeNullablePoint(Nullable<Option<StagedPoint>>);
    MaybeByte(Option<u8>);
    Names(Vec<Name>);
    MaybeNames(Option<Vec<MaybeName>>);
    Objs(Vec<MaybeObj>);
}

/// A row lent to JS as `&[&str]`, so a slice of rows is a sequence of
/// sequences that gives nothing.
pub struct Row(Vec<&'static str>);

impl WasmDescribe for Row {
    fn describe() {
        <&[&str]>::describe()
    }
}

impl IntoWasmAbi for &Row {
    type Abi = WasmSlice;

    fn into_abi(self) -> WasmSlice {
        self.0.as_slice().into_abi()
    }
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
    #[wasm_bindgen(js_name = jsNestedNames)]
    fn js_nested_names(v: Vec<Names>) -> Vec<Names>;
    #[wasm_bindgen(js_name = jsNestedObjs)]
    fn js_nested_objs(v: Vec<Objs>) -> Vec<Objs>;
    #[wasm_bindgen(js_name = jsRows)]
    fn js_rows(v: &[Row]) -> String;
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
pub fn staged_nested_names(v: Vec<Names>) -> Vec<Names> {
    v
}

#[wasm_bindgen]
pub fn staged_nested_maybe_names(v: Vec<MaybeNames>) -> Vec<MaybeNames> {
    v
}

#[wasm_bindgen]
pub fn staged_nested_objs(v: Vec<Objs>) -> Vec<Objs> {
    v
}

#[wasm_bindgen]
pub fn staged_nested_objs_then_names(_a: Vec<Objs>, _b: Vec<Name>) {}

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

#[wasm_bindgen_test]
fn nested_imports() {
    let names = |rows: &[&[&str]]| -> Vec<Names> {
        rows.iter()
            .map(|row| Names(row.iter().map(|name| Name(name.to_string())).collect()))
            .collect()
    };
    assert_eq!(
        js_nested_names(names(&[&["a"], &[], &["b", "c"]])),
        names(&[&["b", "c"], &[], &["a"], &["js"]])
    );

    let live = wasm_bindgen::externref_heap_live_count();
    let objs = js_nested_objs(vec![
        Objs(vec![MaybeObj(Some(JsValue::from(1))), MaybeObj(None)]),
        Objs(vec![]),
    ]);
    let values = |objs: &[Objs]| -> Vec<Vec<Option<f64>>> {
        objs.iter()
            .map(|row| {
                let row = row.0.iter();
                row.map(|obj| obj.0.as_ref().and_then(JsValue::as_f64))
                    .collect()
            })
            .collect()
    };
    assert_eq!(
        values(&objs),
        [vec![], vec![Some(1.0), None], vec![Some(2.0)]]
    );
    drop(objs);
    assert_eq!(wasm_bindgen::externref_heap_live_count(), live);

    let rows = [Row(vec!["a", "é"]), Row(vec![]), Row(vec!["b"])];
    assert_eq!(js_rows(&rows), r#"[["a","é"],[],["b"]]"#);
}
