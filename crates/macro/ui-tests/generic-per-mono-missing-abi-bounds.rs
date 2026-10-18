use wasm_bindgen::convert::FromWasmAbi;
use wasm_bindgen::prelude::*;

// The ABI bounds an `experimental_generic_mono` import needs are part of its
// declared contract and are not synthesized. Each import below omits the bound
// for one position, which must be reported at that position in the
// declaration, with a suggestion naming the missing bound.
#[wasm_bindgen]
extern "C" {
    // Needs `T: IntoWasmAbi`.
    #[wasm_bindgen(experimental_generic_mono)]
    fn argument<T>(x: T);

    // Needs `T: RefIntoWasmAbi`.
    #[wasm_bindgen(experimental_generic_mono)]
    fn reference_argument<T>(x: &T);

    // The suggested `where` clause follows the whole declared return type.
    #[wasm_bindgen(experimental_generic_mono, catch)]
    fn reference_argument_catch<T>(x: &T) -> Result<Option<u32>, JsValue>;

    // Needs `T: FromWasmAbi`.
    #[wasm_bindgen(experimental_generic_mono)]
    fn ret<T>() -> T;

    // Needs `T: FromWasmAbi + 'static`.
    #[wasm_bindgen(experimental_generic_mono)]
    async fn async_return<T>() -> T;

    #[wasm_bindgen(experimental_generic_mono)]
    async fn async_return_static<T: FromWasmAbi>() -> T;

    // Needs `A: FromWasmAbi`.
    #[wasm_bindgen(experimental_generic_mono)]
    fn callback_argument<A>(f: &dyn Fn(A));

    // Needs `R: IntoWasmAbi`.
    #[wasm_bindgen(experimental_generic_mono)]
    fn callback_return<R>(f: &dyn Fn() -> R);
}

fn main() {}
