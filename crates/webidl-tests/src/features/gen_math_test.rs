pub mod math_test {
    #![allow(unused_imports)]
    #![allow(clippy::all)]
    use super::super::*;
    use wasm_bindgen::prelude::*;
    #[wasm_bindgen]
    extern "C" {
        # [wasm_bindgen (js_name = math_test)]
        pub type JsNamespaceMathTest;
    }
    #[wasm_bindgen]
    extern "C" {
        # [wasm_bindgen (static_method_of = JsNamespaceMathTest , js_class = "math_test" , getter , js_name = PI)]
        #[doc = "Getter for the `math_test.PI` field."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/math_test/PI)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `math_test`*"]
        pub fn pi() -> f64;
        # [wasm_bindgen (js_namespace = math_test , js_name = add_one)]
        #[doc = "The `math_test.add_one()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/math_test/add_one)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `math_test`*"]
        pub fn add_one(val: i32) -> i32;
        # [wasm_bindgen (js_namespace = math_test , js_name = pow)]
        #[doc = "The `math_test.pow()` function."]
        #[doc = ""]
        #[doc = "[MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/API/math_test/pow)"]
        #[doc = ""]
        #[doc = "*This API requires the following crate features to be activated: `math_test`*"]
        pub fn pow(base: f64, exponent: f64) -> f64;
    }
}
