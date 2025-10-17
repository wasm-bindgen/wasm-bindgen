use js_sys::Number;
use wasm_bindgen::prelude::*;

fn main() {
    // Invalid: narrowing return type (Number -> i32)
    let closure: Closure<dyn Fn() -> Number> = Closure::new(|| Number::from(42));
    let _bad: &Closure<dyn Fn() -> i32> = closure.upcast_ref();

    // Invalid: widening arg type (i32 -> Number)
    let closure2: Closure<dyn Fn(i32)> = Closure::new(|_: i32| {});
    let _bad2: &Closure<dyn Fn(Number)> = closure2.upcast_ref();
}
