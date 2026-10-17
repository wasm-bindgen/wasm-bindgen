#![deny(warnings, unused_qualifications, trivial_casts, trivial_numeric_casts)]
#![allow(dead_code)]

use wasm_bindgen::convert::{FromWasmAbi, IntoWasmAbi, ReturnWasmAbi};
use wasm_bindgen::prelude::*;

// The declared ABI bounds for each position are the whole contract: generic
// callers that state the same bounds can forward to the import.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(experimental_generic_mono)]
    fn argument<T: IntoWasmAbi>(x: T);

    #[wasm_bindgen(experimental_generic_mono)]
    fn reference_argument<T>(x: &T)
    where
        for<'a> &'a T: IntoWasmAbi;

    #[wasm_bindgen(experimental_generic_mono)]
    fn ret<T: FromWasmAbi>() -> T;

    #[wasm_bindgen(experimental_generic_mono, catch)]
    fn ret_catch<T: FromWasmAbi>() -> Result<T, JsValue>;

    #[wasm_bindgen(experimental_generic_mono)]
    async fn async_return<T: FromWasmAbi + 'static>() -> T;

    #[wasm_bindgen(experimental_generic_mono)]
    fn callback_argument<A: FromWasmAbi>(f: &dyn Fn(A));

    #[wasm_bindgen(experimental_generic_mono)]
    fn callback_return<R: IntoWasmAbi>(f: &dyn Fn() -> R);

    #[wasm_bindgen(experimental_generic_mono)]
    fn callback_result_return<R>(f: &dyn Fn() -> Result<R, JsValue>)
    where
        Result<R, JsValue>: ReturnWasmAbi;

    #[wasm_bindgen(experimental_generic_mono)]
    fn option<T>(x: Option<T>) -> Option<T>
    where
        Option<T>: IntoWasmAbi + FromWasmAbi;
}

fn forward<T: IntoWasmAbi + FromWasmAbi>(x: T) -> T {
    argument(x);
    ret()
}

fn forward_reference<T>(x: &T)
where
    for<'a> &'a T: IntoWasmAbi,
{
    reference_argument(x);
}

async fn forward_async<T: FromWasmAbi + 'static>() -> T {
    async_return().await
}

fn concrete() {
    argument(1u32);
    reference_argument(&JsValue::NULL);
    let _: String = ret();
    let _ = ret_catch::<u32>();
    callback_argument(&|_: u32| {});
    callback_return(&|| 1u32);
    callback_result_return(&|| Ok(1u32));
    let _ = option(Some(1u32));
}

fn main() {}
