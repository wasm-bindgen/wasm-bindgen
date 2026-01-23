use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/async_vecs.js")]
extern "C" {
    async fn js_works();
}

#[wasm_bindgen]
extern "C" {
    pub type RegExp;
    #[wasm_bindgen(constructor)]
    fn new(re: &str) -> RegExp;
}

#[wasm_bindgen]
pub async fn async_number_vec() -> Vec<i32> {
    vec![1, -3, 7, 12]
}

#[wasm_bindgen]
pub async fn async_jsvalue_vec() -> Vec<JsValue> {
    vec![
        1u32.into(),
        "hi".into(),
        Vec::<f64>::new().into(),
        JsValue::NULL,
    ]
}

#[wasm_bindgen]
pub async fn async_import_vec() -> Vec<RegExp> {
    vec![RegExp::new("hi|bye"), RegExp::new("hello w[a-z]rld")]
}

#[wasm_bindgen]
pub async fn async_string_vec() -> Vec<String> {
    vec!["a".to_owned(), "b".to_owned(), "c".to_owned()]
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct AnotherStruct;

#[wasm_bindgen]
pub async fn async_struct_vec() -> Vec<AnotherStruct> {
    vec![AnotherStruct; 2]
}

#[wasm_bindgen]
#[derive(Clone)]
pub enum AnotherEnum {
    A,
    B,
    C,
}

#[wasm_bindgen]
pub async fn async_enum_vec() -> Vec<AnotherEnum> {
    vec![AnotherEnum::C, AnotherEnum::A, AnotherEnum::B]
}

#[wasm_bindgen_test]
#[ignore]
async fn works() {
    js_works().await;
}
