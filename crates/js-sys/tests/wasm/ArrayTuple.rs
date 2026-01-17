use js_sys::{Array, ArrayTuple, Function, JsString, Number, Promise, TypedFunction};
use wasm_bindgen::{JsCast, JsValue};
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

#[wasm_bindgen_test]
fn first_method() {
    let tuple1: ArrayTuple<JsString> = ArrayTuple::new1(&JsString::from("first"));
    assert_eq!(tuple1.first(), "first");

    let tuple2: ArrayTuple<JsString, Number> =
        ArrayTuple::new2(&JsString::from("one"), &Number::from(2));
    assert_eq!(tuple2.first(), "one");

    let tuple3: ArrayTuple<Number, JsString, JsString> = ArrayTuple::new3(
        &Number::from(42),
        &JsString::from("middle"),
        &JsString::from("end"),
    );
    assert_eq!(tuple3.first(), 42);
}

#[wasm_bindgen_test]
fn last_method() {
    let tuple1: ArrayTuple<JsString> = ArrayTuple::new1(&JsString::from("only"));
    assert_eq!(tuple1.last(), "only");

    let tuple2: ArrayTuple<Number, JsString> =
        ArrayTuple::new2(&Number::from(1), &JsString::from("last"));
    assert_eq!(tuple2.last(), "last");

    let tuple3: ArrayTuple<JsString, Number, JsString> = ArrayTuple::new3(
        &JsString::from("first"),
        &Number::from(2),
        &JsString::from("final"),
    );
    assert_eq!(tuple3.last(), "final");

    let tuple5 = ArrayTuple::new5(
        &Number::from(1),
        &Number::from(2),
        &Number::from(3),
        &Number::from(4),
        &Number::from(999),
    );
    assert_eq!(tuple5.last(), 999);

    let tuple9 = ArrayTuple::new9(
        &JsString::from("a"),
        &JsString::from("b"),
        &JsString::from("c"),
        &JsString::from("d"),
        &JsString::from("e"),
        &JsString::from("f"),
        &JsString::from("g"),
        &JsString::from("h"),
        &JsString::from("ninth"),
    );
    assert_eq!(tuple9.last(), "ninth");
}

#[wasm_bindgen_test]
fn into_parts_method() {
    let tuple1: ArrayTuple<JsString> = ArrayTuple::new1(&JsString::from("single"));
    let (a,) = tuple1.into_parts();
    assert_eq!(a, "single");

    let tuple2: ArrayTuple<JsString, Number> =
        ArrayTuple::new2(&JsString::from("hello"), &Number::from(42));
    let (first, second) = tuple2.into_parts();
    assert_eq!(first, "hello");
    assert_eq!(second, 42);

    let tuple3: ArrayTuple<Number, JsString, Number> = ArrayTuple::new3(
        &Number::from(1),
        &JsString::from("middle"),
        &Number::from(3),
    );
    let (a, b, c) = tuple3.into_parts();
    assert_eq!(a, 1);
    assert_eq!(b, "middle");
    assert_eq!(c, 3);

    let tuple5 = ArrayTuple::new5(
        &JsString::from("a"),
        &JsString::from("b"),
        &JsString::from("c"),
        &JsString::from("d"),
        &JsString::from("e"),
    );
    let (v1, v2, v3, v4, v5) = tuple5.into_parts();
    assert_eq!(v1, "a");
    assert_eq!(v2, "b");
    assert_eq!(v3, "c");
    assert_eq!(v4, "d");
    assert_eq!(v5, "e");

    let tuple9 = ArrayTuple::new9(
        &Number::from(1),
        &Number::from(2),
        &Number::from(3),
        &Number::from(4),
        &Number::from(5),
        &Number::from(6),
        &Number::from(7),
        &Number::from(8),
        &Number::from(9),
    );
    let (n1, n2, n3, n4, n5, n6, n7, n8, n9) = tuple9.into_parts();
    assert_eq!(n1, 1);
    assert_eq!(n2, 2);
    assert_eq!(n3, 3);
    assert_eq!(n4, 4);
    assert_eq!(n5, 5);
    assert_eq!(n6, 6);
    assert_eq!(n7, 7);
    assert_eq!(n8, 8);
    assert_eq!(n9, 9);
}

#[wasm_bindgen_test]
fn covariance_to_array() {
    use wasm_bindgen::prelude::Upcast;

    fn accepts_number_array(arr: Array<Number>) -> u32 {
        arr.length()
    }

    let tuple: ArrayTuple<Number, Number> = ArrayTuple::new2(&Number::from(42), &Number::from(100));

    let length = accepts_number_array(tuple.upcast());
    assert_eq!(length, 2);
}

