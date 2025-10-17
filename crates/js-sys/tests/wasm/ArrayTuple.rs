use js_sys::{Array, ArrayTuple, JsString, Number};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn new_1() {
    let tuple: ArrayTuple<JsString> = ArrayTuple::new1(&JsString::from("first"));
    assert_eq!(tuple.get0(), "first");
}

#[wasm_bindgen_test]
fn new_2() {
    let tuple: ArrayTuple<JsString, Number> =
        ArrayTuple::new2(&JsString::from("a"), &Number::from(42));
    assert_eq!(tuple.get0(), "a");
    assert_eq!(tuple.get1(), 42);
}

#[wasm_bindgen_test]
fn new_3() {
    let tuple: ArrayTuple<JsString, JsString, JsString> = ArrayTuple::new3(
        &JsString::from("x"),
        &JsString::from("y"),
        &JsString::from("z"),
    );
    assert_eq!(tuple.get0(), "x");
    assert_eq!(tuple.get1(), "y");
    assert_eq!(tuple.get2(), "z");
}

#[wasm_bindgen_test]
fn new_4() {
    let tuple = ArrayTuple::new4(
        &JsString::from("a"),
        &JsString::from("b"),
        &JsString::from("c"),
        &JsString::from("d"),
    );
    assert_eq!(tuple.get0(), "a");
    assert_eq!(tuple.get3(), "d");
}

#[wasm_bindgen_test]
fn new_5() {
    let tuple = ArrayTuple::new5(
        &JsString::from("1"),
        &JsString::from("2"),
        &JsString::from("3"),
        &JsString::from("4"),
        &JsString::from("5"),
    );
    assert_eq!(tuple.get0(), "1");
    assert_eq!(tuple.get4(), "5");
}

#[wasm_bindgen_test]
fn new_6() {
    let tuple = ArrayTuple::new6(
        &Number::from(1),
        &Number::from(2),
        &Number::from(3),
        &Number::from(4),
        &Number::from(5),
        &Number::from(6),
    );
    assert_eq!(tuple.get0(), 1);
    assert_eq!(tuple.get5(), 6);
}

#[wasm_bindgen_test]
fn new_7() {
    let tuple = ArrayTuple::new7(
        &JsString::from("a"),
        &JsString::from("b"),
        &JsString::from("c"),
        &JsString::from("d"),
        &JsString::from("e"),
        &JsString::from("f"),
        &JsString::from("g"),
    );
    assert_eq!(tuple.get0(), "a");
    assert_eq!(tuple.get6(), "g");
}

#[wasm_bindgen_test]
fn new_8() {
    let tuple = ArrayTuple::new8(
        &Number::from(1),
        &Number::from(2),
        &Number::from(3),
        &Number::from(4),
        &Number::from(5),
        &Number::from(6),
        &Number::from(7),
        &Number::from(8),
    );
    assert_eq!(tuple.get0(), 1);
    assert_eq!(tuple.get7(), 8);
}

#[wasm_bindgen_test]
fn new_9() {
    let tuple = ArrayTuple::new9(
        &JsString::from("1"),
        &JsString::from("2"),
        &JsString::from("3"),
        &JsString::from("4"),
        &JsString::from("5"),
        &JsString::from("6"),
        &JsString::from("7"),
        &JsString::from("8"),
        &JsString::from("9"),
    );
    assert_eq!(tuple.get0(), "1");
    assert_eq!(tuple.get8(), "9");
}

#[wasm_bindgen_test]
fn mixed_types() {
    let tuple: ArrayTuple<JsString, Number, JsString> = ArrayTuple::new3(
        &JsString::from("text"),
        &Number::from(123),
        &JsString::from("more"),
    );
    assert_eq!(tuple.get0(), "text");
    assert_eq!(tuple.get1(), 123);
    assert_eq!(tuple.get2(), "more");
}

#[wasm_bindgen_test]
fn set_values() {
    let tuple: ArrayTuple<JsString, Number> =
        ArrayTuple::new2(&JsString::from("a"), &Number::from(1));
    tuple.set0(&JsString::from("changed"));
    tuple.set1(&Number::from(99));
    assert_eq!(tuple.get0(), "changed");
    assert_eq!(tuple.get1(), 99);
}

#[wasm_bindgen_test]
fn tuple_extends_array() {
    let tuple: ArrayTuple<JsString, JsString> =
        ArrayTuple::new2(&JsString::from("a"), &JsString::from("b"));
    let arr: &Array = tuple.unchecked_ref();
    assert_eq!(arr.length(), 2);
}
