use wasm_bindgen::prelude::*;

trait Marker<T> {}

#[wasm_bindgen]
extern "C" {
    type Value;
    type Shaped<T>;

    // These all determine `T` from the impl self type and therefore satisfy
    // rustc's constrained-impl-parameter rule.
    #[wasm_bindgen(method, experimental_generic_mono, js_name = raw)]
    fn raw_pointer<T>(this: &Shaped<*const T>);

    #[wasm_bindgen(method, experimental_generic_mono, js_name = function)]
    fn function_pointer<T>(this: &Shaped<fn(T)>);

    #[wasm_bindgen(method, experimental_generic_mono, js_name = dynamic)]
    fn trait_object<T>(this: &Shaped<Box<dyn Marker<T>>>);
}

fn assert_constraining_shapes(
    raw: &Shaped<*const Value>,
    function: &Shaped<fn(Value)>,
    dynamic: &Shaped<Box<dyn Marker<Value>>>,
) {
    raw.raw_pointer();
    function.function_pointer();
    dynamic.trait_object();
}

fn main() {}