#[wasm_bindgen_test]
fn complex_nested_covariance() {
    use wasm_bindgen::prelude::Upcast;

    let func: TypedFunction<Number> = Function::new_no_args_typed("return 42");

    let promise_func: Promise<TypedFunction<Number>> = Promise::resolve(&func);

    let num: Number = Number::from(42);
    let num_as_jsvalue: JsValue = num.upcast();
    assert!(num_as_jsvalue.as_f64().is_some());

    let func_num: TypedFunction<Number> = Function::new_no_args_typed("return 42");
    let func_jsvalue: TypedFunction<JsValue> = func_num.upcast();
    assert!(func_jsvalue.is_function());

    let promise_func_num: Promise<Function<Number>> = Promise::resolve(&func).upcast();
    let promise_func_jsvalue: Promise<Function<JsValue>> = promise_func_num.upcast();
    assert!(promise_func_jsvalue.is_object());

    let tuple_complex: ArrayTuple<Promise<Function<Number>>, Number> =
        ArrayTuple::new2(promise_func.upcast_ref(), &Number::from(100));
    let tuple_wider: ArrayTuple<Promise<Function>, JsValue> = tuple_complex.upcast();
    assert_eq!(tuple_wider.length(), 2);
}

#[wasm_bindgen_test]
fn arraytuple_to_array_mixed_types() {
    use wasm_bindgen::prelude::Upcast;

    let tuple: ArrayTuple<Number, JsString, Array> =
        ArrayTuple::new3(&Number::from(42), &JsString::from("hello"), &Array::new());

    let array: Array<JsValue> = tuple.upcast();
    assert_eq!(array.length(), 3);

    assert!(array.get(0).as_f64().is_some());
    assert!(array.get(1).is_string());
    assert!(Array::is_array(&array.get(2)));
}

#[wasm_bindgen_test]
fn arraytuple_to_array_single_type() {
    use wasm_bindgen::prelude::Upcast;

    let tuple1: ArrayTuple<Number> = ArrayTuple::new1(&Number::from(100));
    let array1: Array<JsValue> = tuple1.upcast();
    assert_eq!(array1.length(), 1);
    assert_eq!(array1.get(0).as_f64().unwrap(), 100.0);

    let tuple_strings: ArrayTuple<JsString, JsString, JsString> = ArrayTuple::new3(
        &JsString::from("a"),
        &JsString::from("b"),
        &JsString::from("c"),
    );
    let array_strings: Array<JsString> = tuple_strings.upcast();
    assert_eq!(array_strings.length(), 3);
    assert_eq!(array_strings.get(0).as_string().unwrap(), "a");
}

#[wasm_bindgen_test]
fn arraytuple_to_array_large_tuple() {
    use wasm_bindgen::prelude::Upcast;

    let tuple9: ArrayTuple<Number, Number, Number, Number, Number, Number, Number, Number, Number> =
        ArrayTuple::new9(
            &Number::from(1),
            &Number::from(2),
            &Number::from(3),
            &Number::from(4),
            &Number::from(5),
            &Number::from(6),
            &Number::from(7),
            &Number::from(8),
            &Number::from(9),
        );
    let array9: Array<Number> = tuple9.upcast();
    assert_eq!(array9.length(), 9);
    assert_eq!(array9.get(0).value_of(), 1.0);
    assert_eq!(array9.get(8).value_of(), 9.0);

    let tuple5: ArrayTuple<JsString, Number, Array, JsString, Number> = ArrayTuple::new5(
        &JsString::from("a"),
        &Number::from(2),
        &Array::new(),
        &JsString::from("d"),
        &Number::from(5),
    );
    let array5: Array<JsValue> = tuple5.upcast();
    assert_eq!(array5.length(), 5);
}

#[wasm_bindgen_test]
fn arraytuple_to_array_nested_generics() {
    use wasm_bindgen::prelude::Upcast;

    let num_array: Array<Number> = Array::new_typed();
    num_array.push(&Number::from(1));

    let str_array: Array<JsString> = Array::new_typed();
    str_array.push(&JsString::from("test"));

    let tuple: ArrayTuple<Array<Number>, Array<JsString>> =
        ArrayTuple::new2(&num_array, &str_array);

    let array_of_arrays: Array<Array<JsValue>> = tuple.upcast();
    assert_eq!(array_of_arrays.length(), 2);

    let first = array_of_arrays.get(0);
    assert_eq!(first.length(), 1);
}

#[wasm_bindgen_test]
fn arraytuple_identity_covariance() {
    use wasm_bindgen::prelude::Upcast;

    let tuple: ArrayTuple<Number, JsString> =
        ArrayTuple::new2(&Number::from(42), &JsString::from("test"));

    let same: ArrayTuple<Number, JsString> = tuple.upcast();
    assert_eq!(same.length(), 2);
    assert_eq!(same.get0(), 42);
    assert_eq!(same.get1(), "test");
}
