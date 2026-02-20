use crate::generated::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern "C" {
    fn assert_dict_c(c: &C);
    #[wasm_bindgen(js_name = assert_dict_c)]
    fn assert_dict_c2(c: C);
    #[wasm_bindgen(js_name = assert_dict_c)]
    fn assert_dict_c3(c: Option<&C>);
    #[wasm_bindgen(js_name = assert_dict_c)]
    fn assert_dict_c4(c: Option<C>);
    fn mk_dict_a() -> A;
    #[wasm_bindgen(js_name = mk_dict_a)]
    fn mk_dict_a2() -> Option<A>;
    fn assert_dict_required(r: &Required);
    fn assert_camel_case(dict: &PreserveNames);
}

#[wasm_bindgen_test]
fn smoke() {
    let a = A::new();
    a.set_c(1);
    a.set_g(2);
    a.set_h(3);
    a.set_d(4);

    let b = B::new();
    b.set_c(1);
    b.set_g(2);
    b.set_h(3);
    b.set_d(4);
    b.set_a(5);
    b.set_b(6);

    let c = C::new();
    c.set_a(1);
    c.set_b(2);
    c.set_c(3);
    c.set_d(4);
    c.set_e(5);
    c.set_f(6);
    c.set_g(7);
    c.set_h(8);
    assert_dict_c(&c);
    assert_dict_c2(c.clone());
    assert_dict_c3(Some(&c));
    assert_dict_c4(Some(c));
}

#[wasm_bindgen_test]
fn get_dict() {
    mk_dict_a();
    assert!(mk_dict_a2().is_some());
}

#[wasm_bindgen_test]
fn casing() {
    CamelCaseMe::new().set_snake_case_me(3);
}

#[wasm_bindgen_test]
fn many_types() {
    ManyTypes::new().set_a("a");
}

#[wasm_bindgen_test]
fn required() {
    let dict = Required::new(3, "a");
    dict.set_c(4);
    assert_dict_required(&dict);
}

#[wasm_bindgen_test]
fn correct_casing_in_js() {
    let dict = PreserveNames::new();
    dict.set_weird_field_name(1);
    assert_camel_case(&dict);
}

#[wasm_bindgen_test]
fn nullable_sequence_field() {
    // Regression test: nullable sequence dictionary fields should compile.
    // The builder pattern previously broke due to is_js_value_ref_option_type
    // type mismatch between the setter and the unwrap_or call.
    let dict = DictWithNullableSequence::new();
    #[cfg(not(wbg_next_unstable))]
    dict.set_nullable_sequence(&JsValue::NULL);
    #[cfg(wbg_next_unstable)]
    dict.set_nullable_sequence(None);
}
