//! A type crossing through hand-written impls of the conversion traits, in
//! the shape `tsify`'s derive emits: each delegates to an imported JS type,
//! the vector traits and `OptionIntoWasmAbi` / `OptionFromWasmAbi` included.
//! Bindings written before sequences crossed by their element's own ABI must
//! keep compiling as they are, and keep crossing.

use core::ops::Deref;

use wasm_bindgen::convert::{
    FromWasmAbi, IntoWasmAbi, OptionFromWasmAbi, OptionIntoWasmAbi, RefFromWasmAbi,
    VectorFromWasmAbi, VectorIntoWasmAbi,
};
use wasm_bindgen::describe::{WasmDescribe, WasmDescribeVector};
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(typescript_type = "Point")]
    pub type JsType;
}

#[wasm_bindgen(module = "tests/wasm/third_party_abi.js")]
extern "C" {
    #[wasm_bindgen(js_name = makePoint)]
    fn make_point(x: f64, y: f64) -> JsType;
    #[wasm_bindgen(js_name = pointX)]
    fn point_x(point: &JsType) -> f64;
    #[wasm_bindgen(js_name = pointY)]
    fn point_y(point: &JsType) -> f64;

    #[wasm_bindgen(js_name = shiftPoint)]
    fn shift_point(point: Point) -> Point;
    #[wasm_bindgen(js_name = shiftOptPoint)]
    fn shift_opt_point(point: Option<Point>) -> Option<Point>;
    #[wasm_bindgen(js_name = shiftPoints)]
    fn shift_points(points: Vec<Point>) -> Vec<Point>;
    #[wasm_bindgen(js_name = shiftOptPoints)]
    fn shift_opt_points(points: Option<Vec<Point>>) -> Option<Vec<Point>>;
    #[wasm_bindgen(js_name = sumPointRef)]
    fn sum_point_ref(point: &Point) -> f64;
    #[wasm_bindgen(js_name = sumPointSlice)]
    fn sum_point_slice(points: &[Point]) -> f64;
    #[wasm_bindgen(js_name = callExports)]
    fn call_exports();
}

impl Point {
    fn to_js(&self) -> JsType {
        make_point(self.x, self.y)
    }

    fn from_js(js: &JsType) -> Self {
        Point {
            x: point_x(js),
            y: point_y(js),
        }
    }
}

impl WasmDescribe for Point {
    #[inline]
    fn describe() {
        JsType::describe()
    }
}

impl WasmDescribeVector for Point {
    #[inline]
    fn describe_vector() {
        JsType::describe_vector()
    }
}

impl IntoWasmAbi for &Point {
    type Abi = <JsType as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        self.to_js().into_abi()
    }
}

impl IntoWasmAbi for Point {
    type Abi = <JsType as IntoWasmAbi>::Abi;

    #[inline]
    fn into_abi(self) -> Self::Abi {
        (&self).into_abi()
    }
}

impl OptionIntoWasmAbi for Point {
    #[inline]
    fn none() -> Self::Abi {
        <JsType as OptionIntoWasmAbi>::none()
    }
}

impl From<Point> for JsValue {
    #[inline]
    fn from(value: Point) -> Self {
        value.to_js().into()
    }
}

impl VectorIntoWasmAbi for Point {
    type Abi = <JsType as VectorIntoWasmAbi>::Abi;

    #[inline]
    fn vector_into_abi(vector: Box<[Self]>) -> Self::Abi {
        let values = vector.iter().map(|value| value.to_js().into()).collect();

        JsValue::vector_into_abi(values)
    }
}

impl FromWasmAbi for Point {
    type Abi = <JsType as FromWasmAbi>::Abi;

    #[inline]
    unsafe fn from_abi(js: Self::Abi) -> Self {
        Self::from_js(&JsType::from_abi(js))
    }
}

impl OptionFromWasmAbi for Point {
    #[inline]
    fn is_none(js: &Self::Abi) -> bool {
        <JsType as OptionFromWasmAbi>::is_none(js)
    }
}

pub struct SelfOwner<T>(T);

impl<T> Deref for SelfOwner<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RefFromWasmAbi for Point {
    type Abi = <JsType as RefFromWasmAbi>::Abi;

    type Anchor = SelfOwner<Self>;

    unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
        SelfOwner(Self::from_js(&*JsType::ref_from_abi(js)))
    }
}

impl VectorFromWasmAbi for Point {
    type Abi = <JsType as VectorFromWasmAbi>::Abi;

    #[inline]
    unsafe fn vector_from_abi(js: Self::Abi) -> Box<[Self]> {
        JsValue::vector_from_abi(js)
            .iter()
            .map(|value| Self::from_js(value.unchecked_ref()))
            .collect()
    }
}

#[wasm_bindgen]
pub fn third_party_point(point: Point) -> Point {
    Point {
        x: point.x * 2.,
        y: point.y * 2.,
    }
}

#[wasm_bindgen]
pub fn third_party_opt_point(point: Option<Point>) -> Option<Point> {
    point.map(third_party_point)
}

#[wasm_bindgen]
pub fn third_party_points(points: Vec<Point>) -> Vec<Point> {
    points.into_iter().map(third_party_point).collect()
}

#[wasm_bindgen]
pub fn third_party_opt_points(points: Option<Vec<Point>>) -> Option<Vec<Point>> {
    points.map(third_party_points)
}

#[wasm_bindgen]
pub fn third_party_point_ref(point: &Point) -> f64 {
    point.x + point.y
}

pub(crate) fn point(x: f64, y: f64) -> Point {
    Point { x, y }
}

#[wasm_bindgen_test]
fn imports() {
    assert_eq!(shift_point(point(1., 2.)), point(2., 3.));
    assert_eq!(shift_opt_point(Some(point(1., 2.))), Some(point(2., 3.)));
    assert_eq!(shift_opt_point(None), None);
    assert_eq!(
        shift_points(vec![point(1., 2.), point(3., 4.)]),
        [point(2., 3.), point(4., 5.)]
    );
    assert_eq!(shift_points(vec![]), []);
    assert_eq!(
        shift_opt_points(Some(vec![point(1., 2.)])),
        Some(vec![point(2., 3.)])
    );
    assert_eq!(shift_opt_points(None), None);
    assert_eq!(sum_point_ref(&point(1., 2.)), 3.);
    assert_eq!(sum_point_slice(&[point(1., 2.), point(3., 4.)]), 10.);
}

#[wasm_bindgen_test]
fn exports() {
    call_exports();
}

#[wasm_bindgen_test]
fn into_js_value() {
    let array = JsValue::from(vec![point(1., 2.), point(3., 4.)]);
    assert_eq!(sum_points_value(&array), 10.);
}

#[wasm_bindgen(module = "tests/wasm/third_party_abi.js")]
extern "C" {
    #[wasm_bindgen(js_name = sumPointSlice)]
    fn sum_points_value(points: &JsValue) -> f64;
}

// The vector traits still do what they did when called directly.
#[wasm_bindgen_test]
fn vector_traits() {
    let abi = Point::vector_into_abi(vec![point(1., 2.), point(3., 4.)].into_boxed_slice());
    let points = unsafe { Point::vector_from_abi(abi) };
    assert_eq!(&*points, [point(1., 2.), point(3., 4.)]);
}
