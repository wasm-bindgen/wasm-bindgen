use js_sys::*;
use wasm_bindgen::convert::Upcast;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
#[cfg(js_sys_unstable_apis)]
use wasm_bindgen::JsGeneric;
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
    let args: Array<JsValue> = Array::new();
    args.push(&1.into());
    args.push(&2.into());
    args.push(&3.into());
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(
        MAX.with(|max| max.apply(&JsValue::undefined(), &args))
            .unwrap(),
        3
    );
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(
        MAX.with(|max| max
            .unchecked_ref::<Function>()
            .apply(&JsValue::undefined(), &args))
            .unwrap(),
        JsValue::from(3)
    );

    let arr = Array::<JsValue>::new();
    let args: Array<JsValue> = Array::new();
    args.push(&1.into());
    ARRAY_PROTOTYPE2
        .with(ArrayPrototype::push)
        .apply(&arr, &args)
        .unwrap();
    assert_eq!(Array::from_iterable(&arr).unwrap().length(), 1);
}

#[wasm_bindgen(module = "tests/wasm/Function.js")]
extern "C" {
    fn get_function_to_bind() -> Function<fn() -> Number>;
    fn get_value_to_bind_to() -> JsValue;
    #[cfg(not(js_sys_unstable_apis))]
    fn list() -> Function;
    #[cfg(js_sys_unstable_apis)]
    fn list() -> Function<
        fn(
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
        ) -> Array<Number>,
    >;
    #[cfg(not(js_sys_unstable_apis))]
    fn add_arguments() -> Function<fn(JsValue, JsValue) -> JsValue>;
    #[cfg(js_sys_unstable_apis)]
    fn add_arguments() -> Function<fn(Number, Number) -> Number>;
    #[cfg(not(js_sys_unstable_apis))]
    fn call_function<'a>(f: &Function) -> JsValue;
    #[cfg(js_sys_unstable_apis)]
    fn call_function<T: JsGeneric>(f: &Function<fn() -> T>) -> T;
    #[wasm_bindgen(js_name = call_function)]
    fn call_function_none(f: &Function<fn() -> Number>) -> Number;
    #[cfg(not(js_sys_unstable_apis))]
    fn call_function_arg(f: &Function, arg0: JsValue) -> JsValue;
    #[cfg(js_sys_unstable_apis)]
    fn call_function_arg<Ret: JsGeneric, Arg: JsGeneric>(
        f: &Function<fn(Arg) -> Ret>,
        arg0: Arg,
    ) -> Ret;
    #[wasm_bindgen(js_name = call_function_arg)]
    fn call_function_arg_num(f: &Function<fn(Number) -> Undefined>, arg0: Number);
    #[cfg(not(js_sys_unstable_apis))]
    fn sum_many_arguments() -> Function;
    #[cfg(js_sys_unstable_apis)]
    fn sum_many_arguments() -> Function<
        fn(
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
        ) -> Number,
    >;
    fn test_context() -> JsValue;
    #[cfg(not(js_sys_unstable_apis))]
    fn multiply_sum() -> Function;
    #[cfg(js_sys_unstable_apis)]
    fn multiply_sum() -> Function<
        fn(
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
            JsOption<Number>,
        ) -> Number,
    >;
}

#[wasm_bindgen_test]
fn bind() {
    let f = get_function_to_bind();
    #[cfg(not(js_sys_unstable_apis))]
    let new_f = f.bind0(&get_value_to_bind_to());
    #[cfg(js_sys_unstable_apis)]
    let new_f = f.bind(&get_value_to_bind_to(), ());
    #[cfg(not(js_sys_unstable_apis))]
    {
        assert_eq!(call_function(f.unchecked_ref()), 1);
        assert_eq!(call_function(new_f.unchecked_ref()), 2);
    }
    #[cfg(js_sys_unstable_apis)]
    {
        assert_eq!(call_function::<Number>(&f), 1);
        assert_eq!(call_function::<Number>(&new_f), 2);
    }
}

#[wasm_bindgen_test]
fn bind0() {
    let f = get_function_to_bind();
    #[cfg(not(js_sys_unstable_apis))]
    {
        let new_f = f.bind0(&get_value_to_bind_to());
        assert_eq!(call_function(f.unchecked_ref()), 1);
        assert_eq!(call_function(new_f.unchecked_ref()), 2);
    }
    #[cfg(js_sys_unstable_apis)]
    {
        let new_f = f.bind(&get_value_to_bind_to(), ());
        assert_eq!(call_function::<Number>(&f), 1);
        assert_eq!(call_function::<Number>(&new_f), 2);
    }
}

