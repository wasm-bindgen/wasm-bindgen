use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Value;
    type Holder<T = JsValue>;

    // The type-erasure path must preserve the explicit class argument and
    // hoist `T` onto the impl, just as it does for instance methods.
    #[wasm_bindgen(static_method_of = Holder<T>)]
    fn create<T>(value: T);
}

fn assert_method_is_on_parameterized_class(value: Value) {
    Holder::<Value>::create(value);
}

fn main() {}
