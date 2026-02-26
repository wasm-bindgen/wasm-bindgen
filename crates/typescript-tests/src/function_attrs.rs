use wasm_bindgen::prelude::*;

/// Description for fn_with_attr
#[wasm_bindgen(
    unchecked_return_type = "number",
    return_description = "returns 1 if arg2 is true, or arg1 if arg2 is undefined or false"
)]
pub async fn fn_with_attr(
    #[wasm_bindgen(js_name = "firstArg", param_description = "some number")] arg1: u32,
    #[wasm_bindgen(js_name = "secondArg", unchecked_param_type = "boolean | undefined")]
    arg2: JsValue,
) -> Result<JsValue, JsValue> {
    if arg2.is_undefined() {
        Ok(arg1.into())
    } else if arg2.is_truthy() {
        Ok(1u32.into())
    } else {
        Ok(arg1.into())
    }
}

/// Description for HoldsNumber
#[wasm_bindgen]
pub struct HoldsNumber {
    inner: JsValue,
}

#[wasm_bindgen]
impl HoldsNumber {
    /// Inner value
    #[wasm_bindgen(getter = "inner", unchecked_return_type = "number")]
    pub fn get_inner(&self) -> JsValue {
        self.inner.clone()
    }

    /// Description for static_fn_with_attr
    #[wasm_bindgen(
        return_description = "returns an instance of HoldsNumber, holding arg1 if arg2 is undefined and holding arg2 if not"
    )]
    pub fn static_fn_with_attr(
        #[wasm_bindgen(js_name = "firstArg", param_description = "some number")] arg1: u32,
        #[wasm_bindgen(js_name = "secondArg", unchecked_param_type = "number | undefined")]
        arg2: JsValue,
    ) -> HoldsNumber {
        if arg2.is_undefined() {
            HoldsNumber { inner: arg1.into() }
        } else {
            HoldsNumber { inner: arg2 }
        }
    }

    /// Description for method_with_attr
    #[wasm_bindgen(
        unchecked_return_type = "number",
        return_description = "returns arg1 if arg2 is true, or holding value of self if arg2 is undefined or false"
    )]
    pub fn method_with_attr(
        &self,
        #[wasm_bindgen(js_name = "firstArg", param_description = "some number")] arg1: u32,
        #[wasm_bindgen(js_name = "secondArg", unchecked_param_type = "boolean | undefined")]
        arg2: JsValue,
    ) -> JsValue {
        if arg2.is_undefined() {
            self.inner.clone()
        } else if arg2.is_truthy() {
            arg1.into()
        } else {
            self.inner.clone()
        }
    }
}

/// Test function for unchecked_optional_param_type
#[wasm_bindgen]
pub fn fn_with_optional_param(
    #[wasm_bindgen(unchecked_optional_param_type = "string")] name: JsValue,
) -> JsValue {
    if name.is_undefined() {
        "Hello, World!".into()
    } else {
        format!("Hello, {}!", name.as_string().unwrap_or_default()).into()
    }
}

/// Test function combining optional param with other attributes
#[wasm_bindgen]
pub fn fn_with_optional_param_and_attrs(
    #[wasm_bindgen(
        js_name = "userName",
        unchecked_optional_param_type = "string",
        param_description = "the name to greet"
    )]
    name: JsValue,
) -> JsValue {
    if name.is_undefined() {
        "Hello, World!".into()
    } else {
        format!("Hello, {}!", name.as_string().unwrap_or_default()).into()
    }
}
