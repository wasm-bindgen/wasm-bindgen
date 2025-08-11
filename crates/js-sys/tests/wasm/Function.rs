use js_sys::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(thread_local_v2, js_name = max, js_namespace = Math)]
    static MAX: Function;

    type ArrayPrototype;
    #[wasm_bindgen(method, getter, structural)]
    pub fn push(this: &ArrayPrototype) -> Function;
    #[wasm_bindgen(thread_local_v2, js_name = prototype, js_namespace = Array)]
    static ARRAY_PROTOTYPE2: ArrayPrototype;
}

#[wasm_bindgen_test]
fn apply() {
    let args = Array::new();
    args.push(&1.into());
    args.push(&2.into());
    args.push(&3.into());
    assert_eq!(
        MAX.with(|max| max.apply(&JsValue::undefined(), &args))
            .unwrap(),
        3
    );

    let arr = JsValue::from(Array::new());
    let args = Array::new();
    args.push(&1.into());
    ARRAY_PROTOTYPE2
        .with(ArrayPrototype::push)
        .apply(&arr, &args)
        .unwrap();
    assert_eq!(Array::from(&arr).length(), 1);
}

#[wasm_bindgen(module = "tests/wasm/Function.js")]
extern "C" {
    fn get_function_to_bind() -> Function;
    fn get_value_to_bind_to() -> JsValue;
    fn list() -> Function;
    fn add_arguments() -> Function;
    fn call_function(f: &Function) -> JsValue;
    fn call_function_arg(f: &Function, arg0: JsValue) -> JsValue;
    fn sum_many_arguments() -> Function;
    fn test_context() -> JsValue;
    fn multiply_sum() -> Function;

}

#[wasm_bindgen_test]
fn bind() {
    let f = get_function_to_bind();
    let new_f = f.bind(&get_value_to_bind_to());
    assert_eq!(call_function(&f), 1);
    assert_eq!(call_function(&new_f), 2);
}

#[wasm_bindgen_test]
fn bind0() {
    let f = get_function_to_bind();
    let new_f = f.bind0(&get_value_to_bind_to());
    assert_eq!(call_function(&f), 1);
    assert_eq!(call_function(&new_f), 2);
}

#[wasm_bindgen_test]
fn bind1() {
    let a_list = list();
    let prepended_list = a_list.bind1(&JsValue::NULL, &JsValue::from(2));

    assert_eq!(Array::from(&call_function(&prepended_list)).pop(), 2);

    let adder = add_arguments();
    let add_42 = adder.bind1(&JsValue::NULL, &JsValue::from(42));

    assert_eq!(call_function_arg(&add_42, JsValue::from(1)), 43);
    assert_eq!(call_function_arg(&add_42, JsValue::from(378)), 420);
}

#[wasm_bindgen_test]
fn bind2() {
    let a_list = list();
    let prepended_list = a_list.bind2(&JsValue::NULL, &JsValue::from(2), &JsValue::from(3));

    let arr = Array::from(&call_function(&prepended_list));

    assert_eq!(arr.pop(), 3);
    assert_eq!(arr.pop(), 2);

    let adder = add_arguments();
    let always_69 = adder.bind2(&JsValue::NULL, &JsValue::from(66), &JsValue::from(3));

    assert_eq!(call_function(&always_69), 69);
}

#[wasm_bindgen_test]
fn bind3() {
    let a_list = list();
    let prepended_list = a_list.bind3(
        &JsValue::NULL,
        &JsValue::from(2),
        &JsValue::from(3),
        &JsValue::from(4),
    );

    let arr = Array::from(&call_function(&prepended_list));

    assert_eq!(arr.pop(), 4);
    assert_eq!(arr.pop(), 3);
    assert_eq!(arr.pop(), 2);

    let adder = add_arguments();
    let always_69 = adder.bind2(&JsValue::NULL, &JsValue::from(66), &JsValue::from(3));

    assert_eq!(call_function(&always_69), 69);
}

#[wasm_bindgen_test]
fn length() {
    assert_eq!(MAX.with(Function::length), 2);
    assert_eq!(ARRAY_PROTOTYPE2.with(ArrayPrototype::push).length(), 1);
}

#[wasm_bindgen_test]
fn name() {
    assert_eq!(JsValue::from(MAX.with(Function::name)), "max");
    assert_eq!(
        JsValue::from(ARRAY_PROTOTYPE2.with(ArrayPrototype::push).name()),
        "push"
    );
}

#[wasm_bindgen_test]
fn to_string() {
    assert!(MAX.with(Function::to_string).length() > 0);
}

#[wasm_bindgen_test]
fn function_inheritance() {
    assert!(MAX.with(Function::is_instance_of::<Function>));
    assert!(MAX.with(Function::is_instance_of::<Object>));
    MAX.with(|max| {
        let _: &Object = max.as_ref();
    });
}

