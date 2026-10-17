//! A shared reference to a primitive crosses to an import as the value.

use wasm_bindgen::convert::IntoWasmAbi;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(inline_js = "export function describe(v) { return `${typeof v} ${v}`; }")]
extern "C" {
    #[wasm_bindgen(js_name = describe)]
    fn describe_f64(v: &f64) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn describe_char(v: &char) -> String;
    #[wasm_bindgen(js_name = describe)]
    fn describe_u128(v: &u128) -> String;

    #[wasm_bindgen(experimental_generic_mono, js_name = describe)]
    fn describe<T>(v: &T) -> String
    where
        for<'a> &'a T: IntoWasmAbi;
}

#[wasm_bindgen_test]
fn primitive_refs_cross_as_values() {
    assert_eq!(describe_f64(&1.5), "number 1.5");
    assert_eq!(describe_char(&'x'), "string x");
    assert_eq!(describe_u128(&u128::MAX), format!("bigint {}", u128::MAX));

    assert_eq!(describe(&true), "boolean true");
    assert_eq!(describe(&'y'), "string y");
    assert_eq!(describe(&-3i8), "number -3");
    assert_eq!(describe(&u32::MAX), format!("number {}", u32::MAX));
    assert_eq!(describe(&7usize), "number 7");
    assert_eq!(describe(&2.5f32), "number 2.5");
    assert_eq!(describe(&i64::MIN), format!("bigint {}", i64::MIN));
    assert_eq!(describe(&i128::MIN), format!("bigint {}", i128::MIN));
}
