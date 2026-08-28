use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type Value;
    type Qualified<T = JsValue>;

    // `self::Qualified` names the same local imported class as `Qualified` and
    // must still trigger self-returning static-method hoisting.
    #[wasm_bindgen(static_method_of = Qualified, experimental_generic_mono, js_name = of)]
    fn qualified_static<T>(value: T) -> self::Qualified<T>;
}

fn assert_qualified_return(value: Value) -> Qualified<Value> {
    Qualified::<Value>::qualified_static(value)
}

fn main() {}