#[wasm_bindgen_test]
fn bind1() {
    let a_list = list();
    #[cfg(not(js_sys_unstable_apis))]
    let prepended_list = a_list.bind1(&JsValue::NULL, &JsValue::from(2));
    #[cfg(js_sys_unstable_apis)]
    let prepended_list = a_list.bindn(&JsValue::NULL.into(), (Number::from(2).upcast(),));

    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(
        Array::from(&call_function(prepended_list.upcast()))
            .pop_checked()
            .unwrap(),
        2
    );

    #[cfg(js_sys_unstable_apis)]
    assert_eq!(
        Array::from_iterable(&call_function::<Array<Number>>(prepended_list.upcast()))
            .unwrap()
            .pop_checked()
            .unwrap(),
        2
    );

    let adder = add_arguments();
    #[cfg(not(js_sys_unstable_apis))]
    let add_42 = adder.bind1(&JsValue::NULL, &JsValue::from(42));
    #[cfg(js_sys_unstable_apis)]
    let add_42 = adder.bindn(&JsValue::NULL, (&Number::from(42),));

    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(call_function_arg(add_42.upcast(), JsValue::from(1)), 43);
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(call_function_arg::<Number, _>(&add_42, Number::from(1)), 43);
    #[cfg(not(js_sys_unstable_apis))]
    assert_eq!(call_function_arg(add_42.upcast(), JsValue::from(378)), 420);
    #[cfg(js_sys_unstable_apis)]
    assert_eq!(call_function_arg(&add_42, Number::from(378)), 420);
}

