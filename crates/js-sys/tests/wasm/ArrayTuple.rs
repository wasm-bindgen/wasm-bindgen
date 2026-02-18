use js_sys::{Array, ArrayTuple, Function, JsString, Number, Promise};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn new_1() {
    let tuple = ArrayTuple::<(JsString,)>::new(&"first".into());
    assert_eq!(tuple.get0(), "first");
}

#[wasm_bindgen_test]
fn new_2() {
    let tuple = ArrayTuple::<(JsString, Number)>::new(&JsString::from("a"), &Number::from(42));
    assert_eq!(tuple.get0(), "a");
    assert_eq!(tuple.get1(), 42);
}

#[wasm_bindgen_test]
fn new_3() {
    let tuple = ArrayTuple::<(JsString, JsString, JsString)>::new(
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
    let tuple = ArrayTuple::<(JsString, JsString, JsString, JsString)>::new(
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
fn mixed_types() {
    let tuple = ArrayTuple::<(JsString, Number)>::new(&"text".into(), &123.into());
    assert_eq!(tuple.get0(), "text");
    assert_eq!(tuple.get1(), 123);
}

#[wasm_bindgen_test]
fn set_values() {
    let tuple: ArrayTuple<(JsString, Number)> =
        ArrayTuple::new2(&JsString::from("a"), &Number::from(1));
    tuple.set0(&JsString::from("changed"));
    tuple.set1(&Number::from(99));
    assert_eq!(tuple.get0(), "changed");
    assert_eq!(tuple.get1(), 99);
}

#[wasm_bindgen_test]
fn tuple_extends_array() {
    let tuple: ArrayTuple<(JsString, JsString)> =
        ArrayTuple::new2(&JsString::from("a"), &JsString::from("b"));
    let arr: &Array = tuple.unchecked_ref();
    assert_eq!(arr.length(), 2);
}

#[wasm_bindgen_test]
fn first_method() {
    let tuple1: ArrayTuple<(JsString,)> = ArrayTuple::new1(&JsString::from("first"));
    assert_eq!(tuple1.first(), "first");

    let tuple2: ArrayTuple<(JsString, Number)> =
        ArrayTuple::new2(&JsString::from("one"), &Number::from(2));
    assert_eq!(tuple2.first(), "one");

    let tuple3: ArrayTuple<(Number, JsString, JsString)> = ArrayTuple::new3(
        &Number::from(42),
        &JsString::from("middle"),
        &JsString::from("end"),
    );
    assert_eq!(tuple3.first(), 42);
}

#[wasm_bindgen_test]
fn last_method() {
    let tuple1: ArrayTuple<(JsString,)> = ArrayTuple::new1(&JsString::from("only"));
    assert_eq!(tuple1.last(), "only");

    let tuple2: ArrayTuple<(Number, JsString)> =
        ArrayTuple::new2(&Number::from(1), &JsString::from("last"));
    assert_eq!(tuple2.last(), "last");

    let tuple3: ArrayTuple<(JsString, Number, JsString)> = ArrayTuple::new3(
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
}

#[wasm_bindgen_test]
fn into_parts_method() {
    let tuple1: ArrayTuple<(JsString,)> = ArrayTuple::new1(&JsString::from("single"));
    let (a,) = tuple1.into_parts();
    assert_eq!(a, "single");

    let tuple2: ArrayTuple<(JsString, Number)> =
        ArrayTuple::new2(&JsString::from("hello"), &Number::from(42));
    let (first, second) = tuple2.into_parts();
    assert_eq!(first, "hello");
    assert_eq!(second, 42);

    let tuple3: ArrayTuple<(Number, JsString, Number)> = ArrayTuple::new3(
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
}

#[wasm_bindgen_test]
fn covariance_to_array() {
    use wasm_bindgen::prelude::Upcast;

    fn accepts_number_array(arr: Array<Number>) -> u32 {
        arr.length()
    }

    let tuple: ArrayTuple<(Number, Number)> =
        ArrayTuple::new2(&Number::from(42), &Number::from(100));

    let length = accepts_number_array(tuple.upcast_into());
    assert_eq!(length, 2);
}

#[wasm_bindgen_test]
fn complex_nested_covariance() {
    use wasm_bindgen::prelude::Upcast;

    let func: Function<fn() -> Number> = Function::new_no_args_typed("return 42");

    let promise_func: Promise<Function<fn() -> Number>> = Promise::resolve(&func);

    let num: Number = Number::from(42);
    let num_as_jsvalue: JsValue = num.upcast_into();
    assert!(num_as_jsvalue.as_f64().is_some());

    let func_num: Function<fn() -> Number> = Function::new_no_args_typed("return 42");
    let func_jsvalue: Function<fn() -> JsValue> = func_num.upcast_into();
    assert!(func_jsvalue.is_function());

    let promise_func_num: Promise<Function<fn() -> Number>> = Promise::resolve(&func).upcast_into();
    let promise_func_jsvalue: Promise<Function<fn() -> JsValue>> = promise_func_num.upcast_into();
    assert!(promise_func_jsvalue.is_object());

    let tuple_complex: ArrayTuple<(Promise<Function<fn() -> Number>>, Number)> =
        ArrayTuple::new2(promise_func.upcast(), &Number::from(100));
    // In js_sys_unstable_apis, the Upcast impl for deeply nested generics requires exact type matching
    // for the inner generic parameters, so we cast to Array<JsValue> instead
    #[cfg(not(js_sys_unstable_apis))]
    {
        let tuple_wider: ArrayTuple<(Promise<Function>, JsValue)> = tuple_complex.upcast_into();
        assert_eq!(tuple_wider.len(), 2);
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let tuple_wider: Array<JsValue> = tuple_complex.upcast_into();
        assert_eq!(tuple_wider.length(), 2);
    }
}

#[wasm_bindgen_test]
fn arraytuple_to_array_mixed_types() {
    use wasm_bindgen::prelude::Upcast;

    let tuple: ArrayTuple<(Number, JsString, Array)> =
        ArrayTuple::new3(&Number::from(42), &JsString::from("hello"), &Array::new());

    let array: Array<JsValue> = tuple.upcast_into();
    assert_eq!(array.length(), 3);

    #[cfg(not(js_sys_unstable_apis))]
    {
        assert!(array.get(0).as_f64().is_some());
        assert!(array.get(1).is_string());
        assert!(Array::is_array(&array.get(2)));
    }
    #[cfg(js_sys_unstable_apis)]
    {
        assert!(array.get(0).unwrap().as_f64().is_some());
        assert!(array.get(1).unwrap().is_string());
        assert!(Array::is_array(&array.get(2).unwrap()));
    }
}

#[wasm_bindgen_test]
fn arraytuple_to_array_single_type() {
    use wasm_bindgen::prelude::Upcast;

    let tuple1: ArrayTuple<(Number,)> = ArrayTuple::new1(&Number::from(100));
    let array1: Array<JsValue> = tuple1.upcast_into();
    assert_eq!(array1.length(), 1);
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(array1.get(0).as_f64().unwrap(), 100.0);
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(array1.get(0).unwrap().as_f64().unwrap(), 100.0);

    let tuple_strings: ArrayTuple<(JsString, JsString, JsString)> = ArrayTuple::new3(
        &JsString::from("a"),
        &JsString::from("b"),
        &JsString::from("c"),
    );
    let array_strings: Array<JsString> = tuple_strings.upcast_into();
    assert_eq!(array_strings.length(), 3);
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(array_strings.get(0).as_string().unwrap(), "a");
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(array_strings.get(0).unwrap().as_string().unwrap(), "a");
}

#[wasm_bindgen_test]
fn arraytuple_to_array_large_tuple() {
    use wasm_bindgen::prelude::Upcast;

    let tuple8: ArrayTuple<(
        Number,
        Number,
        Number,
        Number,
        Number,
        Number,
        Number,
        Number,
    )> = ArrayTuple::new8(
        &Number::from(1),
        &Number::from(2),
        &Number::from(3),
        &Number::from(4),
        &Number::from(5),
        &Number::from(6),
        &Number::from(7),
        &Number::from(8),
    );
    let array8: Array<Number> = tuple8.upcast_into();
    assert_eq!(array8.length(), 8);
    #[cfg(not(js_sys_unstable_apis))]
    {
        assert_eq!(array8.get(0).value_of(), 1.0);
        assert_eq!(array8.get(7).value_of(), 8.0);
    }
    #[cfg(js_sys_unstable_apis)]
    {
        assert_eq!(array8.get(0).unwrap().value_of(), 1.0);
        assert_eq!(array8.get(7).unwrap().value_of(), 8.0);
    }

    let tuple5: ArrayTuple<(JsString, Number, Array, JsString, Number)> = ArrayTuple::new5(
        &JsString::from("a"),
        &Number::from(2),
        &Array::new(),
        &JsString::from("d"),
        &Number::from(5),
    );
    let array5: Array<JsValue> = tuple5.upcast_into();
    assert_eq!(array5.length(), 5);
}

#[wasm_bindgen_test]
fn arraytuple_to_array_nested_generics() {
    use wasm_bindgen::prelude::Upcast;

    let num_array: Array<Number> = Array::new_typed();
    num_array.push(&Number::from(1));

    let str_array: Array<JsString> = Array::new_typed();
    str_array.push(&JsString::from("test"));

    let tuple: ArrayTuple<(Array<Number>, Array<JsString>)> =
        ArrayTuple::new2(&num_array, &str_array);

    let array_of_arrays: Array<Array<JsValue>> = tuple.upcast_into();
    assert_eq!(array_of_arrays.length(), 2);

    #[cfg(not(js_sys_unstable_apis))]
    let first = array_of_arrays.get(0);
    #[cfg(js_sys_unstable_apis)]
    let first = array_of_arrays.get(0).unwrap();
    assert_eq!(first.length(), 1);
}

#[wasm_bindgen_test]
fn arraytuple_identity_covariance() {
    use wasm_bindgen::prelude::Upcast;

    let tuple: ArrayTuple<(Number, JsString)> =
        ArrayTuple::new2(&Number::from(42), &JsString::from("test"));

    let same: ArrayTuple<(Number, JsString)> = tuple.upcast_into();
    assert_eq!(same.len(), 2);
    assert_eq!(same.get0(), 42);
    assert_eq!(same.get1(), "test");
}

#[wasm_bindgen_test]
fn default_creates_typed_tuple() {
    // Test ArrayTuple::default() for various arities
    let tuple1: ArrayTuple<(JsString,)> = Default::default();
    assert_eq!(tuple1.len(), 1);
    assert_eq!(tuple1.get0(), "");

    let tuple2: ArrayTuple<(Number, JsString)> = Default::default();
    assert_eq!(tuple2.len(), 2);
    assert_eq!(tuple2.get0(), 0);
    assert_eq!(tuple2.get1(), "");

    let tuple3: ArrayTuple<(Number, Number, Number)> = Default::default();
    assert_eq!(tuple3.len(), 3);

    let tuple4: ArrayTuple<(JsString, JsString, JsString, JsString)> = Default::default();
    assert_eq!(tuple4.len(), 4);

    let tuple5: ArrayTuple<(Number, Number, Number, Number, Number)> = Default::default();
    assert_eq!(tuple5.len(), 5);

    let tuple6: ArrayTuple<(JsString, JsString, JsString, JsString, JsString, JsString)> =
        Default::default();
    assert_eq!(tuple6.len(), 6);

    let tuple7: ArrayTuple<(Number, Number, Number, Number, Number, Number, Number)> =
        Default::default();
    assert_eq!(tuple7.len(), 7);

    let tuple8: ArrayTuple<(
        JsString,
        JsString,
        JsString,
        JsString,
        JsString,
        JsString,
        JsString,
        JsString,
    )> = Default::default();
    assert_eq!(tuple8.len(), 8);
}

#[wasm_bindgen_test]
fn default_tuple_can_be_modified() {
    let tuple: ArrayTuple<(Number, JsString)> = Default::default();
    // Check initial default values
    assert_eq!(tuple.get0(), 0);
    assert_eq!(tuple.get1(), "");

    // Modify values
    tuple.set0(&Number::from(42));
    tuple.set1(&JsString::from("hello"));

    assert_eq!(tuple.get0(), 42);
    assert_eq!(tuple.get1(), "hello");
}

#[wasm_bindgen_test]
fn default_tuple_3_can_be_modified() {
    let tuple: ArrayTuple<(JsString, Number, JsString)> = Default::default();
    tuple.set0(&JsString::from("first"));
    tuple.set1(&Number::from(99));
    tuple.set2(&JsString::from("last"));

    assert_eq!(tuple.get0(), "first");
    assert_eq!(tuple.get1(), 99);
    assert_eq!(tuple.get2(), "last");
}
