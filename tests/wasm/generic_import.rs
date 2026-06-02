use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

#[wasm_bindgen(module = "tests/wasm/generic_import.js")]
extern "C" {
    // Opt-in per-monomorphisation generic import. Each concrete `T`
    // gets its own JS adapter with `T`-specific marshalling, all bound
    // to the same JS `record_generic` function. The `IntoWasmAbi +
    // WasmDescribe` bounds are supplied by the macro expansion.
    #[wasm_bindgen()]
    fn record_generic<T>(x: T);

    // Multiple arguments, mixed concrete + generic, with the generic
    // parameter repeated across two argument positions.
    #[wasm_bindgen()]
    fn record_mixed<T>(a: T, b: u32, c: T);

    // Multiple distinct type parameters.
    #[wasm_bindgen()]
    fn record_two<A, B>(a: A, b: B);

    // Generic argument and generic return (round-trip).
    #[wasm_bindgen()]
    fn groundtrip<T>(x: T) -> T;

    // Zero arguments, generic return position only.
    #[wasm_bindgen()]
    fn gget<T>() -> T;

    // Generic parameter nested inside `Option<_>`.
    #[wasm_bindgen()]
    fn record_opt<T>(x: Option<T>);

    // Generic parameter behind a shared reference.
    #[wasm_bindgen()]
    fn record_ref<T>(x: &T);

    // Generic closure argument: the JS side invokes the callback with
    // each value, marshalled per `T` via a per-monomorphisation invoke shim.
    #[wasm_bindgen()]
    fn call_each<T>(f: &mut dyn FnMut(T));

    // Generic parameter nested inside `Vec<_>`.
    #[wasm_bindgen()]
    fn record_vec<T>(xs: Vec<T>);

    // Generic closure taking `Option<T>`.
    #[wasm_bindgen()]
    fn call_each_option<T>(f: &mut dyn FnMut(Option<T>));

    // Generic closure returning `T`.
    #[wasm_bindgen()]
    fn call_each_return<T>(f: &mut dyn FnMut() -> T);

    // `catch` with a generic argument and generic `Ok` return.
    #[wasm_bindgen(catch)]
    fn try_maybe<T>(x: T) -> Result<T, JsValue>;

    // Async generic import: the JS side returns a Promise that resolves to
    // the per-`T` value; the wrapper awaits it via a typed `JsFuture<T>`.
    #[wasm_bindgen()]
    async fn async_echo<T>(x: T) -> T;

    // High arity: nine generic arguments sharing one hole, exercising the
    // `wbg_generic_import_9` courier and the wider `GenericFills` carriers.
    #[wasm_bindgen()]
    #[allow(clippy::too_many_arguments)]
    fn record_many<T>(a: T, b: T, c: T, d: T, e: T, f: T, g: T, h: T, i: T);

    // Generic parameter nested two levels deep: `Vec<Option<T>>` crosses
    // as a `(T | undefined)[]` externref array.
    #[wasm_bindgen()]
    fn record_vec_opt<T>(xs: Vec<Option<T>>);

    // Generic closure with two distinct type parameters.
    #[wasm_bindgen()]
    fn call_pair<A, B>(f: &mut dyn FnMut(A, B));

    fn take_generic_log() -> Vec<JsValue>;

    type Stats;
    // Generic static method: each `T` binds to the same JS static.
    #[wasm_bindgen(static_method_of = Stats, js_name = "combine")]
    fn combine<T>(a: T, b: T) -> T;

    // Generic constructor on a generic class, plus a generic getter-style
    // method, both actually instantiated (not compile-only).
    type Boxed<T>;
    #[wasm_bindgen(constructor)]
    fn new_boxed<T>(v: T) -> Boxed<T>;
    #[wasm_bindgen(method)]
    fn unwrap<T>(this: &Boxed<T>) -> T;

    type Recorder;
    #[wasm_bindgen(constructor)]
    fn new() -> Recorder;
    // Generic instance method on a concrete class: each `T` binds to the
    // same JS `pushVal` with `T`-specific marshalling.
    #[wasm_bindgen(method, js_name = "pushVal")]
    fn push_val<T>(this: &Recorder, x: T);
    #[wasm_bindgen(method, getter)]
    fn last(this: &Recorder) -> JsValue;
    // Generic setter on a concrete class.
    #[wasm_bindgen(method, setter = tag)]
    fn set_tag<T>(this: &Recorder, x: T);
    #[wasm_bindgen(method, getter)]
    fn tag(this: &Recorder) -> JsValue;
}