#[cfg(not(js_sys_unstable_apis))]
#[wasm_bindgen_test]
fn bind2() {
    let a_list = list();
    let prepended_list = a_list.bind2(&JsValue::NULL, &JsValue::from(2), &JsValue::from(3));

    #[cfg(not(js_sys_unstable_apis))]
    let arr = Array::from(&call_function(prepended_list.upcast()));
    #[cfg(js_sys_unstable_apis)]
    let arr = Array::from_iterable(&call_function(&prepended_list)).unwrap();

    assert_eq!(arr.pop(), 3);
    assert_eq!(arr.pop(), 2);

    let adder = add_arguments();
    let always_69 = adder.bind2(&JsValue::NULL, &JsValue::from(66), &JsValue::from(3));

    assert_eq!(call_function(always_69.upcast()), 69);
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn bind2() {
    let a_list = list();
    let prepended_list = a_list.bindn(
        &JsValue::NULL,
        (Number::from(2).upcast(), Number::from(3).upcast()),
    );

    #[cfg(not(js_sys_unstable_apis))]
    let arr =
        Array::from_iterable(&call_function(prepended_list.unchecked_ref::<Function>())).unwrap();
    #[cfg(js_sys_unstable_apis)]
    let arr =
        Array::from_iterable(&call_function::<Array<Number>>(&prepended_list.upcast())).unwrap();

    assert_eq!(arr.pop_checked(), Some(Number::from(3)));
    assert_eq!(arr.pop_checked(), Some(Number::from(2)));

    let adder = add_arguments();
    let always_69 = adder.bindn(&JsValue::NULL, (&Number::from(66), &Number::from(3)));

    assert_eq!(call_function::<Number>(&always_69), 69);
}

#[cfg(not(js_sys_unstable_apis))]
#[wasm_bindgen_test]
fn bind3() {
    let a_list = list();
    let prepended_list = a_list.bind3(
        &JsValue::NULL,
        &JsValue::from(2),
        &JsValue::from(3),
        &JsValue::from(4),
    );

    #[cfg(not(js_sys_unstable_apis))]
    let arr = Array::from(&call_function(prepended_list.upcast()));
    #[cfg(js_sys_unstable_apis)]
    let arr = Array::from_iterable(&call_function(&prepended_list)).unwrap();

    assert_eq!(arr.pop(), 4);
    assert_eq!(arr.pop(), 3);
    assert_eq!(arr.pop(), 2);

    let adder = add_arguments();
    let always_69 = adder.bind2(&JsValue::NULL, &JsValue::from(66), &JsValue::from(3));

    assert_eq!(call_function(always_69.upcast()), 69);
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn bind3() {
    let a_list = list();
    let prepended_list = a_list.bind(
        &JsValue::NULL,
        (
            Number::from(2).upcast(),
            Number::from(3).upcast(),
            Number::from(4).upcast(),
        ),
    );

    let arr =
        Array::from_iterable(&call_function::<Array<Number>>(&prepended_list.upcast())).unwrap();

    assert_eq!(arr.pop(), Some(Number::from(4)));
    assert_eq!(arr.pop(), Some(Number::from(3)));
    assert_eq!(arr.pop(), Some(Number::from(2)));

    let adder = add_arguments();
    let always_69 = adder.bind(&JsValue::NULL, (&Number::from(66), &Number::from(3)));

    assert_eq!(call_function::<Number>(&always_69), 69);
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
    assert!(MAX.with(|v| Object::to_string(v)).length() > 0);
}

#[wasm_bindgen_test]
fn function_inheritance() {
    assert!(MAX.with(Function::is_instance_of::<Function>));
    assert!(MAX.with(Function::is_instance_of::<Object>));
    MAX.with(|max| {
        let _: &Object = max.as_ref();
    });
}

#[cfg(not(js_sys_unstable_apis))]
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
    assert_eq!(call_function(add_fixed.upcast()), 10);
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn bind4() {
    let adder = sum_many_arguments();
    let add_fixed = adder.bind(
        &JsValue::NULL,
        (
            Number::from(1).upcast(),
            Number::from(2).upcast(),
            Number::from(3).upcast(),
            Number::from(4).upcast(),
        ),
    );
    assert_eq!(call_function::<Number>(&add_fixed.upcast()), 10);
}

#[cfg(not(js_sys_unstable_apis))]
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
    assert_eq!(call_function(add_fixed.upcast()), 15);
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn bind5() {
    let adder = sum_many_arguments();
    let add_fixed = adder.bind(
        &JsValue::NULL,
        (
            Number::from(1).upcast(),
            Number::from(2).upcast(),
            Number::from(3).upcast(),
            Number::from(4).upcast(),
            Number::from(5).upcast(),
        ),
    );
    assert_eq!(call_function::<Number>(&add_fixed.upcast()), 15);
}

#[cfg(not(js_sys_unstable_apis))]
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
    assert_eq!(call_function(add_fixed.upcast()), 21);
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn bind6() {
    let adder = sum_many_arguments();
    let add_fixed = adder.bind(
        &JsValue::NULL,
        (
            Number::from(1).upcast(),
            Number::from(2).upcast(),
            Number::from(3).upcast(),
            Number::from(4).upcast(),
            Number::from(5).upcast(),
            Number::from(6).upcast(),
        ),
    );
    assert_eq!(call_function::<Number>(&add_fixed.upcast()), 21);
}

#[cfg(not(js_sys_unstable_apis))]
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
    assert_eq!(call_function(add_fixed.upcast()), 28);
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn bind7() {
    let adder = sum_many_arguments();
    let add_fixed = adder.bind(
        &JsValue::NULL,
        (
            Number::from(1).upcast(),
            Number::from(2).upcast(),
            Number::from(3).upcast(),
            Number::from(4).upcast(),
            Number::from(5).upcast(),
            Number::from(6).upcast(),
            Number::from(7).upcast(),
        ),
    );
    assert_eq!(call_function::<Number>(&add_fixed.upcast()), 28);
}

#[cfg(not(js_sys_unstable_apis))]
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
    assert_eq!(call_function(add_fixed.upcast()), 36);
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn bind8() {
    let adder = sum_many_arguments();
    let add_fixed = adder.bind(
        &JsValue::NULL,
        (
            Number::from(1).upcast(),
            Number::from(2).upcast(),
            Number::from(3).upcast(),
            Number::from(4).upcast(),
            Number::from(5).upcast(),
            Number::from(6).upcast(),
            Number::from(7).upcast(),
            Number::from(8).upcast(),
        ),
    );
    assert_eq!(call_function::<Number>(&add_fixed), 36);
}

#[cfg(not(js_sys_unstable_apis))]
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

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn call4() {
    let multiply = multiply_sum();
    let result = multiply
        .call(
            &test_context(),
            (
                Number::from(1).upcast(),
                Number::from(2).upcast(),
                Number::from(3).upcast(),
                Number::from(4).upcast(),
            ),
        )
        .unwrap();
    assert_eq!(result, 100);
}

#[cfg(not(js_sys_unstable_apis))]
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

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn call5() {
    let multiply = multiply_sum();
    let result = multiply
        .call(
            &test_context(),
            (
                Number::from(1).upcast(),
                Number::from(2).upcast(),
                Number::from(3).upcast(),
                Number::from(4).upcast(),
                Number::from(5).upcast(),
            ),
        )
        .unwrap();
    assert_eq!(result, 150);
}

#[cfg(not(js_sys_unstable_apis))]
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

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn call6() {
    let multiply = multiply_sum();
    let result = multiply
        .call(
            &test_context(),
            (
                Number::from(1).upcast(),
                Number::from(2).upcast(),
                Number::from(3).upcast(),
                Number::from(4).upcast(),
                Number::from(5).upcast(),
                Number::from(6).upcast(),
            ),
        )
        .unwrap();
    assert_eq!(result, 210);
}

#[cfg(not(js_sys_unstable_apis))]
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

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn call7() {
    let multiply = multiply_sum();
    let result = multiply
        .call(
            &test_context(),
            (
                Number::from(1).upcast(),
                Number::from(2).upcast(),
                Number::from(3).upcast(),
                Number::from(4).upcast(),
                Number::from(5).upcast(),
                Number::from(6).upcast(),
                Number::from(7).upcast(),
            ),
        )
        .unwrap();
    assert_eq!(result, 280);
}

#[cfg(not(js_sys_unstable_apis))]
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

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn call8() {
    let multiply: Function = multiply_sum().unchecked_into();
    let result = multiply
        .call(
            &test_context(),
            (
                &JsValue::from(1),
                &JsValue::from(2),
                &JsValue::from(3),
                &JsValue::from(4),
                &JsValue::from(5),
                &JsValue::from(6),
                &JsValue::from(7),
                &JsValue::from(8),
            ),
        )
        .unwrap();
    assert_eq!(result, 360);
}

#[cfg(not(js_sys_unstable_apis))]
#[wasm_bindgen_test]
fn generic_function_new() {
    let f: Function<fn() -> Number> = Function::new_no_args_typed("return 42");
    assert_eq!(call_function_none(f.upcast()), 42);
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn generic_function_new() {
    let f: Function<fn() -> Number> = Function::new_no_args_typed("return 42");
    assert_eq!(call_function_none(f.upcast()).value_of(), 42.0);
}

#[wasm_bindgen_test]
fn generic_function_new1() {
    let f = Function::new_with_args_typed("x", "return x * 2");
    call_function_arg_num(&f, Number::from(21));
}

#[wasm_bindgen_test]
fn generic_function_new2() {
    let f = Function::<fn(Number, Number) -> Number>::new_with_args_typed("x, y", "return x + y");
    let result = f
        .call(&JsValue::NULL, (&Number::from(10), &Number::from(32)))
        .unwrap();
    assert_eq!(result, 42);
}

#[cfg(not(js_sys_unstable_apis))]
#[wasm_bindgen_test]
fn closure_to_function_covariance() {
    let closure: Closure<dyn FnMut(Number) -> ()> = Closure::new(|_: Number| -> () {});
    let foo: Function<fn(Number) -> Undefined> = Function::from_closure(closure);
    let _ret1 = foo.call(&JsValue::UNDEFINED, (&Number::from(5),)).unwrap();

    let closure: Closure<dyn Fn(Number) -> ()> = Closure::new(|_: Number| -> () {});
    let foo: Function<fn(Number) -> Undefined> = Function::from_closure(closure);
    let _ret1 = foo.call(&JsValue::UNDEFINED, (&Number::from(5),)).unwrap();

    call_function_arg_num(foo.upcast(), Number::from(42));

    let closure_i32: Closure<dyn FnMut(Number) -> i32> =
        Closure::new(|foo: Number| -> i32 { foo.as_f64().unwrap() as i32 + 5 });
    let func_i32: Function<fn(Number) -> Number> = Function::from_closure(closure_i32);
    let bound = func_i32.bind1(&JsValue::UNDEFINED, &Number::from(5));

    let result_f32 = call_function_none(bound.upcast());
    assert_eq!(result_f32.value_of(), 10.0);

    let closure_f32: Closure<dyn FnMut() -> f32> = Closure::new(|| -> f32 { 3.14 });
    let func_f32: &Function<fn() -> Number> = Function::closure_ref(&closure_f32);

    let result_f32 = call_function_none(func_f32.upcast());
    assert_eq!(result_f32.value_of() as f32, 3.14);
}

#[cfg(js_sys_unstable_apis)]
#[wasm_bindgen_test]
fn closure_to_function_covariance() {
    let closure: Closure<dyn FnMut(f64) -> ()> = Closure::new(|_: f64| -> () {});
    let foo: Function<fn(Number) -> Undefined> = Function::from_closure(closure);
    let foo_uc = &foo;
    let _ret1 = foo_uc
        .call(&JsValue::UNDEFINED, (&Number::from(5),))
        .unwrap();

    let closure: Closure<dyn Fn(f64) -> ()> = Closure::new(|_: f64| -> () {});
    let foo: Function<fn(Number) -> Undefined> = Function::from_closure(closure);
    let _ret1 = foo.call(&JsValue::UNDEFINED, (&Number::from(5),)).unwrap();

    call_function_arg_num(foo.upcast(), Number::from(42));

    let closure_i32: Closure<dyn FnMut(f64) -> i32> =
        Closure::new(|foo| -> i32 { (foo as i32) + 5 });
    let func_i32: Function<fn(Number) -> Number> = Function::from_closure(closure_i32);
    let bound = func_i32.bind(&JsValue::UNDEFINED, (&Number::from(5),));

    let result_f32 = call_function_none(bound.upcast());
    assert_eq!(result_f32.value_of(), 10.0);

    let closure_f32: Closure<dyn FnMut() -> f32> = Closure::new(|| -> f32 { 3.14 });
    let func_f32: &Function<fn() -> Number> = Function::closure_ref(&closure_f32);

    let result_f32 = call_function_none(func_f32.upcast());
    assert_eq!(result_f32.value_of() as f32, 3.14);
}

#[wasm_bindgen_test]
fn function_returning_array_of_functions() {
    let outer: Function<fn() -> Array<Function<fn() -> Number>>> =
        Function::new_no_args_typed("return [function() { return 42; }]");

    let outer_cast: Function<fn() -> Array<Function<fn() -> JsValue>>> = outer.upcast_into();

    let arr = outer_cast.call(&JsValue::NULL, ()).unwrap();
    assert_eq!(arr.length(), 1);
}

#[wasm_bindgen_test]
fn function_accepting_array_of_functions() {
    let outer: Function<fn(Array<Function<fn(Array<Number>) -> Number>>) -> Number> =
        Function::new_with_args_typed("funcs", "return funcs[0]([1, 2])");

    let arr: Array<Function<fn(Array<Number>) -> Number>> = Array::new_typed();
    let f: Function<fn(Array<Number>) -> Number> =
        Function::new_with_args_typed("x", "return x.length");
    arr.push(&f);

    let result2 = outer.call(&Undefined::UNDEFINED, (&arr,)).unwrap();
    assert_eq!(result2.value_of(), 2.0);
}

#[wasm_bindgen(module = "tests/wasm/Function.js")]
extern "C" {
    /// Simulates a for_each method like DOMTokenList.forEach
    /// Calls the callback with (string, index) for each item
    fn invoke_for_each_callback(
        callback: &Function<fn(JsString, Number) -> Undefined>,
        items: &Array<JsString>,
    );
}

/// Test for_each pattern: passing a Rust closure through Function::closure_ref
/// to a function that expects `&Function<fn(JsString, Number) -> Undefined>`
/// This simulates the signature:
/// ```ignore
/// pub fn for_each(
///     this: &DomTokenList,
///     callback: &::js_sys::Function<fn(::js_sys::JsString, ::js_sys::Number) -> ::js_sys::Undefined>,
/// ) -> Result<(), JsValue>;
/// ```
#[wasm_bindgen_test]
fn for_each_with_closure_conversion() {
    let items: Array<JsString> = Array::new_typed();
    items.push(&JsString::from("apple"));
    items.push(&JsString::from("banana"));
    items.push(&JsString::from("cherry"));

    let mut results = Vec::new();
    // Rust closure, borrowing results data
    let mut func = |value: JsString, index: Number| {
        results.push((value.as_string().unwrap(), index.value_of() as u32));
    };
    // Scoped closure, borrowing the Rust closure
    {
        let scoped_closure = ScopedClosure::borrow_mut(&mut func);
        invoke_for_each_callback(Function::closure_ref(&scoped_closure), &items);
    }

    // Verify the closure was called correctly for each item
    assert_eq!(results.len(), 3);
    assert_eq!(results[0], ("apple".to_string(), 0));
    assert_eq!(results[1], ("banana".to_string(), 1));
    assert_eq!(results[2], ("cherry".to_string(), 2));
}