#[wasm_bindgen_test]
fn bind4() {
    let adder = sum_many_arguments();
    let add_fixed = adder.bind4(
        &JsValue::NULL,
        &JsValue::from(1),
        &JsValue::from(2),
        &JsValue::from(3),
        &JsValue::from(4),
    );
    assert_eq!(call_function(&add_fixed), 10);
}

#[wasm_bindgen_test]
fn bind5() {
    let adder = sum_many_arguments();
    let add_fixed = adder.bind5(
        &JsValue::NULL,
        &JsValue::from(1),
        &JsValue::from(2),
        &JsValue::from(3),
        &JsValue::from(4),
        &JsValue::from(5),
    );
    assert_eq!(call_function(&add_fixed), 15);
}

#[wasm_bindgen_test]
fn bind6() {
    let adder = sum_many_arguments();
    let add_fixed = adder.bind6(
        &JsValue::NULL,
        &JsValue::from(1),
        &JsValue::from(2),
        &JsValue::from(3),
        &JsValue::from(4),
        &JsValue::from(5),
        &JsValue::from(6),
    );
    assert_eq!(call_function(&add_fixed), 21);
}

#[wasm_bindgen_test]
fn bind7() {
    let adder = sum_many_arguments();
    let add_fixed = adder.bind7(
        &JsValue::NULL,
        &JsValue::from(1),
        &JsValue::from(2),
        &JsValue::from(3),
        &JsValue::from(4),
        &JsValue::from(5),
        &JsValue::from(6),
        &JsValue::from(7),
    );
    assert_eq!(call_function(&add_fixed), 28);
}

#[wasm_bindgen_test]
fn bind8() {
    let adder = sum_many_arguments();
    let add_fixed = adder.bind8(
        &JsValue::NULL,
        &JsValue::from(1),
        &JsValue::from(2),
        &JsValue::from(3),
        &JsValue::from(4),
        &JsValue::from(5),
        &JsValue::from(6),
        &JsValue::from(7),
        &JsValue::from(8),
    );
    assert_eq!(call_function(&add_fixed), 36);
}

#[wasm_bindgen_test]
fn call4() {
    let multiply = multiply_sum();
    let result = multiply
        .call4(
            &test_context(),
            &JsValue::from(1),
            &JsValue::from(2),
            &JsValue::from(3),
            &JsValue::from(4),
        )
        .unwrap();
    assert_eq!(result, 100);
}

#[wasm_bindgen_test]
fn call5() {
    let multiply = multiply_sum();
    let result = multiply
        .call5(
            &test_context(),
            &JsValue::from(1),
            &JsValue::from(2),
            &JsValue::from(3),
            &JsValue::from(4),
            &JsValue::from(5),
        )
        .unwrap();
    assert_eq!(result, 150);
}

#[wasm_bindgen_test]
fn call6() {
    let multiply = multiply_sum();
    let result = multiply
        .call6(
            &test_context(),
            &JsValue::from(1),
            &JsValue::from(2),
            &JsValue::from(3),
            &JsValue::from(4),
            &JsValue::from(5),
            &JsValue::from(6),
        )
        .unwrap();
    assert_eq!(result, 210);
}

#[wasm_bindgen_test]
fn call7() {
    let multiply = multiply_sum();
    let result = multiply
        .call7(
            &test_context(),
            &JsValue::from(1),
            &JsValue::from(2),
            &JsValue::from(3),
            &JsValue::from(4),
            &JsValue::from(5),
            &JsValue::from(6),
            &JsValue::from(7),
        )
        .unwrap();
    assert_eq!(result, 280);
}

#[wasm_bindgen_test]
fn call8() {
    let multiply = multiply_sum();
    let result = multiply
        .call8(
            &test_context(),
            &JsValue::from(1),
            &JsValue::from(2),
            &JsValue::from(3),
            &JsValue::from(4),
            &JsValue::from(5),
            &JsValue::from(6),
            &JsValue::from(7),
            &JsValue::from(8),
        )
        .unwrap();
    assert_eq!(result, 360);
}

#[wasm_bindgen_test]
fn bind9() {
    let adder = sum_many_arguments();
    let add_fixed = adder.bind9(
        &JsValue::NULL,
        &JsValue::from(1),
        &JsValue::from(2),
        &JsValue::from(3),
        &JsValue::from(4),
        &JsValue::from(5),
        &JsValue::from(6),
        &JsValue::from(7),
        &JsValue::from(8),
        &JsValue::from(9),
    );
    assert_eq!(call_function(&add_fixed), 45);
}

#[wasm_bindgen_test]
fn call9() {
    let multiply = multiply_sum();
    let result = multiply
        .call9(
            &test_context(),
            &JsValue::from(1),
            &JsValue::from(2),
            &JsValue::from(3),
            &JsValue::from(4),
            &JsValue::from(5),
            &JsValue::from(6),
            &JsValue::from(7),
            &JsValue::from(8),
            &JsValue::from(9),
        )
        .unwrap();
    assert_eq!(result, 450);
}