#[wasm_bindgen_test]
fn generic_import_marshals_per_type() {
    record_generic(42u32);
    record_generic(3.5f64);
    record_generic("hello");

    let log = take_generic_log();
    assert_eq!(log.len(), 3);
    // u32 arrives as a JS number.
    assert_eq!(log[0].as_f64(), Some(42.0));
    // f64 arrives as a JS number.
    assert_eq!(log[1].as_f64(), Some(3.5));
    // &str arrives as a JS string (decoded from the two-word ABI) —
    // impossible under the type-erased generics path.
    assert_eq!(log[2].as_string(), Some("hello".to_string()));
}

#[wasm_bindgen_test]
fn generic_import_mixed_and_repeated_params() {
    // `T = &str` at positions 0 and 2 (one hole, used twice), concrete
    // `u32` at position 1.
    record_mixed("x", 9, "y");

    let log = take_generic_log();
    assert_eq!(log.len(), 3);
    assert_eq!(log[0].as_string(), Some("x".to_string()));
    assert_eq!(log[1].as_f64(), Some(9.0));
    assert_eq!(log[2].as_string(), Some("y".to_string()));
}

#[wasm_bindgen_test]
fn generic_import_multiple_type_params() {
    record_two(1u32, "two");

    let log = take_generic_log();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0].as_f64(), Some(1.0));
    assert_eq!(log[1].as_string(), Some("two".to_string()));
}

#[wasm_bindgen_test]
fn generic_import_generic_return() {
    // Generic argument and generic return marshalled per `T`.
    let s: String = groundtrip("round".to_string());
    assert_eq!(s, "round");

    let n: u32 = groundtrip(123u32);
    assert_eq!(n, 123);
}

#[wasm_bindgen_test]
fn generic_import_return_only() {
    let n: u32 = gget();
    assert_eq!(n, 7);
}

#[wasm_bindgen_test]
fn generic_import_option_arg() {
    record_opt(Some(5u32));
    record_opt::<u32>(None);

    let log = take_generic_log();
    assert_eq!(log.len(), 2);
    assert_eq!(log[0].as_f64(), Some(5.0));
    assert!(log[1].is_undefined() || log[1].is_null());
}

#[wasm_bindgen_test]
fn generic_import_ref_arg() {
    let v = JsValue::from(99u32);
    record_ref(&v);

    let log = take_generic_log();
    assert_eq!(log.len(), 1);
    assert_eq!(log[0].as_f64(), Some(99.0));
}

mod dobj {
    pub trait DurableObject {
        type Stub;
    }
}

// Generic method on a *generic* class with an associated-type return:
// exercises class-generic hoisting (`impl<T> DoNamespace<T>`) and a hole
// that is a projection (`<T as DurableObject>::Stub`), not a bare param.
// Compile-only: never instantiated, so no courier is monomorphised.
#[wasm_bindgen]
extern "C" {
    type MyDo;
    type MyDoStub;
    type DoNamespace<T: dobj::DurableObject = MyDo>;

    #[wasm_bindgen(method, js_name = "getByName")]
    fn get_by_name<T: dobj::DurableObject = MyDo>(
        this: &DoNamespace<T>,
        name: &str,
    ) -> <T as dobj::DurableObject>::Stub;
}

impl dobj::DurableObject for MyDo {
    type Stub = MyDoStub;
}

#[wasm_bindgen_test]
fn generic_import_method() {
    let r = Recorder::new();
    r.push_val(7u32);
    assert_eq!(r.last().as_f64(), Some(7.0));
    r.push_val("hi");
    assert_eq!(r.last().as_string(), Some("hi".to_string()));
}

#[wasm_bindgen_test]
fn generic_import_closure() {
    let mut sum = 0u32;
    call_each(&mut |x: u32| sum += x);
    assert_eq!(sum, 6);

    let mut acc = String::new();
    call_each(&mut |x: f64| acc.push_str(&x.to_string()));
    assert_eq!(acc, "123");
}

#[wasm_bindgen_test]
fn generic_import_vec_arg() {
    record_vec(vec![1u32, 2, 3]);

    let log = take_generic_log();
    assert_eq!(log.len(), 1);
    let arr = js_sys::Array::from(&log[0]);
    assert_eq!(arr.length(), 3);
    assert_eq!(arr.get(0).as_f64(), Some(1.0));
    assert_eq!(arr.get(2).as_f64(), Some(3.0));
}

#[wasm_bindgen_test]
fn generic_import_closure_option() {
    let mut got: Vec<Option<u32>> = Vec::new();
    call_each_option(&mut |x: Option<u32>| got.push(x));
    assert_eq!(got, vec![Some(5), None, Some(7)]);
}

