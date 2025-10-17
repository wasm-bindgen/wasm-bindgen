use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/Thenable.js")]
extern "C" {
    #[wasm_bindgen(js_name = createNumericThenable)]
    fn create_numeric_thenable(value: f64) -> NumericThenable;

    // A function that accepts anything that promises a Number
    #[wasm_bindgen(js_name = processNumericPromising)]
    fn process_numeric_promising<T: Promising<Resolution = Number>>(value: T) -> Promise<JsString>;
}

// A custom thenable that resolves to Number
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(no_promising)]
    type NumericThenable;

    #[wasm_bindgen(method)]
    fn then(this: &NumericThenable, callback: &Function) -> Promise<JsValue>;
}

// Manually implement Promising to specify what this thenable resolves to
impl Promising for NumericThenable {
    type Resolution = Number;
}

#[wasm_bindgen_test]
async fn custom_thenable_works_with_promising_trait() {
    let thenable = create_numeric_thenable(42.0);

    // This demonstrates that NumericThenable can be used anywhere
    // a Promising<Output = Number> is expected
    let promise = process_numeric_promising(thenable);

    let result = JsFuture::from(promise).await.unwrap();
    let result_str: JsString = result.unchecked_into();

    assert_eq!(result_str, "Number: 42");
}

#[wasm_bindgen_test]
async fn promise_resolve_also_works() {
    // Promise.resolve also works because Promise<Number> implements
    // Promising<Output = Number>
    let promise = Promise::resolve(&Number::from(100));
    let processed = process_numeric_promising(promise);

    let result = JsFuture::from(processed).await.unwrap();
    let result_str: JsString = result.unchecked_into();

    assert_eq!(result_str, "Number: 100");
}