#[wasm_bindgen_test]
fn generic_import_closure_return() {
    let mut next = 10u32;
    call_each_return(&mut || {
        let v = next;
        next += 1;
        v
    });
    assert_eq!(next, 12);
}

#[wasm_bindgen_test]
async fn generic_import_async() {
    let n: u32 = async_echo(42u32).await;
    assert_eq!(n, 42);

    let s: String = async_echo("hello".to_string()).await;
    assert_eq!(s, "hello");
}

#[wasm_bindgen_test]
fn generic_import_catch() {
    let ok: Result<u32, JsValue> = try_maybe(5u32);
    assert_eq!(ok.unwrap(), 5);

    let err: Result<u32, JsValue> = try_maybe(13u32);
    assert!(err.is_err());
}

#[wasm_bindgen_test]
fn generic_import_primitive_abis() {
    // Distinct ABIs each force a fresh monomorphisation: bool (i32),
    // i64 (two-word / bigint), char (u32 scalar).
    record_generic(true);
    record_generic(-9i64);
    record_generic('z');

    let log = take_generic_log();
    assert_eq!(log.len(), 3);
    assert_eq!(log[0].as_bool(), Some(true));
    assert_eq!(log[1].clone().try_into(), Ok(-9i64));
    assert_eq!(log[2].as_string(), Some("z".to_string()));
}

#[wasm_bindgen_test]
fn generic_import_high_arity() {
    record_many(1u32, 2, 3, 4, 5, 6, 7, 8, 9);

    let log = take_generic_log();
    assert_eq!(log.len(), 9);
    assert_eq!(log[0].as_f64(), Some(1.0));
    assert_eq!(log[8].as_f64(), Some(9.0));
}

#[wasm_bindgen_test]
fn generic_import_vec_handle() {
    // `Vec<T>` where `T` is a JS-handle type exercises the zero-copy
    // `ErasableGeneric<Repr = JsValue>` slice path, distinct from the
    // native typed-array path used by `Vec<u32>` above.
    record_vec(vec![JsValue::from("a"), JsValue::from(2u32)]);

    let log = take_generic_log();
    assert_eq!(log.len(), 1);
    let arr = js_sys::Array::from(&log[0]);
    assert_eq!(arr.length(), 2);
    assert_eq!(arr.get(0).as_string(), Some("a".to_string()));
    assert_eq!(arr.get(1).as_f64(), Some(2.0));
}

#[wasm_bindgen_test]
fn generic_import_nested_vec_option() {
    record_vec_opt(vec![Some(1u32), None, Some(3u32)]);

    let log = take_generic_log();
    assert_eq!(log.len(), 1);
    let arr = js_sys::Array::from(&log[0]);
    assert_eq!(arr.length(), 3);
    assert_eq!(arr.get(0).as_f64(), Some(1.0));
    assert!(arr.get(1).is_undefined() || arr.get(1).is_null());
    assert_eq!(arr.get(2).as_f64(), Some(3.0));
}

#[wasm_bindgen_test]
fn generic_import_closure_two_params() {
    let mut nums = 0u32;
    let mut strs = String::new();
    call_pair(&mut |n: u32, s: String| {
        nums += n;
        strs.push_str(&s);
    });
    assert_eq!(nums, 3);
    assert_eq!(strs, "ab");
}

#[wasm_bindgen_test]
fn generic_import_static_method() {
    assert_eq!(Stats::combine(3u32, 4u32), 7);
    assert_eq!(Stats::combine("a".to_string(), "b".to_string()), "ab");
}

#[wasm_bindgen_test]
fn generic_import_generic_constructor() {
    let b = Boxed::new_boxed(55u32);
    assert_eq!(b.unwrap(), 55u32);

    let s = Boxed::new_boxed("boxed".to_string());
    assert_eq!(s.unwrap(), "boxed");
}

#[wasm_bindgen_test]
fn generic_import_setter() {
    let r = Recorder::new();
    r.set_tag(5u32);
    assert_eq!(r.tag().as_f64(), Some(5.0));
    r.set_tag("hi");
    assert_eq!(r.tag().as_string(), Some("hi".to_string()));
}

#[wasm_bindgen_test]
async fn promise_resolve_primitive() {
    let p: js_sys::Promise<u32> = js_sys::Promise::resolve(&5u32);
    let v = wasm_bindgen_futures::JsFuture::from(p).await.unwrap();
    assert_eq!(v, 5u32);
}
